use limine::LimineMemmapEntry;

use crate::memory::{PhysAddr, Bytes};

/// Represents a [memmap]/[memory chunk] which is given by limine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Memmap {
    /// The starting adress of the memory chunk.
    pub base: PhysAddr,
    /// The size of the memory chunk/memmap.
    pub len: Bytes,
}

impl Default for Memmap {
    fn default() -> Self {
        Self {
            base: 0,
            len: 0,
        }
    }
}

impl From<&LimineMemmapEntry> for Memmap {
    fn from(entry: &LimineMemmapEntry) -> Self {
        Self {
            base: entry.base,
            len: entry.len,
        }
    }
}
