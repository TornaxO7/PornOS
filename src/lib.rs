#![no_std]
#![feature(abi_x86_interrupt)]

pub mod vga;
pub mod qemu;
pub mod serial;
pub mod cpu_exception;
pub mod idt;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
