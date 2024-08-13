use spin::Once;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{OffsetPageTable, PageTable},
    VirtAddr,
};

use crate::{serial_print, serial_println};

/// **S**uper **i**mpressive **m**a**p**per
pub static SIMP: Once<OffsetPageTable> = Once::new();

pub fn init() {
    serial_print!("SIMP... ");

    SIMP.call_once(|| {
        let hhdm = super::HHDM_REQUEST.get_response().unwrap().offset();
        let (page_table_frame, _) = Cr3::read();

        let page_table: &'static mut PageTable = {
            let ptr = VirtAddr::new(page_table_frame.start_address().as_u64() + hhdm)
                .as_mut_ptr::<PageTable>();

            unsafe { &mut *ptr }
        };

        unsafe { OffsetPageTable::new(page_table, VirtAddr::new(hhdm)) }
    });

    serial_println!("OK");
}
