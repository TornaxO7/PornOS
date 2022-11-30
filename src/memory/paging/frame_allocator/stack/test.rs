use x86_64::PhysAddr;

use crate::memory::paging::page_frame::PageFrame;
use crate::memory::paging::{PageSize, PhysMemMap};
use crate::{print, println};

use super::Stack;

pub fn tests(phys_mmap: &PhysMemMap) {
    let mut stack = Stack::new(phys_mmap, PageSize::Page4KB);

    test_push_pop(&mut stack);
    test_get_entry(&mut stack);
    test_get_entry_addr(&mut stack);
}

fn test_push_pop(stack: &mut Stack) {
    print!("[Test] Frame-Allocator-Stack: test_push_pop ... ");

    let addr = PhysAddr::new(0xCAFEBABE);

    assert!(stack.pop().is_some());

    assert!(stack.push(PageFrame {
        start: addr,
        size: PageSize::Page4KB
    }));
    assert_eq!(
        stack.pop(),
        Some(PageFrame {
            start: addr,
            size: PageSize::Page4KB
        })
    );

    println!("OK");
}

fn test_get_entry(stack: &mut Stack) {
    print!("[Test] Frame-Allocator-Stack: test_get_entry ... ");

    let addr = PhysAddr::new(0xCAFEBABE);
    assert!(stack.pop().is_some());
    assert!(stack.push(PageFrame {
        start: addr,
        size: PageSize::Page4KB
    }));

    assert!(stack.get_entry(stack.len).is_none());
    assert_eq!(stack.get_entry(stack.len - 1), Some(addr.as_u64()));

    println!("OK");
}

fn test_get_entry_addr(stack: &mut Stack) {
    print!("[Test] Frame-Allocator-Stack: test_get_entry_addr ... ");

    let test_addr = PhysAddr::new(0xB00BA);

    if stack.len == 0 {
        assert!(stack.push(PageFrame {
            start: test_addr,
            size: PageSize::Page4KB
        }));
    }

    assert_eq!(stack.get_entry_addr(0), Some(stack.start));

    println!("OK");
}
