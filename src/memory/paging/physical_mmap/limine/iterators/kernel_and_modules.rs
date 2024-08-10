use limine::memory_map::{Entry, EntryType};

use super::MemChunkIterator;

pub struct KernelAndModulesIterator(MemChunkIterator);

impl Iterator for KernelAndModulesIterator {
    type Item = &'static Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap: &Self::Item| mmap.entry_type == EntryType::KERNEL_AND_MODULES)
    }
}
