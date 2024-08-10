use {
    lazy_static::lazy_static,
    x86_64::{
        registers::segmentation::{Segment, CS, DS, ES, FS, GS, SS},
        structures::gdt::{Descriptor, DescriptorFlags, GlobalDescriptorTable, SegmentSelector},
    },
};

use crate::{print, println};

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let code_16bit = (DescriptorFlags::USER_SEGMENT
            | DescriptorFlags::PRESENT
            | DescriptorFlags::LIMIT_0_15
            | DescriptorFlags::ACCESSED
            | DescriptorFlags::EXECUTABLE)
            .bits();
        gdt.append(Descriptor::UserSegment(code_16bit));

        let data_16bit = (DescriptorFlags::USER_SEGMENT
            | DescriptorFlags::PRESENT
            | DescriptorFlags::LIMIT_0_15
            | DescriptorFlags::ACCESSED
            | DescriptorFlags::WRITABLE)
            .bits();
        gdt.append(Descriptor::UserSegment(data_16bit));

        let code_32bit = DescriptorFlags::KERNEL_CODE32.bits();
        gdt.append(Descriptor::UserSegment(code_32bit));

        let data_32bit = DescriptorFlags::KERNEL_DATA.bits();
        gdt.append(Descriptor::UserSegment(data_32bit));

        let code_64bit = DescriptorFlags::KERNEL_CODE64.bits();
        let kcode_seg = gdt.append(Descriptor::UserSegment(code_64bit));

        let data_64bit = DescriptorFlags::KERNEL_DATA.bits();
        let kdata_seg = gdt.append(Descriptor::UserSegment(data_64bit));

        (
            gdt,
            Selectors {
                kcode_seg,
                kdata_seg,
            },
        )
    };
}

struct Selectors {
    kcode_seg: SegmentSelector,
    kdata_seg: SegmentSelector,
}

pub fn init() {
    x86_64::instructions::interrupts::disable();

    print!("GDT ... ");
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kcode_seg);
        DS::set_reg(GDT.1.kdata_seg);
        ES::set_reg(GDT.1.kdata_seg);
        FS::set_reg(GDT.1.kdata_seg);
        GS::set_reg(GDT.1.kdata_seg);
        SS::set_reg(GDT.1.kdata_seg);
    }

    println!("OK");
}
