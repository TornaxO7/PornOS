use crate::{print, println};

pub mod level4_paging;
pub mod physical;

pub type VirtAddr = u64;
pub type PhysAddr = u64;

pub type Bytes = u64;

pub fn init() {
    print!("Memory ... ");

    println!("OK");
}
