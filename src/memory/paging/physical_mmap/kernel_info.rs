//! Just a helper module to get some information about the kernel.

use limine::LimineKernelAddressRequest;
use x86_64::{PhysAddr, VirtAddr};

use crate::{memory::types::Bytes, println};

static KERNEL_ADDRESS_REQUEST: LimineKernelAddressRequest = LimineKernelAddressRequest::new(0);

#[no_mangle]
static KERNEL_START: u8 = 0;
#[no_mangle]
static KERNEL_END: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelData {
    pub phys_addr: PhysAddr,
    pub virt_addr: VirtAddr,
    pub len: Bytes,
}

impl KernelData {
    pub fn new() -> Self {
        let response = KERNEL_ADDRESS_REQUEST.get_response().get().unwrap();
        let len = {
            let start_ptr = (&KERNEL_START as * const u8) as u64;
            let end_ptr = (&KERNEL_END as * const u8) as u64;

            println!("start: 0x{:x}, end: 0x{:x}", start_ptr, end_ptr);

            Bytes::new(end_ptr - start_ptr)
        };


        Self {
            phys_addr: PhysAddr::new(response.physical_base),
            virt_addr: VirtAddr::new(response.virtual_base),
            len,
        }
    }
}
