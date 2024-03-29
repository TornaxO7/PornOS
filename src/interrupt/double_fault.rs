use x86_64::structures::idt::InterruptStackFrame;

use crate::println;

pub extern "x86-interrupt" fn handler(_: InterruptStackFrame, error_code: u64) -> ! {
    println!("Double fault: {:x}", error_code);
    loop {
        x86_64::instructions::hlt();
    }
}
