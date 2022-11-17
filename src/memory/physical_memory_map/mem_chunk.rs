use limine::LimineMemmapEntry;

use crate::memory::{addr::PhysAddr, Bytes};

/// Represents a [memmap]/[memory chunk] which is given by limine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MemChunk {
    /// The starting adress of the memory chunk.
    pub base: PhysAddr,
    /// The size of the memory chunk/memmap.
    pub len: Bytes,
}

impl MemChunk {
    pub const fn new() -> Self {
        Self {
            base: PhysAddr::new(0),
            len: 0,
        }
    }
}

impl From<&LimineMemmapEntry> for MemChunk {
    fn from(entry: &LimineMemmapEntry) -> Self {
        Self {
            base: PhysAddr::new(entry.base),
            len: entry.len,
        }
    }
}
