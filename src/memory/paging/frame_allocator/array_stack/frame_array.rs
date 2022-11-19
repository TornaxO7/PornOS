/// TODO: Refactor the array to hold bits to mark the frames as used or not!

use alloc::vec::IntoIter;

use crate::memory::{
    paging::{PageSize, PhysLinearAddr, PhysMemMap},
    types::{Bytes, Byte},
    VirtAddr,
};

use super::{FrameIndex, FrameIndexByteIterator};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameArray {
    pub start: VirtAddr,
    pub len: Bytes,
}

impl Default for FrameArray {
    fn default() -> Self {
        Self {
            start: VirtAddr::zero(),
            len: Bytes::new(0),
        }
    }
}

impl FrameArray {
    pub fn new(start: PhysLinearAddr, phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        let start = start.align_up(FrameArrayEntry::SIZE.as_u64());
        let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);

        let entry = FrameArrayEntry { used: false };

        for offset_multilplier in 0..amount_page_frames {
            let offset = FrameArrayEntry::SIZE * offset_multilplier;

            if !phys_mmap.write_value(entry, start + offset) {
                panic!("Not enough useable RAM");
            }
        }

        Self {
            start,
            len: Bytes::new(amount_page_frames * FrameArrayEntry::SIZE.as_u64()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct FrameArrayEntry(bool);

impl FrameArrayEntry {
    const SIZE: Bytes = Bytes::new(core::mem::size_of::<Self>() as u64);
}

impl IntoIterator for FrameArrayEntry {
    type Item = Byte;

    type IntoIter = FrameIndexByteIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::from(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameArrayEntryByteIterator {
    bytes: [Byte; FrameArrayEntry::SIZE.as_u64() as usize],
    index: usize,
}

impl Iterator for FrameArrayEntryByteIterator {
    type Item = Byte;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= FrameArrayEntry::SIZE.as_usize() {
            None
        } else {
            let return_value = Some(self.bytes[self.index]);
            self.index += 1;

            return_value
        }
    }
}

impl From<FrameArrayEntry> for FrameArrayEntryByteIterator {
    fn from(entry: FrameArrayEntry) -> Self {
        Self {
            bytes: 
        }
    }
}
