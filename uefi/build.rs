use std::path::Path;
use std::process::Command;
use std::{env, error::Error};

const ASM_FILES: [&str; 2] = ["host_entrypoint.S", "isr.S"];

fn main() -> Result<(), Box<dyn Error>> {
    use_direct()
}

fn use_direct() -> Result<(), Box<dyn Error>> {
    let src_dir = &format!("{}/src", env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = env::var("OUT_DIR").unwrap();
    let full_lib = &format!("{}/{}", out_dir, "asm.lib");

    let mut objs = Vec::new();

    for filename in ASM_FILES.iter() {
        let full_filename = &format!("{}/{}", src_dir, filename);
        let obj = format!("{}/{}.obj", out_dir, filename).clone();

        {
            Command::new("clang")
                .args(&[
                    full_filename,
                    "-target",
                    "x86_64-unknown-windows",
                    "-o",
                    &obj,
                    "-c",
                ])
                .current_dir(&Path::new(&out_dir))
                .status()
                .unwrap();
        }
        objs.push(obj);

        println!("cargo:rerun-if-changed={}", filename);
    }

    let mut args = vec![format!("-out:{}", full_lib)];
    args.append(&mut objs);
    Command::new("llvm-lib")
        .args(args)
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=asm");
    Ok(())
}
