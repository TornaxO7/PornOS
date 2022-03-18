use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Qemu;

impl Qemu {
    const IO_QEMU_EXIT_PORT: u16 = 0xf4;

    pub fn exit_qemu(exit_code: QemuExitCode) {
        unsafe {
            let mut port = Port::new(Qemu::IO_QEMU_EXIT_PORT);
            port.write(exit_code as u32);
        }
    }
}
