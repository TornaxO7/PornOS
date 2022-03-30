#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod qemu;
pub mod serial;

use core::panic::PanicInfo;
use qemu::{Qemu, QemuExitCode};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Not Stonks]\n");
    serial_println!("[Error:] {}\n", info);
    Qemu::exit_qemu(QemuExitCode::Failure);
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} pornos", tests.len());

    for test in tests {
        test.run();
    }

    Qemu::exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn() 
{
    fn run(&self) {
        serial_print!("{}...\t\t", core::any::type_name::<T>());
        self();
        serial_println!("[Stonks]");
    }
}
