# bnd-openssl

Rust FFI bindings for OpenSSL 3.x (`libssl` + `libcrypto`), generated
through the production direct-Clang pipeline:

```text
OpenSSL headers -> bnd-clang -> defining-header RDL -> flat temporary WinMD
                -> defining-header remap -> canonical namespaced WinMD
                -> bnd-bindgen -> Rust
```

The checked-in canonical metadata is `winmd/bnd-openssl.winmd`, whose
`openssl.<header-module>` namespaces map directly to Rust modules and
same-named Cargo features such as `bio`, `crypto`, `ssl`, and `types`.
The default feature set remains `bio`, `bn`, `crypto`, `evp`, `rand`,
`sha`, `ssl`, and `types`; generated dependency features enable additional
header modules when required.

OpenSSL metadata references the canonical
`../bnd-linux/winmd/bnd-linux.winmd` at both the Clang and RDL stages.
External POSIX types are owned by `bnd-linux` and generate as exact paths
such as `bnd_linux::libc::file::FILE`,
`bnd_linux::libc::struct_tm::tm`, and
`bnd_linux::libc::types::off_t`. One namespace-preserving `libc` reference
routes all of them; they are not duplicated in this crate.

The checked-in bindings are generated and validated for
`x86_64-unknown-linux-gnu`. They are also exposed on
`aarch64-unknown-linux-gnu`, but that target has not been generated or
ABI-validated and is used at the consumer's risk. On other targets, the crate
compiles without exporting binding modules.

## Regenerating

Regenerate Linux first because OpenSSL consumes its canonical WinMD:

```sh
just generate-openssl
```

Equivalent commands from the repository root are:

```sh
cargo run -p bnd-linux-gen
cargo run -p bnd-openssl-gen
```

Generation refreshes `src/openssl/`, generated Cargo features, and
`winmd/bnd-openssl.winmd`. Do not edit generated files manually.

## Default API Modules

| Module | Library | APIs |
|---|---|---|
| `types` | — | ~130 opaque typedefs (`EVP_MD`, `SSL`, `BIO`, `BIGNUM`, …) |
| `crypto` | `libcrypto` | Version queries, `CRYPTO_malloc`/`CRYPTO_free` |
| `err` | `libcrypto` | Error queue inspection, creation, and formatting |
| `rand` | `libcrypto` | `RAND_bytes`, `RAND_status` |
| `bn` | `libcrypto` | `BN_new`, `BN_set_word`, `BN_bn2hex` |
| `evp` | `libcrypto` | `EVP_DigestInit_ex`, `EVP_sha256`, `EVP_MAX_MD_SIZE` |
| `sha` | `libcrypto` | `SHA1`, `SHA256`, digest length constants |
| `bio` | `libcrypto` | `BIO_new`, `BIO_read`, `BIO_write`, memory BIOs |
| `ssl` | `libssl` | `SSL_CTX_new`, `SSL_new`, `TLS_client_method`, `SSL_ERROR_*` |

The complete generated defining-header module set is `asn1`, `bio`, `bn`,
`buffer`, `comp`, `conf`, `conftypes`, `core`, `crypto`, `err`, `evp`,
`rand`, `rsa`, `sha`, `ssl`, `tls1`, `types`, and `x509`. Each module has a
same-named Cargo feature; the default feature dependency closure enables
the complete current generated surface.

## Prerequisites

- **`libssl-dev`** — `apt install libssl-dev`

## Example

```rust
use bnd_openssl::openssl::{evp, sha};

unsafe {
    let md = evp::EVP_sha256();
    assert!(!md.is_null());
    assert_eq!(sha::SHA256_DIGEST_LENGTH, 32);
}
```

All function bindings are `unsafe` — they call directly into the OpenSSL shared libraries.
