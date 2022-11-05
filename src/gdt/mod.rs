pub mod tss;

use crate::{print, println};
use lazy_static::lazy_static;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

lazy_static! {
    // static ref GDT: GlobalDescriptorTable = {
    //     let mut gdt = GlobalDescriptorTable::new();
    //     gdt.add_entry(Descriptor::kernel_code_segment());
    //     gdt.add_entry(Descriptor::tss_segment(&tss::TSS));
    //     gdt
    // };
}

static GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

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
