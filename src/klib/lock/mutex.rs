use core::cell::UnsafeCell;

use crate::scheduling::cooperative::kasync::AsyncRuntime;

#[derive(Default)]
pub struct Mutex<T> {
    runtime: AsyncRuntime,
    value: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            runtime: AsyncRuntime::new(),
            value: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) {
    }
}
