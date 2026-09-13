use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static SIMPLE_WINMD: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let temp = tempfile::tempdir().expect("create temporary output directory");
    let rdl = temp.path().join("simple.rdl");
    let winmd = temp.path().join("simple.winmd");

    windows_clang::clang()
        .inputs([
            fixture("simple/simple.h"),
            fixture("simple/partial_bitfield.h"),
        ])
        .args(["-x", "c", "-std=c11", "-DCUSTOM_DEPTH=42"])
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
fn generates_simple_types() {
    let index = open_index();
    let types: Vec<String> = index
        .types()
        .filter(|ty| ty.namespace() == "SimpleTest")
        .map(|ty| ty.name().to_string())
        .collect();

    for expected in [
        "AlignedInner",
        "BitfieldKind",
        "CacheAligned",
        "Color",
        "CompareFunc",
        "EmbeddingAligned",
        "HasAnonUnion",
        "NetAddr",
        "Rect",
        "Value",
        "Widget",
        "WithAnon2DArrayField",
        "WithAnonArrayField",
        "WithBitfield",
    ] {
        assert!(
            types.iter().any(|name| name == expected),
            "{expected} missing. Found: {types:?}"
        );
    }
}

#[test]
fn generates_union_and_anonymous_records() {
    let index = open_index();

    let value = index.expect("SimpleTest", "Value");
    assert!(
        value
            .flags()
            .contains(windows_metadata::TypeAttributes::ExplicitLayout)
    );
    assert_eq!(
        value.fields().map(|field| field.name()).collect::<Vec<_>>(),
        ["i", "f", "bytes"]
    );

    let net_addr = index.expect("SimpleTest", "NetAddr");
    assert_eq!(
        net_addr
            .fields()
            .map(|field| field.name())
            .collect::<Vec<_>>(),
        ["addr", "scope_id"]
    );

    let anonymous = index.expect("SimpleTest", "HasAnonUnion");
    assert_eq!(
        anonymous
            .fields()
            .map(|field| field.name())
            .collect::<Vec<_>>(),
        ["before", "Anonymous", "after"]
    );
}

#[test]
fn generates_anonymous_record_arrays() {
    let index = open_index();

    let two_dimensional = index.expect("SimpleTest", "WithAnon2DArrayField");
    let fields: Vec<_> = two_dimensional.fields().collect();
    assert_eq!(fields[0].name(), "tc_rxq");
    assert!(matches!(
        fields[0].ty(),
        windows_metadata::Type::ArrayFixed(outer, 4)
            if matches!(*outer, windows_metadata::Type::ArrayFixed(_, 8))
    ));
    assert_eq!(fields[1].name(), "count");

    let one_dimensional = index.expect("SimpleTest", "WithAnonArrayField");
    let fields: Vec<_> = one_dimensional.fields().collect();
    assert_eq!(fields[0].name(), "entries");
    assert!(matches!(
        fields[0].ty(),
        windows_metadata::Type::ArrayFixed(_, 4)
    ));
    assert_eq!(fields[1].name(), "count");
}

#[test]
fn generates_bitfield_storage() {
    let index = open_index();
    let bitfield = index.expect("SimpleTest", "WithBitfield");
    let fields: Vec<_> = bitfield.fields().collect();

    assert_eq!(
        fields.iter().map(|field| field.name()).collect::<Vec<_>>(),
        ["name", "_bitfield", "data"]
    );
    assert_eq!(
        fields[1].ty(),
        windows_metadata::Type::value_named("SimpleTest", "BitfieldKind")
    );
}

#[test]
fn generates_exact_partial_bitfield_storage() {
    let index = open_index();
    let bitfield = index.expect("SimpleTest", "PartialBitfield");
    let fields: Vec<_> = bitfield.fields().collect();

    assert_eq!(
        fields.iter().map(|field| field.name()).collect::<Vec<_>>(),
        ["_bitfield", "next"]
    );
    assert_eq!(
        fields[0].ty(),
        windows_metadata::Type::ArrayFixed(Box::new(windows_metadata::Type::U8), 3)
    );
}

#[test]
fn generates_bool_function_and_conditional_constant() {
    let index = open_index();
    let apis = index.expect("SimpleTest", "Apis");

    let visible = apis
        .methods()
        .find(|method| method.name() == "widget_is_visible")
        .expect("widget_is_visible method");
    assert_eq!(
        visible.signature(&[]).return_type,
        windows_metadata::Type::Bool
    );

    let max_depth = apis
        .fields()
        .find(|field| field.name() == "MAX_DEPTH")
        .expect("MAX_DEPTH constant");
    assert_eq!(
        max_depth.constant().expect("MAX_DEPTH value").value(),
        windows_metadata::Value::I32(42)
    );
}

#[test]
fn skips_unsupported_int128_typedefs() {
    let index = open_index();
    let types: Vec<_> = index.types().map(|ty| ty.name()).collect();

    for unsupported in ["__s128", "__u128", "s128", "u128"] {
        assert!(
            !types.contains(&unsupported),
            "{unsupported} should not be emitted"
        );
    }
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
