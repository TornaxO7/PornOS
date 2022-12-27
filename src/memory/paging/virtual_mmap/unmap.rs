use x86_64::structures::paging::{
    page_table::{PageTableEntry, PageTableLevel},
    Page, PageSize, PageTable, PhysFrame, Size4KiB,
};

use crate::memory::paging::frame_allocator::FRAME_ALLOCATOR;

use super::{Mapper, VMMapperGeneral};

pub unsafe trait VMmapperUnmap<P: PageSize>: VMMapperGeneral<P> {
    /// Unmpas the given page and returns the unmapped page frame if everything
    /// works fine.
    ///
    /// * `page`: The page which should be unmapped.
    ///
    /// # Returns
    /// - `Ok(addr)`: The page frame which was mapped by the given page.
    /// - `Err(addr)`:
    unsafe fn unmap_page(&self, page: Page) -> Option<PhysFrame>;
}

const AMOUNT_PAGE_TABLES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageTables([*mut PageTable; AMOUNT_PAGE_TABLES]);

impl Mapper {
    unsafe fn collect_page_tables(&self, page: &Page) -> Option<PageTables> {
        let mut page_tables: [Option<*mut PageTable>; AMOUNT_PAGE_TABLES] =
            [None; AMOUNT_PAGE_TABLES];
        let mut pt_index: usize = 0;
        page_tables[pt_index] = Some(self.p4_ptr);
        pt_index += 1;

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

            page_tables[pt_index] = Some(pt_ptr);
            pt_index += 1;
        }

        Some(PageTables(
            page_tables.map(|page_table| page_table.unwrap()),
        ))
    }
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
        let page_tables = unsafe {self.collect_page_tables(&page)}?;
        let phys_frame = {
            let p1_table = page_tables.get_pt(PageTableLevel::One);
            let entry = (*p1_table)[page.p1_index()];
            (*p1_table)[page.p1_index()] = PageTableEntry::new();
            entry
        };

        for (parent_pt, child_pt, level) in page_tables {
            let entry_index = match level {
                PageTableLevel::Four => page.p4_index(),
                PageTableLevel::Three => page.p3_index(),
                PageTableLevel::Two => page.p2_index(),
                PageTableLevel::One => unreachable!("Ehhh this should be done before."),
            };

            if child_pt.is_empty() {
                self.free_pt(parent_pt, child_pt, level, entry_index);
            }
        }

        phys_frame
    }
}
