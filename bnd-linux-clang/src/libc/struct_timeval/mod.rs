#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy, Default)]
pub struct timeval {
    pub tv_sec: super::types::__time_t,
    pub tv_usec: super::types::__suseconds_t,
}
