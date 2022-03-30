#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pornos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod stivale;

use pornos;
use pornos::println;

#[no_mangle]
pub extern "C" fn pornos_entry() -> ! {
    println!("Starting PornOS...");

    #[cfg(test)]
    test_main();

    println!("Stopping OS by entering an infinite loop...");
    loop {}
}
