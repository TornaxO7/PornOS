use core::ops::Range;

use crate::memory::paging::frame_allocator::stack::POINTER_SIZE;
use crate::memory::paging::physical_mmap::{self, UseableMemChunkIterator};
use crate::memory::HHDM;
use crate::print;
use crate::println;
use x86_64::structures::paging::{PageSize, Size4KiB};
use x86_64::PhysAddr;

use super::{Stack, StackIndex};

impl Stack {
    /// Creates a new frame-stack with the given arguments.
    pub fn new() -> Self {
        print!("Using Frame-Allocator-Stack ... ");
        let amount_page_frames = physical_mmap::get_amount_useable_page_frames::<Size4KiB>();
        let stack_start = get_stack_page_frame();
        let capacity = amount_page_frames;

        let mut stack = Self {
            start: stack_start,
            len: capacity,
            capacity,
            amount_used_page_frames: 0,
        };

        stack.add_useable_page_frames();
        stack.swap_stack_frames();

        println!("OK");
        stack
    }

    /// Fills the stack with pointers to the page frames.
    /// WORKS:
    fn add_useable_page_frames(&mut self) {
        let mut entry_virt_addr = *HHDM + self.start.as_u64();

        for mmap in UseableMemChunkIterator::new() {
            for frame_offset in (0..mmap.len).step_by(Self::PAGE_SIZE.as_usize()) {
                let frame_addr = PhysAddr::new(mmap.base + frame_offset);

                {
                    let entry_virt_addr_ptr = entry_virt_addr.as_mut_ptr() as *mut u64;
                    unsafe {
                        entry_virt_addr_ptr.write(frame_addr.as_u64());
                    }
                }
                entry_virt_addr += *POINTER_SIZE;
            }
        }
    }

    /// Moves the frames which the stack uses to the top of the stack.
    /// Then the stack reduces it's capacity to the first real free frame.
    ///
    /// This makes it possible to get the physical addresses of the stack-frames without the
    /// conflict of popping or pushing.
    fn swap_stack_frames(&mut self) {
        let stack_range = self.get_stack_range().unwrap();
        self.amount_used_page_frames = stack_range.end - stack_range.start;

        if stack_range.end < self.len {
            let mut stack_entry_virt_addr = {
                let entry_phys_addr = self.start + (*POINTER_SIZE) * (stack_range.end - 1);
                *HHDM + entry_phys_addr.as_u64()
            };
            let mut entry_switch_virt_addr = {
                let entry_phys_addr = self.start + (*POINTER_SIZE) * (self.len - 1);
                *HHDM + entry_phys_addr.as_u64()
            };

            for _ in 0..self.amount_used_page_frames {
                let stack_entry_ptr = stack_entry_virt_addr.as_mut_ptr() as *mut u64;
                let entry_switch_ptr = entry_switch_virt_addr.as_mut_ptr() as *mut u64;
                unsafe {
                    core::ptr::swap(stack_entry_ptr, entry_switch_ptr);
                }

                stack_entry_virt_addr -= *POINTER_SIZE;
                entry_switch_virt_addr -= *POINTER_SIZE;
            }
        }

        self.len -= self.amount_used_page_frames;
        self.capacity = self.len;
    }

    /// Returns a range where:
    ///     - `start` is the starting index inside the stack which points to the page-frame where
    ///     the stack resides
    ///     - `end` (exclusive) is the ending index inside the stack which points to the page-frame
    ///     which is *behind* the last page-frame where the stack resides.
    fn get_stack_range(&self) -> Option<Range<StackIndex>> {
        if let Some(start) = self.get_stack_start_index() {
            if let Some(end) = self.get_stack_end_index() {
                return Some(Range { start, end });
            }
        }
        None
    }

    /// Returns the stack index which holds the frame where the stack starts.
    ///
    /// # Return
    /// - `Some<StackIndex>`: If the given frame could be found.
    /// - `None`: If the frame isn't in the stack anymore.
    fn get_stack_start_index(&self) -> Option<StackIndex> {
        for stack_index in 0..self.len {
            let page_frame = self.get_entry_value(stack_index).unwrap();
            if page_frame == self.start {
                return Some(stack_index);
            }
        }
        None
    }

    /// Return the stack index of the last page-frame which includes the data of the stack.
    fn get_stack_end_index(&self) -> Option<StackIndex> {
        let amount_used_page_frames = (self.capacity * (*POINTER_SIZE)).div_ceil(Size4KiB::SIZE);
        self.get_stack_start_index()
            .map(|start_index| start_index + amount_used_page_frames)
    }
}

// Returns the physical address where the stack can start to store itself. It's guaranteed that the
// given start-address has enough space for the stack.
//
// FUTURE: It could happen, that we'll get the last frame because the other frames might
// be too small....
fn get_stack_page_frame() -> PhysAddr {
    let amount_page_frames = physical_mmap::get_amount_useable_page_frames::<Size4KiB>();
    let needed_free_space = POINTER_SIZE * amount_page_frames;

    for mmap in UseableMemChunkIterator::new() {
        let has_enough_space = mmap.len >= needed_free_space.as_u64();
        if has_enough_space {
            return PhysAddr::new(mmap.base);
        }
    }

    unreachable!("Bro, download some RAM: http://downloadramdownloadramdownloadram.com");
}
