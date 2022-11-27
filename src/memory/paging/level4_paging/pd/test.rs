use x86_64::PhysAddr;

use crate::{print, println};

use super::{flags::PageDirectoryFlags, PageDirectory};

pub fn tests() {
    test_set_pd_phys_addr();
}

fn test_set_pd_phys_addr() {
    print!("[Test] PageDirectory: test_set_pd_phys_addr ... ");

    let pd = PageDirectory::new(PageDirectoryFlags::empty())
        .set_pd_phys_addr(PhysAddr::new((1 << 39) - 1));

    assert_eq!(
        pd.as_u64(),
        0b0000_0000_0000_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_0000_0000_0000
    );

    println!("OK");
}
