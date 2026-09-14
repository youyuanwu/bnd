#[cfg(feature = "types")]
windows_link::link!("c" "C" fn timerfd_create(__clock_id : super::types::__clockid_t, __flags : i32) -> i32);
#[cfg(all(
    feature = "struct_itimerspec",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn timerfd_gettime(__ufd : i32, __otmr : *mut super::struct_itimerspec::itimerspec) -> i32);
#[cfg(all(
    feature = "struct_itimerspec",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn timerfd_settime(__ufd : i32, __flags : i32, __utmr : *const super::struct_itimerspec::itimerspec, __otmr : *mut super::struct_itimerspec::itimerspec) -> i32);
pub const TFD_CLOEXEC: u32 = 524288;
pub const TFD_NONBLOCK: u32 = 2048;
pub const TFD_TIMER_ABSTIME: u32 = 1;
pub const TFD_TIMER_CANCEL_ON_SET: u32 = 2;
