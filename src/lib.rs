#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod qemu;
pub mod serial;

use core::panic::PanicInfo;
use qemu::{Qemu, QemuExitCode};

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    Qemu::exit_qemu(QemuExitCode::Success);
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    Qemu::exit_qemu(QemuExitCode::Failure);
    loop {}
}
