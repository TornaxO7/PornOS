//! This module contains the frame allocator.
//! Each submodule represents a diffrent way how to manage the page frames.
#[cfg(feature = "frame-allocator-array-stack")]
mod array_stack;

#[cfg(feature = "frame-allocator-bitflag")]
mod bitflag;

#[cfg(feature = "frame-allocator-stack")]
mod stack;

mod phys_frame_index;

pub use phys_frame_index::PhysFrameIndex;

use spin::RwLock;
use x86_64::structures::paging::Size4KiB;

pub use self::stack::Stack;

pub static FRAME_ALLOCATOR: RwLock<Stack> = RwLock::new(Stack::new());

#[cfg(feature = "test")]
pub fn tests<P: PageSize + Send + Sync + Debug>(phys_mmap: &PhysMemMap<P>) {
    stack::tests(phys_mmap);
}

// The main frame allocator struct which manages the frames.
// #[derive(Debug)]
// pub struct FrameAllocator<P: PageSize + Send + Sync + Debug> {
//     /// this stores the datastructure how the frames are stored.
//     frame_manager: Stack<P>,
// }
//
// impl<P: PageSize + Send + Sync + Debug> FrameManager<P> for FrameAllocator<P> {
//     /// Returns the starting address of a free frame.
//     fn get_free_frame(&mut self) -> Option<PhysFrame<P>> {
//         self.frame_manager.get_free_frame()
//     }
//
//     fn free_frame(&mut self, frame: PhysFrame<P>) {
//         self.frame_manager.free_frame(frame);
//     }
// }
