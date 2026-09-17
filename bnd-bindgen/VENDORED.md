# Vendored windows-bindgen

This crate contains an experimental fork of `windows-bindgen`.

| Field | Value |
|---|---|
| Upstream repository | <https://github.com/microsoft/windows-rs> |
| Upstream path | `crates/libs/bindgen` |
| Upstream revision | `ca99b307b4e3705da54be35396a33a5afad229fd` |
| Upstream version | `0.100.0` |
| Vendored directory | `vendored/windows-bindgen` |
| License | MIT OR Apache-2.0 |

The vendored directory started as an exact copy of the upstream crate at the
recorded revision, including its original manifest, README, and license
files. The upstream manifest is stored as `Cargo.toml.upstream` so Cargo does
not exclude the vendored source as a nested package. The bnd package manifest
lives one level above it and points its library target at the vendored source.

Local experiments should modify files under `vendored/windows-bindgen` so
the fork remains easy to compare with upstream.

## Local changes

- Crate-level documentation uses the bnd package README while the upstream
  README remains preserved in the vendored directory.
- Package feature dependencies can use an explicitly configured root
  namespace without flattening its Rust module hierarchy.
- Metadata filters can be routed to caller-provided external Rust crate or
  module prefixes with the repeatable `Bindgen::reference` and
  `Bindgen::external_reference` builder methods. Referenced types are excluded
  from local package output and Cargo feature gates; malformed and overlapping
  routes are rejected. Explicit external references always own their filter,
  while ordinary references allow explicitly selected local types to win.
- External references support `full`, `skip-root`, and `flat` namespace
  styles through the typed `ReferenceStyle` API and the compatible
  `--reference rust-path,style,filter` argument.
- Namespace path segments use Rust keyword escaping, allowing header-derived
  modules such as `libc::r#in` to be referenced from sibling modules.
- String constants in portable `libc` sys packages use inline
  NUL-terminated pointers instead of requiring `windows-sys` string wrappers.
