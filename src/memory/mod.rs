use crate::{print, println};

pub mod addr;
mod allocator;
pub mod level4_paging;
pub mod physical_memory_map;

pub type Bytes = u64;

pub fn init() {
    print!("Memory ... ");

    physical_memory_map::init();

    println!("OK");
}
