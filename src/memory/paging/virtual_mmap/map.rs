use x86_64::structures::paging::{
    page_table::{PageTableEntry, PageTableLevel},
    Page, PageSize, PageTable, PageTableFlags, PageTableIndex, PhysFrame, Size4KiB, FrameAllocator,
};

use crate::memory::{
    types::Bytes, paging::{mem_structure::MEM_STRUCTURE, physical_mmap::frame_allocator::FRAME_ALLOCATOR},
};

use super::{Mapper, VMMapperGeneral};

/// The trait which includes the functions to map pages to page frames.
///
/// # Safety
/// You could mess this pretty much up by mapping a page to the wrong page
/// frame, so keep an Eye on it, duh.
pub unsafe trait VMMapperMap<P: PageSize>: VMMapperGeneral<P> {
    /// Creates a new instance.
    ///
    /// # Note
    /// Don't forget to map the PML4 page table in this function!
    fn new() -> Self;

    /// Maps a page to the given page_frame (if available) with the given flags.
    ///
    /// * `page`: The page to be mapped.
    /// * `page_frame`: If it's `Some`, then the page will be mapped to the given page frame,
    ///                 otherwise a new page frame is allocated and mapped by
    ///                 `self.translate_addr`.
    /// * `flags`: The flags for the given mapping.
    ///
    /// # Safety
    /// Make sure that the given page + page frame are valid otherwise you're
    /// playing russian roulette with a shotgun. Have fun!
    unsafe fn map_page(&self, page: Page, page_frame: Option<PhysFrame>, flags: PageTableFlags);

    /// Maps the given page frame by a standard-mapping implementation.
    ///
    /// * `page_frame`: The page frame which should be mapped.
    /// * `flags`: The flags for the page.
    ///
    /// # Safety
    /// The given page frame **must** be valid.
    unsafe fn map_page_frame(&self, page_frame: PhysFrame, flags: PageTableFlags) {
        let page = {
            let addr = self.translate_addr(page_frame.start_address());
            Page::from_start_address(addr).unwrap()
        };

        unsafe { self.map_page(page, Some(page_frame), flags) };
    }

    /// Maps a range of pages in a romw.
    ///
    /// * `page`: The starting page which should be mapped.
    /// * `page_frame`: The starting page frame (if available) which should be mapped.
    ///                 If it's `None`, random page-frames are picked up then.
    /// * `len`: The amount of bytes which should be mapped in a row.
    /// * `flags`: The flags for each page.
    ///
    /// # Safety
    /// If `page_frame` is `Some(...)`, then you **have to** make sure that, the
    /// range, starting from the given page frame until `start + len`
    /// (exclusive) **is** a **valid** Physicall address range!!!
    unsafe fn map_page_range(
        &self,
        page: Page,
        page_frame: Option<PhysFrame>,
        len: Bytes,
        flags: PageTableFlags,
    );
}

impl Mapper {
    /// Adds the given page frame to the givien page table.
    ///
    /// * `page_table`: The page which shoud get the page frame as the entry
    ///                 value.
    /// * `entry_index`: The index of the page table where to insert the value.
    /// * `flags`: The flags for the entry.
    ///
    /// # Safety
    /// You need to make sure that the pointer points to a avlid page table!
    unsafe fn set_pt_entry(
        &self,
        page_table: *mut PageTable,
        entry_index: PageTableIndex,
        page_frame: PhysFrame,
        flags: PageTableFlags,
    ) {
        let entry = {
            let mut entry = PageTableEntry::new();
            entry.set_addr(page_frame.start_address(), flags);
            entry
        };

        unsafe {
            (*page_table)[entry_index] = entry;
        }
    }
}

unsafe impl VMMapperMap<Size4KiB> for Mapper {
    fn new() -> Self {
        let start = MEM_STRUCTURE.hhdm;
        let pml4e = MEM_STRUCTURE.pml4.get().unwrap();

        let mapper = Self {
            start,
            p4_ptr: pml4e.virt.as_mut_ptr() as *mut PageTable,
        };

        unsafe {
            let page_frame = PhysFrame::from_start_address(pml4e.phys).unwrap();
            let page = Page::from_start_address(pml4e.virt).unwrap();
            mapper.map_page(
                page,
                Some(page_frame),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE,
            );
        }

        mapper
    }

    unsafe fn map_page(&self, page: Page, page_frame: Option<PhysFrame>, flags: PageTableFlags) {
        let mut pt_ptr = self.p4_ptr;
        let mut level = PageTableLevel::Four;

        while let Some(lower_level) = level.next_lower_level() {
            let entry_index = match lower_level {
                PageTableLevel::Three => page.start_address().p4_index(),
                PageTableLevel::Two => page.start_address().p3_index(),
                PageTableLevel::One => page.start_address().p2_index(),
                _ => unreachable!("Ayo, '{:?}' shouldn't be here <.<", lower_level),
            };

            let table_entry = unsafe { (*pt_ptr)[entry_index].clone() };

            level = lower_level;
            pt_ptr = {
                let addr = if table_entry.is_unused() {
                    let page_table_frame = FRAME_ALLOCATOR.write().allocate_frame().unwrap();
                    unsafe {
                        self.set_pt_entry(
                            pt_ptr,
                            entry_index,
                            page_table_frame,
                            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                        );
                    }
                    page_table_frame.start_address()
                } else {
                    table_entry.addr()
                };
                self.translate_addr(addr).as_mut_ptr() as *mut PageTable
            }
        }

        let page_frame =
            page_frame.unwrap_or_else(|| FRAME_ALLOCATOR.write().allocate_frame().unwrap());

        unsafe { self.set_pt_entry(pt_ptr, page.p1_index(), page_frame, flags) }
    }

    /// Maps a range of pages in a row.
    ///
    /// * `page`: The starting page which should be mapped.
    /// * `page_frame`: The starting page frame (if available) which should be mapped.
    ///                 If it's `None`, random page-frames are picked up then.
    /// * `len`: The amount of bytes which should be mapped in a row.
    /// * `flags`: The flags for each page.
    ///
    /// # Note
    /// If `page_frame` is `Some(...)`, then you **have to** make sure that, the range, starting
    /// from the given page frame until `start + len` is **a valid Physicall address range**!!!
    unsafe fn map_page_range(
        &self,
        page: Page,
        page_frame: Option<PhysFrame>,
        len: Bytes,
        flags: PageTableFlags,
    ) {
        for offset in (0..len.as_u64()).step_by(Size4KiB::SIZE.try_into().unwrap()) {
            let page = {
                let addr = (page.start_address() + offset).align_down(Size4KiB::SIZE);
                Page::from_start_address(addr).unwrap()
            };

            let page_frame = page_frame.map(|frame| {
                let addr = (frame.start_address() + offset).align_down(Size4KiB::SIZE);
                PhysFrame::from_start_address(addr).unwrap()
            });

            unsafe { self.map_page(page, page_frame, flags) };
        }
    }
}
