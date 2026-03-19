use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let submodule_path = PathBuf::from(&manifest_dir).join("external/riscv-opcodes");

    let status = Command::new("make")
        .arg("inst.rs")
        .current_dir(&submodule_path)
        .status()
        .expect("Failed to run make in riscv-opcodes");

    if !status.success() {
        panic!("Make failed");
    }

    let generated_file_name = "inst.rs";
    let generated_path = submodule_path.join(generated_file_name);
    let destination_path = Path::new(&out_dir).join("opcodes.rs");

    let content = fs::read_to_string(&generated_path).expect("Failed to read generated file");
    let transformed_content = content.replace("const ", "pub const ");

    fs::write(&destination_path, transformed_content)
        .expect("Failed to write opcodes.rs to OUT_DIR");

    println!("cargo:rerun-if-changed=external/riscv-opcodes/extensions");
    println!("cargo:rerun-if-changed=external/riscv-opcodes/Makefile");
    println!("cargo:rerun-if-changed=build.rs");
}
