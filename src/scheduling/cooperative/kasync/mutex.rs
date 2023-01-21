//! This module holds the implementation of the [mutex] in PornOS with
//! cooperative scheduling.
//!
//! [mutex]: https://en.wikipedia.org/wiki/Mutual_exclusion
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll, Waker},
};

use futures::Future;

use alloc::collections::VecDeque;

use crate::klib::lock::spinlock::Spinlock;

/// The `Mutex` struct which can be used in a cooperative environment.
///
/// # Example
/// ```rust
/// # use pornos::scheduling::cooperative::kasync::mutex::Mutex;
/// # use pornos::scheduling::cooperative::kasync::AsyncRuntime;
/// # use pornos::println;
///
/// async fn interesting() {
///     // *This* mutex struct can be only used in a cooperative environment.
///     // In rust, that will be the async-environment.
///     let mutex = Mutex::new(48);
///     let guard = mutex.lock().await;
///
///     println!("My num: {}", *guard);
///
///     // guard will be dropped afterwards
/// }
///
/// fn main() {
///     let mut runtime = Runtime::new();
///     assert!(runtime.add(interesting()));
///     runtime.run();
/// }
///
/// ```
pub struct Mutex<T> {
    value: UnsafeCell<T>,
    is_locked: AtomicBool,
    sleeping_threads: Spinlock<VecDeque<Waker>>,
}

unsafe impl<'a, T> Send for Mutex<T> {}
unsafe impl<'a, T> Sync for Mutex<T> {}

impl<'a, T> Mutex<T> {
    /// Creates a new mutex with the given data.
    pub fn new(data: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            sleeping_threads: Spinlock::new(VecDeque::new()),
            value: UnsafeCell::new(data),
        }
    }

    /// Returns a future-lock-guard which can be aquired, if you're await it.
    pub fn lock(&'a self) -> FutureMutexLockGuard<T> {
        FutureMutexLockGuard { mutex: self }
    }
}

/// The mutex lock guard.
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

/// When the lock-guard is dropped, it'll wake up the next thread (if it exists)
/// which requested the data from the lock.
impl<'a, T> Drop for MutexLockGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.is_locked.store(false, Ordering::Release);
        if let Some(waker) = { self.mutex.sleeping_threads.lock().pop_front() } {
            waker.wake();
        }
    }
}

/// A helper struct which represents the future-lock guard.
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
    use crate::{
        print, println,
        scheduling::cooperative::kasync::{AsyncRuntime, AsyncRuntimeExitErrStatus},
    };

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
