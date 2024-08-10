use core::iter::Iterator;

use limine::memory_map::{Entry, EntryType};

use super::MemChunkIterator;

pub struct UseableMemChunkIterator(MemChunkIterator);

impl UseableMemChunkIterator {
    pub fn new() -> Self {
        Self(MemChunkIterator::new())
    }
}

impl Iterator for UseableMemChunkIterator {
    type Item = &'static Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap: &Self::Item| mmap.entry_type == EntryType::USABLE)
    }
}
