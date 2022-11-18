use limine::{LimineMemmapRequest, LimineMemmapResponse, LimineMemoryMapEntryType, NonNullPtr, LimineMemmapEntry};
use spin::mutex::SpinMutex;

use super::{paging::PageSize, Byte, VirtAddr, Bytes};

pub type PhysLinearAddr = u64;

static MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);

lazy_static::lazy_static! {
    pub static ref PHYS_MEMMAP: SpinMutex<PhysMemMap> = {
        let phys_mmap = PhysMemMap {
            entry_count: PhysMemMap::get_memmap_response().entry_count,
        };

        SpinMutex::new(phys_mmap)
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysMemMap {
    entry_count: u64,
}

impl PhysMemMap {
    pub fn get_amount_page_frames(&self, page_size: PageSize) -> u64 {
        let mut page_frame_counter = 0;
        let response = MEMMAP_REQUEST.get_response().get().unwrap();

        for index in 0..response.entry_count {
            let entry = &response.memmap()[index as usize];
            if entry.typ == LimineMemoryMapEntryType::Usable {
                page_frame_counter += entry.len / page_size.size();
            }
        }

        page_frame_counter
    }

    pub fn write(&self, value: impl IntoBytes, start_addr: PhysLinearAddr) -> bool {
        let bytes = value.into_bytes();
        let mmaps = Self::get_mmaps();

        let (mem_chunk_index, offset) = self.get_matching_mem_chunk(start_addr);
    }

    pub fn read(&self, addr: PhysLinearAddr) -> Byte {
        todo!()
    }

    /// Returns the index of the matching memory chunk, where `start_addr` points
    /// and the offset in this memory chunk.
    fn get_matching_mem_chunk(&self, addr: PhysLinearAddr) -> Option<(usize, Bytes)> {
        let mmaps = Self::get_mmaps();
        let mut read_bytes: Bytes = 0;

        for mem_chunk_index in 0..self.entry_count as usize {
            let mem_chunk_index: usize = mem_chunk_index;

            let found_memory_chunk = addr < (read_bytes + mmaps[mem_chunk_index].len);
            if found_memory_chunk {
                return Some((mem_chunk_index, addr - read_bytes));
            }

            read_bytes += mmaps[mem_chunk_index].len;
        }

        None
    }

    fn get_mmaps() -> &'static [NonNullPtr<LimineMemmapEntry>] {
        Self::get_memmap_response().memmap()
    }

    fn get_memmap_response() -> &'static LimineMemmapResponse {
        MEMMAP_REQUEST.get_response().get().unwrap()
    }
}

pub trait IntoBytes {
    fn into_bytes(&self) -> &[Byte];
}
