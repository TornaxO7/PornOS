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

use crate::klib::lock::spinlock::Spinlock;

pub use self::stack::Stack;

lazy_static::lazy_static! {
    /// The good old page frame allocator.
    pub static ref FRAME_ALLOCATOR: Spinlock<Stack> = Spinlock::new(Stack::new());
}
