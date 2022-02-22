#![no_std]
#![no_main]

use pornos::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Pornos goes brrrrrrrrrrrr");
    loop {}
}
