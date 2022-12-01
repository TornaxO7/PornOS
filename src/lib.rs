#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
// #![feature(alloc_error_handler)]

#![allow(non_snake_case)]

// extern crate alloc;

pub mod util;
pub mod gdt;
mod interrupt;
pub mod io;
pub mod memory;

pub fn prolog_init() -> ! {
    gdt::init();
    interrupt::init();
    memory::init();
}

pub fn init() -> ! {
    println!("SIU!!!");

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
