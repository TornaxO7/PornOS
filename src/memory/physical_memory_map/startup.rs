use limine::{LimineMemmapEntry, LimineMemoryMapEntryType};

use crate::memory::Bytes;

use super::{mem_chunk::MemChunk, PhysMemMapper};

pub static mut STARTUP_MMAP: StartupMmap = StartupMmap {
    entries: [MemChunk::new(); StartupMmap::AMOUNT_ENTRIES],
    len: 0,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StartupMmap {
    entries: [MemChunk; Self::AMOUNT_ENTRIES],
    /// The amount of valid entries.
    pub len: usize,
}

impl PhysMemMapper for StartupMmap {
    fn useable_mem(&self) -> Bytes {
        let mut size: Bytes = 0;
        for index in 0..self.len {
            size = size.saturating_add(self.entries[index].len);
        }

        size
    }

    fn init(&mut self) {
        self.collect_entries();
    }
}

impl StartupMmap {
    /// The maximal amount of valid entries in this struct.
    pub const AMOUNT_ENTRIES: usize = 10;

    /// Returns the memmap at the given index.
    ///
    /// # Returns
    /// `Some(...)`: If the given index is valid
    /// `None`: If `index > self.len`.
    pub fn get(&self, index: usize) -> Option<&MemChunk> {
        if index < self.len {
            Some(&self.entries[index])
        } else {
            None
        }
    }

    /// Collect all useable memory chunks which are collected by limine.
    fn collect_entries(&mut self) {
        let response = super::MEMMAP_REQUEST.get_response().get().unwrap();
        for index in 0..response.entry_count {
            let entry: &LimineMemmapEntry = &response.memmap()[index as usize];
            if LimineMemoryMapEntryType::Usable == entry.typ && !self.add(entry) {
                break;
            }
        }
    }

    /// Tries to add the given limine entry to the this struct.
    ///
    /// # Returns
    /// `true`: If the entry could be added
    /// `false`: If there is no free slot anymore and the entry couldn't be added.
    #[must_use]
    fn add(&mut self, limine_entry: &LimineMemmapEntry) -> bool {
        if self.len < Self::AMOUNT_ENTRIES {
            self.entries[self.len] = MemChunk::from(limine_entry);
            self.len += 1;

            true
        } else {
            false
        }
    }
}

impl Default for StartupMmap {
    fn default() -> Self {
        Self {
            entries: [MemChunk::default(); Self::AMOUNT_ENTRIES],
            len: 0,
        }
    }
}
