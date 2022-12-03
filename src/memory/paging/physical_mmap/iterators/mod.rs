mod useable;
mod kernel_and_modules;


use limine::{LimineMemmapEntry, NonNullPtr};

pub use useable::UseableMemChunkIterator;
pub use kernel_and_modules::KernelAndModulesIterator;

pub struct MemChunkIterator {
    index: u64,
    entry_count: u64,
}

impl MemChunkIterator {
    pub fn new() -> Self {
        Self {
            index: 0,
            entry_count: super::get_memmap_response().entry_count,
        }
    }
}

impl Iterator for MemChunkIterator {
    type Item = &'static NonNullPtr<LimineMemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        let mmaps = super::get_mmaps();
        if self.index < self.entry_count {
            let mmap = &mmaps[self.index as usize];
            self.index += 1;
            return Some(mmap);
        }

        None
    }
}
