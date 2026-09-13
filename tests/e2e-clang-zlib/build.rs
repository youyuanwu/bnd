use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let types_rdl = out_dir.join("types.rdl");
    let types_winmd = out_dir.join("types.winmd");
    let zlib_rdl = out_dir.join("zlib.rdl");
    let zlib_winmd = out_dir.join("zlib.winmd");
    let bindings = out_dir.join("bindings.rs");

    generate_types(&types_rdl);
    compile_rdl(&[&types_rdl], &types_winmd);
    generate_zlib(&types_winmd, &zlib_rdl);
    compile_rdl(&[&types_rdl, &zlib_rdl], &zlib_winmd);

    windows_bindgen::bindgen([
        "--in",
        zlib_winmd.to_str().unwrap(),
        "--out",
        bindings.to_str().unwrap(),
        "--filter",
        "Zlib",
        "--flat",
        "--sys",
    ]);

    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rerun-if-changed=/usr/include/zconf.h");
    println!("cargo:rerun-if-changed=/usr/include/zlib.h");
    println!("cargo:rerun-if-changed=../../bnd-clang/vendored/windows-clang/src");
    println!("cargo:rerun-if-changed=../../bnd-bindgen/vendored/windows-bindgen/src");
}

fn generate_types(output: &Path) {
    windows_clang::clang()
        .input("/usr/include/zconf.h")
        .args(["-x", "c", "-std=c11", "-I/usr/include"])
        .filter("zconf.h")
        .namespace("Zlib.Types")
        .library("z")
        .output(output)
        .write()
        .expect("generate zconf RDL");
}

fn generate_zlib(types_winmd: &Path, output: &Path) {
    windows_clang::clang()
        .input("/usr/include/zlib.h")
        .reference(types_winmd)
        .args(["-x", "c", "-std=c11", "-I/usr/include"])
        .filter("zlib.h")
        .namespace("Zlib")
        .library("z")
        .output(output)
        .write()
        .expect("generate zlib RDL");
}

fn compile_rdl(inputs: &[&Path], output: &Path) {
    windows_rdl::reader()
        .inputs(inputs)
        .reference_default()
        .output(output)
        .write()
        .expect("compile RDL");
}
