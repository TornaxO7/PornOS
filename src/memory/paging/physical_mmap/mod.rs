pub mod frame_allocator;
pub mod kernel_info;
pub mod limine;

#[cfg(feature = "test")]
pub mod tests {
    use super::*;

    pub fn main() {
        frame_allocator::tests::main();
    }
}
