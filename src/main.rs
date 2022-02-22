#![no_std]
#![no_main]

use core::panic::PanicInfo;

use pornos::vga::VGA;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = VGA::new();
    vga.puts("I'm using pornOS every day.");
    loop {}
}
