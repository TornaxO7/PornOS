use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr::NonNull;

use linked_list_allocator::Heap;
use spin::Mutex;
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::PageSize;
use x86_64::structures::paging::PageTableFlags;
use x86_64::structures::paging::Size4KiB;
use x86_64::VirtAddr;

use crate::memory::paging::{
    frame_allocator::FRAME_ALLOCATOR, virtual_mmap::SIMP, VMMapperMap, HEAP_SIZE, HEAP_START,
};
use crate::memory::types::Bytes;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

pub fn init_heap() {
    unsafe {
        ALLOCATOR.init(HEAP_START.as_mut_ptr(), HEAP_SIZE);
    }
}

/// Some wrapper functions for the actual Global allocation.
///
/// # Safety
/// Basically the same safety rules as for the GlobalAlloc.
unsafe trait GlobalAllocWrapper<P: PageSize>: GlobalAlloc {
    unsafe fn wrap_alloc(&self, layout: Layout) -> *mut u8;

    unsafe fn wrap_dealloc(&self, ptr: *mut u8, layout: Layout);

    fn allocate_page_frame(&self, heap_top: VirtAddr) -> Result<Bytes, AllocationPageFrameError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllocationPageFrameError {
    NoFreeFrame,
}

/// The allocator for linked list.
//
// It has a `Mutex<LockedHeap>` because we need another lock since before
// allocating memory, we want to check first if there's even enough space in the
// linked list to allocated. Instead of getting a page fault, we want to
// allocate new memory first for the heap.
pub struct Allocator(Mutex<Heap>);

impl Allocator {
    pub const fn new() -> Self {
        Self(Mutex::new(Heap::empty()))
    }

    pub unsafe fn init(&self, heap_start: *mut u8, heap_size: usize) {
        let mut heap = self.0.lock();
        unsafe { heap.init(heap_start, heap_size) };
    }
}

unsafe impl GlobalAllocWrapper<Size4KiB> for Allocator {
    unsafe fn wrap_alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = self.0.lock();

        let has_enough_space = heap.free() > layout.size();
        if !has_enough_space {
            let heap_top_addr = {
                let heap_top_addr = VirtAddr::new(heap.top() as u64);
                heap_top_addr.align_down(Size4KiB::SIZE)
            };
            let added_size = self.allocate_page_frame(heap_top_addr).unwrap();
            unsafe { heap.extend(added_size.as_usize()) };
        }

        heap.allocate_first_fit(layout).unwrap().as_ptr()
    }

    unsafe fn wrap_dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }

        let mut heap = self.0.lock();
        unsafe { heap.deallocate(NonNull::new(ptr).unwrap(), layout) }
    }

    /// Requests a new page frame from the page frame allocator and adds it to
    /// the heap. *Stonks*
    fn allocate_page_frame(&self, heap_top: VirtAddr) -> Result<Bytes, AllocationPageFrameError> {
        let page_frame = FRAME_ALLOCATOR
            .write()
            .allocate_frame()
            .ok_or(AllocationPageFrameError::NoFreeFrame)?;

        let page = Page::<Size4KiB>::from_start_address(heap_top).unwrap();

        unsafe {
            SIMP.lock().map_page(
                page,
                Some(page_frame),
                PageTableFlags::WRITABLE | PageTableFlags::PRESENT | PageTableFlags::NO_CACHE,
            );
        }

        Ok(Bytes::new(Size4KiB::SIZE))
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.wrap_alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.wrap_dealloc(ptr, layout) }
    }
}
