use limine::LimineHhdmRequest;

use crate::{print, println};

pub mod paging;
pub mod physical_memory_mapper;

pub type VirtAddr = u64;
pub type PhysAddr = u64;
pub type Bytes = u64;
pub type Byte = u8;

lazy_static::lazy_static! {
    pub static ref HHDM: VirtAddr = LimineHhdmRequest::new(0)
        .get_response()
        .get()
        .unwrap()
        .offset;
}

pub fn init() {
    print!("Memory ... ");

    paging::init();

    println!("OK");
}
