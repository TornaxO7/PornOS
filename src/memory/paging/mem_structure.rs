use {
    lazy_static::lazy_static,
    limine::LimineHhdmRequest,
    spin::Once,
    x86_64::{
        structures::paging::{FrameAllocator, PageSize},
        PhysAddr, VirtAddr,
    },
};

use crate::memory::types::Bytes;

use super::{
    physical_mmap::{
        frame_allocator::FRAME_ALLOCATOR,
        limine::{get_mmaps, iterators::UseableMemChunkIterator},
    },
    virtual_mmap::{VMMapperGeneral, SIMP},
};

static HHDM_REQUEST: LimineHhdmRequest = LimineHhdmRequest::new(0);

lazy_static! {
    pub static ref MEM_STRUCTURE: MemStructure = MemStructure {
        hhdm: VirtAddr::new(HHDM_REQUEST.get_response().get().unwrap().offset),
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
pub struct Heap(pub VirtAddr);

impl Heap {
    pub fn new(page_size: Bytes) -> Self {
        let mut last_useable_addr = {
            let last_useable = UseableMemChunkIterator::new().last().unwrap();
            let addr = PhysAddr::new(last_useable.base);
            SIMP.lock().translate_addr(addr)
        };

        let addr = last_useable_addr.align_up(page_size.as_u64());
        Self(addr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stack(pub VirtAddr);

impl Stack {
    pub fn new(page_size: Bytes) -> Self {
        let mut stack_addr: VirtAddr = MEM_STRUCTURE.kstart - 1u64;
        Self(stack_addr.align_down(page_size.as_u64()))
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
            .write()
            .allocate_frame()
            .unwrap()
            .start_address();
        let virt = MEM_STRUCTURE.hhdm + phys.as_u64();
        Self { phys, virt }
    }
}
