use good_memory_allocator::SpinLockedAllocator;
use limine::request::MemoryMapRequest;

use crate::{serial_print, serial_println};

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub fn init() {
    serial_print!("Init Heap... ");

    todo!();

    serial_println!("OK");
}
