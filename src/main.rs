#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use pornos::println;
use limine::LimineBootInfoRequest;

static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);

/// Kernel Entry Point
///
/// `_start` is defined in the linker script as the entry point for the ELF file.
/// Unless the [`Entry Point`](limine::LimineEntryPointRequest) feature is requested,
/// the bootloader will transfer control to this function.
#[cfg(not(feature = "test"))]
#[no_mangle]
pub extern "C" fn pornos_entry() -> ! {
    if let Some(bootinfo) = BOOTLOADER_INFO.get_response().get() {
        println!(
            "booted by {} v{}",
            bootinfo.name.to_str().unwrap().to_str().unwrap(),
            bootinfo.version.to_str().unwrap().to_str().unwrap(),
        );
    }

    pornos::init();

    hlt_loop();
}

#[cfg(feature = "test")]
#[no_mangle]
pub extern "C" fn pornos_entry() -> ! {
    println!("=== TEST MODE ===");
    pornos::tests();

    println!("=== TESTS DONE ===");

    hlt_loop();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
