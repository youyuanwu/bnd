fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let fixtures = manifest_dir.join("../../bnd-clang/tests/fixtures/simple");
    let rdl = out_dir.join("simple.rdl");
    let winmd = out_dir.join("simple.winmd");
    let bindings = out_dir.join("bindings.rs");

    windows_clang::clang()
        .inputs([
            fixtures.join("simple.h"),
            fixtures.join("partial_bitfield.h"),
        ])
        .args(["-x", "c", "-std=c11", "-DCUSTOM_DEPTH=42"])
        .namespace("SimpleTest")
        .library("simple")
        .output(&rdl)
        .write()
        .expect("generate simple RDL");

    windows_rdl::reader()
        .input(&rdl)
        .output(&winmd)
        .write()
        .expect("compile simple RDL");

    windows_bindgen::bindgen([
        "--in",
        winmd.to_str().unwrap(),
        "--out",
        bindings.to_str().unwrap(),
        "--filter",
        "SimpleTest",
        "--flat",
        "--sys",
    ]);

    let target_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("cannot derive target directory from OUT_DIR");
    println!("cargo:rustc-link-search=native={}", target_dir.display());
    println!("cargo:rustc-link-lib=dylib=simple");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", target_dir.display());
    println!("cargo:rerun-if-changed={}", fixtures.display());
    println!("cargo:rerun-if-changed=../../bnd-clang/vendored/windows-clang/src");
    println!("cargo:rerun-if-changed=../../bnd-bindgen/vendored/windows-bindgen/src");
}
