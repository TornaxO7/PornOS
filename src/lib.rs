#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
#![feature(strict_provenance)]
#![feature(alloc_error_handler)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

extern crate alloc;

pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod memory;
pub mod util;

pub fn init() -> ! {
    gdt::init();
    interrupt::init();
    memory::paging::init_heap();

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
