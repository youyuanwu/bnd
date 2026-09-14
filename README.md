# bnd

Generate Rust FFI bindings from C headers using
[WinMD](https://ecma-international.org/publications-and-standards/standards/ecma-335/)
(ECMA-335) as an intermediate representation.

```
C headers -> bnd-clang -> RDL -> windows-rdl -> canonical WinMD
                                                     |
                           defining-header remap + bnd-bindgen package mode
                                                     |
                                                     v
                                             Rust FFI modules
```

Production generators parse one coherent translation unit, emit RDL by
defining header under one flat canonical WinMD namespace, then remap a
temporary metadata copy so `bnd-bindgen` package mode can generate
header-owned Rust modules and Cargo features. External WinMD references are
preserved through the Clang and RDL stages and routed to their owning Rust
crate during bindgen.

## Crates

| Crate | Purpose |
|---|---|
| [`bnd-clang`](bnd-clang/) | Vendored direct-Clang C header → RDL frontend used by production generators |
| [`bnd-bindgen`](bnd-bindgen/) | Vendored WinMD → Rust generator used by production generators |
| [`bnd-macros`](bnd-macros/) | Link macros used by generated sys bindings |
| [`bnd-linux-gen`](bnd-linux-gen/) | In-repo Linux generator: Clang/RDL, canonical WinMD, remapping, and package generation |
| [`bnd-linux`](bnd-linux/) | Generated POSIX and Linux system bindings |
| [`bnd-openssl-gen`](bnd-openssl-gen/) | In-repo OpenSSL generator with external `bnd-linux` metadata routes |
| [`bnd-openssl`](bnd-openssl/) | Generated OpenSSL 3.x bindings with POSIX types owned by `bnd-linux` |

## Example bindings

| Crate | Source | Description |
|---|---|---|
| [`bnd-linux`](bnd-linux/) | glibc and Linux system headers | Defining-header modules under the flat `bnd_linux::libc` root |
| [`bnd-openssl`](bnd-openssl/) | OpenSSL 3.x headers | Defining-header modules across libssl + libcrypto |

## Prerequisites

- **libclang** — `apt install libclang-dev` (or equivalent)
- **just** — command runner for generation and CI checks
- **Rust nightly** — see `rust-toolchain.toml`

## Development

- Run `just` or `just generate` to regenerate all bindings.
- Run `just ci` to execute the same checks as GitHub Actions.

## License

MIT
