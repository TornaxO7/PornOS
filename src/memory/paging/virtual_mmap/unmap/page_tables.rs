use x86_64::structures::paging::{page_table::PageTableLevel, PageTable};

use super::utils::{next_higher_level, ptl_to_index};

const AMOUNT_PAGE_TABLES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageTables([Option<*mut PageTable>; AMOUNT_PAGE_TABLES]);

impl PageTables {
    pub fn new(p4_ptr: *mut PageTable) -> Self {
        let mut page_tables = Self([None; AMOUNT_PAGE_TABLES]);
        page_tables.set_pt(p4_ptr, PageTableLevel::Four);
        page_tables
    }

    pub fn set_pt(&mut self, page_table: *mut PageTable, level: PageTableLevel) {
        let index = ptl_to_index(level);
        self.0[index] = Some(page_table);
    }

    pub fn get_pt(&self, level: PageTableLevel) -> *mut PageTable {
        self.try_get_pt(level).unwrap()
    }

    pub fn try_get_pt(&self, level: PageTableLevel) -> Option<*mut PageTable> {
        let index = ptl_to_index(level);
        self.0[index]
    }

    pub fn pairs(&self) -> Option<PageTablesPairIterator> {
        if self.0.iter().any(|page_table| page_table.is_none()) {
            return None;
        }

        Some(PageTablesPairIterator {
            page_tables: self.0.map(|page_table| page_table.unwrap()),
            parent_level: PageTableLevel::Two,
        })
    }
}

pub struct PageTablesPairIterator {
    page_tables: [*mut PageTable; AMOUNT_PAGE_TABLES],
    parent_level: PageTableLevel,
}

impl Iterator for PageTablesPairIterator {
    type Item = (*mut PageTable, *mut PageTable, PageTableLevel);

    fn next(&mut self) -> Option<Self::Item> {
        let parent_pt = self.page_tables[ptl_to_index(self.parent_level)];
        let child_pt =
            self.page_tables[ptl_to_index(self.parent_level.next_lower_level().unwrap())];
        let parent_pt_level = self.parent_level;

        self.parent_level = next_higher_level(self.parent_level).unwrap();

        Some((parent_pt, child_pt, parent_pt_level))
    }
}
