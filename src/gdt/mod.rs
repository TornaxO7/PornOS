pub mod tss;

use crate::{print, println};
use lazy_static::lazy_static;
use x86_64::{
    registers::segmentation::{Segment, CS, DS, GS, FS, SS, ES},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_seg = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_seg = gdt.add_entry(Descriptor::kernel_data_segment());
        // gdt.add_entry(Descriptor::tss_segment(&tss::TSS));
        (gdt, Selectors {code_seg, data_seg})
    };
}

struct Selectors {
    code_seg: SegmentSelector,
    data_seg: SegmentSelector,
}

pub fn init() {
    print!("GDT ... ");

    x86_64::instructions::interrupts::disable();

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_seg);
        DS::set_reg(GDT.1.data_seg);
        ES::set_reg(GDT.1.data_seg);
        CS::set_reg(GDT.1.data_seg);
        SS::set_reg(GDT.1.data_seg);
        FS::set_reg(GDT.1.data_seg);
        GS::set_reg(GDT.1.data_seg);
    }

    println!("OK");
}
