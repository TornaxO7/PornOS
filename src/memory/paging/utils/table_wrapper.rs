use x86_64::structures::paging::{
    page_table::PageTableEntry, FrameAllocator, PageTable, PageTableFlags, PageTableIndex,
    PhysFrame,
};

use crate::{memory::paging::frame_allocator::FRAME_ALLOCATOR, println};

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
        Self {
            ptr,
        }
    }

    /// Creates a new entry for the page table by allocating a new page-frame and inserting it's
    /// physical address at the given index.
    ///
    /// * `index`: The table-index where to write the physical starting address of the page-frame.
    pub fn create_entry(&mut self, index: PageTableIndex) -> PageTableEntry {
        let page_table_entry_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        let new_page_frame = FRAME_ALLOCATOR.write().allocate_frame().unwrap();

        let new_entry = {
            let mut entry = PageTableEntry::new();
            entry.set_addr(new_page_frame.start_address(), page_table_entry_flags);
            entry
        };

        self.set_entry(index, new_entry.clone());

        new_entry
    }

    /// Updates the entry at the given index in the page table and also writes that into the memory.
    pub fn set_entry(&mut self, index: PageTableIndex, entry: PageTableEntry) {
        unsafe {
            (*self.ptr)[index] = entry;
        }
    }

    pub fn set_page_frame(&mut self, index: PageTableIndex, page_frame: PhysFrame) {
        let page_table_entry_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        let entry = {
            let mut entry = PageTableEntry::new();
            entry.set_addr(page_frame.start_address(), page_table_entry_flags);
            entry
        };

        self.set_entry(index, entry);
    }

    pub fn get_entry(&self, index: PageTableIndex) -> &PageTableEntry {
        unsafe {
            &(*self.ptr)[index]
        }
    }
}
