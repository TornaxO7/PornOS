use core::ops::{Add, AddAssign};

use x86_64::{align_down, align_up};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PhysLinearAddr(u64);

impl PhysLinearAddr {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn align_up<U: Into<u64>>(&mut self, align: U) -> Self {
        Self(align_up(self.0, align.into()))
    }

    pub fn align_down<D: Into<u64>>(&mut self, align: D) -> Self {
        Self(align_down(self.0, align.into()))
    }
}

impl Add for PhysLinearAddr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<u64> for PhysLinearAddr {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u64> for PhysLinearAddr {
    fn add_assign(&mut self, other: u64) {
        self.0 += other;
    }
}
