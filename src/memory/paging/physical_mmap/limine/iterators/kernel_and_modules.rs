use limine::{MemmapEntry, MemoryMapEntryType, NonNullPtr};

use super::MemChunkIterator;

pub struct KernelAndModulesIterator(MemChunkIterator);

impl Iterator for KernelAndModulesIterator {
    type Item = &'static NonNullPtr<MemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .find(|&mmap: &Self::Item| mmap.typ == MemoryMapEntryType::KernelAndModules)
    }
}
