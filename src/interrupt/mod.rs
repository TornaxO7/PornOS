mod divide_error;
mod debug;
mod non_maskable_interrupt;
mod breakpoint;
mod overflow;
mod bound_range_exceeded;
mod invalid_opcode;
mod device_not_available;
mod double_fault;
mod invalid_tss;
mod segment_not_present;
mod stack_segment_fault;
mod general_protection_fault;
mod page_fault;
mod x87_floating_point;
mod alignment_check;
mod machine_check;
mod simd_floating_point;
mod vmm_communication_exception;
mod security_exception;

use lazy_static::lazy_static;
use x86_64::{structures::idt::{InterruptDescriptorTable, InterruptStackFrame}, set_general_handler};

use crate::println;

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        set_general_handler!(&mut idt, general_handler);

        idt.divide_error.set_handler_fn(divide_error::handler);
        idt.debug.set_handler_fn(debug::handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt::handler);
        idt.breakpoint.set_handler_fn(breakpoint::handler);
        idt.overflow.set_handler_fn(overflow::handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded::handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode::handler);
        idt.device_not_available.set_handler_fn(device_not_available::handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault::handler)
                .set_stack_index(super::gdt::tss::DOUBLE_FAULT_IST_INDEX);
        }
        idt.invalid_tss.set_handler_fn(invalid_tss::handler);
        idt.segment_not_present.set_handler_fn(segment_not_present::handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault::handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault::handler);
        idt.page_fault.set_handler_fn(page_fault::handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point::handler);
        idt.alignment_check.set_handler_fn(alignment_check::handler);
        idt.machine_check.set_handler_fn(machine_check::handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point::handler);
        idt.vmm_communication_exception.set_handler_fn(vmm_communication_exception::handler);

        idt
    };
}

pub fn init() {
    println!("Setting up IDT...");
    IDT.load();
    println!("Finished setting up IDT...");
}

fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("{:?}, {}, {:?}", stack_frame, index, error_code);
}
