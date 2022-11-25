// currently implements only 4KiB pages

use spin::{mutex::SpinMutex, Spin};

use crate::{print, println, memory::types::Bytes};

use self::pmle4::PMLE4;

use super::frame_allocator::FrameAllocator;

mod page;
mod page_table;
mod pdpt;
mod pmle4;

lazy_static::lazy_static! {
    static ref KPMLE4_MAP: Spin<PMLE4> = Spin::new(PMLE4::new());
}

/// 512: 512 entries per level
/// 8: Each entry is 8 bytes big
/// 3: Three levels need to be in memory. Level 1 (PMLE4) is already in the binary
const PMLE4_MAP_SIZE: Bytes = Bytes::new((8 * 512) * 3);

pub fn init(frame_allocator: &FrameAllocator) {
    print!("\n\tInit Level 4 Paging ... ");

    println!("OK");
}

pub fn load() {
}
