use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll, Waker},
};

use futures::Future;

use alloc::collections::VecDeque;

use crate::{klib::lock::spinlock::Spinlock, println};

pub struct Mutex<T> {
    value: UnsafeCell<T>,
    is_locked: AtomicBool,
    sleeping_threads: Spinlock<VecDeque<Waker>>,
}

unsafe impl<'a, T> Send for Mutex<T> {}
unsafe impl<'a, T> Sync for Mutex<T> {}

impl<'a, T> Mutex<T> {
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

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        println!("RIP Mutex");
        while let Some(waker) = { self.sleeping_threads.lock().pop_front() } {
            waker.wake();
        }
    }
}

pub struct MutexLockGuard<'a, T> {
    mutex: &'a Mutex<T>,
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
        self.mutex.is_locked.store(false, Ordering::Release);
        if let Some(waker) = { self.mutex.sleeping_threads.lock().pop_front() } {
            waker.wake();
        }
    }
}

pub struct FutureMutexLockGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> Future for FutureMutexLockGuard<'a, T> {
    type Output = MutexLockGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self
            .mutex
            .is_locked
            .compare_exchange(false, true, Ordering::Release, Ordering::Acquire)
            .is_ok()
        {
            Poll::Ready(MutexLockGuard { mutex: self.mutex })
        } else {
            let waker = cx.waker().clone();
            self.mutex.sleeping_threads.lock().push_back(waker);
            Poll::Pending
        }
    }
}

#[cfg(feature = "test")]
pub mod tests {
    use crate::{scheduling::cooperative::kasync::{AsyncRuntime, AsyncRuntimeExitErrStatus}, print, println};

    use super::Mutex;

    pub fn main() {
        test_mutex_success();
        test_mutex_deadlock();
    }

    fn test_mutex_success() {
        print!("test_mutex_success ... ");

        let mut runtime = AsyncRuntime::new();
        assert!(runtime.add(test_common_lock_usage()));
        assert!(runtime.run().is_ok());

        println!("OK");
    }

    fn test_mutex_deadlock() {
        print!("test_mutex_deadlock ... ");

        let mut runtime = AsyncRuntime::new();
        assert!(runtime.add(deadlock_fn()));
        assert_eq!(
            runtime.run(),
            Err(AsyncRuntimeExitErrStatus::UnfinishedTasks)
        );

        println!("OK");
    }

    async fn test_common_lock_usage() {
        let mutex = Mutex::new(69);
        let yes = mutex.lock();
        let no = mutex.lock();
        {
            let mut guard = yes.await;
            *guard = 42;
        }
        {
            let guard = no.await;
            assert_eq!(*guard, 42);
        }
    }

    async fn deadlock_fn() {
        let mutex = Mutex::new(69);
        let _lock1 = mutex.lock().await;
        let lock2 = mutex.lock();

        lock2.await;
    }
}
