use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Default)]
pub struct Spinlock<T> {
    lock: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Spinlock<T> {}
unsafe impl<T> Send for Spinlock<T> {}

impl<'a, T> Spinlock<T> {
    const OPEN_LOCK: bool = false;
    const CLOSED_LOCK: bool = true;

    pub const fn new(value: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&'a self) -> LockGuard<'a, T> {
        while self
            .lock
            .compare_exchange(
                Self::OPEN_LOCK,
                Self::CLOSED_LOCK,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_err()
        {
            core::hint::spin_loop();
        }

        LockGuard { lock: &self }
    }
}

#[derive(Debug)]
pub struct LockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<'a, T> Deref for LockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> DerefMut for LockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<'a, T> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock
            .lock
            .store(Spinlock::<T>::OPEN_LOCK, Ordering::Release);
    }
}
