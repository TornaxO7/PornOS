use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

// lazy_static!{ 
//     static ref IDT: InterruptDescriptorTable = {
//         let mut idt = InterruptDescriptorTable::new();
//         // idt.breakpoint.set_handler_fn(breakpoint_handler);
//         idt
//     };
// }

lazy_static!{
    static ref IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
}

fn general_handler(_stack_frame: InterruptStackFrame, _index: u8, _error_code: Option<u64>) {
    todo!("General handler TODO");
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT REACHED\n{:#?}", stack_frame);
}
