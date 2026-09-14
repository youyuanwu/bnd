# Design: Pure-Rust C Header → WinMD Pipeline

> **Scope:** This document describes the standalone `bnd-winmd` library,
> CLI, and fixture architecture. `bnd-winmd` remains supported and its
> fixture tests remain active, but production `bnd-linux` and `bnd-openssl`
> now use the direct `bnd-clang` → RDL/WinMD → `bnd-bindgen` pipeline.

## Context

[CSharpGenerator.md](CSharpGenerator.md) describes a pipeline that reuses
[win32metadata](https://github.com/microsoft/win32metadata)'s .NET components
(ClangSharp + Roslyn Emitter) to generate `.winmd` from C headers. That approach
requires the .NET SDK and shells out to C# tools.

This document covers the **all-Rust alternative** that was chosen and
implemented: the [`clang`](https://crates.io/crates/clang) crate (idiomatic
libclang wrapper) for C header parsing, plus the
[`windows-metadata`](https://crates.io/crates/windows-metadata) crate for
winmd emission.

A later assessment of replacing this pipeline with the upstream
`windows-clang` and RDL stack is documented in
[WindowsClangEvaluation.md](WindowsClangEvaluation.md).

---

## Architecture

```
C/C++ Headers
      │
      ▼
┌─────────────┐
│   clang rs   │  Idiomatic libclang wrapper
│  + sonar     │  find_structs(), find_enums(), etc.
└──────┬───────┘
       │  Entity / Type / Declaration
       ▼
┌─────────────┐
│  bnd-winmd   │  Maps Entity → intermediate model → ECMA-335
│  (extract +  │  No intermediate code generation
│   emit)      │
└──────┬───────┘
       │  Vec<u8>
       ▼
  output.winmd
```

**Why this approach** (over `bindgen` → `syn` → winmd):
- **3–4 code layers** vs 6 with the bindgen pipeline
- **Direct access** to `Type::get_sizeof()`, `get_alignof()`, `Entity::get_bit_field_width()`, `Type::get_calling_convention()`  — no information loss or reverse mapping
- **Same underlying API as ClangSharp** — the conceptual C AST → ECMA-335 mapping is identical
- **No reverse type mapping** — reads C `int` directly instead of reversing `::std::os::raw::c_int`

---

## Implementation Status

> **Status: standalone tool implemented and tested.** Clippy clean.

### What Is Implemented

| Feature | Notes |
|---|---|
| CLI (`clap`) + TOML config parsing | `main.rs`, `config.rs` |
| Intermediate model types | `model.rs` — `StructDef`, `EnumDef`, `FunctionDef`, `TypedefDef`, `ConstantDef`, `CType`, `TypeRegistry` |
| Clang extraction (`clang` crate + sonar) | `extract.rs` — `collect_*` helpers for uniform extraction, custom typedef/struct discovery to work around sonar limitations |
| Partition filtering by source location | `should_emit_by_location()` checks `Entity::get_location()` against traverse file list |
| Type mapping (clang `TypeKind` → `CType`) | Void, Bool, char types, int/uint (all widths), float/double, Pointer, ConstantArray, IncompleteArray, Elaborated, Typedef, Record, Enum, FunctionPrototype. Incomplete records → Void. Int128/UInt128 → error (skipped with warning). |
| System typedef resolution | `CType::Named { resolved }` carries clang's canonical type; emit falls back to it for unregistered typedefs. `va_list` → `*mut c_void` at extraction. Typedefs shadowing Rust primitives (`bool`, `i32`, etc.) are skipped. |
| WinMD emission | `emit.rs` — enums, structs, unions, typedefs, delegates, functions (P/Invoke), constants |
| Union support | `StructDef.is_union` flag. `ExplicitLayout` + `FieldLayout(offset=0)` for unions, `SequentialLayout` for structs. Supplemental pass detects `UnionDecl`. |
| Anonymous nested types | `try_extract_anonymous_field()` detects `Entity::is_anonymous()` on canonical type declarations. Recursive extraction with synthetic names (`ParentName_FieldName`). |
| Anonymous enum → constants | `collect_enums()` detects unnamed enums (e.g. `enum { DT_UNKNOWN = 0, ... }`) and emits variants as standalone `ConstantDef` entries instead of named enum TypeDefs. |
| Hex constant extraction | Supplemental `MacroDefinition` pass with `parse_hex_or_suffixed_int()` handles `0x` hex, `0` octal, and `U`/`L`/`UL`/`ULL` suffixes. |
| Opaque typedef handling | Void-underlying typedefs (e.g. `DIR`) emit `isize` instead of `c_void` for copyable handle-like types. |
| Function pointer → delegate | Detects `Ptr(FnPtr{...})` and bare `FnPtr{...}`, emits TypeDef extending MulticastDelegate with Invoke method |
| `#define` integer constants | `sonar::find_definitions()` with `detailed_preprocessing_record` + supplemental hex pass |
| Cross-partition type references | `TypeRegistry` maps type name → namespace; emits `TypeRef` for named types |
| Structured logging (`tracing`) | `RUST_LOG=bnd_winmd=debug` shows per-declaration detail |
| Variadic function skipping | `collect_functions()` checks `Entity::is_variadic()` and warns/skips |
| Array parameter decay | `extract_function()` converts `CType::Array` params → `CType::Ptr` (C semantics; avoids `ELEMENT_TYPE_ARRAY` blob incompatibility with windows-bindgen) |
| Function deduplication | `collect_functions()` uses `HashSet<String>` to skip duplicates from glibc `__REDIRECT` macros |
| PtrConst workaround | Always emit `PtrMut` — `ELEMENT_TYPE_CMOD_REQD` mid-chain in pointer blobs panics windows-bindgen. Mutability preserved via `ParamAttributes::Out` on mutable pointer parameters. |
| Warn-and-skip error handling | Non-fatal failures log `tracing::warn!` and skip the declaration |
| Round-trip integration tests | Multiple fixture files |
| E2E integration tests | Standalone simple, multi, and zlib fixture crates |
| Package-mode code generation | Supported by the tool; no longer the production Linux/OpenSSL path |
| Cross-WinMD type imports | `[[type_import]]` in TOML pre-seeds TypeRegistry from external WinMD; retained for standalone users and historical fixtures |
| Pre-emit type-reference validation | `validate_type_references()` walks all CType trees before emit, catches `Named { resolved: None }` types missing from TypeRegistry. Reports actionable errors with type name, context (function param / struct field), and partition. |

### What Is NOT Yet Implemented

| Feature | Complexity |
|---|---|
| Bitfield attribute emission (`NativeBitfieldAttribute`) | Medium — extraction works, emission TODO |
| Multi-header wrapper generation | Low |
| COM interface support | Medium — needs `ELEMENT_TYPE_CLASS` fix in `windows-metadata` |
| Nested types (`NestedClass`) | Low |
| Inline function skipping | Low — detect `static inline` via storage class |

---

## File Structure

```text
bnd-winmd/
├── Cargo.toml
├── src/
│   ├── lib.rs               # Public API and orchestration
│   ├── config.rs            # TOML config deserialization
│   ├── model.rs             # Intermediate C/metadata model
│   ├── extract.rs           # clang Entity/Type -> model
│   └── emit.rs              # model -> windows-metadata writer
└── tests/
    ├── roundtrip_simple.rs
    ├── roundtrip_multi.rs
    ├── roundtrip_posixfile.rs
    ├── roundtrip_zlib.rs
    ├── roundtrip_openssl.rs
    └── roundtrip_validation.rs

tests/
├── fixtures/
├── simple-impl/
├── e2e-simple/
├── e2e-multi/
└── e2e-zlib/
```

The Linux and OpenSSL product layouts are documented in
[BndLinux.md](BndLinux.md) and
[systesting/Openssl.md](systesting/Openssl.md).

---

## Dependencies

```toml
[dependencies]
clang = { version = "2.0", features = ["clang_10_0"] }
windows-metadata = "0.60"
toml = "1"
clap = { version = "4", features = ["derive"] }
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
serde = { version = "1", features = ["derive"] }
```

---

## Implementation Discoveries

1. **`File` method borrow conflicts** — All `writer::File` methods take
   `&mut self`. Must compute TypeRef into a local variable before passing to
   TypeDef to avoid double mutable borrow.

2. **`detailed_preprocessing_record` required** — libclang does not expose
   macro definitions unless the parser is created with
   `detailed_preprocessing_record(true)`.

3. **Function pointer typedefs are `Ptr(FnPtr{...})`** — C `typedef int
   (*Name)(...)` maps to `CType::Ptr { pointee: FnPtr { ... } }`, not bare
   `FnPtr`. Delegate detection unwraps the pointer layer.

4. **Config path relativity** — Header paths in TOML are relative to the
   config file's directory. System headers require absolute paths.

5. **`sonar::find_definitions` returns `Definition` not `Declaration`** —
   Uses `DefinitionValue::Integer(bool, u64) | Real(f64)` where the `bool`
   indicates negation.

6. **Clang singleton constraint** — `clang::Clang::new()` can only be called
   once per process. Tests use `LazyLock<Vec<u8>>` to share winmd bytes.

7. **sonar drops typedef-to-typedef aliases** — `sonar::find_typedefs` filters
   out typedefs whose underlying type is "elaborated" (includes typedef chains
   like `typedef Byte Bytef`). **Fix**: custom `collect_typedefs()` iterates
   `TypedefDecl` entities directly.

8. **sonar misses structs without matching typedef** — `sonar::find_structs`
   discovers structs through `TypedefDecl` entities only. **Fix**:
   `collect_structs()` runs a supplemental pass over `StructDecl` entities
   with `entity.is_definition()`.

9. **System typedefs resolved via clang canonical types** — All typedef names
   preserved as `CType::Named { name, resolved }` during extraction.
   `resolved` holds the clang-resolved canonical primitive (e.g. `__mode_t` →
   `U32`). At emit time, `ctype_to_wintype()` checks `TypeRegistry::contains()`
   — if registered, emits TypeRef; if not, falls back to the `resolved` type.
   No hardcoded typedef table needed. Only `va_list` (compiler built-in with
   no canonical primitive) is special-cased during extraction (→ `*mut c_void`).

10. **Incomplete/opaque record types** — Forward-declared structs map to
    `CType::Void` so pointers become `*mut c_void`.

---

## Test Coverage

All passing, clippy clean.

### Roundtrip Tests

Parse headers → emit winmd → read back → assert.

**roundtrip_simple.rs** (`simple.h`): typedefs present, enum variants,
struct fields, union fields, anonymous nested types, functions, function params,
constants, delegate, pinvoke.

**roundtrip_multi.rs** (multi-partition): namespace placement,
functions, cross-partition typeref, constants, enums.

**roundtrip_posixfile.rs** (bnd-linux): fcntl/stat/unistd functions,
struct fields, struct sizes, constants, pinvoke.

**roundtrip_zlib.rs** (system headers): zlib structs, delegates,
functions, constants, z_stream fields, pinvoke.

**roundtrip_openssl.rs** (openssl): multi-library partitioning (crypto vs
ssl), opaque typedefs at scale, cross-WinMD type import validation (asserts
`tm` and `_IO_FILE` are not local TypeDefs).

### E2E Tests

Generated FFI bindings linked against real native libraries.

| Crate | What it exercises |
|---|---|
| `e2e-simple` | Single partition, simple.h, widgets + unions + anonymous nested types |
| `e2e-multi` | Multi-partition, cross-namespace type references |
| `e2e-zlib` | System header, real libz.so, compress/uncompress roundtrip |
| Historical `bnd-linux` integration | Former production consumer before the direct-Clang cutover |
| Historical `bnd-openssl` integration | Former production consumer before the direct-Clang cutover |

### Doc Tests + Freshness Test

---

## Known Limitations & Future Work

### `ELEMENT_TYPE_VALUETYPE` Hardcoded in Writer

The `windows-metadata` writer encodes all `Type::Name` references as
`ELEMENT_TYPE_VALUETYPE`. COM interfaces require `ELEMENT_TYPE_CLASS`.

**Mitigation**: v1 excludes COM interfaces. For v2: submit PR to
`windows-metadata` adding `Type::Class(TypeName)`, or manually encode the
signature blob bytes.

### `AssemblyRef` Is Private

The writer's `File::AssemblyRef()` method is private and uses a root-namespace
heuristic. Cannot create AssemblyRef with exact assembly name like
`"Windows.Win32"`.

**Mitigation**: v1 defines imported types locally. For v2: submit PR to
`windows-metadata` to expose a public `AssemblyRef()` method. The underlying
ECMA-335 table support already exists.

### C `long` Size — Linux LP64 ABI

C `long` is 64-bit on Linux x86-64 (LP64 ABI). The type mapping uses
`TypeKind::Long` → `I64` and `TypeKind::ULong` → `U64` to match the
host platform. This is correct for Linux-only usage. If Windows ABI
support is needed in the future, the mapping would need to become
platform-conditional (`long` → 32-bit on Windows LLP64).

### `clang` Crate Max Feature Is `clang_10_0`

The crate's highest feature flag is `clang_10_0`, but system libclang may be
18+. Works fine — all C header parsing APIs are stable since clang 3.x. If
a newer API is needed, add `clang-sys` as a direct dependency for raw FFI.

### ~~Cross-WinMD Type References~~ ✅ Implemented

The `[[type_import]]` TOML config pre-seeds the `TypeRegistry` from an
external winmd file. Types found in the external winmd are emitted as
TypeRef rows instead of local TypeDefs. At generation time, both winmds
are passed to `windows-bindgen` via `--in`, with `--reference` suppressing
local codegen for the imported namespace. This avoids `AssemblyRef` — the
merged reader resolves types by `(namespace, name)` across all loaded
winmds.

The first production consumer was the former `bnd-openssl` generator. The
current direct-Clang production design instead references the flat canonical
Linux WinMD at the Clang and RDL stages and uses exact Rust ownership routes.
See [CrossWinmdReferences.md](features/CrossWinmdReferences.md).

### `windows-bindgen` Compatibility Conventions

For bnd-winmd output to be consumed by `windows-bindgen`, these conventions
must hold:

- TypeDef row ordering — `FieldList`/`MethodList` delimit ownership via row indices
- Enums — extend `System.Enum`, have `value__` field, literal fields with `Constant`
- Structs — extend `System.ValueType`, `SequentialLayout` flag, `ClassLayout`
- Delegates — extend `System.MulticastDelegate`, have `Invoke` method
- Functions — `ImplMap` entries pointing to `ModuleRef` (DLL/so name)
- Custom attributes — `NativeTypedefAttribute`, `NativeBitfieldAttribute`, etc.

All of the above are implemented and verified by tests.

---

## Actual LOC

| Component | LOC | Scope |
|---|---|---|
| Public API + module declarations | ~190 | `lib.rs` |
| TOML config | ~145 | `config.rs` |
| Intermediate model types | ~200 | `model.rs` |
| Extraction (clang → model) | ~860 | `extract.rs` |
| Emission (model → winmd) | ~430 | `emit.rs` |
| Roundtrip tests | ~1,340 | 5 files |
| E2E test crates (simple/multi/zlib) | ~530 | 3 crates |
| Historical bnd-linux integration tests | ~2,000 | 14 files |
| Historical bnd-openssl integration tests | ~370 | 7 files |
| **Total (bnd-winmd library)** | **~1,855** | |
| **Total (library + all tests)** | **~6,095** | |

---

## Open Questions

### `ELEMENT_TYPE_CLASS` for COM Interfaces

The writer hardcodes `ELEMENT_TYPE_VALUETYPE` for all named types. Options:
- Submit PR to `windows-metadata` adding `Type::Class(TypeName)`
- Manually encode the signature blob bytes
- Fork the writer

### Minimum Viable v2 Feature Set

- ✅ Unions (explicit layout)
- ✅ Anonymous nested types (synthetic naming)
- ✅ Anonymous enum → constants
- ✅ Hex/octal constant extraction
- ✅ Opaque typedef handling
- ✅ PtrConst workaround
- ⬜ Bitfield emission (`NativeBitfieldAttribute`)
- ✅ Cross-WinMD type imports ([design doc](CrossWinmdReferences.md))
- ⬜ COM interfaces (needs ELEMENT_TYPE_CLASS)
- ⬜ Nested types

---

## Reference Links

| Resource | URL |
|---|---|
| `windows-metadata` crate | https://crates.io/crates/windows-metadata |
| `windows-metadata` source | https://github.com/microsoft/windows-rs/tree/master/crates/libs/metadata |
| Writer `File` API | https://github.com/microsoft/windows-rs/blob/master/crates/libs/metadata/src/writer/file/mod.rs |
| Type enum | https://github.com/microsoft/windows-rs/blob/master/crates/libs/metadata/src/ty.rs |
| `windows-bindgen` | https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen |
| `clang` crate | https://crates.io/crates/clang |
| `clang` crate docs | https://docs.rs/clang/latest/clang/ |
| `clang::Entity` API | https://docs.rs/clang/latest/clang/struct.Entity.html |
| `clang::Type` API | https://docs.rs/clang/latest/clang/struct.Type.html |
| `clang::sonar` module | https://docs.rs/clang/latest/clang/sonar/index.html |
| `clang-sys` crate | https://crates.io/crates/clang-sys |
| ECMA-335 spec | https://www.ecma-international.org/publications-and-standards/standards/ecma-335/ |
