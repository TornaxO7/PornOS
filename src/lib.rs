#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
#![feature(strict_provenance)]
#![feature(alloc_error_handler)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use alloc::{boxed::Box, string::String};
use x86_64::{
    structures::paging::{FrameAllocator, Page, PageTableFlags},
    VirtAddr,
};

use crate::memory::paging::{VMMapperMap, FRAME_ALLOCATOR, SIMP};

extern crate alloc;

pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod memory;
pub mod util;

pub fn init() -> ! {
    gdt::init();
    interrupt::init();
    memory::paging::init_heap();

    {
        let page_frame = FRAME_ALLOCATOR.write().allocate_frame().unwrap();
        let page = Page::from_start_address(VirtAddr::new(0x2000)).unwrap();
        unsafe {
            SIMP.lock().map_page(
                page,
                Some(page_frame),
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            );
        }
    }

    // let _test = Box::new([0u8; 5000]);

    println!("Entering infinity-loop...");
    hlt_loop();
}

#[cfg(feature = "test")]
pub fn tests() {
    memory::tests();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
