#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "pthreadtypes", feature = "types"))]
#[derive(Clone, Copy)]
pub struct sigevent_t {
    pub sigev_value: super::__sigval_t::__sigval_t,
    pub sigev_signo: i32,
    pub sigev_notify: i32,
    pub _sigev_un: sigevent_t_0,
}
#[cfg(all(feature = "__sigval_t", feature = "pthreadtypes", feature = "types"))]
impl Default for sigevent_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "pthreadtypes", feature = "types"))]
#[derive(Clone, Copy)]
pub union sigevent_t_0 {
    pub _pad: [i32; 12],
    pub _tid: super::types::__pid_t,
    pub _sigev_thread: sigevent_t_0_0,
}
#[cfg(all(feature = "__sigval_t", feature = "pthreadtypes", feature = "types"))]
impl Default for sigevent_t_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "pthreadtypes", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct sigevent_t_0_0 {
    pub _function: *mut u8,
    pub _attribute: *mut super::pthreadtypes::pthread_attr_t,
}
