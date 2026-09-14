#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy, Default)]
pub struct timespec {
    pub tv_sec: super::types::__time_t,
    pub tv_nsec: super::types::__syscall_slong_t,
}
