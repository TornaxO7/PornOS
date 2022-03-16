use volatile::Volatile;
use spin::Mutex;

pub struct TextModeStructure {
    buffer: Volatile<&'static mut [char]>,
    amount_rows: u16,
    amount_cols: u16,
    char_size: u16,

    col_index: u16,
    row_index: u16,
}

impl TextModeStructure {
}
