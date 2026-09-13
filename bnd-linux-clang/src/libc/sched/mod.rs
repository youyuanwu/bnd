windows_link::link!("c" "C" fn sched_get_priority_max(__algorithm : i32) -> i32);
windows_link::link!("c" "C" fn sched_get_priority_min(__algorithm : i32) -> i32);
#[cfg(all(feature = "struct_sched_param", feature = "types"))]
windows_link::link!("c" "C" fn sched_getparam(__pid : super::types::__pid_t, __param : *mut super::struct_sched_param::sched_param) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn sched_getscheduler(__pid : super::types::__pid_t) -> i32);
#[cfg(all(feature = "struct_timespec", feature = "types"))]
windows_link::link!("c" "C" fn sched_rr_get_interval(__pid : super::types::__pid_t, __t : *mut super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "struct_sched_param", feature = "types"))]
windows_link::link!("c" "C" fn sched_setparam(__pid : super::types::__pid_t, __param : *const super::struct_sched_param::sched_param) -> i32);
#[cfg(all(feature = "struct_sched_param", feature = "types"))]
windows_link::link!("c" "C" fn sched_setscheduler(__pid : super::types::__pid_t, __policy : i32, __param : *const super::struct_sched_param::sched_param) -> i32);
windows_link::link!("c" "C" fn sched_yield() -> i32);
pub const SCHED_FIFO: i32 = 1;
pub const SCHED_OTHER: i32 = 0;
pub const SCHED_RR: i32 = 2;
