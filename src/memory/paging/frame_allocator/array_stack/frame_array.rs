//! Contains the implementation of the frame array.
//! In memory in looks as follows:
//! ```
//! |-----------------------------------------|
//! |u8|u8|u8|...                             |
//! |-----------------------------------------|
//! ```
//! You need to imagine the array as a bit-array rather than a `u8`-array.
//! Each bitflag marks the frame as used or not.
//! 
//! # Example
//! If the `8th` bit is 1 then the `8th` frame is already used.
use crate::memory::{
    paging::{PageSize, PhysLinearAddr, PhysMemMap},
    types::{Bytes, Byte},
    VirtAddr,
};

/// Holds the implementation of the frame array.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameArray {
    /// Holds the starting virtual address of the array.
    start: VirtAddr,
    /// Holds the length (in bytes) of the array.
    len: Bytes,
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
    /// Creates a new frame array with the given arguments
    pub fn new(start: PhysLinearAddr, phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        let start = start.align_up(FrameArrayEntry::SIZE.as_u64());
        let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);

        let entry = FrameArrayEntry::empty();

        for offset_multilplier in 0..amount_page_frames {
            let offset = FrameArrayEntry::SIZE * offset_multilplier;

            if !phys_mmap.write_value(entry, start + offset.as_u64()) {
                panic!("Not enough useable RAM");
            }
        }

        Self {
            start: phys_mmap.convert_to_virt(&start).unwrap(),
            len: Bytes::new(amount_page_frames * FrameArrayEntry::SIZE.as_u64()),
        }
    }
}

bitflags::bitflags! {
    struct FrameArrayEntry: u8 {
        const F1 = 0b0000_0001;
        const F2 = 0b0000_0010;
        const F3 = 0b0000_0100;
        const F4 = 0b0000_1000;
        const F5 = 0b0001_0000;
        const F6 = 0b0001_0000;
        const F7 = 0b0010_0000;
        const F8 = 0b0100_0000;
        const F9 = 0b1000_0000;
    }
}

impl FrameArrayEntry {
    const SIZE: Bytes = Bytes::new(core::mem::size_of::<Self>() as u64);
}

struct FrameArrayEntryIterator {
    byte: Byte,
    index: usize,
}

impl IntoIterator for FrameArrayEntry {
    type Item = Byte;

    type IntoIter = FrameArrayEntryIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            byte: Byte::new(self.bits()),
            index: 0,
        }
    }
}

impl Iterator for FrameArrayEntryIterator {
    type Item = Byte;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 1 {
            None
        } else {
            Some(self.byte)
        }
    }
}
