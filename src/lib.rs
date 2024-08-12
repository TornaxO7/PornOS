#![no_std]

use limine::request::HhdmRequest;
use x86_64::{PhysAddr, VirtAddr};
pub mod qemu;
pub mod serial;

pub mod memory;

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub fn translate_to_virt(addr: PhysAddr) -> VirtAddr {
    let hhdm = HHDM_REQUEST.get_response().expect("Get HHDM address.");

    VirtAddr::new(addr.as_u64() + hhdm.offset())
}
