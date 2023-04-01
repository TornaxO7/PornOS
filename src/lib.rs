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

    #[cfg(not(feature = "test"))]
    start_pornos();

    #[cfg(feature = "test")]
    tests::main();

    hlt_loop();
}

#[cfg(not(feature = "test"))]
fn start_pornos() {
    x86_64::instructions::interrupts::enable();

    let mut runtime = AsyncRuntime::new();
    assert!(runtime.run().is_ok());
}

/// A simple function which let's the kernel loop forever.
pub fn hlt_loop() -> ! {
    println!("Entering halting loop...");
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(feature = "test")]
pub mod tests {
    use super::*;

    pub fn main() {
        println!("{0} TEST-MODE {0}", "=".repeat(5));

        memory::tests::main();
        scheduling::tests::main();

        println!("\nAll testes passed! Hooray!");
    }
}
