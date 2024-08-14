use core::marker::PhantomData;

use limine::{memory_map::EntryType, request::MemoryMapRequest};
use spin::{Mutex, MutexGuard, Once};
use x86_64::{
    structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

use crate::{serial_print, serial_println};

#[used]
#[link_section = ".requests"]
static MMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

static FAK: Once<Mutex<FrameAllocatorKing<Size4KiB>>> = Once::new();

pub fn init() {
    serial_print!("FAK... ");

    FAK.call_once(|| Mutex::new(FrameAllocatorKing::new()));

    serial_println!("OK");
}

pub fn get_fak<'a>() -> MutexGuard<'a, FrameAllocatorKing<Size4KiB>> {
    FAK.get().unwrap().lock()
}

/// **F**rame **A**llocator **K**ing
#[derive(Debug)]
pub struct FrameAllocatorKing<S: PageSize + 'static> {
    frames: &'static mut [PhysFrame<S>],
    length: usize,
    _phantom: PhantomData<S>,
}

impl<S: PageSize> FrameAllocatorKing<S> {
    pub fn new() -> Self {
        let hhdm = super::get_hhdm();
        let entries = MMAP_REQUEST.get_response().unwrap().entries();

        let total_amount_phys_frames: u64 = entries
            .iter()
            .filter(|entry| entry.entry_type == EntryType::USABLE)
            .map(|entry| entry.length / S::SIZE)
            .sum::<u64>();

        let meta_mem_size = total_amount_phys_frames * core::mem::size_of::<PhysFrame>() as u64;

        let fak_entry = {
            let mut fak_entry = entries
                .iter()
                .find(|entry| entry.entry_type == EntryType::USABLE && entry.length > meta_mem_size)
                .expect("Find big enough continuous physical memory for FAK");

            for entry in entries {
                if entry.length < meta_mem_size {
                    continue;
                }

                if entry.length < fak_entry.length {
                    fak_entry = entry;
                }
            }

            fak_entry
        };

        let mut fak = {
            let frames_ptr = VirtAddr::new(hhdm + fak_entry.base).as_mut_ptr();
            let frames: &'static mut [PhysFrame<S>] = unsafe {
                core::slice::from_raw_parts_mut(frames_ptr, total_amount_phys_frames as usize)
            };

            Self {
                frames,
                length: 0,
                _phantom: PhantomData,
            }
        };

        // fill FAK
        for entry in entries
            .iter()
            .filter(|entry| entry.entry_type == EntryType::USABLE)
        {
            let (base, length): (PhysFrame<S>, u64) = if entry.base != fak_entry.base {
                (
                    PhysFrame::from_start_address(PhysAddr::new(entry.base)).unwrap(),
                    entry.length,
                )
            } else {
                let start_addr =
                    PhysAddr::new(x86_64::align_up(fak_entry.base + meta_mem_size, S::SIZE));
                let length = entry.length - start_addr.as_u64();
                (PhysFrame::from_start_address(start_addr).unwrap(), length)
            };

            for offset in (0..length).step_by(S::SIZE as usize) {
                fak.push(base + offset);
            }
        }

        fak
    }

    pub fn push(&mut self, frame: PhysFrame<S>) {
        self.frames[self.length] = frame;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<PhysFrame<S>> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;
        let frame = self.frames[self.length];
        Some(frame)
    }
}

unsafe impl FrameAllocator<Size4KiB> for FrameAllocatorKing<Size4KiB> {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        self.pop()
    }
}

impl FrameDeallocator<Size4KiB> for FrameAllocatorKing<Size4KiB> {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.push(frame);
    }
}
