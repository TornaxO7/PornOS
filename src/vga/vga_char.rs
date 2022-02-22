use super::color_code::ColorCode;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    pub ascii_char: char,
    pub color_code: ColorCode,
}

impl VGAChar {
    pub fn new_char(ascii_char: char) -> Self {
        Self::new(ascii_char, ColorCode::default())
    }

    pub fn new(ascii_char: char, color_code: ColorCode) -> Self {
        Self {
            ascii_char,
            color_code,
        }
    }

    pub fn get_char(&self) -> char {
        self.ascii_char
    }
}
