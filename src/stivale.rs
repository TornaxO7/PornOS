use stivale_boot::v2::{StivaleAnyVideoTag, StivaleHeader};

pub const PORNOS_STACK_SIZE: usize = 8_192;

#[link_section = ".porno_stack"]
pub static PORNOS_STACK: [u8; PORNOS_STACK_SIZE] = [0; PORNOS_STACK_SIZE];

#[used]
#[no_mangle]
#[link_section = ".stivale2hdr"]
pub static STIVALE_HEADER: StivaleHeader = StivaleHeader::new()
    .stack(PORNOS_STACK.as_ptr())
    .flags(0b11110)
    .tags(&ANY_VIDEO_HEADER_TAG as *const StivaleAnyVideoTag as *const ());

pub static ANY_VIDEO_HEADER_TAG: StivaleAnyVideoTag = StivaleAnyVideoTag::new().preference(1);

