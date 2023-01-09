use core::task::Waker;

use alloc::{task::Wake, sync::Arc};
use crossbeam::queue::ArrayQueue;
use spin::Mutex;

use super::{task::Task, AsyncRuntime};

pub struct TaskWaker {
    task: Arc<Mutex<Task>>,
    ready_queue: Arc<Mutex<ArrayQueue<Arc<Mutex<Task>>>>>,
}

impl TaskWaker {
    pub fn new(task: Arc<Mutex<Task>>, ready_queue: Arc<Mutex<ArrayQueue<Arc<Mutex<Task>>>>>) -> Waker {
        Waker::from(Arc::new(Self {
            task,
            ready_queue,
        }))
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.ready_queue.lock().push(self.task.clone()).unwrap_or_else(|_| panic!("{}", AsyncRuntime::FULL_READY));
    }
}
