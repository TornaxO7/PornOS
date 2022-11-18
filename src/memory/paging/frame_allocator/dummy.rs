use crate::memory::PhysAddr;

use super::FrameManager;

pub static mut DUMMY_FRAME_MANAGER: DummyFrameManager = DummyFrameManager;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DummyFrameManager;

impl FrameManager for DummyFrameManager {
    fn init(&mut self, _amount_page_frames: u64) {
    }

    fn get_free_frame(&mut self) -> PhysAddr {
        0
    }

    fn free_frame(&mut self, _addr: PhysAddr) {
    }
}
