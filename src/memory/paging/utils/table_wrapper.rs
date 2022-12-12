use x86_64::structures::paging::{
    page_table::PageTableEntry, FrameAllocator, PageTable, PageTableFlags, PageTableIndex,
    PhysFrame,
};

use crate::memory::paging::frame_allocator::FRAME_ALLOCATOR;

/// A little helper struct which should help to make more readable code by
/// taking care of writing into the table and reading from it.
#[derive(Debug, Clone)]
pub struct TableWrapper {
    ptr: *mut PageTable,
}

impl TableWrapper {
    /// Creates a new table wrapper of the given start address of a table.
    ///
    /// * `ptr`: A pointer to a page table which the struct should wrap.
    pub fn new(ptr: *mut PageTable) -> Self {
        Self { ptr }
    }

    /// Updates the entry at the given index in the page table and also writes that into the memory.
    pub fn set_entry(&mut self, index: PageTableIndex, entry: PageTableEntry) {
        unsafe {
            (*self.ptr)[index] = entry;
        }
    }

    /// Inserts the given page frame, if available, at the corresponding index in the current page
    /// table with the given flags.
    ///
    /// * `index`: The entry-index where to put the page-frame.
    /// * `page_frame`: The page frame which should be stored in the given index.
    /// * `flags`: The flags of the new page-frame entry.
    pub fn set_page_frame(
        &mut self,
        index: PageTableIndex,
        page_frame: Option<PhysFrame>,
        flags: PageTableFlags,
    ) {
        let page_frame =
            page_frame.unwrap_or_else(|| FRAME_ALLOCATOR.write().allocate_frame().unwrap());

        let entry = {
            let mut entry = PageTableEntry::new();
            entry.set_addr(page_frame.start_address(), flags);
            entry
        };

        self.set_entry(index, entry);
    }

    pub fn get_entry(&self, index: PageTableIndex) -> &PageTableEntry {
        unsafe { &(*self.ptr)[index] }
    }
}

impl TableWrapper {}
