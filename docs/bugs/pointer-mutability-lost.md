# Pointer Mutability Lost for Function Parameters — ✅ Resolved

## Summary

All pointer function parameters in generated Rust bindings were `*const`,
even when the C declaration uses a mutable pointer (`T *`).

**Status**: Fixed. `emit.rs` now sets `ParamAttributes::Out` on mutable
pointer parameters, which prevents windows-bindgen from applying
`to_const_ptr()`. All bindings regenerated and tests updated.

## Root Cause

Three things interacted:

1. **Extraction correct** — `extract.rs` reads `is_const` from clang
2. **Emission discards `is_const`** — `ctype_to_wintype` always emits
   `Type::PtrMut` (required because windows-bindgen can't parse nested
   `PtrConst` blobs). The comment claimed `ConstAttribute` would track
   constness, but that was never implemented.
3. **windows-bindgen defaults params to `*const`** — `method_def.rs`
   applies `to_const_ptr()` to every parameter without `ParamAttributes::Out`

Return types were unaffected (they skip `to_const_ptr()`).

## Fix Applied

- `model.rs`: added `CType::is_outer_ptr_mut()` — returns `true` for
  `Ptr { is_const: false, .. }`
- `emit.rs` `emit_function`: sets `ParamAttributes::Out` on params where
  `is_outer_ptr_mut()` is true
- `emit.rs` `ctype_to_wintype`: updated comment to document the actual
  `ParamAttributes::Out` mechanism
- `roundtrip_simple.rs`: `roundtrip_param_mutability` test validates
  `create_widget`'s `out` and `destroy_widget`'s `w` params carry the
  `Out` flag, while `name` (`const char *`) does not

## Impact

All bnd-posix (12 files, 217 changed signatures) and bnd-openssl (7 files)
bindings regenerated. E2E tests in 12 test files updated to remove
spurious `as *const _` casts and replace `null()` with `null_mut()`
where appropriate.
