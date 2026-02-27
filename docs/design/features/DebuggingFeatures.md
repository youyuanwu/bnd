# Debugging & Validation Features

## 1. Silent Extraction Warning

**Status:** Implemented

After extraction, warns if a partition produced zero types (0 structs,
enums, functions, typedefs, and constants). Catches misconfigured
headers/traverse paths immediately.

```
WARN  partition extracted 0 types — check headers and traverse paths  namespace=rko.sync
```

## 2. `--dry-run` Mode

**Status:** Implemented

```
bnd-winmd --dry-run config.toml
```

Runs the full pipeline (extraction, injection, registry, dedup,
validation, winmd emit) but does not write the output file. Prints
partition stats, registry summary, and exits with non-zero on
unresolved types. Useful for fast config validation.

Example output:

```
INFO  partition extraction complete  namespace=SimpleTest structs=8 enums=1 functions=4
INFO  type registry built  types=13 partitions=1 injected=4 imported=0
INFO  generated winmd  size=2828
INFO  validation passed
```

## 3. Registry Summary Log

**Status:** Implemented

After building the type registry, logs total types, partition count,
injected count (from `[[inject_type]]`), and imported count (from
`[[type_import]]`) at `info` level:

```
INFO  type registry built  types=342 partitions=8 injected=3 imported=45
```

## 4. Duplicate Type Summary

**Status:** Implemented

After the dedup pass, logs an `info`-level summary when duplicates
were dropped, with a hint to see details at `warn` level:

```
INFO  deduplicated types across partitions  dropped=3 (set RUST_LOG=warn for details)
```


## 5. Out-of-Scope Type Trace

**Status:** Implemented

At `trace` level, logs every struct, enum, function, and typedef that
is parsed from a header but excluded because it doesn't belong to a
`traverse` file. Useful for diagnosing missing types — if an expected
type isn't extracted, run with `RUST_LOG=bnd_winmd=trace` and search
for it:

```
TRACE  skipping out-of-scope type  kind="enum" name="fs_value_type" file="/usr/include/linux/fs_context.h"
```

This reveals whether the type was seen by clang (in a parsed header)
but excluded by the traverse filter, or whether it wasn't parsed at
all (header not in `headers` list).

```sh
RUST_LOG=bnd_winmd=trace bnd-winmd config.toml 2>&1 | grep "out-of-scope" | grep "my_type"
```

Forward declarations are not traced (they are silently skipped before
the scope check).

