#[cfg(feature = "global-allocator-dummy")]
mod dummy;

#[cfg(feature = "global-allocator-linked-list-allocator-crate")]
mod linked_list;

pub fn init() {
    #[cfg(feature = "global-allocator-dummy")]
    dummy::init_heap();

    #[cfg(feature = "global-allocator-linked-list-allocator-crate")]
    linked_list::init_heap();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error {:?}", layout);
}
