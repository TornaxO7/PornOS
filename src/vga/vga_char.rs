use super::color_code::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    ascii_char: u8,
    color_code: ColorCode,
}

impl VGAChar {
    pub fn new(ascii_char: u8, color_code: ColorCode) -> Self {
        Self {
            ascii_char,
            color_code,
        }
    }

    pub fn get_char(&self) -> char {
        char::from(self.ascii_char)
    }
}
