#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: siginfo_t_0,
}
#[cfg(all(feature = "__sigval_t", feature = "types"))]
impl Default for siginfo_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy)]
pub union siginfo_t_0 {
    pub _pad: [i32; 28],
    pub _kill: siginfo_t_0_0,
    pub _timer: siginfo_t_0_1,
    pub _rt: siginfo_t_0_2,
    pub _sigchld: siginfo_t_0_3,
    pub _sigfault: siginfo_t_0_4,
    pub _sigpoll: siginfo_t_0_5,
    pub _sigsys: siginfo_t_0_6,
}
#[cfg(all(feature = "__sigval_t", feature = "types"))]
impl Default for siginfo_t_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct siginfo_t_0_0 {
    pub si_pid: super::types::__pid_t,
    pub si_uid: super::types::__uid_t,
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy)]
pub struct siginfo_t_0_1 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: super::__sigval_t::__sigval_t,
}
#[cfg(all(feature = "__sigval_t", feature = "types"))]
impl Default for siginfo_t_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy)]
pub struct siginfo_t_0_2 {
    pub si_pid: super::types::__pid_t,
    pub si_uid: super::types::__uid_t,
    pub si_sigval: super::__sigval_t::__sigval_t,
}
#[cfg(all(feature = "__sigval_t", feature = "types"))]
impl Default for siginfo_t_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct siginfo_t_0_3 {
    pub si_pid: super::types::__pid_t,
    pub si_uid: super::types::__uid_t,
    pub si_status: i32,
    pub si_utime: super::types::__clock_t,
    pub si_stime: super::types::__clock_t,
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy)]
pub struct siginfo_t_0_4 {
    pub si_addr: *mut core::ffi::c_void,
    pub si_addr_lsb: i16,
    pub _bounds: siginfo_t_0_4_0,
}
#[cfg(all(feature = "__sigval_t", feature = "types"))]
impl Default for siginfo_t_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy)]
pub union siginfo_t_0_4_0 {
    pub _addr_bnd: siginfo_t_0_4_0_0,
    pub _pkey: super::types::__uint32_t,
}
#[cfg(all(feature = "__sigval_t", feature = "types"))]
impl Default for siginfo_t_0_4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct siginfo_t_0_4_0_0 {
    pub _lower: *mut core::ffi::c_void,
    pub _upper: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct siginfo_t_0_5 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[repr(C)]
#[cfg(all(feature = "__sigval_t", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct siginfo_t_0_6 {
    pub _call_addr: *mut core::ffi::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
