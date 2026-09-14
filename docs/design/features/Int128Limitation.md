# `__int128` Type Limitation

## Summary

`__int128` and `unsigned __int128` cannot be represented in WinMD.
The maintained `bnd-clang` fork omits typedefs that canonically resolve to
128-bit integer or floating-point types, including chained aliases such as
`typedef __int128 __s128; typedef __s128 s128;`. Functions and records that
depend on unrepresentable value types are also omitted rather than emitted
with an incorrect ABI.

Active coverage is in
[`bnd-clang/tests/linux_simple.rs`](../../../bnd-clang/tests/linux_simple.rs)
and
[`tests/e2e-clang-simple/src/lib.rs`](../../../tests/e2e-clang-simple/src/lib.rs).

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

The direct frontend checks the canonical Clang type while collecting
typedefs. `CXType_Int128`, `CXType_UInt128`, `CXType_Float128`, and C complex
types are not emitted. A later dependency pass prevents functions, callbacks,
records, and aliases from referring to the omitted definitions.

The generated-Rust E2E asserts that direct and chained 128-bit aliases are
absent.

## Workarounds for Downstream Consumers

Do not replace an exposed `__int128` value with a same-sized Rust struct:
SysV register classification and alignment can still differ.

Prefer one of:

- omit APIs that expose the type;
- add a C shim with a representable parameter/result ABI;
- keep the value behind an opaque pointer when the native API already uses
  pointer indirection.

## Possible Future Fixes

- **Upstream ECMA-335 extension** for 128-bit types. Unlikely.
- **Generator-specific wrapper metadata** paired with C shims. This can
  expose a representable ABI but is not a transparent mapping of native
  `__int128`.
