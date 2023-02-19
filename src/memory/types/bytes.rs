use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, Sub},
};

/// Just a simple type-safety struct which should represent
/// the amount of bytes.
///
/// # Example
/// ```no_run
/// let bytes = Bytes::new(10);
/// ```
/// just means that `bytes` reprents the amount of 10 bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Bytes(u64);

impl Bytes {
    pub const fn new(bytes: u64) -> Self {
        Self(bytes)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        usize::try_from(self.0).unwrap()
    }
}

impl Borrow<u64> for Bytes {
    fn borrow(&self) -> &u64 {
        &self.0
    }
}

impl BorrowMut<u64> for Bytes {
    fn borrow_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl Deref for Bytes {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add for Bytes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Bytes {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Bytes {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<u64> for Bytes {
    type Output = Bytes;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<usize> for Bytes {
    type Output = Bytes;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs_u64 = u64::try_from(rhs).unwrap();
        Self(self.0 * rhs_u64)
    }
}

impl From<u16> for Bytes {
    fn from(value: u16) -> Self {
        Self(u64::from(value))
    }
}
