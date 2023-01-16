//! Holds the waker of a task.
use core::task::Waker;

use {
    alloc::{collections::BTreeSet, sync::Arc, task::Wake},
};

use crate::klib::lock::spinlock::Spinlock;

use super::task::TaskId;

/// The struct which wakes up a task.
pub struct TaskWaker {
    task_id: TaskId,
    ready_queue: Arc<Spinlock<BTreeSet<TaskId>>>,
}

impl TaskWaker {
    /// Creates a new waker for a given task.
    ///
    /// * `task_id`: The ID of the task which should be waked up.
    /// * `ready_queue`: The ready queue where the task should be added if it
    /// should "wake up".
    pub fn create(task_id: TaskId, ready_queue: Arc<Spinlock<BTreeSet<TaskId>>>) -> Waker {
        Waker::from(Arc::new(Self { task_id, ready_queue }))
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        assert!(self.ready_queue.lock().insert(self.task_id));
    }
}
