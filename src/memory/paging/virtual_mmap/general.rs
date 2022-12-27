use x86_64::{structures::paging::{PageSize, Size4KiB}, PhysAddr, VirtAddr};

use super::Mapper;

/// A trait which includes the general methods which all vm-mapper should have.
pub trait VMMapperGeneral<P: PageSize> {
    /// Implements a standard translation function how the mapper translates the
    /// givien physical address.
    ///
    /// * `addr`: the physical address which shoulud be translated into a
    /// virtual address.
    fn translate_addr(&self, addr: PhysAddr) -> VirtAddr;
}

impl VMMapperGeneral<Size4KiB> for Mapper {
    fn translate_addr(&self, addr: PhysAddr) -> VirtAddr {
        self.start + addr.as_u64()
    }
}
