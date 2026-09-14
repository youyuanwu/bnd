# Design: OpenSSL References to bnd-linux Types

> **Status: Production.** `bnd-openssl` references external POSIX types from
> the canonical `bnd-linux` WinMD and projects them to exact
> `bnd_linux::libc::<defining-header-module>` Rust paths.

## Problem

OpenSSL headers use POSIX types such as `FILE`, `tm`, `time_t`, `off_t`,
`ssize_t`, `timeval`, and pthread typedefs. Generating local copies in
`bnd-openssl` would create distinct Rust types for the same native ABI and
would prevent values from moving directly between Linux and OpenSSL APIs.

The production design keeps both metadata and Rust ownership external:

- `bnd-linux/winmd/bnd-linux.winmd` defines the canonical flat `libc`
  metadata.
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
                               -> flat openssl WinMD
                               -> temporary package remap
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

## Exact Rust Ownership Routes

WinMD identifies metadata namespace and type names, but it does not encode
the Cargo crate or Rust module that owns an external type. The local
`bnd-bindgen` fork therefore accepts caller-supplied external ownership
routes.

`bnd-openssl-gen` owns the concrete route table:

| Metadata type | Generated Rust owner |
|---|---|
| `libc.FILE` | `bnd_linux::libc::file` |
| `libc.hostent` | `bnd_linux::libc::netdb` |
| `libc.pthread_key_t`, `pthread_once_t`, `pthread_t` | `bnd_linux::libc::pthreadtypes` |
| `libc.timeval` | `bnd_linux::libc::struct_timeval` |
| `libc.tm` | `bnd_linux::libc::struct_tm` |
| `libc.time_t` | `bnd_linux::libc::time_t` |
| `libc.off_t`, `ssize_t` | `bnd_linux::libc::types` |

Routes are exact because the flat canonical `libc` namespace does not carry
Rust defining-header ownership. The generator derives the set of external
types actually used by OpenSSL, rejects missing routes, and marks transitive
Linux metadata dependencies as external so bindgen does not generate a
local `libc` tree.

The routing API is generic. Linux- and OpenSSL-specific names remain in
`bnd-openssl-gen`, not in the vendored bindgen implementation.

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
the temporary package metadata.

## Validation

The OpenSSL freshness test verifies that:

- No local `src/libc` tree is generated.
- Generated Rust contains every expected exact external route.
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

The earlier `bnd-winmd` production path used nested
`libc.posix.*` metadata and a namespace-wide `--reference` mapping. The
direct-Clang cutover changed canonical Linux metadata to flat `libc` and
made defining-header Rust ownership explicit through exact bindgen routes.

The old design remains relevant only as history for the retired standalone
implementation and its removed fixtures.
