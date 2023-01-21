pub mod kasync;

#[cfg(feature = "test")]
pub mod tests {
    use super::kasync;

    pub fn main() {
        kasync::tests::main();
    }
}
