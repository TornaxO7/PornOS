pub mod tss;

use crate::{print, println};
use lazy_static::lazy_static;
use x86_64::{
    registers::segmentation::{Segment, CS, DS, SS, GS, FS, ES},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kcode_seg = gdt.add_entry(Descriptor::kernel_code_segment());
        let kdata_seg = gdt.add_entry(Descriptor::kernel_data_segment());
        // gdt.add_entry(Descriptor::tss_segment(&tss::TSS));
        (gdt, Selectors {kcode_seg, kdata_seg})
    };
}

struct Selectors {
    kcode_seg: SegmentSelector,
    kdata_seg: SegmentSelector,
}

pub fn init() {
    print!("GDT ... ");

    x86_64::instructions::interrupts::disable();

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kcode_seg);
        DS::set_reg(GDT.1.kdata_seg);
        ES::set_reg(GDT.1.kdata_seg);
        FS::set_reg(GDT.1.kdata_seg);
        GS::set_reg(GDT.1.kdata_seg);
        SS::set_reg(GDT.1.kdata_seg);
    }

    // println!("OK");
}
