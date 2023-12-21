use limine::{MemmapEntry, MemoryMapEntryType, NonNullPtr};

use core::iter::Iterator;

use super::MemChunkIterator;

pub struct UseableMemChunkIterator(MemChunkIterator);

impl UseableMemChunkIterator {
    pub fn new() -> Self {
        Self(MemChunkIterator::new())
    }
}

impl Iterator for UseableMemChunkIterator {
    type Item = &'static NonNullPtr<MemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap: &Self::Item| mmap.typ == MemoryMapEntryType::Usable)
    }
}
