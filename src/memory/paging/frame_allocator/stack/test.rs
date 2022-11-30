use x86_64::PhysAddr;

use crate::memory::paging::{PageSize, PhysMemMap};
use crate::{print, println};

use core::fmt::Debug;

use super::Stack;

pub fn tests<P: PageSize + Send + Sync + Debug>(phys_mmap: &PhysMemMap<P>) {
    let mut stack = Stack::<P>::new(phys_mmap);

    test_push_pop(&mut stack);
    test_get_entry(&mut stack);
    test_get_entry_addr(&mut stack);
}

fn test_push_pop<P: PageSize + Send + Sync + Debug>(stack: &mut Stack<P>) {
    print!("[Test] Frame-Allocator-Stack: test_push_pop ... ");

    let addr = PhysAddr::new(0xCAFEBABE);

    assert!(stack.pop().is_some());

    assert!(stack.push(addr));
    assert_eq!(stack.pop(), Some(addr));

    println!("OK");
}

fn test_get_entry<P: PageSize + Send + Sync + Debug>(stack: &mut Stack<P>) {
    print!("[Test] Frame-Allocator-Stack: test_get_entry ... ");

    let addr = PhysAddr::new(0xCAFEBABE);
    assert!(stack.pop().is_some());
    assert!(stack.push(addr));

    assert!(stack.get_entry(stack.len).is_none());
    assert_eq!(stack.get_entry(stack.len - 1), Some(addr));

    println!("OK");
}

fn test_get_entry_addr<P: PageSize + Send + Sync + Debug>(stack: &mut Stack<P>) {
    print!("[Test] Frame-Allocator-Stack: test_get_entry_addr ... ");

    let test_addr = PhysAddr::new(0xB00BA);

    if stack.len == 0 {
        assert!(stack.push(test_addr));
    }

    assert_eq!(stack.get_entry_addr(0), Some(stack.start));

    println!("OK");
}
