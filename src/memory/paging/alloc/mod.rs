#[cfg(feature = "global-allocator-dummy")]
mod dummy;

#[cfg(feature = "global-allocator-linked-list")]
mod linked_list;

pub fn init() {
    #[cfg(feature = "global-allocator-linked-list")]
    linked_list::init_heap();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error {:?}", layout);
}
