use crate::memory::paging::frame_allocator::stack::POINTER_SIZE;
use crate::println;
use crate::{
    memory::paging::{PageSize, PhysMemMap},
    print,
};
use x86_64::PhysAddr;

use super::Stack;

type StackIndex = u64;

impl Stack {
    /// Creates a new frame-stack with the given arguments.
    pub fn new(phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        print!("Using Frame-Allocator-Stack ... ");
        let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);
        let stack_start = get_start_addr(phys_mmap, page_size);
        let capacity = amount_page_frames;

        let mut stack = Self {
            start: stack_start,
            len: capacity,
            capacity,
            page_size,
        };

        stack.add_entries(phys_mmap);
        stack.swap_stack_frames(phys_mmap);

        println!("OK");
        stack
    }

    /// Fills the stack with pointers to the useable memory chunks.
    fn add_entries(&self, phys_mmap: &PhysMemMap) {
        let amount_page_frames = phys_mmap.get_amount_page_frames(self.page_size);
        for entry_index in 0..amount_page_frames {
            let entry_start_addr: *mut u64 =
                (self.start.as_u64() + *POINTER_SIZE * entry_index) as *mut u64;

            if let Some(entry_value) = self.get_entry_value(phys_mmap, entry_index) {
                unsafe {
                    *entry_start_addr = entry_value.as_u64();
                }
            } else {
                break;
            }
        }
    }

    /// # Return
    /// - `Some<PhysAddr>` if a free frame could be found for the given entry index.
    /// - `None`: If there are no free frames anymore.
    fn get_entry_value(&self, phys_mmap: &PhysMemMap, entry_index: u64) -> Option<PhysAddr> {
        let start_physical_address = {
            let start_physical_address = self.page_size.size().as_u64() * entry_index;
            PhysAddr::new(start_physical_address)
        };

        phys_mmap.get_frame(
            start_physical_address,
            self.page_size,
            self.page_size.size(),
        )
    }

    /// Moves the frames which the stack uses to the top of the stack.
    /// Then the stack reduces it's capacity to the first real free frame.
    ///
    /// This makes it possible to get the physical addresses of the stack-frames without the
    /// conflict of popping or pushing.
    fn swap_stack_frames(&mut self, phys_mmap: &PhysMemMap) {
        if let Some(stack_frame_index) = self.get_stack_frame_index() {
            let used_frames = self.get_used_frames();
            for index in stack_frame_index..stack_frame_index + used_frames {
                let used_frame_addr: * mut u64 = {
                    let addr = self.start.as_u64() + (POINTER_SIZE * index).as_u64();
                    addr as * mut u64
                };

                let free_frame_addr: * mut u64 = {
                    let addr = self.start.as_u64() + (POINTER_SIZE * (index + used_frames)).as_u64();
                    addr as * mut u64
                };

                unsafe {
                    core::ptr::swap(used_frame_addr, free_frame_addr);
                }
            }

            self.capacity = phys_mmap.get_amount_page_frames(self.page_size) - used_frames;
            self.len = self.capacity;
        }
    }

    /// Returns the stack index which holds the frame where the stack starts.
    ///
    /// # Return
    /// - `Some<StackIndex>`: If the given frame could be found.
    /// - `None`: If the frame isn't in the stack anymore.
    fn get_stack_frame_index(&self) -> Option<StackIndex> {
        for stack_index in 0..self.len {
            let frame_addr = self.get_entry(stack_index).unwrap();
            if frame_addr == self.start.as_u64() {
                return Some(frame_addr);
            }
        }
        None
    }

    /// # Return
    /// The amount of frames which the stack uses.
    fn get_used_frames(&self) -> u64 {
        self.capacity.div_ceil(self.page_size.size().as_u64())
    }
}

fn get_start_addr(phys_mmap: &PhysMemMap, page_size: PageSize) -> PhysAddr {
    let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);
    let needed_free_space = POINTER_SIZE * amount_page_frames;

    // FUTURE: It could happen, that we'll get the last frame because the other frames might
    // be too small....
    phys_mmap
        .get_frame(PhysAddr::zero(), page_size, needed_free_space)
        .unwrap()
}
