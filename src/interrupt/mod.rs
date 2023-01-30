use core::{arch::asm, marker::PhantomData};

use x86_64::VirtAddr;

use crate::{memory::types::Bytes, print, println};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GateType {
    Reserved1,
    TSS16Available,
    LDT,
    TSS16Busy,
    CallGate16,
    TaskGate,
    InterruptGate16,
    TrapGate16,
    Reserved2,
    TSS32Available,
    Reserved3,
    TSS32Busy,
    CallGate32,
    Reserved4,
    InterruptGate32,
    TrapGate32,
}

#[repr(transparent)]
struct GateOptions(u16);

impl GateOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_present(&mut self, value: bool) {
        if value {
            self.0 |= 1 << 16;
        } else {
            self.0 &= !(1 << 16);
        }
    }

    pub fn set_dpl(&mut self) {
        todo!()
    }

    pub fn set_type(&mut self, gate_type: GateType) {
        self.0 &= !(0b1111 << 8);
        self.0 |= {
            #[rustfmt::skip]
            let bitmap = match gate_type {
                // clear already happened above
                GateType::Reserved1       => 0b0000,
                GateType::TSS16Available  => 0b0001,
                GateType::LDT             => 0b0010,
                GateType::TSS16Busy       => 0b0011,
                GateType::CallGate16      => 0b0100,
                GateType::TaskGate        => 0b0101,
                GateType::InterruptGate16 => 0b0110,
                GateType::TrapGate16      => 0b0111,
                GateType::Reserved2       => 0b1000,
                GateType::TSS32Available  => 0b1001,
                GateType::Reserved3       => 0b1010,
                GateType::TSS32Busy       => 0b1011,
                GateType::CallGate32      => 0b1100,
                GateType::Reserved4       => 0b1101,
                GateType::InterruptGate32 => 0b1110,
                GateType::TrapGate32      => 0b1111,
            };

            bitmap << 8
        };
    }

    pub unsafe fn set_ist(&mut self, index: u8) {
        const MIN: u8 = 0;
        const MAX: u8 = 0b111;

        if index > MAX {
            panic!("Index has to be in the range: [{}, {}]", MIN, MAX);
        }

        self.0 &= !0b111u16;
        self.0 |= u16::from(index);
    }
}

impl Default for GateOptions {
    fn default() -> Self {
        Self(0)
    }
}

#[repr(C)]
struct Gate {
    offset1: u16,
    segment_selector: u16,
}

/// Alignment recommended by the intel manual (Volume 3, Page 201)
#[repr(C, align(8))]
struct IDT {}

impl IDT {
    pub fn new() -> Self {
        todo!()
    }

    pub unsafe fn load(&self) {
        let idtr: u64 = { todo!() };

        unsafe {
            asm! {
                "lidt [{}]",
                in(reg) idtr,
            }
        }
    }
}

pub fn init() {
    print!("IDT ... ");

    println!("OK");
}
