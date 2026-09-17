# Design: OpenSSL References to bnd-linux Types

> **Status: Production.** `bnd-openssl` references external POSIX types from
> the canonical `bnd-linux` WinMD and projects them through one
> namespace-preserving reference to
> `bnd_linux::libc::<defining-header-module>` Rust paths.

## Problem

OpenSSL headers use POSIX types such as `FILE`, `tm`, `time_t`, `off_t`,
`ssize_t`, `timeval`, and pthread typedefs. Generating local copies in
`bnd-openssl` would create distinct Rust types for the same native ABI and
would prevent values from moving directly between Linux and OpenSSL APIs.

The production design keeps both metadata and Rust ownership external:

- `bnd-linux/winmd/bnd-linux.winmd` defines canonical
  `libc.<defining-header-module>` metadata.
- `bnd-openssl/winmd/bnd-openssl.winmd` contains external `libc` TypeRefs,
  not local libc TypeDefs.
- Generated OpenSSL Rust uses types from the `bnd-linux` crate.

## Generation Pipeline

```text
bnd-linux/winmd/bnd-linux.winmd
             | reference at Clang stage
             | reference at RDL stage
             v
OpenSSL headers -> bnd-clang -> defining-header RDL
                               -> flat temporary openssl WinMD
                               -> canonical defining-header remap
                               -> external-scope RDL repair
             + canonical Linux WinMD
                               -> bnd-bindgen
                               -> bnd-openssl/src/openssl/**
```

The Linux WinMD is supplied to both metadata stages:

1. `bnd-clang` resolves Linux definitions and leaves them externally owned.
2. `windows-rdl` resolves the emitted external TypeRefs while compiling the
   canonical OpenSSL WinMD.

Passing the Linux WinMD only to bindgen would be too late: the canonical
OpenSSL metadata itself must already contain valid external references.

## Namespace-wide Rust Ownership

WinMD identifies metadata namespace and type names, but it does not encode
the Cargo crate or Rust module that owns an external type. The local
`bnd-bindgen` fork therefore accepts caller-supplied external ownership
routes. Canonical metadata now carries defining-header ownership, so one
route covers the complete namespace:

```rust
bindgen.reference(
    "bnd_linux",
    windows_bindgen::ReferenceStyle::Full,
    "libc",
);
```

For example, `libc.file.FILE` becomes
`bnd_linux::libc::file::FILE`, while `libc.types.off_t` becomes
`bnd_linux::libc::types::off_t`. The namespace route also covers transitive
Linux metadata dependencies, so no per-type route table is required.

## Cargo Features

`bnd-openssl` depends on `bnd-linux` with default features disabled and only
the defining-header features required by routed types:

- `file`
- `netdb`
- `pthreadtypes`
- `struct_timeval`
- `struct_tm`
- `time_t`
- `types`

Generated OpenSSL feature dependencies remain separate and are derived from
the canonical package metadata.

## Validation

The OpenSSL freshness test verifies that:

- No local `src/libc` tree is generated.
- Generated Rust contains the expected namespace-preserving external paths.
- Rust sources, the canonical OpenSSL WinMD, and generated Cargo features
  match checked-in artifacts.
- A second generation is identical.

Metadata assertions also verify that canonical OpenSSL metadata contains no
local `libc` definitions and that native functions retain the correct
`crypto` or `ssl` library.

## Regeneration Order

The Linux canonical WinMD must exist before OpenSSL generation:

```sh
just generate-openssl
```

This runs `bnd-linux-gen` followed by `bnd-openssl-gen`.

## History

The earlier `bnd-winmd` production path used nested `libc.posix.*` metadata
and a namespace-wide `--reference` mapping. The initial direct-Clang cutover
temporarily changed canonical Linux metadata to flat `libc` and required
exact bindgen routes. The current design retains the direct-Clang flat scrape
but structurally remaps the canonical metadata to the current defining-header
module layout before applying one namespace-wide reference.
