use std::{env, fs, io, path::PathBuf};

const LINKER_SCRIPT_PATH: &str = "../grub/linker.ld";

fn main() -> io::Result<()> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(LINKER_SCRIPT_PATH, out_dir.join("linking_stuff.ld"))?;

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed={}", LINKER_SCRIPT_PATH);
    Ok(())
}
