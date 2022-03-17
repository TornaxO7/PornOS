#![no_std]
#![no_main]
#![feature(const_ptr_offset)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub fn test_runner(_test: &[&i32]) {
    println!("Running tests...");
    loop {}
}
