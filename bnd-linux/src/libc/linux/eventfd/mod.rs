windows_link::link!("c" "C" fn eventfd(__count : u32, __flags : i32) -> i32);
windows_link::link!("c" "C" fn eventfd_read(__fd : i32, __value : *mut eventfd_t) -> i32);
windows_link::link!("c" "C" fn eventfd_write(__fd : i32, __value : eventfd_t) -> i32);
pub const EFD_CLOEXEC: u32 = 524288;
pub const EFD_NONBLOCK: u32 = 2048;
pub const EFD_SEMAPHORE: u32 = 1;
pub const _SYS_EVENTFD_H: i32 = 1;
pub type eventfd_t = u64;
