use std::{env, error::Error, fs::File, io::Write, path::PathBuf};
use std::process::Command;
use std::path::Path;
use cc::Build;

const ASM_FILES: [&str;2] = [
    "host_entrypoint.S",
    "isr.S",
];


fn main() -> Result<(), Box<dyn Error>> {
    use_direct()
}

fn use_direct() -> Result<(), Box<dyn Error>> {
    let src_dir = &format!("{}/src", env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    let full_lib = &format!("{}/{}", out_dir,"asm.lib");

    let mut objs = Vec::new();

    for filename in ASM_FILES.iter() {
        let full_filename =  &format!("{}/{}", src_dir,filename);
        let obj =  format!("{}/{}.obj", out_dir,filename).clone();

        {
    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("clang").args(&[full_filename, "-target", "x86_64-unknown-windows", "-o", &obj, "-c"])
    .current_dir(&Path::new(&out_dir))
    .status().unwrap();
    }
    objs.push(obj);

    println!("cargo:rerun-if-changed={}", filename);

    }

    let mut args = vec![format!("-out:{}", full_lib)];
    args.append(&mut objs);
    Command::new("llvm-lib").args(args)
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=asm");
    Ok(())
}

fn use_cc() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let src_dir = &format!("{}/src", env::var("CARGO_MANIFEST_DIR").unwrap());


    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put `link.x` in the build directory
    //File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    for file_name in ASM_FILES.iter() {
        let full_file_name = &format!("{}/{}", src_dir,file_name);
        //Build::new().compiler("clang").file(file).target("x86_64-unknown-uefi").compile("asm");
        Build::new().compiler("clang").file(full_file_name).target("x86_64-unknown-uefi").compile("asm");
        println!("cargo:rerun-if-changed={}", full_file_name);
    }

    Ok(())
}
