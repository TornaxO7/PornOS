use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn handler(_: InterruptStackFrame, _error_code: PageFaultErrorCode) {
    todo!("Bruh");
}
