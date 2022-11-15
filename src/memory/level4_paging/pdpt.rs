pub const AMOUNT_ENTRIES: usize = 512;

bitflags::bitflags! {
    pub struct PDPTEntry: u64 {
        const P    = 1 << 0;
        const RW   = 1 << 1;
        const US   = 1 << 2;
        const PWT  = 1 << 3;
        const PCD  = 1 << 4;
        const A    = 1 << 5;
        const PS   = 1 << 7;
        const R    = 1 << 11;
        const ADDR = 0b111111111111111111111111111111111111111 << 12;
        const XD   = 1 << 63;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Pdpt {
    entries: [PDPTEntry; AMOUNT_ENTRIES],
}

impl Pdpt {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Pdpt {
    fn default() -> Self {
        Self {
            entries: [PDPTEntry::empty(); AMOUNT_ENTRIES],
        }
    }
}
