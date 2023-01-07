use crate::kasync::Task;

use super::AsyncExecutor;

/// A dummy executor which just basically does nothing.
pub struct DummyExecutor;

unsafe impl Sync for DummyExecutor {}

impl DummyExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl AsyncExecutor for DummyExecutor {
    fn run(&self) {}

    fn add_task(&mut self, task: Task) {}
}
