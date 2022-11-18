use crate::memory::{Bytes, VirtAddr, physical_memory_mapper::IntoBytes, Byte};

use super::FrameIndex;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FrameStack {
    pub start: VirtAddr,
    pub offset: Bytes,
    pub len: Bytes,
}

impl FrameStack {
    pub fn new(start: VirtAddr, amount_page_frames: u64) -> Self {
        let mut frame_index = FrameIndex(0);

        // set the frame indexes in the stack
        for index in 0..amount_page_frames {
            let addr = (start + index * FrameIndex::SIZE as u64) as *mut FrameIndex;
            unsafe {
                *addr = frame_index;
            }
            frame_index.0 += 1;
        }

        let offset = amount_page_frames * FrameIndex::SIZE as u64;
        Self { start, offset, len: offset }
    }

    pub fn pop(&mut self) -> FrameIndex {
        todo!()
    }
}
