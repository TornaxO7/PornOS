use x86_64::{structures::paging::{PhysFrame, Size4KiB}, PhysAddr};

use crate::memory::HHDM;

use super::{Stack, POINTER_SIZE};

impl IntoIterator for Stack {
    type Item = PhysFrame;

    type IntoIter = StackIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator::new(&self)
    }
}

pub struct StackIterator {
    start: PhysAddr,
    index: u64,
    capacity: u64,
}

impl StackIterator {
    pub fn new(stack: &Stack) -> Self {
        Self {
            start: stack.start,
            index: 0,
            capacity: stack.capacity,
        }
    }
}

impl Iterator for StackIterator {
    type Item = PhysFrame<Size4KiB>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.capacity {
            let entry_addr = *HHDM + self.start + self.index * (*POINTER_SIZE);
            let ptr = entry_addr.as_u64() as * mut u64;

            let page_frame = {
                let entry_value = PhysAddr::new(unsafe {ptr.read()});
                PhysFrame::from_start_address(entry_value)
            };

            self.index += 1;
            page_frame.ok()
        } else {
            None
        }
    }
}
