#[cfg(feature = "global-allocator-dummy")]
mod dummy;

#[cfg(feature = "global-allocator-linked-list")]
mod linked_list;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error {:?}", layout);
}
