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

pub fn init() {
    print!("Memory ... ");

    println!("OK");
}

pub trait KernelPage {
    fn for_kernel() -> Self;
}
