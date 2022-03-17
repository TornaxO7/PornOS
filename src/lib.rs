#![no_std]
#![no_main]
#![feature(const_ptr_offset)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub mod vga;
pub mod boot;
