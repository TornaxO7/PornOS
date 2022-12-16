use limine::{LimineMemmapEntry, LimineMemoryMapEntryType, NonNullPtr};

use core::iter::Iterator;

use super::MemChunkIterator;

pub struct LimineBootloaderReclaimable(MemChunkIterator);

impl LimineBootloaderReclaimable {
    pub fn new() -> Self {
        Self(MemChunkIterator::new())
    }
}

impl Iterator for LimineBootloaderReclaimable {
    type Item = &'static NonNullPtr<LimineMemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap| mmap.typ == LimineMemoryMapEntryType::BootloaderReclaimable)
    }
}
