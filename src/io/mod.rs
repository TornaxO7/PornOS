//! This modules contains all available Terminal-IO options.
mod serial;

use core::fmt::{self, Write};

use spin::Mutex;

static PORNOS_TERMINAL: Mutex<TerminalOutput> = Mutex::new(TerminalOutput::Serial);

/// The output type which you can choose.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TerminalOutput {
    /// Use port-I/O: https://crates.io/crates/uart_16550
    Serial,
}

impl fmt::Write for TerminalOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let serial = &mut *serial::PORNOS_WRITER.lock() as &mut dyn PornosWriter;

        let writer: &mut dyn PornosWriter = match self {
            Self::Serial => serial,
        };

        writer.write_str(s)
    }
}

pub trait PornosWriter: core::fmt::Write {}

pub fn _print(args: fmt::Arguments) {
    let mut writer_guard = PORNOS_TERMINAL.lock();
    writer_guard.write_fmt(args).ok();
}

pub fn set_output(output: TerminalOutput) {
    *PORNOS_TERMINAL.lock() = output;
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::io::_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)); };
}
