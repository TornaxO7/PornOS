pub const AMOUNT_ENTRIES: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PMLE4Entry(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PMLE4 {
    entries: [PMLE4Entry; AMOUNT_ENTRIES],
}
