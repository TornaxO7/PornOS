use x86_64::PhysAddr;

use crate::{
    memory::paging::level4_paging::pt::{flags::PageTableFlags, PageTable},
    print, println,
};

pub fn tests() {
    test_set_page_phys_addr();
    test_set_protection_key();
}

fn test_set_page_phys_addr() {
    print!("[Test] Page Table: test_set_page_phys_addr ... ");

    let pt =
        PageTable::new(PageTableFlags::empty()).set_page_phys_addr(PhysAddr::new((1 << 51) - 1));

    assert_eq!(
        pt.as_u64(),
        0b0000_0000_0000_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000_0000_0000
    );

    println!("OK");
}

fn test_set_protection_key() {
    print!("[Test] Page Table: test_set_protection_key ... ");

    let pt = PageTable::new(PageTableFlags::empty()).set_protection_key(0b1111);

    assert_eq!(
        pt.as_u64(),
        0b0111_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
    );

    println!("OK");
}
