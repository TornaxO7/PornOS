#![no_std]
#![feature(abi_x86_interrupt)]

pub mod vga;
pub mod qemu;
pub mod serial;
pub mod interrupts;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub fn init() {
    interrupts::init_idt();
}
