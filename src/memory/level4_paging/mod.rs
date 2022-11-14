// currently implements only 4KiB pages

use self::{
    page::PageEntry, page_table::PageTableEntry, pdpt::PDPTEntry, pmle4::PMLE4Entry,
};

mod page;
mod page_table;
mod pdpt;
mod pmle4;

#[repr(C)]
pub struct LogicAddr {
}

pub fn init() {
}
