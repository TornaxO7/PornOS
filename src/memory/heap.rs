use good_memory_allocator::SpinLockedAllocator;
use x86_64::{
    structures::paging::{Mapper, OffsetPageTable, Page, PageSize, PageTableFlags, Size4KiB},
    VirtAddr,
};

use crate::{serial_print, serial_println};

type Bytes = usize;
const HEAP_SIZE: Bytes = x86_64::align_up(256_000_000, Size4KiB::SIZE) as usize; // at least 256 MB
const HEAP_START: VirtAddr = VirtAddr::new(x86_64::align_up(0xBABE, Size4KiB::SIZE));

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub fn init() {
    serial_println!("Init Heap...");

    // prepare the heap cause I'm a sheep... eh what?
    serial_print!("\tPreparing page tables...");
    prepare_heap_4096();
    serial_println!("OK");

    serial_print!("\tPrepare Allocator...");
    unsafe {
        ALLOCATOR.init(HEAP_START.as_u64() as usize, HEAP_SIZE);
    }
    serial_println!("OK");

    serial_println!("...OK");
}

fn prepare_heap_4096()
where
    OffsetPageTable<'static>: Mapper<Size4KiB>,
{
    let mut fak = super::get_fak();
    let mut simp = super::get_simp();

    let amount_page_frames = HEAP_SIZE / Size4KiB::SIZE as usize;
    for i in 0..amount_page_frames as u64 {
        let page = Page::from_start_address(HEAP_START + i * Size4KiB::SIZE).unwrap();
        let frame = fak.pop().unwrap();

        let flusher = unsafe {
            simp.map_to(
                page,
                frame,
                PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                &mut (*fak),
            )
            .unwrap()
        };

        let phys_frame = simp.translate_page(page).unwrap();
        assert_eq!(phys_frame, frame);

        flusher.flush();

        unsafe {
            page.start_address()
                .as_mut_ptr::<&'static str>()
                .write("bro why")
        };
    }
}
