# bnd-linux — Linux-Only System Bindings via WinMD

`bnd-linux` provides Rust bindings for **Linux-specific** system APIs
that are **not part of POSIX**, generated from C system headers through
the **bnd-winmd → WinMD → windows-bindgen** pipeline.

This crate complements `bnd-posix` (which covers POSIX-standardized APIs
like fcntl, unistd, socket, pthread, etc.) with Linux-only extensions:
epoll, eventfd, timerfd, signalfd, inotify, sendfile, xattr, mount,
and kernel UAPI headers like `linux/io_uring.h`.

## Scope Boundary: POSIX vs Linux

Headers under `sys/` are **not automatically Linux-specific** — many are
POSIX-standardized. The rule:

| Header | Standard | Crate |
|---|---|---|
| `sys/epoll.h` | Linux-only | **bnd-linux** |
| `sys/eventfd.h` | Linux-only | **bnd-linux** |
| `sys/timerfd.h` | Linux-only | **bnd-linux** |
| `sys/signalfd.h` | Linux-only | **bnd-linux** |
| `sys/inotify.h` | Linux-only | **bnd-linux** |
| `sys/sendfile.h` | Linux-only (different API on BSDs) | **bnd-linux** |
| `sys/xattr.h` | Linux-specific (different on macOS/FreeBSD) | **bnd-linux** |
| `sys/mount.h` | Linux-specific (no POSIX mount API) | **bnd-linux** |
| `sys/prctl.h` | Linux-only (variadic — auto-skipped) | **bnd-linux** |
| `linux/io_uring.h` | Linux kernel UAPI | **bnd-linux** |
| `sys/utsname.h` | POSIX | bnd-posix |
| `sys/resource.h` | POSIX | bnd-posix |
| `sys/wait.h` | POSIX | bnd-posix |
| `poll.h` | POSIX | bnd-posix |

---

## Modules

| Module | Header(s) | Functions | Constants | Structs |
|---|---|---|---|---|
| `linux::epoll` | `sys/epoll.h`, `bits/epoll.h` | 6 | 20 | `epoll_event`, `epoll_data_t` (union) |
| `linux::eventfd` | `sys/eventfd.h`, `bits/eventfd.h` | 3 | 4 | — (typedef `eventfd_t = u64`) |
| `linux::timerfd` | `sys/timerfd.h`, `bits/timerfd.h` | 3 | 5 | — (uses `posix::time::itimerspec`) |
| `linux::signalfd` | `sys/signalfd.h`, `bits/signalfd.h` | 1 | 3 | `signalfd_siginfo` |
| `linux::inotify` | `sys/inotify.h` | 4 | 23 | `inotify_event` (flexible array member) |
| `linux::sendfile` | `sys/sendfile.h` | 1 | 1 | — |
| `linux::xattr` | `sys/xattr.h` | 12 | 3 | — |
| `linux::mount` | `sys/mount.h` | 10 | 71 | `mount_attr` |

### Future / Lower Priority

| Module | Header(s) | Notes |
|---|---|---|
| `linux::io_uring` | `linux/io_uring.h` | Constants + structs only (no glibc function wrappers). 20 structs with deeply nested anonymous unions, ~94 defines, 9 enums. Uses kernel types (`__u8`, `__u32`, `__u64`). |
| `linux::seccomp` | `linux/seccomp.h` | ~39 constants, 5 structs. Kernel UAPI. |
| `linux::futex` | `linux/futex.h` | ~49 constants, no structs. Kernel UAPI. |

### Usage

```rust
use bnd_linux::linux::{epoll, eventfd, timerfd};

// Create an epoll instance
let epfd = unsafe { epoll::epoll_create1(0) };
assert!(epfd >= 0);

// Create an eventfd
let efd = unsafe { eventfd::eventfd(0, eventfd::EFD_NONBLOCK as i32) };
assert!(efd >= 0);

// Create a timerfd
let tfd = unsafe { timerfd::timerfd_create(1 /* CLOCK_MONOTONIC */, 0) };
assert!(tfd >= 0);
```

---

## Architecture

The bindings follow the same **generator + product crate** pattern as
`bnd-posix` and `bnd-openssl`. A separate generator crate
(`bnd-linux-gen`) runs bnd-winmd + windows-bindgen `--package` offline,
producing a checked-in `bnd-linux/` source tree — there is no `build.rs`
for codegen.

```
  bnd-linux-gen (cargo run -p bnd-linux-gen)
  ┌─────────────────────────────────────────────────────────┐
  │                                                         │
  │  linux.toml ──▶ bnd-winmd ──▶ .winmd                   │
  │                                   │                     │
  │                     windows-bindgen --package            │
  │                                   │                     │
  │                                   ▼                     │
  │                           bnd-linux/src/                 │
  │                           ├── linux/                     │
  │                           │   ├── mod.rs                 │
  │                           │   ├── epoll/mod.rs           │
  │                           │   ├── eventfd/mod.rs         │
  │                           │   ├── timerfd/mod.rs         │
  │                           │   └── ...                    │
  │                           └── lib.rs (hand-written)      │
  └─────────────────────────────────────────────────────────┘
```

To regenerate:

```sh
cargo run -p bnd-linux-gen
```

1. **bnd-winmd** parses `linux.toml`, invokes clang on system headers,
   extracts types/functions/constants, and writes a temporary `.winmd` file.
2. **windows-bindgen `--package`** reads the `.winmd` and generates one
   `mod.rs` per namespace under `src/linux/`, with `#[cfg(feature)]`
   gating on each sub-module. The `--no-toml` flag is used to skip
   Cargo.toml feature generation (features are maintained manually)
   because cargo needs features to exist before parsing the workspace,
   creating a chicken-and-egg problem. This is the same pattern used
   by `bnd-posix-gen` and `bnd-openssl-gen`.
3. The intermediate `.winmd` is preserved in `bnd-linux/winmd/bnd-linux.winmd`
   so that downstream crates can import Linux types via cross-WinMD
   references.

### Cross-WinMD Type References

Several Linux APIs reference POSIX types already defined in `bnd-posix`:

| Linux API | POSIX type needed | Source partition |
|---|---|---|
| `epoll_pwait`, `epoll_pwait2` | `__sigset_t`, `struct timespec` | `posix.signal`, `posix.stat` |
| `timerfd_create` | `clockid_t` | `posix.time` |
| `timerfd_settime`, `timerfd_gettime` | `struct itimerspec` | `posix.time` |
| `signalfd` | `sigset_t` | `posix.signal` |
| `sendfile` | `off_t` | `posix.types` |

These types are imported from `bnd-posix.winmd` via `[[type_import]]`
(same pattern as `bnd-openssl`), avoiding duplication:

```toml
[[type_import]]
winmd = "../bnd-posix/winmd/bnd-posix.winmd"
namespace = "posix"
```

The generator passes `--reference bnd_posix,full,posix` to
`windows-bindgen`, producing paths like
`bnd_posix::posix::time::itimerspec` in the generated code.

---

## Partition Config

The TOML config lives at `bnd-linux-gen/linux.toml` and defines
partitions for Linux-only APIs. All partitions use `library = "c"`
because on glibc 2.34+, these wrappers live in libc.

```toml
include_paths = ["/usr/include/x86_64-linux-gnu", "/usr/include"]

[output]
name = "linux"
file = "bnd-linux.winmd"

[[type_import]]
winmd = "../bnd-posix/winmd/bnd-posix.winmd"
namespace = "posix"
```

| Partition | Namespace | Library | Headers traversed |
|---|---|---|---|
| Epoll | `linux.epoll` | `c` | `sys/epoll.h`, `bits/epoll.h` |
| Eventfd | `linux.eventfd` | `c` | `sys/eventfd.h`, `bits/eventfd.h` |
| Timerfd | `linux.timerfd` | `c` | `sys/timerfd.h`, `bits/timerfd.h` |
| Signalfd | `linux.signalfd` | `c` | `sys/signalfd.h`, `bits/signalfd.h` |
| Inotify | `linux.inotify` | `c` | `sys/inotify.h` |
| Sendfile | `linux.sendfile` | `c` | `sys/sendfile.h` |
| Xattr | `linux.xattr` | `c` | `sys/xattr.h` |
| Mount | `linux.mount` | `c` | `sys/mount.h` |

---

## Anticipated Challenges

### 1. `epoll_data_t` union inside `epoll_event` packed struct

`epoll_event` contains a union field (`epoll_data_t`) and has
`__attribute__((packed))` on some architectures. This exercises both
the union-in-struct path and packed struct layout. bnd-winmd already
handles unions via `ExplicitLayout` (proven with `sigaction` in
bnd-posix), but the packed attribute may affect `ClassLayout` alignment.

```c
typedef union epoll_data {
    void *ptr;
    int fd;
    uint32_t u32;
    uint64_t u64;
} epoll_data_t;

struct epoll_event {
    uint32_t events;
    epoll_data_t data;
} __EPOLL_PACKED;
```

**Risk**: Low — union emission is proven; packed attribute needs
verification.

### 2. `inotify_event` flexible array member

`struct inotify_event` has `char name[]` (flexible array member):

```c
struct inotify_event {
    int wd;
    uint32_t mask;
    uint32_t cookie;
    uint32_t len;
    char name __flexarr;   /* variable-length */
};
```

This is a **known bnd-winmd limitation** (WIP.md §8: "Flexible array
member handling — `IncompleteArray` → `CType::Ptr` adds a spurious
pointer-sized field"). The emitted struct will have incorrect layout.

**Mitigation**: The inotify partition is still useful for its 4 functions
and ~27 `IN_*` constants. Document the struct size mismatch. Users who
need `inotify_event` can define it manually or use the raw buffer
approach (read bytes, parse header, then name).

### 3. Octal constant extraction

The `EFD_*`, `TFD_*`, `SFD_*` flags use octal literals:

```c
EFD_CLOEXEC  = 02000000,
EFD_NONBLOCK = 00004000
```

bnd-winmd's `parse_hex_or_suffixed_int()` already handles `0`-prefixed
octal (proven with mmap partition). Should work out of the box.

### 4. `EPOLL_EVENTS` named enum with `#define` aliases

`sys/epoll.h` defines a named enum `EPOLL_EVENTS` where each variant
has a matching `#define`:

```c
enum EPOLL_EVENTS {
    EPOLLIN = 0x001,
#define EPOLLIN EPOLLIN
    ...
};
```

bnd-winmd extracts both the enum variants and the `#define` constants.
In practice, the `#define FOO FOO` self-referential macros are resolved
by the preprocessor back to the enum value, so both the enum variant and
the `#define` produce valid integer constants. The generated bindings
extract these correctly, but the signalfd `SFD_*` and epoll `EPOLL_CLOEXEC`
constants in `bits/*.h` use the same pattern with anonymous enums.

### 5. Cross-partition POSIX type references

`epoll_pwait` uses `__sigset_t` (from `posix.signal`),
`epoll_pwait2` uses `struct timespec` (from `posix.stat`),
`timerfd_*` uses `clockid_t` and `struct itimerspec` (from `posix.time`),
`signalfd` uses `sigset_t` (from `posix.signal`),
`sendfile` uses `off_t` (from `posix.types`).

All of these are resolved via cross-WinMD type import from
`bnd-posix.winmd`, same pattern as `bnd-openssl`.

**Required `bnd-posix` features**: `signal`, `stat`, `time`, `types`.

### 6. `glibc __REDIRECT` duplicates for timerfd/mount

`timerfd_settime` and `timerfd_gettime` have `__REDIRECT` variants for
64-bit time support (same pattern seen in bnd-posix). The existing
function deduplication in `collect_functions()` handles this.

### 7. Mount API: new-style `fsopen`/`fsmount`/`fsconfig` (glibc 2.36+)

`sys/mount.h` includes both the classic `mount`/`umount`/`umount2`
functions and the newer mount API (`fsopen`, `fsmount`, `move_mount`,
`fsconfig`, `fspick`, `open_tree`, `mount_setattr`). The newer functions
may not be available on older glibc versions. The bindings will extract
whatever the system headers declare.

### 8. `prctl` is variadic

`prctl(int option, ...)` is variadic and will be auto-skipped. The
`PR_*` constants from `linux/prctl.h` (~69 defines) are still useful.
Consider a prctl partition that only provides constants, with a note
that the function itself requires manual FFI.

### 9. Mount header conflict with `linux/mount.h`

`sys/mount.h` includes `linux/mount.h`, which defines `MOUNT_ATTR_SIZE_VER0`.
This causes two problems:

- `struct mount_attr` in `sys/mount.h` is guarded by
  `#ifndef MOUNT_ATTR_SIZE_VER0`, so it becomes hidden when
  `linux/mount.h` is included first.
- `MS_*` constants are defined as both enum values (from `sys/mount.h`)
  and `#define` macros (from `linux/mount.h`), creating duplicate
  constants with non-deterministic types (`u32` vs `i32`).

**Fix**: The mount partition uses `clang_args = ["-D_LINUX_MOUNT_H"]` to
pre-define `linux/mount.h`'s include guard, preventing its inclusion.
This ensures `sys/mount.h`'s own `struct mount_attr` definition is
visible and only the enum-based `MS_*` constants are extracted.

### 10. Non-deterministic cross-WinMD type resolution

`__sigset_t` is defined in both `posix.signal` and `posix.pthread`
partitions of `bnd-posix.winmd`. When bnd-winmd seeds its TypeRegistry
from the external winmd via `[[type_import]]`, HashMap iteration order
determines which partition's definition is used. This causes
non-deterministic output for functions that reference `__sigset_t`
(e.g., `epoll_pwait`, `signalfd`).

Both paths (`posix::signal::__sigset_t` and `posix::pthread::__sigset_t`)
are functionally identical at runtime. The up-to-date freshness test in
`bnd-linux-gen` normalizes this variance during comparison.

### 11. Kernel UAPI headers (`linux/io_uring.h`)

Kernel UAPI headers define structs and constants but **no glibc function
wrappers** — the actual syscalls (`io_uring_setup`, `io_uring_enter`,
`io_uring_register`) are invoked via raw `syscall()` or through
`liburing`. Key challenges:

- **Deeply nested anonymous unions**: `struct io_uring_sqe` has 6 nested
  anonymous unions with struct-in-union patterns. bnd-winmd handles
  anonymous nested types via synthetic names (`ParentName_FieldName`),
  but this depth is unprecedented.
- **Kernel types**: `__u8`, `__u32`, `__u64`, `__s32`, `__s64` are
  typedefs from `linux/types.h` → `asm/types.h` → `asm-generic/int-ll64.h`.
  These should resolve via clang canonical types, but need verification.
- **No functions**: The partition would be structs + constants only.
  Useful for crates that do raw `syscall()` or wrap `liburing`.

**Recommendation**: Defer `io_uring` to a later phase. Start with the
glibc-wrapped APIs (epoll, eventfd, timerfd, etc.) which have proven
patterns.

---

## Crate Layout

```
bnd-linux-gen/                    ← generator (cargo run -p bnd-linux-gen)
├── Cargo.toml
├── linux.toml                    ← multi-partition config
├── src/
│   ├── lib.rs                    ← generate(&Path) → bnd-winmd → windows-bindgen --package
│   └── main.rs                   ← fn main() { generate(&bnd_linux_dir) }
└── tests/
    └── up_to_date.rs             ← regen-to-tempdir and diff

bnd-linux/                        ← product crate
├── Cargo.toml                    ← hand-written header + # generated features
├── winmd/
│   └── bnd-linux.winmd           ← preserved for downstream cross-WinMD refs
├── src/
│   ├── lib.rs                    ← hand-written: pub mod linux;
│   └── linux/                    ← GENERATED by windows-bindgen --package
│       ├── mod.rs
│       ├── epoll/mod.rs
│       ├── eventfd/mod.rs
│       ├── timerfd/mod.rs
│       ├── signalfd/mod.rs
│       ├── inotify/mod.rs
│       ├── sendfile/mod.rs
│       ├── xattr/mod.rs
│       └── mount/mod.rs
└── tests/                        ← E2E tests (one per partition)
    ├── epoll_e2e.rs
    ├── eventfd_e2e.rs
    ├── timerfd_e2e.rs
    ├── signalfd_e2e.rs
    ├── inotify_e2e.rs
    ├── sendfile_e2e.rs
    ├── xattr_e2e.rs
    └── mount_e2e.rs

```

---

## Dependencies

**`bnd-linux/Cargo.toml`**:

```toml
[package]
name = "bnd-linux"
version = "0.0.1"
edition.workspace = true

[dependencies]
bnd-posix = { path = "../bnd-posix", features = ["signal", "stat", "time", "types"] }
windows-link.workspace = true

[dev-dependencies]
libc = "0.2"

[features]
default = ["epoll", "eventfd", "inotify", "mount", "sendfile", "signalfd", "timerfd", "xattr"]
Foundation = []
# generated features (maintained manually due to --no-toml)
epoll = ["Foundation"]
eventfd = ["Foundation"]
inotify = ["Foundation"]
mount = ["Foundation"]
sendfile = ["Foundation"]
signalfd = ["Foundation"]
timerfd = ["Foundation"]
xattr = ["Foundation"]
```

Note: `libc` is a dev-dependency only — used for helper functions in
E2E tests (`close`, `write`, `sigprocmask`, etc.) that aren't part of
the bnd-linux bindings themselves.

---

## Implementation Plan

### Phase 1: Core event APIs (epoll, eventfd, timerfd, signalfd)

These are the most commonly used Linux-specific APIs and form the
foundation of async I/O on Linux. They are clean, small, and exercise
cross-WinMD type references (sigset, timespec, itimerspec, clockid_t).

1. Create `bnd-linux-gen/linux.toml` with 4 partitions (epoll, eventfd,
   timerfd, signalfd)
2. Create `bnd-linux/` product crate (Cargo.toml, src/lib.rs)
3. Create `bnd-linux-gen/` generator crate (Cargo.toml, src/lib.rs,
   src/main.rs)
4. Run generator, iterate on traverse lists
5. Add E2E tests for each partition
6. Add up-to-date test in `bnd-linux-gen/tests/up_to_date.rs`

### Phase 2: File and filesystem APIs (inotify, sendfile, xattr, mount)

Expand with filesystem-related Linux APIs. These are independent of
Phase 1 and can be developed in parallel once the crate structure
exists.

7. Add inotify, sendfile, xattr, mount partitions to `linux.toml`
8. Run generator, handle new traverse paths (mount requires `clang_args`)
9. Add E2E tests for each partition
10. Document `inotify_event` flexible array member limitation

### Phase 3: Kernel UAPI (io_uring, futex, seccomp) — Future

Kernel UAPI headers provide structs and constants but no glibc function
wrappers. These are lower priority and may stress-test bnd-winmd's
handling of deeply nested anonymous types.

11. Add `linux/io_uring.h` partition (structs + constants only)
12. Verify kernel type resolution (`__u8`, `__u32`, `__u64`)
13. Add `linux/futex.h` (constants only)
14. Add `linux/seccomp.h` (structs + constants)

---

## E2E Test Plan

### epoll

| Test | What it does |
|---|---|
| `epoll_create1_returns_valid_fd` | `epoll_create1(0)` returns fd ≥ 0 |
| `epoll_ctl_add_eventfd` | Create epoll + eventfd, `epoll_ctl(EPOLL_CTL_ADD)` succeeds |
| `epoll_wait_eventfd_readable` | Write to eventfd, `epoll_wait` returns 1 with `EPOLLIN` |
| `epoll_event_constants` | `EPOLLIN`, `EPOLLOUT`, `EPOLLERR`, `EPOLLHUP` have expected values |
| `epoll_ctl_constants` | `EPOLL_CTL_ADD == 1`, `EPOLL_CTL_DEL == 2`, `EPOLL_CTL_MOD == 3` |
| `epoll_event_struct_layout` | `size_of::<epoll_event>()` matches C `sizeof` |

### eventfd

| Test | What it does |
|---|---|
| `eventfd_create_close` | `eventfd(0, 0)` returns fd ≥ 0, close succeeds |
| `eventfd_write_read_roundtrip` | `eventfd_write(efd, 42)` then `eventfd_read(efd, &val)` returns 42 |
| `efd_nonblock_constant` | `EFD_NONBLOCK` has expected value |
| `efd_cloexec_constant` | `EFD_CLOEXEC` has expected value |
| `efd_semaphore_constant` | `EFD_SEMAPHORE` has expected value |

### timerfd

| Test | What it does |
|---|---|
| `timerfd_create_monotonic` | `timerfd_create(CLOCK_MONOTONIC, 0)` returns fd ≥ 0 |
| `timerfd_settime_gettime` | Set a timer, read it back with `timerfd_gettime` |
| `tfd_constants` | `TFD_CLOEXEC`, `TFD_NONBLOCK`, `TFD_TIMER_ABSTIME` values |

### signalfd

| Test | What it does |
|---|---|
| `signalfd_create` | Block SIGUSR1 with sigprocmask, `signalfd(-1, &mask, SFD_NONBLOCK)` returns fd ≥ 0 |
| `signalfd_siginfo_struct_size` | `size_of::<signalfd_siginfo>()` == 128 |
| `sfd_constants` | `SFD_CLOEXEC`, `SFD_NONBLOCK` values |

### inotify

| Test | What it does |
|---|---|
| `inotify_init1_returns_valid_fd` | `inotify_init1(0)` returns fd ≥ 0 |
| `inotify_add_rm_watch` | Add watch on `/tmp`, get wd ≥ 0, remove watch succeeds |
| `in_constants` | `IN_CREATE`, `IN_DELETE`, `IN_MODIFY`, `IN_MOVED_FROM`, `IN_MOVED_TO` values |

### sendfile

| Test | What it does |
|---|---|
| `sendfile_between_fds` | Write to file, `sendfile` to pipe, read back, verify content |

### xattr

| Test | What it does |
|---|---|
| `setxattr_getxattr_roundtrip` | Set `user.test` attr on tmpfile, read it back |
| `listxattr_contains_attr` | After setxattr, listxattr includes `user.test` |
| `removexattr_removes_attr` | After setxattr + removexattr, getxattr fails with ENODATA |
| `xattr_constants` | `XATTR_CREATE == 1`, `XATTR_REPLACE == 2` |

### mount

| Test | What it does |
|---|---|
| `mount_constants` | `MS_RDONLY`, `MS_NOSUID`, `MS_NODEV`, `MS_NOEXEC` values |
| `mount_attr_struct_size` | `size_of::<mount_attr>()` matches C `sizeof` |
| `mount_attr_size_ver0_matches_struct` | `MOUNT_ATTR_SIZE_VER0` equals struct size |
| `fsconfig_cmd_constants` | `FSCONFIG_SET_FLAG`, `FSCONFIG_SET_STRING`, etc. values |

Note: actual `mount`/`umount` calls require `CAP_SYS_ADMIN` and are
not suitable for unprivileged CI. Constant and struct tests only.

---

## Comparison with Existing Crates

| Aspect | bnd-posix | bnd-openssl | bnd-linux |
|---|---|---|---|
| Scope | POSIX-standardized APIs | OpenSSL 3.x crypto/TLS | Linux-only system APIs |
| Headers | glibc POSIX headers | `openssl/*.h` | `sys/epoll.h`, `sys/eventfd.h`, `linux/*.h`, etc. |
| Libraries | 1 (`libc`) | 2 (`libssl`, `libcrypto`) | 1 (`libc`) |
| Partitions | 16 | 8 | 8 (Phase 1+2) |
| Cross-WinMD refs | No | Yes (bnd-posix) | Yes (bnd-posix) |
| Key new patterns | sub-header traverse at scale | multi-library + opaque typedefs | union-in-packed-struct, kernel UAPI headers, flexible array member |
| Binding mode | `--package --sys` | `--package --sys` | `--package --sys` |

---

## Dependency Graph

```
bnd-posix-gen
    │
    ▼
bnd-posix.winmd ──────────────────┐
    │                              │
    ▼                              ▼
bnd-posix (crate)         bnd-linux-gen
                              │    reads posix.winmd for
                              │    type_import + --reference
                              ▼
                         bnd-linux.winmd
                              │
                              ▼
                         bnd-linux (crate)
                              │
                              ▼
                         depends on bnd-posix (runtime)
```

Build order: `bnd-posix-gen` must run before `bnd-linux-gen` so that
`bnd-posix.winmd` exists. The gen crates run outside `cargo build`
(manual `cargo run -p`), so a clear error message is emitted when the
referenced winmd file doesn't exist.
