use x86_64::PhysAddr;

use crate::{
    memory::{paging::frame_allocator::stack::StackPushError, HHDM},
    print, println,
};

use super::Stack;

pub fn tests() {
    test_stack_full_push();
    test_stack_empty_pop();
    test_fill_frames();
}

/// Tests if you get an error if you try to push into a full stack.
fn test_stack_full_push() {
    print!("[Test] Frame-Allocator-Stack: test_stack_full_push ... ");
    let mut stack = Stack::new();

    assert_eq!(stack.push(PhysAddr::zero()), Err(StackPushError::FullStack));

    println!("OK");
}

/// Tests if you get `None` back if the stack is empty.
fn test_stack_empty_pop() {
    print!("[Test] Frame-Allocator-Stack: test_stack_empty_pop ... ");
    let mut stack = Stack::new();

    for _ in 0..stack.len {
        assert!(stack.pop().is_some());
    }

    assert!(stack.pop().is_none());

    println!("OK");
}

/// Tests if you can fill each page frame completely and read from it.
fn test_fill_frames() {
    print!("[Test] Frame-Allocator-Stack: test_fill_frames ... ");
    let stack = Stack::new();

    for page_frame in stack.clone().into_iter() {
        let page = *HHDM + page_frame.start_address().as_u64();
        let page_ptr = page.as_mut_ptr() as *mut [u64; 512];

        let full_page_frame: [u64; 512] = [u64::MAX; 512];
        let orig_page_frame: [u64; 512] = unsafe { *page_ptr };

        // write test
        unsafe {
            *page_ptr = full_page_frame;
        };

        // read test
        assert_eq!(unsafe { *page_ptr }, full_page_frame);

        // clean up
        unsafe { *page_ptr = orig_page_frame };
    }

    println!("OK");
}
