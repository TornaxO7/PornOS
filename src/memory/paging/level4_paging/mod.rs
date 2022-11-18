// currently implements only 4KiB pages

use spin::mutex::SpinMutex;

use crate::{print, println, memory::Bytes};

use self::pmle4::PMLE4;

mod page;
mod page_table;
mod pdpt;
mod pmle4;

lazy_static::lazy_static! {
    static ref PMLE4_MAP: SpinMutex<PMLE4> = SpinMutex::new(PMLE4::new());
}

/// 512: 512 entries per level
/// 8: Each entry is 8 bytes big
/// 3: Three levels need to be in memory. Level 1 (PMLE4) is already in the binary
const PMLE4_MAP_SIZE: Bytes = (8 * 512) * 3;

pub fn init() {
    print!("\n\tInit Level 4 Paging ... ");

    // if PHYS_MEMMAP.lock().useable_mem() < PMLE4_MAP_SIZE {
    //     panic!("Not enough useable memory for paging");
    // }

    println!("OK");
}

pub trait PagingIndex {
    fn new(value: u64) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PMLE4Index(u64);

impl PagingIndex for PMLE4Index {
    fn new(value: u64) -> Self {
        if value > 0b1_1111_1111 {
            panic!("Invalid pmle4 index.");
        }
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PDPTIndex(u64);

impl PagingIndex for PDPTIndex {
    fn new(value: u64) -> Self {
        if value > 0b1_1111_1111 {
            panic!("Invalid page directory pointer table index.");
        }
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageDirectoryIndex(pub u64);

impl PagingIndex for PageDirectoryIndex {
    fn new(value: u64) -> Self {
        if value > 0b1_1111_1111 {
            panic!("Invalid page directory index.");
        }
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageTableIndex(pub u64);

impl PagingIndex for PageTableIndex {
    fn new(value: u64) -> Self {
        if value > 0b1_1111_1111 {
            panic!("Invalid page table index.");
        }
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageOffset(pub u64);

impl PagingIndex for PageOffset {
    fn new(value: u64) -> Self {
        if value > 0b1111_1111_1111 {
            panic!("Invalid page offset.");
        }
        Self(value)
    }
}
