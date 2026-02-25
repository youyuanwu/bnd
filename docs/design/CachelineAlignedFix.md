# Cacheline-Aligned Struct Fix

## Problem

Some Linux kernel structs use `____cacheline_aligned` which expands to
`__attribute__((aligned(64)))`. This pads the struct to the next
multiple of 64 bytes. For example, `inode_operations` has 200 bytes of
fields but `sizeof` is 256 (next 64-byte boundary).

bnd-winmd extracts the correct `size=256, align=64` from clang and
writes them into WinMD's `ClassLayout` table. However, windows-bindgen:

1. Reads `packing_size=64` → emits `#[repr(C, packed(64))]`
2. Computes struct size from fields alone (200 bytes)
3. Ignores `ClassSize` (256) — the trailing 56 padding bytes are lost

The generated Rust struct is 200 bytes instead of 256, causing memory
corruption when embedded in larger aggregates.

## Fix: Trailing Padding Field

When `StructDef.size` exceeds the sum of field sizes, append a
`_padding` field to make up the difference. This works within the
existing pipeline — no windows-bindgen changes needed.

### Where

In `bnd-winmd/src/extract.rs`, in `extract_struct_inner()`, after
collecting all fields and before returning the `StructDef`.

### Logic

```
field_size = sum of each field's contribution to struct layout
             (use clang's sizeof for each field type)
padding    = struct.size - field_size

if padding > 0:
    fields.push(FieldDef {
        name: "_padding",
        ty: CType::Array { element: U8, len: padding },
        bitfield_width: None,
        bitfield_offset: None,
    })
```

The exact `field_size` calculation doesn't need to replicate C struct
layout rules (alignment gaps between fields). clang already accounts
for inter-field padding in its `sizeof(struct)`. The only padding we
need to add is the *trailing* padding that comes from the alignment
attribute, which is `sizeof(struct) - offsetof(last_field) - sizeof(last_field)`.

A simpler approach: use `clang_Cursor_getOffsetOfField()` on the last
field to compute the occupied range, then pad the remainder:

```
last_field_end = (offset_of_last_field_bits / 8) + sizeof(last_field)
trailing_pad   = struct.size - last_field_end

if trailing_pad > 0:
    append _padding: [u8; trailing_pad]
```

### Why This Works

- `packed(64)` caps alignment at 64 but doesn't affect size
- The `[u8; 56]` padding field adds 56 bytes, bringing total to 256
- windows-bindgen sees 256 bytes of fields, emits correct size
- No upstream changes required

### Alignment Caveat

`packed(64)` caps alignment — it doesn't raise it above the natural
field alignment (8, from pointer fields). The generated struct gets
align=8, not align=64. For most uses (struct-in-struct embedding),
this is fine because the kernel doesn't require 64-byte alignment of
the *container* — it requires the struct's *size* to be a multiple of
64 so that adjacent structs don't overlap. If true 64-byte alignment
is needed, a downstream `#[repr(align(64))]` wrapper is required.

### Edge Cases

- **No trailing padding** (most structs) — no field added, no change
- **Union types** — unions use `ExplicitLayout`; all fields overlap at
  offset 0, so `size == max(field_sizes)` with possible alignment
  padding. Same logic applies.
- **Bitfields** — bitfield widths don't map cleanly to byte sizes.
  Use clang's `sizeof(struct)` as ground truth rather than summing
  field sizes manually.

## Test Plan

Add a test struct to `simple.h` with `__attribute__((aligned(64)))`:

```c
struct CacheAligned {
    int x;
    int y;
} __attribute__((aligned(64)));
```

This gives `sizeof = 64` (8 bytes of fields padded to 64). Verify:

```rust
assert_eq!(size_of::<CacheAligned>(), 64);
```
