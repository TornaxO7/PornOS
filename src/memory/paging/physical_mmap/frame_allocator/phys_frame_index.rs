//! Contains the type-safety implemenation of the index which points to the array of bits.
use crate::memory::types::Byte;

const AMOUNT_BYTES: usize = 8;

/// A helper struct which can be used as an index, if the frames are stored in an array.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PhysFrameIndex(pub u64);

impl IntoIterator for PhysFrameIndex {
    type Item = Byte;

    type IntoIter = FrameIndexByteIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::from(self)
    }
}

/// A helper struct to be able to iterato through the bytes of the FrameIndex value.
///
/// # Note
/// The bytes are gonna be in **little endian**!
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameIndexByteIterator {
    /// Contains the bytes of the `FrameArrayIndex` value.
    bytes: [Byte; AMOUNT_BYTES],
    /// The current byte-index which should be returned next.
    index: usize,
}

impl From<PhysFrameIndex> for FrameIndexByteIterator {
    fn from(frame_index: PhysFrameIndex) -> Self {
        Self {
            bytes: frame_index.0.to_le_bytes().map(Byte::new),
            index: 0,
        }
    }
}

impl Iterator for FrameIndexByteIterator {
    type Item = Byte;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= AMOUNT_BYTES {
            None
        } else {
            let ret_value = Some(self.bytes[self.index]);
            self.index += 1;

            ret_value
        }
    }
}
