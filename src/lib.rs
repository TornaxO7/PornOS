#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(non_snake_case)]

pub mod gdt;
mod interrupt;
pub mod memory;
pub mod io;

pub fn init() {
    gdt::init();
    interrupt::init();
    memory::level4_paging::init();
}
