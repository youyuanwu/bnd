# Injected Types — User-Declared Type Overrides

## Problem

bnd-winmd's clang extractor cannot handle certain C type patterns:

1. **Bitfield enums** — `enum fs_value_type type:8;` in a struct member.
2. **Anonymous enums inside structs** — no tag name, cannot register.
3. **Compiler-internal types** — e.g. `__int128` has no WinMD mapping.

## Solution

A top-level `[[inject_type]]` array in the TOML config. Each entry
declares a type that is merged into the specified partition **after**
clang extraction but **before** type registry construction and winmd
emission. Extracted types win on conflict, so injections are safe to
leave even after bnd-winmd learns to extract the type natively.

Injections are top-level (not nested under `[[partition]]`) because
they represent extraction workarounds, not normal partition content.
Keeping them separate makes the distinction visible.

## TOML Schema

```toml
[[inject_type]]
namespace = "rko.fs_context"   # target partition namespace
name = "fs_value_type"         # type name
kind = "enum"                  # enum | typedef | struct
underlying = "u8"              # integer type (enum/typedef only)
variants = [                   # enum variants (enum only, optional)
  { name = "fs_value_is_undefined", value = 0 },
  { name = "fs_value_is_flag", value = 1 },
]

[[inject_type]]
namespace = "rko.sysctl"
name = "ctl_table_header_type"
kind = "typedef"
underlying = "i32"

[[inject_type]]
namespace = "rko.types"
name = "opaque_handle"
kind = "struct"
size = 16
align = 8
```

### Fields

| Field | Required | Kind | Description |
|---|---|---|---|
| `namespace` | Yes | all | Target partition namespace |
| `name` | Yes | all | Type name as it appears in C |
| `kind` | Yes | all | `enum`, `typedef`, or `struct` |
| `underlying` | Yes | enum, typedef | One of `i8`–`u64` |
| `variants` | No | enum | `{ name, value }` pairs |
| `size` | Yes | struct | Size in bytes |
| `align` | Yes | struct | Alignment in bytes (1, 2, 4, or 8) |

## Behavior

### Pipeline position

```
clang extraction → partition model
                        │
inject_type entries ────┤  (extracted wins on conflict)
                        │
type registry ──────────┤
dedup / validate / emit ┘
```

### Conflict resolution

- **Extracted type exists** → injection skipped (`debug` log).
- **No matching partition** → warning, entry discarded.
- **Duplicate injection name** → first entry wins.

### Opaque struct padding

windows-bindgen collapses empty structs to 1-byte newtypes and emits
`packed(N)` which caps but does not raise alignment. To get correct
size and alignment, injected structs use a `_reserved` array field
whose element type matches the requested alignment:

| `align` | Element | Example for size=32 |
|---|---|---|
| 8 | `u64` | `_reserved: [u64; 4]` |
| 4 | `u32` | `_reserved: [u32; 8]` |
| 2 | `u16` | `_reserved: [u16; 16]` |
| 1 | `u8` | `_reserved: [u8; 32]` |

`size` must be a multiple of `align`.

## Tests

Covered in `tests/fixtures/simple/simple.toml` and `tests/e2e-simple`:

- **Injected enum** — `Priority` with 3 variants, correct values
- **Injected typedef** — `handle_t` as `u64`
- **Injected opaque struct** — `OpaqueCtx` with `size=32, align=8`
- **Conflict resolution** — `Color` injected but extracted version wins
