use lazy_static::lazy_static;
use x86_64::{structures::idt::{InterruptDescriptorTable, InterruptStackFrame}, set_general_handler};

use crate::println;

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        set_general_handler!(&mut idt, general_handler);

        idt
    };
}

fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("{:?}, {}, {:?}", stack_frame, index, error_code);
}

pub fn init() {
    println!("Setting up IDT...");
    IDT.load();
    println!("Finished setting up IDT...");
}
