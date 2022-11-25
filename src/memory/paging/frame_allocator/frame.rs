use x86_64::VirtAddr;

use crate::memory::{types::Bytes, paging::PageSize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Frame {
    pub start: VirtAddr,
    pub len: PageSize,
}
