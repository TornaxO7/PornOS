#![no_std]
#![no_main]
#![feature(const_ptr_offset)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::porno_test)]

use core::panic::PanicInfo;

pub mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub fn porno_test(_test: &[&dyn Fn()]) {
    println!("Running tests...");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("Installing arch... ");
    assert_eq!(1, 1);
    println!("Ok");
}
