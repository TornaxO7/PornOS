#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PhysAddr(u64);

impl PhysAddr {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}
