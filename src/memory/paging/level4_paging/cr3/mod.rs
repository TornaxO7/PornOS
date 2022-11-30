#[cfg(feature = "test")]
mod test;
#[cfg(feature = "test")]
pub use test::tests;
mod flags;

pub use flags::Cr3Flag;

use x86_64::PhysAddr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Cr3Value(u64);

impl Cr3Value {
    pub const fn new(flags: Cr3Flag) -> Self {
        Self(flags.bits())
    }

    pub const fn set_pwt(mut self) -> Self {
        self.0 |= 1 << 3;
        self
    }

    pub const fn set_pcd(mut self) -> Self {
        self.0 |= 1 << 4;
        self
    }

    pub const fn set_pml4e_phys_addr(mut self, addr: PhysAddr) -> Self {
        let value = {
            let value = addr.as_u64() & ((1 << 40) - 1);
            value << 12
        };
        self.0 |= value;
        self
    }
}

impl Cr3Value {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
