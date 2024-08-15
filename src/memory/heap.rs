use good_memory_allocator::SpinLockedAllocator;
use x86_64::{
    structures::paging::{
        Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags, Size4KiB,
    },
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

fn _traverse(p4_table: &PageTable, page: &Page) {
    let addr = page.start_address();

    let p3 = &p4_table[addr.p4_index()];
    if p3.is_unused() {
        serial_println!("p3 is unused");
        return;
    }
    let p3_table: &mut PageTable =
        unsafe { &mut *(VirtAddr::new(p3.addr().as_u64() + super::get_hhdm()).as_mut_ptr()) };

    let p2 = &p3_table[addr.p3_index()];
    if p2.is_unused() {
        serial_println!("p2 is unused");
        return;
    }
    let p2_table: &mut PageTable =
        unsafe { &mut *(VirtAddr::new(p2.addr().as_u64() + super::get_hhdm()).as_mut_ptr()) };

    let p1 = &p2_table[addr.p2_index()];
    if p1.is_unused() {
        serial_println!("p1 is unused");
        return;
    }
    let p1_table: &mut PageTable =
        unsafe { &mut *(VirtAddr::new(p1.addr().as_u64() + super::get_hhdm()).as_mut_ptr()) };

    let entry = &p1_table[addr.p1_index()];
    if entry.is_unused() {
        serial_println!("Entry is unused");
        return;
    }

    serial_println!("It's mapped ^^");
}
