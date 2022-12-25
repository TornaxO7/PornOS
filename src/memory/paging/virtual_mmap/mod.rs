pub mod level_page_tables;
pub mod vmmap_traits;

mod unmap;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{
    structures::paging::{
        page_table::{PageTableEntry, PageTableLevel},
        FrameAllocator, FrameDeallocator, Page, PageSize, PageTable, PageTableFlags, PhysFrame,
        Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::memory::{types::Bytes, HHDM};

use self::{
    level_page_tables::PTLevels,
    vmmap_traits::{VMMMapper, VMMapperMap, VMmapperUnmap},
};

use super::{frame_allocator::FRAME_ALLOCATOR, PML4E_ADDR};

lazy_static! {
    /// **S**uper **I**mpressive **M**a**p**per
    ///
    /// Yes, this is the mapper which maps page to page frames or in other
    /// words: The ultimate ***SIMP***
    ///
    /// This is also known as the "Page-Mapper".
    pub static ref SIMP: Mutex<Mapper> = Mutex::new(Mapper::new());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mapper {
    p4_ptr: *mut PageTable,

    /// The starting virt-address where the mappings can start to be mapped.
    start: VirtAddr,
}

unsafe impl Send for Mapper {}

unsafe impl VMMMapper<Size4KiB> for Mapper {
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

    fn translate_addr(&self, addr: PhysAddr) -> VirtAddr {
        self.start + addr.as_u64()
    }
}

unsafe impl VMMapperMap<Size4KiB> for Mapper {
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

    /// Maps the given page frame to the page which is calculated as follows:
    ///
    ///     page-start-addr = self.start + page_frame.start_address()
    ///
    unsafe fn map_page_frame(&self, page_frame: PhysFrame, flags: PageTableFlags) {
        let page = {
            let addr = self.start + page_frame.start_address().as_u64();
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

unsafe impl VMmapperUnmap<Size4KiB> for Mapper {
    unsafe fn unmap_page(&self, page: Page) -> Option<PhysFrame> {
        let page_tables = self.get_page_tables(page)?;
        let freed_page_frame = {
            let p1 = page_tables.get_pt(PageTableLevel::One);
            let p1_entry = (*p1)[page.p1_index()];

            (*p1)[page.p1_index()] = PageTableEntry::new();
            PhysFrame::from_start_address(p1_entry.addr()).unwrap()
        };

        if page_tables.is_empty(PageTableLevel::One) {
            // let p1 = page_tables.get_pt(PageTableLevel::One);
            // let page = Page::from_start_address(VirtAddr::from_ptr(p1)).unwrap();
            // let page_frame = self.unmap_page(page).unwrap();
            // FRAME_ALLOCATOR.write().deallocate_frame(page_frame);
            //
            page_tables.free_pt(PageTableLevel::Two, PageTableLevel::One, page.p2_index());
        }
        if page_tables.is_empty(PageTableLevel::Two) {
            page_tables.free_pt(PageTableLevel::Three, PageTableLevel::Two, page.p3_index());
        }
        if page_tables.is_empty(PageTableLevel::Three) {
            page_tables.free_pt(PageTableLevel::Four, PageTableLevel::Three, page.p4_index());
        }

        Some(freed_page_frame)
    }

    unsafe fn get_page_tables(&self, page: Page) -> Option<PTLevels> {
        let mut page_tables = PTLevels::new(self.p4_ptr);

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
            if table_entry.is_unused() {
                return None;
            }

            level = lower_level;
            pt_ptr = {
                let addr = table_entry.addr();
                self.translate_addr(addr).as_mut_ptr() as *mut PageTable
            };

            page_tables.set_pt(pt_ptr, lower_level);
        }

        Some(page_tables)
    }
}
