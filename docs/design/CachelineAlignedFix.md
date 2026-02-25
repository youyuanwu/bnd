# Cacheline-Aligned Struct Fix

## Problem

Some Linux kernel structs use `____cacheline_aligned` which expands to
`__attribute__((aligned(64)))`. This pads the struct to the next
multiple of 64 bytes. bnd-winmd extracts the correct `size` and `align`
from clang, but windows-bindgen computes struct size from fields alone
and ignores `ClassSize` — the trailing padding bytes are lost.

## Fix

In `extract_struct_inner()`, after collecting all fields, compare
clang's `sizeof(struct)` against what `repr(C)` layout would produce
from fields alone. If clang's size exceeds the natural field-based
size, append a `_padding: [u8; N]` field for the difference.

### Detection Logic

1. Find `last_field_end` = max(offset + sizeof) across all `FieldDecl`
   children (using `get_offset_of_field()` in bits / 8)
2. Find `max_field_align` = max natural alignment across all fields
3. Compute `natural_size` = round_up(last_field_end, max_field_align)
4. If `clang_size > natural_size`: append `_padding: [u8; clang_size - last_field_end]`

This avoids false positives on normal structs where trailing padding
comes from natural alignment (e.g. `Widget` with pointer fields has
natural 8-byte alignment padding that `packed(8)` handles correctly).

### Alignment Caveat

`packed(N)` caps alignment — it doesn't raise it above the natural
field alignment. A struct with align=64 but max field align=8 gets
Rust align=8, not 64. This is acceptable: the fix ensures correct
**size**, which prevents overlap in struct-of-structs. True 64-byte
alignment requires a downstream `#[repr(align(64))]` wrapper.

## Tests

`tests/fixtures/simple/simple.h`:
```c
struct CacheAligned {
    int x;
    int y;
} __attribute__((aligned(64)));
```

`e2e-simple::test_cacheline_aligned_struct` verifies `size_of == 64`.
