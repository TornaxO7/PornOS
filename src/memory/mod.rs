use limine::LimineMemmapRequest;
use x86_64::{VirtAddr, structures::paging::PageTable};

static MMAP_FEATURE: LimineMemmapRequest = LimineMemmapRequest::new(0);

pub fn init() {
    match MMAP_FEATURE.get_response().get() {
        Some(memmap_response) => (),
        None => (),
    };
}

fn enable_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: * mut PageTable = virt.as_mut_ptr();

    unsafe {
        &mut *page_table_ptr
    }
}