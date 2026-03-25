# windows-bindgen Uses `packed(N)` Where `align(N)` Is Needed — ⚠️ Upstream

## Summary

windows-bindgen interprets WinMD `ClassLayout` packing values as
`#[repr(C, packed(N))]`, which sets a **maximum** alignment cap.
When the source type has `__attribute__((aligned(N)))` — a **minimum**
alignment floor — the generated Rust struct has correct size but wrong
alignment semantics. Fields of that type embedded in other structs are
placed at the wrong offset.

**Status**: Workaround in bnd-winmd (explicit `_pad_N` fields).
Should be reported upstream to `microsoft/windows-rs`.

**Component**: `windows-bindgen` 0.66.0
(`microsoft/windows-rs/crates/libs/bindgen`)

## Root Cause

### ECMA-335 spec limitation

`ClassLayout` is defined in the **ECMA-335 standard** (Common Language
Infrastructure, Partition II §22.8). Its two columns are:

- **PackingSize** (u16) — analogous to C's `#pragma pack(N)`, controls
  the maximum field alignment (a cap, not a floor).
- **ClassSize** (u32) — explicit struct byte size.

There is no ECMA-335 equivalent of `__attribute__((aligned(N)))` — the
spec was designed for .NET/Windows where minimum-alignment attributes
don't exist. The `windows-metadata` crate faithfully implements the
spec, and `windows-bindgen` correctly translates `PackingSize` →
`packed(N)`. The bug is a **spec gap**, not an implementation error.

### How it manifests

bnd-winmd writes the clang alignment value into `PackingSize`:

```rust
// bnd-winmd/src/emit.rs:145
file.ClassLayout(td, s.align as u16, s.size as u32);
```

windows-bindgen reads this and emits `#[repr(C, packed(N))]`. The two
Rust attributes have opposite semantics:

| Attribute    | Meaning                | Effect on align(64) struct with 8-byte fields |
|-------------|------------------------|-----------------------------------------------|
| `packed(64)` | alignment ≤ 64 (cap)   | effective align = 8 (natural, since 8 < 64)  |
| `align(64)`  | alignment ≥ 64 (floor) | effective align = 64 ✓                        |

With `packed(64)`, `repr(C)` uses the natural field alignment (8),
not 64. The struct's **size** is correct (ClassSize is honored), but
when embedded in another struct, `repr(C)` places it at an 8-byte
boundary instead of a 64-byte boundary.

## Reproduction

```c
// C source
struct AlignedInner {
    long a;
    long b;
} __attribute__((aligned(64)));

struct EmbeddingAligned {
    long before_a;                       // offset 0
    long before_b;                       // offset 8
    struct AlignedInner aligned_member;  // offset 64 in C
    int after;                           // offset 128 in C
};
// sizeof(EmbeddingAligned) == 192
```

Current windows-bindgen output (incorrect embedding):

```rust
#[repr(C, packed(64))]       // ← should be align(64)
pub struct AlignedInner {
    pub a: i64,
    pub b: i64,
}
// size_of = 64 ✓ (ClassSize honored)
// align_of = 8 ✗ (packed(64) doesn't raise alignment)

#[repr(C)]
pub struct EmbeddingAligned {
    pub before_a: i64,                   // offset 0
    pub before_b: i64,                   // offset 8
    pub aligned_member: AlignedInner,    // offset 16 ✗ (should be 64)
    pub after: i32,                      // offset 80 ✗ (should be 128)
}
// size_of = 88 ✗ (should be 192)
```

Expected output if windows-bindgen emitted `align(N)`:

```rust
#[repr(C, align(64))]        // ← correct: minimum alignment
pub struct AlignedInner {
    pub a: i64,
    pub b: i64,
}
// size_of = 64 ✓
// align_of = 64 ✓

#[repr(C)]
pub struct EmbeddingAligned {
    pub before_a: i64,                   // offset 0  ✓
    pub before_b: i64,                   // offset 8  ✓
    pub aligned_member: AlignedInner,    // offset 64 ✓ (auto-aligned)
    pub after: i32,                      // offset 128 ✓
}
// size_of = 192 ✓ — no explicit padding needed
```

## Suggested Fix

Since the limitation is in the ECMA-335 spec itself (no `align(N)`
concept), any fix must extend beyond the standard. The `ClassLayout`
table cannot be repurposed — `PackingSize` has defined semantics.

Options:

- **Option A**: Add a WinMD **custom attribute** (e.g. `AlignAttribute(N)`)
  on types that need minimum alignment. windows-bindgen checks for it
  and emits `#[repr(C, align(N))]` instead of `packed(N)`. This is the
  cleanest approach — custom attributes are an ECMA-335 extension point.
- **Option B**: Use a heuristic — if `PackingSize > natural max field
  alignment`, treat it as `align(N)` instead of `packed(N)`. This
  requires no metadata changes but may misinterpret edge cases.
- **Option C**: Always emit `align(N)` when `PackingSize > 1` and the
  struct has no `#pragma pack` directive. This matches non-Windows
  usage where packing above natural alignment is meaningless.

Any option eliminates the need for explicit padding workarounds in
WinMD producers like bnd-winmd.

## bnd-winmd Workaround

`bnd-winmd/src/extract.rs` inserts explicit `_pad_N: [u8; gap]` fields
wherever clang's field offset exceeds the natural `repr(C)` position.
See `docs/design/features/CachelineAlignedFix.md` for details.

This produces correct layouts but pollutes the API with synthetic
padding fields that would be unnecessary if windows-bindgen emitted
`align(N)`.

## Impact

Affects any C type with alignment attributes exceeding natural field
alignment. In the Linux kernel, `____cacheline_aligned_in_smp` and
`____cacheline_aligned` are common (dozens of networking structs).
Without the workaround, Rust code reads fields at wrong offsets,
causing data corruption or kernel panics.
