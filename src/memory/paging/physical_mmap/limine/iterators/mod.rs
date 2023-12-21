mod kernel_and_modules;
mod useable;

use core::slice::Iter;

use limine::{MemmapEntry, NonNullPtr};

pub use {kernel_and_modules::KernelAndModulesIterator, useable::UseableMemChunkIterator};

pub struct MemChunkIterator(Iter<'static, NonNullPtr<MemmapEntry>>);

impl MemChunkIterator {
    pub fn new() -> Self {
        Self(super::get_mmaps().iter())
    }
}

impl Iterator for MemChunkIterator {
    type Item = &'static NonNullPtr<MemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
