use x86_64::PhysAddr;

use crate::{memory::HHDM, print, println};

use super::Stack;

pub fn tests() {
    let mut stack = Stack::new();

    test_push_pop(&mut stack);
    test_get_entry(&mut stack);
    test_get_entry_addr(&mut stack);

    test_fill_frames(&stack);
}

fn test_push_pop(stack: &mut Stack) {
    print!("[Test] Frame-Allocator-Stack: test_push_pop ... ");

    let addr = PhysAddr::new(0xCAFEBABE).align_down(4096u64);

    let original_entry = stack.pop().unwrap();

    assert!(stack.push(addr).is_ok());
    assert_eq!(stack.pop(), Some(addr));
    assert!(stack.push(original_entry).is_ok());

    println!("OK");
}

fn test_get_entry(stack: &mut Stack) {
    print!("[Test] Frame-Allocator-Stack: test_get_entry ... ");

    let original_entry = stack.pop().unwrap();

    let addr = PhysAddr::new(0xCAFEBABE).align_down(4096u64);
    assert!(stack.push(addr).is_ok());

    assert!(stack.get_entry_value(stack.len).is_none());
    assert_eq!(stack.get_entry_value(stack.len - 1), Some(addr));

    assert_eq!(stack.pop(), Some(addr));
    assert!(stack.push(original_entry).is_ok());

    println!("OK");
}

fn test_get_entry_addr(stack: &mut Stack) {
    print!("[Test] Frame-Allocator-Stack: test_get_entry_addr ... ");

    // we need at least one value in it
    if stack.len == 0 {
        let test_addr = PhysAddr::new(0xB00BA).align_down(4096u64);
        assert!(stack.push(test_addr).is_ok());
    }

    assert_eq!(stack.get_entry_phys_ptr(0), Some(stack.start));

    if stack.len == 1 {
        assert!(stack.pop().is_some());
    }

    println!("OK");
}

/// Tests if you can fill each page frame completely and read from it.
fn test_fill_frames(stack: &Stack) {
    print!("[Test] Frame-Allocator-Stack: test_fill_frames ... ");
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
