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
mod iterators;
mod page_frame_allocator;

use core::mem::size_of;

type StackIndex = u64;

use x86_64::{
    structures::paging::{PageSize, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

use crate::memory::{paging::mem_structure::MEM_STRUCTURE, types::Bytes};

/// The size of a pointer in bytes.
const POINTER_SIZE: Bytes = Bytes::new(size_of::<*const u8>() as u64);

/// The different errors which can appear when you try to push a value onto the stack.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StackPushError {
    /// The stack is already full
    FullStack,
    /// The given entry is not aligned
    EntryNotAligned,
}

/// A Page-Frame Allocator which includes pointer to 4KiB big Page-Frames.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack {
    /// Points where the stack starts
    pub start: PhysAddr,

    /// The current amount of available page frames
    len: u64,

    /// The maximum amount of available page frames.
    capacity: u64,

    /// The amount of page frames which the stack itself uses to store the information.
    amount_used_page_frames: u64,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            start: PhysAddr::zero(),
            len: 0,
            capacity: 0,
            amount_used_page_frames: 0,
        }
    }
}

impl Stack {
    pub const PAGE_SIZE: Bytes = Bytes::new(Size4KiB::SIZE);

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
    ///
    /// # WARNING
    /// You have to make sure that the given frame index ***is*** free! Otherwise Undefined
    /// Behaviour will be your OS.
    pub fn push(&mut self, entry_value: PhysAddr) -> Result<(), StackPushError> {
        let exceeds_capacity = self.len >= self.capacity;
        if exceeds_capacity {
            return Err(StackPushError::FullStack);
        } else if !entry_value.is_aligned(Self::PAGE_SIZE.as_u64()) {
            return Err(StackPushError::EntryNotAligned);
        }

        self.len += 1;
        self.set_entry_value(
            self.len - 1,
            PhysFrame::from_start_address(entry_value).unwrap(),
        );
        Ok(())
    }

    /// Return the value at the given index of the stack.
    ///
    /// * `index`: The index of the stack where to get the value from.
    ///
    /// # Return
    /// Returns the physical address, the value at the given index, if the index doesn't exceed the
    /// capacity of the stack.
    pub fn get_entry_value(&self, index: StackIndex) -> Option<PhysAddr> {
        self.get_entry_virt_ptr(index).map(|entry_virt_ptr| {
            let entry_ptr = entry_virt_ptr.as_mut_ptr() as *const u64;
            let entry_value = unsafe { entry_ptr.read() };
            PhysAddr::new(entry_value)
        })
    }

    /// Sets the value at the given index in the stack to the starting address of the given page
    /// frame.
    pub fn set_entry_value(&self, index: StackIndex, page_frame: PhysFrame) {
        if let Some(entry_virt_ptr) = self.get_entry_virt_ptr(index) {
            let entry_ptr = entry_virt_ptr.as_mut_ptr() as *mut u64;
            let new_entry_value = page_frame.start_address();
            unsafe { entry_ptr.write(new_entry_value.as_u64()) };
        }
    }

    /// Returns a pointer to the physical address of the entry with the given index in the stack.
    ///
    /// * `index`: The entry index to the entry where the pointer should point to.
    fn get_entry_phys_ptr(&self, index: StackIndex) -> Option<PhysAddr> {
        if index >= self.len {
            return None;
        }

        let entry_phys_addr = self.start + (*POINTER_SIZE) * index;
        Some(entry_phys_addr)
    }

    /// Returns a pointer to the virtual address of the entry with the given index in the stack.
    ///
    /// * `index`: The entryz index to the entry where the pointer should point to.
    fn get_entry_virt_ptr(&self, index: StackIndex) -> Option<VirtAddr> {
        self.get_entry_phys_ptr(index)
            .map(|entry_phys_ptr| MEM_STRUCTURE.hhdm + entry_phys_ptr.as_u64())
    }
}
