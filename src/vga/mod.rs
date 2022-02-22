pub mod color;
pub mod color_code;
pub mod vga_char;

use vga_char::VGAChar;
use color_code::ColorCode;

use volatile::Volatile;

#[derive(Debug)]
pub struct VGA {
    row_index: usize,
    column_index: usize,
    buffer: &'static mut [[Volatile<VGAChar>; Self::BUFFER_WIDTH]; Self::BUFFER_HEIGHT],
}

impl VGA {
    const BUFFER_HEIGHT: usize = 25;
    const BUFFER_WIDTH: usize = 80;
    const VGA_BUFFER_ADDR: usize = 0xb8000;

    pub fn new() -> Self {
        Self {
            row_index: 0,
            column_index: 0,
            buffer: unsafe {&mut *(Self::VGA_BUFFER_ADDR as * mut _)},
        }
    }

    pub fn puts(&mut self, string: &str) {
        string.bytes()
            .for_each(|c| self.putc(char::from(c)));
    }

    pub fn putc(&mut self, c: char) {
        self.put_vga_char(VGAChar::new_char(c));
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
        }
    }

    pub fn putc_at(&mut self, char: VGAChar, row: usize, column: usize) {
        if row >= Self::BUFFER_HEIGHT {
            panic!("Row '{}' can't be accessed in tha VGA-Buffer!", row);
        } else if column >= Self::BUFFER_WIDTH {
            panic!("Column '{}' can't be accessed in tha VGA-Buffer!", row);
        }
        self.buffer[row][column].write(char);
    }

    pub fn clear_line(&mut self, row: usize) {
        let white_space_char = VGAChar::new_char(' ');
        for column_index in 0..Self::BUFFER_WIDTH {
            self.putc_at(white_space_char.clone(), row, column_index);
        }
    }

    fn move_lines_up(&mut self) {
        for row_index in 1..Self::BUFFER_HEIGHT {
            for column_index in 0..Self::BUFFER_WIDTH {
                self.buffer[row_index - 1][column_index] = self.buffer[row_index][column_index].clone()
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
    }

    fn reached_last_line(&self) -> bool {
        self.row_index >= Self::BUFFER_HEIGHT - 1
    }
}
