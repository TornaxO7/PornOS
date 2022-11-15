
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PageEntry(u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Page(u64);
