use x86_64::structures::paging::{
    page_table::{PageTableEntry, PageTableLevel},
    PageTable, PageTableIndex, Page,
};

use super::SIMP;

const AMOUNT_PAGE_TABLES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PTLevels([Option<*mut PageTable>; AMOUNT_PAGE_TABLES]);

impl PTLevels {
    pub unsafe fn new(p4: * mut PageTable) -> Self {
        let mut ptlevels = Self([None; AMOUNT_PAGE_TABLES]);
        ptlevels.set_pt(p4, PageTableLevel::Four);
        ptlevels
    }

    pub unsafe fn set_pt(&mut self, page_table: *mut PageTable, level: PageTableLevel) {
        self.0[level as usize] = Some(page_table);
    }

    pub unsafe fn get_pt(&self, level: PageTableLevel) -> *mut PageTable {
        self.0[level as usize].unwrap()
    }

    pub unsafe fn is_empty(&self, level: PageTableLevel) -> bool {
        (*self.get_pt(level)).iter().all(|entry| entry.is_unused())
    }

    pub unsafe fn free_pt(&self, parent_level: PageTableLevel, child_level: PageTableLevel, parent_index: PageTableIndex) {
        let parent_table = self.get_pt(parent_level);
        let child_table = self.get_pt(child_level);
    }

    pub unsafe fn clear_entry(&self, level: PageTableLevel, index: PageTableIndex) {
        (*self.get_pt(level))[index] = PageTableEntry::new();
    }
}
