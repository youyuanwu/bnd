use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

const ROOT_HEADERS: &[&str] = &[
    "openssl/types.h",
    "openssl/crypto.h",
    "openssl/rand.h",
    "openssl/bn.h",
    "openssl/evp.h",
    "openssl/sha.h",
    "openssl/bio.h",
    "openssl/ssl.h",
    "openssl/tls1.h",
];

// `openssl/err.h` is intentionally excluded: its
// `lhash_st_ERR_STRING_DATA::dummy` inline union projects as a by-value
// `core::ffi::c_void`, which cannot derive Clone, Copy, or Default.

const EXTERNAL_REFERENCE_ROUTES: &[(&str, &str)] = &[
    ("FILE", "bnd_linux_clang::libc::file"),
    ("hostent", "bnd_linux_clang::libc::netdb"),
    ("off_t", "bnd_linux_clang::libc::types"),
    ("pthread_key_t", "bnd_linux_clang::libc::pthreadtypes"),
    ("pthread_once_t", "bnd_linux_clang::libc::pthreadtypes"),
    ("pthread_t", "bnd_linux_clang::libc::pthreadtypes"),
    ("ssize_t", "bnd_linux_clang::libc::types"),
    ("time_t", "bnd_linux_clang::libc::time_t"),
    ("timeval", "bnd_linux_clang::libc::struct_timeval"),
    ("tm", "bnd_linux_clang::libc::struct_tm"),
];

/// Generate the staged OpenSSL crate through one direct-Clang translation unit.
pub fn generate(output_dir: &Path) {
    let linux_winmd = linux_winmd();
    let temp = tempfile::tempdir_in(
        output_dir
            .parent()
            .expect("bnd-openssl-clang output must have a parent directory"),
    )
    .expect("failed to create temporary OpenSSL metadata directory");
    let generated_winmd = generate_metadata(temp.path(), &linux_winmd, ROOT_HEADERS);
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir)
        .expect("failed to create bnd-openssl-clang WinMD directory");
    let winmd = winmd_dir.join("bnd-openssl-clang.winmd");
    std::fs::copy(&generated_winmd, &winmd).expect("failed to save bnd-openssl-clang WinMD");

    let remapped_winmd = temp.path().join("bnd-openssl-clang.remapped.winmd");
    remap_metadata(
        &temp.path().join("metadata"),
        &generated_winmd,
        &remapped_winmd,
    );

    let external_types = external_libc_types(&generated_winmd, &linux_winmd);
    let external_routes = checked_external_routes(&external_types);
    let external_dependencies = external_libc_dependencies(&external_types, &linux_winmd);
    assert_metadata_contract(&generated_winmd, &remapped_winmd, &external_types);

    let manifest_path = output_dir.join("Cargo.toml");
    let manifest =
        std::fs::read(&manifest_path).expect("failed to preserve bnd-openssl-clang Cargo.toml");
    let generation = std::panic::catch_unwind(|| {
        let mut bindgen = staged_bindgen::Bindgen::new();
        bindgen
            .inputs([&remapped_winmd, &linux_winmd])
            .output(output_dir)
            .filter("openssl")
            .filter("!libc")
            .sys()
            .package()
            .package_feature_root("openssl");
        for (type_name, rust_path) in &external_routes {
            bindgen.external_reference(&format!("libc.{type_name}"), rust_path);
        }
        // Excluded libc definitions still participate in bindgen's dependency closure.
        // Mark their transitive dependencies external; only the direct routes above are emitted.
        for type_name in external_dependencies.difference(&external_types) {
            bindgen.external_reference(&format!("libc.{type_name}"), "bnd_linux_clang::libc");
        }
        bindgen.write();
    });
    if let Err(payload) = generation {
        std::fs::write(manifest_path, manifest)
            .expect("failed to restore bnd-openssl-clang Cargo.toml");
        std::panic::resume_unwind(payload);
    }
}

fn linux_winmd() -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../bnd-linux-clang/winmd/bnd-linux-clang.winmd");
    assert!(
        path.exists(),
        "bnd-linux-clang WinMD not found at {}\nHint: run `cargo run -p bnd-linux-gen` first",
        path.display()
    );
    path
}

fn generate_metadata(output_dir: &Path, linux_winmd: &Path, headers: &[&str]) -> PathBuf {
    let rdl_dir = output_dir.join("metadata");
    clear_rdl_dir(&rdl_dir);
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create OpenSSL WinMD directory");
    let openssl_winmd = winmd_dir.join("bnd-openssl-clang.winmd");
    let source = headers
        .iter()
        .map(|header| format!("#include <{header}>\n"))
        .collect::<String>();

    windows_clang::clang()
        .input_text(&source)
        .reference(linux_winmd)
        .args(["-x", "c", "-std=gnu11"])
        .namespace("openssl")
        .library("crypto")
        .header_libraries([("openssl/ssl.h", "ssl"), ("openssl/tls1.h", "ssl")])
        .scope_headers(headers.iter().copied())
        .output(&rdl_dir)
        .write_by_header()
        .expect("bnd-clang failed to generate OpenSSL RDL partitions");

    windows_rdl::reader()
        .input(&rdl_dir)
        .reference(linux_winmd)
        .reference_default()
        .output(&openssl_winmd)
        .write()
        .expect("windows-rdl failed to compile OpenSSL metadata");
    openssl_winmd
}

fn remap_metadata(rdl_dir: &Path, input: &Path, output: &Path) {
    let mut rdl_files: Vec<_> = std::fs::read_dir(rdl_dir)
        .expect("failed to read OpenSSL RDL directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rdl"))
        .collect();
    rdl_files.sort();

    let mut routes = HashMap::new();
    for path in rdl_files {
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("OpenSSL RDL file has no UTF-8 stem");
        let stem = module_stem(stem);
        for name in windows_rdl::item_names(&path, "openssl")
            .expect("failed to read OpenSSL RDL item names")
        {
            routes.insert(name, format!("openssl.{stem}"));
        }
    }

    windows_metadata::remap()
        .source("openssl")
        .fallback("openssl")
        .routes(routes)
        .input(input)
        .output(output)
        .remap()
        .expect("failed to remap OpenSSL metadata");
}

fn external_libc_types(input: &Path, linux_winmd: &Path) -> BTreeSet<String> {
    let openssl = windows_metadata::reader::File::new(
        std::fs::read(input).expect("failed to read generated OpenSSL WinMD"),
    )
    .expect("failed to parse generated OpenSSL WinMD");
    let linux = windows_metadata::reader::File::new(
        std::fs::read(linux_winmd).expect("failed to read bnd-linux-clang WinMD"),
    )
    .expect("failed to parse bnd-linux-clang WinMD");
    let index = windows_metadata::reader::Index::new(vec![openssl, linux]);
    let mut result = BTreeSet::new();
    for ty in index.types().filter(|ty| ty.namespace() == "openssl") {
        for field in ty.fields() {
            collect_libc_types(&field.ty(), &mut result);
        }
        for method in ty.methods() {
            let signature = method.signature(&[]);
            collect_libc_types(&signature.return_type, &mut result);
            for ty in signature.types {
                collect_libc_types(&ty, &mut result);
            }
        }
    }
    result
}

fn collect_libc_types(ty: &windows_metadata::Type, result: &mut BTreeSet<String>) {
    match ty {
        windows_metadata::Type::ClassName(name) | windows_metadata::Type::ValueName(name) => {
            if name.namespace == "libc" {
                result.insert(name.name.clone());
            }
            for generic in &name.generics {
                collect_libc_types(generic, result);
            }
        }
        windows_metadata::Type::Array(inner)
        | windows_metadata::Type::RefMut(inner)
        | windows_metadata::Type::RefConst(inner)
        | windows_metadata::Type::PtrMut(inner, _)
        | windows_metadata::Type::PtrConst(inner, _)
        | windows_metadata::Type::ArrayFixed(inner, _) => collect_libc_types(inner, result),
        _ => {}
    }
}

fn checked_external_routes(types: &BTreeSet<String>) -> BTreeMap<String, String> {
    let available: BTreeMap<_, _> = EXTERNAL_REFERENCE_ROUTES.iter().copied().collect();
    let missing: Vec<_> = types
        .iter()
        .filter(|name| !available.contains_key(name.as_str()))
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "missing bnd-linux-clang Rust routes for external libc types: {missing:?}"
    );
    let unused: Vec<_> = available
        .keys()
        .filter(|name| !types.contains(**name))
        .copied()
        .collect();
    assert!(
        unused.is_empty(),
        "unused bnd-linux-clang Rust routes do not match OpenSSL metadata: {unused:?}"
    );
    types
        .iter()
        .map(|name| {
            (
                name.clone(),
                available
                    .get(name.as_str())
                    .expect("external route checked above")
                    .to_string(),
            )
        })
        .collect()
}

fn external_libc_dependencies(types: &BTreeSet<String>, linux_winmd: &Path) -> BTreeSet<String> {
    let linux = open_index(linux_winmd);
    let mut dependencies = types.clone();
    let mut pending: Vec<_> = types.iter().cloned().collect();

    while let Some(name) = pending.pop() {
        let ty = linux.expect("libc", &name);
        let mut referenced = BTreeSet::new();
        for field in ty.fields() {
            collect_libc_types(&field.ty(), &mut referenced);
        }
        for method in ty.methods() {
            let signature = method.signature(&[]);
            collect_libc_types(&signature.return_type, &mut referenced);
            for ty in signature.types {
                collect_libc_types(&ty, &mut referenced);
            }
        }
        for name in referenced {
            if dependencies.insert(name.clone()) {
                pending.push(name);
            }
        }
    }

    dependencies
}

fn assert_metadata_contract(canonical: &Path, remapped: &Path, external_types: &BTreeSet<String>) {
    let canonical = open_index(canonical);
    let namespaces: BTreeSet<_> = canonical
        .types()
        .map(|ty| ty.namespace().to_string())
        .collect();
    assert_eq!(
        namespaces,
        ["openssl".to_string()].into_iter().collect(),
        "canonical OpenSSL metadata must contain one flat namespace"
    );
    assert!(
        !external_types.is_empty(),
        "OpenSSL metadata should retain external libc TypeRefs"
    );
    assert!(
        canonical.types().all(|ty| ty.namespace() != "libc"),
        "OpenSSL metadata must not define libc types locally"
    );
    let expected_external_types: BTreeSet<_> = EXTERNAL_REFERENCE_ROUTES
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect();
    assert_eq!(
        external_types, &expected_external_types,
        "OpenSSL metadata has an unexpected external libc TypeRef surface"
    );
    let local_type_names: BTreeSet<_> = canonical.types().map(|ty| ty.name().to_string()).collect();
    let locally_defined_external_types: Vec<_> = external_types
        .iter()
        .filter(|name| local_type_names.contains(*name))
        .collect();
    assert!(
        locally_defined_external_types.is_empty(),
        "OpenSSL metadata defines external libc types locally: {locally_defined_external_types:?}"
    );

    let remapped = open_index(remapped);
    let mut method_counts = BTreeMap::<String, usize>::new();
    for ty in remapped.types().filter(|ty| ty.name() == "Apis") {
        let namespace = ty.namespace();
        let expected_library = if matches!(namespace, "openssl.ssl" | "openssl.tls1") {
            "ssl"
        } else {
            "crypto"
        };
        for method in ty.methods() {
            let import = method
                .impl_map()
                .unwrap_or_else(|| panic!("{} has no native import", method.name()));
            assert_eq!(
                import.import_scope().name(),
                expected_library,
                "{} in {namespace} has the wrong native library",
                method.name()
            );
            *method_counts.entry(namespace.to_string()).or_default() += 1;
        }
    }
    for namespace in [
        "openssl.crypto",
        "openssl.rand",
        "openssl.bn",
        "openssl.evp",
        "openssl.sha",
        "openssl.bio",
        "openssl.ssl",
        "openssl.tls1",
    ] {
        assert!(
            method_counts.get(namespace).is_some_and(|count| *count > 0),
            "no native functions were generated for {namespace}; generated method counts: {method_counts:?}"
        );
    }
}

fn open_index(path: &Path) -> windows_metadata::reader::Index {
    let file = windows_metadata::reader::File::new(
        std::fs::read(path)
            .unwrap_or_else(|error| panic!("failed to read `{}`: {error}", path.display())),
    )
    .unwrap_or_else(|| panic!("failed to parse `{}`", path.display()));
    windows_metadata::reader::Index::new(vec![file])
}

fn module_stem(header_stem: &str) -> String {
    let mut stem: String = header_stem
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();
    if stem
        .as_bytes()
        .first()
        .is_some_and(|byte| byte.is_ascii_digit())
    {
        stem.insert(0, '_');
    }
    stem
}

fn clear_rdl_dir(rdl_dir: &Path) {
    std::fs::create_dir_all(rdl_dir).expect("failed to create OpenSSL RDL directory");
    for entry in std::fs::read_dir(rdl_dir).expect("failed to read OpenSSL RDL directory") {
        let path = entry.expect("failed to read OpenSSL RDL entry").path();
        if path.extension().is_some_and(|extension| extension == "rdl") {
            std::fs::remove_file(path).expect("failed to remove stale OpenSSL RDL partition");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_surface(
        index: &windows_metadata::reader::Index,
        namespace: &str,
        types: &[&str],
        constants: &[&str],
        methods: &[&str],
    ) {
        let namespace_types: BTreeSet<_> = index
            .types()
            .filter(|ty| ty.namespace() == namespace)
            .map(|ty| ty.name().to_string())
            .collect();
        for name in types {
            assert!(
                namespace_types.contains(*name),
                "{name} missing from {namespace}; types: {namespace_types:?}"
            );
        }

        if constants.is_empty() && methods.is_empty() {
            return;
        }

        let apis = index.expect(namespace, "Apis");
        let actual_constants: BTreeSet<_> = apis
            .fields()
            .map(|field| field.name().to_string())
            .collect();
        for name in constants {
            assert!(
                actual_constants.contains(*name),
                "{name} missing from {namespace}; constants: {actual_constants:?}"
            );
        }

        let actual_methods: BTreeSet<_> = apis
            .methods()
            .map(|method| method.name().to_string())
            .collect();
        for name in methods {
            assert!(
                actual_methods.contains(*name),
                "{name} missing from {namespace}; methods: {actual_methods:?}"
            );
        }
    }

    fn assert_i32_constant(
        index: &windows_metadata::reader::Index,
        namespace: &str,
        name: &str,
        expected: i32,
    ) {
        let field = index
            .expect(namespace, "Apis")
            .fields()
            .find(|field| field.name() == name)
            .unwrap_or_else(|| panic!("{name} missing from {namespace}"));
        assert_eq!(
            field.constant().expect("constant value").value(),
            windows_metadata::Value::I32(expected),
            "{namespace}.{name} has the wrong value"
        );
    }

    fn assert_opaque(index: &windows_metadata::reader::Index, name: &str) {
        let fields: Vec<_> = index
            .expect("openssl", name)
            .fields()
            .map(|field| (field.name().to_string(), field.ty()))
            .collect();
        assert_eq!(
            fields,
            [],
            "openssl.{name} is no longer represented as an opaque type"
        );
    }

    #[test]
    fn generates_active_flat_openssl_metadata() {
        let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../target");
        std::fs::create_dir_all(&target).expect("create target directory");
        let temp = tempfile::tempdir_in(target).expect("create OpenSSL test output");
        let linux_winmd = linux_winmd();
        let winmd = generate_metadata(temp.path(), &linux_winmd, ROOT_HEADERS);
        let remapped = temp.path().join("bnd-openssl-clang.remapped.winmd");
        remap_metadata(&temp.path().join("metadata"), &winmd, &remapped);
        let external_types = external_libc_types(&winmd, &linux_winmd);
        checked_external_routes(&external_types);
        assert_metadata_contract(&winmd, &remapped, &external_types);

        let index = open_index(&winmd);
        for name in [
            "bignum_st",
            "bio_st",
            "evp_md_st",
            "evp_md_ctx_st",
            "ssl_st",
            "ssl_ctx_st",
        ] {
            assert_opaque(&index, name);
        }

        let callback = index.expect("openssl", "pem_password_cb");
        assert!(
            format!("{:?}", callback.extends().expect("callback base type"))
                .contains("MulticastDelegate"),
            "pem_password_cb must remain a delegate"
        );
        let invoke = callback
            .methods()
            .find(|method| method.name() == "Invoke")
            .expect("pem_password_cb Invoke method");
        let signature = invoke.signature(&[]);
        assert_eq!(signature.return_type, windows_metadata::Type::I32);
        assert_eq!(
            signature.types,
            [
                windows_metadata::Type::PtrMut(Box::new(windows_metadata::Type::I8), 1),
                windows_metadata::Type::I32,
                windows_metadata::Type::I32,
                windows_metadata::Type::PtrMut(Box::new(windows_metadata::Type::Void), 1),
            ]
        );

        let crypto_thread_id = index.expect("openssl", "CRYPTO_THREADID");
        assert_eq!(
            crypto_thread_id
                .fields()
                .map(|field| (field.name().to_string(), field.ty()))
                .collect::<Vec<_>>(),
            [("dummy".to_string(), windows_metadata::Type::I32)]
        );
        let bio_msg = index.expect("openssl", "BIO_MSG");
        assert_eq!(
            bio_msg
                .fields()
                .map(|field| field.name().to_string())
                .collect::<Vec<_>>(),
            ["data", "data_len", "peer", "local", "flags"]
        );
        assert_eq!(
            bio_msg.fields().map(|field| field.ty()).collect::<Vec<_>>(),
            [
                windows_metadata::Type::PtrMut(Box::new(windows_metadata::Type::Void), 1),
                windows_metadata::Type::USize,
                windows_metadata::Type::PtrMut(
                    Box::new(windows_metadata::Type::value_named("openssl", "BIO_ADDR")),
                    1,
                ),
                windows_metadata::Type::PtrMut(
                    Box::new(windows_metadata::Type::value_named("openssl", "BIO_ADDR")),
                    1,
                ),
                windows_metadata::Type::U64,
            ]
        );
        let ssl_shutdown = index.expect("openssl", "SSL_SHUTDOWN_EX_ARGS");
        assert_eq!(
            ssl_shutdown
                .fields()
                .map(|field| (field.name().to_string(), field.ty()))
                .collect::<Vec<_>>(),
            [
                ("quic_error_code".to_string(), windows_metadata::Type::U64),
                (
                    "quic_reason".to_string(),
                    windows_metadata::Type::PtrConst(Box::new(windows_metadata::Type::I8), 1),
                ),
            ]
        );

        let remapped = open_index(&remapped);
        assert_surface(
            &remapped,
            "openssl.types",
            &[
                "BIO",
                "BIGNUM",
                "EVP_MD",
                "EVP_MD_CTX",
                "SSL",
                "SSL_CTX",
                "pem_password_cb",
            ],
            &[],
            &[],
        );
        for (namespace, types, constants, methods) in [
            (
                "openssl.crypto",
                &["CRYPTO_THREADID"][..],
                &["OPENSSL_VERSION", "OPENSSL_VERSION_STRING"][..],
                &["CRYPTO_free", "CRYPTO_malloc", "OpenSSL_version"][..],
            ),
            (
                "openssl.rand",
                &[][..],
                &["RAND_DRBG_STRENGTH"][..],
                &["RAND_bytes", "RAND_priv_bytes", "RAND_seed", "RAND_status"][..],
            ),
            (
                "openssl.bn",
                &[][..],
                &["BN_BYTES", "BN_FLG_CONSTTIME"][..],
                &["BN_free", "BN_get_word", "BN_new", "BN_set_word"][..],
            ),
            (
                "openssl.evp",
                &[][..],
                &["EVP_MAX_MD_SIZE"][..],
                &[
                    "EVP_DigestFinal_ex",
                    "EVP_DigestInit_ex",
                    "EVP_DigestUpdate",
                    "EVP_MD_CTX_free",
                    "EVP_MD_CTX_new",
                    "EVP_sha256",
                ][..],
            ),
            (
                "openssl.sha",
                &[][..],
                &["SHA_DIGEST_LENGTH", "SHA256_DIGEST_LENGTH"][..],
                &["SHA1", "SHA256"][..],
            ),
            (
                "openssl.bio",
                &["BIO_MSG", "BIO_POLL_DESCRIPTOR"][..],
                &["BIO_CLOSE", "BIO_NOCLOSE"][..],
                &[
                    "BIO_ctrl_pending",
                    "BIO_free",
                    "BIO_new",
                    "BIO_read",
                    "BIO_s_mem",
                    "BIO_write",
                ][..],
            ),
            (
                "openssl.ssl",
                &["SSL_METHOD", "SSL_POLL_ITEM", "SSL_SHUTDOWN_EX_ARGS"][..],
                &["SSL_ERROR_NONE", "SSL_ERROR_SSL", "SSL_VERIFY_NONE"][..],
                &[
                    "SSL_CTX_free",
                    "SSL_CTX_new",
                    "SSL_free",
                    "SSL_new",
                    "TLS_client_method",
                ][..],
            ),
        ] {
            assert_surface(&remapped, namespace, types, constants, methods);
        }
        assert_surface(
            &remapped,
            "openssl.tls1",
            &[],
            &["TLS1_2_VERSION_MAJOR", "TLS1_2_VERSION_MINOR"],
            &["SSL_get1_builtin_sigalgs"],
        );

        assert_i32_constant(&remapped, "openssl.crypto", "OPENSSL_VERSION", 0);
        assert_i32_constant(&remapped, "openssl.rand", "RAND_DRBG_STRENGTH", 256);
        assert_i32_constant(&remapped, "openssl.bn", "BN_BYTES", 8);
        assert_i32_constant(&remapped, "openssl.evp", "EVP_MAX_MD_SIZE", 64);
        assert_i32_constant(&remapped, "openssl.sha", "SHA256_DIGEST_LENGTH", 32);
        assert_i32_constant(&remapped, "openssl.bio", "BIO_CLOSE", 1);
        assert_i32_constant(&remapped, "openssl.ssl", "SSL_ERROR_SSL", 1);

        let canonical_apis = index.expect("openssl", "Apis");
        let gmtime = canonical_apis
            .methods()
            .find(|method| method.name() == "OPENSSL_gmtime")
            .expect("OPENSSL_gmtime method");
        assert_eq!(
            gmtime.signature(&[]).return_type,
            windows_metadata::Type::PtrMut(
                Box::new(windows_metadata::Type::value_named("libc", "tm")),
                1,
            )
        );
        let sendfile = canonical_apis
            .methods()
            .find(|method| method.name() == "SSL_sendfile")
            .expect("SSL_sendfile method");
        let signature = sendfile.signature(&[]);
        assert_eq!(
            signature.return_type,
            windows_metadata::Type::value_named("libc", "ssize_t")
        );
        assert_eq!(
            signature.types.get(2),
            Some(&windows_metadata::Type::value_named("libc", "off_t"))
        );
    }
}
