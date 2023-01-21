use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicU8, Ordering},
};

#[derive(Debug)]
pub struct Once<T> {
    status: AtomicU8,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Once<T> {}

impl<T> Once<T> {
    pub const fn new() -> Self {
        Self {
            status: AtomicU8::new(Status::Uninitialised as u8),
            data: UnsafeCell::new(unsafe { core::mem::MaybeUninit::zeroed().assume_init() }),
        }
    }

    pub fn get(&self) -> Option<&T> {
        if self.status.load(Ordering::Acquire) == Status::Initialised as u8 {
            Some(unsafe { &*self.data.get() })
        } else {
            None
        }
    }

    pub fn call_once(&self, value: impl FnOnce() -> T) {
        if self
            .status
            .compare_exchange(
                Status::Uninitialised as u8,
                Status::Running as u8,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            unsafe {
                *self.data.get() = value();
            }

            self.status
                .store(Status::Initialised as u8, Ordering::Release);
        }
    }
}

#[repr(u8)]
enum Status {
    Uninitialised,
    Running,
    Initialised,
}
