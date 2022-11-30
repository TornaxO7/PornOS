use x86_64::PhysAddr;

use super::flags::PML4EFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PML4EEntry(u64);

impl PML4EEntry {
    pub fn new(flags: PML4EFlags) -> Self {
        Self(flags.bits())
    }

    pub fn set_pdpt_phys_addr(mut self, addr: PhysAddr) -> Self {
        let value = {
            let value = addr.as_u64() & ((1 << 39) - 1);
            value << 12
        };

        self.0 |= value;
        self
    }
}

impl PML4EEntry {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
