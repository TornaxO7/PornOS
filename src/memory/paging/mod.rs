//! Includes the different paging implementation.
mod alloc;
mod frame_allocator;
mod physical_mmap;
mod virtual_mmap;
mod utils;

use core::{arch::asm, marker::PhantomData, ops::Range};

use spin::Once;
use x86_64::{
    structures::paging::{FrameAllocator, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

use self::{
    frame_allocator::FRAME_ALLOCATOR,
    virtual_mmap::{SIMP, vmmap_traits::VMMapperMap},
};

use crate::memory::{paging::physical_mmap::KernelData, HHDM};

use super::types::Bytes;

pub const HEAP_SIZE: usize = 0x1000;
lazy_static::lazy_static! {
    pub static ref HEAP_START: VirtAddr = VirtAddr::new(0x1000);
}

pub static PML4E_ADDR: Once<PhysAddr> = Once::new();

/// The amount of pages which should be used in the beginning for the stack.
/// == 64KiB
const STACK_INIT_PAGES: u64 = 16;
pub static STACK_START: Once<VirtAddr> = Once::new();

pub fn init() -> ! {
    let pml4e_addr = FRAME_ALLOCATOR
        .write()
        .allocate_frame()
        .unwrap()
        .start_address();

    PML4E_ADDR.call_once(|| pml4e_addr.clone());

    let p_configurator = KPagingConfigurator::<Size4KiB>::new();
    p_configurator.map_kernel();
    p_configurator.map_heap();
    p_configurator.map_stack();
    p_configurator.map_frame_allocator();

    p_configurator.switch_paging();
}

pub fn init_heap() {
    alloc::init();
}

#[cfg(feature = "test")]
pub fn tests() {
    frame_allocator::tests();
}

/// The paging configurator which sets up the different paging levels.
///
/// # SAFETY
/// It assumes, that we are still using the paging table of Limine!!!!
#[derive(Debug, Clone)]
pub struct KPagingConfigurator<P: PageSize> {
    size: PhantomData<P>,
    p4_phys_addr: PhysAddr,
}

impl<P: PageSize> KPagingConfigurator<P> {
    /// Creates a new pornos-paging-configurator
    pub fn new() -> Self {
        let p4_phys_addr = PML4E_ADDR.get().unwrap().clone();

        Self {
            size: PhantomData,
            p4_phys_addr,
        }
    }

    /// This maps the kernel and its modules to the same virtual address as the given virtual
    /// address of limine.
    pub fn map_kernel(&self) {
        let data = KernelData::<P>::new();

        self.map_kernel_part(
            data.start_phys,
            data.start_virt,
            data.code,
            PageTableFlags::PRESENT,
        );
        self.map_kernel_part(
            data.start_phys,
            data.start_virt,
            data.read_only,
            PageTableFlags::PRESENT | PageTableFlags::NO_EXECUTE,
        );
        self.map_kernel_part(
            data.start_phys,
            data.start_virt,
            data.data,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
        );
    }

    /// Map a heap for the kernel.
    pub fn map_heap(&self) {
        let heap_page = Page::from_start_address(*HEAP_START).unwrap();

        unsafe {
            SIMP.lock().map_page(
                heap_page,
                None,
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            )
        };
    }

    /// Creates a new stack mapping for the kernel.
    pub fn map_stack(&self) {
        // "- P::SIZE" to let the stack start in the allocated frame
        STACK_START.call_once(|| {
            let mut stack_addr = (HHDM.as_u64() - 4u64) & ((1 << 48) - 1);
            stack_addr -= P::SIZE;
            VirtAddr::new(stack_addr).align_down(P::SIZE)
        });
        let mut addr = *STACK_START.get().unwrap();

        for _page_num in 0..STACK_INIT_PAGES {
            let page = Page::from_start_address(addr.align_down(P::SIZE)).unwrap();

            unsafe {
                SIMP.lock().map_page(
                    page,
                    None,
                    PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                )
            };

            addr -= P::SIZE;
        }
    }

    /// Maps the pages of the frame allocator.
    pub fn map_frame_allocator(&self) {
        let stack_page_frames = { FRAME_ALLOCATOR.read().get_frame_allocator_page_frames() };
        for page_frame in stack_page_frames {
            let page: Page = {
                let page_addr = *HHDM + page_frame.start_address().as_u64();
                Page::from_start_address(page_addr).unwrap()
            };

            unsafe {
                SIMP.lock().map_page(
                    page,
                    Some(page_frame),
                    PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                )
            };
        }
    }
}

impl<P: PageSize> KPagingConfigurator<P> {
    /// Switches the page-tables of limine with the custom one.
    pub fn switch_paging(&self) -> ! {
        let p4_phys_addr = self.p4_phys_addr.as_u64();
        let stack_start = STACK_START.get().unwrap().as_u64();
        unsafe {
            asm! {
                "mov r9, {1}",
                "mov r8, {0}",
                "mov rsp, r9",
                "mov rbp, r9",
                "mov cr3, r8",
                in(reg) p4_phys_addr,
                in(reg) stack_start,
                inout("r8") 0 => _,
                inout("r9") 0 => _,
            }
        }
        crate::init();
    }
}

impl<P: PageSize> KPagingConfigurator<P> {
    pub fn map_kernel_part(
        &self,
        phys_kernel_start: PhysAddr,
        virt_kernel_start: VirtAddr,
        range: Range<VirtAddr>,
        flags: PageTableFlags,
    ) {
        let len = Bytes::new(range.end - range.start);
        let page = Page::from_start_address(range.start).unwrap();

        let page_frame = {
            let offset = range.start - virt_kernel_start;
            PhysFrame::from_start_address(phys_kernel_start + offset).unwrap()
        };

        unsafe {
            SIMP.lock()
                .map_page_range(page, Some(page_frame), len, flags)
        };
    }
}
