pub const AMOUNT_ENTRIES: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PDPTEntry(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PDPT {
    entries: [PDPTEntry; AMOUNT_ENTRIES],
}
