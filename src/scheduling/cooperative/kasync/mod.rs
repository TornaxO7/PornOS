//! This module contains the Async-Runtime of the kernel.
mod task;
mod waker;

use {alloc::sync::Arc, futures::Future};

use core::task::{Context, Poll};

use alloc::collections::{BTreeMap, BTreeSet};

use crate::klib::lock::spinlock::Spinlock;

use self::{
    task::{Task, TaskId},
    waker::TaskWaker,
};

/// The async runtime which executes the async functions.
#[derive(Default)]
pub struct AsyncRuntime {
    /// Holds all tasks which are currently in the runtime.
    tasks: Spinlock<BTreeMap<TaskId, Task>>,

    /// The ready queue which holds the id's of the tasks which can be run next.
    ready_queue: Arc<Spinlock<BTreeSet<TaskId>>>,
}

/// Holds some general implementations of the environment.
impl AsyncRuntime {
    /// Returns the amount of registered tasks.
    pub fn get_amount_tasks(&self) -> usize {
        self.tasks.lock().len()
    }
}

/// Holds the relevant functions to actually interact with the runtime.
impl AsyncRuntime {
    /// The maximal amount of tasks which the environment can take.
    pub const MAX_AMOUNT_PROCESSES: usize = 69;

    /// Creates a new async-runtime environment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an async function to the runtime.
    ///
    /// * `future_fn`: The async function which should be added to the runtime.
    ///
    /// # Returns
    /// - `true`: If the given future could be added to the runtime
    /// - `false`: If `future_fn` couldn't be added to the runtime, because
    /// the runtime is already full.
    #[must_use]
    pub fn add(&mut self, future_fn: impl Future<Output = ()> + 'static + Send + Sync) -> bool {
        if self.get_amount_tasks() >= Self::MAX_AMOUNT_PROCESSES {
            return false;
        }

        let id = TaskId::new();
        let task = Task::new(id, future_fn);
        assert!(self.tasks.lock().insert(id, task).is_none());
        assert!(self.ready_queue.lock().insert(id));

        true
    }

    /// Starts the async environment.
    ///
    /// # Returns
    /// Returns if tall tasks have been processed.
    pub fn run(&mut self) {
        while let Some(ref task_id) = { self.ready_queue.lock().pop_first() } {
            let mut tasks = self.tasks.lock();
            let task = tasks.get_mut(task_id).unwrap();

            let waker = TaskWaker::create(task.id, self.ready_queue.clone());
            let mut ctx = Context::from_waker(&waker);

            match task.future_fn.as_mut().poll(&mut ctx) {
                Poll::Pending => {}
                Poll::Ready(()) => {
                    tasks.remove(task_id);
                }
            };
        }
    }
}
