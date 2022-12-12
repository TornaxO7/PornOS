use x86_64::structures::paging::PageSize;

use super::PhysMemMap;

impl<P: PageSize> PhysMemMap<P> {
    /// this can be used to debug it
    pub fn test(&self) {
    }
}
