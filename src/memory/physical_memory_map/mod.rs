use limine::LimineMemmapRequest;
use spin::mutex::SpinMutex;

use crate::{memory::physical_memory_map::startup::STARTUP_MMAP, print, println};

use super::Bytes;

pub mod main;
pub mod mem_chunk;
pub mod startup;

pub type LinearAddrMemmap = u64;

static MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);

lazy_static::lazy_static! {
    pub static ref PHYS_MEMMAP: SpinMutex<&'static dyn PhysMemMapper> = SpinMutex::new(&STARTUP_MMAP);
}

pub trait PhysMemMapper: Sync {
    /// Returns the useable memory in bytes for the OS.
    fn useable_mem(&self) -> Bytes;

    /// initialise the memory mapper
    fn init(&mut self);
}

pub fn init() {
    print!("\tStartup Physical-MMAP ... ");

    println!("OK");
}
