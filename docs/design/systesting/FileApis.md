# System Header E2E Testing: POSIX File I/O

## Goal

Validate bindscrape against **POSIX file I/O headers** — `<fcntl.h>`,
`<unistd.h>`, and `<sys/stat.h>`. This exercises a fundamentally different
API surface than zlib: many new system typedefs (`mode_t`, `uid_t`, `pid_t`,
`time_t`, etc.), variadic functions (`open`), large/complex structs
(`struct stat`), and a dense `#define` constant space (`O_RDONLY`,
`S_IRUSR`, etc.).

---

## Why File I/O

- **Always available** — no additional `-dev` package needed
- **Many new system typedefs**: `mode_t`, `uid_t`, `gid_t`, `pid_t`,
  `time_t`, `dev_t`, `ino_t`, `nlink_t`, `blksize_t`, `blkcnt_t` — all
  must flow through `map_system_typedef()` in the emit layer
- **Variadic function**: `open(const char *path, int flags, ...)` — not
  currently handled by bindscrape; will surface whether this needs
  explicit support or warn-and-skip
- **Large struct**: `struct stat` has 13+ fields with mixed typedef types,
  potential bitfield packing on some platforms
- **Dense `#define` constants**: `O_RDONLY`, `O_WRONLY`, `O_CREAT`,
  `O_TRUNC`, `S_IRUSR`, `S_IWUSR`, `S_IRGRP`, etc. — tests the constant
  extraction at scale
- **Straightforward E2E testing**: `open`/`write`/`read`/`close`/`stat`
  on a temp file is deterministic and safe

---

## Headers & Partitions

### Headers Involved

| Header | Key declarations |
|---|---|
| `<sys/types.h>` | `mode_t`, `uid_t`, `gid_t`, `pid_t`, `off_t`, `dev_t`, `ino_t`, `nlink_t`, `blksize_t`, `blkcnt_t`, `time_t`, `ssize_t` |
| `<fcntl.h>` | `open()`, `creat()`, `fcntl()`, `O_RDONLY`, `O_WRONLY`, `O_RDWR`, `O_CREAT`, `O_TRUNC`, `O_APPEND`, `O_EXCL`, `O_NONBLOCK`, `AT_FDCWD` |
| `<unistd.h>` | `read()`, `write()`, `close()`, `lseek()`, `ftruncate()`, `unlink()`, `access()`, `getpid()`, `STDIN_FILENO`, `SEEK_SET`, `SEEK_CUR`, `SEEK_END`, `R_OK`, `W_OK`, `F_OK` |
| `<sys/stat.h>` | `struct stat`, `stat()`, `fstat()`, `lstat()`, `chmod()`, `mkdir()`, `S_IRUSR`, `S_IWUSR`, `S_IXUSR`, `S_IRGRP`, `S_IRWXU`, `S_ISREG()`, `S_ISDIR()` |

### Proposed Partition Layout

```toml
[output]
name = "PosixFile"
file = "posixfile.winmd"

# Partition 1: base typedefs from sys/types.h
[[partition]]
namespace = "PosixFile.Types"
library = "c"
headers = ["/usr/include/sys/types.h"]
traverse = ["/usr/include/sys/types.h"]

# Partition 2: fcntl — open/creat + O_* flags
[[partition]]
namespace = "PosixFile.Fcntl"
library = "c"
headers = ["/usr/include/fcntl.h"]
traverse = ["/usr/include/fcntl.h"]

# Partition 3: unistd — read/write/close/lseek
[[partition]]
namespace = "PosixFile.Unistd"
library = "c"
headers = ["/usr/include/unistd.h"]
traverse = ["/usr/include/unistd.h"]

# Partition 4: sys/stat — struct stat + stat/fstat/chmod
[[partition]]
namespace = "PosixFile.Stat"
library = "c"
headers = ["/usr/include/sys/stat.h"]
traverse = ["/usr/include/sys/stat.h"]
```

Key points:
- **`library = "c"`** — functions live in libc (`libc.so.6`), linked as `-lc`
- **Multi-partition** — mirrors the zlib pattern; `sys/types.h` typedefs
  go to `PosixFile.Types`, function-bearing headers get their own namespaces
- **No extra packages** — these headers are part of the base system

### Alternative: Fewer Partitions

A simpler 2-partition layout may be sufficient if per-header namespacing
isn't important:

```toml
# Partition 1: sys/types.h typedefs
[[partition]]
namespace = "PosixFile.Types"
library = "c"
headers = ["/usr/include/sys/types.h"]
traverse = ["/usr/include/sys/types.h"]

# Partition 2: all file API functions & structs
[[partition]]
namespace = "PosixFile"
library = "c"
headers = ["/usr/include/fcntl.h", "/usr/include/unistd.h", "/usr/include/sys/stat.h"]
traverse = ["/usr/include/fcntl.h", "/usr/include/unistd.h", "/usr/include/sys/stat.h"]
```

---

## New System Typedefs Required

These typedefs will appear in `struct stat` fields and function signatures.
All must be added to `map_system_typedef()` in `emit.rs`:

| Typedef | Canonical type | Winmd mapping |
|---|---|---|
| `mode_t` | `unsigned int` | `U32` |
| `uid_t` | `unsigned int` | `U32` |
| `gid_t` | `unsigned int` | `U32` |
| `pid_t` | `int` | `I32` |
| `time_t` | `long` (64-bit on Linux x64) | `I64` |
| `dev_t` | `unsigned long` | `U64` |
| `ino_t` | `unsigned long` | `U64` |
| `nlink_t` | `unsigned long` | `U64` |
| `blksize_t` | `long` | `I64` |
| `blkcnt_t` | `long` | `I64` |
| `clockid_t` | `int` | `I32` |

Note: These sizes are **Linux x86-64**. The winmd targets Windows ABI
equivalence, so mapping may need annotation. For now, follow the same
pattern as `off_t` → `I64`.

---

## Expected Challenges

### 1. Variadic functions (`open`)

`open(const char *pathname, int flags, ...)` is variadic. bindscrape
currently has no handling for variadic functions — `clang::Type::is_variadic()`
returns `true` but the extracted `FunctionDef` doesn't model the `...`.

**Options**:
- **Warn and skip**: Log a warning and omit variadic functions entirely.
  `open` would need a non-variadic wrapper or manual binding.
- **Emit without the variadic part**: Emit `open(pathname, flags)` as a
  2-param P/Invoke. The caller would pass the `mode_t` via a separate
  overload or cast. This is what many bindgen-based projects do.
- **Emit two overloads**: `open(path, flags)` and `open(path, flags, mode)`.
  Requires detecting that the common variadic arg is `mode_t`.

Recommendation: **warn and skip** for v1. Variadic P/Invoke is not
well-supported in ECMA-335 metadata. The E2E tests can use `creat()` or
a non-variadic wrapper instead.

### 2. `struct stat` complexity

`struct stat` on Linux x86-64 has ~13 fields with platform-specific
padding and reserved fields (`__pad0`, `__unused`, `__glibc_reserved`).
Some glibc versions define it via macro expansion or conditional
compilation. clang should resolve the final layout, but field names
may include glibc-internal prefixes.

### 3. Header nesting depth

`<fcntl.h>` transitively includes `<bits/fcntl-linux.h>`, `<bits/types.h>`,
etc. The traverse filter must correctly restrict to the top-level header.
May need to check whether `O_RDONLY` and similar constants are actually
declared in `<fcntl.h>` itself or in a `<bits/>` sub-header — if the
latter, the traverse list may need adjustment.

### 4. `S_ISREG` / `S_ISDIR` — function-like macros

These are `#define S_ISREG(m) (((m) & S_IFMT) == S_IFREG)` — not
extractable as constants. They would need to be either skipped or
provided as helper functions in a hand-written wrapper.

### 5. Inline functions in headers

`<unistd.h>` may contain `static inline` functions (glibc versions
vary). These would be extracted as regular functions but have no symbol
in `libc.so`. The P/Invoke would fail at runtime. Need to detect
`EntityKind::FunctionDecl` with `is_inline()` and skip them.

### 6. `__` prefixed internal typedefs

glibc internally uses `__mode_t`, `__uid_t`, `__pid_t` etc. These may
appear in struct fields if clang resolves through the typedef chain.
`map_system_typedef()` should handle both `mode_t` and `__mode_t`.

---

## API Surface (Expected)

### PosixFile.Types (sys/types.h)

~12 typedefs (`mode_t`, `uid_t`, `gid_t`, `pid_t`, `off_t`, `dev_t`,
`ino_t`, `nlink_t`, `blksize_t`, `blkcnt_t`, `time_t`, `ssize_t`).

### PosixFile.Fcntl (fcntl.h)

**Functions**: `open`, `creat`, `fcntl`, `openat`, ...
**Constants**: `O_RDONLY`, `O_WRONLY`, `O_RDWR`, `O_CREAT`, `O_TRUNC`,
`O_APPEND`, `O_EXCL`, `O_NONBLOCK`, `AT_FDCWD`, ...

### PosixFile.Unistd (unistd.h)

**Functions**: `read`, `write`, `close`, `lseek`, `ftruncate`, `unlink`,
`access`, `getpid`, `dup`, `dup2`, `pipe`, `fsync`, ...
**Constants**: `STDIN_FILENO`, `STDOUT_FILENO`, `STDERR_FILENO`,
`SEEK_SET`, `SEEK_CUR`, `SEEK_END`, `R_OK`, `W_OK`, `X_OK`, `F_OK`, ...

### PosixFile.Stat (sys/stat.h)

**Structs**: `stat` (13+ fields, ~144 bytes on x86-64)
**Functions**: `stat`, `fstat`, `lstat`, `chmod`, `fchmod`, `mkdir`,
`mkdirat`, `umask`, ...
**Constants**: `S_IRUSR`, `S_IWUSR`, `S_IXUSR`, `S_IRWXU`, `S_IRGRP`,
`S_IWGRP`, `S_IXGRP`, `S_IRWXG`, `S_IROTH`, `S_IRWXO`, `S_IFMT`,
`S_IFREG`, `S_IFDIR`, `S_IFLNK`, ...

---

## Proposed E2E Tests

Test against real filesystem operations using temp files.

| Test | What it does |
|---|---|
| `creat_and_close` | `creat(tmppath, 0o644)` returns valid fd, `close(fd)` returns 0 |
| `write_then_read` | Write "hello" to tmpfile, lseek to start, read back, assert equal |
| `stat_file_size` | Write 13 bytes, `fstat(fd)` → `st_size == 13` |
| `stat_is_regular_file` | `fstat(fd)` → `st_mode & S_IFREG != 0` |
| `unlink_file` | `unlink(tmppath)` returns 0 |
| `lseek_returns_offset` | `lseek(fd, 10, SEEK_SET)` returns 10 |
| `access_existing_file` | `access(tmppath, F_OK)` returns 0 |
| `access_nonexistent_file` | `access("/nonexistent", F_OK)` returns -1 |
| `getpid_returns_positive` | `getpid()` > 0 |
| `o_rdonly_is_zero` | `O_RDONLY == 0` |
| `seek_set_is_zero` | `SEEK_SET == 0` |
| `s_irusr_is_0o400` | `S_IRUSR == 0o400` |

---

## Dependencies

- No additional packages — `sys/types.h`, `fcntl.h`, `unistd.h`,
  `sys/stat.h` are part of the base `libc6-dev` / `linux-libc-dev`
  install (already present if `libclang-dev` is installed)
- libc is implicitly linked — `cargo:rustc-link-lib=dylib=c` may not
  even be necessary, but explicit is safer

---

## Implementation Steps

1. Add new system typedefs to `map_system_typedef()` in `emit.rs`
2. Decide on variadic function handling (skip or partial emit)
3. Create `bindscrape/tests/fixtures/posixfile/posixfile.toml`
4. Add roundtrip tests in `roundtrip_posixfile.rs`
5. Create `tests/e2e-posixfile/` crate
6. Iterate on traverse paths (may need `bits/` sub-headers)
7. Handle inline function skipping if encountered
8. Add `e2e-posixfile` to workspace members
