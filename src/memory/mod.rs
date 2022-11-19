use limine::LimineHhdmRequest;
use x86_64::VirtAddr;

use crate::{print, println};

pub mod paging;
pub mod types;
pub mod util;

lazy_static::lazy_static! {
    pub static ref HHDM: VirtAddr = VirtAddr::new(LimineHhdmRequest::new(0)
        .get_response()
        .get()
        .unwrap()
        .offset);
}

pub fn init() {
    print!("Memory ... ");

    paging::init();

    println!("OK");
}
