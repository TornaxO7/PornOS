#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::utils::_vga_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _vga_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    crate::vga::VGA_WRITER.lock().write_fmt(args).unwrap();
}
