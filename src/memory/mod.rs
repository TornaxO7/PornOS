//! This module contains everything Memory related.
use {limine::LimineHhdmRequest, x86_64::VirtAddr};

pub mod paging;
pub mod types;

/// Setting up the memory stuff.
pub fn init() -> ! {
    paging::init();
}

#[cfg(feature = "test")]
pub fn tests() {
    paging::tests();
}
