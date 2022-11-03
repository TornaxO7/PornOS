use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::println;

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt
    };
}

pub fn init() {
    println!("Setting up IDT...");
    IDT.load();
}
