#[cfg(feature = "posix_types")]
windows_link::link!("c" "C" fn sendfile(__out_fd : i32, __in_fd : i32, __offset : *mut super::super::posix::types::off_t, __count : u64) -> super::super::posix::types::ssize_t);
pub const _SYS_SENDFILE_H: i32 = 1;
