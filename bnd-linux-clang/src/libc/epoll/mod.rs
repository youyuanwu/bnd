windows_link::link!("c" "C" fn epoll_create(__size : i32) -> i32);
windows_link::link!("c" "C" fn epoll_create1(__flags : i32) -> i32);
windows_link::link!("c" "C" fn epoll_ctl(__epfd : i32, __op : i32, __fd : i32, __event : *mut epoll_event) -> i32);
#[cfg(feature = "__sigset_t")]
windows_link::link!("c" "C" fn epoll_pwait(__epfd : i32, __events : *mut epoll_event, __maxevents : i32, __timeout : i32, __ss : *const super::__sigset_t::__sigset_t) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn epoll_pwait2(__epfd : i32, __events : *mut epoll_event, __maxevents : i32, __timeout : *const super::struct_timespec::timespec, __ss : *const super::__sigset_t::__sigset_t) -> i32);
windows_link::link!("c" "C" fn epoll_wait(__epfd : i32, __events : *mut epoll_event, __maxevents : i32, __timeout : i32) -> i32);
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLIN: EPOLL_EVENTS = 1;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLL_CLOEXEC: u32 = 524288;
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
pub const EPOLL_CTL_MOD: i32 = 3;
pub type EPOLL_EVENTS = u32;
pub const EPOLL_IOC_TYPE: i32 = 138;
#[repr(C)]
#[derive(Clone, Copy)]
pub union epoll_data_t {
    pub ptr: *mut core::ffi::c_void,
    pub fd: i32,
    pub u32: u32,
    pub u64: u64,
}
impl Default for epoll_data_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data_t,
}
impl Default for epoll_event {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct epoll_params {
    pub busy_poll_usecs: u32,
    pub busy_poll_budget: u16,
    pub prefer_busy_poll: u8,
    pub __pad: u8,
}
