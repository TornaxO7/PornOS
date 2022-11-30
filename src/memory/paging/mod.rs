//! Includes the different paging implementation.
mod frame_allocator;
mod heap;
// pub mod level4_paging;
mod physical_mmap;

use core::marker::PhantomData;

use limine::LimineKernelAddressRequest;
pub use physical_mmap::{PhysLinearAddr, PhysMemMap};
use spin::{Once, RwLock};
use x86_64::{
    structures::paging::{Page, PageSize, Size4KiB, PageTable},
    VirtAddr,
};

use self::frame_allocator::{FRAME_ALLOCATOR, Stack};

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
    pml4e: PageTable,
}

impl<'a, P: PageSize> KPagingConfigurator<'a, P> {
    pub fn new(phys_mmap: &'a PhysMemMap<P>) -> Self {
        Self {
            size: PhantomData,
            phys_mmap,
            pml4e: PageTable::new(),
        }
    }

    pub fn map_kernel(&mut self) {

    }

    pub fn map_heap(&mut self) {}

    #[must_use]
    pub fn register_page(&mut self, addr: VirtAddr) -> bool {
        todo!()
    }
}
