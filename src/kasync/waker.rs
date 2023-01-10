use core::task::Waker;

use {
    alloc::{collections::BTreeSet, sync::Arc, task::Wake},
    spin::Mutex,
};

use super::task::TaskId;

pub struct TaskWaker {
    task_id: TaskId,
    ready_queue: Arc<Mutex<BTreeSet<TaskId>>>,
}

impl TaskWaker {
    pub fn new(task_id: TaskId, ready_queue: Arc<Mutex<BTreeSet<TaskId>>>) -> Waker {
        Waker::from(Arc::new(Self { task_id, ready_queue }))
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.ready_queue.lock().insert(self.task_id);
    }
}
