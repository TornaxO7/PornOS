use lazy_static::lazy_static;
use x86_64::{
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        set_general_handler!(&mut idt, general_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

fn general_handler(_stack_frame: InterruptStackFrame, _index: u8, _error_code: Option<u64>) {
    println!("Hallo du Hurensohn");
    loop {}
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT REACHED\n{:#?}", stack_frame);
}
