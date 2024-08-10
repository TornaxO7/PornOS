mod kernel_and_modules;
mod useable;

use core::slice::Iter;

use limine::memory_map::Entry;

pub use {kernel_and_modules::KernelAndModulesIterator, useable::UseableMemChunkIterator};

pub struct MemChunkIterator(Iter<'static, &'static Entry>);

impl MemChunkIterator {
    pub fn new() -> Self {
        Self(super::get_mmaps().iter())
    }
}

impl Iterator for MemChunkIterator {
    type Item = &'static Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|&entry| entry)
    }
}
