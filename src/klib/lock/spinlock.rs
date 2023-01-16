use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Default)]
pub struct Spinlock<T> {
    is_locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Spinlock<T> {}
unsafe impl<T> Send for Spinlock<T> {}

impl<'a, T> Spinlock<T> {
    const IS_OPEN: bool = false;
    const IS_CLOSED: bool = true;

    pub const fn new(value: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&'a self) -> LockGuard<'a, T> {
        loop {
            if self
                .is_locked
                .compare_exchange_weak(
                    Self::IS_OPEN,
                    Self::IS_CLOSED,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                break;
            }

            while self.is_locked.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
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
            .is_locked
            .store(Spinlock::<T>::IS_OPEN, Ordering::Release);
    }
}
