//! Includes the different paging implementation.
mod frame_allocator;
pub mod level4_paging;
mod page_size;
mod physical_mmap;
mod heap;

pub use page_size::PageSize;
pub use physical_mmap::{PhysMemMap, PhysLinearAddr};
use frame_allocator::FRAME_ALLOCATOR;

use crate::{print, println};

pub fn init() {
    let phys_mmap = PhysMemMap::new();

    frame_allocator::init(&phys_mmap);
    level4_paging::init(&FRAME_ALLOCATOR.get().unwrap());
    // heap::init();
    //
    // level4_paging::load();
}
