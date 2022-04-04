#![no_std]
#![no_main]

mod stivale;

use pornos;
use pornos::println;

#[no_mangle]
pub extern "C" fn pornos_entry() -> ! {
    println!("Starting PornOS...");

    println!("Stopping OS by entering an infinite loop...");
    loop {}
}
