# Anonymous Struct Used as Array Element Not Extracted

**Component:** bnd-winmd  
**Status:** Fixed

## Problem

When a struct contains a field whose type is an **array of an anonymous
struct**, bnd-winmd fails to extract the anonymous struct and reports
an unresolved type reference at validation time:

```
struct (unnamed at rte_ethdev.h:999:2) — referenced in field `pool_map`
of struct `rte_eth_vmdq_dcb_conf`
```

## Affected Pattern

```c
struct rte_eth_vmdq_dcb_conf {
    // ...
    struct {
        uint16_t vlan_id;
        uint64_t pools;
    } pool_map[RTE_ETH_VMDQ_MAX_VLAN_FILTERS];  // array of anonymous struct
    // ...
};
```

The field type is `ConstantArray` of an anonymous `Record` — not a
bare `Record`. The named-field variant (`union { ... } addr;`) is
handled, but the array variant is not.

## Root Cause

`try_extract_anonymous_field()` checks the canonical type kind:

```rust
let canonical = field_type.get_canonical_type();
if canonical.get_kind() != TypeKind::Record {
    return None;  // ← returns here for ConstantArray
}
```

When `field_type` is `struct { ... } pool_map[N]`, the canonical type
is `ConstantArray`, not `Record`. The function returns `None` without
extracting the anonymous element type. The field is then mapped via
`map_clang_type()`, which produces `CType::Array { element: CType::Named("struct (unnamed ...)"), ... }` — an unresolvable named reference.

## Fix

In `try_extract_anonymous_field()`, add a peel step before the
`Record` check: if the canonical type is `ConstantArray`, extract the
element type and array length, then check if the element is an
anonymous record.

## Implementation

Two changes in `bnd-winmd/src/extract.rs`:

**1. `try_extract_anonymous_field`** — changed return type from
`Option<String>` to `Option<CType>`. Peels one `ConstantArray` level
before the `Record` check, then wraps the result in
`CType::Array { element: Named(synthetic), len }` when needed.

**2. `named_anon_decls` set** — also peels arrays when checking
whether a named `FieldDecl` references an anonymous record. Without
this fix, the same anonymous struct would be extracted twice: once by
`try_extract_anonymous_field` (as `WithAnonArrayField_entries`) and
once by the C11 anonymous member path (as `WithAnonArrayField__anon_0`).

The call site uses the returned `CType` directly — no `CType::Named`
wrapping needed.

## Test Result

`WithAnonArrayField` in `simple.h` with `struct { unsigned short id; unsigned int mask; } entries[4]`:
- `WithAnonArrayField_entries` extracted as synthetic struct (8 bytes)
- `WithAnonArrayField.entries` emitted as `[WithAnonArrayField_entries; 4]`
- `size_of::<WithAnonArrayField>() == 36` (matches C)
- No double-extraction, no unresolved type references

Test: `test_anon_struct_array_field` in `tests/e2e-simple/src/lib.rs`.
