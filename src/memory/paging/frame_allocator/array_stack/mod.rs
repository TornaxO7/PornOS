mod frame_array;
mod frame_stack;

use crate::memory::{Bytes, PhysAddr, HHDM, physical_memory_mapper::IntoBytes};

use self::{frame_array::FrameArray, frame_stack::FrameStack};

use super::FrameManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
struct FrameIndex(pub u64);

impl FrameIndex {
    pub const SIZE: Bytes = core::mem::size_of::<Self>() as Bytes;
}

impl IntoBytes for FrameIndex {
    fn into_bytes(&self) -> &[crate::memory::Byte] {
        // [self.0 >> ]
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayStack {
    /// stortes the current free frames
    stack: FrameStack,
    /// stores all available frames
    array: FrameArray,
}

impl FrameManager for ArrayStack {
    fn new(amount_page_frames: u64) -> Self {
        let stack = FrameStack::new(*HHDM, amount_page_frames);
        let stack_len = stack.len;
        Self {
            stack,
            array: FrameArray::new(*HHDM + stack_len, amount_page_frames),
        }
    }

    fn get_free_frame(&mut self) -> PhysAddr {
        todo!()
    }

    fn free_frame(&mut self, _addr: PhysAddr) {
        todo!()
    }
}
