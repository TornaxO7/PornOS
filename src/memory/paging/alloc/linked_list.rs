use linked_list_allocator::LockedHeap;

use crate::memory::paging::{HEAP_START, HEAP_SIZE};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START.as_mut_ptr(), HEAP_SIZE);
    }
}
