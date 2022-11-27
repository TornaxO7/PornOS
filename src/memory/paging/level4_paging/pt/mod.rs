#[cfg(feature = "test")]
mod test;
#[cfg(feature = "test")]
pub use test::tests;
use x86_64::PhysAddr;

use self::flags::PageTableFlags;

mod flags;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PageTable(u64);

impl PageTable {
    pub fn new(flags: PageTableFlags) -> Self {
        Self(flags.bits())
    }

    pub fn set_protection_key(mut self, key: u8) -> Self {
        let value = {
            let value = u64::from(key & ((1 << 4) - 1));
            value << 59
        };

        self.0 |= value;
        self
    }

    pub fn set_page_phys_addr(mut self, addr: PhysAddr) -> Self {
        let value = {
            let value = addr.as_u64() & ((1 << 39) - 1);
            value << 12
        };
        self.0 |= value;
        self
    }
}

impl PageTable {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
