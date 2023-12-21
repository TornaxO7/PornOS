use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn handler(_: InterruptStackFrame, _error_code: u64) {
    todo!();
}
