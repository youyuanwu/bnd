use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const ROOT_HEADERS: &[&str] = &[
    "openssl/types.h",
    "openssl/crypto.h",
    "openssl/err.h",
    "openssl/rand.h",
    "openssl/bn.h",
    "openssl/evp.h",
    "openssl/sha.h",
    "openssl/bio.h",
    "openssl/ssl.h",
    "openssl/tls1.h",
];

/// Generate the bnd-openssl crate through one direct-Clang translation unit.
pub fn generate(output_dir: &Path) {
    let linux_winmd = linux_winmd();
    let temp = tempfile::tempdir_in(
        output_dir
            .parent()
            .expect("bnd-openssl output must have a parent directory"),
    )
    .expect("failed to create temporary OpenSSL metadata directory");
    let flat_winmd = generate_metadata(temp.path(), &linux_winmd, ROOT_HEADERS);
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create bnd-openssl WinMD directory");
    let winmd = winmd_dir.join("bnd-openssl.winmd");
    remap_metadata(
        &temp.path().join("metadata"),
        &flat_winmd,
        &winmd,
        &temp.path().join("remap"),
        &linux_winmd,
    );

    let external_types = external_libc_types(&winmd, &linux_winmd);
    assert_metadata_contract(&winmd, &external_types);

    let manifest_path = output_dir.join("Cargo.toml");
    let manifest =
        std::fs::read(&manifest_path).expect("failed to preserve bnd-openssl Cargo.toml");
    let generation = std::panic::catch_unwind(|| {
        let mut bindgen = staged_bindgen::Bindgen::new();
        bindgen
            .inputs([&winmd, &linux_winmd])
            .output(output_dir)
            .filter("openssl")
            .filter("!libc")
            .sys()
            .package()
            .package_feature_root("openssl")
            .reference("bnd_linux", staged_bindgen::ReferenceStyle::Full, "libc");
        bindgen.write();
    });
    if let Err(payload) = generation {
        std::fs::write(manifest_path, manifest).expect("failed to restore bnd-openssl Cargo.toml");
        std::panic::resume_unwind(payload);
    }
}

fn remap_metadata(
    rdl_dir: &Path,
    flat_winmd: &Path,
    output: &Path,
    scratch_dir: &Path,
    linux_winmd: &Path,
) {
    let mut remap = windows_clang::remap_by_header();
    remap
        .rdl_dir(rdl_dir)
        .input(flat_winmd)
        .output(output)
        .scratch_dir(scratch_dir)
        .source("openssl")
        .import("Windows::Win32")
        .reference(linux_winmd)
        .reference_default();
    for namespace in metadata_namespaces(linux_winmd, "libc") {
        remap.import(&namespace.replace('.', "::"));
    }
    remap
        .write()
        .expect("failed to remap canonical bnd-openssl metadata");
}

fn linux_winmd() -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../bnd-linux/winmd/bnd-linux.winmd");
    assert!(
        path.exists(),
        "bnd-linux WinMD not found at {}\nHint: run `cargo run -p bnd-linux-gen` first",
        path.display()
    );
    path
}

fn generate_metadata(output_dir: &Path, linux_winmd: &Path, headers: &[&str]) -> PathBuf {
    let rdl_dir = output_dir.join("metadata");
    clear_rdl_dir(&rdl_dir);
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create OpenSSL WinMD directory");
    let openssl_winmd = winmd_dir.join("bnd-openssl.winmd");
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

fn external_libc_types(input: &Path, linux_winmd: &Path) -> BTreeSet<String> {
    let openssl = windows_metadata::reader::File::new(
        std::fs::read(input).expect("failed to read generated OpenSSL WinMD"),
    )
    .expect("failed to parse generated OpenSSL WinMD");
    let linux = windows_metadata::reader::File::new(
        std::fs::read(linux_winmd).expect("failed to read bnd-linux WinMD"),
    )
    .expect("failed to parse bnd-linux WinMD");
    let index = windows_metadata::reader::Index::new(vec![openssl, linux]);
    let mut result = BTreeSet::new();
    for ty in index
        .types()
        .filter(|ty| ty.namespace().starts_with("openssl."))
    {
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

fn metadata_namespaces(input: &Path, root: &str) -> BTreeSet<String> {
    open_index(input)
        .types()
        .map(|ty| ty.namespace().to_string())
        .filter(|namespace| {
            namespace == root
                || namespace
                    .strip_prefix(root)
                    .is_some_and(|suffix| suffix.starts_with('.'))
        })
        .collect()
}

fn collect_libc_types(ty: &windows_metadata::Type, result: &mut BTreeSet<String>) {
    match ty {
        windows_metadata::Type::ClassName(name) | windows_metadata::Type::ValueName(name) => {
            if name.namespace.starts_with("libc.") {
                result.insert(format!("{}.{}", name.namespace, name.name));
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

fn assert_metadata_contract(canonical: &Path, external_types: &BTreeSet<String>) {
    let canonical = open_index(canonical);
    let namespaces: BTreeSet<_> = canonical
        .types()
        .map(|ty| ty.namespace().to_string())
        .collect();
    assert!(
        namespaces
            .iter()
            .all(|namespace| namespace.starts_with("openssl.")),
        "canonical OpenSSL metadata contains unexpected namespaces: {namespaces:?}"
    );
    assert!(
        !external_types.is_empty(),
        "OpenSSL metadata should retain external libc TypeRefs"
    );
    assert!(
        canonical
            .types()
            .all(|ty| !ty.namespace().starts_with("libc.")),
        "OpenSSL metadata must not define libc types locally"
    );
    let local_type_names: BTreeSet<_> = canonical.types().map(|ty| ty.name().to_string()).collect();
    let locally_defined_external_types: Vec<_> = external_types
        .iter()
        .filter_map(|name| name.rsplit_once('.').map(|(_, name)| name))
        .filter(|name| local_type_names.contains(*name))
        .collect();
    assert!(
        locally_defined_external_types.is_empty(),
        "OpenSSL metadata defines external libc types locally: {locally_defined_external_types:?}"
    );

    let mut method_counts = BTreeMap::<String, usize>::new();
    for ty in canonical.types().filter(|ty| ty.name() == "Apis") {
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

    fn assert_opaque(index: &windows_metadata::reader::Index, namespace: &str, name: &str) {
        let fields: Vec<_> = index
            .expect(namespace, name)
            .fields()
            .map(|field| (field.name().to_string(), field.ty()))
            .collect();
        assert_eq!(
            fields,
            [],
            "{namespace}.{name} is no longer represented as an opaque type"
        );
    }

    #[test]
    fn generates_active_partitioned_openssl_metadata() {
        let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../target");
        std::fs::create_dir_all(&target).expect("create target directory");
        let temp = tempfile::tempdir_in(target).expect("create OpenSSL test output");
        let linux_winmd = linux_winmd();
        let flat_winmd = generate_metadata(temp.path(), &linux_winmd, ROOT_HEADERS);
        let winmd = temp.path().join("bnd-openssl.winmd");
        remap_metadata(
            &temp.path().join("metadata"),
            &flat_winmd,
            &winmd,
            &temp.path().join("remap"),
            &linux_winmd,
        );
        let external_types = external_libc_types(&winmd, &linux_winmd);
        assert_metadata_contract(&winmd, &external_types);

        let index = open_index(&winmd);
        for name in [
            "bignum_st",
            "bio_st",
            "evp_md_st",
            "evp_md_ctx_st",
            "ssl_st",
            "ssl_ctx_st",
        ] {
            assert_opaque(&index, "openssl.types", name);
        }

        let callback = index.expect("openssl.types", "pem_password_cb");
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

        let crypto_thread_id = index.expect("openssl.crypto", "CRYPTO_THREADID");
        assert_eq!(
            crypto_thread_id
                .fields()
                .map(|field| (field.name().to_string(), field.ty()))
                .collect::<Vec<_>>(),
            [("dummy".to_string(), windows_metadata::Type::I32)]
        );
        let bio_msg = index.expect("openssl.bio", "BIO_MSG");
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
                    Box::new(windows_metadata::Type::value_named(
                        "openssl.bio",
                        "BIO_ADDR",
                    )),
                    1,
                ),
                windows_metadata::Type::PtrMut(
                    Box::new(windows_metadata::Type::value_named(
                        "openssl.bio",
                        "BIO_ADDR",
                    )),
                    1,
                ),
                windows_metadata::Type::U64,
            ]
        );
        let ssl_shutdown = index.expect("openssl.ssl", "SSL_SHUTDOWN_EX_ARGS");
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
        let error_strings = index.expect("openssl.err", "lhash_st_ERR_STRING_DATA");
        assert_eq!(
            error_strings
                .fields()
                .map(|field| (field.name().to_string(), field.ty()))
                .collect::<Vec<_>>(),
            [(
                "dummy".to_string(),
                windows_metadata::Type::value_named("openssl.err", "lhash_st_ERR_STRING_DATA_0",),
            )]
        );
        let error_strings_dummy = index.expect("openssl.err", "lhash_st_ERR_STRING_DATA_0");
        assert!(
            error_strings_dummy
                .flags()
                .contains(windows_metadata::TypeAttributes::ExplicitLayout)
        );
        assert_eq!(
            error_strings_dummy
                .fields()
                .map(|field| (field.name().to_string(), field.ty()))
                .collect::<Vec<_>>(),
            [
                (
                    "d1".to_string(),
                    windows_metadata::Type::PtrMut(Box::new(windows_metadata::Type::Void), 1,),
                ),
                ("d2".to_string(), windows_metadata::Type::U64),
                ("d3".to_string(), windows_metadata::Type::I32),
            ]
        );

        assert_surface(
            &index,
            "openssl.types",
            &[
                "BIO",
                "BIGNUM",
                "BN_CTX",
                "EVP_CIPHER",
                "EVP_CIPHER_CTX",
                "EVP_MD",
                "EVP_MD_CTX",
                "EVP_PKEY",
                "EVP_PKEY_CTX",
                "OSSL_LIB_CTX",
                "OSSL_PARAM",
                "SSL",
                "SSL_CTX",
                "X509",
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
                "openssl.err",
                &[
                    "ERR_STRING_DATA",
                    "err_state_st",
                    "lhash_st_ERR_STRING_DATA",
                    "lhash_st_ERR_STRING_DATA_0",
                ][..],
                &["ERR_LIB_NONE", "ERR_TXT_STRING"][..],
                &[
                    "ERR_clear_error",
                    "ERR_get_error",
                    "ERR_new",
                    "ERR_set_error",
                ][..],
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
            assert_surface(&index, namespace, types, constants, methods);
        }
        assert_surface(
            &index,
            "openssl.tls1",
            &[],
            &["TLS1_2_VERSION_MAJOR", "TLS1_2_VERSION_MINOR"],
            &["SSL_get1_builtin_sigalgs"],
        );

        assert_i32_constant(&index, "openssl.crypto", "OPENSSL_VERSION", 0);
        assert_i32_constant(&index, "openssl.rand", "RAND_DRBG_STRENGTH", 256);
        assert_i32_constant(&index, "openssl.bn", "BN_BYTES", 8);
        assert_i32_constant(&index, "openssl.evp", "EVP_MAX_MD_SIZE", 64);
        assert_i32_constant(&index, "openssl.sha", "SHA256_DIGEST_LENGTH", 32);
        assert_i32_constant(&index, "openssl.bio", "BIO_CLOSE", 1);
        assert_i32_constant(&index, "openssl.ssl", "SSL_ERROR_SSL", 1);

        let crypto_apis = index.expect("openssl.crypto", "Apis");
        let gmtime = crypto_apis
            .methods()
            .find(|method| method.name() == "OPENSSL_gmtime")
            .expect("OPENSSL_gmtime method");
        assert_eq!(
            gmtime.signature(&[]).return_type,
            windows_metadata::Type::PtrMut(
                Box::new(windows_metadata::Type::value_named("libc.struct_tm", "tm",)),
                1,
            )
        );
        let sendfile = index
            .expect("openssl.ssl", "Apis")
            .methods()
            .find(|method| method.name() == "SSL_sendfile")
            .expect("SSL_sendfile method");
        let signature = sendfile.signature(&[]);
        assert_eq!(
            signature.return_type,
            windows_metadata::Type::value_named("libc.types", "ssize_t")
        );
        assert_eq!(
            signature.types.get(2),
            Some(&windows_metadata::Type::value_named("libc.types", "off_t"))
        );
    }
}
