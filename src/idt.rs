use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{serial_print, serial_println};

static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn init() {
    serial_print!("IDT...");

    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(handle_divide_error);
        idt.debug.set_handler_fn(handle_debug);
        idt.non_maskable_interrupt
            .set_handler_fn(handle_non_maskable_interrupt);
        idt.breakpoint.set_handler_fn(handle_breakpoint);
        idt.overflow.set_handler_fn(handle_overflow);
        idt.bound_range_exceeded
            .set_handler_fn(handle_bound_range_exceeded);
        idt.invalid_opcode.set_handler_fn(handle_invalid_opcode);
        idt.device_not_available
            .set_handler_fn(handle_device_not_available);
        idt.double_fault.set_handler_fn(handle_double_fault);
        idt.invalid_tss.set_handler_fn(handle_invalid_tss);
        idt.segment_not_present
            .set_handler_fn(handle_segment_not_present);
        idt.stack_segment_fault
            .set_handler_fn(handle_stack_segment_fault);
        idt.general_protection_fault
            .set_handler_fn(handle_general_protection_fault);
        idt.page_fault.set_handler_fn(handle_page_fault);
        idt.x87_floating_point
            .set_handler_fn(handle_x87_floating_point);
        idt.alignment_check.set_handler_fn(handle_alignment_check);
        idt.machine_check.set_handler_fn(handle_machine_check);
        idt.simd_floating_point
            .set_handler_fn(handle_simd_floating_point);
        idt.virtualization.set_handler_fn(handle_virtualization);
        idt.cp_protection_exception
            .set_handler_fn(handle_cp_protection_exception);
        idt.hv_injection_exception
            .set_handler_fn(handle_hv_injection_exception);
        idt.vmm_communication_exception
            .set_handler_fn(handle_vmm_communication_exception);
        idt.security_exception
            .set_handler_fn(handle_security_exception);

        idt
    });

    IDT.get().unwrap().load();

    serial_println!("OK");
}

extern "x86-interrupt" fn handle_divide_error(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_debug(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_non_maskable_interrupt(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_breakpoint(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_overflow(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_bound_range_exceeded(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_invalid_opcode(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_device_not_available(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_double_fault(_frame: InterruptStackFrame, _error_code: u64) -> ! {
    todo!()
}

extern "x86-interrupt" fn handle_invalid_tss(_frame: InterruptStackFrame, _error_code: u64) {
    todo!()
}

extern "x86-interrupt" fn handle_segment_not_present(
    _frame: InterruptStackFrame,
    _error_code: u64,
) {
    todo!()
}

extern "x86-interrupt" fn handle_stack_segment_fault(
    _frame: InterruptStackFrame,
    _error_code: u64,
) {
    todo!()
}

extern "x86-interrupt" fn handle_general_protection_fault(
    _frame: InterruptStackFrame,
    _error_code: u64,
) {
    todo!()
}

extern "x86-interrupt" fn handle_page_fault(
    _frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    todo!()
}

extern "x86-interrupt" fn handle_x87_floating_point(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_alignment_check(_frame: InterruptStackFrame, _error_code: u64) {
    todo!()
}

extern "x86-interrupt" fn handle_machine_check(_frame: InterruptStackFrame) -> ! {
    todo!()
}

extern "x86-interrupt" fn handle_simd_floating_point(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_virtualization(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_cp_protection_exception(
    _frame: InterruptStackFrame,
    _error_code: u64,
) {
    todo!()
}

extern "x86-interrupt" fn handle_hv_injection_exception(_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_vmm_communication_exception(
    _frame: InterruptStackFrame,
    _err_code: u64,
) {
    todo!()
}

extern "x86-interrupt" fn handle_security_exception(_frame: InterruptStackFrame, _error_code: u64) {
    todo!()
}
