# `__int128` Type Limitation

## Summary

`__int128` and `unsigned __int128` cannot be represented in WinMD.
bnd-winmd skips any type that resolves to a 128-bit integer and logs
a warning. Typedef chains (e.g. `typedef __int128 __s128; typedef
__s128 s128;`) are also skipped recursively.

## Why It Cannot Be Mapped

### No 128-bit integer in ECMA-335

WinMD is based on the ECMA-335 (CLI) type system. The largest integer
types are `I8`/`U8` (64-bit). There is no `I16`/`U16` (128-bit)
variant. The `windows-metadata` crate's `Type` enum confirms this —
it stops at `I64`/`U64`.

### Could it be an opaque struct?

A 16-byte struct would preserve size, but alignment is wrong:

| Approach | Size | Align | Correct? |
|---|---|---|---|
| `__int128` on x86_64 | 16 | 16 | ✓ |
| Struct with `[u64; 2]` field | 16 | 8 | ✗ |
| Struct with `[u8; 16]` field | 16 | 1 | ✗ |

`__int128` requires 16-byte alignment on x86_64. The largest primitive
in WinMD is `u64` (align=8), so a padding field can only achieve
align=8 at best.

### WinMD ClassLayout only has PackingSize

The ECMA-335 `ClassLayout` table has two columns:

- **PackingSize** (u16) — windows-bindgen emits `#[repr(C, packed(N))]`
- **ClassSize** (u32) — explicit struct size

`packed(N)` *caps* alignment at N — it does not *raise* it above the
natural alignment of the fields. Setting `packed(16)` with a `[u64; 2]`
field still produces align=8 because `u64` is naturally 8-byte aligned.

There is no `align(N)` equivalent in WinMD. windows-bindgen has no
code path that emits `#[repr(align(N))]`.

### ABI mismatch for function parameters

Even if size and alignment were correct, `__int128` is passed in
registers on x86_64 SysV ABI (integer class). A struct of the same
size would be passed differently (memory/pointer), producing incorrect
calling convention at the FFI boundary.

## Current Behavior

1. `map_clang_type` encounters `TypeKind::Int128` or `TypeKind::UInt128`
2. Returns `bail!` with a descriptive message
3. Caller (`collect_typedefs`, `collect_structs`) catches the error and
   logs a `warn!` — the containing typedef or struct field is skipped
4. Typedef chains resolve recursively: `typedef __s128 s128` calls
   `map_clang_type` on the canonical type, which hits the same bail

## Workarounds for Downstream Consumers

If a struct contains an `__int128` field and the consumer accepts the
alignment trade-off, use `[[inject_type]]`:

```toml
[[inject_type]]
namespace = "my.types"
name = "__int128"
kind = "struct"
size = 16
align = 8    # best available; true alignment is 16
```

This gives size=16 with align=8 — usable for opaque storage but not
for arithmetic or correct ABI on function boundaries.

## Possible Future Fixes

- **Patch windows-bindgen** to emit `#[repr(C, align(N))]` when the
  packing size exceeds the natural field alignment. This is the
  correct fix but requires an upstream change.
- **Post-process generated Rust** to replace `packed(16)` with
  `align(16)`. Fragile and version-dependent.
- **Upstream ECMA-335 extension** for 128-bit types. Unlikely.
