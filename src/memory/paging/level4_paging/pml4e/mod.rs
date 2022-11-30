mod entry;
mod flags;
#[cfg(feature = "test")]
mod test;

#[cfg(feature = "test")]
pub use test::tests;

use x86_64::PhysAddr;
pub use self::flags::PML4EFlags;
pub use self::entry::PML4EEntry;

const AMOUNT_ENTRIES: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PML4E {
    start: PhysAddr,
    entries: [PML4EEntry; AMOUNT_ENTRIES],
}

impl PML4E {
    pub fn new(start: PhysAddr) -> Self {
        Self {
            start,
            entries: [PML4EEntry::default(); AMOUNT_ENTRIES],
        }
    }

    pub fn write(&self) {
    }
}
