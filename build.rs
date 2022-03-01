use std::{process::Command, io};

fn main() -> io::Result<()> {
    // paths
    let built_utils_dir: &str = "../build_utils";
    let linker_script = format!("{}/linker.ld", built_utils_dir);

    let boot_asm = format!("{}/multiboot.as", built_utils_dir);
    let boot_asm_out = format!("{}/boot.o", built_utils_dir);

    // assemble multiboot
    println!("cargo:rustc-if-changed={}", boot_asm);
    Command::new("as")
        .args(["-o", &boot_asm_out])
        .arg(&boot_asm)
        .spawn()
        .expect(&format!("Couldn't assemble {}", &boot_asm));

    // linking script
    println!("cargo:rustc-link-search={}", built_utils_dir);
    println!("cargo:rerun-if-changed={}", linker_script);
    Ok(())
}
