use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static SIMPLE_WINMD: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let temp = tempfile::tempdir().expect("create temporary output directory");
    let rdl = temp.path().join("simple.rdl");
    let winmd = temp.path().join("simple.winmd");

    windows_clang::clang()
        .input(fixture("simple/simple.h"))
        .target("x86_64-unknown-linux-gnu")
        .args([
            "-x",
            "c",
            "-std=c11",
            "-DCUSTOM_DEPTH=42",
            "-DBND_CLANG_SKIP_INT128",
        ])
        .namespace("SimpleTest")
        .library("simple")
        .output(&rdl)
        .write()
        .expect("generate RDL from simple.h");

    windows_rdl::reader()
        .input(&rdl)
        .output(&winmd)
        .write()
        .expect("compile generated simple RDL");

    std::fs::read(winmd).expect("read generated simple WinMD")
});

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(path)
}

fn open_index() -> windows_metadata::reader::Index {
    let file =
        windows_metadata::reader::File::new(SIMPLE_WINMD.clone()).expect("parse generated WinMD");
    windows_metadata::reader::Index::new(vec![file])
}

#[test]
fn generates_linux_long_as_64_bit() {
    use windows_metadata::reader::HasAttributes;

    let index = open_index();
    let inner = index.expect("SimpleTest", "AlignedInner");
    let fields: Vec<_> = inner.fields().collect();

    assert!(inner.has_attribute("AlignmentAttribute"));
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "a");
    assert_eq!(fields[0].ty(), windows_metadata::Type::I64);
    assert_eq!(fields[1].name(), "b");
    assert_eq!(fields[1].ty(), windows_metadata::Type::I64);
}

#[test]
fn generates_embedded_aligned_record() {
    let index = open_index();
    let outer = index.expect("SimpleTest", "EmbeddingAligned");
    let fields: Vec<_> = outer.fields().collect();

    assert_eq!(
        fields.iter().map(|field| field.name()).collect::<Vec<_>>(),
        ["before_a", "before_b", "aligned_member", "after"]
    );
    assert_eq!(fields[0].ty(), windows_metadata::Type::I64);
    assert_eq!(fields[1].ty(), windows_metadata::Type::I64);
    assert_eq!(
        fields[2].ty(),
        windows_metadata::Type::value_named("SimpleTest", "AlignedInner")
    );
    assert_eq!(fields[3].ty(), windows_metadata::Type::I32);
}

#[test]
fn preserves_windows_long_as_32_bit() {
    let temp = tempfile::tempdir().expect("create temporary output directory");
    let rdl = temp.path().join("widths.rdl");
    let winmd = temp.path().join("widths.winmd");

    windows_clang::clang()
        .input_text("struct Widths { long signed_value; unsigned long unsigned_value; };")
        .target("x86_64-pc-windows-msvc")
        .args(["-x", "c", "-std=c11"])
        .namespace("TargetTest")
        .output(&rdl)
        .write()
        .expect("generate Windows width RDL");

    windows_rdl::reader()
        .input(&rdl)
        .output(&winmd)
        .write()
        .expect("compile Windows width RDL");

    let file = windows_metadata::reader::File::new(
        std::fs::read(winmd).expect("read Windows width WinMD"),
    )
    .expect("parse Windows width WinMD");
    let index = windows_metadata::reader::Index::new(vec![file]);
    let widths = index.expect("TargetTest", "Widths");
    let fields: Vec<_> = widths.fields().collect();

    assert_eq!(fields[0].ty(), windows_metadata::Type::I32);
    assert_eq!(fields[1].ty(), windows_metadata::Type::U32);
}
