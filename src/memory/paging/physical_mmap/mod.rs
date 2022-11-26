mod phys_linear_addr;
mod test;

use limine::{
    LimineMemmapEntry, LimineMemmapRequest, LimineMemmapResponse, LimineMemoryMapEntryType,
    NonNullPtr,
};
use x86_64::{PhysAddr, VirtAddr};

use crate::{
    memory::{
        paging::PageSize,
        types::{Byte, Bytes},
        HHDM,
    },
    println,
};

pub use phys_linear_addr::PhysLinearAddr;

/// A little helper type to show that the given value can be used as an index to get the appropriate
/// memory chunk.
pub type MemChunkIndex = usize;

static MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);

/// This struct holds all physical memory chunks.
/// # Note
/// This is only used at the time where the kernel-paging isn't loaded yet.
/// This should be used to setup the frame-allocator and the first memory mapping of paging.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysMemMap {
    entry_count: u64,
}

impl PhysMemMap {
    pub fn new() -> Self {
        Self {
            entry_count: Self::get_memmap_response().entry_count,
        }
    }

    /// Tries to find an address which guarantees to have the given size.
    pub fn get_frame(&self, start: PhysAddr, align: PageSize, size: Bytes) -> Option<PhysAddr> {
        let start = start.align_up(align.size().as_u64());

        let mmaps = Self::get_mmaps();
        for index in 0..self.entry_count {
            let mmap = &mmaps[index as usize];

            let has_enough_space = {
                let skipped_bytes = start.as_u64().saturating_sub(mmap.base);
                let useable_mem = mmap.len - skipped_bytes;

                useable_mem >= size.as_u64()
            };

            if mmap.typ == LimineMemoryMapEntryType::Usable && has_enough_space {
                return Some(PhysAddr::new(mmap.base));
            }
        }

        None
    }

    /// Converts the given physical linear address into a virtual address.
    /// # Returns
    /// `Some(VirtAddr)`: If the given phys_linear_addr is valid isn't over the available physical
    ///                   ram.
    /// `None`: If the virtual address is over the available phyiscal ram.
    pub fn convert_to_virt(&self, phys_linear_addr: &PhysLinearAddr) -> Option<VirtAddr> {
        if let Some((mem_chunk_index, offset)) = self.get_matching_mem_chunk(phys_linear_addr) {
            let mmaps = Self::get_mmaps();
            Some(VirtAddr::new(
                HHDM.as_u64() + mmaps[mem_chunk_index].base + offset.as_u64(),
            ))
        } else {
            None
        }
    }

    pub fn get_phys_addr(&self, phys_linear_addr: &PhysLinearAddr) -> PhysAddr {
        let (index, offset) = self.get_matching_mem_chunk(phys_linear_addr).unwrap();
        let mmap = Self::get_mmaps();

        PhysAddr::new(mmap[index as usize].base + offset.as_u64())
    }

    /// Returns the amount of available page frames according to the given page-frame-size.
    pub fn get_amount_page_frames(&self, page_size: PageSize) -> u64 {
        let mut page_frame_counter = 0;
        let response = MEMMAP_REQUEST.get_response().get().unwrap();

        for index in 0..response.entry_count {
            let entry = &response.memmap()[index as usize];
            if entry.typ == LimineMemoryMapEntryType::Usable {
                page_frame_counter += entry.len / page_size.size().as_u64();
            }
        }

        page_frame_counter
    }

    /// Returns the total amount of free useable ram in bytes.
    pub fn get_amount_useable_ram(&self, page_size: PageSize) -> Bytes {
        page_size.size() * self.get_amount_page_frames(page_size)
    }

    /// Writes the given consecutive bytes, given by the iterator to the consecutive bytes in the
    /// useable memory chunks.
    #[must_use]
    pub fn write_value(
        &self,
        value: impl IntoIterator<Item = Byte>,
        start_addr: PhysLinearAddr,
    ) -> bool {
        let mut offset = PhysLinearAddr::new(0);

        for value_byte in value.into_iter() {
            if !self.write_byte(value_byte, start_addr + offset) {
                return false;
            }

            offset += 1u64;
        }

        true
    }

    /// Tries to write the given byte at the physical linear addr `addr`.
    /// # Returns
    /// `false`: If the addr exceeds the usable memory and the value couldn't be written
    /// `true`: If the byte could be written successfully.
    #[must_use]
    fn write_byte(&self, byte: Byte, addr: PhysLinearAddr) -> bool {
        let mmaps = Self::get_mmaps();

        if let Some((mem_chunk_index, offset)) = self.get_matching_mem_chunk(&addr) {
            let mem_chunk_base = mmaps[mem_chunk_index].base;
            let dest_addr = (HHDM.as_u64() + mem_chunk_base + offset.as_u64()) as *mut Byte;

            unsafe {
                *dest_addr = byte;
            }

            true
        } else {
            false
        }
    }

    pub fn read(&self, addr: PhysLinearAddr) -> Byte {
        todo!()
    }

    /// Returns the index of the matching useable memory chunk, where `start_addr` points
    /// and the offset in this memory chunk.
    #[must_use]
    fn get_matching_mem_chunk(&self, addr: &PhysLinearAddr) -> Option<(MemChunkIndex, Bytes)> {
        let mmaps = Self::get_mmaps();
        let mut read_bytes: Bytes = Bytes::new(0);

        for mem_chunk_index in 0..self.entry_count as usize {
            let mem_chunk_index: usize = mem_chunk_index;

            if mmaps[mem_chunk_index].typ == LimineMemoryMapEntryType::Usable {
                let found_memory_chunk =
                    addr.as_u64() < (read_bytes.as_u64() + mmaps[mem_chunk_index].len);
                if found_memory_chunk {
                    return Some((mem_chunk_index, Bytes::new(addr.as_u64()) - read_bytes));
                }

                *read_bytes += mmaps[mem_chunk_index].len;
            }
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
