use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, PhysFrame, Size4KiB};

use super::Stack;

unsafe impl FrameAllocator<Size4KiB> for Stack {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.pop()
            .map(|phys_addr| PhysFrame::from_start_address(phys_addr).unwrap())
    }
}

impl FrameDeallocator<Size4KiB> for Stack {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.push(frame.start_address()).unwrap()
    }
}
