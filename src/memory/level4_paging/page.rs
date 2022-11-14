use core::convert::TryFrom;

pub enum PageError {

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PageEntry(u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Page(u64);

impl Page {
}

impl TryFrom<u64> for Page {
    type Error = PageError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        

    }
}
