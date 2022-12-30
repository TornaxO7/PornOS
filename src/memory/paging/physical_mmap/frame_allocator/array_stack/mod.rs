//! This module uses an array and a stack in order to manage the available frames.
//! It has the following memory structure:
//! ```
//! HHDM
//! |
//! |----------------------------------------|
//! |         |         |                    |
//! |  Stack  |  Array  |    Other stuff     |
//! |         |         |                    |
//! |----------------------------------------|
//! ```
//! For more information how the stack and array works, take a look into the respective files.
mod frame_array;
mod frame_index;
mod frame_stack;

use crate::memory::paging::{PageSize, PhysLinearAddr, PhysMemMap};

use self::{frame_array::FrameArray, frame_stack::FrameStack};

use super::FrameManager;

pub use frame_index::{FrameArrayIndex, FrameIndexByteIterator};
use x86_64::VirtAddr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayStack {
    /// stortes the current free frames
    stack: FrameStack,
    /// stores all available frames
    array: FrameArray,
}

impl ArrayStack {
    /// The starting address in the physical linear address space where its components should be
    /// stored.
    const START: PhysLinearAddr = PhysLinearAddr::new(0);

    pub fn new(phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        let stack = FrameStack::new(Self::START, phys_mmap, page_size);
        let stack_capacity = stack.get_capacity();

        let array_phys_start = PhysLinearAddr::new(stack_capacity.as_u64() + 1);
        Self {
            stack,
            array: FrameArray::new(array_phys_start, phys_mmap, page_size),
        }
    }
}

impl FrameManager for ArrayStack {
    fn get_free_frame(&mut self) -> VirtAddr {
        todo!()
    }

    fn free_frame(&mut self, _addr: VirtAddr) {
        todo!()
    }

    fn mark_used_frames(&mut self, physical_mmap: &PhysMemMap) {
        todo!()
    }
}
