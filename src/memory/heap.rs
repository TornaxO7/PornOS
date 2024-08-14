use good_memory_allocator::SpinLockedAllocator;
use x86_64::{
    structures::paging::{
        Mapper, OffsetPageTable, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::{serial_print, serial_println};

struct HeapInfo {
    start: VirtAddr,
    length: usize,
}

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub fn init() {
    serial_print!("Init Heap... ");

    // prepare the heap cause I'm a sheep... eh what?
    let heap_info = prepare_heap::<Size4KiB>();

    unsafe {
        ALLOCATOR.init(heap_info.start.as_u64() as usize, heap_info.length);
    }

    serial_println!("OK");
}

fn prepare_heap<S: PageSize>() -> HeapInfo
where
    OffsetPageTable<'static>: Mapper<S>,
    S: core::fmt::Debug,
{
    todo!()
}
