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
use core::{fmt::Debug, marker::PhantomData};

#[cfg(feature = "test")]
pub use test::tests;

type StackIndex = u64;

use x86_64::{
    structures::paging::{PageSize, PhysFrame},
    PhysAddr,
};

use crate::memory::types::Bytes;

use super::FrameManager;

/// The size of a pointer in bytes.
const POINTER_SIZE: Bytes = Bytes::new(8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack<P: PageSize + Send + Sync + Debug> {
    start: PhysAddr,
    len: u64,
    capacity: u64,
    psize: PhantomData<P>,
}

impl<P: PageSize + Send + Sync + Debug> Default for Stack<P> {
    fn default() -> Self {
        Self {
            start: PhysAddr::zero(),
            len: 0,
            capacity: 0,
            psize: PhantomData,
        }
    }
}

impl<P: PageSize + Send + Sync + Debug> Stack<P> {
    /// # Returns
    /// - `Some<FrameIndex>`: The frame index of the frame which isn't used yet.
    /// - `None`: If there are no free frames anymore.
    #[must_use]
    pub fn pop(&mut self) -> Option<PhysAddr> {
        if let Some(value_index) = self.len.checked_sub(1) {
            let value = self.get_entry(value_index).unwrap();
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

        let new_entry_ptr = {
            let phys_addr = self.get_entry_addr(self.len).unwrap();
            phys_addr.as_u64() as *mut u64
        };

        unsafe {
            *new_entry_ptr = entry_value.as_u64();
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
    fn get_entry(&self, index: StackIndex) -> Option<PhysAddr> {
        if index < self.len {
            if let Some(entry_addr) = self.get_entry_addr(index) {
                let entry_addr = entry_addr.as_u64() as *const u64;
                return Some(PhysAddr::new(unsafe { *entry_addr }));
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
    fn get_entry_addr(&self, index: u64) -> Option<PhysAddr> {
        if index <= self.len {
            let phys_addr = PhysAddr::new(self.start.as_u64() + (POINTER_SIZE * index).as_u64());
            Some(phys_addr)
        } else {
            None
        }
    }
}

impl<P: PageSize + Send + Sync + Debug> FrameManager<P> for Stack<P> {
    fn get_free_frame(&mut self) -> Option<PhysFrame<P>> {
        self.pop()
            .map(|phys_addr| PhysFrame::from_start_address(phys_addr).unwrap())
    }

    fn free_frame(&mut self, frame: PhysFrame<P>) {
        assert!(self.push(frame.start_address()));
    }
}
