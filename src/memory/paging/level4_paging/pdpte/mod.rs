#[cfg(feature = "test")]
mod test;
#[cfg(feature = "test")]
pub use test::tests;
use x86_64::PhysAddr;

use self::flags::PDPTEFlags;

mod flags;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PDPTE(u64);

impl PDPTE {
    pub fn new(flags: PDPTEFlags) -> Self {
        Self(flags.bits())
    }

    pub fn set_pd_phys_addr(mut self, addr: PhysAddr) -> Self {
        let value = {
            let value = addr.as_u64() & ((1 << 39) - 1);
            value << 12
        };
        self.0 |= value;
        self
    }
}

impl PDPTE {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
