use core::{pin::Pin, sync::atomic::{AtomicU64, Ordering}};

use alloc::boxed::Box;
use futures::Future;

pub struct Task {
    pub id: TaskId,
    pub future_fn: Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>,
}

impl Task {
    pub fn new(id: TaskId, future_fn: impl Future<Output = ()> + 'static + Send + Sync) -> Self {
        Self {
            id,
            future_fn: Box::pin(future_fn),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TaskId {
    fn default() -> Self {
        static ID_GEN: AtomicU64 = AtomicU64::new(0);
        Self(ID_GEN.fetch_add(1, Ordering::Relaxed))
    }
}
