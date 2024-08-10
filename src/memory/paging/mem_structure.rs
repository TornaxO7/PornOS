use {
    lazy_static::lazy_static,
    x86_64::{PhysAddr, VirtAddr},
};

use {limine::request::HhdmRequest, x86_64::structures::paging::FrameAllocator};

use crate::{klib::lock::once::Once, memory::types::Bytes};

use super::physical_mmap::{
    frame_allocator::FRAME_ALLOCATOR, limine::iterators::UseableMemChunkIterator,
};

static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

lazy_static! {
    pub static ref MEM_STRUCTURE: MemStructure = MemStructure {
        hhdm: VirtAddr::new(HHDM_REQUEST.get_response().unwrap().offset()),
        kstart: VirtAddr::new(0xffffffff80000000),
        heap: Once::new(),
        stack: Once::new(),
        pml4: Once::new(),
    };
}

#[derive(Debug)]
pub struct MemStructure {
    /// The starting address of the HHDM
    pub hhdm: VirtAddr,

    /// The starting address of the kernel code-section
    pub kstart: VirtAddr,

    pub heap: Once<Heap>,
    pub stack: Once<Stack>,
    pub pml4: Once<Pml4>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Heap {
    pub start: VirtAddr,
    pub init_size: Bytes,
}

impl Heap {
    pub fn new(page_size: Bytes) -> Self {
        let last_useable = UseableMemChunkIterator::new().last().unwrap();

        let start = {
            let last_addr = last_useable.base + last_useable.length + 1u64;
            let addr = PhysAddr::new(last_addr);
            let addr = addr.align_up(page_size.as_u64());
            super::virtual_mmap::translate_addr(addr)
        };

        Self {
            start,
            init_size: page_size,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack(pub VirtAddr);

impl Stack {
    pub fn new() -> Self {
        Self(MEM_STRUCTURE.kstart)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pml4 {
    pub virt: VirtAddr,
    pub phys: PhysAddr,
}

impl Pml4 {
    pub fn new() -> Self {
        let phys = FRAME_ALLOCATOR
            .lock()
            .allocate_frame()
            .unwrap()
            .start_address();
        let virt = MEM_STRUCTURE.hhdm + phys.as_u64();
        Self { phys, virt }
    }
}
