# Sonar Forward Declaration Skips Enum

**Component:** bnd-winmd  
**Status:** Fixed — supplemental `EnumDecl` pass in `collect_enums()`

## Problem

`sonar::find_enums` (in the `clang` crate v2.0.0) silently skips
enums that have a forward declaration in the clang AST before their
full definition. The forward declaration poisons sonar's `seen` set,
preventing the actual definition from being returned.

Discovered via `enum fs_value_type` in `linux/fs_context.h` — other
enums in the same file (`fs_context_purpose`, `fs_context_phase`)
are extracted correctly because they lack forward declarations.

## Root Cause

Sonar's `next()` function adds the enum name to `seen` unconditionally
but only returns the declaration if it has children (variants):

```rust
if entity.get_kind() == EnumDecl {
    if let Some(name) = entity.get_name() {
        if !seen.contains(&name) {
            seen.insert(name);              // added unconditionally
            if entity.get_child(0).is_some() {  // only returns if has children
                return Some(Declaration::new(...));
            }
        }
    }
}
```

When clang emits a forward `EnumDecl` (no children) before the full
definition (with children), sonar marks the name as seen on the
forward decl, then skips the real definition because it's already
in `seen`.

Clang AST for `fs_context.h`:

```
EnumDecl <line:66:1, col:6> fs_value_type         ← forward decl (no children)
...
EnumDecl prev 0x... <line:51:1, line:58:1> fs_value_type ← definition (has children)
```

## Not a Bitfield Issue

Initially misdiagnosed as "enum used in bitfield not extracted".
Testing proved bitfield usage is irrelevant — `fs_context_purpose`
is also used as `:8` bitfield and IS extracted. The bnd simple test
(`BitfieldKind` in `simple.h`) confirms enum extraction works in
bitfield context when there's no forward declaration.

## Upstream Fix (Not Planned)

In sonar's `next()`, only add the name to `seen` when the entity
has children (is a definition, not a forward declaration):

```rust
if entity.get_kind() == EnumDecl {
    if let Some(name) = entity.get_name() {
        if !seen.contains(&name) {
            if entity.get_child(0).is_some() {
                seen.insert(name);
                return Some(Declaration::new(...));
            }
            // forward decl: do NOT add to seen
        }
    }
}
```

The same pattern likely affects `find_structs` — forward-declared
structs (`struct foo;`) could have the same problem.

We will not patch the `clang` crate. The fix below is in bnd-winmd.

## Fix: Supplemental `EnumDecl` Pass (Implemented)

Implemented in `collect_enums()` in `bnd-winmd/src/extract.rs`.
After `sonar::find_enums`, a supplemental pass walks top-level
`EnumDecl` entities, matching the established pattern used by
`collect_structs()` (supplemental `StructDecl`/`UnionDecl` walk),
`collect_typedefs()` (fully custom), and `collect_constants()`
(supplemental hex macro walk).

The `is_definition()` check skips forward declarations — the same
guard `collect_structs()` uses. `extract_enum()` delegates to
`extract_enum_from_entity()` to share logic between both paths.

## Workaround (no longer needed)

`[[inject_type]]` with `kind = "enum"` still works but should no
longer be necessary for enums missed due to forward declarations.

## Related

- `BitfieldLayoutNotPreserved.md` — separate issue where bitfield
  widths are lost during emission (fixed by `flatten_bitfields()`)
- `bnd-winmd --dry-run` and registry summary logs help detect
  silently missed types
