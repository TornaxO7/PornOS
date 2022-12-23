use core::alloc::GlobalAlloc;
use core::ops::Deref;

use linked_list_allocator::LockedHeap;

use crate::memory::paging::{HEAP_SIZE, HEAP_START};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

pub fn init_heap() {
    unsafe {
        ALLOCATOR.init(HEAP_START.as_mut_ptr(), HEAP_SIZE);
    }
}

pub struct Allocator(LockedHeap);

impl Allocator {
    pub const fn new() -> Self {
        Self(LockedHeap::empty())
    }

    pub unsafe fn init(&self, heap_start: *mut u8, heap_size: usize) {
        let mut heap = self.0.lock();
        heap.init(heap_start, heap_size);
    }
}

impl Deref for Allocator {
    type Target = LockedHeap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.0.dealloc(ptr, layout)
    }
}
