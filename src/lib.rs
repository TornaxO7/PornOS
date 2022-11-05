#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(non_snake_case)]

pub mod gdt;
mod interrupt;
pub mod io;

pub fn entry() {
    interrupt::init();
    gdt::init();
}
