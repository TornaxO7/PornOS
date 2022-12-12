//! Credits to Phillop Oppermann (<https://os.phil-opp.com/testing/>)

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

use super::PornosWriter;

pub static PORNOS_WRITER: Mutex<Writer> = Mutex::new(Writer);

lazy_static! {
    static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe {SerialPort::new(0x3F8)};
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        SERIAL1.lock().write_str(s)
    }
}

impl PornosWriter for Writer {}
