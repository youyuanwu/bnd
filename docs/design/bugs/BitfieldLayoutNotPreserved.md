# Bitfield Layout Not Preserved

## Problem

Bitfield fields are emitted as full-width typed fields, producing
wrong struct layout. For example, `enum BitfieldKind kind:8` becomes
a full `u32` field instead of occupying 1 byte, making the generated
Rust struct larger than the C struct.

### Enum extraction is NOT the issue

Originally this bug was filed as "enum not extracted in bitfield
context". Testing proved this wrong: a top-level enum in a traverse
header IS found by sonar regardless of whether it's used in a
bitfield. The `fs_value_type` case in `linux/fs_context.h` was
likely missed for a different reason (non-traversed header, include
chain, or clang AST merging) — not because of bitfield usage.

If an enum is genuinely missed by sonar (e.g. nested in a struct or
in a non-traversed include), `[[inject_type]]` remains the correct
workaround.

## Root Cause: Bitfield Width Not Preserved

Even when the enum is present (via injection or extraction), the
bitfield width (`:8`) is lost. There is a `TODO` in `emit.rs:150`:

```rust
file.Field(&field.name, &wintype, FieldAttributes::Public);
// TODO: emit NativeBitfieldAttribute for bitfield fields
```

bnd-winmd extracts `bitfield_width` and `bitfield_offset` from clang
into `FieldDef`, but the emit phase does not attach any metadata.
The field is emitted as a full-width typed field. For
`enum fs_value_type type:8`, the generated Rust field is a full
`u32` instead of an 8-bit bitfield, producing wrong struct layout.

### NativeBitfieldAttribute — upstream status

`NativeBitfieldAttribute` **does** exist in the win32metadata ecosystem.
The `win32metadata` project emits it as a custom attribute on a backing
`_bitfield` field:

```csharp
public struct FLICK_DATA
{
    [NativeBitfield("iFlickActionCommandCode", 0L, 5L)]
    [NativeBitfield("iFlickDirection", 5L, 3L)]
    [NativeBitfield("fControlModifier", 8L, 1L)]
    // ...
    public int _bitfield;
}
```

Each attribute carries `(name, offset, width)` — the same data we
have in `FieldDef.bitfield_offset` / `bitfield_width`.

However, the attribute **omits the original field type** (e.g.
`FLICKACTION_COMMANDCODE` vs `BOOL` vs `INT` all become bare `int`).
This was flagged as a metadata bug (win32metadata#1392).

**windows-bindgen has zero code to read `NativeBitfield` attributes.**
Issue [windows-rs#2942](https://github.com/microsoft/windows-rs/issues/2942)
was closed without resolution because:

1. The metadata lost field types — can't generate typed accessors
2. Rust has no native bitfield support (RFC #314, open since 2014)
3. Implementing getter/setter codegen is a significant upstream change

Our `flatten_bitfields()` approach matches what win32metadata does:
pack adjacent bitfields into a single backing integer field. We could
emit `NativeBitfield` attributes today, but nobody would read them.
When windows-bindgen eventually adds support, we can add attribute
emission and get generated accessors for free.

### Impact (before fix)

Structs with bitfield members have wrong layout. The struct's total
size from `ClassLayout` is correct, but windows-bindgen ignores
`ClassSize` and computes from fields — so the generated struct is
**larger** than the C struct. This is the opposite of the cacheline
padding case (where the struct was too small).

## Fix: Bitfield Type Flattening (Implemented)

Implemented in `flatten_bitfields()` in `bnd-winmd/src/extract.rs`.
Called after field collection and before trailing padding logic in
`extract_struct_from_entity()`.

Replace each bitfield field's type with the smallest integer that
fits its bit width. This produces correct struct layout without
requiring upstream windows-bindgen changes. Type information is lost
but layout is correct.

### Single bitfield (no overlap)

Replace the field type based on bit width:

| Bit width | Replacement type |
|---|---|
| 1–8 | `CType::U8` |
| 9–16 | `CType::U16` |
| 17–32 | `CType::U32` |
| 33–64 | `CType::U64` |

Example: `enum fs_value_type type:8` → `type: u8`

### Adjacent bitfields sharing bytes

Multiple bitfields can pack into the same byte(s):

```c
struct {
    unsigned a:4;    // offset 0, width 4
    unsigned b:4;    // offset 4, width 4
    unsigned c:16;   // offset 8, width 16
    int x;           // regular field
};
```

Here `a` and `b` share byte 0, and `c` occupies bytes 1–2. If each
is emitted as its own integer field, the struct is too large.

Use `bitfield_offset` (in bits, from clang) to detect groups:

1. Sort bitfield fields by `bitfield_offset`
2. Group adjacent bitfields whose bit ranges fit within the same
   backing storage unit
3. For each group, emit a single field:
   - Name: `_bitfield_{byte_offset}` (or first field's name if solo)
   - Type: smallest integer covering the group's total bit span
   - Clear `bitfield_width`/`bitfield_offset` on the merged field

For the example above:
- `a` (offset 0, width 4) + `b` (offset 4, width 4) → group spans
  bits 0–7 → `_bitfield_0: u8`
- `c` (offset 8, width 16) → solo → `_bitfield_1: u16`
- `x` → regular field, unchanged

### Validation

After flattening, compare the sum of field sizes against
`StructDef.size` from clang. If they don't match, log a warning —
the trailing padding logic will add `_padding` for any remaining gap.

### Test Result

`WithBitfield` in `simple.h` has `kind:8` + `flags:24` (adjacent
bitfields packing into 4 bytes) plus a pointer and int field.
`flatten_bitfields` merges `kind` + `flags` into `_bitfield_0: u32`.
Generated Rust struct is 16 bytes, matching C sizeof.

Test: `test_bitfield_enum_extracted` in `tests/e2e-simple/src/lib.rs`.
