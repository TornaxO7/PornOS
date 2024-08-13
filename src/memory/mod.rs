pub mod frame_allocator;
pub mod heap;
pub mod mapper;

pub use mapper::SIMP;

use limine::{
    memory_map::Entry,
    request::{HhdmRequest, MemoryMapRequest},
};

#[used]
#[link_section = ".requests"]
static MMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

pub fn get_entries() -> &'static [&'static Entry] {
    MMAP_REQUEST.get_response().unwrap().entries()
}

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub fn get_hhdm() -> u64 {
    HHDM_REQUEST.get_response().unwrap().offset()
}
