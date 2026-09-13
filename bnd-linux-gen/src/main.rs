use std::path::PathBuf;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let bnd_linux_dir = workspace_dir.join("bnd-linux");
    let bnd_linux_clang_dir = workspace_dir.join("bnd-linux-clang");

    bnd_linux_gen::generate(&bnd_linux_dir);
    bnd_linux_gen::clang::generate(&bnd_linux_clang_dir);

    println!("Generated bnd-linux crate at {}", bnd_linux_dir.display());
    println!(
        "Generated bnd-linux-clang crate at {}",
        bnd_linux_clang_dir.display()
    );
}
