use crate::memory::{
    paging::{PageSize, PhysLinearAddr, PhysMemMap},
    types::Bytes,
    VirtAddr,
};

use super::FrameIndex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameStack {
    /// The address where the stack starts
    start: VirtAddr,
    /// The length of the stack
    len: Bytes,
    /// The capacity of the stack
    capacity: Bytes,
}

impl Default for FrameStack {
    fn default() -> Self {
        Self {
            start: VirtAddr::zero(),
            len: Bytes::new(0),
            capacity: Bytes::new(0),
        }
    }
}

impl FrameStack {
    pub fn new(start: PhysLinearAddr, phys_mmap: &PhysMemMap, page_size: PageSize) -> Self {
        let start = start.align_up(FrameIndex::SIZE.as_u64());
        let amount_page_frames = phys_mmap.get_amount_page_frames(page_size);
        let mut frame_index = FrameIndex(amount_page_frames);

        // init the stack entries. Starting from frame index <amount_page_frames> up to 0
        for offset_multiplier in 0..amount_page_frames {
            let offset = FrameIndex::SIZE * offset_multiplier;
            if !phys_mmap.write_value(frame_index, start + offset.as_u64()) {
                panic!("Not enough useable RAM for frames :(");
            }
            frame_index.0 -= 1;
        }

        let capacity = page_size.size() * amount_page_frames;

        Self {
            start: phys_mmap.convert_to_virt(&start).unwrap(),
            len: capacity,
            capacity,
        }
    }

    pub fn pop(&mut self) -> FrameIndex {
        todo!()
    }

    pub fn push(&mut self, frame_index: FrameIndex) {
        todo!()
    }

    pub fn get_capacity(&self) -> Bytes {
        self.capacity
    }
}
