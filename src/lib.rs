#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
#![feature(strict_provenance)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]
// #![featuwlqanre(return_position_impl_trait_in_trait)]
#![feature(const_maybe_uninit_zeroed)]

#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use scheduling::cooperative::kasync::{AsyncRuntime, Mutex};

extern crate alloc;

pub mod klib;
pub mod scheduling;
pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod memory;

async fn test_lock<'a>(mutex: &'a Mutex<'a, i32>) {
    let yes = mutex.lock().await;
    println!("{}", *yes);
}

/// Initialises the kernel after loading the page tables of Pornos.
pub fn init() -> ! {
    gdt::init();
    interrupt::init();
    memory::paging::init_heap();

    let mut runtime = AsyncRuntime::new();
    runtime.add(async {println!("ja");});
    runtime.run();

    hlt_loop();
}

#[cfg(feature = "test")]
pub fn tests() {
    memory::tests();
}

/// A simple function which let's the kernel loop forever.
pub fn hlt_loop() -> ! {
    println!("Entering halting loop...");
    loop {
        x86_64::instructions::hlt();
    }
}
