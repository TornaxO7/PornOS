use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll},
};

use futures::Future;

use alloc::collections::VecDeque;

use crate::klib::lock::spinlock::Spinlock;

#[derive(Default)]
pub struct Mutex<'a, T> {
    value: UnsafeCell<T>,
    is_locked: AtomicBool,
    sleeping_threads: Spinlock<VecDeque<&'a Mutex<'a, T>>>,
}

unsafe impl<'a, T> Send for Mutex<'a, T> {}
unsafe impl<'a, T> Sync for Mutex<'a, T> {}

impl<'a, T> Mutex<'a, T> {
    pub fn new(data: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            sleeping_threads: Spinlock::new(VecDeque::new()),
            value: UnsafeCell::new(data),
        }
    }

    pub fn lock(&'a self) -> FutureMutexLockGuard<T> {
        FutureMutexLockGuard { mutex: self }
    }
}

pub struct MutexLockGuard<'a, T> {
    mutex: &'a Mutex<'a, T>,
}

impl<'a, T> Deref for MutexLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<'a, T> DerefMut for MutexLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<'a, T> Drop for MutexLockGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.sleeping_threads.lock().pop_front();
    }
}

pub struct FutureMutexLockGuard<'a, T> {
    mutex: &'a Mutex<'a, T>,
}

impl<'a, T> Future for FutureMutexLockGuard<'a, T> {
    type Output = MutexLockGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self
            .mutex
            .is_locked
            .compare_exchange(false, true, Ordering::Release, Ordering::Acquire)
            .is_ok()
        {
            Poll::Ready(MutexLockGuard { mutex: self.mutex })
        } else {
            self.mutex.sleeping_threads.lock().push_back(self.mutex);
            Poll::Pending
        }
    }
}
