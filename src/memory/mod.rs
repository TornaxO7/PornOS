//! This module contains everything Memory related.
use limine::LimineHhdmRequest;
use x86_64::VirtAddr;

use crate::{print, println};

pub mod paging;
pub mod types;

lazy_static::lazy_static! {
    /// This variable contains the starting virtual address of the higher half virtual memory.
    pub static ref HHDM: VirtAddr = VirtAddr::new(LimineHhdmRequest::new(0)
        .get_response()
        .get()
        .unwrap()
        .offset);
}

/// Setting up the memory stuff.
pub fn init() {
    print!("Memory ... ");

    paging::init();

    println!("OK");
}
