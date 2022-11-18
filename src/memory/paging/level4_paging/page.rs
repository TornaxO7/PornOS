use crate::memory::PhysAddr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Page(PhysAddr);
