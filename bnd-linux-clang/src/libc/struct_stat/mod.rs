#[cfg(all(feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn stat(__file : *const i8, __buf : *mut stat) -> i32);
#[repr(C)]
#[cfg(all(feature = "struct_timespec", feature = "types"))]
#[derive(Clone, Copy)]
pub struct stat {
    pub st_dev: super::types::__dev_t,
    pub st_ino: super::types::__ino_t,
    pub st_nlink: super::types::__nlink_t,
    pub st_mode: super::types::__mode_t,
    pub st_uid: super::types::__uid_t,
    pub st_gid: super::types::__gid_t,
    pub __pad0: i32,
    pub st_rdev: super::types::__dev_t,
    pub st_size: super::types::__off_t,
    pub st_blksize: super::types::__blksize_t,
    pub st_blocks: super::types::__blkcnt_t,
    pub st_atim: super::struct_timespec::timespec,
    pub st_mtim: super::struct_timespec::timespec,
    pub st_ctim: super::struct_timespec::timespec,
    pub __glibc_reserved: [super::types::__syscall_slong_t; 3],
}
#[cfg(all(feature = "struct_timespec", feature = "types"))]
impl Default for stat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
