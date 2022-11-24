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
mod frame_index;

use x86_64::VirtAddr;

use crate::{memory::{
    paging::{PageSize, PhysLinearAddr, PhysMemMap},
    types::Bytes,
}, println};

use self::frame_index::FrameIndex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack {
    start: VirtAddr,
    len: Bytes,
    capacity: Bytes,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            start: VirtAddr::zero(),
            len: Bytes::new(0),
            capacity: Bytes::new(0),
        }
    }
}

impl Stack {
    /// Creates a new frame-stack with the given arguments.
    pub fn new(start: PhysLinearAddr, phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        let start = start.align_up(FrameIndex::SIZE.as_u64());
        let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);
        let mut frame_index = FrameIndex(amount_page_frames);

        // init the stack entries. Starting from frame index <amount_page_frames> up to 0
        for offset_multiplier in 0..amount_page_frames {
            let offset = FrameIndex::SIZE * offset_multiplier;
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
    pub fn pop(&mut self) -> Option<FrameIndex> {
        if let Some(new_len) = self.len.checked_sub(FrameIndex::SIZE.as_u64()) {
            self.len = Bytes::new(new_len);
        } else {
            println!("STACK IS EMPTY");
            return None;
        }

        let frame_index = {
            let ptr: * const u64 = (self.start.as_u64() + self.len.as_u64()) as * const u64;
            let frame_index_value = unsafe {*ptr};
            FrameIndex(frame_index_value)
        };

        Some(frame_index)
    }

    /// Pushes the given frame index onto the stack.
    ///
    /// # Returns
    /// `true` if the given frame index could be pushed successfully.
    /// `false` if the stack is already full.
    ///
    /// # WARNING
    /// You have to make sure that the given frame index ***is*** free! Otherwise Undefined
    /// Behaviour will be your OS.
    #[must_use]
    pub fn push(&mut self, frame_index: FrameIndex) -> bool {
        let exceeds_capacity = (self.start.as_u64() + self.len.as_u64()) >= self.capacity.as_u64();
        if exceeds_capacity {
            return false;
        }

        let ptr: * mut u64 = (self.start.as_u64() + self.len.as_u64()) as * mut u64;
        unsafe {
            *ptr = frame_index.0;
        }

        // SAFETY: Check if self.len exceeds self.capacity already done before
        self.len += FrameIndex::SIZE;
        true
    }

    /// Returns the capacity in bytes of the stack.
    pub fn get_capacity(&self) -> Bytes {
        self.capacity
    }
}
