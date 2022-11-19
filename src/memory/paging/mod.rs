mod frame_allocator;
pub mod level4_paging;
mod page_size;
mod physical_mmap;

pub use page_size::PageSize;
pub use physical_mmap::{PhysMemMap, PhysLinearAddr};

use crate::{print, println};

pub fn init() {
    print!("Paging ... ");

    let phys_mmap = PhysMemMap::new();
    frame_allocator::init(&phys_mmap);

    println!("OK");
}
