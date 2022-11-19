use crate::memory::types::Bytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageSize {
    Page4KB,
}

impl PageSize {
    pub fn size(&self) -> Bytes {
        match self {
            PageSize::Page4KB => Bytes::new(4096),
        }
    }
}
