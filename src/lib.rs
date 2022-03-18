#![no_std]
#![no_main]
#![feature(const_ptr_offset)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

pub mod vga;
pub mod qemu;

use core::panic::PanicInfo;
use qemu::{Qemu, QemuExitCode};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    Qemu::exit_qemu(QemuExitCode::Success);
}

