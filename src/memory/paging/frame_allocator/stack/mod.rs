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
mod iterator;
mod page_frame_allocator;

#[cfg(feature = "test")]
mod test;
use core::fmt::Debug;

#[cfg(feature = "test")]
pub use test::tests;

type StackIndex = u64;

use x86_64::{
    structures::paging::{PageSize, Size4KiB},
    PhysAddr,
};

use crate::memory::{types::Bytes, HHDM};

// use super::FrameManager;

/// The size of a pointer in bytes.
const POINTER_SIZE: Bytes = Bytes::new(8);

/// A Page-Frame Allocator which includes pointer to 4KiB big Page-Frames.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack {
    start: PhysAddr,
    len: u64,
    capacity: u64,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            start: PhysAddr::zero(),
            len: 0,
            capacity: 0,
        }
    }
}

impl Stack {
    pub const PAGE_SIZE: usize = Size4KiB::SIZE as usize;

    /// # Returns
    /// - `Some<FrameIndex>`: The frame index of the frame which isn't used yet.
    /// - `None`: If there are no free frames anymore.
    #[must_use]
    pub fn pop(&mut self) -> Option<PhysAddr> {
        if self.len > 0 {
            let value = self.get_entry_value(self.len - 1).unwrap();
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
    pub fn push(&mut self, entry_value: PhysAddr) -> bool {
        let exceeds_capacity = self.len >= self.capacity;
        if exceeds_capacity {
            return false;
        }

        let new_entry_ptr = self.get_entry_ptr_mut(self.len - 1).unwrap();

        unsafe {
            *new_entry_ptr = entry_value.as_u64();
        }

        // SAFETY: Check if self.len exceeds self.capacity already done before
        self.len += 1;
        true
    }

    /// Return the value at the given index of the stack.
    ///
    /// * `index`: The index of the stack where to get the value from.
    pub fn get_entry_value(&self, index: StackIndex) -> Option<PhysAddr> {
        self.get_entry_ptr(index)
            .map(|ptr| unsafe {PhysAddr::new(*ptr)})
    }

    /// Returns a pointer to the entry with the given index in the stack.
    ///
    /// * `index`: The entry index to the entry where the pointer should point to.
    fn get_entry_ptr(&self, index: StackIndex) -> Option<*const u64> {
        if index >= self.len {
            return None;
        }

        let entry_phys_addr = self.start + (*POINTER_SIZE) * index;
        let entry_virt_addr = *HHDM + entry_phys_addr.as_u64();
        Some(entry_phys_addr.as_u64() as *const u64)
    }

    /// Returns a mut pointer to the entry with the given index in the stack.
    ///
    /// * `index`: The entry index to the entry where the pointer should point to.
    fn get_entry_ptr_mut(&self, index: StackIndex) -> Option<*mut u64> {
        if index >= self.len {
            return None;
        }

        let entry_phys_addr = self.start + (*POINTER_SIZE) * index;
        let entry_virt_addr = *HHDM + entry_phys_addr.as_u64();
        Some(entry_phys_addr.as_u64() as *mut u64)
    }
}
