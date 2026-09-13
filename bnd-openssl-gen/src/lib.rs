use std::path::Path;

const EXTERNAL_NAMESPACE: &str = "__bnd_linux";

/// Generate the bnd-openssl source tree at `output_dir`.
///
/// 1. Runs bnd-winmd on `openssl.toml` to produce a `.winmd`.
/// 2. Runs `windows-bindgen --package` to emit `src/openssl/*/mod.rs`.
///    Passes both the OpenSSL and namespaced bnd-linux metadata so cross-winmd
///    types resolve, then maps those references to the external `bnd_linux`
///    crate.
/// 3. Saves the `.winmd` under `output_dir/winmd/`.
pub fn generate(output_dir: &Path) {
    let gen_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Step 1: Generate .winmd
    let winmd_dir = output_dir.join("winmd");
    std::fs::create_dir_all(&winmd_dir).expect("failed to create winmd directory");
    let openssl_winmd = winmd_dir.join("bnd-openssl.winmd");
    bnd_winmd::run(&gen_dir.join("openssl.toml"), Some(&openssl_winmd))
        .expect("bnd-winmd failed to generate winmd");

    // Step 2: Locate bnd-linux winmd (produced by bnd-linux-gen)
    let linux_winmd = gen_dir.join("../bnd-linux/winmd/bnd-linux.winmd");
    assert!(
        linux_winmd.exists(),
        "bnd-linux winmd not found at {}\n\
         Hint: run `cargo run -p bnd-linux-gen` first",
        linux_winmd.display()
    );

    // windows-bindgen 0.100 removed custom crate references. Generate with a
    // temporary, nested copy of the Linux metadata, then rewrite those paths
    // back to the external bnd-linux crate.
    let temp_dir = tempfile::tempdir().expect("failed to create temporary generator directory");
    let prefixed_linux_winmd = temp_dir.path().join("bnd-linux.winmd");
    let linux_config = std::fs::read_to_string(gen_dir.join("../bnd-linux-gen/bnd-linux.toml"))
        .expect("failed to read bnd-linux config");
    let prefixed_linux_config = linux_config.replace(
        "namespace = \"libc.",
        &format!("namespace = \"openssl.{EXTERNAL_NAMESPACE}.libc."),
    );
    assert_ne!(
        prefixed_linux_config, linux_config,
        "bnd-linux config contains no libc namespaces"
    );
    let prefixed_linux_config_path = temp_dir.path().join("bnd-linux.toml");
    std::fs::write(&prefixed_linux_config_path, prefixed_linux_config)
        .expect("failed to write temporary bnd-linux config");
    bnd_winmd::run(&prefixed_linux_config_path, Some(&prefixed_linux_winmd))
        .expect("failed to generate prefixed bnd-linux winmd");

    let prefixed_openssl_winmd = temp_dir.path().join("bnd-openssl.winmd");
    let openssl_config = std::fs::read_to_string(gen_dir.join("openssl.toml"))
        .expect("failed to read OpenSSL config");
    let prefixed_openssl_config = openssl_config
        .replace(
            "winmd = \"../bnd-linux/winmd/bnd-linux.winmd\"",
            &format!("winmd = \"{}\"", prefixed_linux_winmd.display()),
        )
        .replace(
            "namespace = \"libc\"",
            &format!("namespace = \"openssl.{EXTERNAL_NAMESPACE}.libc\""),
        );
    assert_ne!(
        prefixed_openssl_config, openssl_config,
        "OpenSSL config contains no bnd-linux type import"
    );
    let prefixed_openssl_config_path = temp_dir.path().join("openssl.toml");
    std::fs::write(&prefixed_openssl_config_path, prefixed_openssl_config)
        .expect("failed to write temporary OpenSSL config");
    bnd_winmd::run(&prefixed_openssl_config_path, Some(&prefixed_openssl_winmd))
        .expect("failed to generate prefixed OpenSSL winmd");

    // Step 3: Generate crate source tree via windows-bindgen package mode.
    let manifest_path = output_dir.join("Cargo.toml");
    let manifest =
        std::fs::read(&manifest_path).expect("failed to preserve bnd-openssl Cargo.toml");
    let generation = std::panic::catch_unwind(|| {
        windows_bindgen::bindgen([
            "--in",
            prefixed_openssl_winmd.to_str().unwrap(),
            "--in",
            prefixed_linux_winmd.to_str().unwrap(),
            "--out",
            output_dir.to_str().unwrap(),
            "--filter",
            "openssl",
            "--sys",
            "--package",
        ]);
    });
    std::fs::write(manifest_path, manifest).expect("failed to restore bnd-openssl Cargo.toml");
    if let Err(payload) = generation {
        std::panic::resume_unwind(payload);
    }

    let generated_root = output_dir.join("src/openssl");
    rewrite_external_references(&generated_root);
    std::fs::remove_dir_all(generated_root.join(EXTERNAL_NAMESPACE))
        .expect("failed to remove temporary bnd-linux bindings");
}

fn rewrite_external_references(dir: &Path) {
    for entry in std::fs::read_dir(dir).expect("failed to read generated bindings directory") {
        let path = entry.expect("failed to read generated entry").path();
        if path
            .file_name()
            .is_some_and(|name| name == EXTERNAL_NAMESPACE)
        {
            continue;
        }
        if path.is_dir() {
            rewrite_external_references(&path);
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }

        let source = std::fs::read_to_string(&path).expect("failed to read generated Rust file");
        let source = source
            .replace(
                &format!(
                    "#[cfg(feature = \"{EXTERNAL_NAMESPACE}\")]\npub mod {EXTERNAL_NAMESPACE};\n"
                ),
                "",
            )
            .replace(
                &format!("super::{EXTERNAL_NAMESPACE}::libc::"),
                "bnd_linux::libc::",
            );
        let source = remove_external_feature_conditions(&source);
        assert!(
            !source.contains(EXTERNAL_NAMESPACE),
            "unhandled temporary namespace reference in {}",
            path.display()
        );
        std::fs::write(path, source).expect("failed to rewrite generated Rust file");
    }
}

fn remove_external_feature_conditions(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut lines = source.lines();

    while let Some(line) = lines.next() {
        if !line.starts_with("#[cfg(") {
            output.push_str(line);
            output.push('\n');
            continue;
        }

        let mut attribute = line.to_string();
        while !attribute.ends_with(")]") {
            attribute.push('\n');
            attribute.push_str(lines.next().expect("unterminated generated cfg attribute"));
        }

        let features = feature_names(&attribute);
        if !features
            .iter()
            .any(|feature| feature.starts_with(EXTERNAL_NAMESPACE))
        {
            output.push_str(&attribute);
            output.push('\n');
            continue;
        }

        if attribute.starts_with("#[cfg(any(") {
            continue;
        }
        assert!(
            attribute.starts_with("#[cfg(all(") || features.len() == 1,
            "unsupported generated cfg attribute: {attribute}"
        );

        let features: Vec<_> = features
            .into_iter()
            .filter(|feature| !feature.starts_with(EXTERNAL_NAMESPACE))
            .collect();
        match features.as_slice() {
            [] => {}
            [feature] => {
                output.push_str(&format!("#[cfg(feature = \"{feature}\")]\n"));
            }
            _ => {
                let conditions = features
                    .iter()
                    .map(|feature| format!("feature = \"{feature}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                output.push_str(&format!("#[cfg(all({conditions}))]\n"));
            }
        }
    }

    output
}

fn feature_names(attribute: &str) -> Vec<&str> {
    let mut names = Vec::new();
    let mut remaining = attribute;
    const PREFIX: &str = "feature = \"";

    while let Some(start) = remaining.find(PREFIX) {
        remaining = &remaining[start + PREFIX.len()..];
        let end = remaining
            .find('"')
            .expect("unterminated generated feature name");
        names.push(&remaining[..end]);
        remaining = &remaining[end + 1..];
    }

    names
}
