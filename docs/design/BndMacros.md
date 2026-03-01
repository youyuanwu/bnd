# bnd-macros — Replace windows-link with bnd-owned link macro

## Goal

Create a `bnd-macros` crate that provides `link!` and `link_raw!` macros,
removing the `windows-link` dependency from all bnd product crates
(`bnd-linux`, `bnd-openssl`) and test crates.

## Motivation

- **Unnecessary dependency**: `windows-link` is a Windows-oriented crate.
  On non-Windows platforms, the `link!` macro simply expands to a bare
  `extern "C" { pub fn ...; }` block — there is no Windows raw-dylib
  logic to leverage.
- **Naming confusion**: a crate named `windows-link` in a Linux-focused
  bindings project is misleading to users.
- **Self-contained**: bnd should own its entire codegen stack. The macro
  is ~10 lines of code.

## Design

### bnd-macros crate

A minimal `no_std` proc-macro-free crate with two `macro_rules!` macros.
Both accept the same syntax as `windows_link::link!` but differ in
whether they emit `#[link]` attributes:

- **`link!`** — Emits `#[link(name = "...")]` so the linker pulls in the
  named shared library. Use when the consumer wants automatic dynamic
  linking (e.g. `-lcrypto`, `-lssl`).
- **`link_raw!`** — Ignores the library name and emits a bare `extern`
  block. Use when the consumer handles linking externally (e.g. via
  build script, system-wide linkage, or static linking) and does not
  want the compiler to inject `-l` flags.

Both macros emit `unsafe extern` to comply with Rust 2024 edition.
Functions inside `unsafe extern` blocks are implicitly `unsafe` — callers
must use `unsafe {}`, matching the original `windows-link` behavior.

```rust
// bnd-macros/src/lib.rs
#![no_std]

#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

#[macro_export]
macro_rules! link_raw {
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}
```

### Integration — `extern crate` alias

Generated code emits `windows_link::link!(...)`. Rather than
post-processing generated files, each product crate re-exports
`bnd_macros` under the `windows_link` name at the crate root:

```rust
// bnd-linux/src/lib.rs
extern crate bnd_macros as windows_link;
pub mod libc;
```

This makes `windows_link::link!` resolve to `bnd_macros::link!`
throughout all submodules without touching any generated code.
No generator changes are needed.

### Which macro to use

| Scenario | Macro | Why |
|---|---|---|
| Linking against system libc | `link_raw!` | libc is linked by default; no `-l` needed |
| Linking against libcrypto/libssl | `link!` | Need `#[link(name = "crypto")]` to emit `-lcrypto` |
| Static linking via build script | `link_raw!` | Build script handles linking; extra `-l` would conflict |
| Cross-compiling with sysroot | Either | Depends on toolchain setup |

The generated code uses `link!` by default (via the `windows_link` alias).

### Crates using bnd-macros

Each crate depends on `bnd-macros` (workspace dependency) and adds the
`extern crate` alias in its `lib.rs`:

- `bnd-linux` — `extern crate bnd_macros as windows_link;`
- `bnd-openssl` — `extern crate bnd_macros as windows_link;`
- `tests/e2e-simple` — `extern crate bnd_macros as windows_link;`
- `tests/e2e-multi` — `extern crate bnd_macros as windows_link;`
- `tests/e2e-zlib` — `extern crate bnd_macros as windows_link;`

## Status

Implemented. `windows-link` has been fully removed from the workspace.
