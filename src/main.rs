#![no_std]
#![no_main]

use pornos::println;
use stivale_boot::v2::StivaleStruct;

#[no_mangle]
pub extern "C" fn pornos_entry(_stivale_struct: &'static StivaleStruct) -> ! {
    println!("Startig up PornOS...");

    loop {}
}
