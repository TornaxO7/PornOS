//! Contains the physical frame allocator.
//! Currently only 4KiB page frames are possible.
mod iterators;
// mod test;

use limine::{LimineMemmapEntry, LimineMemmapRequest, NonNullPtr, LimineMemmapResponse};

pub use iterators::{MemChunkIterator, UseableMemChunkIterator, KernelAndModulesIterator};

use x86_64::structures::paging::PageSize;

static MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);

pub fn get_amount_page_frames<P: PageSize>() -> u64 {
    let mut page_frame_counter = 0;
    for mmap in UseableMemChunkIterator::new() {
        page_frame_counter += mmap.len / P::SIZE;
    }

    page_frame_counter
}

fn get_memmap_response() -> &'static LimineMemmapResponse {
    MEMMAP_REQUEST.get_response().get().unwrap()
}

fn get_mmaps() -> &'static [NonNullPtr<LimineMemmapEntry>] {
    get_memmap_response().memmap()
}
