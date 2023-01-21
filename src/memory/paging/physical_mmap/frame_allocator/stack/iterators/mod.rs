use x86_64::{structures::paging::PhysFrame, PhysAddr};

use crate::memory::paging::mem_structure::MEM_STRUCTURE;

use super::{Stack, POINTER_SIZE};

mod stack_page_frames;

pub struct StackIterator {
    start: PhysAddr,
    len: u64,
    index: u64,
}

impl Iterator for StackIterator {
    type Item = PhysFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let return_value = {
                let entry_addr =
                    MEM_STRUCTURE.hhdm + self.start.as_u64() + (POINTER_SIZE * self.index).as_u64();
                let entry_ptr = entry_addr.as_ptr() as *const u64;
                let entry_value = unsafe { entry_ptr.read() };

                PhysAddr::new(entry_value)
            };
            self.index += 1;
            return Some(PhysFrame::from_start_address(return_value).unwrap());
        }

        None
    }
}

impl IntoIterator for Stack {
    type Item = PhysFrame;

    type IntoIter = StackIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator {
            start: self.start,
            len: self.len,
            index: 0,
        }
    }
}
