use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // Build the fixture at a known path instead of relying on Cargo's artifact layout.
    let status = Command::new(env::var_os("RUSTC").unwrap())
        .args([
            "--edition=2024",
            "--crate-name=simple",
            "--crate-type=cdylib",
            "src/native.rs",
        ])
        .arg("--target")
        .arg(env::var_os("TARGET").unwrap())
        .arg("--out-dir")
        .arg(&out_dir)
        .status()
        .expect("run rustc for libsimple");
    assert!(status.success(), "failed to build libsimple: {status}");
    println!("cargo::rerun-if-changed=src/native.rs");
}
