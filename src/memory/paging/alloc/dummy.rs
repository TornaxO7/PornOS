use core::alloc::GlobalAlloc;

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        panic!("[ALLOCATOR] Dummy::dealloc got called!!!");
    }
}
