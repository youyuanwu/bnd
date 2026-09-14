# bnd-openssl-clang

Staging crate for Rust FFI bindings to OpenSSL 3.x (`libssl` + `libcrypto`)
generated through the direct `bnd-clang` pipeline.

This crate is **not published and is not the production OpenSSL binding
crate**. Use [`bnd-openssl`](../bnd-openssl/) for the current production
bindings.

The staged bindings use [`bnd-linux-clang`](../bnd-linux-clang/) for
external POSIX types and expose the same eight default feature names as the
production crate: `types`, `crypto`, `rand`, `bn`, `evp`, `sha`, `bio`, and
`ssl`.

Generated `src/openssl/` modules and canonical WinMD metadata are checked in.
Run `cargo run -p bnd-openssl-gen` to refresh both the production and staged
OpenSSL crates.

## Prerequisites

- OpenSSL development headers and libraries (`libssl-dev` on Ubuntu)
- Generated `bnd-linux-clang` metadata and bindings
