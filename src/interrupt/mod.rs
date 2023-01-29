use core::arch::asm;

use x86_64::VirtAddr;

use crate::{print, println, memory::types::Bytes};

trait TaskGate {}

trait InterruptGate {}

trait TrapGate {}

/// Alignment recommended by the intel manual (Volume 3, Page 201)
#[repr(C, align(8))]
struct IDT {
}

struct IDTR {
    base: VirtAddr,
    limit: Bytes,
}

impl IDTR {
    pub const fn new(base: u32, limit: u16) -> Self {
        Self {
            base,
            limit: Bytes::from(limit),
        }
    }
}

pub fn init() {
    print!("IDT ... ");
    println!("OK");
}
