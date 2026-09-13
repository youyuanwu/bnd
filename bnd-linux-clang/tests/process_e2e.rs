use bnd_linux_clang::libc::{pthread, pthreadtypes, sched, signal, sigset_t};

unsafe extern "C" fn return_argument(argument: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    argument
}

#[test]
fn pthread_create_join_roundtrip() {
    let current = unsafe { pthread::pthread_self() };
    assert_ne!(unsafe { pthread::pthread_equal(current, current) }, 0);
    assert_eq!(unsafe { sched::sched_yield() }, 0);

    let argument = Box::into_raw(Box::new(42u32)).cast();
    let mut thread: pthreadtypes::pthread_t = 0;
    assert_eq!(
        unsafe {
            pthread::pthread_create(
                &mut thread,
                std::ptr::null(),
                return_argument as *const () as *mut u8,
                argument,
            )
        },
        0
    );

    let mut result = std::ptr::null_mut();
    assert_eq!(unsafe { pthread::pthread_join(thread, &mut result) }, 0);
    assert_eq!(*unsafe { Box::from_raw(result.cast::<u32>()) }, 42);
}

#[test]
fn generated_signal_set_matches_libc() {
    let mut set = sigset_t::sigset_t::default();
    assert_eq!(unsafe { signal::sigemptyset(&mut set) }, 0);
    assert_eq!(unsafe { signal::sigaddset(&mut set, libc::SIGUSR1) }, 0);
    assert_eq!(unsafe { signal::sigismember(&set, libc::SIGUSR1) }, 1);
    assert_eq!(
        std::mem::size_of::<sigset_t::sigset_t>(),
        std::mem::size_of::<libc::sigset_t>()
    );
}
