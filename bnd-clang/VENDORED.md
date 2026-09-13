# Vendored windows-clang

This crate contains an experimental fork of `windows-clang`.

| Field | Value |
|---|---|
| Upstream repository | <https://github.com/microsoft/windows-rs> |
| Upstream path | `crates/libs/clang` |
| Upstream revision | `ca99b307b4e3705da54be35396a33a5afad229fd` |
| Upstream version | `0.100.0` |
| Vendored directory | `vendored/windows-clang` |
| License | MIT OR Apache-2.0 |

The vendored directory started as an exact copy of the upstream crate at the
recorded revision, including its original manifest, README, and license
files. The bnd package manifest lives one level above it and points its
library target at the vendored source.

Local experiments should modify files under `vendored/windows-clang` so the
fork remains easy to compare with upstream.

## Local changes

- Runtime libclang loading is optional behind the bnd package's `runtime`
  feature. Workspace builds use the linked libclang selected by `clang-sys`,
  avoiding feature unification with the existing `clang` wrapper and keeping
  binding regeneration deterministic.
- Plain functions and callbacks parsed from C translation units are emitted
  with the C calling convention. This preserves their source ABI instead of
  falling back to the Windows platform-default convention in RDL.
- C `long` and `unsigned long` are mapped from the widths reported by Clang
  for the active host ABI.
- Typedefs that shadow Rust primitive names are omitted to prevent recursive
  generated aliases such as `pub type bool = bool`.
- Direct and chained `__int128` typedefs are omitted because WinMD has no
  128-bit integer representation.
- Partial bitfield allocation units use Clang field offsets to preserve their
  exact occupied byte span and the enclosing record's native alignment.
