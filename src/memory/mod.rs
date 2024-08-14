pub mod frame_allocator;
pub mod heap;
pub mod mapper;

pub use frame_allocator::get_fak;
pub use mapper::get_simp;

use limine::request::HhdmRequest;

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub fn get_hhdm() -> u64 {
    HHDM_REQUEST.get_response().unwrap().offset()
}
