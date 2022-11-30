mod useable;

use core::marker::PhantomData;

use limine::{LimineMemmapEntry, NonNullPtr};
use x86_64::structures::paging::PageSize;

use super::PhysMemMap;

impl<P: PageSize> PhysMemMap<P> {
    pub fn into_iter_mem_chunk(&self) -> MemChunkIterator<P> {
        MemChunkIterator::new(self.entry_count)
    }
}

pub struct MemChunkIterator<P: PageSize> {
    index: u64,
    entry_count: u64,
    size: PhantomData<P>,
}

impl<P: PageSize> MemChunkIterator<P> {
    pub fn new(entry_count: u64) -> Self {
        Self {
            index: 0,
            entry_count,
            size: PhantomData,
        }
    }
}

impl<P: PageSize> Iterator for MemChunkIterator<P> {
    type Item = &'static NonNullPtr<LimineMemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        let mmaps = PhysMemMap::<P>::get_mmaps();
        if self.index < self.entry_count {
            let mmap = &mmaps[self.index as usize];
            self.index += 1;
            return Some(mmap);
        }

        None
    }
}
