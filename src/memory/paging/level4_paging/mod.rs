// currently implements only 4KiB pages

use crate::{
    memory::paging::{frame_allocator::FrameManager, level4_paging::cr3::{Cr3Flag, Cr3Value}},
    print, println,
};
use limine::LimineKernelAddressRequest;
use spin::RwLock;
use x86_64::VirtAddr;

use super::{frame_allocator::FrameAllocator, page_frame::PageFrame, PhysMemMap};

static KERNEl_REQUEST: LimineKernelAddressRequest = LimineKernelAddressRequest::new(0);

mod cr3;
mod pd;
mod pdpte;
mod pml4e;
mod pt;

pub fn init(phys_mmap: &PhysMemMap, frame_allocator: &RwLock<FrameAllocator>) {
    print!("Init Level 4 Paging ... ");

    let mut pml4e = frame_allocator.write().get_free_frame().unwrap();
    let cr3_value = Cr3Value::new(Cr3Flag::PWT | Cr3Flag::PCD).set_pml4e_phys_addr(pml4e.start);

    map_kernel(phys_mmap, &mut pml4e);
    map_heap(&mut pml4e);
    map_stack(&mut pml4e);

    load_paging(cr3_value);

    println!("OK");
}

#[cfg(feature = "test")]
pub fn tests() {
    cr3::tests();
    pml4e::tests();
    pdpte::tests();
    pd::tests();
    pt::tests();
}

fn load_paging(_cr3_value: Cr3Value) {}

fn map_kernel(phys_mmap: &PhysMemMap, pml4e: &mut PageFrame) {
    let (_, klen) = phys_mmap.get_kernel_frame();
    let virt_kstart = {
        let virt_addr = KERNEl_REQUEST.get_response().get().unwrap().virtual_base;
        VirtAddr::new(virt_addr)
    };
}

fn map_heap(pml4e: &mut PageFrame) {}

fn map_stack(pml4e: &mut PageFrame) {}
