#![no_std]
#![no_main]

use core::arch::asm;

use limine::BaseRevision;
use pornos::{
    memory,
    qemu::{exit_qemu, QemuExitCode},
    serial_println,
};

/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
    assert!(BASE_REVISION.is_supported());

    serial_println!("Starting PornOS...");

    memory::frame_allocator::init();
    memory::mapper::init();
    memory::heap::init();

    exit_qemu(QemuExitCode::Success);
    halt_loop();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);

    halt_loop();
}

fn halt_loop() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
