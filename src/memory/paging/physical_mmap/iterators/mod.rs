mod useable;
mod kernel_and_modules;

use core::slice::Iter;

use limine::{LimineMemmapEntry, NonNullPtr};

pub use useable::UseableMemChunkIterator;
pub use kernel_and_modules::KernelAndModulesIterator;

pub struct MemChunkIterator(Iter<'static, NonNullPtr<LimineMemmapEntry>>);

impl MemChunkIterator {
    pub fn new() -> Self {
        Self(super::get_mmaps().into_iter())
    }
}

impl Iterator for MemChunkIterator {
    type Item = &'static NonNullPtr<LimineMemmapEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
