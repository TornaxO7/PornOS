//! Holds the implementation of a task from the async-environment.
use core::{pin::Pin, sync::atomic::{AtomicU64, Ordering}};

use alloc::boxed::Box;
use futures::Future;

/// Represents a task from the async-environment.
pub struct Task {
    /// The ID of the task.
    pub id: TaskId,

    /// The future-object of the task.
    pub future_fn: Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>,
}

impl Task {
    /// Creates a new task.
    ///
    /// * `id`: The ID of the task.
    /// * `future_fn`: The future-object of the task.
    pub fn new(id: TaskId, future_fn: impl Future<Output = ()> + 'static + Send + Sync) -> Self {
        Self {
            id,
            future_fn: Box::pin(future_fn),
        }
    }
}

/// A new-type which represents unique task ids.
///
/// # Safety
/// Panics if all IDs have been used.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

impl TaskId {
    /// Creates a new task-id.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TaskId {
    fn default() -> Self {
        static ID_GEN: AtomicU64 = AtomicU64::new(0);

        let old_id = ID_GEN.fetch_add(0, Ordering::Relaxed);
        let new_id = ID_GEN.fetch_add(1, Ordering::Relaxed);

        if new_id < old_id {
            panic!("All TaskID's used! Old-ID: {}, New-ID: {}", old_id, new_id);
        }

        Self(new_id)
    }
}
