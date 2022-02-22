pub mod color;
pub mod color_code;
pub mod vga_char;
pub mod utils;

use vga_char::VGAChar;
use color_code::ColorCode;
use volatile::Volatile;
use core::fmt;
use spin::Mutex;

lazy_static::lazy_static! {
    pub static ref VGA_WRITER: Mutex<VGA> = Mutex::new(VGA {
        row_index: 0,
        column_index: 0,
        color: ColorCode::default(),
        buffer: Volatile::new(unsafe{core::slice::from_raw_parts_mut(VGA::VGA_BUFFER_ADDR as * mut _, VGA::BUFFER_WIDTH * VGA::BUFFER_HEIGHT)})
    });
}

pub struct VGA {
    row_index: usize,
    column_index: usize,
    color: ColorCode,
    buffer: Volatile<&'static mut [VGAChar]>,
}

impl VGA {
    const BUFFER_HEIGHT: usize = 25;
    const BUFFER_WIDTH: usize = 80;
    const VGA_BUFFER_ADDR: usize = 0xb8000;

    pub fn new() -> Self {
        Self {
            row_index: 0,
            column_index: 0,
            color: ColorCode::default(),
            buffer: Volatile::new(unsafe{core::slice::from_raw_parts_mut(Self::VGA_BUFFER_ADDR as * mut _, Self::BUFFER_WIDTH * Self::BUFFER_HEIGHT)}),
        }
    }

    pub fn puts(&mut self, string: &str) {
        string.bytes()
            .for_each(|c| self.putc(c));
    }

    pub fn putc(&mut self, c: u8) {
        self.put_vga_char(VGAChar::new(c, self.color));
    }

    fn put_vga_char(&mut self, char: VGAChar) {
        if char.get_char() == '\n' {
            self.next_line();
        } else {
            if self.column_index >= Self::BUFFER_WIDTH {
                self.column_index = 0;
                self.next_line();
            }

            self.putc_at(char, self.row_index, self.column_index);
            self.column_index += 1;
        }
    }

    pub fn putc_at(&mut self, char: VGAChar, row: usize, column: usize) {
        if row >= Self::BUFFER_HEIGHT {
            panic!("Row '{}' can't be accessed in tha VGA-Buffer!", row);
        } else if column >= Self::BUFFER_WIDTH {
            panic!("Column '{}' can't be accessed in tha VGA-Buffer!", row);
        }

        self.buffer
            .index_mut(row * Self::BUFFER_WIDTH + column)
            .write(char.clone());
    }

    pub fn clear_line(&mut self, row: usize) {
        let white_space_char = VGAChar::new(b' ', self.color);
        for column_index in 0..Self::BUFFER_WIDTH {
            self.putc_at(white_space_char.clone(), row, column_index);
        }
    }

    fn move_lines_up(&mut self) {
        for row_index in 1..Self::BUFFER_HEIGHT {
            for column_index in 0..Self::BUFFER_WIDTH {
                let new_char = self.buffer
                    .index(row_index * Self::BUFFER_WIDTH + column_index)
                    .read();

                self.putc_at(new_char, row_index - 1, column_index);
            }
        }
    }

    fn next_line(&mut self) {
        if self.reached_last_line() {
            self.move_lines_up();
            self.clear_line(Self::BUFFER_HEIGHT - 1);
        } else {
            self.row_index += 1;
        }
        self.column_index = 0;
    }

    fn reached_last_line(&self) -> bool {
        self.row_index >= Self::BUFFER_HEIGHT - 1
    }
}

impl fmt::Write for VGA {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.puts(string);
        Ok(())
    }
}
