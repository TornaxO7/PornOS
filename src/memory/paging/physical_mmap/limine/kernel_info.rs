//! Just a helper module to get some information about the kernel.

use core::{marker::PhantomData, ops::Range};

use {
    limine::LimineKernelAddressRequest,
    x86_64::{structures::paging::PageSize, PhysAddr, VirtAddr},
};

use crate::memory::paging::physical_mmap::kernel_info::{
    KernelData, CODE_END, DATA_END, READ_ONLY_END,
};

static KERNEL_ADDRESS_REQUEST: LimineKernelAddressRequest = LimineKernelAddressRequest::new(0);

impl<P: PageSize> KernelData<P> {
    pub fn from_limine() -> Self {
        let response = KERNEL_ADDRESS_REQUEST.get_response().get().unwrap();
        let code = {
            let section_addr = (&CODE_END as *const u8).addr() as u64;

            let start = VirtAddr::new(response.virtual_base);
            let end = {
                let section_size = section_addr - start.as_u64();
                let end_addr = (start + section_size).align_up(P::SIZE);
                end_addr - 1u64
            };
            Range { start, end }
        };

        let read_only = {
            let section_addr = (&READ_ONLY_END as *const u8).addr() as u64;

            let start = code.end.align_up(P::SIZE);
            let end = VirtAddr::new(section_addr).align_up(P::SIZE) - 1u64;
            Range { start, end }
        };

        let data = {
            let section_addr = (unsafe { &DATA_END as *const u8 }).addr() as u64;

            let start = read_only.end.align_up(P::SIZE);
            let end = VirtAddr::new(section_addr).align_up(P::SIZE) - 1u64;

            Range { start, end }
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
