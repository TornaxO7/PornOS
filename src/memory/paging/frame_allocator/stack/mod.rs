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
use x86_64::{PhysAddr, VirtAddr};

use crate::{
    memory::{
        paging::{frame_allocator::PhysFrameIndex, PageSize, PhysLinearAddr, PhysMemMap},
        types::Bytes,
    },
    print, println,
};

use super::{frame::Frame, FrameManager};

const POINTER_SIZE: Bytes = Bytes::new(8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack {
    start: PhysAddr,
    len: Bytes,
    capacity: Bytes,
    page_size: PageSize,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            start: PhysAddr::zero(),
            len: Bytes::new(0),
            capacity: Bytes::new(0),
            page_size: PageSize::Page4KB,
        }
    }
}

impl Stack {
    // TODO: Check for safety again
    /// Creates a new frame-stack with the given arguments.
    pub fn new(phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        print!("Using Frame-Allocator-Stack ... ");
        let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);

        let (stack_start, stack_end) = {
            let needed_free_space = POINTER_SIZE * amount_page_frames;

            // FUTURE: It could happen, that we'll get the last frame because the other frames might
            // be too small....
            let start = phys_mmap
                .get_frame(PhysAddr::zero(), page_size, needed_free_space)
                .unwrap();
            // SAFETY: `start` has definetely the needed free space!
            let end = start + needed_free_space.as_u64();

            (start, end)
        };

        for offset_multiplier in 0..amount_page_frames {
            let offset = POINTER_SIZE
                .as_u64()
                .checked_mul(offset_multiplier)
                .unwrap();

            let entry_start_addr: *mut u64 = (stack_start.as_u64() + offset) as *mut u64;
            let phys_frame_start_addr: PhysAddr = {
                // the amount of bytes which the already pushed-frames already reserved
                let reserved_bytes = page_size.size() * offset;
                // FUTURE: Maybe instead of starting to collect the frames *after* the stack,
                // include the frames which the stack uses as well!
                let phys_addr_start = PhysAddr::new(reserved_bytes.as_u64() + stack_end.as_u64());
                phys_mmap
                    .get_frame(phys_addr_start, page_size, page_size.size())
                    .unwrap()
            };

            unsafe {
                *entry_start_addr = phys_frame_start_addr.as_u64();
            }
        }

        let capacity = page_size.size() * amount_page_frames;

        println!("OK");

        Self {
            start: stack_start,
            len: capacity,
            capacity,
            page_size,
        }
    }

    fn get_used_bytes(&self) -> Bytes {
        PhysFrameIndex::SIZE * self.len.as_u64()
    }

    /// # Returns
    /// - `Some<FrameIndex>`: The frame index of the frame which isn't used yet.
    /// - `None`: If there are no free frames anymore.
    #[must_use]
    pub fn pop(&mut self) -> Option<PhysFrameIndex> {
        if let Some(new_len) = self.len.checked_sub(PhysFrameIndex::SIZE.as_u64()) {
            self.len = Bytes::new(new_len);
        } else {
            println!("STACK IS EMPTY");
            return None;
        }

        let frame_index = {
            let ptr: *const u64 = (self.start.as_u64() + self.len.as_u64()) as *const u64;
            let frame_index_value = unsafe { *ptr };
            PhysFrameIndex(frame_index_value)
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
    pub fn push(&mut self, frame_index: PhysFrameIndex) -> bool {
        let exceeds_capacity = (self.start.as_u64() + self.len.as_u64()) >= self.capacity.as_u64();
        if exceeds_capacity {
            return false;
        }

        let ptr: *mut u64 = (self.start.as_u64() + self.len.as_u64()) as *mut u64;
        unsafe {
            *ptr = frame_index.0;
        }

        // SAFETY: Check if self.len exceeds self.capacity already done before
        self.len += PhysFrameIndex::SIZE;
        true
    }

    /// Returns the capacity in bytes of the stack.
    pub fn get_capacity(&self) -> Bytes {
        self.capacity
    }
}

impl FrameManager for Stack {
    fn get_free_frame(&mut self) -> Option<Frame> {
        todo!()
    }

    fn free_frame(&mut self, _addr: VirtAddr) {
        todo!()
    }
}
