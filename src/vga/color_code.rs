use super::color::Color;

#[derive(Debug, Clone, Copy,PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
    pub fn new(fg: Color, bg: Color, blink: bool) -> Self {
        Self((fg as u8) | (bg as u8) >> 4 | (blink as u8) >> 7)
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::White, Color::Black, false)
    }
}
