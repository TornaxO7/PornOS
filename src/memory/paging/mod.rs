//! Includes the different paging implementation.
mod frame_allocator;
mod physical_mmap;

use core::marker::PhantomData;

use lazy_static::lazy_static;
pub use physical_mmap::{PhysLinearAddr, PhysMemMap};
use spin::{RwLock, Once};
use x86_64::{
    structures::paging::{
        page_table::{PageTableEntry, PageTableLevel},
        Page, PageSize, PageTable, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use self::frame_allocator::{FrameManager, Stack, FRAME_ALLOCATOR};

use crate::memory::HHDM;

lazy_static! {
    pub static ref HEAP_START: VirtAddr = *HHDM;
}

/// The amount of pages which should be used in the beginning for the stack.
/// == 64KiB
const STACK_INIT_PAGES: u64 = 16;
pub const STACK_START: Once<VirtAddr> = Once::new();

pub fn init() -> ! {
    let phys_mmap = PhysMemMap::<Size4KiB>::new();
    FRAME_ALLOCATOR.call_once(|| RwLock::new(Stack::new(&phys_mmap)));

    let p_configurator = KPagingConfigurator::<Size4KiB>::new(&phys_mmap);
    p_configurator.map_kernel();
    p_configurator.map_heap();
    p_configurator.map_stack();

    crate::init();
}

#[cfg(feature = "test")]
pub fn tests() {
    let phys_mmap: PhysMemMap<Size4KiB> = PhysMemMap::new();
    frame_allocator::tests(&phys_mmap);
}

/// The paging configurator which sets up the different paging levels.
///
/// # SAFETY
/// It assumes, that we are still using the paging table of Limine!!!!
#[derive(Debug, Clone)]
pub struct KPagingConfigurator<'a, P: PageSize> {
    size: PhantomData<P>,
    phys_mmap: &'a PhysMemMap<P>,
    p4_ptr: *mut PageTable,
}

impl<'a, P: PageSize> KPagingConfigurator<'a, P> {
    pub fn new(phys_mmap: &'a PhysMemMap<P>) -> Self {
        let pml4e_addr = *HHDM + Self::get_free_virt_frame().as_u64();
        Self {
            size: PhantomData,
            phys_mmap,
            p4_ptr: pml4e_addr.as_mut_ptr() as *mut PageTable,
        }
    }

    /// This maps the kernel and its modules to the same virtual address as the given virtual
    /// address of limine.
    pub fn map_kernel(&self) {
        let mut kernel_iter = self.phys_mmap.into_iter_mem_chunk();
        while let Some(mmap) = kernel_iter.next() {
            for offset in (0..mmap.len).step_by(P::SIZE.try_into().unwrap()) {
                let virt_kframe_addr = *HHDM + mmap.base + offset;
                let page_frame_ptr = virt_kframe_addr.as_mut_ptr() as *const Page;
                let page = unsafe { page_frame_ptr.read() };
                self.map_existing_frame(page);
            }
        }
    }

    /// Maps a heap for the kernel.
    pub fn map_heap(&self) {
        let heap_page = Page::from_start_address(*HHDM).unwrap();
        let heap_page_frame = PhysFrame::from_start_address(Self::get_free_phys_frame()).unwrap();

        self.map_page(heap_page, heap_page_frame);
    }

    /// Creates a new stack mapping for the kernel.
    pub fn map_stack(&self) {
        // "- P::SIZE" to let the stack start in the allocated frame
        STACK_START.call_once(|| VirtAddr::new_truncate(u64::MAX));
        let mut addr = STACK_START.get().unwrap().clone();

        for _page_num in 0..STACK_INIT_PAGES {
            let phys_frame = {
                let phys_addr = Self::get_free_phys_frame();
                PhysFrame::from_start_address(phys_addr).unwrap()
            };

            let virt_frame = Page::from_start_address(addr).unwrap();
            self.map_page(virt_frame, phys_frame);

            addr -= P::SIZE;
        }
    }
}

impl<'a, P: PageSize + 'a> KPagingConfigurator<'a, P> {
    /// Maps the given virtual page to the given physical page/frame.
    pub fn map_page(&self, vpage: Page, ppage: PhysFrame) {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        let p1_entry_ptr = self.get_page_table_entry_ptr(vpage.start_address());

        let new_page_entry = {
            let mut page_entry = PageTableEntry::new();
            page_entry.set_frame(ppage, flags);
            page_entry
        };

        unsafe { p1_entry_ptr.write(new_page_entry) };
    }

    /// This function maps the given frame as the paging of limine does.
    ///
    /// This is useful to add already used entries which can't be changed anymore like the kernel
    /// code and its modules.
    pub fn map_existing_frame(&self, frame: Page) {
        // u8: Just a type to be able to use the function
        self.get_mem_mut_ptr::<u8>(frame.start_address());
    }

    /// Returns a pointer to physical address of the given addr.
    pub fn get_mem_mut_ptr<T>(&self, addr: VirtAddr) -> *mut T {
        let page_table_entry = {
            let page_table_entry_ptr = self.get_page_table_entry_ptr(addr);
            unsafe { page_table_entry_ptr.read() }
        };

        let page_frame_phys_addr = page_table_entry.addr();
        let page_frame_virt_addr = *HHDM + page_frame_phys_addr.as_u64();
        let value_virt_addr: VirtAddr = page_frame_virt_addr + u64::from(addr.page_offset());
        value_virt_addr.as_mut_ptr() as *mut T
    }

    /// Returns a pointer to the page entry (the entry of the page table (level 1)).
    pub fn get_page_table_entry_ptr(&self, addr: VirtAddr) -> *mut PageTableEntry {
        let new_table_entry_flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        let mut table = unsafe { self.p4_ptr.read() };
        let mut level = PageTableLevel::Four;

        while let Some(lower_level) = level.next_lower_level() {
            let table_entry = match lower_level {
                PageTableLevel::Three => &mut table[addr.p4_index()],
                PageTableLevel::Two => &mut table[addr.p3_index()],
                PageTableLevel::One => &mut table[addr.p2_index()],
                _ => unreachable!("Ayo, '{:?}' shouldn't be here <.<", lower_level),
            };

            let next_table_vtr_ptr = if table_entry.is_unused() {
                let new_frame = Self::get_free_phys_frame();
                table_entry.set_addr(new_frame, new_table_entry_flags);

                *HHDM + new_frame.as_u64()
            } else {
                *HHDM + table_entry.addr().as_u64()
            };
            let next_table_ptr = next_table_vtr_ptr.as_mut_ptr() as *mut PageTable;

            table = unsafe { next_table_ptr.read() };
            level = lower_level;
        }

        let page_table_entry_phys_addr = &table[addr.p1_index()].addr();
        let page_table_entry_virt_addr = *HHDM + page_table_entry_phys_addr.as_u64();
        page_table_entry_virt_addr.as_mut_ptr() as *mut PageTableEntry
    }

    /// Returns the physical address of a free page frame.
    fn get_free_phys_frame() -> PhysAddr {
        let new_frame = FRAME_ALLOCATOR
            .get()
            .unwrap()
            .write()
            .get_free_frame()
            .unwrap();
        new_frame.start_address()
    }

    /// Returns the virtual address of a free page frame.
    fn get_free_virt_frame() -> VirtAddr {
        let phys_frame = Self::get_free_phys_frame();
        let new_frame_virt_addr = *HHDM + phys_frame.as_u64();
        new_frame_virt_addr
    }
}
