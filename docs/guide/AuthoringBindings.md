# Authoring Bindings with the Direct-Clang Pipeline

This guide shows external users how to generate Rust FFI bindings from C
headers with `bnd-clang`, `windows-rdl`, and `bnd-bindgen`.

For adding a generator and product crate inside this repository, see
[ContributingBindings.md](ContributingBindings.md).

## Pipeline

```text
C headers -> bnd-clang -> RDL -> windows-rdl -> canonical WinMD
                                                  |
                                                  v
                                             bnd-bindgen
                                                  |
                                                  v
                                             Rust bindings
```

`bnd-clang` is the package name of this repository's maintained
`windows-clang` fork; its Rust library name is `windows_clang`.
`bnd-bindgen` similarly exposes the `windows_bindgen` library.

## Dependency sources

`bnd-clang` and `bnd-bindgen` are currently non-published fork packages.
External projects must use them from a local checkout/vendor directory or a
Git dependency pinned to a specific bnd commit. Do not substitute upstream
`windows-clang` or `windows-bindgen` without validating the Linux ABI,
bitfield, alignment, package-generation, and external-reference behavior
described in the fork `VENDORED.md` files.

A local-checkout setup can use:

```toml
[dependencies]
bnd-macros = { path = "../bnd/bnd-macros" }

[build-dependencies]
windows-clang = { package = "bnd-clang", path = "../bnd/bnd-clang" }
windows-rdl = { version = "0.100", default-features = false }
windows-bindgen = { package = "bnd-bindgen", path = "../bnd/bnd-bindgen" }
```

The path values are examples; adjust them for your checkout. If you use Git
dependencies instead, pin all bnd packages to the same commit.

## Minimal flat binding

The following `build.rs` parses `zstd.h`, writes RDL and WinMD to `OUT_DIR`,
then generates a single Rust file:

```rust
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let rdl = out.join("zstd.rdl");
    let winmd = out.join("zstd.winmd");
    let bindings = out.join("bindings.rs");

    windows_clang::clang()
        .input("/usr/include/zstd.h")
        .args(["-x", "c", "-std=c11"])
        .filter("zstd.h")
        .namespace("zstd")
        .library("zstd")
        .output(&rdl)
        .write()
        .expect("generate zstd RDL");

    windows_rdl::reader()
        .input(&rdl)
        .output(&winmd)
        .write()
        .expect("compile zstd WinMD");

    let mut bindgen = windows_bindgen::Bindgen::new();
    bindgen
        .input(&winmd)
        .output(&bindings)
        .filter("zstd")
        .flat()
        .sys()
        .write();

    println!("cargo:rustc-link-lib=dylib=zstd");
    println!("cargo:rerun-if-changed=/usr/include/zstd.h");
}
```

Load the generated file from the crate:

```rust
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
extern crate bnd_macros as windows_link;
```

The `windows_link` alias satisfies the link macro emitted by sys-mode
`bnd-bindgen`.

The active
[`e2e-clang-simple`](../../tests/e2e-clang-simple/build.rs) and
[`e2e-clang-zlib`](../../tests/e2e-clang-zlib/build.rs) packages are complete
examples of this pattern.

## Multiple header owners and metadata references

Use separate Clang passes when a header should reference types emitted by an
earlier pass:

```rust
windows_clang::clang()
    .input("types.h")
    .args(["-x", "c", "-std=c11"])
    .namespace("Example.Types")
    .library("example")
    .output("types.rdl")
    .write()
    .expect("generate types RDL");

windows_rdl::reader()
    .input("types.rdl")
    .output("types.winmd")
    .write()
    .expect("compile types WinMD");

windows_clang::clang()
    .input("api.h")
    .reference("types.winmd")
    .args(["-x", "c", "-std=c11"])
    .filter("api.h")
    .namespace("Example.Api")
    .library("example")
    .output("api.rdl")
    .write()
    .expect("generate API RDL");

windows_rdl::reader()
    .inputs(["types.rdl", "api.rdl"])
    .output("example.winmd")
    .write()
    .expect("compile combined WinMD");
```

Supply every external WinMD reference at both stages:

1. `windows_clang::clang().reference(...)` so the header frontend keeps the
   referenced declarations externally owned.
2. `windows_rdl::reader().reference(...)` so RDL compilation can resolve the
   emitted TypeRefs.

The active
[`e2e-clang-multi`](../../tests/e2e-clang-multi/build.rs) package demonstrates
the two-pass form. Production OpenSSL generation demonstrates references to
another product's canonical WinMD.

## Canonical WinMD and package mode

For a checked-in bindings product, follow the production convention:

1. Parse one coherent translation unit.
2. Emit RDL by defining header with `write_by_header()`.
3. Compile all RDL into one canonical WinMD whose types share one flat root
   namespace.
4. Derive defining-header ownership from the RDL files.
5. Remap a temporary copy of the metadata to header-owned namespaces.
6. Run `bnd-bindgen` in package mode against the remapped copy.

Package generation uses the builder APIs:

```rust
let mut bindgen = windows_bindgen::Bindgen::new();
bindgen
    .input("example.remapped.winmd")
    .output("path/to/product-crate")
    .filter("example")
    .sys()
    .package()
    .package_feature_root("example")
    .write();
```

The temporary remapped WinMD is only a code-generation input. The canonical
flat WinMD is the metadata contract to check in and use as an external
reference.

See
[`bnd-linux-gen/src/clang.rs`](../../bnd-linux-gen/src/clang.rs) for
defining-header package generation and
[`bnd-openssl-gen/src/clang.rs`](../../bnd-openssl-gen/src/clang.rs) for
external metadata and Rust-route handling.

## External Rust ownership routes

WinMD identifies a referenced type by metadata namespace and name, not by
Cargo crate. When generated Rust must use a type owned by another crate, add
an exact route before `write()`:

```rust
bindgen.external_reference(
    "libc.tm",
    "bnd_linux::libc::struct_tm",
);
```

Route all external types used by the generated surface and include the
referenced WinMD as a bindgen input. Exact routes are preferable when one
flat metadata namespace maps to several defining-header Rust modules.

## Builder policy

- Use `.args(...)` for language mode, target, include directories, and
  preprocessor definitions.
- Use `.filter(...)` for a simple source-path suffix filter.
- Use `.scope_headers(...)` and `write_by_header()` when package ownership
  follows defining headers.
- Use `.library(...)`, `.libraries(...)`, and `.header_libraries(...)` to
  preserve native library ownership.
- Use `.reference(...)` for metadata ownership; do not re-emit externally
  owned declarations locally.
- Keep the canonical WinMD namespace flat. Rust module ownership is a
  code-generation concern handled by remapping and package mode.

## Prerequisites and validation

- A compatible libclang development package.
- Development headers and shared libraries for the target C API.
- The Rust toolchain required by the selected bnd commit.

At minimum, validate:

- generated metadata signatures and native library mappings;
- Rust sizes, alignments, offsets, constants, and callbacks;
- real native calls through the generated bindings;
- deterministic regeneration when artifacts are checked in.
