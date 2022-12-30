use limine::{LimineMemmapEntry, LimineMemoryMapEntryType, NonNullPtr};

use core::iter::Iterator;

use super::MemChunkIterator;

pub struct UseableMemChunkIterator(MemChunkIterator);

impl UseableMemChunkIterator {
    pub fn new() -> Self {
        Self(MemChunkIterator::new())
    }
}

impl Iterator for UseableMemChunkIterator {
    type Item = &'static NonNullPtr<LimineMemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap| mmap.typ == LimineMemoryMapEntryType::Usable)
    }
}
