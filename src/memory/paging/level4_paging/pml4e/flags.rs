use bitflags::bitflags;

bitflags! {
    pub struct PML4EFlags: u64 {
        const PRESENT = 1 << 0;
        const RW = 1 << 1;
        const US = 1 << 2;
        const PWT = 1 << 3;
        const PCD = 1 << 4;
        const A = 1 << 5;
        const R = 1 << 11;
        const XD = 1 << 63;
    }
}
