// use stivale_boot::v2::{StivaleHeader, StivaleFramebufferHeaderTag, StivaleAnyVideoTag};
//
// // use crate::vga::VGA;
//
// pub mod consoles;
//
// pub const PORNOS_STACK_SIZE: usize = 8_192;
// pub static PORNOS_STACK: [u8; PORNOS_STACK_SIZE] = [0; PORNOS_STACK_SIZE];
//
// // #[used]
// // #[no_mangle]
// // #[link_section = ".stivale2hdr"]
// // pub static STIVALE_HEADER: StivaleHeader = StivaleHeader::new()
// //     .stack(unsafe {
// //         PORNOS_STACK
// //             .as_ptr()
// //             .offset(PORNOS_STACK_SIZE as isize)
// //     })
// //     .flags((1 << 3) | (1 << 4))
// //     .tags(&ANY_VIDEO_HEADER_TAG as * const StivaleAnyVideoTag as * const ());
// //     // .tags(&FRAME_BUFFER_HEADER_TAG as * const StivaleFramebufferHeaderTag as * const ());
// //
// // // pub static FRAME_BUFFER_HEADER_TAG: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
// // //     .framebuffer_height(VGA::BUFFER_HEIGHT as u16)
// // //     .framebuffer_width(VGA::BUFFER_WIDTH as u16)
// // //     .next(&ANY_VIDEO_HEADER_TAG as * const StivaleAnyVideoTag as * const ());
// // //
// // pub static ANY_VIDEO_HEADER_TAG: StivaleAnyVideoTag = StivaleAnyVideoTag::new()
// //     .preference(1);

