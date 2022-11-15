// currently implements only 4KiB pages

use crate::{print, println, memory::level4_paging::util::Memmaps};

mod util;
mod page;
mod page_table;
mod pdpt;
mod pmle4;

lazy_static::lazy_static! {
    static ref MEMMAPS: Memmaps = Memmaps::new();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtAddr(u64);

pub fn init() {
    print!("Memory ... ");

    for index in 0..MEMMAPS.len {
        println!("{:?}", MEMMAPS.get(index).unwrap());
    }

    println!("OK");
}

pub trait KernelPage {
    fn for_kernel() -> Self;
}
