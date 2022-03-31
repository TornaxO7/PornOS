use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_itd() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack: InterruptStackFrame) {
    println!("Excepion: Breakpoint\n{:#?})", stack);
}
