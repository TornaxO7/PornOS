#![no_std]
#![no_main]

mod stivale;

use pornos;
use pornos::println;

#[no_mangle]
pub extern "C" fn pornos_entry() -> ! {
    println!("Starting PornOS...");

    pornos::init();

    // x86_64::instructions::interrupts::int3();

    println!("Stopping OS by entering an infinite loop...");
    loop {}
}
