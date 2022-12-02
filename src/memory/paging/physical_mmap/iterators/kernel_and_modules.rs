use limine::{LimineMemmapEntry, LimineMemoryMapEntryType, NonNullPtr};
use x86_64::structures::paging::PageSize;

use crate::memory::paging::PhysMemMap;

use super::MemChunkIterator;

impl<P: PageSize> PhysMemMap<P> {
    pub fn into_iter_kernel_and_modules(&self) -> KernelAndModulesIterator<P> {
        KernelAndModulesIterator::new(self.entry_count)
    }
}

pub struct KernelAndModulesIterator<P: PageSize>(MemChunkIterator<P>);

impl<P: PageSize> KernelAndModulesIterator<P> {
    pub fn new(entry_count: u64) -> Self {
        Self(MemChunkIterator::new(entry_count))
    }
}

impl<P: PageSize> Iterator for KernelAndModulesIterator<P> {
    type Item = &'static NonNullPtr<LimineMemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap| mmap.typ == LimineMemoryMapEntryType::KernelAndModules)
    }
}
