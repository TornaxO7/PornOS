use core::pin::Pin;

use {
    alloc::{boxed::Box, sync::Arc},
    futures::{task::ArcWake, Future},
    spin::Mutex,
};

pub mod executer;
pub mod spawner;

mod raw {
}

pub struct Task {
    /// The future function which should be executed
    future_fn: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn lock(&self) -> TaskGuard {
    }
}

pub struct TaskGuard {

}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            future_fn: Box::pin(future),
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        todo!()
    }
}
