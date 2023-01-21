#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use pornos::{hlt_loop, println};

/// Kernel Entry Point
///
/// `_start` is defined in the linker script as the entry point for the ELF file.
/// Unless the [`Entry Point`](limine::LimineEntryPointRequest) feature is requested,
/// the bootloader will transfer control to this function.
#[cfg(not(feature = "test"))]
#[no_mangle]
pub extern "C" fn pornos_entry() -> ! {
    pornos::memory::init();
}

#[cfg(feature = "test")]
#[no_mangle]
pub extern "C" fn pornos_entry() -> ! {
    println!("=== TEST MODE ===");
    pornos::tests::main();

    println!("=== TESTS DONE ===");
    hlt_loop()
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("==== STEP BRO I'M STUCK ====");
    println!("{}", info);
    hlt_loop();
}
