use limine::memory_map::Entry;
use x86_64::{
    structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

use crate::{serial_print, serial_println};

static mut FAK: Option<FrameManager> = None;

pub fn init() {
    serial_print!("FAK... ");

    unsafe { FAK = Some(FrameManager::new()) };

    serial_println!("OK");
}

/// **F**rame **A**llocator **K**ing
#[derive(Debug)]
pub struct FrameManager {
    ptr: *mut [PhysFrame],
    length: usize,
}

impl FrameManager {
    pub fn new() -> Self {
        let fak_entry = get_fak_entry();
        let all_entries = super::get_entries();
        let capacity = fak_entry.length as usize / core::mem::size_of::<PhysFrame>();
        let mut fak = {
            let base_ptr = VirtAddr::new(fak_entry.base).as_mut_ptr::<PhysFrame>();
            let ptr = core::ptr::slice_from_raw_parts_mut(base_ptr, capacity);

            Self { ptr, length: 0 }
        };

        for entry in all_entries {
            if fak.length >= capacity {
                break;
            }

            if entry.base != fak_entry.base {
                // split up into smaller chunks
                let amount = fak_entry.length / Size4KiB::SIZE;

                for i in 0..amount {
                    fak.push(
                        PhysFrame::from_start_address(PhysAddr::new(
                            entry.base + i * Size4KiB::SIZE,
                        ))
                        .unwrap(),
                    );
                }
            }
        }

        fak
    }

    pub fn push(&mut self, frame: PhysFrame) {
        unsafe {
            (*self.ptr)[self.length] = frame;
        };
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<PhysFrame> {
        if self.length == 0 {
            return None;
        }

        let frame = unsafe { (*self.ptr)[self.length] };
        self.length -= 1;
        Some(frame)
    }
}

unsafe impl FrameAllocator<Size4KiB> for FrameManager {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        self.pop()
    }
}

impl FrameDeallocator<Size4KiB> for FrameManager {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.push(frame);
    }
}

/// Returns the entry, where the frame allocator should be and all other entries (inclusive the entry of the frame allocator)
fn get_fak_entry() -> &'static Entry {
    super::get_entries()
        .iter()
        .min_by(|a, b| a.length.cmp(&b.length))
        .unwrap()
}
