//! Credits goes to Markus Pfeifenberger and Raphael O. and their xernel code.

use core::fmt;
use core::fmt::Write;

use x86_64::instructions::port::Port;

struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut port = Port::new(0xe9);

        for c in s.chars() {
            unsafe {
                port.write(c as u8);
            }
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut writer = Writer;
    // UNWRAP: We always return `Ok(())` inside `write_str` so this is unreachable.
    writer.write_fmt(args).unwrap();
    writer.write_char('\n').unwrap();
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ($crate::io::logger::_print(format_args!($($arg)*)));
}
