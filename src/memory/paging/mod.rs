//! Includes the different paging implementation.
mod frame_allocator;
mod physical_mmap;
mod utils;
mod alloc;

use core::{arch::asm, marker::PhantomData, ops::Range};

use spin::Once;
use x86_64::{
    structures::paging::{
        page_table::PageTableLevel, FrameAllocator, Page, PageSize, PageTable, PageTableFlags,
        PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use self::{frame_allocator::FRAME_ALLOCATOR, utils::table_wrapper::TableWrapper};

use crate::memory::{paging::physical_mmap::KernelData, HHDM};

use super::types::Bytes;

pub const HEAP_SIZE: usize = 0x1000;
lazy_static::lazy_static! {
    pub static ref HEAP_START: VirtAddr = VirtAddr::new(0x1000);
}

pub static PML4E_ADDR: Once<VirtAddr> = Once::new();

/// The amount of pages which should be used in the beginning for the stack.
/// == 64KiB
const STACK_INIT_PAGES: u64 = 16;
pub static STACK_START: Once<VirtAddr> = Once::new();

pub fn init() -> ! {
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
    p4_ptr: *mut PageTable,
    p4_phys_addr: PhysAddr,
}

impl<P: PageSize> KPagingConfigurator<P> {
    /// Creates a new pornos-paging-configurator
    pub fn new() -> Self {
        let pml4e_addr = FRAME_ALLOCATOR
            .write()
            .allocate_frame()
            .unwrap()
            .start_address();

        PML4E_ADDR.call_once(|| *HHDM + pml4e_addr.as_u64());
        let ptr = PML4E_ADDR.get().unwrap().as_mut_ptr() as *mut PageTable;

        Self {
            size: PhantomData,
            p4_phys_addr: pml4e_addr,
            p4_ptr: ptr,
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

        self.map_page(
            heap_page,
            None,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );
    }

    /// Creates a new stack mapping for the kernel.
    pub fn map_stack(&self) {
        // "- P::SIZE" to let the stack start in the allocated frame
        STACK_START
            .call_once(|| VirtAddr::new((HHDM.as_u64() - 1) & ((1 << 48) - 1)).align_down(4u64));
        let mut addr = *STACK_START.get().unwrap();

        for _page_num in 0..STACK_INIT_PAGES {
            let page = Page::from_start_address(addr.align_down(P::SIZE)).unwrap();

            self.map_page(
                page,
                None,
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );

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

            self.map_page(
                page,
                Some(page_frame),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );
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
    /// Maps a page to the given page_frame (if available) with the given flags.
    ///
    /// * `page`: The page to be mapped.
    /// * `page_frame`: If it's `Some`, then the page will be mapped to the given page frame,
    ///                 otherwise a new page frame will ba allocated.
    /// * `flags`: The flags for the given mapping.
    pub fn map_page(&self, page: Page, page_frame: Option<PhysFrame>, flags: PageTableFlags) {
        let mut table_wrapper = TableWrapper::new(self.p4_ptr);
        let mut level = PageTableLevel::Four;

        while let Some(lower_level) = level.next_lower_level() {
            let entry_index = match lower_level {
                PageTableLevel::Three => page.start_address().p4_index(),
                PageTableLevel::Two => page.start_address().p3_index(),
                PageTableLevel::One => page.start_address().p2_index(),
                _ => unreachable!("Ayo, '{:?}' shouldn't be here <.<", lower_level),
            };
            let table_entry = table_wrapper.get_entry(entry_index);

            let next_table_ptr = {
                let next_table_vtr_ptr = if table_entry.is_unused() {
                    let flags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT;
                    table_wrapper.set_page_frame(entry_index, None, flags);
                    *HHDM + table_wrapper.get_entry(entry_index).addr().as_u64()
                } else {
                    *HHDM + table_entry.addr().as_u64()
                };
                next_table_vtr_ptr.as_mut_ptr() as *mut PageTable
            };

            table_wrapper = TableWrapper::new(next_table_ptr);
            level = lower_level;
        }

        table_wrapper.set_page_frame(page.p1_index(), page_frame, flags);
    }

    /// Maps a range of pages in a romw.
    ///
    /// * `page`: The starting page which should be mapped.
    /// * `page_frame`: The starting page frame (if available) which should be mapped.
    ///                 If it's `None`, random page-frames are picked up then.
    /// * `len`: The amount of bytes which should be mapped in a row.
    /// * `flags`: The flags for each page.
    ///
    /// # Note
    /// If `page_frame` is `Some(...)`, then you **have to** make sure that, the range, starting
    /// from the given page frame until `start + len` is **a valid Physicall address range**!!!
    pub fn map_page_range(
        &self,
        page: Page,
        page_frame: Option<PhysFrame>,
        len: Bytes,
        flags: PageTableFlags,
    ) {
        for offset in (0..len.as_u64()).step_by(P::SIZE.try_into().unwrap()) {
            let page = {
                let addr = (page.start_address() + offset).align_down(P::SIZE);
                Page::from_start_address(addr).unwrap()
            };

            let page_frame = page_frame.map(|frame| {
                let addr = (frame.start_address() + offset).align_down(P::SIZE);
                PhysFrame::from_start_address(addr).unwrap()
            });

            self.map_page(page, page_frame, flags);
        }
    }

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

        self.map_page_range(page, Some(page_frame), len, flags);
    }
}
