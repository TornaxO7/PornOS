#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
#![feature(strict_provenance)]
#![feature(alloc_error_handler)]

#![allow(non_snake_case)]

extern crate alloc;

use alloc::boxed::Box;

pub mod util;
pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod memory;

pub fn init() -> ! {
    gdt::init();
    interrupt::init();

    let yes = Box::new(20);

    println!("Entering infinity-loop...");
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
