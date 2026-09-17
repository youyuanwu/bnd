# bnd-bindgen

`bnd-bindgen` is bnd's maintained fork of
[`windows-bindgen`](https://crates.io/crates/windows-bindgen). It generates
Rust bindings from WinMD metadata and adds package-generation support used by
bnd's Linux and OpenSSL bindings.

The package keeps the upstream-compatible `windows_bindgen` Rust library
name.

## Usage

```toml
[build-dependencies]
windows-bindgen = { package = "bnd-bindgen", version = "0.0.8" }
```

```rust,no_run
windows_bindgen::Bindgen::new()
    .input("example.winmd")
    .output("src/bindings.rs")
    .filter("example")
    .flat()
    .sys()
    .write();
```

Package mode can generate namespace-partitioned source files and Cargo
features:

```rust,no_run
windows_bindgen::Bindgen::new()
    .input("example.winmd")
    .output("generated-package")
    .filter("example")
    .sys()
    .package()
    .package_feature_root("example")
    .write();
```

Use `reference` to route types owned by referenced WinMD metadata to an
existing Rust crate or module instead of generating duplicate local
definitions. `ReferenceStyle::Full` preserves the metadata namespace below
the configured Rust crate path. The equivalent command-line form is
`--reference rust-path,full,metadata-filter`.

Generated sys bindings require a compatible implementation of the
`windows_link` macros. Bnd product crates use
[`bnd-macros`](https://crates.io/crates/bnd-macros).

See [`VENDORED.md`](VENDORED.md) for the upstream revision, licensing, and
the maintained fork changes.

## License

MIT or Apache-2.0
