# Anonymous Union/Struct Members Fix

## Problem

C11 anonymous struct/union members (no field name, no tag) were
silently dropped during extraction, causing wrong struct sizes and
field offsets:

```c
struct inode {
    u64 i_ino;
    union { unsigned int i_nlink; unsigned int __i_nlink; }; // was dropped
    dev_t i_rdev;
};
```

## What Already Worked

Anonymous records **with a field name** (`union { ... } val;`) were
handled by `try_extract_anonymous_field()`, which detects the anonymous
type on the `FieldDecl` and extracts it as a synthetic named type.

## What Was Broken

Anonymous members **without a field name** appear as bare `UnionDecl`
or `StructDecl` children in clang's AST, not as `FieldDecl`. The
extraction loop filtered on `FieldDecl` only, so these were skipped.

## Fix

Extended `extract_struct_inner()` to also process anonymous
`UnionDecl`/`StructDecl` children:

- Builds a set of anonymous record decls that already have a named
  `FieldDecl` sibling (e.g. `union { ... } addr;`) — these are
  excluded since `try_extract_anonymous_field` handles them.
- For remaining anonymous union/struct children, extracts them as
  synthetic `{Parent}__anon_{N}` types and adds a field referencing
  the synthetic name.
- Recursion is supported for deeply nested anonymous members.

## Real-World Impact

`sigcontext` in bnd-linux gained the previously-dropped
`fpstate`/`__fpstate_word` anonymous union. The bnd-linux sources
and winmd were regenerated.

## Tests

`tests/fixtures/simple/simple.h` has `HasAnonUnion`:
```c
typedef struct {
    int before;
    union { int x; float y; };
    int after;
} HasAnonUnion;
```

`e2e-simple::test_c11_anonymous_union_member` verifies:
- `size_of::<HasAnonUnion>() == 12`
- `offset_of!(HasAnonUnion, before) == 0`
- `offset_of!(HasAnonUnion, after) == 8` (not 4)
- Union field access works through `HasAnonUnion__anon_0`
