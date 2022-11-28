// currently implements only 4KiB pages

use spin::RwLock;

use crate::{print, println, memory::{types::Bytes, paging::level4_paging::cr3::Cr3Value}};

use super::frame_allocator::FrameAllocator;

mod cr3;
mod pml4e;
mod pdpte;
mod pd;
mod pt;

// lazy_static::lazy_static! {
//     static ref KPMLE4_MAP: RwLock<PMLE4> = RwLock::new(PMLE4::new());
// }

/// 512: 512 entries per level
/// 8: Each entry is 8 bytes big
/// 3: Three levels need to be in memory. Level 1 (PMLE4) is already in the binary
const PMLE4_MAP_SIZE: Bytes = Bytes::new((8 * 512) * 3);

pub fn init(frame_allocator: &FrameAllocator) {
    print!("Init Level 4 Paging ... ");

    // let cr3_value = Cr3Value::new();

    println!("OK");
}

#[cfg(feature = "test")]
pub fn tests() {
    cr3::tests();
    pml4e::tests();
    pdpte::tests();
    pd::tests();
    pt::tests();
}

fn load() {
}
