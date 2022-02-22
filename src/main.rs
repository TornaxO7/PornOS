#![no_std]
#![no_main]

use core::panic::PanicInfo;

use pornos::vga::VGA;
use pornos::vga::color::Color;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = VGA::new();
    vga.putc('a');

    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in b"bruh".iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    loop {}
}
