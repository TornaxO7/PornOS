use x86_64::{structures::paging::PhysFrame, PhysAddr};

use super::{Stack, POINTER_SIZE};

impl Stack {
    pub fn get_frame_allocator_page_frames(&self) -> StackPageFrames {
        StackPageFrames {
            index: 0,
            len: self.amount_used_page_frames as usize,
            stack_page_frame_start: self.start.clone() + (POINTER_SIZE * (self.capacity + 1u64)).as_u64(),
        }
    }
}

pub struct StackPageFrames {
    index: usize,
    len: usize,
    stack_page_frame_start: PhysAddr,
}

impl Iterator for StackPageFrames {
    type Item = PhysFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let stack_page_frame = {
            let addr = self.stack_page_frame_start + (POINTER_SIZE * self.index).as_u64();
            let ptr = addr.as_u64() as * const u64;
            let page_frame_addr = PhysAddr::new(unsafe {*ptr});
            PhysFrame::from_start_address(page_frame_addr).unwrap()
        };

        self.index += 1;
        Some(stack_page_frame)
    }
}
