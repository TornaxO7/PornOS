//! Just a helper module to get some information about the kernel.

use core::{ops::Range, marker::PhantomData};

use limine::LimineKernelAddressRequest;
use x86_64::{PhysAddr, VirtAddr, structures::paging::PageSize};

static KERNEL_ADDRESS_REQUEST: LimineKernelAddressRequest = LimineKernelAddressRequest::new(0);

#[no_mangle]
#[link_section = ".pornos_code_end"]
static CODE_END: u8 = 0;

#[no_mangle]
#[link_section = ".pornos_read_only_end"]
static READ_ONLY_END: u8 = 0;

#[no_mangle]
#[link_section = ".pornos_datad_end"]
static DATA_END: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelData<P: PageSize> {
    pub start_phys: PhysAddr,
    pub start_virt: VirtAddr,

    pub code: Range<VirtAddr>,
    pub read_only: Range<VirtAddr>,
    pub data: Range<VirtAddr>,
    size: PhantomData<P>,
}

impl<P: PageSize> KernelData<P> {
    pub fn new() -> Self {
        let response = KERNEL_ADDRESS_REQUEST.get_response().get().unwrap();
        let code = {
            let section_addr = (&CODE_END as * const u8) as u64;

            let start = VirtAddr::new(response.virtual_base);
            let end = (start + (section_addr - start.as_u64())).align_down(P::SIZE);
            Range {
                start,
                end,
            }
        };

        let read_only = {
            let section_addr = (&READ_ONLY_END as * const u8) as u64;

            let start = code.end;
            let end = VirtAddr::new(section_addr).align_up(P::SIZE);
            Range {
                start,
                end,
            }
        };

        let data = {
            let section_addr = (&DATA_END as * const u8) as u64;

            let start = read_only.end;
            let end = VirtAddr::new(section_addr).align_up(P::SIZE);

            Range {
                start,
                end,
            }
        };

        Self {
            start_phys: PhysAddr::new(response.physical_base),
            start_virt: VirtAddr::new(response.virtual_base),
            code,
            read_only,
            data,
            size: PhantomData,
        }
    }
}
