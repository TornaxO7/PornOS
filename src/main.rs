#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pornos::test_starter)]
#![reexport_test_harness_main = "test_main"]

use pornos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Startig up PornOS...");

    #[cfg(test)]
    test_main();

    loop {}
}
