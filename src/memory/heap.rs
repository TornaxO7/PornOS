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

    let biggest_entry = {
        let mut free_entries = super::get_free_entries();

        let mut biggest_entry = free_entries.next().unwrap();
        while let Some(entry) = free_entries.next() {
            if entry.length > biggest_entry.length {
                biggest_entry = entry;
            }
        }

        free_entries.heap = Some(biggest_entry);
        biggest_entry
    };

    let mut phys = PhysAddr::new(biggest_entry.base);
    let mut virt = VirtAddr::new(phys.as_u64() + hhdm);

    let simp = crate::SIMP.get().expect("SIMP initialised");
}
