//! This holds the stack implementation which looks in the memory as follows:
//! ```
//! |----------------------------------------------------------|
//! |u64|u64|u64|u64|...                                       |
//! |----------------------------------------------------------|
//! ```
//! Each `u64` represents an index for the array and so a single frame.
//! All indexes in the stack are pointing to a frame which is *free*.
//! So if you need a new frame is the equal operation of a pop from the stack
//! and register a freed frame is basically a push.
use crate::memory::{
    paging::{PageSize, PhysLinearAddr, PhysMemMap},
    types::Bytes,
    VirtAddr,
};

use super::FrameArrayIndex;

/// The memory structure of the stack.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameStack {
    /// The address where the stack starts
    start: VirtAddr,
    /// The length of the stack
    len: Bytes,
    /// The capacity of the stack
    capacity: Bytes,
}

impl Default for FrameStack {
    fn default() -> Self {
        Self {
            start: VirtAddr::zero(),
            len: Bytes::new(0),
            capacity: Bytes::new(0),
        }
    }
}

impl FrameStack {
    /// Creates a new frame-stack with the given arguments.
    pub fn new(start: PhysLinearAddr, phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        let start = start.align_up(FrameArrayIndex::SIZE.as_u64());
        let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);
        let mut frame_index = FrameArrayIndex(amount_page_frames);

        // init the stack entries. Starting from frame index <amount_page_frames> up to 0
        for offset_multiplier in 0..amount_page_frames {
            let offset = FrameArrayIndex::SIZE * offset_multiplier;
            if !phys_mmap.write_value(frame_index, start + offset.as_u64()) {
                panic!("Not enough useable RAM for frames :(");
            }
            frame_index.0 -= 1;
        }

        let capacity = page_size.size() * amount_page_frames;

        Self {
            start: phys_mmap.convert_to_virt(&start).unwrap(),
            len: capacity,
            capacity,
        }
    }

    /// # Returns
    /// - `Some<FrameIndex>`: The frame index of the frame which isn't used yet.
    /// - `None`: If there are no free frames anymore.
    #[must_use]
    pub fn pop(&mut self) -> Option<FrameArrayIndex> {
        todo!()
    }

    /// Pushes the given frame index onto the stack.
    ///
    /// # WARNING
    /// You have to make sure that the given frame index ***is*** free! Otherwise Undefined
    /// Behaviour will be your OS.
    #[must_use]
    pub fn push(&mut self, frame_index: FrameArrayIndex) {
        todo!()
    }

    /// Returns the capacity in bytes of the stack.
    pub fn get_capacity(&self) -> Bytes {
        self.capacity
    }
}
