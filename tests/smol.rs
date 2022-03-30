#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pornos::test_runner)]
#![reexport_test_harness_main = "test_main"]

/// Just some tests to look, if the tests can run

use core::panic::PanicInfo;
use pornos::test_panic_handler;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pornos::test_panic_handler(info)
}
