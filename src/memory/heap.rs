use good_memory_allocator::SpinLockedAllocator;
use x86_64::{structures::paging::Mapper, PhysAddr, VirtAddr};

use crate::{serial_print, serial_println};

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub fn init() {
    serial_print!("Init Heap... ");

    // prepare the heap cause I'm a sheep... eh what?
    prepare_heap();

    // unsafe {
    //     ALLOCATOR.init();
    // }

    serial_println!("OK");
}

fn prepare_heap() {
    let hhdm = super::get_hhdm();
    let biggest_entry = super::get_entries()
        .iter()
        .max_by(|a, b| a.length.cmp(&b.length))
        .map(|&entry| entry)
        .unwrap();

    let mut phys = PhysAddr::new(biggest_entry.base);
    let mut virt = VirtAddr::new(phys.as_u64() + hhdm);

    let simp = crate::SIMP.get().expect("SIMP initialised");
}
