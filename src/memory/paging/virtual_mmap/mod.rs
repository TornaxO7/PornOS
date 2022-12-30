pub mod vmmap_traits;

mod general;
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

use self::vmmap_traits::VMMapper;
pub use self::{general::VMMapperGeneral, map::VMMapperMap, unmap::VMmapperUnmap};

lazy_static! {
    /// **S**uper **I**mpressive **M**a**p**per
    ///
    /// Yes, this is the mapper which maps page to page frames or in other
    /// words: The ultimate ***SIMP***
    ///
    /// This is also known as the "Page-Mapper".
    pub static ref SIMP: Mutex<Mapper> = Mutex::new(Mapper::new());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mapper {
    p4_ptr: *mut PageTable,

    /// The starting virt-address where the mappings can start to be mapped.
    start: VirtAddr,
}

unsafe impl Send for Mapper {}

impl VMMapper<Size4KiB> for Mapper {}
