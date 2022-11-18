#[cfg(feature = "frame-allocator-array-stack")]
mod array_stack;

#[cfg(feature = "frame-allocator-bitflag")]
mod bitflag;

use crate::{
    memory::{physical_memory_mapper::PHYS_MEMMAP, PhysAddr, VirtAddr},
    print, println,
};

use self::array_stack::ArrayStack;
use super::page_size::PageSize;

lazy_static::lazy_static! {
    static ref FRAME_ALLOCATOR: FrameAllocator = FrameAllocator {
        page_size: PageSize::Page4KB,
        frame_manager: ArrayStack::new(PHYS_MEMMAP.lock().get_amount_page_frames(PageSize::Page4KB)),
    };
}

pub fn init() {
    print!("Frame allocator ... ");

    println!("OK");
}

pub trait FrameManager: Send + Sync + core::fmt::Debug {
    fn new(amount_page_frames: u64) -> Self;

    fn get_free_frame(&mut self) -> PhysAddr;

    fn free_frame(&mut self, addr: PhysAddr);
}

#[derive(Debug)]
pub struct FrameAllocator {
    page_size: PageSize,
    frame_manager: ArrayStack,
}

impl FrameAllocator {
    pub fn get_free_frame(&mut self) -> VirtAddr {
        todo!()
    }

    pub fn free_frame(&mut self, _frame_addr: VirtAddr) {
        todo!()
    }
}
