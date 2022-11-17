// currently implements only 4KiB pages

use crate::{print, println};

mod page;
mod page_table;
mod pdpt;
mod pmle4;

pub fn init() {
    print!("Memory ... ");

    println!("OK");
}

pub trait KernelPage {
    fn for_kernel() -> Self;
}
