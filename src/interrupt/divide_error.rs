use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn handler(_: InterruptStackFrame) {
    todo!();
}
