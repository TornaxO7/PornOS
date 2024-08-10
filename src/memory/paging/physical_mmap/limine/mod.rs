pub mod iterators;
pub mod kernel_info;

use {
    limine::BaseRevision,
    x86_64::structures::paging::{PageSize, Size4KiB},
};

use limine::{
    memory_map::Entry,
    request::{MemoryMapRequest, StackSizeRequest},
    response::MemoryMapResponse,
};

use self::iterators::UseableMemChunkIterator;

#[used]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
pub static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(STACK_SIZE);
pub static STACK_SIZE: u64 = Size4KiB::SIZE * 8;

static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

pub fn get_amount_useable_page_frames<P: PageSize>() -> u64 {
    let mut page_frame_counter = 0;
    for mmap in UseableMemChunkIterator::new() {
        page_frame_counter += mmap.length.div_floor(P::SIZE);
    }

    page_frame_counter
}

pub fn get_memmap_response() -> &'static MemoryMapResponse {
    MEMORY_MAP_REQUEST.get_response().unwrap()
}

pub fn get_mmaps() -> &'static [&'static Entry] {
    get_memmap_response().entries()
}
