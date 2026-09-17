# bnd-linux — Direct-Clang POSIX and Linux Bindings

`bnd-linux` provides generated Rust FFI bindings for POSIX and Linux system
headers. Production generation uses `bnd-clang`, RDL, the
`windows-rdl` WinMD writer, and `bnd-bindgen` end to end.

## Production Pipeline

```text
one GNU C11 translation unit containing the supported system headers
                              |
                              v
                bnd-clang / windows-clang fork
                              |
                 RDL partitioned by defining header
                  under a flat `libc` namespace
                              |
                              v
                         windows-rdl
                              |
                     flat temporary WinMD
                              |
                 defining-header metadata remap
                  + external-scope RDL repair
                              |
                              v
             bnd-linux/winmd/bnd-linux.winmd
                (canonical namespaced metadata)
                              |
                              v
                         bnd-bindgen
                              |
                              v
                 bnd-linux/src/libc/<module>/
```

`bnd-linux-gen` parses all root headers together so Clang sees one coherent
glibc type graph. It emits RDL files by the header that defines each item,
then compiles those files into a flat temporary WinMD. The generator derives
item ownership from the RDL filenames, structurally remaps the metadata, and
round-trips it through RDL to restore external TypeRef scopes. The resulting
`bnd-linux/winmd/bnd-linux.winmd` is canonical and uses
`libc.<defining-header-module>` namespaces.

## Rust Module and Feature Ownership

Defining-header ownership determines the generated Rust module. Header
stems are sanitized into Rust/Cargo names:

| Defining header | Rust module | Cargo feature |
|---|---|---|
| `sys/epoll.h` | `bnd_linux::libc::epoll` | `epoll` |
| `bits/types/struct_tm.h` | `bnd_linux::libc::struct_tm` | `struct_tm` |
| `bits/pthreadtypes.h` | `bnd_linux::libc::pthreadtypes` | `pthreadtypes` |
| `sys/types.h` | `bnd_linux::libc::types` | `types` |
| `netinet/in.h` | `bnd_linux::libc::in_` | `in_` |

This is a flat Rust module layout under `libc`; there are no
`libc::posix` or `libc::linux` package layers. A type is imported from the
module for the header that defines it, not necessarily from the top-level
header that uses it. Examples include:

- `FILE` → `bnd_linux::libc::file::FILE`
- `tm` → `bnd_linux::libc::struct_tm::tm`
- `pthread_once_t` → `bnd_linux::libc::pthreadtypes::pthread_once_t`
- `off_t` and `ssize_t` → `bnd_linux::libc::types`

Each module is gated by a same-named generated Cargo feature.
`bnd-bindgen` also generates feature dependencies for cross-header type
references. All generated features are enabled by default; consumers may
set `default-features = false` and request only entry-point features.

## Header and Native Library Policy

The generator includes the supported POSIX, glibc, and Linux roots in one
GNU C11 translation unit. Declaration scope is restricted to the configured
root and defining headers, including required `bits/`, `asm/`, and
`asm-generic/` definitions.

Functions link to `libc` by default. Known ownership exceptions are applied
at generation time:

- `crypt` links to `libcrypt`.
- `inet_net_ntop`, `inet_net_pton`, and `inet_neta` link to `libresolv`.

Macro inclusion and symbol exclusions are caller policy in
`bnd-linux-gen`, not Linux-specific behavior embedded in the vendored
frontend.

## Generated Artifacts

The product crate checks in:

- `bnd-linux/src/libc/**` — generated Rust modules.
- `bnd-linux/Cargo.toml` — hand-written package metadata plus generated
  features below `# generated features`.
- `bnd-linux/winmd/bnd-linux.winmd` — canonical defining-header metadata.

The freshness test regenerates all three artifacts, compares them byte for
byte, and generates a second time to verify determinism.

## Regeneration

From the repository root:

```sh
just generate-linux
```

or:

```sh
cargo run -p bnd-linux-gen
```

`just generate` and `just generate-openssl` also generate Linux before
OpenSSL because the OpenSSL generator references the canonical Linux WinMD.

## OpenSSL Integration

`bnd-openssl-gen` supplies
`bnd-linux/winmd/bnd-linux.winmd` to both the Clang and RDL stages. This
keeps POSIX declarations as external `libc` TypeRefs in the canonical
OpenSSL metadata.

During Rust generation, one namespace-preserving `libc` reference maps those
TypeRefs to `bnd_linux::libc::<defining-header-module>`. `bnd-openssl`
therefore uses the same Rust `FILE`, `tm`, pthread, time, and offset types as
`bnd-linux`; it does not generate local copies.

```text
bnd-linux-gen
    |
    v
bnd-linux/winmd/bnd-linux.winmd
    |                         |
    v                         v
bnd-linux crate       bnd-openssl-gen
                              |
                              v
                      bnd-openssl crate
                      (depends on bnd-linux)
```

## Testing

The `tests/bnd-linux-tests` crate exercises the generated APIs against the
host libraries. Generator tests additionally cover metadata contracts,
native library routing, canonical WinMD freshness, generated source
freshness, manifest freshness, and idempotence.

## History

Earlier production versions used `bnd-winmd` and organized generated Rust
under `libc::posix::*` and `libc::linux::*`. A direct-Clang implementation
was first validated in a separate staging crate, then promoted into
`bnd-linux`; the staging crate was removed during the production cutover.

The standalone implementation and its fixture packages were later retired
after direct-Clang coverage became authoritative.
