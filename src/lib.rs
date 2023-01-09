#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
#![feature(strict_provenance)]
#![feature(alloc_error_handler)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

extern crate alloc;

pub mod kasync;
pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod memory;

pub fn init() -> ! {
    gdt::init();
    interrupt::init();
    memory::paging::init_heap();

    let mut runtime = kasync::AsyncRuntime::new();
    runtime.run();
}

#[cfg(feature = "test")]
pub fn tests() {
    memory::tests();
}

pub fn hlt_loop() -> ! {
    println!("Entering halting loop...");
    loop {
        x86_64::instructions::hlt();
    }
}
