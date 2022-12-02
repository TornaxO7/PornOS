use crate::memory::paging::PhysMemMap;
use limine::{LimineMemmapEntry, LimineMemoryMapEntryType, NonNullPtr};

use x86_64::structures::paging::PageSize;

use core::iter::Iterator;

use super::MemChunkIterator;

impl<P: PageSize> PhysMemMap<P> {
    /// # Returns
    /// Returns an iterator through all useable memory chunks.
    pub fn into_iter_useable(&self) -> UseableMemChunkIterator<P> {
        UseableMemChunkIterator::new(self.entry_count)
    }
}

pub struct UseableMemChunkIterator<P: PageSize>(MemChunkIterator<P>);

impl<P: PageSize> UseableMemChunkIterator<P> {
    pub fn new(entry_count: u64) -> Self {
        Self(MemChunkIterator::new(entry_count))
    }
}

impl<P: PageSize> Iterator for UseableMemChunkIterator<P> {
    type Item = &'static NonNullPtr<LimineMemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap| mmap.typ == LimineMemoryMapEntryType::Usable)
    }
}
