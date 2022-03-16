use std::io;

fn main() -> io::Result<()> {
    // paths
    let build_utils_dir = "build_utils";

    let linker_file_path = format!("{}/linker.ld", &build_utils_dir);

    // linking script
    println!("cargo:rustc-link-search={}", &build_utils_dir);
    println!("cargo:rerun-if-changed={}", &linker_file_path);
    Ok(())
}
