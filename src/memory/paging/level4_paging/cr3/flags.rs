use bitflags::bitflags;

bitflags! {
    pub struct Cr3Flag: u64 {
        const PWT = 1 << 3;
        const PCD = 1 << 4;
    }
}
