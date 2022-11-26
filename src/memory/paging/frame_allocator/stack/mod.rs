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
mod init;
#[cfg(feature = "test")]
mod test;
#[cfg(feature = "test")]
pub use test::tests;

use x86_64::{PhysAddr, VirtAddr};

use crate::memory::{paging::PageSize, types::Bytes};

use super::{frame::Frame, FrameManager};

/// The size of a pointer in bytes.
const POINTER_SIZE: Bytes = Bytes::new(8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack {
    start: PhysAddr,
    len: u64,
    capacity: u64,
    page_size: PageSize,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            start: PhysAddr::zero(),
            len: 0,
            capacity: 0,
            page_size: PageSize::Page4KB,
        }
    }
}

impl Stack {
    /// # Returns
    /// - `Some<FrameIndex>`: The frame index of the frame which isn't used yet.
    /// - `None`: If there are no free frames anymore.
    #[must_use]
    pub fn pop(&mut self) -> Option<PhysAddr> {
        if let Some(value_index) = self.len.checked_sub(1) {
            let value = PhysAddr::new(self.get_entry(value_index).unwrap());
            self.len -= 1;
            Some(value)
        } else {
            None
        }
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
    pub fn push(&mut self, frame_addr: PhysAddr) -> bool {
        let exceeds_capacity = self.len >= self.capacity;
        if exceeds_capacity {
            return false;
        }

        let new_entry_ptr = {
            let phys_addr = self.get_entry_addr(self.len).unwrap();
            phys_addr.as_u64() as *mut u64
        };

        unsafe {
            *new_entry_ptr = frame_addr.as_u64();
        }

        // SAFETY: Check if self.len exceeds self.capacity already done before
        self.len += 1;
        true
    }

    /// Returns the value at the given index in the stack.
    /// `0` points to the bottom of the stack.
    ///
    /// # Return
    /// - `Some<u64>`: The value at the given index.
    /// - `None`: If the given index exceeds the current length of the stack.
    pub fn get_entry(&self, index: u64) -> Option<u64> {
        if index < self.len {
            if let Some(entry_addr) = self.get_entry_addr(index) {
                let entry_addr = entry_addr.as_u64() as *const u64;
                return Some(unsafe { *entry_addr });
            }
        }
        None
    }

    /// Returns the physical address of the entry with the given index.
    /// Index 0 starts from the bottom of the stack.
    ///
    /// # Returns
    /// - `Some<PhysAddr>`: If the index is valid (<= self.len).
    /// - `None`: If the index is greater than the amount of valid entries.
    pub fn get_entry_addr(&self, index: u64) -> Option<PhysAddr> {
        if index <= self.len {
            let phys_addr = PhysAddr::new(self.start.as_u64() + (POINTER_SIZE * index).as_u64());
            Some(phys_addr)
        } else {
            None
        }
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
