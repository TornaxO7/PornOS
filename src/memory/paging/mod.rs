//! Includes the different paging implementation.
mod frame_allocator;
mod heap;
mod physical_mmap;

use core::marker::PhantomData;

pub use physical_mmap::{PhysLinearAddr, PhysMemMap};
use spin::RwLock;
use x86_64::{
    structures::paging::{Page, PageSize, PageTable, PageTableFlags, PhysFrame, Size4KiB, page_table::PageTableLevel},
    VirtAddr, PhysAddr,
};

use self::frame_allocator::{FrameManager, Stack, FRAME_ALLOCATOR};

use super::HHDM;

pub fn init() {
    let phys_mmap = PhysMemMap::<Size4KiB>::new();
    FRAME_ALLOCATOR.call_once(|| RwLock::new(Stack::new(&phys_mmap)));

    let mut p_configurator = KPagingConfigurator::<Size4KiB>::new(&phys_mmap);
    p_configurator.map_kernel();
    p_configurator.map_heap();
}

#[cfg(feature = "test")]
pub fn tests() {
    let phys_mmap: PhysMemMap<Size4KiB> = PhysMemMap::new();
    frame_allocator::tests(&phys_mmap);
}

/// The paging configurator which sets up the different paging levels.
///
/// # SAFETY
/// It assumes, that we are still using the paging table of Limine!!!!
#[derive(Debug, Clone)]
pub struct KPagingConfigurator<'a, P: PageSize> {
    size: PhantomData<P>,
    phys_mmap: &'a PhysMemMap<P>,
    p4_ptr: *mut PageTable,
}

impl<'a, P: PageSize> KPagingConfigurator<'a, P> {
    pub fn new(phys_mmap: &'a PhysMemMap<P>) -> Self {
        let pml4e_addr = *HHDM + Self::get_free_virt_frame().as_u64();
        Self {
            size: PhantomData,
            phys_mmap,
            p4_ptr: pml4e_addr.as_mut_ptr() as *mut PageTable,
        }
    }

    /// This maps the kernel and its modules to the same virtual address as the given virtual
    /// address of limine.
    pub fn map_kernel(&mut self) {
        let mut kernel_iter = self.phys_mmap.into_iter_mem_chunk();
        while let Some(mmap) = kernel_iter.next() {
            for offset in (0..mmap.len).step_by(P::SIZE.try_into().unwrap()) {
                let virt_kframe_addr = *HHDM + mmap.base + offset;
                let page_frame_ptr = virt_kframe_addr.as_mut_ptr() as * const Page;
                let page = unsafe {page_frame_ptr.read()};
                self.map_existing_frame(page);
            }
        }
    }

    /// Maps a heap for the kernel.
    pub fn map_heap(&mut self) {
    }

    /// This functions creates all necessary entries in the paging-hierarchy so it gets mapped.
    pub fn map_existing_frame(&self, frame: Page) {
        // u8: Just a type to be able to use the function
self.get_mem_mut_ptr::<u8>(frame.start_address());
    }

    /// Returns a pointer of the given type which points on a free memory chunk.
    pub fn get_mem_mut_ptr<T>(&self, addr: VirtAddr) -> *mut T {
        let new_table_entry_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        let mut table = unsafe { self.p4_ptr.read() };
        let mut level = PageTableLevel::Four;

        while let Some(lower_level) = level.next_lower_level() {
            let table_entry = match lower_level {
                PageTableLevel::Three => &mut table[addr.p4_index()],
                PageTableLevel::Two => &mut table[addr.p3_index()],
                PageTableLevel::One => &mut table[addr.p2_index()],
                _ => unreachable!("Ayo, '{:?}' shouldn't be here <.<", lower_level),
            };

            let next_table_vtr_ptr = if table_entry.is_unused() {
                let new_frame = Self::get_free_phys_frame();
                table_entry.set_addr(new_frame , new_table_entry_flags);

                *HHDM + new_frame.as_u64()
            } else {
                *HHDM + table_entry.addr().as_u64()
            };
            let next_table_ptr = next_table_vtr_ptr.as_mut_ptr() as * mut PageTable;

            table = unsafe {next_table_ptr.read()};
            level = lower_level;
        }

        let page_frame_phys_addr = &table[addr.p1_index()];
        let page_frame_virt_addr = *HHDM + page_frame_phys_addr.addr().as_u64();
        let value_virt_addr: VirtAddr = page_frame_virt_addr + u64::from(addr.page_offset());
        value_virt_addr.as_mut_ptr() as *mut T
    }

    fn get_free_phys_frame() -> PhysAddr {
        let new_frame = FRAME_ALLOCATOR
            .get()
            .unwrap()
            .write()
            .get_free_frame()
            .unwrap();
        new_frame.start_address()
    }

    fn get_free_virt_frame() -> VirtAddr {
        let phys_frame = Self::get_free_phys_frame();
        let new_frame_virt_addr = *HHDM + phys_frame.as_u64();
        new_frame_virt_addr
    }
}
