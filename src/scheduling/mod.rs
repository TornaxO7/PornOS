pub mod cooperative;

#[cfg(feature = "test")]
pub mod tests {
    use super::cooperative;

    pub fn main() {
        cooperative::tests::main();
    }
}
