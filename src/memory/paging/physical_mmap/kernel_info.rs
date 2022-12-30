use core::{marker::PhantomData, ops::Range};

use x86_64::{structures::paging::PageSize, PhysAddr, VirtAddr};

#[no_mangle]
#[link_section = ".pornos_code_end"]
pub static CODE_END: u8 = 0;

#[no_mangle]
#[link_section = ".pornos_read_only_end"]
pub static READ_ONLY_END: u8 = 0;

#[no_mangle]
#[link_section = ".pornos_data_end"]
pub static mut DATA_END: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelData<P: PageSize> {
    pub start_phys: PhysAddr,
    pub start_virt: VirtAddr,

    pub code: Range<VirtAddr>,
    pub read_only: Range<VirtAddr>,
    pub data: Range<VirtAddr>,
    pub size: PhantomData<P>,
}
