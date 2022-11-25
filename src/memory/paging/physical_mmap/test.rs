use crate::println;

use crate::memory::types::Byte;

use super::{PhysMemMap, PhysLinearAddr};

impl PhysMemMap {
    /// this can be used to debug it
    pub fn test(&self) {
        self.write_normal_text();
        self.write_into_next_mem_chunk();
    }

    /// simulates a simple text into the first page
    fn write_normal_text(&self) {
        let mut addr = PhysLinearAddr::new(0);

        for c in "Hello".chars() {
            let ascii_byte = Byte::new(c as u8);
            assert!(self.write_byte(ascii_byte, addr));
            addr += 1;
        }

    }

    /// simulates as if you would try to write into the next memory chunk
    fn write_into_next_mem_chunk(&self) {
        let mut addr = PhysLinearAddr::new(4096);

        for c in "Hello2".chars() {
            let ascii_byte = Byte::new(c as u8);
            assert!(self.write_byte(ascii_byte, addr));
            addr += 1;
        }
    }

}
