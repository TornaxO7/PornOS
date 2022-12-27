use x86_64::structures::paging::{
    page_table::{PageTableEntry, PageTableLevel},
    FrameAllocator, Page, PageSize, PageTable, PageTableFlags, PhysFrame, Size4KiB,
};

use crate::memory::{
    paging::{frame_allocator::FRAME_ALLOCATOR, PML4E_ADDR},
    types::Bytes,
    HHDM,
};

use super::{Mapper, VMMapperGeneral};

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
    ///                 otherwise a new page frame will ba allocated.
    /// * `flags`: The flags for the given mapping.
    unsafe fn map_page(&self, page: Page, page_frame: Option<PhysFrame>, flags: PageTableFlags);

    /// Maps the given page frame by a standard-mapping implementation.
    ///
    /// * `page_frame`: The page frame which should be mapped.
    /// * `flags`: The flags for the page.
    unsafe fn map_page_frame(&self, page_frame: PhysFrame, flags: PageTableFlags) {
        let page = {
            let addr = self.translate_addr(page_frame.start_address());
            Page::from_start_address(addr).unwrap()
        };

        self.map_page(page, Some(page_frame), flags);
    }

    /// Maps a range of pages in a romw.
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
    );
}

unsafe impl VMMapperMap<Size4KiB> for Mapper {
    fn new() -> Self {
        let start = {
            let pf_allocator_end_addr = *HHDM
                + FRAME_ALLOCATOR.read().start.as_u64()
                + FRAME_ALLOCATOR.read().get_size().as_u64();
            pf_allocator_end_addr.align_up(Size4KiB::SIZE)
        };

        let mapper = Self {
            start,
            p4_ptr: (start + PML4E_ADDR.get().unwrap().as_u64()).as_mut_ptr() as *mut PageTable,
        };

        unsafe {
            mapper.map_page_frame(
                PhysFrame::from_start_address(*PML4E_ADDR.get().unwrap()).unwrap(),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
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

            let table_entry = &(*pt_ptr)[entry_index];

            level = lower_level;
            pt_ptr = {
                let addr = if table_entry.is_unused() {
                    let page_frame = { FRAME_ALLOCATOR.write().allocate_frame().unwrap() };
                    self.map_page_frame(
                        page_frame,
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                    );
                    page_frame.start_address()
                } else {
                    table_entry.addr()
                };
                self.translate_addr(addr).as_mut_ptr() as *mut PageTable
            }
        }

        let page_frame =
            page_frame.unwrap_or_else(|| FRAME_ALLOCATOR.write().allocate_frame().unwrap());

        let entry = {
            let mut entry = PageTableEntry::new();
            entry.set_addr(page_frame.start_address(), flags);
            entry
        };

        unsafe {
            (*pt_ptr)[page.p1_index()] = entry;
        }
    }

    /// Maps a range of pages in a romw.
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

            self.map_page(page, page_frame, flags);
        }
    }
}
