fn main() {
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tconf/linker.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=conf/linker.ld");
}
