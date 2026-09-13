#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn adjtime(__delta : *const super::struct_timeval::timeval, __olddelta : *mut super::struct_timeval::timeval) -> i32);
#[cfg(feature = "struct_tm")]
windows_link::link!("c" "C" fn asctime(__tp : *const super::struct_tm::tm) -> *mut i8);
#[cfg(feature = "struct_tm")]
windows_link::link!("c" "C" fn asctime_r(__tp : *const super::struct_tm::tm, __buf : *mut i8) -> *mut i8);
#[cfg(all(feature = "clock_t", feature = "types"))]
windows_link::link!("c" "C" fn clock() -> super::clock_t::clock_t);
#[cfg(all(feature = "clockid_t", feature = "types"))]
windows_link::link!("c" "C" fn clock_getcpuclockid(__pid : super::types::pid_t, __clock_id : *mut super::clockid_t::clockid_t) -> i32);
#[cfg(all(feature = "clockid_t", feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn clock_getres(__clock_id : super::clockid_t::clockid_t, __res : *mut super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "clockid_t", feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn clock_gettime(__clock_id : super::clockid_t::clockid_t, __tp : *mut super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "clockid_t", feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn clock_nanosleep(__clock_id : super::clockid_t::clockid_t, __flags : i32, __req : *const super::struct_timespec::timespec, __rem : *mut super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "clockid_t", feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn clock_settime(__clock_id : super::clockid_t::clockid_t, __tp : *const super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn ctime(__timer : *const super::time_t::time_t) -> *mut i8);
#[cfg(all(feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn ctime_r(__timer : *const super::time_t::time_t, __buf : *mut i8) -> *mut i8);
#[cfg(all(feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn difftime(__time1 : super::time_t::time_t, __time0 : super::time_t::time_t) -> f64);
windows_link::link!("c" "C" fn dysize(__year : i32) -> i32);
#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn futimes(__fd : i32, __tvp : *mut super::struct_timeval::timeval) -> i32);
#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn getitimer(__which : __itimer_which_t, __value : *mut itimerval) -> i32);
#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn gettimeofday(__tv : *mut super::struct_timeval::timeval, __tz : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "struct_tm", feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn gmtime(__timer : *const super::time_t::time_t) -> *mut super::struct_tm::tm);
#[cfg(all(feature = "struct_tm", feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn gmtime_r(__timer : *const super::time_t::time_t, __tp : *mut super::struct_tm::tm) -> *mut super::struct_tm::tm);
#[cfg(all(feature = "struct_tm", feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn localtime(__timer : *const super::time_t::time_t) -> *mut super::struct_tm::tm);
#[cfg(all(feature = "struct_tm", feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn localtime_r(__timer : *const super::time_t::time_t, __tp : *mut super::struct_tm::tm) -> *mut super::struct_tm::tm);
#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn lutimes(__file : *const i8, __tvp : *mut super::struct_timeval::timeval) -> i32);
#[cfg(all(feature = "struct_tm", feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn mktime(__tp : *mut super::struct_tm::tm) -> super::time_t::time_t);
#[cfg(all(feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn nanosleep(__requested_time : *const super::struct_timespec::timespec, __remaining : *mut super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn setitimer(__which : __itimer_which_t, __new : *const itimerval, __old : *mut itimerval) -> i32);
#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn settimeofday(__tv : *const super::struct_timeval::timeval, __tz : *const timezone) -> i32);
#[cfg(feature = "struct_tm")]
windows_link::link!("c" "C" fn strftime(__s : *mut i8, __maxsize : usize, __format : *const i8, __tp : *const super::struct_tm::tm) -> usize);
#[cfg(all(feature = "__locale_t", feature = "locale_t", feature = "struct_tm"))]
windows_link::link!("c" "C" fn strftime_l(__s : *mut i8, __maxsize : usize, __format : *const i8, __tp : *const super::struct_tm::tm, __loc : super::locale_t::locale_t) -> usize);
#[cfg(all(feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn time(__timer : *mut super::time_t::time_t) -> super::time_t::time_t);
#[cfg(all(feature = "struct_tm", feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn timegm(__tp : *mut super::struct_tm::tm) -> super::time_t::time_t);
#[cfg(all(feature = "struct_tm", feature = "time_t", feature = "types"))]
windows_link::link!("c" "C" fn timelocal(__tp : *mut super::struct_tm::tm) -> super::time_t::time_t);
#[cfg(all(
    feature = "__sigval_t",
    feature = "clockid_t",
    feature = "pthreadtypes",
    feature = "sigevent_t",
    feature = "timer_t",
    feature = "types"
))]
windows_link::link!("c" "C" fn timer_create(__clock_id : super::clockid_t::clockid_t, __evp : *mut super::sigevent_t::sigevent_t, __timerid : *mut super::timer_t::timer_t) -> i32);
#[cfg(all(feature = "timer_t", feature = "types"))]
windows_link::link!("c" "C" fn timer_delete(__timerid : super::timer_t::timer_t) -> i32);
#[cfg(all(feature = "timer_t", feature = "types"))]
windows_link::link!("c" "C" fn timer_getoverrun(__timerid : super::timer_t::timer_t) -> i32);
#[cfg(all(
    feature = "struct_itimerspec",
    feature = "struct_timespec",
    feature = "timer_t",
    feature = "types"
))]
windows_link::link!("c" "C" fn timer_gettime(__timerid : super::timer_t::timer_t, __value : *mut super::struct_itimerspec::itimerspec) -> i32);
#[cfg(all(
    feature = "struct_itimerspec",
    feature = "struct_timespec",
    feature = "timer_t",
    feature = "types"
))]
windows_link::link!("c" "C" fn timer_settime(__timerid : super::timer_t::timer_t, __flags : i32, __value : *const super::struct_itimerspec::itimerspec, __ovalue : *mut super::struct_itimerspec::itimerspec) -> i32);
#[cfg(all(feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn timespec_get(__ts : *mut super::struct_timespec::timespec, __base : i32) -> i32);
windows_link::link!("c" "C" fn tzset());
#[cfg(all(feature = "struct_timeval", feature = "types"))]
windows_link::link!("c" "C" fn utimes(__file : *const i8, __tvp : *mut super::struct_timeval::timeval) -> i32);
#[cfg(feature = "types")]
pub const CLOCKS_PER_SEC: super::types::__clock_t = 1000000;
pub const CLOCK_BOOTTIME: i32 = 7;
pub const CLOCK_BOOTTIME_ALARM: i32 = 9;
pub const CLOCK_MONOTONIC: i32 = 1;
pub const CLOCK_MONOTONIC_COARSE: i32 = 6;
pub const CLOCK_MONOTONIC_RAW: i32 = 4;
pub const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
pub const CLOCK_REALTIME: i32 = 0;
pub const CLOCK_REALTIME_ALARM: i32 = 8;
pub const CLOCK_REALTIME_COARSE: i32 = 5;
pub const CLOCK_TAI: i32 = 11;
pub const CLOCK_THREAD_CPUTIME_ID: i32 = 3;
pub const ITIMER_PROF: __itimer_which = 2;
pub const ITIMER_REAL: __itimer_which = 0;
pub const ITIMER_VIRTUAL: __itimer_which = 1;
pub const TIMER_ABSTIME: i32 = 1;
pub const TIME_UTC: i32 = 1;
pub type __itimer_which = u32;
pub type __itimer_which_t = i32;
#[repr(C)]
#[cfg(all(feature = "struct_timeval", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct itimerval {
    pub it_interval: super::struct_timeval::timeval,
    pub it_value: super::struct_timeval::timeval,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
