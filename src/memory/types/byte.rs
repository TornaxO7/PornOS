use core::ops::{Deref, DerefMut};

/// This struct simply represents a byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Byte(u8);

impl Byte {
    /// Creates a new byte by the given value.
    pub fn new(byte: u8) -> Self {
        Self(byte)
    }
}

impl Deref for Byte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Byte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
