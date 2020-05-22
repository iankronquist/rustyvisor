use std::{env, fs::File, io::Write, path::Path, process::Command};

fn tags() -> String {
    let output = Command::new("git")
        .args(&["describe", "--abbrev=0"])
        .output()
        .expect("Failed to execute git describe");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn branch() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to execute git rev-parse");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn date() -> String {
    let output = Command::new("date")
        .output()
        .expect("Failed to execute date");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn commit_sha() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git rev-parse");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn local_changes() -> bool {
    let output = Command::new("git")
        .args(&["diff", "--exit-code"])
        .output()
        .expect("Failed to execute git diff");
    !output.status.success()
}

fn kernel_version() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute uname");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let tag = tags();
    let tag_message = if !tag.is_empty() {
        format!("Tag: {}\n", tag)
    } else {
        String::new()
    };
    let mut change_message = "";

    if local_changes() {
        change_message = " with local changes";
    }

    let version_info = format!(
        "RustyVisor Version: {}\nCommit: {}{}\n{}Branch: {}\nBuilt on: {}\nKernel version: {}",
        version,
        commit_sha(),
        change_message,
        tag_message,
        branch(),
        date(),
        kernel_version()
    );

    let version_code = format!("const VERSION: &'_ str = \"{}\";", version_info);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(version_code.as_bytes()).unwrap();
}
