windows_link::link!("c" "C" fn __libc_current_sigrtmax() -> i32);
windows_link::link!("c" "C" fn __libc_current_sigrtmin() -> i32);
windows_link::link!("c" "C" fn __sysv_signal(__sig : i32, __handler : __sighandler_t) -> __sighandler_t);
windows_link::link!("c" "C" fn gsignal(__sig : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn kill(__pid : super::types::__pid_t, __sig : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn killpg(__pgrp : super::types::__pid_t, __sig : i32) -> i32);
#[cfg(all(feature = "__sigval_t", feature = "siginfo_t", feature = "types"))]
windows_link::link!("c" "C" fn psiginfo(__pinfo : *const super::siginfo_t::siginfo_t, __s : *const i8));
windows_link::link!("c" "C" fn psignal(__sig : i32, __s : *const i8));
windows_link::link!("c" "C" fn raise(__sig : i32) -> i32);
#[cfg(feature = "__sigset_t")]
windows_link::link!("c" "C" fn sigaction(__sig : i32, __act : *const sigaction, __oact : *mut sigaction) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigaddset(__set : *mut super::sigset_t::sigset_t, __signo : i32) -> i32);
#[cfg(feature = "stack_t")]
windows_link::link!("c" "C" fn sigaltstack(__ss : *const super::stack_t::stack_t, __oss : *mut super::stack_t::stack_t) -> i32);
windows_link::link!("c" "C" fn sigblock(__mask : i32) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigdelset(__set : *mut super::sigset_t::sigset_t, __signo : i32) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigemptyset(__set : *mut super::sigset_t::sigset_t) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigfillset(__set : *mut super::sigset_t::sigset_t) -> i32);
windows_link::link!("c" "C" fn siggetmask() -> i32);
windows_link::link!("c" "C" fn siginterrupt(__sig : i32, __interrupt : i32) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigismember(__set : *const super::sigset_t::sigset_t, __signo : i32) -> i32);
windows_link::link!("c" "C" fn signal(__sig : i32, __handler : __sighandler_t) -> __sighandler_t);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigpending(__set : *mut super::sigset_t::sigset_t) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigprocmask(__how : i32, __set : *const super::sigset_t::sigset_t, __oset : *mut super::sigset_t::sigset_t) -> i32);
#[cfg(all(feature = "__sigval_t", feature = "types"))]
windows_link::link!("c" "C" fn sigqueue(__pid : super::types::__pid_t, __sig : i32, __val : super::__sigval_t::sigval) -> i32);
#[cfg(all(feature = "sigcontext", feature = "types"))]
windows_link::link!("c" "C" fn sigreturn(__scp : *mut super::sigcontext::sigcontext) -> i32);
windows_link::link!("c" "C" fn sigsetmask(__mask : i32) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigsuspend(__set : *const super::sigset_t::sigset_t) -> i32);
#[cfg(all(
    feature = "__sigset_t",
    feature = "__sigval_t",
    feature = "siginfo_t",
    feature = "sigset_t",
    feature = "struct_timespec",
    feature = "types"
))]
windows_link::link!("c" "C" fn sigtimedwait(__set : *const super::sigset_t::sigset_t, __info : *mut super::siginfo_t::siginfo_t, __timeout : *const super::struct_timespec::timespec) -> i32);
#[cfg(all(feature = "__sigset_t", feature = "sigset_t"))]
windows_link::link!("c" "C" fn sigwait(__set : *const super::sigset_t::sigset_t, __sig : *mut i32) -> i32);
#[cfg(all(
    feature = "__sigset_t",
    feature = "__sigval_t",
    feature = "siginfo_t",
    feature = "sigset_t",
    feature = "types"
))]
windows_link::link!("c" "C" fn sigwaitinfo(__set : *const super::sigset_t::sigset_t, __info : *mut super::siginfo_t::siginfo_t) -> i32);
windows_link::link!("c" "C" fn ssignal(__sig : i32, __handler : __sighandler_t) -> __sighandler_t);
pub const NSIG: i32 = 65;
pub type __sighandler_t = Option<unsafe extern "C" fn(param0: i32)>;
pub type sig_t = __sighandler_t;
#[repr(C)]
#[cfg(feature = "__sigset_t")]
#[derive(Clone, Copy)]
pub struct sigaction {
    pub __sigaction_handler: sigaction_0,
    pub sa_mask: super::__sigset_t::__sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: *mut u8,
}
#[cfg(feature = "__sigset_t")]
impl Default for sigaction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "__sigset_t")]
#[derive(Clone, Copy)]
pub union sigaction_0 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: *mut u8,
}
#[cfg(feature = "__sigset_t")]
impl Default for sigaction_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
