mod phys_linear_addr;
mod test;
mod iterators;

use core::{marker::PhantomData, array::IntoIter};

use limine::{
    LimineMemmapEntry, LimineMemmapRequest, LimineMemmapResponse, LimineMemoryMapEntryType,
    NonNullPtr,
};
use x86_64::{PhysAddr, structures::paging::PageSize};

use crate::memory::types::Bytes;

pub use phys_linear_addr::PhysLinearAddr;

/// A little helper type to show that the given value can be used as an index to get the appropriate
/// memory chunk.
pub type MemChunkIndex = usize;

static MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);

/// This struct holds all physical memory chunks.
/// # Note
/// This is only used at the time where the kernel-paging isn't loaded yet.
/// This should be used to setup the frame-allocator and the first memory mapping of paging.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PhysMemMap<P: PageSize> {
    pub entry_count: u64,
    size: PhantomData<P>,
}

impl<P: PageSize> PhysMemMap<P> {
    pub fn new() -> Self {
        Self {
            entry_count: Self::get_memmap_response().entry_count,
            size: PhantomData,
        }
    }

    /// Returns the amount of available page frames according to the given page-frame-size.
    pub fn get_amount_page_frames(&self) -> u64 {
        let mut page_frame_counter = 0;
        for mmap in self.into_iter_useable() {
            page_frame_counter += mmap.len / P::SIZE;
        }

        page_frame_counter
    }

    fn get_mmaps() -> &'static [NonNullPtr<LimineMemmapEntry>] {
        Self::get_memmap_response().memmap()
    }

    fn get_memmap_response() -> &'static LimineMemmapResponse {
        MEMMAP_REQUEST.get_response().get().unwrap()
    }
}
