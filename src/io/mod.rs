mod port;
mod limine_terminal;

use core::fmt::{self, Write};

use spin::Mutex;

static PORNOS_TERMINAL: Mutex<TerminalOutput> = Mutex::new(TerminalOutput::LimineTerminal);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TerminalOutput {
    LimineTerminal,
}

impl fmt::Write for TerminalOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut writer = match self {
            Self::LimineTerminal => limine_terminal::WRITER.lock(),
        };

        writer.write_str(s)
    }
}

pub fn _print(args: fmt::Arguments) {
    let mut writer_guard = PORNOS_TERMINAL.lock();
    writer_guard.write_fmt(args).ok();
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
