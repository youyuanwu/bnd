use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let fixtures = manifest_dir.join("../../bnd-clang/tests/fixtures/multi");

    let types_rdl = out_dir.join("types.rdl");
    let types_winmd = out_dir.join("types.winmd");
    let widget_rdl = out_dir.join("widget.rdl");
    let multi_winmd = out_dir.join("multi.winmd");
    let bindings = out_dir.join("bindings.rs");

    generate_types(&fixtures, &types_rdl);
    compile_rdl(&[&types_rdl], &types_winmd);
    generate_widget(&fixtures, &types_winmd, &widget_rdl);
    compile_rdl(&[&types_rdl, &widget_rdl], &multi_winmd);

    windows_bindgen::bindgen([
        "--in",
        multi_winmd.to_str().unwrap(),
        "--out",
        bindings.to_str().unwrap(),
        "--filter",
        "MultiTest",
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

fn generate_types(fixtures: &Path, output: &Path) {
    windows_clang::clang()
        .input(fixtures.join("types.h"))
        .args(["-x", "c", "-std=c11"])
        .namespace("MultiTest.Types")
        .library("simple")
        .output(output)
        .write()
        .expect("generate types RDL");
}

fn generate_widget(fixtures: &Path, types_winmd: &Path, output: &Path) {
    windows_clang::clang()
        .input(fixtures.join("widget.h"))
        .reference(types_winmd)
        .args(["-x", "c", "-std=c11"])
        .filter("widget.h")
        .namespace("MultiTest.Widgets")
        .library("simple")
        .output(output)
        .write()
        .expect("generate widget RDL");
}

fn compile_rdl(inputs: &[&Path], output: &Path) {
    windows_rdl::reader()
        .inputs(inputs)
        .output(output)
        .write()
        .expect("compile RDL");
}
