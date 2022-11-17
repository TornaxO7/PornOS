mod memmap_entry;
pub mod memmaps;

use memmaps::Memmaps;

lazy_static::lazy_static! {
    static ref PHYS_START_MEMMAP: Memmaps = Memmaps::new();
}
