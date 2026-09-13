# Evaluation: windows-clang and RDL for bnd

## Status

> **Decision: retain `bnd-winmd` as the C-header frontend.**
>
> `windows-clang` cannot directly replace it for Linux and OpenSSL
> generation at the evaluated revision. The most promising reuse boundary is
> `windows-rdl`: bnd could continue extracting and normalizing Linux ABI data,
> serialize its intermediate model as RDL, and let `windows-rdl` produce the
> WinMD.

This evaluation uses the local `microsoft/windows-rs` checkout at commit
[`ca99b307b4e3705da54be35396a33a5afad229fd`](https://github.com/microsoft/windows-rs/commit/ca99b307b4e3705da54be35396a33a5afad229fd)
from September 11, 2026. The relevant crate is located at
`crates/libs/clang`; there is no `crates/tools/windows-clang` directory in
that revision.

## Question

Can bnd replace its custom C-header-to-WinMD generator with
[`windows-clang`](https://crates.io/crates/windows-clang), either directly
or through a small adapter?

The answer depends on more than whether the upstream tool can parse C
headers. bnd requires:

- Linux x86-64 LP64 type semantics.
- Exact glibc and OpenSSL ABI layouts.
- Ordered namespace partitions with independent library mappings.
- Header inclusion separated from declaration traversal.
- Constants, callbacks, anonymous records, unions, and typedefs.
- Type injection for declarations that cannot be represented directly.
- References from OpenSSL metadata to types owned by `bnd-linux`.
- Deterministic checked-in Rust output through `windows-bindgen`.

## Upstream Architecture

`windows-clang` is a published Rust library, not a standalone binding
generator executable. Its primary builder is
[`windows_clang::Clang`](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/lib.rs#L515-L890).
The complete upstream pipeline is:

```text
C/C++ headers
      |
      v
windows_clang::Clang
      |
      | formatted declarations
      v
RDL text
      |
      v
windows_rdl::reader()
      |
      v
WinMD
      |
      v
windows-bindgen
      |
      v
Rust bindings
```

RDL is a Rust-like declaration language, not the generator configuration
format. It is:

1. The output produced by `windows-clang`.
2. A hand-authorable metadata declaration format.
3. The intermediate input compiled to WinMD by `windows-rdl`.

The WebView generator demonstrates the intended end-to-end use:
[`crates/tools/webview/src/main.rs:24-62`](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/tools/webview/src/main.rs#L24-L62).

```rust
clang()
    .args([
        "-x",
        "c++",
        "--target=x86_64-pc-windows-msvc",
        "-fms-extensions",
    ])
    .input("WebView2.h")
    .reference_default()
    .namespace("WebView2")
    .library("WebView2Loader.dll")
    .output("WebView2.rdl")
    .write()?;

windows_rdl::reader()
    .input("WebView2.rdl")
    .reference_default()
    .output("WebView2.winmd")
    .write()?;
```

The declaration models, canonicalization logic, collector, and RDL emission
used inside `windows-clang` are private modules
([`lib.rs:24-50`](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/lib.rs#L24-L50)).
Consumers can configure the `Clang` builder but cannot replace individual
type-mapping or layout decisions without changing or forking the crate.

## Capability Comparison

Statuses below are relative to bnd's current requirements.

| Capability | `windows-clang` | bnd impact |
|---|---|---|
| Functions | Supported | Includes parameters, return types, libraries, and common calling conventions |
| Structs and unions | Supported | Supports sequential and explicit layouts |
| Enums and typedefs | Supported | Includes anonymous enums as loose constants |
| Object-like macros | Partial | Supports common literals and known forms, not arbitrary macro expressions |
| Callback typedefs | Partial | Non-variadic function pointers are supported; some inline or variadic forms become opaque |
| Fixed arrays | Supported | Recursively maps constant arrays |
| Multidimensional arrays | Supported, lightly tested | The implementation is recursive, but equivalent Linux fixtures are not present upstream |
| Anonymous records | Supported | Direct and recursively named nested records are handled |
| Packed records | Supported | Emits packing metadata |
| Over-aligned records | Supported | RDL carries alignment metadata consumed by `windows-bindgen` |
| Logical bitfield metadata | Supported | Emits backing fields plus bitfield descriptions |
| Partial-width bitfield storage | Missing for bnd | Does not preserve bnd's exact-byte layout workaround |
| Pointer constness | Partial | Mixed pointer chains are collapsed to the metadata model's available representation |
| SAL directionality | Supported for common forms | Richer than bnd, but designed around Windows annotations |
| External WinMD references | Supported | References must be supplied to both Clang and the RDL reader |
| Cross-crate Rust references | Missing | The existing OpenSSL post-generation rewrite remains necessary |
| Per-partition namespaces and libraries | Partial | Requires multiple invocations and external orchestration |
| Traverse-style source filtering | Partial | Filtering and reachability primitives exist, but not bnd's configuration model |
| Structured type injection | Missing | Extra declarations would need hand-authored or generated RDL |
| Custom type remapping | Missing from public API | Canonicalization is internal |
| Deterministic output | Mostly supported | Ordered collections are used, but callers should provide explicitly ordered inputs |
| Linux ABI semantics | Missing | Scalar canonicalization follows Windows LLP64 |

## Blocking Differences

### Linux `long` is mapped as a Windows type

The decisive compatibility problem is scalar canonicalization.
`windows-clang` maps C `long` and `unsigned long` to 32-bit values:

- [`canon.rs:725-740`](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/canon.rs#L725-L740)

That is correct for the Windows LLP64 ABI. It is incorrect for the Linux
x86-64 LP64 ABI, where both types are 64-bit. bnd handles this explicitly in
`bnd-winmd/src/extract.rs:1122-1146`.

This difference affects function signatures, typedefs, fields, array
elements, and layouts throughout glibc and OpenSSL. It makes direct adoption
unsafe even when libclang parses the headers successfully.

### Partial bitfields require bnd's layout normalization

`windows-clang` groups bitfields and preserves logical bitfield descriptions,
but its backing storage is based on the declared C storage type:

- [`struct.rs:139-183`](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/struct.rs#L139-L183)

bnd additionally uses Clang's bit offsets to preserve the exact occupied
byte range. This is required for layouts such as glibc's `_IO_FILE`, where a
24-bit run is followed immediately by another field. Representing that run
as a full `u32` shifts all following fields and changes the generated Rust
size. The current implementation is in
`bnd-winmd/src/extract.rs:617-767`, with its regression test at
`bnd-winmd/src/extract.rs:1443-1488`.

Upstream's logical bitfield metadata is useful for generated accessors, but
it does not replace bnd's ABI-preserving storage transformation.

### Configuration models differ

bnd's TOML model provides:

- Global include paths and Clang arguments.
- Ordered partitions.
- Per-partition namespace, library, headers, traverse files, and arguments.
- Namespace overrides.
- External WinMD type imports.
- Injected enums, typedefs, and opaque structs.

These are defined in `bnd-winmd/src/config.rs:7-59` and
`bnd-winmd/src/config.rs:115-167`.

`windows-clang` exposes useful primitives for inputs, filters, scopes,
symbols, namespaces, libraries, and references, but does not provide this
orchestration as a single abstraction. A compatibility wrapper would need to
translate every partition, merge its RDL output, preserve first-owner type
resolution, and then invoke `windows-rdl` with the correct references.

That is a new generator layer rather than a small command-line substitution.

### Cross-WinMD references do not solve cross-crate code generation

`windows-clang` and `windows-rdl` can resolve types from referenced WinMD
files. This could represent OpenSSL signatures that use `bnd-linux` types.

However, `windows-bindgen` 0.100 no longer provides bnd's former custom
crate-reference option. `bnd-openssl-gen/src/lib.rs:23-100` therefore
generates a temporary single-root namespace and rewrites resulting paths to
`bnd_linux::libc::*`.

Changing the header frontend does not remove that downstream limitation.

### Linux execution is not first-class upstream

Core parsing uses runtime-loaded libclang, so execution on Linux is
mechanically possible with a compatible library. The upstream provisioning
path is nevertheless Windows-specific: it retrieves Windows NuGet packages,
expects `libclang.dll`, and invokes Windows tool names.

- [`provision.rs:5-75`](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/provision.rs#L5-L75)

The evaluated revision pins libclang 22.1.8. Its Clang tests are run by
upstream on Windows hosts, so Linux use would require bnd to own libclang
discovery, versioning, and compatibility testing.

## Adoption Options

### Direct replacement

**Not viable.**

The Linux scalar mapping and partial-bitfield behavior can silently produce
incorrect FFI declarations. Configuration and external-reference behavior
also do not match bnd's generator interface.

### Compatibility wrapper around `windows-clang`

**Possible, but large.**

The wrapper would need to:

1. Run partitions independently with stable ordering.
2. Recreate header-versus-traverse semantics.
3. Add target-dependent LP64 canonicalization.
4. Correct partial bitfield layouts.
5. Merge injected declarations into generated RDL.
6. Propagate external WinMD references into both pipeline stages.
7. Preserve bnd's package-generation and external-crate rewrite.

Because the relevant upstream models and mapping modules are private, items
3 and 4 require either upstream API changes or a maintained fork.

### Selective `windows-rdl` adoption

**Technically credible.**

bnd can retain:

- Its TOML configuration.
- Clang extraction.
- Linux LP64 type mapping.
- Layout normalization.
- Injection and validation.
- Type ownership and external import rules.

Instead of calling `windows-metadata` directly in
`bnd-winmd/src/emit.rs`, bnd could serialize its normalized model to RDL and
compile that RDL with `windows_rdl::reader()`.

This would reuse upstream metadata encoding, RDL validation, alignment
attributes, and future schema work without adopting the Windows-specific
header canonicalizer. It adds a textual intermediate stage, so it should
only be adopted if reducing direct WinMD writer maintenance outweighs that
extra layer.

### Retain `bnd-winmd`

**Recommended.**

The current implementation already encodes the behavior required by the
generated Linux and OpenSSL crates:

- LP64 scalar widths.
- Exact bitfield and over-aligned record layouts.
- Ordered namespace ownership and deduplication.
- Anonymous record handling.
- Type injection.
- Cross-WinMD imports and unresolved-reference diagnostics.
- Deterministic regeneration tests.

Replacing it provides no immediate functional benefit and would introduce
ABI risk.

## Recommendation

Keep `bnd-winmd` as the authoritative header parser and ABI normalization
layer. Do not wrap or fork `windows-clang` unless upstream first exposes
target-aware scalar mapping and layout customization.

If reducing custom metadata emission becomes a maintenance goal, evaluate a
model-to-RDL prototype using the existing simple and zlib fixtures before
trying the full Linux/OpenSSL surface. That prototype must preserve generated
Rust APIs and the ABI assertions already covered by bnd's end-to-end tests.

## Fork Experiment

The repository includes two non-published crates for experiments:

- `bnd-clang`, containing a fork of `windows-clang` under
  `bnd-clang/vendored/windows-clang`.
- `bnd-bindgen`, containing a fork of `windows-bindgen` under
  `bnd-bindgen/vendored/windows-bindgen`.

Each crate records the upstream revision and license in `VENDORED.md`.
They participate in normal workspace builds. `bnd-clang` makes
`clang-sys/runtime` optional and leaves it disabled for workspace builds, so
both Clang frontends use the same linked libclang selected at build time.
This avoids runtime discovery selecting a different libclang and changing
generated ABI output.

The fork now maps C `long` and `unsigned long` from the widths reported by
Clang, preserving Linux LP64 and Windows LLP64 behavior. Plain C functions
and callbacks also retain the C calling convention rather than falling back
to RDL's Windows platform default. Integration coverage ports the `multi`
fixture through C headers, RDL, WinMD, generated Rust, and linked runtime
calls. The first `simple` fixture tests cover LP64 fields and over-aligned
records.

These experiments remove the scalar-width and C-calling-convention blockers
from the fork, but not the partial-bitfield, configuration, injection, or
cross-crate generation gaps. Neither fork replaces the production
`windows-bindgen` dependency or the current `bnd-winmd` pipeline yet.

## Upstream References

- [`windows-clang` manifest](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/Cargo.toml#L1-L19)
- [`windows-clang` public API](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/lib.rs#L515-L890)
- [Scalar canonicalization](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/canon.rs#L725-L740)
- [Struct and bitfield handling](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/struct.rs#L24-L317)
- [libclang provisioning](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/clang/src/provision.rs#L1-L185)
- [RDL reader pipeline](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/rdl/src/reader/mod.rs#L43-L258)
- [Bindgen layout handling](https://github.com/microsoft/windows-rs/blob/ca99b307b4e3705da54be35396a33a5afad229fd/crates/libs/bindgen/src/types/cpp_struct.rs#L270-L529)
