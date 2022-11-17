#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn pml4(&self) -> u64 {
        (self.0 >> 39) & 0b1_1111_1111
    }

    pub fn pdpt(&self) -> u64 {
        (self.0 >> 30) & 0b1_1111_1111
    }

    pub fn page_directory(&self) -> u64 {
        (self.0 >> 21) & 0b1_1111_1111
    }

    pub fn page_table(&self) -> u64 {
        (self.0 >> 12) & 0b1_1111_1111
    }

    pub fn offset(&self) -> u64 {
        self.0 & 0b1111_1111_1111
    }
}
