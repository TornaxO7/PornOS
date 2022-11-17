// currently implements only 4KiB pages

use crate::{print, println};

use self::pmle4::PMLE4;

mod page;
mod page_table;
mod pdpt;
mod pmle4;

lazy_static::lazy_static! {
    static ref PMLE4_MAP: PMLE4 = PMLE4::new();
}

pub fn init() {
    print!("\tLevel 4 Paging ... ");

    println!("OK");
}

pub trait KernelPage {
    fn for_kernel() -> Self;
}
