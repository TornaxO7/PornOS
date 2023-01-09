use core::pin::Pin;

use {alloc::boxed::Box, futures::Future};

pub struct Task {
    pub future_fn: Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>,
}

impl Task {
    pub fn new(future_fn: impl Future<Output = ()> + 'static + Send + Sync) -> Self {
        Self {
            future_fn: Box::pin(future_fn),
        }
    }
}
