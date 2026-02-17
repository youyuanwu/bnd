# NativeArrayInfo for Array Parameters

## Status

Proposed — not yet implemented.

## Problem

C functions can declare array parameters with a fixed size:

```c
extern int futimens(int __fd, const struct timespec __times[2]);
```

Today `bnd-winmd` decays these to raw pointers during extraction:

```rust
// extract.rs — extract_function()
CType::Array { element, .. } => CType::Ptr { pointee: element, is_const: false }
```

This is ABI-correct (C11 §6.7.6.3p7), but discards the size hint.
The generated Rust FFI is `*const timespec` — the caller gets no indication
that exactly 2 elements are expected.

## Opportunity

WinMD supports a `NativeArrayInfoAttribute` custom attribute on `Param` rows.
`windows-bindgen` already reads it and generates safe wrappers:

| Attribute value | `ParamHint` | Wrapper signature |
|---|---|---|
| `I32(len)` | `ArrayFixed(len)` | `times: &[timespec; 2]` |
| `I16(idx)` | `ArrayRelativeLen(idx)` | `buf: &[u8]` (len from param `idx`) |

For `futimens`, emitting `NativeArrayInfoAttribute(CountConst = 2)` on the
`__times` param would let `windows-bindgen` generate:

```rust
// safe wrapper (non-sys mode)
pub unsafe fn futimens(fd: i32, times: &[timespec; 2]) -> i32
```

Instead of today's:

```rust
// sys mode — raw FFI
windows_link::link!("c" "C" fn futimens(fd: i32, times: *const timespec) -> i32);
```

## Constraint: `--sys` mode

`bnd-winmd` currently uses `--sys` (and `--package`) for all codegen.
In `--sys` mode, `windows-bindgen` skips `CppMethod` entirely and emits
raw `windows_link::link!()` declarations. **`NativeArrayInfo` is ignored.**

Safe wrappers (`&[T; N]`, `&[T]`) are only generated in **non-sys** (default)
mode, which produces higher-level API wrappers around the raw FFI.

### Implication

Emitting `NativeArrayInfoAttribute` today would:
- ✅ Correctly encode the metadata in the `.winmd`
- ❌ Have no visible effect on generated Rust code (since we use `--sys`)

The attribute becomes useful only if/when:
1. We switch to non-sys mode, or
2. We generate both sys and non-sys layers, or
3. A downstream consumer reads the `.winmd` in non-sys mode

## Design

### Model change

Add an optional array length to `ParamDef`:

```rust
pub struct ParamDef {
    pub name: String,
    pub ty: CType,
    /// If the original C parameter was `T param[N]`, the fixed array length.
    /// Used to emit NativeArrayInfoAttribute in the WinMD.
    pub array_len: Option<usize>,
}
```

### Extraction change

In `extract_function()`, preserve the array length before decaying:

```rust
let (ty, array_len) = match ty {
    CType::Array { element, len } => (
        CType::Ptr { pointee: element, is_const: false },
        Some(len),
    ),
    other => (other, None),
};
params.push(ParamDef { name, ty, array_len });
```

The pointer decay remains — the FFI type must be a pointer regardless.

### Emission change

In `emit_function()`, after emitting `file.Param(...)`, attach the attribute:

```rust
let param_id = file.Param(&param.name, (i + 1) as u16, attrs);

if let Some(len) = param.array_len {
    // NativeArrayInfoAttribute with CountConst = len (I32 → ArrayFixed)
    let attr_ctor = native_array_info_ctor(file);
    file.Attribute(
        HasAttribute::Param(param_id),
        AttributeType::MemberRef(attr_ctor),
        &[("CountConst".to_string(), Value::I32(len as i32))],
    );
}
```

The `native_array_info_ctor` helper creates (or reuses) the `MemberRef` for
`NativeArrayInfoAttribute::.ctor`:

```rust
fn native_array_info_ctor(file: &mut File) -> id::MemberRef {
    let type_ref = file.TypeRef("Windows.Win32.Foundation.Metadata", "NativeArrayInfoAttribute");
    file.MemberRef(
        ".ctor",
        &Signature { /* void() */ },
        MemberRefParent::TypeRef(type_ref),
    )
}
```

### Required imports

```rust
use windows_metadata::writer::{HasAttribute, AttributeType};
```

These are not currently imported in `emit.rs`.

## Future: `ArrayRelativeLen`

Some C APIs use a `(buf, len)` pattern:

```c
ssize_t read(int fd, void *buf, size_t count);
```

This maps to `NativeArrayInfoAttribute(CountParamIndex = 2)` (I16 value),
producing `buf: &mut [u8]` with length from the `count` parameter.

This is harder to detect automatically from C headers — the relationship
between pointer and length params isn't explicit. Possible approaches:

- Manual annotation in the TOML config
- Heuristic matching (e.g., consecutive `ptr` + `size_t` params)
- Ignore (leave as raw pointers)

Out of scope for the initial implementation.

## Decision needed

Since `--sys` mode ignores the attribute, the implementation effort only pays
off when we move to non-sys codegen. Options:

1. **Implement now** — metadata is correct, ready for future non-sys mode
2. **Defer** — implement when non-sys mode is added
3. **Implement metadata only** — emit the attribute, keep `--sys`, document
   that safe wrappers require non-sys mode

## References

- `windows-bindgen` reader: [`cpp_method.rs:41-46`](../../) — `NativeArrayInfoAttribute` → `ParamHint`
- `windows-bindgen` codegen: [`cpp_method.rs:527-541`](../../) — `ArrayFixed` → `&[T; N]`
- `windows-bindgen` sys bypass: [`cpp_fn.rs:107`](../../) — `config.sys` early return
- `windows-metadata` writer: [`file/mod.rs:262`](../../) — `Attribute()` API
- ECMA-335 II.22.10 — CustomAttribute table
- Current workaround: [`extract.rs:581-588`](../../bnd-winmd/src/extract.rs) — array→pointer decay
- Bug doc: [`element-type-array-mismatch.md`](../bugs/element-type-array-mismatch.md)
