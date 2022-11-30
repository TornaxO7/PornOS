//! Includes the different paging implementation.
mod frame_allocator;
mod heap;
pub mod level4_paging;
mod page_frame;
mod physical_mmap;

use limine::LimineKernelAddressRequest;
pub use physical_mmap::{PhysLinearAddr, PhysMemMap};
use spin::{Once, RwLock};
use x86_64::{
    structures::paging::{Page, PageSize, Size4KiB},
    VirtAddr,
};

static KERNEL_ADDR_REQUEST: LimineKernelAddressRequest = LimineKernelAddressRequest::new(0);
static PML4E: Once<RwLock<Page<Size4KiB>>> = Once::new();

pub fn init() {
    let phys_mmap = PhysMemMap::new();

    frame_allocator::init(&phys_mmap);

    let mut p_configurator = KPagingConfigurator::new::<Size4KiB>(&phys_mmap);
    p_configurator.map_kernel();
}

#[cfg(feature = "test")]
pub fn tests() {
    let phys_mmap = PhysMemMap::new();
    frame_allocator::tests(&phys_mmap);
    level4_paging::tests();
}

/// The paging configurator which sets up the different paging levels.
///
/// # SAFETY
/// It assumes, that we are still using the paging table of Limine!!!!
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KPagingConfigurator<'a, P: PageSize> {
    phys_mmap: &'a PhysMemMap<P>,
}

impl<'a, P: PageSize> KPagingConfigurator<'a, P> {
    pub fn new(phys_mmap: &'a PhysMemMap<P>) -> Self {
        let p_configurator = Self { phys_mmap };

        p_configurator.setup_pml4e();
        p_configurator
    }

    pub fn map_kernel(&mut self) {}

    #[must_use]
    pub fn register_page(&mut self, addr: VirtAddr) -> bool {
        todo!()
    }

    pub fn get_new_mapped_frame(&self) -> Page {
        todo!()
    }

    fn setup_pml4e(&self) {

        PML4E.call_once(|| RwLock::new(todo!()));
    }
}
