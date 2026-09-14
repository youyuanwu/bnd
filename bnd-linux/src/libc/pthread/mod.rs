#[cfg(all(
    feature = "__sigset_t",
    feature = "setjmp",
    feature = "struct___jmp_buf_tag"
))]
windows_link::link!("c" "C" fn __sigsetjmp(__env : *mut super::struct___jmp_buf_tag::__jmp_buf_tag, __savemask : i32) -> i32);
windows_link::link!("c" "C" fn pthread_atfork(__prepare : *mut u8, __parent : *mut u8, __child : *mut u8) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_destroy(__attr : *mut super::pthreadtypes::pthread_attr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getdetachstate(__attr : *const super::pthreadtypes::pthread_attr_t, __detachstate : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getguardsize(__attr : *const super::pthreadtypes::pthread_attr_t, __guardsize : *mut usize) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getinheritsched(__attr : *const super::pthreadtypes::pthread_attr_t, __inherit : *mut i32) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_sched_param"))]
windows_link::link!("c" "C" fn pthread_attr_getschedparam(__attr : *const super::pthreadtypes::pthread_attr_t, __param : *mut super::struct_sched_param::sched_param) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getschedpolicy(__attr : *const super::pthreadtypes::pthread_attr_t, __policy : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getscope(__attr : *const super::pthreadtypes::pthread_attr_t, __scope : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getstack(__attr : *const super::pthreadtypes::pthread_attr_t, __stackaddr : *mut *mut core::ffi::c_void, __stacksize : *mut usize) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getstackaddr(__attr : *const super::pthreadtypes::pthread_attr_t, __stackaddr : *mut *mut core::ffi::c_void) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_getstacksize(__attr : *const super::pthreadtypes::pthread_attr_t, __stacksize : *mut usize) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_init(__attr : *mut super::pthreadtypes::pthread_attr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setdetachstate(__attr : *mut super::pthreadtypes::pthread_attr_t, __detachstate : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setguardsize(__attr : *mut super::pthreadtypes::pthread_attr_t, __guardsize : usize) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setinheritsched(__attr : *mut super::pthreadtypes::pthread_attr_t, __inherit : i32) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_sched_param"))]
windows_link::link!("c" "C" fn pthread_attr_setschedparam(__attr : *mut super::pthreadtypes::pthread_attr_t, __param : *const super::struct_sched_param::sched_param) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setschedpolicy(__attr : *mut super::pthreadtypes::pthread_attr_t, __policy : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setscope(__attr : *mut super::pthreadtypes::pthread_attr_t, __scope : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setstack(__attr : *mut super::pthreadtypes::pthread_attr_t, __stackaddr : *mut core::ffi::c_void, __stacksize : usize) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setstackaddr(__attr : *mut super::pthreadtypes::pthread_attr_t, __stackaddr : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_attr_setstacksize(__attr : *mut super::pthreadtypes::pthread_attr_t, __stacksize : usize) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_barrier_destroy(__barrier : *mut super::pthreadtypes::pthread_barrier_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_barrier_init(__barrier : *mut super::pthreadtypes::pthread_barrier_t, __attr : *const super::pthreadtypes::pthread_barrierattr_t, __count : u32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_barrier_wait(__barrier : *mut super::pthreadtypes::pthread_barrier_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_barrierattr_destroy(__attr : *mut super::pthreadtypes::pthread_barrierattr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_barrierattr_getpshared(__attr : *const super::pthreadtypes::pthread_barrierattr_t, __pshared : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_barrierattr_init(__attr : *mut super::pthreadtypes::pthread_barrierattr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_barrierattr_setpshared(__attr : *mut super::pthreadtypes::pthread_barrierattr_t, __pshared : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_cancel(__th : super::pthreadtypes::pthread_t) -> i32);
#[cfg(all(
    feature = "atomic_wide_counter",
    feature = "pthreadtypes",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_cond_broadcast(__cond : *mut super::pthreadtypes::pthread_cond_t) -> i32);
#[cfg(all(
    feature = "atomic_wide_counter",
    feature = "pthreadtypes",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_cond_destroy(__cond : *mut super::pthreadtypes::pthread_cond_t) -> i32);
#[cfg(all(
    feature = "atomic_wide_counter",
    feature = "pthreadtypes",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_cond_init(__cond : *mut super::pthreadtypes::pthread_cond_t, __cond_attr : *const super::pthreadtypes::pthread_condattr_t) -> i32);
#[cfg(all(
    feature = "atomic_wide_counter",
    feature = "pthreadtypes",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_cond_signal(__cond : *mut super::pthreadtypes::pthread_cond_t) -> i32);
#[cfg(all(
    feature = "atomic_wide_counter",
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "struct_timespec",
    feature = "thread_shared_types",
    feature = "types"
))]
windows_link::link!("c" "C" fn pthread_cond_timedwait(__cond : *mut super::pthreadtypes::pthread_cond_t, __mutex : *mut super::pthreadtypes::pthread_mutex_t, __abstime : *const super::struct_timespec::timespec) -> i32);
#[cfg(all(
    feature = "atomic_wide_counter",
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_cond_wait(__cond : *mut super::pthreadtypes::pthread_cond_t, __mutex : *mut super::pthreadtypes::pthread_mutex_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_condattr_destroy(__attr : *mut super::pthreadtypes::pthread_condattr_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "types"))]
windows_link::link!("c" "C" fn pthread_condattr_getclock(__attr : *const super::pthreadtypes::pthread_condattr_t, __clock_id : *mut super::types::__clockid_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_condattr_getpshared(__attr : *const super::pthreadtypes::pthread_condattr_t, __pshared : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_condattr_init(__attr : *mut super::pthreadtypes::pthread_condattr_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "types"))]
windows_link::link!("c" "C" fn pthread_condattr_setclock(__attr : *mut super::pthreadtypes::pthread_condattr_t, __clock_id : super::types::__clockid_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_condattr_setpshared(__attr : *mut super::pthreadtypes::pthread_condattr_t, __pshared : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_create(__newthread : *mut super::pthreadtypes::pthread_t, __attr : *const super::pthreadtypes::pthread_attr_t, __start_routine : *mut u8, __arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_detach(__th : super::pthreadtypes::pthread_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_equal(__thread1 : super::pthreadtypes::pthread_t, __thread2 : super::pthreadtypes::pthread_t) -> i32);
windows_link::link!("c" "C" fn pthread_exit(__retval : *mut core::ffi::c_void) -> !);
#[cfg(all(feature = "pthreadtypes", feature = "types"))]
windows_link::link!("c" "C" fn pthread_getcpuclockid(__thread_id : super::pthreadtypes::pthread_t, __clock_id : *mut super::types::__clockid_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_sched_param"))]
windows_link::link!("c" "C" fn pthread_getschedparam(__target_thread : super::pthreadtypes::pthread_t, __policy : *mut i32, __param : *mut super::struct_sched_param::sched_param) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_getspecific(__key : super::pthreadtypes::pthread_key_t) -> *mut core::ffi::c_void);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_join(__th : super::pthreadtypes::pthread_t, __thread_return : *mut *mut core::ffi::c_void) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_key_create(__key : *mut super::pthreadtypes::pthread_key_t, __destr_function : *mut u8) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_key_delete(__key : super::pthreadtypes::pthread_key_t) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_consistent(__mutex : *mut super::pthreadtypes::pthread_mutex_t) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_destroy(__mutex : *mut super::pthreadtypes::pthread_mutex_t) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_getprioceiling(__mutex : *const super::pthreadtypes::pthread_mutex_t, __prioceiling : *mut i32) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_init(__mutex : *mut super::pthreadtypes::pthread_mutex_t, __mutexattr : *const super::pthreadtypes::pthread_mutexattr_t) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_lock(__mutex : *mut super::pthreadtypes::pthread_mutex_t) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_setprioceiling(__mutex : *mut super::pthreadtypes::pthread_mutex_t, __prioceiling : i32, __old_ceiling : *mut i32) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "struct_timespec",
    feature = "thread_shared_types",
    feature = "types"
))]
windows_link::link!("c" "C" fn pthread_mutex_timedlock(__mutex : *mut super::pthreadtypes::pthread_mutex_t, __abstime : *const super::struct_timespec::timespec) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_trylock(__mutex : *mut super::pthreadtypes::pthread_mutex_t) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_mutex",
    feature = "thread_shared_types"
))]
windows_link::link!("c" "C" fn pthread_mutex_unlock(__mutex : *mut super::pthreadtypes::pthread_mutex_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_destroy(__attr : *mut super::pthreadtypes::pthread_mutexattr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_getprioceiling(__attr : *const super::pthreadtypes::pthread_mutexattr_t, __prioceiling : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_getprotocol(__attr : *const super::pthreadtypes::pthread_mutexattr_t, __protocol : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_getpshared(__attr : *const super::pthreadtypes::pthread_mutexattr_t, __pshared : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_getrobust(__attr : *const super::pthreadtypes::pthread_mutexattr_t, __robustness : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_gettype(__attr : *const super::pthreadtypes::pthread_mutexattr_t, __kind : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_init(__attr : *mut super::pthreadtypes::pthread_mutexattr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_setprioceiling(__attr : *mut super::pthreadtypes::pthread_mutexattr_t, __prioceiling : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_setprotocol(__attr : *mut super::pthreadtypes::pthread_mutexattr_t, __protocol : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_setpshared(__attr : *mut super::pthreadtypes::pthread_mutexattr_t, __pshared : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_setrobust(__attr : *mut super::pthreadtypes::pthread_mutexattr_t, __robustness : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_mutexattr_settype(__attr : *mut super::pthreadtypes::pthread_mutexattr_t, __kind : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_once(__once_control : *mut super::pthreadtypes::pthread_once_t, __init_routine : *mut u8) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_rwlock"))]
windows_link::link!("c" "C" fn pthread_rwlock_destroy(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_rwlock"))]
windows_link::link!("c" "C" fn pthread_rwlock_init(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t, __attr : *const super::pthreadtypes::pthread_rwlockattr_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_rwlock"))]
windows_link::link!("c" "C" fn pthread_rwlock_rdlock(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_rwlock",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn pthread_rwlock_timedrdlock(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t, __abstime : *const super::struct_timespec::timespec) -> i32);
#[cfg(all(
    feature = "pthreadtypes",
    feature = "struct_rwlock",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn pthread_rwlock_timedwrlock(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t, __abstime : *const super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_rwlock"))]
windows_link::link!("c" "C" fn pthread_rwlock_tryrdlock(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_rwlock"))]
windows_link::link!("c" "C" fn pthread_rwlock_trywrlock(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_rwlock"))]
windows_link::link!("c" "C" fn pthread_rwlock_unlock(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_rwlock"))]
windows_link::link!("c" "C" fn pthread_rwlock_wrlock(__rwlock : *mut super::pthreadtypes::pthread_rwlock_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_rwlockattr_destroy(__attr : *mut super::pthreadtypes::pthread_rwlockattr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_rwlockattr_getkind_np(__attr : *const super::pthreadtypes::pthread_rwlockattr_t, __pref : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_rwlockattr_getpshared(__attr : *const super::pthreadtypes::pthread_rwlockattr_t, __pshared : *mut i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_rwlockattr_init(__attr : *mut super::pthreadtypes::pthread_rwlockattr_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_rwlockattr_setkind_np(__attr : *mut super::pthreadtypes::pthread_rwlockattr_t, __pref : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_rwlockattr_setpshared(__attr : *mut super::pthreadtypes::pthread_rwlockattr_t, __pshared : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_self() -> super::pthreadtypes::pthread_t);
windows_link::link!("c" "C" fn pthread_setcancelstate(__state : i32, __oldstate : *mut i32) -> i32);
windows_link::link!("c" "C" fn pthread_setcanceltype(__type : i32, __oldtype : *mut i32) -> i32);
#[cfg(all(feature = "pthreadtypes", feature = "struct_sched_param"))]
windows_link::link!("c" "C" fn pthread_setschedparam(__target_thread : super::pthreadtypes::pthread_t, __policy : i32, __param : *const super::struct_sched_param::sched_param) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_setschedprio(__target_thread : super::pthreadtypes::pthread_t, __prio : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_setspecific(__key : super::pthreadtypes::pthread_key_t, __pointer : *const core::ffi::c_void) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_spin_destroy(__lock : *mut super::pthreadtypes::pthread_spinlock_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_spin_init(__lock : *mut super::pthreadtypes::pthread_spinlock_t, __pshared : i32) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_spin_lock(__lock : *mut super::pthreadtypes::pthread_spinlock_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_spin_trylock(__lock : *mut super::pthreadtypes::pthread_spinlock_t) -> i32);
#[cfg(feature = "pthreadtypes")]
windows_link::link!("c" "C" fn pthread_spin_unlock(__lock : *mut super::pthreadtypes::pthread_spinlock_t) -> i32);
windows_link::link!("c" "C" fn pthread_testcancel());
pub const PTHREAD_BARRIER_SERIAL_THREAD: i32 = -1;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: u32 = 1;
pub const PTHREAD_CANCEL_DEFERRED: u32 = 0;
pub const PTHREAD_CANCEL_DISABLE: u32 = 1;
pub const PTHREAD_CANCEL_ENABLE: u32 = 0;
pub const PTHREAD_CREATE_DETACHED: u32 = 1;
pub const PTHREAD_CREATE_JOINABLE: u32 = 0;
pub const PTHREAD_EXPLICIT_SCHED: u32 = 1;
pub const PTHREAD_INHERIT_SCHED: u32 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: u32 = 3;
pub const PTHREAD_MUTEX_DEFAULT: u32 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: u32 = 2;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: u32 = 2;
pub const PTHREAD_MUTEX_NORMAL: u32 = 0;
pub const PTHREAD_MUTEX_RECURSIVE: u32 = 1;
pub const PTHREAD_MUTEX_RECURSIVE_NP: u32 = 1;
pub const PTHREAD_MUTEX_ROBUST: u32 = 1;
pub const PTHREAD_MUTEX_ROBUST_NP: u32 = 1;
pub const PTHREAD_MUTEX_STALLED: u32 = 0;
pub const PTHREAD_MUTEX_STALLED_NP: u32 = 0;
pub const PTHREAD_MUTEX_TIMED_NP: u32 = 0;
pub const PTHREAD_ONCE_INIT: i32 = 0;
pub const PTHREAD_PRIO_INHERIT: u32 = 1;
pub const PTHREAD_PRIO_NONE: u32 = 0;
pub const PTHREAD_PRIO_PROTECT: u32 = 2;
pub const PTHREAD_PROCESS_PRIVATE: u32 = 0;
pub const PTHREAD_PROCESS_SHARED: u32 = 1;
pub const PTHREAD_RWLOCK_DEFAULT_NP: u32 = 0;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: u32 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: u32 = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: u32 = 1;
pub const PTHREAD_SCOPE_PROCESS: u32 = 1;
pub const PTHREAD_SCOPE_SYSTEM: u32 = 0;
#[repr(C)]
#[cfg(feature = "setjmp")]
#[derive(Clone, Copy)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: super::setjmp::__jmp_buf,
    pub __mask_was_saved: i32,
}
#[cfg(feature = "setjmp")]
impl Default for __cancel_jmp_buf_tag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: *mut u8,
    pub __cancel_arg: *mut core::ffi::c_void,
    pub __do_it: i32,
    pub __cancel_type: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _pthread_cleanup_buffer {
    pub __routine: *mut u8,
    pub __arg: *mut core::ffi::c_void,
    pub __canceltype: i32,
    pub __prev: *mut Self,
}
