windows_link::link!("c" "C" fn inotify_add_watch(__fd : i32, __name : *const i8, __mask : u32) -> i32);
windows_link::link!("c" "C" fn inotify_init() -> i32);
windows_link::link!("c" "C" fn inotify_init1(__flags : i32) -> i32);
windows_link::link!("c" "C" fn inotify_rm_watch(__fd : i32, __wd : i32) -> i32);
pub const IN_ACCESS: i32 = 1;
pub const IN_ATTRIB: i32 = 4;
pub const IN_CLOSE_NOWRITE: i32 = 16;
pub const IN_CLOSE_WRITE: i32 = 8;
pub const IN_CREATE: i32 = 256;
pub const IN_DELETE: i32 = 512;
pub const IN_DELETE_SELF: i32 = 1024;
pub const IN_DONT_FOLLOW: i32 = 33554432;
pub const IN_EXCL_UNLINK: i32 = 67108864;
pub const IN_IGNORED: i32 = 32768;
pub const IN_ISDIR: i32 = 1073741824;
pub const IN_MASK_ADD: i32 = 536870912;
pub const IN_MASK_CREATE: i32 = 268435456;
pub const IN_MODIFY: i32 = 2;
pub const IN_MOVED_FROM: i32 = 64;
pub const IN_MOVED_TO: i32 = 128;
pub const IN_MOVE_SELF: i32 = 2048;
pub const IN_ONESHOT: i32 = -2147483648;
pub const IN_ONLYDIR: i32 = 16777216;
pub const IN_OPEN: i32 = 32;
pub const IN_Q_OVERFLOW: i32 = 16384;
pub const IN_UNMOUNT: i32 = 8192;
pub const _SYS_INOTIFY_H: i32 = 1;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct inotify_event {
    pub wd: i32,
    pub mask: u32,
    pub cookie: u32,
    pub len: u32,
    pub name: *mut i8,
}
