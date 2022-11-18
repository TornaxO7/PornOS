mod frame_allocator;
pub mod level4_paging;
mod page_size;

pub use page_size::PageSize;

use crate::{print, println};

pub fn init() {
    print!("Paging ... ");

    frame_allocator::init();

    println!("OK");
}
