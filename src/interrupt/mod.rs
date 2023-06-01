mod gate;

use gate::Gate;

use core::arch::asm;

use crate::{print, println};

lazy_static::lazy_static! {
    static ref IDT: InterruptDiscriptorTable = {
        let mut idt = InterruptDiscriptorTable::new();
        idt.debug_exception_handler(&debug_exception_handler);
        idt
    };
}

pub fn init() {
    print!("IDT ... ");

    unsafe {
        IDT.load();
    }

    println!("OK");
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
        let idtr: u64 = self.get_addr();

        // TODO: Check what to give `lidt` as an argument.
        unsafe {
            asm! {
                "lidt [{}]",
                in(reg) idtr,
                options(nomem, nostack),
            }
        }
    }

    /// Returns the starting address of the interrupt descriptor table.
    fn get_addr(&self) -> u64 {
        // TODO: Cast `self` into the adress and add th needed values to the
        // return address
        todo!();
    }
}

impl InterruptDiscriptorTable {
    pub fn debug_exception_handler(&mut self, handler: impl Fn() -> ()) {
    }
}

fn debug_exception_handler() {
}
