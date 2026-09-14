# Zlib Direct-Clang System Testing

## Goal

Validate the active header-to-Rust pipeline against the real zlib development
headers and `libz`, including typedef chains, callbacks, structs, constants,
function signatures, and LP64 layouts.

## Active architecture

Zlib uses two direct-Clang passes because public zlib declarations depend on
typedefs from `zconf.h`:

```text
/usr/include/zconf.h
    -> bnd-clang (`Zlib.Types`)
    -> types.rdl
    -> windows-rdl
    -> types.winmd
            |
            | reference
            v
/usr/include/zlib.h
    -> bnd-clang (`Zlib`)
    -> zlib.rdl

types.rdl + zlib.rdl
    -> windows-rdl
    -> zlib.winmd
    -> bnd-bindgen --flat --sys
    -> OUT_DIR/bindings.rs
    -> native tests against libz
```

The implementation is
[`tests/e2e-clang-zlib/build.rs`](../../../tests/e2e-clang-zlib/build.rs).
Metadata assertions are in
[`bnd-clang/tests/linux_zlib.rs`](../../../bnd-clang/tests/linux_zlib.rs),
and native assertions are in
[`tests/e2e-clang-zlib/src/lib.rs`](../../../tests/e2e-clang-zlib/src/lib.rs).

## Generator setup

The first pass emits the typedef-owning header:

```rust
windows_clang::clang()
    .input("/usr/include/zconf.h")
    .args(["-x", "c", "-std=c11", "-I/usr/include"])
    .filter("zconf.h")
    .namespace("Zlib.Types")
    .library("z")
    .output(&types_rdl)
    .write()
    .expect("generate zconf RDL");
```

After compiling that RDL to `types.winmd`, the second pass supplies it as an
external metadata reference:

```rust
windows_clang::clang()
    .input("/usr/include/zlib.h")
    .reference(&types_winmd)
    .args(["-x", "c", "-std=c11", "-I/usr/include"])
    .filter("zlib.h")
    .namespace("Zlib")
    .library("z")
    .output(&zlib_rdl)
    .write()
    .expect("generate zlib RDL");
```

The final metadata combines both RDL inputs:

```rust
windows_rdl::reader()
    .inputs([&types_rdl, &zlib_rdl])
    .reference_default()
    .output(&zlib_winmd)
    .write()
    .expect("compile zlib WinMD");
```

`bnd-bindgen` then generates flat sys bindings into `OUT_DIR`, and the build
script emits `cargo:rustc-link-lib=dylib=z`.

## Coverage

### Metadata

The direct metadata tests verify:

- ownership of zconf typedefs such as `Bytef`, `uInt`, and `uLong`;
- `z_stream_s` and `gz_header_s`;
- callback aliases including allocation and stream callbacks;
- representative compression, inflate/deflate, and checksum methods;
- representative constants used by the public API;
- exact `z_stream_s` field names and cross-namespace TypeRefs;
- the `z` import library and C calling convention.

### Native runtime and ABI

The E2E package verifies:

- zlib version discovery;
- `Z_OK`, `Z_STREAM_END`, `Z_DEFLATED`, and `MAX_WBITS`;
- known CRC-32 and Adler-32 values;
- compression and decompression round trips;
- a reasonable `compressBound` result;
- exact LP64 sizes and alignments for public zlib records;
- accessible `gzFile_s` fields.

These tests compile the generated Rust and call the installed native
library, so they cover metadata, projection, linking, and runtime behavior
together.

## Prerequisites

- A compatible libclang development package.
- zlib development headers and `libz` (for example, `zlib1g-dev` on
  Debian/Ubuntu).

Run:

```sh
cargo test -p bnd-clang --test linux_zlib
cargo test -p e2e-clang-zlib
```

## Design notes

- The reference is provided to the second Clang pass so zconf declarations
  remain owned by `Zlib.Types`.
- The final RDL reader receives both RDL files and the default metadata
  reference needed by the RDL toolchain.
- `uLong` follows the host Linux LP64 ABI and is therefore 64-bit on x86-64.
- Outputs are placed in `OUT_DIR`; this E2E does not check generated source
  into the repository.

## History

An earlier system test used the retired `bnd-winmd` TOML pipeline and a
package named `e2e-zlib`. That package, its zlib TOML configuration, and its
checked-in generated Rust were removed after the direct-Clang metadata and
runtime tests reached parity. The old commands and file layout are no longer
valid.
