use limine::LimineMemmapEntry;

use crate::memory::{PhysAddr, Bytes};

/// Represents a [memmap]/[memory chunk] which is given by limine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MemmapEntry {
    /// The starting adress of the memory chunk.
    pub base: PhysAddr,
    /// The size of the memory chunk/memmap.
    pub len: Bytes,
}

impl From<&LimineMemmapEntry> for MemmapEntry {
    fn from(entry: &LimineMemmapEntry) -> Self {
        Self {
            base: entry.base,
            len: entry.len,
        }
    }
}
