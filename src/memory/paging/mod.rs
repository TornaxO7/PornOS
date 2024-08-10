//! This modules holds the implemenation of paging.
mod alloc;
mod mem_structure;
mod physical_mmap;
mod virtual_mmap;

use core::{arch::asm, marker::PhantomData, ops::Range};

use x86_64::{
    structures::paging::{Page, PageSize, PageTableFlags, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

use self::{
    mem_structure::{Heap, Pml4, Stack, MEM_STRUCTURE},
    physical_mmap::{
        frame_allocator::FRAME_ALLOCATOR, kernel_info::KernelData,
        limine::iterators::UseableMemChunkIterator,
    },
    virtual_mmap::{VMMapperMap, SIMP},
};

use super::types::Bytes;

pub fn init() -> ! {
    // NOTE: Page Frame Allocator is automatically initialised!
    MEM_STRUCTURE.pml4.call_once(Pml4::new);
    MEM_STRUCTURE
        .heap
        .call_once(|| Heap::new(Bytes::new(Size4KiB::SIZE)));
    MEM_STRUCTURE.stack.call_once(Stack::new);

    let p_configurator = KPagingConfigurator::<Size4KiB>::new();
    p_configurator.map_kernel();
    p_configurator.map_heap();
    p_configurator.map_stack();
    p_configurator.map_frame_allocator();
    p_configurator.map_page_frames();

    p_configurator.switch_paging();
}

pub fn init_heap() {
    alloc::init();
}

#[cfg(feature = "test")]
pub fn tests() {}

/// The paging configurator which sets up the different paging levels.
///
/// # SAFETY
/// It assumes, that we are still using the paging table of Limine!!!!
#[derive(Debug, Clone)]
pub struct KPagingConfigurator<P: PageSize> {
    size: PhantomData<P>,
    p4: PhysAddr,
}

impl<P: PageSize> KPagingConfigurator<P> {
    /// Creates a new pornos-paging-configurator
    pub fn new() -> Self {
        Self {
            size: PhantomData,
            p4: MEM_STRUCTURE.pml4.get().unwrap().phys,
        }
    }

    /// This maps the kernel and its modules to the same virtual address as the given virtual
    /// address of limine.
    pub fn map_kernel(&self) {
        let data = KernelData::<P>::from_limine();

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
        let heap = MEM_STRUCTURE.heap.get().unwrap();
        let heap_page = Page::from_start_address(heap.start).unwrap();

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
        const AMOUNT_STACK_PAGES: u64 = 16;
        let needed_bytes = Bytes::new(AMOUNT_STACK_PAGES * P::SIZE);

        let start_addr = {
            let addr = MEM_STRUCTURE.stack.get().unwrap().0;
            addr - needed_bytes.as_u64()
        };

        let starting_page = Page::from_start_address(start_addr).unwrap();

        unsafe {
            SIMP.lock().map_page_range(
                starting_page,
                None,
                needed_bytes,
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );
        }
    }

    /// Maps the pages which the page frame allocator uses.
    pub fn map_frame_allocator(&self) {
        let stack_page_frames = { FRAME_ALLOCATOR.lock().get_frame_allocator_page_frames() };
        for page_frame in stack_page_frames {
            let page: Page = {
                let page_addr = virtual_mmap::translate_addr(page_frame.start_address());
                Page::from_start_address(page_addr).unwrap()
            };

            unsafe {
                SIMP.lock().map_page(
                    page,
                    Some(page_frame),
                    PageTableFlags::PRESENT
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::NO_CACHE
                        | PageTableFlags::NO_EXECUTE,
                )
            };
        }
    }

    /// Identity maps the page frames to the HHDM.
    pub fn map_page_frames(&self) {
        for mem_chunk in UseableMemChunkIterator::new() {
            let starting_page = {
                let addr = { virtual_mmap::translate_addr(PhysAddr::new(mem_chunk.base)) };

                Page::from_start_address(addr).unwrap()
            };
            let starting_page_frame = {
                let addr = PhysAddr::new(mem_chunk.base);
                PhysFrame::from_start_address(addr).unwrap()
            };
            let len = Bytes::new(mem_chunk.length);

            unsafe {
                SIMP.lock().map_page_range(
                    starting_page,
                    Some(starting_page_frame),
                    len,
                    PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
                );
            }
        }
    }
}

impl<P: PageSize> KPagingConfigurator<P> {
    /// Switches the page-tables of limine with the custom one.
    pub fn switch_paging(&self) -> ! {
        let p4_phys_addr = self.p4.as_u64();
        let stack_start = MEM_STRUCTURE.stack.get().unwrap().0.as_u64();
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

impl<P: PageSize> Default for KPagingConfigurator<P> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "test")]
pub mod tests {
    use super::*;

    pub fn main() {
        physical_mmap::tests::main();
    }
}
