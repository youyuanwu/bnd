# Cacheline-Aligned Struct Fix

## Problem

Some Linux kernel structs use `____cacheline_aligned_in_smp` or
`____cacheline_aligned` which expand to `__attribute__((aligned(64)))`.
This affects layout in two ways:

1. **Struct size**: The struct is padded to a multiple of 64 bytes.
2. **Embedding offset**: When the struct is embedded as a field in
   another struct, it must start at a 64-byte boundary.

bnd-winmd extracts the correct `size` and `align` from clang and passes
them via WinMD `ClassLayout`. However, windows-bindgen interprets
`ClassLayout` as `packed(N)` which sets **maximum** alignment, not
**minimum**. This means:

- `packed(64)` allows up to 64-byte alignment but doesn't force it.
- The struct's Rust alignment defaults to its max field alignment
  (e.g. 8 for a struct with `long` fields), not 64.
- When embedded, `repr(C)` places the field at the natural offset
  (based on field alignment), not the cacheline-aligned offset.

This causes field offset mismatches and kernel panics when Rust code
accesses fields at the wrong offset (see `docs/bugs/` in rko).

## Fix

### Inter-field padding

After extracting all fields, `insert_alignment_padding()` compares
each field's clang offset (from `get_offset_of_field()`) against where
Rust's `repr(C)` would naturally place it. Padding is only inserted
when clang's offset **exceeds** the natural position — meaning an
alignment attribute is forcing the field further than natural alignment.

Key insight: we compute the **Rust-side** natural offset using the
embedded type's max field alignment (what `repr(C, packed(N))` would
actually use), not clang's type alignment (which includes the alignment
attribute). This avoids false positives on normal structs while
catching `____cacheline_aligned` displacement.

```
Clang layout:                    Rust repr(C) without fix:
┌─────────┐ 0                   ┌─────────┐ 0
│ before_a│                     │ before_a│
│ before_b│ 8                   │ before_b│ 8
├─────────┤ 16                  ├─────────┤ 16  ← WRONG
│ (24 pad)│                     │ aligned │     (should be at 64)
│         │                     │ _member │
├─────────┤ 64  ← aligned(64)  └─────────┘
│ aligned │
│ _member │
├─────────┤ 128
│  after  │
└─────────┘ 192
```

### Trailing padding

After inter-field padding, if clang's `sizeof` exceeds where the last
field ends (rounded up to max Rust-side field alignment), trailing
padding bytes are appended.

### Detection logic

For each field with a known clang offset:

1. Compute `rust_align` = max natural alignment of the embedded type's
   own fields (not the type's declared alignment with attributes).
2. Compute `natural_offset` = round_up(cursor, rust_align).
3. If `clang_offset > natural_offset`: insert `_pad_N: [u8; gap]`.
4. Advance cursor to `clang_offset + field_size`.

For trailing padding:

1. Find `max_rust_field_align` across all fields (using Rust-side alignment).
2. Compute `natural_size` = round_up(cursor, max_rust_field_align).
3. If `struct_size > natural_size`: append `_padding: [u8; struct_size - cursor]`.

Normal alignment gaps (e.g. `int` followed by pointer → 4-byte gap)
match the natural offset and produce no explicit padding.

## Example

```c
struct AlignedInner {
    long a;
    long b;
} __attribute__((aligned(64)));

struct EmbeddingAligned {
    long before_a;    // offset 0
    long before_b;    // offset 8
    struct AlignedInner aligned_member;  // offset 64 (not 16!)
    int after;        // offset 128
};
// sizeof(EmbeddingAligned) == 192
```

Generated Rust (with fix):

```rust
pub struct EmbeddingAligned {
    pub before_a: i64,              // offset 0
    pub before_b: i64,              // offset 8
    pub _pad_0: [u8; 48],           // explicit padding (16→64)
    pub aligned_member: AlignedInner, // offset 64 ✓
    pub after: i32,                 // offset 128 ✓
    pub _padding: [u8; 60],         // trailing (132→192)
}
```

## Tests

`tests/fixtures/simple/simple.h` defines:

- `CacheAligned` — struct with `__attribute__((aligned(64)))`, tests
  trailing padding (8 bytes of fields → 64 bytes total).
- `AlignedInner` — 64-byte aligned struct (16 bytes fields + 48 pad).
- `EmbeddingAligned` — embeds `AlignedInner`, tests inter-field padding.

`e2e-simple` tests:

- `test_cacheline_aligned_struct` — `size_of::<CacheAligned>() == 64`
- `test_cacheline_aligned_embedded`:
  - `size_of::<EmbeddingAligned>() == 192`
  - `offset_of!(EmbeddingAligned, aligned_member) == 64`
  - `offset_of!(EmbeddingAligned, after) == 128`

## Implementation

- `bnd-winmd/src/extract.rs`:
  - `extract_struct_from_entity()` — collects `field_offsets` and
    `field_sizes` parallel vecs during field extraction.
  - `flatten_bitfields()` — maintains offset/size vecs when merging
    bitfield groups.
  - `insert_alignment_padding()` — compares clang offsets against
    Rust-side natural offsets; inserts `_pad_N` and trailing `_padding`.
