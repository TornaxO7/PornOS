use x86_64::structures::paging::{
    page_table::{PageTableEntry, PageTableLevel},
    FrameDeallocator, Page, PageSize, PageTable, PhysFrame, Size4KiB,
};

use crate::memory::paging::frame_allocator::FRAME_ALLOCATOR;

use super::Mapper;

const AMOUNT_PAGE_TABLES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageTables([*mut PageTable; AMOUNT_PAGE_TABLES]);

struct PageTablePairsIterator {
    index: usize,
    page_tables: [*mut PageTable; AMOUNT_PAGE_TABLES],
}

impl PageTablePairsIterator {
    pub fn new(page_tables: PageTables) -> Self {
        Self {
            index: 1,
            page_tables: page_tables.0,
        }
    }
}

impl Iterator for PageTablePairsIterator {
    type Item = (*mut PageTable, *mut PageTable);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < AMOUNT_PAGE_TABLES {
            let return_value = (
                self.page_tables[self.index - 1],
                self.page_tables[self.index],
            );
            self.index += 1;
            Some(return_value)
        } else {
            None
        }
    }
}

impl PageTables {
    pub fn get_pairs(&self) -> PageTablePairsIterator {
        let page_tables: [Option<* mut PageTable>; AMOUNT_PAGE_TABLES] = [None; AMOUNT_PAGE_TABLES];
        page_tables[0] = Some();
    }
}

impl Mapper {
    fn collect_page_tables(&self) -> PageTables {
        todo!()
    }

    unsafe fn _unmap_page(&self, page: Page) -> Option<PhysFrame> {
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
                    return None;
                } else {
                    table_entry.addr()
                };
                self.translate_addr(addr).as_mut_ptr() as *mut PageTable
            }
        }

        let p1_entry = (*pt_ptr)[page.start_address().p1_index()];
        let page_frame = p1_entry.frame().unwrap();

        // clear entry of p1 table
        (*pt_ptr)[page.start_address().p1_index] = PageTableEntry::new();

        Some(page_frame)
    }

    unsafe fn free_page_tables(&self, page: Page) {
        let mut prev_pt_ptr = self.p4_ptr;
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
                    return None;
                } else {
                    table_entry.addr()
                };
                self.translate_addr(addr).as_mut_ptr() as *mut PageTable
            }
        }
    }
}

pub unsafe trait VMmapperUnmap<P: PageSize> {
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

unsafe impl VMmapperUnmap<Size4KiB> for Mapper {
    /// Unmaps the given page and returns the unmapped page frame if everything
    /// works fine.
    ///
    /// * `page`: The page which should be unmapped.
    ///
    /// # Returns
    /// - `Ok(addr)`: The page frame which was mapped by the given page.
    /// - `Err(addr)`:
    fn unmap_page(&self, page: Page) -> Option<PhysFrame> {
        let page_tables = self.collect_page_tables(page);
        let phys_frame = page_tables.clear_entry(PageTableLevel::One, page.p1_index());

        for page_table in page_tables.tables() {
            if page_table.is_empty() {}
        }

        if page_tables.is_empty(PageTableLevel::One) {
            page_tables.free_pt(PageTableLevel::One);
        }
        if page_tables.is_empty(PageTableLevel::Two) {
            page_tables.free_pt(PageTableLevel::Two);
        }
        if page_tables.is_empty(PageTableLevel::Three) {
            page_tables.free_pt(PageTableLevel::Three);
        }

        phys_frame
    }
}
