#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    // 0x0
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,

    // 0x8
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}
