#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
#![feature(strict_provenance)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]
#![feature(const_maybe_uninit_zeroed)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use scheduling::cooperative::kasync::{AsyncRuntime, Mutex};

extern crate alloc;

pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod klib;
pub mod memory;
pub mod scheduling;

/// Initialises the kernel after loading the page tables of Pornos.
pub fn init() -> ! {
    gdt::init();
    interrupt::init();
    memory::paging::init_heap();

    let mut runtime = AsyncRuntime::new();
    assert!(runtime.run().is_ok());

    hlt_loop();
}

#[cfg(feature = "test")]
pub fn tests() {
    memory::tests::main();
    scheduling::tests::main();

    println!("All testes passed! Hooray!");
}

/// A simple function which let's the kernel loop forever.
pub fn hlt_loop() -> ! {
    println!("Entering halting loop...");
    loop {
        x86_64::instructions::hlt();
    }
}
