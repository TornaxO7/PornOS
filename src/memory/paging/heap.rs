use lazy_static::lazy_static;
use x86_64::VirtAddr;

use crate::memory::HHDM;

lazy_static! {
    pub static ref HEAP_START: VirtAddr = *HHDM;
}
