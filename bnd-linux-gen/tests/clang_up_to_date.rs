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
fn clang_generated_sources_are_up_to_date() {
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let checked_in = workspace_dir.join("bnd-linux-clang");
    let temp = tempfile::tempdir().unwrap();
    let manifest = std::fs::read_to_string(checked_in.join("Cargo.toml")).unwrap();
    std::fs::write(temp.path().join("Cargo.toml"), manifest).unwrap();

    bnd_linux_gen::clang::generate(temp.path());

    let checked_in_src = checked_in.join("src/libc");
    let generated_src = temp.path().join("src/libc");
    let checked_in_files = collect_files(&checked_in_src);
    let generated_files = collect_files(&generated_src);
    assert_eq!(checked_in_files, generated_files);

    for path in checked_in_files {
        let expected = std::fs::read_to_string(checked_in_src.join(&path)).unwrap();
        let actual = std::fs::read_to_string(generated_src.join(&path)).unwrap();
        assert_eq!(expected, actual, "{} is out of date", path.display());
    }

    let expected = std::fs::read(checked_in.join("winmd/bnd-linux-clang.winmd"))
        .expect("read checked-in WinMD");
    let actual = std::fs::read(temp.path().join("winmd/bnd-linux-clang.winmd"))
        .expect("read generated WinMD");
    assert_eq!(expected, actual, "bnd-linux-clang.winmd is out of date");

    let expected =
        std::fs::read_to_string(checked_in.join("Cargo.toml")).expect("read checked-in Cargo.toml");
    let actual =
        std::fs::read_to_string(temp.path().join("Cargo.toml")).expect("read generated Cargo.toml");
    assert_eq!(
        expected, actual,
        "bnd-linux-clang Cargo.toml is out of date"
    );
}
