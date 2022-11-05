pub mod tss;

use lazy_static::lazy_static;
use crate::{println, print};
use x86_64::{structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}, registers::segmentation::{CS, Segment}, instructions::tables::load_tss};

lazy_static! {
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::tss_segment(&tss::TSS));
        gdt
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

impl Selectors {
    fn code_selector(&self) -> SegmentSelector {
        self.code_selector
    }
}

pub fn init() {
    print!("GDT... ");
    GDT.load();

    // unsafe {
    //     CS::set_reg(GDT.1.code_selector);
    //     load_tss(GDT.1.tss_selector);
    // }

    println!("OK");
}
