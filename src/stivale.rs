use stivale_boot::v2::{StivaleHeader, StivaleFramebufferHeaderTag};

use crate::vga::VGA;

pub const PORNOS_STACK_SIZE: usize = 8_192;
pub static PORNOS_STACK: [u8; PORNOS_STACK_SIZE] = [0; PORNOS_STACK_SIZE];

#[no_mangle]
#[link_section = ".stivale2hdr"]
pub static STIVALE_HEADER: StivaleHeader = StivaleHeader::new()
    .stack(unsafe {
        PORNOS_STACK
            .as_ptr()
            .offset(PORNOS_STACK_SIZE as isize)
    })
    .flags((1 << 3) | (1 << 4))
    .tags(&FRAME_BUFFER_TAG_HEADER as * const StivaleFramebufferHeaderTag as * const ());

pub static FRAME_BUFFER_TAG_HEADER: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .framebuffer_height(VGA::BUFFER_HEIGHT as u16)
    .framebuffer_width(VGA::BUFFER_WIDTH as u16);
