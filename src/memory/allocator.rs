use core::alloc::GlobalAlloc;

struct Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        todo!()
    }
}

#[alloc_error_handler]
fn allocate_error_handler(_: core::alloc::Layout) -> ! {
    panic!("Allocating panic!");
}
