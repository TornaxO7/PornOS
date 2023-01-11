mod map;
mod unmap;

use {
    lazy_static::lazy_static,
    spin::Mutex,
    x86_64::{
        structures::paging::{PageTable, Size4KiB},
        VirtAddr,
    },
};

use x86_64::{structures::paging::PageSize, PhysAddr};

pub use self::{map::VMMapperMap, unmap::VMmapperUnmap};

use super::mem_structure::MEM_STRUCTURE;

lazy_static! {
    /// **S**uper **I**mpressive **M**a**p**per
    ///
    /// Yes, this is the mapper which maps page to page frames or in other
    /// words: The ultimate ***SIMP***
    ///
    /// This is also known as the "Page-Mapper".
    pub static ref SIMP: Mutex<Mapper> = Mutex::new(Mapper::new());
}

// Create an extra layer for that with an attritbute with a mutex to lock the
// actual unsafe mappings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mapper {
    p4_ptr: *mut PageTable,

    /// The starting virt-address where the mappings can start to be mapped.
    start: VirtAddr,
}

/// The main trait which all vm-mappers need to implement.
pub trait VMMapper<P: PageSize>: VMMapperMap<P> + VMmapperUnmap<P> {}

impl VMMapper<Size4KiB> for Mapper {}

unsafe impl Send for Mapper {}

/// Returns the identitical mapping in the HHDM of the given physical address.
///
/// * `addr`: The physical address which should be translated.
#[inline]
pub fn translate_addr(addr: PhysAddr) -> VirtAddr {
    MEM_STRUCTURE.hhdm + addr.as_u64()
}

#[inline]
/// Returns the identical mapped physical address of the given virtual address.
///
/// * `addr`: The identitcal mapped virtual address.
pub fn detranslate_addr(addr: VirtAddr) -> PhysAddr {
    PhysAddr::new(addr - MEM_STRUCTURE.hhdm)
}
