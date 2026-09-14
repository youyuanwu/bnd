#[cfg(feature = "types")]
windows_link::link!("c" "C" fn chmod(__file : *const i8, __mode : super::types::__mode_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn fchmod(__fd : i32, __mode : super::types::__mode_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn fchmodat(__fd : i32, __file : *const i8, __mode : super::types::__mode_t, __flag : i32) -> i32);
#[cfg(all(
    feature = "struct_stat",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn fstat(__fd : i32, __buf : *mut super::struct_stat::stat) -> i32);
#[cfg(all(
    feature = "struct_stat",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn fstatat(__fd : i32, __file : *const i8, __buf : *mut super::struct_stat::stat, __flag : i32) -> i32);
#[cfg(all(feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn futimens(__fd : i32, __times : *mut super::struct_timespec::timespec) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn lchmod(__file : *const i8, __mode : super::types::__mode_t) -> i32);
#[cfg(all(
    feature = "struct_stat",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn lstat(__file : *const i8, __buf : *mut super::struct_stat::stat) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn mkdir(__path : *const i8, __mode : super::types::__mode_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn mkdirat(__fd : i32, __path : *const i8, __mode : super::types::__mode_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn mkfifo(__path : *const i8, __mode : super::types::__mode_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn mkfifoat(__fd : i32, __path : *const i8, __mode : super::types::__mode_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn mknod(__path : *const i8, __mode : super::types::__mode_t, __dev : super::types::__dev_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn mknodat(__fd : i32, __path : *const i8, __mode : super::types::__mode_t, __dev : super::types::__dev_t) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn umask(__mask : super::types::__mode_t) -> super::types::__mode_t);
#[cfg(all(feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn utimensat(__fd : i32, __path : *const i8, __times : *mut super::struct_timespec::timespec, __flags : i32) -> i32);
pub const ACCESSPERMS: i32 = 511;
pub const ALLPERMS: i32 = 4095;
pub const DEFFILEMODE: i32 = 438;
pub const S_BLKSIZE: i32 = 512;
pub const S_IEXEC: i32 = 64;
pub const S_IREAD: i32 = 256;
pub const S_IWRITE: i32 = 128;
pub const UTIME_NOW: i64 = 1073741823;
pub const UTIME_OMIT: i64 = 1073741822;
