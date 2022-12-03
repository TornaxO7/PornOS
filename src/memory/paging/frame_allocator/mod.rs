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

pub use self::stack::Stack;

lazy_static::lazy_static! {
    pub static ref FRAME_ALLOCATOR: RwLock<Stack> = RwLock::new(Stack::new());
}

#[cfg(feature = "test")]
pub fn tests<P: PageSize + Send + Sync + Debug>(phys_mmap: &PhysMemMap<P>) {
    stack::tests(phys_mmap);
}
