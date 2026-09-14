use std::path::{Path, PathBuf};

#[test]
fn unset_external_routes_preserve_local_package_behavior() {
    let output = package_directory("no-route");
    let (local, external) = synthetic_winmds("Synthetic.External");

    windows_bindgen::Bindgen::new()
        .input_byte_sets([local, external])
        .output(&output)
        .filter("Synthetic")
        .sys()
        .package()
        .package_feature_root("Synthetic")
        .write();

    let local = read(&output.join("src/Synthetic/Local/mod.rs"));
    let root = read(&output.join("src/Synthetic/mod.rs"));
    let manifest = read(&output.join("Cargo.toml"));

    assert!(local.contains("super::External::Shared"));
    assert!(local.contains("super::External::SharedDetail"));
    assert!(output.join("src/Synthetic/External/mod.rs").exists());
    assert!(root.contains("pub mod External"));
    assert!(manifest.contains("External = []"));
    assert!(manifest.contains("Local = [\"External\"]"));

    std::fs::remove_dir_all(output).expect("remove local package output");
}

#[test]
fn exact_route_does_not_own_independently_required_named_dependencies() {
    let output = package_directory("exact-dependency");
    let (local, external) = synthetic_winmds("Synthetic.External");

    windows_bindgen::Bindgen::new()
        .input_byte_sets([local, external])
        .output(&output)
        .filter("Synthetic")
        .sys()
        .package()
        .package_feature_root("Synthetic")
        .external_reference("Synthetic.External.Shared", "synthetic_dep::provided")
        .write();

    let local = read(&output.join("src/Synthetic/Local/mod.rs"));
    let external = read(&output.join("src/Synthetic/External/mod.rs"));
    let manifest = read(&output.join("Cargo.toml"));

    assert!(local.contains("synthetic_dep::provided::Shared"));
    assert!(local.contains("super::External::SharedDetail"));
    assert!(!external.contains("pub struct Shared {"));
    assert!(external.contains("pub struct SharedDetail {"));
    assert!(manifest.contains("External = []"));
    assert!(manifest.contains("Local = [\"External\"]"));

    std::fs::remove_dir_all(output).expect("remove exact-route package output");
}

#[test]
fn namespace_route_owns_matching_named_dependencies() {
    let output = package_directory("namespace-dependency");
    let (local, external) = synthetic_winmds("Synthetic.External");

    windows_bindgen::Bindgen::new()
        .input_byte_sets([local, external])
        .output(&output)
        .filter("Synthetic")
        .sys()
        .package()
        .package_feature_root("Synthetic")
        .external_reference("Synthetic.External", "synthetic_dep::provided")
        .write();

    let local = read(&output.join("src/Synthetic/Local/mod.rs"));
    let manifest = read(&output.join("Cargo.toml"));

    assert!(local.contains("synthetic_dep::provided::Shared"));
    assert!(local.contains("synthetic_dep::provided::SharedDetail"));
    assert!(!output.join("src/Synthetic/External/mod.rs").exists());
    assert!(!manifest.contains("External"));
    assert!(manifest.contains("Local = []"));

    std::fs::remove_dir_all(output).expect("remove namespace-route package output");
}

#[test]
fn overlapping_external_routes_are_rejected() {
    let output = package_directory("ambiguous");
    let (local, external) = synthetic_winmds("SyntheticExternal");
    let result = std::panic::catch_unwind(|| {
        windows_bindgen::Bindgen::new()
            .input_byte_sets([local, external])
            .output(&output)
            .filter("Synthetic")
            .sys()
            .package()
            .package_feature_root("Synthetic")
            .external_reference("SyntheticExternal", "first_dep")
            .external_reference("SyntheticExternal.Shared", "second_dep")
            .write();
    });

    let panic = result.expect_err("overlapping external routes should panic");
    let message = panic_message(panic);
    assert!(message.contains("ambiguous external reference routes for `SyntheticExternal.Shared`"));

    std::fs::remove_dir_all(output).expect("remove ambiguous package output");
}

fn synthetic_winmds(external_namespace: &str) -> (Vec<u8>, Vec<u8>) {
    use windows_metadata::{
        FieldAttributes, MethodAttributes, PInvokeAttributes, Signature, Type, TypeAttributes,
        writer::{File, TypeDefOrRef},
    };

    let mut external = File::new("synthetic-external");
    let value_type = external.TypeRef("System", "ValueType");
    external.TypeDef(
        external_namespace,
        "SharedDetail",
        TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public | TypeAttributes::SequentialLayout | TypeAttributes::Sealed,
    );
    external.Field("code", &Type::I32, FieldAttributes::Public);
    external.TypeDef(
        external_namespace,
        "Shared",
        TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public | TypeAttributes::SequentialLayout | TypeAttributes::Sealed,
    );
    external.Field(
        "detail",
        &Type::value_named(external_namespace, "SharedDetail"),
        FieldAttributes::Public,
    );
    let external = external.into_stream();

    let reference = windows_metadata::reader::Index::new(vec![
        windows_metadata::reader::File::new(external.clone())
            .expect("parse external synthetic WinMD"),
    ]);
    let mut local = File::new("synthetic-local");
    local.set_reference(reference);
    let object = local.TypeRef("System", "Object");
    local.TypeDef(
        "Synthetic.Local",
        "Apis",
        TypeDefOrRef::TypeRef(object),
        TypeAttributes::Public | TypeAttributes::Sealed,
    );
    let signature = Signature {
        flags: Default::default(),
        return_type: Type::Void,
        types: vec![
            Type::value_named(external_namespace, "Shared"),
            Type::value_named(external_namespace, "SharedDetail"),
        ],
    };
    let method = local.MethodDef(
        "consume",
        &signature,
        MethodAttributes::Public
            | MethodAttributes::HideBySig
            | MethodAttributes::Static
            | MethodAttributes::PInvokeImpl,
        Default::default(),
    );
    local.Param("value", 1, Default::default());
    local.Param("detail", 2, Default::default());
    local.ImplMap(
        method,
        PInvokeAttributes::NoMangle | PInvokeAttributes::CallConvCdecl,
        "consume",
        "synthetic",
    );
    (local.into_stream(), external)
}

fn package_directory(name: &str) -> PathBuf {
    let directory = test_directory(name);
    reset_directory(&directory);
    std::fs::write(
        directory.join("Cargo.toml"),
        "[package]\nname = \"synthetic-bindings\"\nversion = \"0.0.0\"\n\n[features]\n# generated features\n",
    )
    .expect("write package manifest");
    directory
}

fn test_directory(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/external-reference-tests")
        .join(format!("{name}-{}", std::process::id()))
}

fn reset_directory(directory: &Path) {
    if directory.exists() {
        std::fs::remove_dir_all(directory).expect("remove stale test directory");
    }
    std::fs::create_dir_all(directory).expect("create test directory");
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read `{}`: {error}", path.display()))
}

fn panic_message(panic: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = panic.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        (*message).to_string()
    } else {
        "non-string panic".to_string()
    }
}
