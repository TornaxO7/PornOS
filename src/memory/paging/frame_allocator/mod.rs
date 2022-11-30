//! This module contains the frame allocator.
//! Each submodule represents a diffrent way how to manage the page frames.
#[cfg(feature = "frame-allocator-array-stack")]
mod array_stack;

#[cfg(feature = "frame-allocator-bitflag")]
mod bitflag;

#[cfg(feature = "frame-allocator-stack")]
mod stack;

mod phys_frame_index;

use core::fmt::Debug;

pub use phys_frame_index::PhysFrameIndex;

use spin::{Once, RwLock};
use x86_64::structures::paging::{PageSize, PhysFrame, Size4KiB};

pub use self::stack::Stack;
use super::PhysMemMap;

// pub static FRAME_ALLOCATOR: Once<RwLock<FrameAllocator<Size4KiB>>> = Once::new();
pub static FRAME_ALLOCATOR: Once<RwLock<Stack<Size4KiB>>> = Once::new();

#[cfg(feature = "test")]
pub fn tests<P: PageSize + Send + Sync + Debug>(phys_mmap: &PhysMemMap<P>) {
    stack::tests(phys_mmap);
}

/// Each frame manager needs to implement those functions.
pub trait FrameManager<P: PageSize>: Send + Sync + Debug {
    /// Returns the starting address of a free frame.
    fn get_free_frame(&mut self) -> Option<PhysFrame<P>>;

    /// Marks the given starting address of a frame as free.
    fn free_frame(&mut self, frame: PhysFrame<P>);
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
