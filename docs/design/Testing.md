# End-to-End Testing

## Goal

Prove the active pipeline works end to end:

```text
C headers -> bnd-clang -> RDL -> windows-rdl -> WinMD
                                                   |
                                                   v
                                              bnd-bindgen
                                                   |
                                                   v
                                      Rust FFI -> native library
```

The direct-Clang E2Es generate RDL, WinMD, and Rust bindings into Cargo's
`OUT_DIR`; they do not modify checked-in source during a test build.

## Active E2E packages

```text
bnd-clang/tests/fixtures/
├── simple/
│   ├── simple.h
│   └── partial_bitfield.h
└── multi/
    ├── types.h
    └── widget.h

tests/
├── simple-impl/             native `libsimple` test implementation
├── e2e-clang-simple/        one Clang/RDL pass, flat Rust output
├── e2e-clang-multi/         referenced metadata across two passes
└── e2e-clang-zlib/          system zlib headers and real `libz`
```

Run:

```sh
cargo test -p bnd-clang
cargo test -p bnd-bindgen
cargo test -p e2e-clang-simple -p e2e-clang-multi -p e2e-clang-zlib
```

## Simple fixture

[`tests/e2e-clang-simple/build.rs`](../../tests/e2e-clang-simple/build.rs)
uses the maintained `windows_clang` library to parse the simple and partial
bitfield fixtures as C11, emits one `SimpleTest` RDL file, compiles it with
`windows-rdl`, and generates flat sys bindings with `bnd-bindgen`.

The simple and multi packages link `tests/simple-impl` as `libsimple`.
Together their runtime tests cover:

- constants and enum values;
- struct, union, anonymous-record, array, bitfield, and over-aligned layouts;
- callbacks and function-table pointers;
- successful and invalid native calls;
- pointer/string round trips;
- omission of WinMD-unrepresentable numeric typedefs.

Metadata tests in
[`bnd-clang/tests/linux_simple.rs`](../../bnd-clang/tests/linux_simple.rs)
validate the corresponding WinMD type and method shapes.

## Multi-pass fixture

[`tests/e2e-clang-multi/build.rs`](../../tests/e2e-clang-multi/build.rs)
generates `MultiTest.Types` first and compiles it to a reference WinMD. A
second Clang pass parses `widget.h` with that reference and emits
`MultiTest.Widgets`. Both RDL files are then compiled into the final WinMD.

This validates:

- source filtering with `.filter("widget.h")`;
- external ownership during the second Clang pass;
- cross-namespace TypeRefs;
- bindgen's namespaced Rust output;
- native calls whose signatures use types from the referenced pass.

Metadata coverage is in
[`bnd-clang/tests/linux_multi.rs`](../../bnd-clang/tests/linux_multi.rs).

## Zlib system test

[`tests/e2e-clang-zlib/build.rs`](../../tests/e2e-clang-zlib/build.rs)
generates zconf types first, references that metadata while parsing `zlib.h`,
then compiles the combined RDL and links the result against `libz`.

The runtime suite validates constants, version discovery, checksums,
compression/decompression, `compressBound`, and exact LP64 struct layouts.
Metadata coverage is in
[`bnd-clang/tests/linux_zlib.rs`](../../bnd-clang/tests/linux_zlib.rs).

See [systesting/Zlib.md](systesting/Zlib.md) for details.

## Production generator validation

The production products add a package-generation layer:

```text
one translation unit
    -> RDL by defining header
    -> one flat canonical WinMD namespace
    -> temporary defining-header metadata remap
    -> bnd-bindgen package mode
    -> checked-in Rust modules and Cargo features
```

`bnd-linux-gen` and `bnd-openssl-gen` test:

- metadata contracts and native library routing;
- generated Rust file lists and bytes;
- checked-in canonical WinMD bytes;
- generated manifest features;
- second-generation determinism.

OpenSSL additionally verifies that POSIX metadata remains externally owned
and that generated Rust uses exact
`bnd_linux::libc::<defining-header-module>` routes.

Run the production checks with:

```sh
cargo test -p bnd-linux-gen --lib
cargo test -p bnd-linux-gen --test clang_up_to_date
cargo test -p bnd-linux -p bnd-linux-tests --all-features
cargo test -p bnd-openssl-gen --lib
cargo test -p bnd-openssl-gen --test clang_up_to_date
cargo test -p bnd-openssl --all-features
```

## What the test layers prove

| Layer | Evidence |
|---|---|
| Clang frontend | Header scope, ABI-aware type mapping, constants, callbacks, records, arrays, bitfields |
| RDL and WinMD | Metadata compiles, references resolve, canonical namespace and library mappings are correct |
| Bindgen | Flat, namespaced, and package output compile; external Rust routes resolve |
| Native runtime | Generated declarations call the intended shared libraries with correct ABI behavior |
| Freshness | Checked-in Rust, WinMD, and manifest features match the generator |
| Determinism | Repeated generation produces identical artifacts |

## History

The repository previously had TOML-driven `bnd-winmd` E2Es named
`e2e-simple`, `e2e-multi`, and `e2e-zlib`. Those packages, their legacy
fixtures/configurations, and checked-in generated binding files were removed
after their meaningful coverage moved to the direct-Clang packages above.
They are historical test architecture, not current commands or examples.
