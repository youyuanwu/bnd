use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const GENERATED_FEATURES_MARKER: &str = "# generated features";

fn collect_files(dir: &Path) -> BTreeSet<PathBuf> {
    let mut files = BTreeSet::new();
    collect_files_recursive(dir, dir, &mut files);
    files
}

fn collect_files_recursive(base: &Path, dir: &Path, files: &mut BTreeSet<PathBuf>) {
    for entry in std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read `{}`: {error}", dir.display()))
    {
        let path = entry.expect("failed to read directory entry").path();
        if path.is_dir() {
            collect_files_recursive(base, &path, files);
        } else {
            files.insert(
                path.strip_prefix(base)
                    .expect("generated path must be below source root")
                    .to_path_buf(),
            );
        }
    }
}

fn manifest_prefix(manifest: &str) -> &str {
    let marker_end = manifest
        .find(GENERATED_FEATURES_MARKER)
        .map(|offset| offset + GENERATED_FEATURES_MARKER.len())
        .expect("staged Cargo.toml is missing `# generated features`");
    let line_end = manifest[marker_end..]
        .find('\n')
        .map_or(manifest.len(), |offset| marker_end + offset + 1);
    &manifest[..line_end]
}

fn first_difference(expected: &[u8], actual: &[u8]) -> String {
    match expected
        .iter()
        .zip(actual)
        .position(|(expected, actual)| expected != actual)
    {
        Some(offset) => format!(
            "first differing byte at offset {offset}: checked-in=0x{:02x}, generated=0x{:02x}",
            expected[offset], actual[offset]
        ),
        None => format!(
            "common prefix is identical, but lengths differ: checked-in={}, generated={}",
            expected.len(),
            actual.len()
        ),
    }
}

fn read_sources(root: &Path, files: &BTreeSet<PathBuf>) -> BTreeMap<PathBuf, Vec<u8>> {
    files
        .iter()
        .map(|path| {
            let bytes = std::fs::read(root.join(path))
                .unwrap_or_else(|error| panic!("failed to read `{}`: {error}", path.display()));
            (path.clone(), bytes)
        })
        .collect()
}

#[test]
fn clang_generated_artifacts_are_up_to_date() {
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let checked_in = workspace_dir.join("bnd-openssl-clang");
    let target = workspace_dir.join("target");
    std::fs::create_dir_all(&target).expect("create target directory");
    let temp = tempfile::Builder::new()
        .prefix("bnd-openssl-clang-freshness-")
        .tempdir_in(target)
        .expect("create temporary staged crate");

    let checked_in_manifest =
        std::fs::read_to_string(checked_in.join("Cargo.toml")).expect("read staged Cargo.toml");
    std::fs::write(
        temp.path().join("Cargo.toml"),
        manifest_prefix(&checked_in_manifest),
    )
    .expect("write staged manifest prefix");

    bnd_openssl_gen::clang::generate(temp.path());

    let checked_in_src = checked_in.join("src/openssl");
    let generated_src = temp.path().join("src/openssl");
    let checked_in_files = collect_files(&checked_in_src);
    let generated_files = collect_files(&generated_src);
    let missing: Vec<_> = checked_in_files
        .difference(&generated_files)
        .map(|path| path.display().to_string())
        .collect();
    let extra: Vec<_> = generated_files
        .difference(&checked_in_files)
        .map(|path| path.display().to_string())
        .collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "generated OpenSSL source file list differs\nmissing generated files:\n  {}\nextra generated files:\n  {}",
        if missing.is_empty() {
            "<none>".to_string()
        } else {
            missing.join("\n  ")
        },
        if extra.is_empty() {
            "<none>".to_string()
        } else {
            extra.join("\n  ")
        }
    );

    assert!(
        !temp.path().join("src/libc").exists(),
        "generation emitted a local libc source tree"
    );
    let generated_sources = read_sources(&generated_src, &generated_files);
    let generated_text = generated_sources
        .values()
        .map(|source| std::str::from_utf8(source).expect("generated Rust must be UTF-8"))
        .collect::<String>();
    assert!(
        !generated_text.contains("super::libc::") && !generated_text.contains("crate::libc::"),
        "generated OpenSSL bindings contain a local libc path"
    );
    for path in [
        "bnd_linux_clang::libc::file::FILE",
        "bnd_linux_clang::libc::netdb::hostent",
        "bnd_linux_clang::libc::pthreadtypes::pthread_key_t",
        "bnd_linux_clang::libc::pthreadtypes::pthread_once_t",
        "bnd_linux_clang::libc::pthreadtypes::pthread_t",
        "bnd_linux_clang::libc::struct_timeval::timeval",
        "bnd_linux_clang::libc::struct_tm::tm",
        "bnd_linux_clang::libc::time_t::time_t",
        "bnd_linux_clang::libc::types::off_t",
        "bnd_linux_clang::libc::types::ssize_t",
    ] {
        assert!(
            generated_text.contains(path),
            "generated OpenSSL bindings are missing external route `{path}`"
        );
    }

    let mut changed = Vec::new();
    for path in &checked_in_files {
        let expected = std::fs::read(checked_in_src.join(path)).unwrap_or_else(|error| {
            panic!("failed to read checked-in `{}`: {error}", path.display())
        });
        let actual = &generated_sources[path];
        if expected.as_slice() != actual.as_slice() {
            changed.push(format!(
                "{} ({})",
                path.display(),
                first_difference(&expected, actual)
            ));
        }
    }
    assert!(
        changed.is_empty(),
        "generated OpenSSL source files changed:\n  {}\nrun `cargo run -p bnd-openssl-gen` to regenerate",
        changed.join("\n  ")
    );

    let expected_winmd = std::fs::read(checked_in.join("winmd/bnd-openssl-clang.winmd"))
        .expect("read checked-in OpenSSL WinMD");
    let actual_winmd = std::fs::read(temp.path().join("winmd/bnd-openssl-clang.winmd"))
        .expect("read generated OpenSSL WinMD");
    assert!(
        expected_winmd == actual_winmd,
        "bnd-openssl-clang.winmd is out of date ({})",
        first_difference(&expected_winmd, &actual_winmd)
    );

    let generated_manifest =
        std::fs::read_to_string(temp.path().join("Cargo.toml")).expect("read generated Cargo.toml");
    assert!(
        checked_in_manifest == generated_manifest,
        "bnd-openssl-clang Cargo.toml is out of date ({})",
        first_difference(
            checked_in_manifest.as_bytes(),
            generated_manifest.as_bytes()
        )
    );

    bnd_openssl_gen::clang::generate(temp.path());

    let regenerated_files = collect_files(&generated_src);
    assert_eq!(
        generated_files, regenerated_files,
        "a second generation changed the OpenSSL source file list"
    );
    let regenerated_sources = read_sources(&generated_src, &regenerated_files);
    let regenerated_winmd = std::fs::read(temp.path().join("winmd/bnd-openssl-clang.winmd"))
        .expect("read regenerated OpenSSL WinMD");
    let regenerated_manifest =
        std::fs::read_to_string(temp.path().join("Cargo.toml")).expect("read regenerated manifest");
    assert_eq!(
        generated_sources, regenerated_sources,
        "a second generation changed OpenSSL Rust sources"
    );
    assert_eq!(
        actual_winmd, regenerated_winmd,
        "a second generation changed the canonical OpenSSL WinMD"
    );
    assert_eq!(
        generated_manifest, regenerated_manifest,
        "a second generation changed the staged Cargo.toml"
    );
}
