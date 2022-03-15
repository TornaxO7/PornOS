use std::{io, process::Command};

fn main() -> io::Result<()> {
    // paths
    let build_utils_dir = "build_utils";

    let linker_file_path = format!("{}/linker.ld", &build_utils_dir);
    let multiboot_as_path = format!("{}/multiboot.as", &build_utils_dir);

    // assemble multiboot
    println!("cargo:rustc-if-changed={}", &multiboot_as_path);
    Command::new("as")
        .args(["-o", "build_utils/boot.o"])
        .arg(&multiboot_as_path)
        .spawn()
        .expect(&format!("Couldn't assemble {}", &multiboot_as_path));

    // linking script
    println!("cargo:rustc-link-search={}", &build_utils_dir);
    println!("cargo:rerun-if-changed={}", &linker_file_path);
    Ok(())
}
