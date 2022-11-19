#[cfg(feature = "frame-allocator-array-stack")]
mod array_stack;

#[cfg(feature = "frame-allocator-bitflag")]
mod bitflag;

use spin::Once;
use x86_64::{PhysAddr, VirtAddr};

use crate::{print, println};

use self::array_stack::ArrayStack;
use super::{page_size::PageSize, PhysMemMap};

static FRAME_ALLOCATOR: Once<FrameAllocator> = Once::new();

pub fn init(phys_mmap: &PhysMemMap) {
    print!("Frame allocator ... ");

    setup_frame_allocator(phys_mmap);

    println!("OK");
}

pub trait FrameManager: Send + Sync + core::fmt::Debug {
    fn new(phys_mmap: &PhysMemMap, page_size: PageSize) -> Self;

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

fn setup_frame_allocator(phys_mmap: &PhysMemMap) {
    let page_size = PageSize::Page4KB;

    FRAME_ALLOCATOR.call_once(|| {
        FrameAllocator {
            page_size,
            frame_manager: ArrayStack::new(phys_mmap, page_size),
        }
    });
}
