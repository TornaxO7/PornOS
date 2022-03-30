#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pornos::test_runner)]
#![reexport_test_harness_main = "test_main"]

/// Just some tests to look, if the tests can run

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}
