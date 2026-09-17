use std::path::{Path, PathBuf};

fn collect_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files_recursive(dir, dir, &mut files);
    files.sort();
    files
}

fn collect_files_recursive(base: &Path, dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_files_recursive(base, &path, files);
        } else {
            files.push(path.strip_prefix(base).unwrap().to_path_buf());
        }
    }
}

#[test]
fn generated_artifacts_are_up_to_date() {
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let checked_in = workspace_dir.join("bnd-linux");
    let temp = tempfile::tempdir().unwrap();
    let manifest = std::fs::read_to_string(checked_in.join("Cargo.toml")).unwrap();
    std::fs::write(temp.path().join("Cargo.toml"), manifest).unwrap();

    bnd_linux_gen::generate(temp.path());

    let checked_in_src = checked_in.join("src/libc");
    let generated_src = temp.path().join("src/libc");
    let checked_in_files = collect_files(&checked_in_src);
    let generated_files = collect_files(&generated_src);
    assert_eq!(checked_in_files, generated_files);

    for path in &checked_in_files {
        let expected = std::fs::read_to_string(checked_in_src.join(path)).unwrap();
        let actual = std::fs::read_to_string(generated_src.join(path)).unwrap();
        assert_eq!(expected, actual, "{} is out of date", path.display());
    }

    let expected =
        std::fs::read(checked_in.join("winmd/bnd-linux.winmd")).expect("read checked-in WinMD");
    let actual =
        std::fs::read(temp.path().join("winmd/bnd-linux.winmd")).expect("read generated WinMD");
    assert_eq!(expected, actual, "bnd-linux.winmd is out of date");
    let metadata =
        windows_metadata::reader::File::new(actual.clone()).expect("parse generated Linux WinMD");
    let namespaces: std::collections::BTreeSet<_> =
        windows_metadata::reader::Index::new(vec![metadata])
            .types()
            .map(|ty| ty.namespace().to_string())
            .collect();
    assert!(
        namespaces
            .iter()
            .all(|namespace| namespace.starts_with("libc.")),
        "canonical Linux WinMD contains unexpected namespaces: {namespaces:?}"
    );
    assert!(
        namespaces.contains("libc.file")
            && namespaces.contains("libc.in_")
            && namespaces.contains("libc.types"),
        "canonical Linux WinMD is missing representative defining-header namespaces: {namespaces:?}"
    );

    let expected =
        std::fs::read_to_string(checked_in.join("Cargo.toml")).expect("read checked-in Cargo.toml");
    let actual =
        std::fs::read_to_string(temp.path().join("Cargo.toml")).expect("read generated Cargo.toml");
    assert_eq!(expected, actual, "bnd-linux Cargo.toml is out of date");

    let first_sources: Vec<_> = generated_files
        .iter()
        .map(|path| {
            (
                path.clone(),
                std::fs::read(generated_src.join(path)).unwrap(),
            )
        })
        .collect();
    let first_winmd = std::fs::read(temp.path().join("winmd/bnd-linux.winmd")).unwrap();
    let first_manifest = std::fs::read(temp.path().join("Cargo.toml")).unwrap();

    bnd_linux_gen::generate(temp.path());

    assert_eq!(
        generated_files,
        collect_files(&generated_src),
        "a second generation changed the Linux source file list"
    );
    let second_sources: Vec<_> = generated_files
        .iter()
        .map(|path| {
            (
                path.clone(),
                std::fs::read(generated_src.join(path)).unwrap(),
            )
        })
        .collect();
    assert_eq!(
        first_sources, second_sources,
        "a second generation changed Linux Rust sources"
    );
    assert_eq!(
        first_winmd,
        std::fs::read(temp.path().join("winmd/bnd-linux.winmd")).unwrap(),
        "a second generation changed the canonical Linux WinMD"
    );
    assert_eq!(
        first_manifest,
        std::fs::read(temp.path().join("Cargo.toml")).unwrap(),
        "a second generation changed the Linux Cargo.toml"
    );
}
