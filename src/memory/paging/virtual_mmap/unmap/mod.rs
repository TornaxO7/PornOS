mod page_tables;
mod utils;

use x86_64::{
    structures::paging::{
        page_table::{PageTableEntry, PageTableLevel},
        FrameDeallocator, Page, PageSize, PageTable, PageTableIndex, PhysFrame, Size4KiB,
    },
    VirtAddr,
};

use crate::memory::paging::frame_allocator::FRAME_ALLOCATOR;

use self::page_tables::PageTables;

use super::{Mapper, VMMapperGeneral};

/// # Safety
/// Make sure that you are not unmapping the wrong page!
pub unsafe trait VMmapperUnmap<P: PageSize>: VMMapperGeneral<P> {
    /// Unmaps the given page and returns the unmapped page frame if everything
    /// works fine.
    ///
    /// * `page`: The page which should be unmapped.
    ///
    /// # Returns
    /// - `Some`: The mapped page frame of the given page (it's not freed!)
    /// - `None`: If the given page wasn't mapped.
    ///
    /// # Safety
    /// Make sure that the the `page` is correct and not any other page which
    /// you currently use!!!
    unsafe fn unmap_page(&self, page: Page) -> Option<PhysFrame>;
}

unsafe impl VMmapperUnmap<Size4KiB> for Mapper {
    /// Unmaps the given page and returns the unmapped page frame if everything
    /// works fine.
    ///
    /// * `page`: The page which should be unmapped.
    ///
    /// # Returns
    /// - `Ok(addr)`: The page frame which was mapped by the given page.
    /// - `Err(addr)`:
    unsafe fn unmap_page(&self, page: Page) -> Option<PhysFrame> {
        let page_tables = unsafe { self.collect_page_tables(&page) }?;
        let phys_frame = {
            let p1_table = page_tables.get_pt(PageTableLevel::One);
            let entry = unsafe { ((*p1_table)[page.p1_index()]).clone() };
            unsafe { (*p1_table)[page.p1_index()] = PageTableEntry::new() };
            entry.frame().unwrap()
        };

        for (parent_pt, child_pt, parent_level) in page_tables.pairs().unwrap() {
            let entry_index = match parent_level {
                PageTableLevel::Four => page.p4_index(),
                PageTableLevel::Three => page.p3_index(),
                PageTableLevel::Two => page.p2_index(),
                PageTableLevel::One => unreachable!("Ehhh this should be done before."),
            };

            if unsafe { self.is_pt_empty(child_pt) } {
                unsafe { self.free_pt(parent_pt, child_pt, entry_index) };
            }
        }

        Some(phys_frame)
    }
}

impl Mapper {
    unsafe fn collect_page_tables(&self, page: &Page) -> Option<PageTables> {
        let mut page_tables = PageTables::new(self.p4_ptr);

        let mut pt_ptr = self.p4_ptr;
        let mut level = PageTableLevel::Four;
        while let Some(lower_level) = level.next_lower_level() {
            let entry_index = match lower_level {
                PageTableLevel::Three => page.start_address().p4_index(),
                PageTableLevel::Two => page.start_address().p3_index(),
                PageTableLevel::One => page.start_address().p2_index(),
                _ => unreachable!("Ayo, '{:?}' shouldn't be here <.<", lower_level),
            };

            let table_entry = unsafe { &(*pt_ptr)[entry_index] };
            if table_entry.is_unused() {
                return None;
            }

            level = lower_level;
            pt_ptr = self.translate_addr(table_entry.addr()).as_mut_ptr() as *mut PageTable;
            page_tables.set_pt(pt_ptr, level);
        }

        Some(page_tables)
    }

    unsafe fn free_pt(
        &self,
        parent_pt: *mut PageTable,
        child_pt: *mut PageTable,
        parent_idx: PageTableIndex,
    ) {
        unsafe { (*parent_pt)[parent_idx] = PageTableEntry::new() };

        let page_frame = {
            let addr = VirtAddr::from_ptr(child_pt);
            let phys_addr = unsafe { self.detranslate_addr(addr) };
            PhysFrame::from_start_address(phys_addr).unwrap()
        };

        unsafe { FRAME_ALLOCATOR.write().deallocate_frame(page_frame) }
    }

    unsafe fn is_pt_empty(&self, pt: *const PageTable) -> bool {
        let page_table = unsafe { (*pt).clone() };
        let is_empty = page_table.iter().all(|entry| entry.is_unused());
        is_empty
    }
}
