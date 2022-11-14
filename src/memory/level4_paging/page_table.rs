pub const AMOUNT_ENTRIES: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PageTableEntry(u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; AMOUNT_ENTRIES],
}
