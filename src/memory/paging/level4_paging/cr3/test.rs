use x86_64::PhysAddr;

use crate::{print, println};

use super::Cr3Value;

pub fn tests() {
    pwt_test();
    pcd_test();
    set_pml4e_phys_addr_test();
}

fn pwt_test() {
    print!("[Test] Cr3Value: pwt_test ... ");

    let cr3 = Cr3Value::new().set_pwt();
    assert_eq!(cr3.as_u64(), 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000);

    println!("OK");
}

fn pcd_test() {
    print!("[Test] Cr3Value: pcd_test ... ");

    let cr3 = Cr3Value::new().set_pcd();

    assert_eq!(cr3.as_u64(), 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000);

    println!("OK");
}

fn set_pml4e_phys_addr_test() {
    print!("[Test] Cr3Value: set_pml4e_phys_addr_test ... ");
    let cr3 = Cr3Value::new().set_pml4e_phys_addr(PhysAddr::new((1 << 52) - 1));

    assert_eq!(cr3.as_u64(),
    0b0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000_0000_0000);
    
    println!("OK");
}
