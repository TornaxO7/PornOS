use alloc::vec::Vec;

use super::{mem_chunk::MemChunk, PhysMemMapper};

pub static MAIN_MEMMAP: MainMemmap = MainMemmap(Vec::new());

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainMemmap(Vec<MemChunk>);

impl PhysMemMapper for MainMemmap {
    fn useable_mem(&self) -> crate::memory::Bytes {
        todo!()
    }

    fn init(&mut self) {
    }
}
