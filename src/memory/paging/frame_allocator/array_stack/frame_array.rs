use crate::memory::{VirtAddr, Bytes};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FrameArray {
    pub start: VirtAddr,
    pub len: Bytes,
}

impl FrameArray {
    pub fn new(start: VirtAddr, amount_page_frames: u64) -> Self {
        let entry = FrameArrayEntry { used: false };

        for index in 0..amount_page_frames {
            let addr = (start + index * FrameArrayEntry::SIZE) as * mut FrameArrayEntry;

            unsafe {
                *addr = entry;
            }
        }

        Self {
            start,
            len: amount_page_frames * FrameArrayEntry::SIZE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct FrameArrayEntry {
    used: bool,
}

impl FrameArrayEntry {
    const SIZE: Bytes = core::mem::size_of::<Self>() as Bytes;
}
