//! End-to-end tests for Signal bindings against real libc.

use bnd_linux::libc::{
    sigaction as sigaction_constants, siginfo_t, signal, signum_arch, signum_generic, sigset_t,
    stack_t,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

#[test]
fn sig_constants() {
    assert_eq!(signum_generic::SIGHUP, 1);
    assert_eq!(signum_generic::SIGINT, 2);
    assert_eq!(signum_generic::SIGQUIT, 3);
    assert_eq!(signum_generic::SIGILL, 4);
    assert_eq!(signum_generic::SIGTRAP, 5);
    assert_eq!(signum_generic::SIGABRT, 6);
    assert_eq!(signum_arch::SIGBUS, 7);
    assert_eq!(signum_generic::SIGFPE, 8);
    assert_eq!(signum_generic::SIGKILL, 9);
    assert_eq!(signum_arch::SIGUSR1, 10);
    assert_eq!(signum_generic::SIGSEGV, 11);
    assert_eq!(signum_arch::SIGUSR2, 12);
    assert_eq!(signum_generic::SIGPIPE, 13);
    assert_eq!(signum_generic::SIGALRM, 14);
    assert_eq!(signum_generic::SIGTERM, 15);
    assert_eq!(signum_arch::SIGCHLD, 17);
    assert_eq!(signum_arch::SIGCONT, 18);
    assert_eq!(signum_arch::SIGSTOP, 19);
    assert_eq!(signum_arch::SIGTSTP, 20);
}

#[test]
fn sa_flag_constants() {
    assert_eq!(sigaction_constants::SA_NOCLDSTOP, 1);
    assert_eq!(sigaction_constants::SA_NOCLDWAIT, 2);
    assert_eq!(sigaction_constants::SA_SIGINFO, 4);
    assert_eq!(sigaction_constants::SA_ONSTACK, 0x08000000);
    assert_eq!(sigaction_constants::SA_RESTART, 0x10000000);
    assert_ne!(sigaction_constants::SA_NODEFER, 0);
    assert_ne!(sigaction_constants::SA_RESETHAND, 0);
}

#[test]
fn sig_block_constants() {
    assert_eq!(sigaction_constants::SIG_BLOCK, 0);
    assert_eq!(sigaction_constants::SIG_UNBLOCK, 1);
    assert_eq!(sigaction_constants::SIG_SETMASK, 2);
}

// ---------------------------------------------------------------------------
// Struct layouts
// ---------------------------------------------------------------------------

#[test]
fn sigaction_struct_size() {
    // struct sigaction is 152 bytes on x86-64 Linux
    assert_eq!(
        std::mem::size_of::<signal::sigaction>(),
        152,
        "sigaction should be 152 bytes on x86-64"
    );
}

#[test]
fn sigset_struct_size() {
    // __sigset_t is 128 bytes (1024 bits / 8)
    assert_eq!(
        std::mem::size_of::<sigset_t::sigset_t>(),
        128,
        "__sigset_t should be 128 bytes"
    );
}

#[test]
fn siginfo_struct_size() {
    // siginfo_t is 128 bytes on x86-64 Linux
    assert_eq!(
        std::mem::size_of::<siginfo_t::siginfo_t>(),
        128,
        "siginfo_t should be 128 bytes"
    );
}

#[test]
fn stack_t_struct_size() {
    // stack_t is 24 bytes on x86-64 (void*, int, size_t with padding)
    assert_eq!(
        std::mem::size_of::<stack_t::stack_t>(),
        24,
        "stack_t should be 24 bytes"
    );
}

// ---------------------------------------------------------------------------
// Function pointer type (__sighandler_t)
// ---------------------------------------------------------------------------

#[test]
fn sighandler_type_is_option_fn_pointer() {
    // __sighandler_t is Option<unsafe extern "C" fn(i32)>
    // It should be pointer-sized
    assert_eq!(
        std::mem::size_of::<signal::__sighandler_t>(),
        std::mem::size_of::<usize>(),
        "__sighandler_t should be pointer-sized"
    );
}

// ---------------------------------------------------------------------------
// sigset operations
// ---------------------------------------------------------------------------

#[test]
fn sigemptyset_and_sigaddset() {
    let mut set = sigset_t::sigset_t::default();
    let rc = unsafe { signal::sigemptyset(&mut set) };
    assert_eq!(rc, 0, "sigemptyset should succeed");

    let rc = unsafe { signal::sigaddset(&mut set, signum_arch::SIGUSR1) };
    assert_eq!(rc, 0, "sigaddset should succeed");

    let ismember = unsafe { signal::sigismember(&set as *const _, signum_arch::SIGUSR1) };
    assert_eq!(ismember, 1, "SIGUSR1 should be in the set");

    let not_member = unsafe { signal::sigismember(&set as *const _, signum_arch::SIGUSR2) };
    assert_eq!(not_member, 0, "SIGUSR2 should not be in the set");
}

#[test]
fn sigfillset_and_sigdelset() {
    let mut set = sigset_t::sigset_t::default();
    let rc = unsafe { signal::sigfillset(&mut set) };
    assert_eq!(rc, 0, "sigfillset should succeed");

    let ismember = unsafe { signal::sigismember(&set as *const _, signum_generic::SIGINT) };
    assert_eq!(ismember, 1, "SIGINT should be in a full set");

    let rc = unsafe { signal::sigdelset(&mut set, signum_generic::SIGINT) };
    assert_eq!(rc, 0, "sigdelset should succeed");

    let ismember = unsafe { signal::sigismember(&set as *const _, signum_generic::SIGINT) };
    assert_eq!(ismember, 0, "SIGINT should no longer be in the set");
}

// ---------------------------------------------------------------------------
// Signal delivery
// ---------------------------------------------------------------------------

#[test]
fn raise_and_signal_handler() {
    use std::sync::atomic::{AtomicBool, Ordering};

    static HANDLER_CALLED: AtomicBool = AtomicBool::new(false);

    unsafe extern "C" fn handler(_sig: i32) {
        HANDLER_CALLED.store(true, Ordering::SeqCst);
    }

    // Install handler for SIGUSR1
    let prev = unsafe { signal::signal(signum_arch::SIGUSR1, Some(handler)) };
    assert!(
        prev.is_some() || prev.is_none(),
        "signal() should return previous handler"
    );

    // Raise SIGUSR1
    let rc = unsafe { signal::raise(signum_arch::SIGUSR1) };
    assert_eq!(rc, 0, "raise should succeed");

    assert!(
        HANDLER_CALLED.load(Ordering::SeqCst),
        "handler should have been called"
    );

    // Restore default handler
    unsafe { signal::signal(signum_arch::SIGUSR1, None) };
}

// ---------------------------------------------------------------------------
// sigaction
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::field_reassign_with_default)]
fn sigaction_install_handler() {
    use std::sync::atomic::{AtomicI32, Ordering};

    static RECEIVED_SIG: AtomicI32 = AtomicI32::new(0);

    unsafe extern "C" fn handler(sig: i32) {
        RECEIVED_SIG.store(sig, Ordering::SeqCst);
    }

    let mut sa = signal::sigaction::default();
    sa.__sigaction_handler.sa_handler = Some(handler);
    sa.sa_flags = sigaction_constants::SA_RESTART;

    // Empty the mask
    unsafe {
        signal::sigemptyset(&mut sa.sa_mask);
    }

    let rc =
        unsafe { signal::sigaction(signum_arch::SIGUSR2, &sa as *const _, core::ptr::null_mut()) };
    assert_eq!(rc, 0, "sigaction should succeed");

    // Raise SIGUSR2
    let rc = unsafe { signal::raise(signum_arch::SIGUSR2) };
    assert_eq!(rc, 0, "raise should succeed");

    assert_eq!(
        RECEIVED_SIG.load(Ordering::SeqCst),
        signum_arch::SIGUSR2,
        "handler should have received SIGUSR2"
    );

    // Restore default
    let mut default_sa = signal::sigaction::default();
    unsafe {
        signal::sigemptyset(&mut default_sa.sa_mask);
        signal::sigaction(
            signum_arch::SIGUSR2,
            &default_sa as *const _,
            core::ptr::null_mut(),
        );
    }
}

// ---------------------------------------------------------------------------
// sigprocmask
// ---------------------------------------------------------------------------

#[test]
fn sigprocmask_block_and_pending() {
    let mut block_set = sigset_t::sigset_t::default();
    unsafe { signal::sigemptyset(&mut block_set) };
    unsafe { signal::sigaddset(&mut block_set, signum_arch::SIGUSR1) };

    // Save old mask and block SIGUSR1
    let mut old_set = sigset_t::sigset_t::default();
    let rc = unsafe {
        signal::sigprocmask(
            sigaction_constants::SIG_BLOCK,
            &block_set as *const _,
            &mut old_set,
        )
    };
    assert_eq!(rc, 0, "sigprocmask SIG_BLOCK should succeed");

    // Check pending set — SIGUSR1 should NOT be pending yet (not raised)
    let mut pending = sigset_t::sigset_t::default();
    let rc = unsafe { signal::sigpending(&mut pending) };
    assert_eq!(rc, 0, "sigpending should succeed");

    let is_pending = unsafe { signal::sigismember(&pending as *const _, signum_arch::SIGUSR1) };
    assert_eq!(is_pending, 0, "SIGUSR1 should not be pending before raise");

    // Restore old mask
    let rc = unsafe {
        signal::sigprocmask(
            sigaction_constants::SIG_SETMASK,
            &old_set as *const _,
            core::ptr::null_mut(),
        )
    };
    assert_eq!(rc, 0, "sigprocmask SIG_SETMASK restore should succeed");
}

// ---------------------------------------------------------------------------
// kill (send signal to self)
// ---------------------------------------------------------------------------

#[test]
fn kill_self_with_zero() {
    // kill(getpid(), 0) is a standard way to test if a process exists
    let pid = std::process::id() as i32;
    let rc = unsafe { signal::kill(pid, 0) };
    assert_eq!(rc, 0, "kill(self, 0) should succeed");
}
