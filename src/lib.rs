#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
// #![feature(alloc_error_handler)]

#![allow(non_snake_case)]

// extern crate alloc;

use io::TerminalOutput;

pub mod util;
pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod memory;

pub fn prolog_init() -> ! {
    gdt::init();
    interrupt::init();
    memory::init();
}

pub fn init() -> ! {
    // now use stdio
    // io::set_output(TerminalOutput::Serial);

    println!("Serial test");
    hlt_loop();
}

#[cfg(feature = "test")]
pub fn tests() {
    memory::tests();
}


pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
