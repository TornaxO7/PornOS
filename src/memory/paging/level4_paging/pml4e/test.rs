use x86_64::PhysAddr;

use crate::{
    memory::paging::level4_paging::pml4e::{flags::PML4EFlags, PML4E},
    print, println,
};

pub fn tests() {
    set_pdpt_phys_addr();
}

fn set_pdpt_phys_addr() {
    print!("[Test] PMLE4: set_pdpt_phys_addr ... ");

    let pml4e = PML4E::new(PML4EFlags::empty()).set_pdpt_phys_addr(PhysAddr::new((1 << 51) - 1));

    assert_eq!(
        pml4e.as_u64(),
        0b0000_0000_0000_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000_0000_0000
    );

    println!("OK");
}
