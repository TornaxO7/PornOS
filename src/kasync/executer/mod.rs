use alloc::boxed::Box;

use super::Task;

#[cfg(feature = "async-executor-dummy")]
pub mod dummy;

pub trait AsyncExecutor {
    fn add_task(&mut self, task: Task);

    fn run(&self);
}

pub fn init() -> Box<dyn AsyncExecutor> {
    #[cfg(feature = "async-executor-dummy")]
    Box::new(self::dummy::DummyExecutor::new())
}
