// currently implements only 4KiB pages

use limine::{LimineMemmapRequest, LimineMemmapResponse, LimineMemmapEntry, LimineMemoryMapEntryType};

use crate::{memory::level4_paging::pmle4::PMLE4, print, println};

mod page;
mod page_table;
mod pdpt;
mod pmle4;

static MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);

pub fn init() {
    println!("Memory ... ");

    let response: &LimineMemmapResponse = MEMMAP_REQUEST.get_response().get().unwrap();
    for index in 0..response.entry_count {
        let entry: &LimineMemmapEntry = &response.memmap()[index as usize];
        if LimineMemoryMapEntryType::Usable == entry.typ {
            println!("{:?}", entry);
        }
    }

    let yes = (1086626725888u64 + 4096u64) as * mut u64;
    // let yes = 278528 as * mut u64;
    unsafe {
        *yes = 0xDEADBEEF;
    }

    println!("{:x}", unsafe {*yes});

    println!("OK");
}

pub trait KernelPage {
    fn for_kernel() -> Self;
}
