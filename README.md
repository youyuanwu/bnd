# bnd

Generate Rust FFI bindings from C headers using [WinMD](https://ecma-international.org/publications-and-standards/standards/ecma-335/) (ECMA-335) as an intermediate representation.

```
C Headers ──→ libclang ──→ bnd-winmd ──→ .winmd ──→ windows-bindgen ──→ Rust FFI module
```

## Crates

| Crate | Purpose |
|---|---|
| [`bnd-winmd`](bnd-winmd/) | Core library + CLI: C header → `.winmd` extraction and emission |

## Example bindings

| Crate | Source | Description |
|---|---|---|
| [`bnd-posix`](bnd-posix/) | glibc system headers | 15 POSIX modules (fcntl, socket, pthread, signal, …) |
| [`bnd-openssl`](bnd-openssl/) | OpenSSL 3.x headers | 8 partitions across libssl + libcrypto |

## Prerequisites

- **libclang** — `apt install libclang-dev` (or equivalent)
- **just** — command runner for generation and CI checks
- **Rust nightly** — see `rust-toolchain.toml`

## Development

- Run `just` or `just generate` to regenerate all bindings.
- Run `just ci` to execute the same checks as GitHub Actions.

## License

MIT
