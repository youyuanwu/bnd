# bnd-macros

Link macros for bnd-generated FFI bindings.

## Overview

`bnd-macros` provides two declarative macros that declare `extern` FFI
functions. It replaces the `windows-link` crate for bnd product crates
(`bnd-linux`, `bnd-openssl`, etc.).

## Macros

### `link!` — with `#[link]` attribute

Emits `#[link(name = "...")]` so the linker automatically pulls in the
named shared library (e.g. `-lcrypto`).

```rust
bnd_macros::link!("crypto" "C" fn EVP_DigestInit(ctx: *mut u8, md: *const u8) -> i32);
```

Expands to:

```rust
#[link(name = "crypto")]
unsafe extern "C" {
    pub fn EVP_DigestInit(ctx: *mut u8, md: *const u8) -> i32;
}
```

### `link_raw!` — without `#[link]` attribute

Ignores the library name and emits a bare `unsafe extern` block. Use when
the consumer handles linking externally (e.g. via build script or system
defaults like libc).

```rust
bnd_macros::link_raw!("c" "C" fn getpid() -> i32);
```

Expands to:

```rust
unsafe extern "C" {
    pub fn getpid() -> i32;
}
```

## Usage with generated code

Generated code from `windows-bindgen` emits `windows_link::link!(...)`.
Add an `extern crate` alias in your crate root to redirect to `bnd-macros`:

```rust
extern crate bnd_macros as windows_link;

pub mod my_bindings;
```

All functions are `unsafe` to call — they invoke C code directly.

## License

MIT
