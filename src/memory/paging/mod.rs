//! Includes the different paging implementation.
mod frame_allocator;
mod physical_mmap;

use core::{arch::asm, marker::PhantomData};

use lazy_static::lazy_static;
pub use physical_mmap::PhysMemMap;
use spin::{Once, RwLock};
use x86_64::{
    structures::paging::{
        page_table::{PageTableEntry, PageTableLevel},
        Page, PageSize, PageTable, PageTableFlags, PageTableIndex, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use self::frame_allocator::{Stack};

use crate::{memory::HHDM, println};

lazy_static! {
    pub static ref HEAP_START: VirtAddr = *HHDM;
}

/// The amount of pages which should be used in the beginning for the stack.
/// == 64KiB
const STACK_INIT_PAGES: u64 = 16;
pub static STACK_START: Once<VirtAddr> = Once::new();

pub fn init() -> ! {
    let phys_mmap = PhysMemMap::<Size4KiB>::new();
    // FRAME_ALLOCATOR.call_once(|| RwLock::new(Stack::new(&phys_mmap)));

    let p_configurator = KPagingConfigurator::<Size4KiB>::new(&phys_mmap);
    p_configurator.map_kernel();
    p_configurator.map_heap();
    p_configurator.map_stack();
    p_configurator.switch_paging();

    crate::init();
}

#[cfg(feature = "test")]
pub fn tests() {
    let phys_mmap: PhysMemMap<Size4KiB> = PhysMemMap::new();
    frame_allocator::tests(&phys_mmap);
}

#[derive(Debug, Clone)]
pub struct TableWrapper {
    ptr: *mut PageTable,
    data: PageTable,
}

impl TableWrapper {
    pub fn new(ptr: *mut PageTable) -> Self {
        Self {
            ptr,
            data: unsafe { ptr.read() },
        }
    }

    pub fn create_entry<P: PageSize>(&mut self, index: PageTableIndex, kpconf: &KPagingConfigurator<P>) -> PageTableEntry {
        let page_table_entry_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        // let new_frame = kpconf::get
        todo!()

    }

    /// Updates the entry at the given index in the page table and also writes that into the memory.
    pub fn update(&mut self, index: PageTableIndex, entry: PageTableEntry) {
        self.data[index] = entry;
        unsafe {
            self.ptr.write(self.data);
        }
    }
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
    p4_phys_addr: PhysAddr,
}

impl<'a, P: PageSize> KPagingConfigurator<'a, P> {
    pub fn new(phys_mmap: &'a PhysMemMap<P>) -> Self {
        let pml4e_addr = Self::get_free_phys_frame();
        let pml4e_virt_addr = *HHDM + pml4e_addr.as_u64();
        Self {
            size: PhantomData,
            phys_mmap,
            p4_phys_addr: pml4e_addr,
            p4_ptr: pml4e_virt_addr.as_mut_ptr() as *mut PageTable,
        }
    }

    /// This maps the kernel and its modules to the same virtual address as the given virtual
    /// address of limine.
    pub fn map_kernel(&self) {
        for kmmap in self.phys_mmap.into_iter_kernel_and_modules() {
            for offset in (0..kmmap.len).step_by(P::SIZE.try_into().unwrap()) {
                let page_frame = {
                    let page_frame_phys_addr = PhysAddr::new(kmmap.base + offset);
                    PhysFrame::from_start_address(page_frame_phys_addr).unwrap()
                };
                let page = {
                    let page_frame_virt_addr = *HHDM + page_frame.start_address().as_u64();
                    Page::from_start_address(page_frame_virt_addr).unwrap()
                };

                self.map_page(page, Some(page_frame));
            }
        }
    }

    /// Maps a heap for the kernel.
    pub fn map_heap(&self) {
        let heap_page = Page::from_start_address(*HHDM).unwrap();
        let heap_page_frame = PhysFrame::from_start_address(Self::get_free_phys_frame()).unwrap();

        self.map_page(heap_page, Some(heap_page_frame));
    }

    /// Creates a new stack mapping for the kernel.
    pub fn map_stack(&self) {
        // "- P::SIZE" to let the stack start in the allocated frame
        STACK_START.call_once(|| VirtAddr::new_truncate(u64::MAX).align_down(P::SIZE));
        let mut addr = *STACK_START.get().unwrap();

        for _page_num in 0..STACK_INIT_PAGES {
            let page_frame = {
                let phys_addr = Self::get_free_phys_frame();
                PhysFrame::from_start_address(phys_addr).unwrap()
            };

            let page = Page::from_start_address(addr).unwrap();
            self.map_page(page, Some(page_frame));

            addr -= P::SIZE;
        }
    }
}

impl<'a, P: PageSize> KPagingConfigurator<'a, P> {
    pub fn switch_paging(&self) {
        let p4_phys_addr = self.p4_phys_addr.as_u64() & !(0xFFF);
        unsafe {
            asm! {
                "xor r8, r8",
                "mov r8, {0}",
                "mov cr3, r8",
                in(reg) p4_phys_addr,
                inout("r8") 0 => _,
            }
        }
    }
}

impl<'a, P: PageSize + 'a> KPagingConfigurator<'a, P> {
    /// Maps the given virtual page to the given physical page-frame if it's set.
    /// If `page_frame` is `None` a new page frame will be mapped to the given page.
    pub fn map_page(&self, page: Page, page_frame: Option<PhysFrame>) {
        // let p1_table_ptr = self.get_p1_table(page);
        //
        // let table_entry_ptr = {
        //     let table_entry = &table[page.start_address().p1_index()];
        //     let table_entry_virt_addr = *HHDM + table_entry.addr().as_u64();
        //     table_entry_virt_addr.as_mut_ptr() as *mut PageTableEntry
        // };
        //
        // let new_page_entry = {
        //     let mut page_entry = PageTableEntry::new();
        //     let page_frame_addr = match page_frame {
        //         Some(pf) => pf.start_address(),
        //         None => Self::get_free_phys_frame(),
        //     };
        //     page_entry.set_addr(page_frame_addr, page_table_entry_flags);
        //     page_entry
        // };
        //
        // unsafe { table_entry_ptr.write(new_page_entry) }
    }

    fn get_p1_table(&self, page: Page) -> *mut PageTable {
        let mut table_wrapper = TableWrapper::new(self.p4_ptr);
        let mut level = PageTableLevel::Four;

        while let Some(lower_level) = level.next_lower_level() {
            let entry_index = match lower_level {
                PageTableLevel::Three => page.start_address().p4_index(),
                PageTableLevel::Two => page.start_address().p3_index(),
                PageTableLevel::One => page.start_address().p2_index(),
                _ => unreachable!("Ayo, '{:?}' shouldn't be here <.<", lower_level),
            };
            let table_entry = table_wrapper.data[entry_index];

            let next_table_ptr = {
                let next_table_vtr_ptr = if table_entry.is_unused() {
                    let new_table_entry = table_wrapper.create_entry(entry_index, &self);
                        // self.register_new_table_entry(table_entry, entry_index, &mut table_wrapper);
                    *HHDM + new_table_entry.addr().as_u64()
                } else {
                    *HHDM + table_entry.addr().as_u64()
                };
                next_table_vtr_ptr.as_mut_ptr() as *mut PageTable
            };

            table_wrapper = TableWrapper::new(next_table_ptr);
            level = lower_level;
        }

        table_wrapper.ptr
    }

    fn register_new_table_entry(
        &self,
        old_entry: PageTableEntry,
        entry_index: PageTableIndex,
        table_wrapper: &mut TableWrapper,
    ) -> PageTableEntry {
        let page_table_entry_flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        let new_frame = frame_allocator::get_free_phys_frame::<P>();

        let mut new_table_entry = old_entry.clone();
        new_table_entry.set_addr(new_frame.start_address(), page_table_entry_flags);
        table_wrapper.update(entry_index, new_table_entry);

        new_table_entry
    }
}
