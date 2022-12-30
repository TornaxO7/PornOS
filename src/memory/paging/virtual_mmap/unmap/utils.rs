use x86_64::structures::paging::page_table::PageTableLevel;

pub fn ptl_to_index(level: PageTableLevel) -> usize {
    match level {
        PageTableLevel::Four => 0,
        PageTableLevel::Three => 1,
        PageTableLevel::Two => 2,
        PageTableLevel::One => 3,
    }
}

pub fn next_higher_level(level: PageTableLevel) -> Option<PageTableLevel> {
    match level {
        PageTableLevel::One => Some(PageTableLevel::Two),
        PageTableLevel::Two => Some(PageTableLevel::Three),
        PageTableLevel::Three => Some(PageTableLevel::Three),
        PageTableLevel::Four => None,
    }
}
