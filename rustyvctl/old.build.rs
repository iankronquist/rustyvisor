use std::env;

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS");
    if os == Ok("uefi".to_string()) {
        println!("cargo:rustc-flags='-Clink-args= /subsystem:EFI_APPLICATION");
    }
}