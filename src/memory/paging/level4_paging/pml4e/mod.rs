use x86_64::PhysAddr;

use self::flags::PML4EFlags;

#[cfg(feature = "test")]
mod test;
#[cfg(feature = "test")]
pub use test::tests;

mod flags;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PML4E(u64);

impl PML4E {
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

impl PML4E {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
