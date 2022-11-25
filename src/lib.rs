#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
// #![feature(alloc_error_handler)]

#![allow(non_snake_case)]

// extern crate alloc;

pub mod util;
pub mod gdt;
mod interrupt;
pub mod io;
pub mod memory;

pub fn init() {
    gdt::init();
    interrupt::init();
    memory::init();
}
