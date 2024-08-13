pub mod frame_allocator;
pub mod heap;
pub mod mapper;

pub use mapper::SIMP;

use limine::{
    memory_map::Entry,
    request::{HhdmRequest, MemoryMapRequest},
};
use spin::{Mutex, MutexGuard, Once};

#[used]
#[link_section = ".requests"]
static MMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

static FREE_ENTRIES: Once<Mutex<FreeEntryIterator>> = Once::new();

pub struct FreeEntryIterator {
    entries: &'static [&'static Entry],
    pub fak: Option<&'static Entry>,
    pub heap: Option<&'static Entry>,
    index: usize,
}

impl FreeEntryIterator {
    fn new() -> Self {
        let entries = MMAP_REQUEST.get_response().unwrap().entries();

        Self {
            entries,
            fak: None,
            heap: None,
            index: 0,
        }
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}

impl Iterator for FreeEntryIterator {
    type Item = &'static Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.entries.len() {
            return None;
        }

        let next_entry = self.entries[self.index];
        self.index += 1;

        if let Some(fak) = self.fak {
            if fak.base == next_entry.base {
                return self.next();
            }
        }

        if let Some(heap) = self.heap {
            if heap.base == next_entry.base {
                return self.next();
            }
        }

        Some(next_entry)
    }
}

pub fn get_free_entries<'a>() -> MutexGuard<'a, FreeEntryIterator> {
    FREE_ENTRIES.call_once(|| Mutex::new(FreeEntryIterator::new()));
    let mut iterator = FREE_ENTRIES.get().unwrap().lock();
    iterator.reset();
    iterator
}

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub fn get_hhdm() -> u64 {
    HHDM_REQUEST.get_response().unwrap().offset()
}
