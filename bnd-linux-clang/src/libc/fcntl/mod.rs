#[cfg(feature = "types")]
windows_link::link!("c" "C" fn creat(__file : *const i8, __mode : super::types::mode_t) -> i32);
windows_link::link!("c" "C" fn fcntl(__fd : i32, __cmd : i32, ...) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn lockf(__fd : i32, __cmd : i32, __len : super::types::off_t) -> i32);
windows_link::link!("c" "C" fn open(__file : *const i8, __oflag : i32, ...) -> i32);
windows_link::link!("c" "C" fn openat(__fd : i32, __file : *const i8, __oflag : i32, ...) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn posix_fadvise(__fd : i32, __offset : super::types::off_t, __len : super::types::off_t, __advise : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn posix_fallocate(__fd : i32, __offset : super::types::off_t, __len : super::types::off_t) -> i32);
pub const AT_EACCESS: i32 = 512;
pub const AT_FDCWD: i32 = -100;
pub const AT_REMOVEDIR: i32 = 512;
pub const AT_SYMLINK_FOLLOW: i32 = 1024;
pub const AT_SYMLINK_NOFOLLOW: i32 = 256;
pub const F_GETLK64: i32 = 5;
pub const F_LOCK: i32 = 1;
pub const F_OK: i32 = 0;
pub const F_SETLK64: i32 = 6;
pub const F_SETLKW64: i32 = 7;
pub const F_TEST: i32 = 3;
pub const F_TLOCK: i32 = 2;
pub const F_ULOCK: i32 = 0;
pub const R_OK: i32 = 4;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const SEEK_SET: i32 = 0;
pub const S_IFBLK: i32 = 24576;
pub const S_IFCHR: i32 = 8192;
pub const S_IFDIR: i32 = 16384;
pub const S_IFIFO: i32 = 4096;
pub const S_IFLNK: i32 = 40960;
pub const S_IFMT: i32 = 61440;
pub const S_IFREG: i32 = 32768;
pub const S_IFSOCK: i32 = 49152;
pub const S_IRGRP: i32 = 32;
pub const S_IROTH: i32 = 4;
pub const S_IRUSR: i32 = 256;
pub const S_IRWXG: i32 = 56;
pub const S_IRWXO: i32 = 7;
pub const S_IRWXU: i32 = 448;
pub const S_ISGID: i32 = 1024;
pub const S_ISUID: i32 = 2048;
pub const S_ISVTX: i32 = 512;
pub const S_IWGRP: i32 = 16;
pub const S_IWOTH: i32 = 2;
pub const S_IWUSR: i32 = 128;
pub const S_IXGRP: i32 = 8;
pub const S_IXOTH: i32 = 1;
pub const S_IXUSR: i32 = 64;
pub const W_OK: i32 = 2;
pub const X_OK: i32 = 1;
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy, Default)]
pub struct flock {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: super::types::__off_t,
    pub l_len: super::types::__off_t,
    pub l_pid: super::types::__pid_t,
}
