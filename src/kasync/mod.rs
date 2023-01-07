use futures::{future::BoxFuture, Future};
use spin::Mutex;

pub mod executer;
pub mod spawner;

pub struct Task {
    future_fn: Mutex<Option<BoxFuture<'static, ()>>>,
}

impl Task {
    pub fn new<T>(future: impl Future<Output = T> + 'static) -> Self {
        todo!()
    }
}
