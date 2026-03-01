use std::path::Path;

/// Generate the bnd-linux source tree at `output_dir`.
///
/// 1. Runs bnd-winmd on `bnd-linux.toml` (merged posix + linux config) to
///    produce a single `.winmd` containing both `posix.*` and `linux.*`
///    partitions.
/// 2. Runs `windows-bindgen --package` to emit `src/posix/*/mod.rs` and
///    `src/linux/*/mod.rs`.
/// 3. Saves the `.winmd` under `output_dir/winmd/`.
pub fn generate(output_dir: &Path) {
    let gen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Step 1: Generate .winmd
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create winmd directory");
    let linux_winmd = winmd_dir.join("bnd-linux.winmd");
    bnd_winmd::run(&gen_dir.join("bnd-linux.toml"), Some(&linux_winmd))
        .expect("bnd-winmd failed to generate winmd");

    // Step 2: Generate crate source tree via windows-bindgen package mode
    // Both posix and linux namespaces are in the same winmd — no --reference needed.
    windows_bindgen::bindgen([
        "--in",
        linux_winmd.to_str().unwrap(),
        "--out",
        output_dir.to_str().unwrap(),
        "--filter",
        "libc",
        "--sys",
        "--package",
    ])
    .unwrap();
}
