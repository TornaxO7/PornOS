// currently implements only 4KiB pages

use self::{
    page::PageEntry,
    page_table::PageTableEntry,
    pdpt::PDPTEntry,
    pmle4::PMLE4,
};

mod page;
mod page_table;
mod pdpt;
mod pmle4;

static PML4: PMLE4 = PMLE4::default();

pub fn init() {}
