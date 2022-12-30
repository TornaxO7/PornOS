use x86_64::{
    structures::paging::{PageSize, Size4KiB},
    PhysAddr, VirtAddr,
};

use super::Mapper;

/// A trait which includes the general methods which all vm-mapper should have.
pub trait VMMapperGeneral<P: PageSize> {
    /// Implements a standard translation function how the mapper translates the
    /// givien physical address.
    ///
    /// * `addr`: the physical address which shoulud be translated into a
    /// virtual address.
    fn translate_addr(&self, addr: PhysAddr) -> VirtAddr;

    /// Implements the complement of `translate_addr` to get the physical
    /// address.
    ///
    /// # SAFETY
    /// You ***must*** make sure that `addr` was translated before by
    /// `translate_addr` by any cost!
    unsafe fn detranslate_addr(&self, addr: VirtAddr) -> PhysAddr;
}

impl VMMapperGeneral<Size4KiB> for Mapper {
    fn translate_addr(&self, addr: PhysAddr) -> VirtAddr {
        self.start + addr.as_u64()
    }

    unsafe fn detranslate_addr(&self, addr: VirtAddr) -> PhysAddr {
        PhysAddr::new(addr - self.start)
    }
}
