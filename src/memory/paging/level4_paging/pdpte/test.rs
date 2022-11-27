use x86_64::PhysAddr;

use crate::{print, println, memory::paging::level4_paging::pdpte::{PDPTE, flags::PDPTEFlags}};

pub fn tests() {
    test_set_pd_phys_addr();
}

fn test_set_pd_phys_addr() {
    print!("[Test] PMLE4: test_set_pd_phys_addr ... ");

    let pdpt = PDPTE::new(PDPTEFlags::empty())
        .set_pd_phys_addr(PhysAddr::new((1 << 51) - 1));

    assert_eq!(pdpt.as_u64(),
    0b0000_0000_0000_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000_0000_0000);

    println!("OK");
}
