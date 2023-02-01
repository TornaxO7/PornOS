use core::arch::asm;

use crate::{print, println};

lazy_static::lazy_static! {
}

pub fn init() {
    print!("IDT ... ");
    println!("OK");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum GateType {
    Reserved1       = 0b0000,
    TSS16Available  = 0b0001,
    LDT             = 0b0010,
    TSS16Busy       = 0b0011,
    CallGate16      = 0b0100,
    TaskGate        = 0b0101,
    InterruptGate16 = 0b0110,
    TrapGate16      = 0b0111,
    Reserved2       = 0b1000,
    TSS32Available  = 0b1001,
    Reserved3       = 0b1010,
    TSS32Busy       = 0b1011,
    CallGate32      = 0b1100,
    Reserved4       = 0b1101,
    InterruptGate32 = 0b1110,
    TrapGate32      = 0b1111,
}

impl GateType {
    /// Converts itself into the bits which can then be simply ORed with the
    /// bits of GateOptions.
    pub fn to_gate_option_bits(self) -> u16 {
        u16::from((self as u8) << 8)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct GateOptions(u16);

impl GateOptions {
    pub fn none() -> Self {
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
        self.0 |= gate_type.to_gate_option_bits();
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
        Self::none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
struct Gate {
    offset1: u16,
    segment_selector: u16,
    options: GateOptions,
    offset2: u16,
    offset3: u32,
    reserved: u32,
}

impl Gate {
    pub fn empty() -> Self {
        Gate::default()
    }
}

/// Alignment recommended by the intel manual (Volume 3, Page 201)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C, align(8))]
struct InterruptDiscriptorTable {
    divide_error: Gate,
    debug_exception: Gate,
    nmi_interrupt: Gate,
    breakpoint: Gate,
    overflow: Gate,
    bound_range_exceeded: Gate,
    invalid_opcode: Gate,
    device_not_available: Gate,
    double_fault: Gate,
    invalid_TSS: Gate,
    segment_not_present: Gate,
    stack_segment_fault: Gate,
    general_protection: Gate,
    page_fault: Gate,
    reserved1: Gate,
    x87_FPU_floating_point_error: Gate,
    alignment_check: Gate,
    machine_check: Gate,
    simd_floating_point_exception: Gate,
    virtualization_exception: Gate,
    control_protection_exception: Gate,
    reserved2: [Gate; 31 - 22],
}

impl InterruptDiscriptorTable {
    pub fn new() -> Self {
        Self::default()
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
