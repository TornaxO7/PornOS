
/// Mostly taken from: https://docs.rs/x86_64/0.14.10/src/x86_64/addr.rs.html#652
pub fn align_up(addr: u64, align: u64) -> u64 {
    assert!(align.is_power_of_two(), "align has to be power of two");

    let align_mask = align - 1;
    if addr & align_mask == 0 {
        addr
    } else {
        if let Some(aligned) = (addr | align_mask).checked_add(1) {
            aligned
        } else {
            panic!("Attempt to add with overflow");
        }
    }
}
