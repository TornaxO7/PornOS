#![no_std]
#![no_main]

use pornos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("I use pornos now");
    loop {}
}
