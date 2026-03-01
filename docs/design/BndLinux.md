# bnd-linux — POSIX and Linux System Bindings via WinMD

`bnd-linux` provides Rust bindings for **POSIX** and **Linux-specific**
system APIs, generated from C system headers through the
**bnd-winmd → WinMD → windows-bindgen** pipeline.

Both API families live in a single crate under a common `libc` root module:
- `libc::posix::*` — POSIX-standardized APIs (fcntl, unistd, socket, pthread, …)
- `libc::linux::*` — Linux-only extensions (epoll, eventfd, timerfd, …)

## Scope Boundary: POSIX vs Linux

Headers under `sys/` are **not automatically Linux-specific** — many are
POSIX-standardized. The rule:

| Header | Standard | Namespace |
|---|---|---|
| `sys/types.h` | POSIX | `libc.posix.types` |
| `sys/stat.h` | POSIX | `libc.posix.stat` |
| `sys/socket.h` | POSIX | `libc.posix.socket` |
| `sys/mman.h` | POSIX | `libc.posix.mmap` |
| `sys/epoll.h` | Linux-only | `libc.linux.epoll` |
| `sys/eventfd.h` | Linux-only | `libc.linux.eventfd` |
| `sys/timerfd.h` | Linux-only | `libc.linux.timerfd` |
| `sys/signalfd.h` | Linux-only | `libc.linux.signalfd` |
| `sys/inotify.h` | Linux-only | `libc.linux.inotify` |
| `sys/sendfile.h` | Linux-only | `libc.linux.sendfile` |
| `sys/xattr.h` | Linux-specific | `libc.linux.xattr` |
| `sys/mount.h` | Linux-specific | `libc.linux.mount` |
| `linux/types.h` | Linux kernel UAPI | `libc.linux.types` |

---

## Modules

### POSIX modules (`libc::posix::*`)

| Module | Header(s) | Functions | Constants | Structs |
|---|---|---|---|---|
| `posix::dirent` | `dirent.h`, `bits/dirent.h` | 12 | ~11 | `dirent` |
| `posix::dl`     | `dlfcn.h`, `bits/dlfcn.h` | 4 | ~8 | — |
| `posix::errno`  | `errno.h`, `bits/errno.h`, `asm-generic/errno*.h` | 1 | ~130 | — |
| `posix::fcntl`  | `fcntl.h` | 4 | ~60 | — |
| `posix::inet`   | `netinet/in.h`, `arpa/inet.h` | 20 | ~75 | `sockaddr_in`, `sockaddr_in6`, `in_addr`, `in6_addr` |
| `posix::mmap`   | `sys/mman.h`, `bits/mman-linux.h`, `bits/mman-map-flags-generic.h` | 13 | ~62 | — |
| `posix::netdb`  | `netdb.h`, `bits/netdb.h` | 56 | ~32 | `addrinfo`, `hostent`, `servent`, `protoent`, `netent` |
| `posix::pthread` | `pthread.h`, `bits/pthreadtypes.h`, … | ~90 | ~30 | `pthread_mutex_t`, `pthread_cond_t`, `pthread_rwlock_t`, … |
| `posix::sched`  | `sched.h`, `bits/sched.h`, `bits/cpu-set.h` | 10 | ~3 | `cpu_set_t`, `sched_param` |
| `posix::signal` | `signal.h`, `bits/sigaction.h`, `bits/signum-*.h`, … | 30 | ~50 | `sigaction`, `siginfo_t`, `__sigset_t`, `stack_t` |
| `posix::socket` | `sys/socket.h`, `bits/socket.h`, `bits/socket_type.h`, … | 20 | ~102 | `sockaddr`, `sockaddr_storage`, `msghdr`, `iovec`, `cmsghdr` |
| `posix::stat`   | `sys/stat.h`, `bits/struct_stat.h`, `bits/types/struct_timespec.h` | 17 | 4 | `stat`, `timespec` |
| `posix::stdio`  | `stdio.h`, `bits/stdio_lim.h`, … | 78 | 21 | `_IO_FILE`, `fpos_t`, `__mbstate_t` |
| `posix::time`   | `time.h`, `bits/time.h` | ~25 | ~12 | `tm`, `itimerspec` |
| `posix::types`  | `sys/types.h`, `bits/types.h` | — | — | `__fsid_t` + 94 shared typedefs |
| `posix::unistd` | `unistd.h` | 103 | ~23 | — |

### Linux modules (`libc::linux::*`)

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
| `linux::types` | `linux/types.h` | — | — | `__be16`, `__be32`, `__be64`, `__le*` typedefs |

### Future / Lower Priority

| Module | Header(s) | Notes |
|---|---|---|
| `linux::io_uring` | `linux/io_uring.h` | Constants + structs only (no glibc wrappers). Deeply nested anonymous unions. |
| `linux::seccomp` | `linux/seccomp.h` | ~39 constants, 5 structs. Kernel UAPI. |
| `linux::futex` | `linux/futex.h` | ~49 constants, no structs. Kernel UAPI. |

### Usage

```rust
use bnd_linux::libc::posix::{fcntl, stat, unistd};
use bnd_linux::libc::linux::{epoll, eventfd, timerfd};

// POSIX: create a file
let path = c"/tmp/example.txt";
let fd = unsafe { fcntl::creat(path.as_ptr(), 0o644) };
assert!(fd >= 0);
unsafe { unistd::close(fd) };

// Linux: create an epoll instance
let epfd = unsafe { epoll::epoll_create1(0) };
assert!(epfd >= 0);
```

---

## Architecture

The bindings are produced by a single **generator crate** (`bnd-linux-gen`)
and checked into the `bnd-linux` source tree — there is no `build.rs`.

```
  bnd-linux-gen (cargo run -p bnd-linux-gen)
  ┌─────────────────────────────────────────────────────────┐
  │                                                         │
  │  bnd-linux.toml ──▶ bnd-winmd ──▶ .winmd               │
  │                                      │                  │
  │                          windows-bindgen --package       │
  │                                      │                  │
  │                                      ▼                  │
  │                              bnd-linux/src/libc/         │
  │                              ├── posix/                  │
  │                              │   ├── mod.rs              │
  │                              │   ├── fcntl/mod.rs        │
  │                              │   ├── stat/mod.rs         │
  │                              │   └── ...                 │
  │                              ├── linux/                  │
  │                              │   ├── mod.rs              │
  │                              │   ├── epoll/mod.rs        │
  │                              │   ├── timerfd/mod.rs      │
  │                              │   └── ...                 │
  │                              └── mod.rs                  │
  └─────────────────────────────────────────────────────────┘
```

To regenerate:

```sh
cargo run -p bnd-linux-gen
```

1. **bnd-winmd** parses `bnd-linux.toml`, invokes clang on system headers,
   extracts types/functions/constants, and writes a `.winmd` file containing
   both `libc.posix.*` and `libc.linux.*` namespaces.
2. **windows-bindgen `--package`** reads the `.winmd` and generates module
   trees under `src/libc/posix/` and `src/libc/linux/`, with `#[cfg(feature)]`
   gating on each sub-module. It also appends feature definitions to
   `Cargo.toml` after the `# generated features` marker.
3. The intermediate `.winmd` is preserved in `bnd-linux/winmd/bnd-linux.winmd`
   so that downstream crates (e.g. `bnd-openssl-gen`) can import types via
   cross-WinMD references.

### Root namespace: `libc`

All partitions use a `libc` root namespace (e.g. `libc.posix.signal`,
`libc.linux.epoll`). This is required because `windows-bindgen`'s
`--package` mode expects a single-root namespace tree for feature
generation. The `feature()` method strips the root, producing features
like `posix_signal` and `linux_epoll`.

### Cross-partition type references

Since both POSIX and Linux partitions share a single winmd, cross-partition
type references resolve automatically. Linux APIs that use POSIX types
generate relative paths:

| Linux API | POSIX type used | Generated path |
|---|---|---|
| `epoll_pwait` | `__sigset_t` | `super::super::posix::pthread::__sigset_t` |
| `timerfd_create` | `clockid_t` | `super::super::posix::types::__clockid_t` |
| `timerfd_settime` | `itimerspec` | `super::super::posix::time::itimerspec` |
| `sendfile` | `off_t`, `ssize_t` | `super::super::posix::types::off_t` |

No `[[type_import]]` or `--reference` flags are needed.

### Namespace modules and deduplication

Multiple partitions reference overlapping system types (`off_t`, `mode_t`,
`uid_t`, etc.). A dedicated **types** partition (`libc.posix.types`) owns
these shared typedefs. During generation, bnd-winmd deduplicates:
the types partition is processed first (first-writer-wins for typedefs
and structs), and later partitions' duplicate copies are removed.

### Feature flags

Features are **auto-generated by `windows-bindgen`** from the namespace
tree. No manual feature list is maintained:

```toml
[features]
Foundation = []
# generated features
linux = ["Foundation"]
linux_epoll = ["linux"]
linux_eventfd = ["linux"]
...
posix = ["Foundation"]
posix_dirent = ["posix"]
posix_signal = ["posix"]
...
```

No default features are enabled. Consumers select what they need:

```toml
[dependencies]
bnd-linux = { path = "../bnd-linux", features = ["posix_fcntl", "posix_unistd", "linux_epoll"] }
```

---

## Partition Config

The TOML config lives at `bnd-linux-gen/bnd-linux.toml` and defines
25 partitions (16 POSIX + 9 Linux). All use `library = "c"` (glibc 2.34+).

### POSIX partitions

| Partition | Namespace | Headers traversed |
|---|---|---|
| Types | `libc.posix.types` | `sys/types.h`, `bits/types.h` |
| Fcntl | `libc.posix.fcntl` | `fcntl.h`, `bits/fcntl-linux.h` |
| Unistd | `libc.posix.unistd` | `unistd.h` |
| Stat | `libc.posix.stat` | `sys/stat.h`, `bits/struct_stat.h`, `bits/types/struct_timespec.h` |
| Mmap | `libc.posix.mmap` | `sys/mman.h`, `bits/mman-linux.h`, `bits/mman-map-flags-generic.h` |
| Dirent | `libc.posix.dirent` | `dirent.h`, `bits/dirent.h` |
| Socket | `libc.posix.socket` | `sys/socket.h`, `bits/socket.h`, `bits/socket_type.h`, `bits/socket-constants.h`, `bits/types/struct_iovec.h` |
| Inet | `libc.posix.inet` | `netinet/in.h`, `arpa/inet.h` |
| Netdb | `libc.posix.netdb` | `netdb.h`, `bits/netdb.h` |
| Signal | `libc.posix.signal` | `signal.h`, `bits/sigaction.h`, `bits/signum-generic.h`, `bits/signum-arch.h`, `bits/sigcontext.h`, `bits/types/__sigset_t.h`, `bits/types/siginfo_t.h`, `bits/types/__sigval_t.h`, `bits/types/stack_t.h`, `bits/types/struct_sigstack.h` |
| Dl | `libc.posix.dl` | `dlfcn.h`, `bits/dlfcn.h` |
| Errno | `libc.posix.errno` | `errno.h`, `bits/errno.h`, `linux/errno.h`, `asm/errno.h`, `asm-generic/errno.h`, `asm-generic/errno-base.h` |
| Sched | `libc.posix.sched` | `sched.h`, `bits/sched.h`, `bits/types/struct_sched_param.h`, `bits/cpu-set.h` |
| Time | `libc.posix.time` | `time.h`, `bits/time.h`, `bits/types/clock_t.h`, `bits/types/struct_tm.h`, `bits/types/clockid_t.h`, `bits/types/timer_t.h`, `bits/types/struct_itimerspec.h`, `bits/types/locale_t.h`, `bits/types/__locale_t.h` |
| Pthread | `libc.posix.pthread` | `pthread.h`, `bits/pthreadtypes.h`, `bits/thread-shared-types.h`, `bits/pthreadtypes-arch.h`, `bits/atomic_wide_counter.h`, `bits/struct_mutex.h`, `bits/struct_rwlock.h`, `bits/types/__sigset_t.h`, `bits/types/struct___jmp_buf_tag.h`, `bits/pthread_stack_min-dynamic.h`, `bits/pthread_stack_min.h` |
| Stdio | `libc.posix.stdio` | `stdio.h`, `bits/stdio_lim.h`, `bits/types/__fpos_t.h`, `bits/types/__mbstate_t.h`, `bits/types/struct_FILE.h`, `bits/types/cookie_io_functions_t.h` |

### Linux partitions

| Partition | Namespace | Headers traversed | Notes |
|---|---|---|---|
| Epoll | `libc.linux.epoll` | `sys/epoll.h`, `bits/epoll.h` | |
| Eventfd | `libc.linux.eventfd` | `sys/eventfd.h`, `bits/eventfd.h` | |
| Timerfd | `libc.linux.timerfd` | `sys/timerfd.h`, `bits/timerfd.h` | Uses `posix.time::itimerspec` |
| Signalfd | `libc.linux.signalfd` | `sys/signalfd.h`, `bits/signalfd.h` | Uses `posix.signal::__sigset_t` |
| Inotify | `libc.linux.inotify` | `sys/inotify.h` | Flexible array member limitation |
| Sendfile | `libc.linux.sendfile` | `sys/sendfile.h` | Uses `posix.types::off_t` |
| Xattr | `libc.linux.xattr` | `sys/xattr.h` | |
| Mount | `libc.linux.mount` | `sys/mount.h` | `clang_args = ["-D_LINUX_MOUNT_H"]` |
| Types | `libc.linux.types` | `linux/types.h` | `__be16`/`__be32`/`__be64`/`__le*` typedefs |

---

## Crate Layout

```
bnd-linux-gen/                    ← generator (cargo run -p bnd-linux-gen)
├── Cargo.toml
├── bnd-linux.toml                ← merged multi-partition config (25 partitions)
├── src/
│   ├── lib.rs                    ← generate(&Path) → bnd-winmd → windows-bindgen --package
│   └── main.rs
└── tests/
    └── up_to_date.rs             ← regen-to-tempdir and diff

bnd-linux/                        ← product crate
├── Cargo.toml                    ← hand-written header + auto-generated features
├── winmd/
│   └── bnd-linux.winmd           ← preserved for downstream cross-WinMD refs
├── src/
│   ├── lib.rs                    ← hand-written: pub mod libc;
│   └── libc/                     ← GENERATED by windows-bindgen --package
│       ├── mod.rs
│       ├── posix/
│       │   ├── mod.rs
│       │   ├── fcntl/mod.rs
│       │   ├── stat/mod.rs
│       │   └── ...
│       └── linux/
│           ├── mod.rs
│           ├── epoll/mod.rs
│           ├── timerfd/mod.rs
│           └── ...

tests/bnd-linux-tests/            ← E2E tests (separate crate, all features enabled)
├── Cargo.toml
├── src/lib.rs
└── tests/
    ├── posixfile_e2e.rs
    ├── stat_e2e.rs
    ├── epoll_e2e.rs
    ├── timerfd_e2e.rs
    └── ...
```

---

## Tests

Tests live in a separate `tests/bnd-linux-tests` crate which depends on
`bnd-linux` with all features enabled. This avoids feature-gating issues
since `bnd-linux` has no default features.

### POSIX tests

| File | Partition |
|---|---|
| `posixfile_e2e.rs` | Fcntl + Unistd (file I/O, constants, syscalls) |
| `stat_e2e.rs` | Stat (file size, mode, struct layout) |
| `mmap_e2e.rs` | Mmap (PROT_*/MAP_*/MS_* constants, mmap roundtrip, mprotect) |
| `dirent_e2e.rs` | Dirent (DT_* constants, opendir/readdir/closedir, dirfd) |
| `socket_e2e.rs` | Socket (SOCK_*/PF_*/MSG_* constants, struct layout, socket/bind/listen/send/recv) |
| `inet_e2e.rs` | Inet (IPPROTO_* constants, struct layout, htons/htonl, inet_pton/ntop) |
| `netdb_e2e.rs` | Netdb (AI_*/EAI_* constants, struct layout, getprotobyname, getaddrinfo) |
| `signal_e2e.rs` | Signal (SIG_*/SA_* constants, struct layout, sigset ops, sigaction, raise) |
| `dl_e2e.rs` | Dlfcn (RTLD_* constants, dlopen/dlclose, dlsym lookup, dlerror) |
| `errno_e2e.rs` | Errno (E* constants, __errno_location pointer, set/read) |
| `sched_e2e.rs` | Sched (SCHED_* constants, sched_yield, priority range, struct layout) |
| `time_e2e.rs` | Time (CLOCK_* constants, clock_gettime, gmtime, mktime roundtrip) |
| `pthread_e2e.rs` | Pthread (PTHREAD_* constants, mutex, rwlock, spinlock, TLS, struct sizes) |
| `stdio_e2e.rs` | Stdio (BUFSIZ/EOF constants, fopen/fclose, fread/fwrite, fseek/ftell, popen) |

### Linux tests

| File | Partition |
|---|---|
| `epoll_e2e.rs` | Epoll (epoll_create1, epoll_ctl, epoll_wait, EPOLL* constants) |
| `eventfd_e2e.rs` | Eventfd (eventfd create, write/read roundtrip, EFD_* constants) |
| `timerfd_e2e.rs` | Timerfd (timerfd_create, settime/gettime roundtrip, TFD_* constants) |
| `signalfd_e2e.rs` | Signalfd (signalfd create, struct size, SFD_* constants) |
| `inotify_e2e.rs` | Inotify (inotify_init1, add/rm watch, IN_* constants) |
| `sendfile_e2e.rs` | Sendfile (sendfile between fds) |
| `xattr_e2e.rs` | Xattr (set/get/list/remove roundtrip, XATTR_* constants) |
| `mount_e2e.rs` | Mount (MS_* constants, mount_attr struct size) |

---

## Challenges Solved

### POSIX-specific

1. **System typedef resolution** — `CType::Named { resolved }` carries
   clang's canonical type; no hardcoded table.
2. **Variadic function skipping** — `printf`, `open`, etc. skipped with warning.
3. **LP64 `long` → `I64`** — C `long` is 8 bytes on Linux x86-64.
4. **Array parameter decay** — `const struct timespec t[2]` → pointer.
5. **Function deduplication** — glibc `__REDIRECT` macros create duplicate
   declarations; deduplicated via `HashSet<String>`.
6. **Cross-partition overlap** — namespace modules prevent duplicate definitions.
7. **Hex/octal constant extraction** — `parse_hex_or_suffixed_int()` handles
   `0x` hex, `0` octal, and `U`/`L`/`UL`/`ULL` suffixes.
8. **PtrConst mid-chain panic** — always emit `PtrMut`; mutability via
   `ParamAttributes::Out`.
9. **Anonymous enum → constants** — unnamed C enums emit variants as standalone constants.
10. **Opaque typedef to void** — emit `isize` for void-underlying typedefs.
11. **`bits/` sub-header traversal** — traverse lists must include sub-headers.
12. **`__va_list_tag` compiler built-in** — mapped to `CType::Void`.

### Linux-specific

13. **`epoll_data_t` union inside packed struct** — exercises union-in-struct
    and packed struct layout.
14. **`inotify_event` flexible array member** — known limitation; struct size
    mismatch. Functions and constants are still useful.
15. **Mount header conflict** — `clang_args = ["-D_LINUX_MOUNT_H"]` prevents
    `linux/mount.h` from hiding `struct mount_attr` and introducing duplicate
    constants.
16. **glibc `__REDIRECT` duplicates for timerfd** — handled by existing
    function deduplication.

---

## How to Add a New Partition

### 1. Identify the API surface

```sh
grep -E "^extern" /usr/include/<header>.h
clang -E -H /usr/include/<header>.h 2>&1 | head -80
```

### 2. Add a `[[partition]]` to `bnd-linux.toml`

```toml
[[partition]]
namespace = "libc.<posix|linux>.<name>"
library = "c"
headers = ["<header>.h"]
traverse = [
    "<header>.h",
    "bits/<sub-header>.h",
]
```

The POSIX `types` partition must remain first (first-writer-wins dedup).

### 3. Run the generator

```sh
cargo run -p bnd-linux-gen
```

This produces:
- `bnd-linux/src/libc/<posix|linux>/<name>/mod.rs`
- Updated feature list in `bnd-linux/Cargo.toml`

### 4. Write E2E tests

Create `tests/bnd-linux-tests/tests/<name>_e2e.rs` and add
the corresponding feature to `tests/bnd-linux-tests/Cargo.toml`:

```rust
use bnd_linux::libc::<posix|linux>::<name>;

#[test]
fn <name>_constants() {
    assert_eq!(<name>::SOME_CONST, expected_value);
}
```

### 5. Update `lib.rs` documentation

Add a line to the module list in `bnd-linux/src/lib.rs`.

### Troubleshooting

| Symptom | Cause | Fix |
|---|---|---|
| `type not found` panic | Struct/typedef in sub-header not in traverse | Add sub-header to traverse list |
| Function missing | Variadic, inline, or in untraversed header | Check with `grep`; add header |
| Cross-partition compile error | Missing feature dependency | Enable both features |
| Struct size mismatch | Missing bitfield or flexible array member | Check C `sizeof` |

---

## Dependency Graph

```
bnd-linux-gen
    │
    ▼
bnd-linux.winmd (libc.posix.* + libc.linux.*)
    │
    ▼
bnd-linux (crate)
    │
    ▼
bnd-openssl-gen ──▶ reads bnd-linux.winmd
                     --reference bnd_linux,full,libc
    │
    ▼
bnd-openssl (crate) ──▶ depends on bnd-linux
```

---

## History

This crate was formed by merging the original `bnd-posix` and `bnd-linux`
crates. Key changes:

- Two separate winmd files → single `bnd-linux.winmd` with `libc` root namespace
- `[[type_import]]` and `--reference bnd_posix` → eliminated (same-winmd resolution)
- Features auto-generated by `windows-bindgen` (`posix_signal`, `linux_epoll`, …)
- `bnd-posix` and `bnd-posix-gen` removed from workspace
- `bnd-openssl` updated to depend on `bnd-linux` with `--reference bnd_linux,full,libc`
