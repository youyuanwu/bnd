# Anonymous Struct Used as Array Element Not Extracted

**Component:** bnd-winmd  
**Status:** Partially fixed — 1D arrays fixed; 2D arrays produce wrong layout (upstream windows-bindgen limitation)

## Problem

When a struct contains a field whose type is an **array of an anonymous
struct**, bnd-winmd fails to extract the anonymous struct and reports
an unresolved type reference at validation time:

```
struct (unnamed at rte_ethdev.h:999:2) — referenced in field `pool_map`
of struct `rte_eth_vmdq_dcb_conf`
```

## Affected Patterns

### 1D array (fixed)

```c
struct {
    uint16_t vlan_id;
    uint64_t pools;
} pool_map[RTE_ETH_VMDQ_MAX_VLAN_FILTERS];
```

Canonical type: `ConstantArray(Record)` — fixed by peeling one level.

### 2D array (open)

```c
struct rte_eth_dcb_tc_queue_mapping {
    struct {
        uint16_t base;
        uint16_t nb_queue;
    } tc_rxq[RTE_ETH_MAX_VMDQ_POOL][RTE_ETH_DCB_NUM_TCS];
};
```

Canonical type: `ConstantArray(ConstantArray(Record))` — the current
single-peel fix finds another `ConstantArray`, not a `Record`, and
returns `None`.

## Root Cause

`try_extract_anonymous_field()` peels exactly one `ConstantArray` level:

```rust
let (inner, array_len) = if canonical.get_kind() == TypeKind::ConstantArray {
    let len = canonical.get_size().unwrap_or(0);
    let elem = canonical.get_element_type()?;
    (elem.get_canonical_type(), Some(len))  // ← only peels once
} else {
    (canonical, None)
};
if inner.get_kind() != TypeKind::Record { return None; }
```

For `tc_rxq[M][N]`, after one peel `inner` is still `ConstantArray(Record)`,
not `Record`, so the function returns `None`.

## Fix for Multi-Dimensional Arrays

Peel **all** array levels, collecting dimensions as a `Vec<usize>`.
Check for anonymous record at the innermost element type. Wrap the
extracted synthetic type in nested `CType::Array` from innermost
outward.

```rust
// Peel all array levels, collecting dims outermost-first.
let mut dims: Vec<usize> = Vec::new();
let mut inner = field_type.get_canonical_type();
while inner.get_kind() == TypeKind::ConstantArray {
    dims.push(inner.get_size().unwrap_or(0));
    inner = match inner.get_element_type() {
        Some(e) => e.get_canonical_type(),
        None => return None,
    };
}

if inner.get_kind() != TypeKind::Record { return None; }
// ... extract anonymous record as synthetic_name ...

// Wrap from innermost outward.
let named = CType::Named { name: synthetic_name, resolved: None };
let ctype = dims.iter().rev().fold(named, |acc, &len| {
    CType::Array { element: Box::new(acc), len }
});
Some(ctype)
```

The same peel-all-levels logic must also be applied to the
`named_anon_decls` set collection to avoid double-extraction.

## Test Plan

Add to `simple.h`:

```c
struct WithAnon2DArrayField {
    struct {
        uint16_t base;
        uint16_t nb_queue;
    } tc_rxq[4][8];
    int count;
};
```

Verify:
- `WithAnon2DArrayField_tc_rxq` extracted as synthetic struct
- `size_of::<WithAnon2DArrayField>() == 4 * 8 * 4 + 4 == 132` (with alignment)
- No unresolved type references

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

## Test Results

**1D**: `WithAnonArrayField` with `struct { ... } entries[4]` → `[WithAnonArrayField_entries; 4]`, size=36. Test: `test_anon_struct_array_field`.

**2D**: `WithAnon2DArrayField` with `struct { ... } tc_rxq[4][8]` — extraction produces the correct nested `CType::Array { Array { Named, 8 }, 4 }`. However, **upstream windows-bindgen does not support nested `ArrayFixed`** and emits `[[T; 8]; 1]` (wrong outer dimension). The local fork handles this correctly and emits `[[T; 8]; 4]`.

The 2D test (`test_anon_struct_2d_array_field`) is `#[ignore]` — passes with the local fork, fails with upstream.

## Known Limitation: 2D Arrays Require windows-bindgen Fork

Upstream windows-bindgen does not support nested `ArrayFixed`. Multi-dimensional anonymous struct array fields produce a binding with the wrong outer dimension. Until the fork change is merged upstream, these bindings are incorrect and should not be used for FFI.
