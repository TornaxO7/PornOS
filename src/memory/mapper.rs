use spin::{Mutex, MutexGuard, Once};
use x86_64::{registers::control::Cr3, structures::paging::OffsetPageTable, VirtAddr};

use crate::{serial_print, serial_println};

/// **S**uper **i**mpressive **m**a**p**per
static SIMP: Once<Mutex<OffsetPageTable>> = Once::new();

pub fn init() {
    serial_print!("SIMP...");

    SIMP.call_once(|| {
        let hhdm = super::get_hhdm();
        let (p4_table_frame, _) = Cr3::read();

        let p4_table = unsafe {
            &mut *(VirtAddr::new(hhdm + p4_table_frame.start_address().as_u64()).as_mut_ptr())
        };

        unsafe { Mutex::new(OffsetPageTable::new(p4_table, VirtAddr::new(hhdm))) }
    });

    serial_println!("OK");
}

pub fn get_simp<'a>() -> MutexGuard<'a, OffsetPageTable<'static>> {
    SIMP.get().expect("SIMP initialised").lock()
}
