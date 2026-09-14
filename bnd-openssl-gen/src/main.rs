use std::path::PathBuf;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let bnd_openssl_dir = workspace_dir.join("bnd-openssl");
    let bnd_openssl_clang_dir = workspace_dir.join("bnd-openssl-clang");

    bnd_openssl_gen::generate(&bnd_openssl_dir);
    bnd_openssl_gen::clang::generate(&bnd_openssl_clang_dir);

    println!(
        "Generated bnd-openssl crate at {}",
        bnd_openssl_dir.display()
    );
    println!(
        "Generated bnd-openssl-clang crate at {}",
        bnd_openssl_clang_dir.display()
    );
}
