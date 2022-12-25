use x86_64::{
    structures::paging::{Page, PageSize, PageTableFlags, PhysFrame, PageTable},
    PhysAddr, VirtAddr,
};

use crate::memory::types::Bytes;

use super::level_page_tables::PTLevels;

/// A trait which each VM-Mapper should implement.
pub unsafe trait VMMMapper<P: PageSize>: VMmapperUnmap<P> + VMMapperMap<P> {
    fn new() -> Self;

    /// Implements a standard translation function how the mapper translates the
    /// givien physical address.
    ///
    /// * `addr`: the physical address which shoulud be translated into a
    /// virtual address.
    fn translate_addr(&self, addr: PhysAddr) -> VirtAddr;
}

pub unsafe trait VMmapperUnmap<P: PageSize> {
    /// Unmpas the given page and returns the unmapped page frame if everything
    /// works fine.
    ///
    /// * `page`: The page which should be unmapped.
    ///
    /// # Returns
    /// - `Ok(addr)`: The page frame which was mapped by the given page.
    /// - `Err(addr)`:
    unsafe fn unmap_page(&self, page: Page) -> Option<PhysFrame>;

    unsafe fn get_page_tables(&self, page: Page) -> Option<PTLevels>;
}

pub unsafe trait VMMapperMap<P: PageSize> {
    /// Maps a page to the given page_frame (if available) with the given flags.
    ///
    /// * `page`: The page to be mapped.
    /// * `page_frame`: If it's `Some`, then the page will be mapped to the given page frame,
    ///                 otherwise a new page frame will ba allocated.
    /// * `flags`: The flags for the given mapping.
    unsafe fn map_page(&self, page: Page, page_frame: Option<PhysFrame>, flags: PageTableFlags);

    /// Maps the given page frame by a standard-mapping implementation.
    ///
    /// * `page_frame`: The page frame which should be mapped.
    /// * `flags`: The flags for the page.
    unsafe fn map_page_frame(&self, page_frame: PhysFrame, flags: PageTableFlags);

    /// Maps a range of pages in a romw.
    ///
    /// * `page`: The starting page which should be mapped.
    /// * `page_frame`: The starting page frame (if available) which should be mapped.
    ///                 If it's `None`, random page-frames are picked up then.
    /// * `len`: The amount of bytes which should be mapped in a row.
    /// * `flags`: The flags for each page.
    ///
    /// # Note
    /// If `page_frame` is `Some(...)`, then you **have to** make sure that, the range, starting
    /// from the given page frame until `start + len` is **a valid Physicall address range**!!!
    unsafe fn map_page_range(
        &self,
        page: Page,
        page_frame: Option<PhysFrame>,
        len: Bytes,
        flags: PageTableFlags,
    );
}
