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

The vendored directory is an unmodified copy of the upstream crate at the
recorded revision, including its original manifest, README, and license
files. The bnd package manifest lives one level above it and points its
library target at the vendored source.

Local experiments should modify files under `vendored/windows-clang` so the
fork remains easy to compare with upstream.
