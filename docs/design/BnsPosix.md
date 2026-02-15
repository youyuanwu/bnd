# bns-posix — POSIX System Bindings via WinMD

`bns-posix` provides Rust bindings for POSIX file I/O and process APIs on
Linux, generated from C system headers through the
**bindscrape → WinMD → windows-bindgen** pipeline.

This is the first *product* crate built on bindscrape, demonstrating that the
C-header-to-WinMD approach scales beyond test fixtures to real system APIs.

## Modules

| Module | Header(s) | Functions | Constants | Structs |
|---|---|---|---|---|
| `PosixFile::Dirent` | `dirent.h`, `bits/dirent.h` | 12 | ~11 | `dirent` |
| `PosixFile::Fcntl`  | `fcntl.h` | 4 | ~60 | — |
| `PosixFile::Inet`   | `netinet/in.h`, `arpa/inet.h` | 20 | ~75 | `sockaddr_in`, `sockaddr_in6`, `in_addr`, `in6_addr` (+unions) |
| `PosixFile::Mmap`   | `sys/mman.h`, `bits/mman-linux.h`, `bits/mman-map-flags-generic.h` | 13 | ~62 | — |
| `PosixFile::Netdb`  | `netdb.h`, `bits/netdb.h` | 56 | ~32 | `addrinfo`, `hostent`, `servent`, `protoent`, `netent` |
| `PosixFile::Socket` | `sys/socket.h`, `bits/socket.h`, `bits/socket_type.h`, `bits/socket-constants.h` | 20 | ~102 | `sockaddr`, `sockaddr_storage`, `msghdr`, `iovec`, `cmsghdr`, `linger` |
| `PosixFile::Stat`   | `sys/stat.h`, `bits/struct_stat.h`, `bits/types/struct_timespec.h` | 17 | 4 | `stat`, `timespec` |
| `PosixFile::Unistd` | `unistd.h` | 103 | ~23 | — |

### Usage

```rust
use bns_posix::PosixFile::{Fcntl, Stat, Unistd};

// Create a file
let path = c"/tmp/example.txt";
let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
assert!(fd >= 0);

// Write
let data = b"hello";
unsafe { Unistd::write(fd, data.as_ptr().cast(), data.len() as u64) };

// Stat
let mut st = Stat::stat::default();
unsafe { Stat::fstat(fd, &mut st as *mut _ as *const _) };
assert_eq!(st.st_size, 5);

// Close
unsafe { Unistd::close(fd) };
```

## Architecture

The bindings are produced by a separate **generator crate** (`bns-posix-gen`)
and checked into the `bns-posix` source tree — there is no `build.rs`.

```
  bns-posix-gen (cargo run -p bns-posix-gen)
  ┌─────────────────────────────────────────────────────────┐
  │                                                         │
  │  bns-posix.toml ──▶ bindscrape ──▶ .winmd               │
  │                                      │                  │
  │                          windows-bindgen --package       │
  │                                      │                  │
  │                                      ▼                  │
  │                              bns-posix/src/              │
  │                              ├── PosixFile/              │
  │                              │   ├── mod.rs              │
  │                              │   ├── Fcntl/mod.rs        │
  │                              │   ├── Stat/mod.rs         │
  │                              │   └── Unistd/mod.rs       │
  │                              └── lib.rs (hand-written)   │
  └─────────────────────────────────────────────────────────┘
```

To regenerate:

```sh
cargo run -p bns-posix-gen
```

1. **bindscrape** parses `bns-posix.toml`, invokes clang on system headers,
   extracts types/functions/constants, and writes a temporary `.winmd` file.
2. **windows-bindgen `--package`** reads the `.winmd` and generates one
   `mod.rs` per namespace under `src/PosixFile/`, with `#[cfg(feature)]`
   gating on each sub-module. It also appends feature definitions to
   `Cargo.toml` after the `# generated features` marker.
3. The intermediate `.winmd` is deleted — `bns-posix` is a pure Rust crate
   with no build-time code generation.

### Why namespace modules?

Multiple partitions extract overlapping system types (`off_t`, `mode_t`,
`SEEK_SET`, etc.). Without `--flat`, windows-bindgen generates nested
`pub mod` modules per namespace, so each partition's types live in their
own module. Cross-partition references use `super::Stat::mode_t` etc.

## Partition Config

The TOML config lives at `tests/fixtures/bns-posix/bns-posix.toml`
and defines eight partitions:

| Partition | Namespace | Headers traversed |
|---|---|---|
| Dirent | `PosixFile.Dirent` | `dirent.h`, `bits/dirent.h` |
| Fcntl | `PosixFile.Fcntl` | `fcntl.h` |
| Inet | `PosixFile.Inet` | `netinet/in.h`, `arpa/inet.h` |
| Mmap | `PosixFile.Mmap` | `sys/mman.h`, `bits/mman-linux.h`, `bits/mman-map-flags-generic.h` |
| Netdb | `PosixFile.Netdb` | `netdb.h`, `bits/netdb.h` |
| Socket | `PosixFile.Socket` | `sys/socket.h`, `bits/socket.h`, `bits/socket_type.h`, `bits/socket-constants.h`, `bits/types/struct_iovec.h` |
| Stat | `PosixFile.Stat` | `sys/stat.h`, `bits/struct_stat.h`, `bits/types/struct_timespec.h` |
| Unistd | `PosixFile.Unistd` | `unistd.h` |

## Challenges Solved

These are issues encountered while building real system bindings and fixed
in bindscrape core (see [bns-posix.md](systesting/bns-posix.md) for details):

1. **System typedef resolution** — `CType::Named { resolved }` carries
   clang's canonical type; no hardcoded table.
2. **Variadic function skipping** — `printf`, `open`, etc. skipped with warning.
3. **LP64 `long` → `I64`** — C `long` is 8 bytes on Linux x86-64.
4. **Array parameter decay** — `const struct timespec t[2]` → pointer
   (avoids `ELEMENT_TYPE_ARRAY` blob incompatibility with windows-bindgen).
5. **Function deduplication** — glibc `__REDIRECT` macros create duplicate
   declarations; deduplicated via `HashSet<String>`.
6. **Cross-partition overlap** — namespace modules prevent duplicate
   definitions of `off_t`, `SEEK_SET`, etc.
7. **Hex/octal constant extraction** — `parse_hex_or_suffixed_int()` handles
   `0x` hex, `0` octal, and `U`/`L`/`UL`/`ULL` suffixes. Found when adding
   Mmap partition (`PROT_READ 0x1`, `MAP_SHARED 0x01` were silently dropped).
8. **PtrConst mid-chain panic** — `PtrMut(PtrConst(Named, 1), 1)` puts
   `ELEMENT_TYPE_CMOD_REQD` mid-chain in pointer blobs, crashing
   windows-bindgen. Fix: always emit `PtrMut`; const-ness tracked by
   `ConstAttribute`. Found when adding Dirent partition.
9. **Anonymous enum → constants** — unnamed C enums (e.g. `DT_*` in
   `dirent.h`) generate invalid Rust type names. Fix: detect anonymous
   enums and emit variants as standalone constants.
10. **Opaque typedef to void** — `typedef struct __dirstream DIR` maps to
    `CType::Void` which emits `c_void` (not `Copy`/`Clone`). Fix: emit
    `isize` for void-underlying typedefs.
11. **`bits/` sub-header traversal** — socket constants (`AF_*`, `SOCK_*`,
    `SOL_*`) live in `bits/socket.h`, `bits/socket_type.h`, and
    `bits/socket-constants.h`. `struct iovec` is in
    `bits/types/struct_iovec.h`, `struct netent` in `bits/netdb.h`.
    Traverse lists must include these sub-headers or types are missing
    and windows-bindgen panics with `type not found`.
12. **Cross-partition type references** — `recv`/`send` use
    `super::Unistd::ssize_t`; `addrinfo` uses `super::Socket::sockaddr`.
    windows-bindgen gates these with `#[cfg(feature = "X")]` automatically.
13. **`htons`/`htonl` as real symbols** — on glibc x86-64, `htons`/`htonl`
    have real symbols in `libc.so` (weak symbols), so P/Invoke works.

## Extending

To add more POSIX APIs (e.g., `sys/socket.h`, `pthread.h`):

1. Add a new `[[partition]]` to `bns-posix.toml` with the desired headers.
2. Run `cargo run -p bns-posix-gen` — bindscrape extracts the new partition,
   windows-bindgen adds a new `src/PosixFile/<Name>/mod.rs` and appends
   the feature to `Cargo.toml`.
3. Add the new feature to the `default` list in `Cargo.toml`.
4. `lib.rs` already does `pub mod PosixFile;` which picks up new sub-modules
   automatically.

## Tests

The crate includes 62 integration tests across 7 test files in `tests/`
that call real libc functions through the generated bindings:

| File | Tests | Partition |
|---|---|---|
| `posixfile_e2e.rs` | 11 | Fcntl + Unistd (file I/O, constants, syscalls) |
| `stat_e2e.rs` | 4 | Stat (file size, mode, struct layout) |
| `mmap_e2e.rs` | 5 | Mmap (PROT_*/MAP_*/MS_* constants, mmap roundtrip, mprotect) |
| `dirent_e2e.rs` | 5 | Dirent (DT_* constants, opendir/readdir/closedir, dirfd) |
| `socket_e2e.rs` | 16 | Socket (SOCK_*/PF_*/MSG_* constants, struct layout, socket/bind/listen/send/recv) |
| `inet_e2e.rs` | 11 | Inet (IPPROTO_* constants, struct layout, htons/htonl, inet_pton/ntop) |
| `netdb_e2e.rs` | 10 | Netdb (AI_*/EAI_* constants, struct layout, getprotobyname, getaddrinfo) |
