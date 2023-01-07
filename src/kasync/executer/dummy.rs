use alloc::collections::VecDeque;
use spin::{Mutex, Once};

use crate::kasync::Task;

use super::AsyncExecutor;

pub static DUMMY_EXECUTOR: Once<DummyExecutor> = Once::new();

pub struct DummyExecutor {
    ready_queue: Mutex<VecDeque<Task>>
}

unsafe impl Sync for DummyExecutor {}

impl DummyExecutor {
    pub fn new() -> Self {
        Self {
            ready_queue: Mutex::new(VecDeque::new())
        }
    }
}

impl AsyncExecutor for DummyExecutor {
    fn run(&self) {
        unreachable!("Bro, that's a dummy!");
    }

    fn add_task(&mut self, task: Task) {}
}
