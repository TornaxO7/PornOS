mod task;
mod waker;

use {alloc::sync::Arc, crossbeam::queue::ArrayQueue, futures::Future, spin::Mutex};

use core::task::{Poll, Context, Waker};

use self::{task::Task, waker::TaskWaker};

pub struct AsyncRuntime {
    ready_queue: Arc<Mutex<ArrayQueue<Arc<Mutex<Task>>>>>,
    waiting_queue: Arc<Mutex<ArrayQueue<Arc<Mutex<Task>>>>>,
}

impl AsyncRuntime {
    pub const FULL_READY: &str = "Ready-Queue is full.";
    pub const FULL_WAITING: &str = "Waiting-Queue is full.";

    /// The maximal amount of tasks which can run in parallel.
    pub const MAX_AMOUNT_PROCESSES: usize = 69;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, future_fn: impl Future<Output = ()> + 'static + Send + Sync) {
        let task = Task::new(future_fn);
        self.ready_queue
            .lock()
            .push(Arc::new(Mutex::new(task)))
            .unwrap_or_else(|_| panic!("{}", Self::FULL_READY));
    }

    pub fn run(&mut self) -> ! {
        loop {
            while let Some(task) = { self.ready_queue.lock().pop() } {
                let waker = Waker::from(TaskWaker::new(task.clone(), self.ready_queue.clone()));
                let mut ctx = Context::from_waker(&waker);

                match task.clone().lock().future_fn.as_mut().poll(&mut ctx) {
                    Poll::Pending => self
                        .waiting_queue
                        .lock()
                        .push(task.clone())
                        .unwrap_or_else(|_| panic!("{}", Self::FULL_READY)),
                    Poll::Ready(()) => {}
                };
            }
        }
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self {
            ready_queue: Arc::new(Mutex::new(ArrayQueue::new(Self::MAX_AMOUNT_PROCESSES))),
            waiting_queue: Arc::new(Mutex::new(ArrayQueue::new(Self::MAX_AMOUNT_PROCESSES))),
        }
    }
}
