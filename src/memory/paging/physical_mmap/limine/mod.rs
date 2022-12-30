pub mod iterators;
pub mod kernel_info;

use {
    limine::{LimineMemmapEntry, LimineMemmapRequest, LimineMemmapResponse, NonNullPtr},
    x86_64::structures::paging::PageSize,
};

use self::iterators::UseableMemChunkIterator;

static MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);

pub fn get_amount_useable_page_frames<P: PageSize>() -> u64 {
    let mut page_frame_counter = 0;
    for mmap in UseableMemChunkIterator::new() {
        page_frame_counter += mmap.len.div_floor(P::SIZE);
    }

    page_frame_counter
}

pub fn get_memmap_response() -> &'static LimineMemmapResponse {
    MEMMAP_REQUEST.get_response().get().unwrap()
}

pub fn get_mmaps() -> &'static [NonNullPtr<LimineMemmapEntry>] {
    get_memmap_response().memmap()
}
