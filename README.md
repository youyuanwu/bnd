# bnd

Generate Rust FFI bindings from C headers using [WinMD](https://ecma-international.org/publications-and-standards/standards/ecma-335/) (ECMA-335) as an intermediate representation.

```
C headers ──→ bnd-clang ──→ RDL ──→ WinMD ──→ bnd-bindgen ──→ Rust FFI modules
```

## Crates

| Crate | Purpose |
|---|---|
| [`bnd-clang`](bnd-clang/) | Vendored direct-Clang C header → RDL frontend used by production generators |
| [`bnd-bindgen`](bnd-bindgen/) | Vendored WinMD → Rust generator used by production generators |
| [`bnd-linux`](bnd-linux/) | Generated POSIX and Linux system bindings |
| [`bnd-openssl`](bnd-openssl/) | Generated OpenSSL 3.x bindings with POSIX types owned by `bnd-linux` |
| [`bnd-winmd`](bnd-winmd/) | Standalone TOML-driven C header → WinMD library and CLI; retained with its fixture tests |

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
