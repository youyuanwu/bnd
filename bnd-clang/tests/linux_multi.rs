use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static MULTI_WINMD: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let temp = tempfile::tempdir().expect("create temporary output directory");
    let types_rdl = temp.path().join("types.rdl");
    let types_winmd = temp.path().join("types.winmd");
    let widget_rdl = temp.path().join("widget.rdl");
    let multi_winmd = temp.path().join("multi.winmd");

    windows_clang::clang()
        .input(fixture("multi/types.h"))
        .args(["-x", "c", "-std=c11"])
        .namespace("MultiTest.Types")
        .library("simple")
        .output(&types_rdl)
        .write()
        .expect("generate RDL from multi/types.h");

    let generated_rdl = std::fs::read_to_string(&types_rdl).expect("read generated types RDL");
    assert!(
        !generated_rdl.is_empty(),
        "generated types RDL should not be empty"
    );

    windows_rdl::reader()
        .input(&types_rdl)
        .output(&types_winmd)
        .write()
        .expect("compile generated types RDL");

    windows_clang::clang()
        .input(fixture("multi/widget.h"))
        .reference(&types_winmd)
        .args(["-x", "c", "-std=c11"])
        .filter("widget.h")
        .namespace("MultiTest.Widgets")
        .library("simple")
        .output(&widget_rdl)
        .write()
        .expect("generate RDL from multi/widget.h");

    let generated_rdl = std::fs::read_to_string(&widget_rdl).expect("read generated widget RDL");
    assert!(
        !generated_rdl.is_empty(),
        "generated widget RDL should not be empty"
    );

    windows_rdl::reader()
        .inputs([&types_rdl, &widget_rdl])
        .output(&multi_winmd)
        .write()
        .expect("compile combined multi RDL");

    std::fs::read(multi_winmd).expect("read generated multi WinMD")
});

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(path)
}

fn open_index() -> windows_metadata::reader::Index {
    let file =
        windows_metadata::reader::File::new(MULTI_WINMD.clone()).expect("parse generated WinMD");
    windows_metadata::reader::Index::new(vec![file])
}

#[test]
fn generates_basic_types() {
    let index = open_index();
    let types: Vec<(String, String)> = index
        .types()
        .map(|ty| (ty.namespace().to_string(), ty.name().to_string()))
        .collect();
    let has = |name: &str| {
        types
            .iter()
            .any(|(namespace, ty)| namespace == "MultiTest.Types" && ty == name)
    };

    assert!(has("Color"), "Color enum missing. Found: {types:?}");
    assert!(has("Rect"), "Rect struct missing. Found: {types:?}");
    assert!(
        has("CompareFunc"),
        "CompareFunc callback missing. Found: {types:?}"
    );
    assert!(has("Apis"), "Apis class missing. Found: {types:?}");
}

#[test]
fn generates_color_variants() {
    let index = open_index();
    let color = index.expect("MultiTest.Types", "Color");
    let fields: Vec<String> = color
        .fields()
        .map(|field| field.name().to_string())
        .collect();

    assert!(fields.contains(&"COLOR_RED".to_string()));
    assert!(fields.contains(&"COLOR_GREEN".to_string()));
    assert!(fields.contains(&"COLOR_BLUE".to_string()));
}

#[test]
fn generates_rect_fields() {
    let index = open_index();
    let rect = index.expect("MultiTest.Types", "Rect");
    let fields: Vec<String> = rect
        .fields()
        .map(|field| field.name().to_string())
        .collect();

    assert_eq!(fields, ["x", "y", "width", "height"]);
}

#[test]
fn generates_compare_callback() {
    let index = open_index();
    let callback = index.expect("MultiTest.Types", "CompareFunc");
    let methods: Vec<String> = callback
        .methods()
        .map(|method| method.name().to_string())
        .collect();

    assert!(methods.contains(&"Invoke".to_string()));
}

#[test]
fn generates_constants() {
    let index = open_index();
    let apis = index.expect("MultiTest.Types", "Apis");
    let fields: Vec<String> = apis
        .fields()
        .map(|field| field.name().to_string())
        .collect();

    assert!(fields.contains(&"MAX_WIDGETS".to_string()));
    assert!(fields.contains(&"DEFAULT_WIDTH".to_string()));
    assert!(fields.contains(&"DEFAULT_HEIGHT".to_string()));
}

#[test]
fn generates_widget_partition() {
    let index = open_index();
    let types: Vec<(String, String)> = index
        .types()
        .map(|ty| (ty.namespace().to_string(), ty.name().to_string()))
        .collect();
    let has = |namespace: &str, name: &str| {
        types.iter().any(|(actual_namespace, actual_name)| {
            actual_namespace == namespace && actual_name == name
        })
    };

    assert!(has("MultiTest.Widgets", "Widget"));
    assert!(has("MultiTest.Widgets", "Apis"));
    assert!(!has("MultiTest.Widgets", "Color"));
    assert!(!has("MultiTest.Widgets", "Rect"));
    assert!(!has("MultiTest.Widgets", "CompareFunc"));
}

#[test]
fn generates_widget_fields() {
    let index = open_index();
    let widget = index.expect("MultiTest.Widgets", "Widget");
    let fields: Vec<_> = widget.fields().collect();
    let names: Vec<_> = fields.iter().map(|field| field.name()).collect();

    assert_eq!(names, ["name", "values", "color"]);
    assert_eq!(
        fields[0].ty(),
        windows_metadata::Type::PtrConst(Box::new(windows_metadata::Type::I8), 1)
    );
    assert_eq!(
        fields[1].ty(),
        windows_metadata::Type::ArrayFixed(Box::new(windows_metadata::Type::I32), 4)
    );
    assert_eq!(
        fields[2].ty(),
        windows_metadata::Type::value_named("MultiTest.Types", "Color")
    );
}

#[test]
fn generates_widget_functions() {
    let index = open_index();
    let apis = index.expect("MultiTest.Widgets", "Apis");
    let methods: Vec<_> = apis.methods().collect();
    let names: Vec<_> = methods.iter().map(|method| method.name()).collect();

    assert!(names.contains(&"create_widget"));
    assert!(names.contains(&"destroy_widget"));
    assert!(names.contains(&"widget_count"));

    let create = methods
        .iter()
        .find(|method| method.name() == "create_widget")
        .expect("create_widget method");
    let signature = create.signature(&[]);
    assert_eq!(signature.return_type, windows_metadata::Type::I32);
    assert!(
        create
            .impl_map()
            .expect("create_widget import")
            .flags()
            .contains(windows_metadata::PInvokeAttributes::CallConvCdecl)
    );
    assert_eq!(
        signature.types,
        [
            windows_metadata::Type::PtrConst(Box::new(windows_metadata::Type::I8), 1),
            windows_metadata::Type::value_named("MultiTest.Types", "Rect"),
            windows_metadata::Type::PtrMut(
                Box::new(windows_metadata::Type::value_named(
                    "MultiTest.Widgets",
                    "Widget",
                )),
                1,
            ),
        ]
    );
}
