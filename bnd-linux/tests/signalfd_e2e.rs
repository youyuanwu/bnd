use bnd_linux::libc::signalfd;

#[test]
fn signalfd_receives_thread_signal() {
    let mut mask = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    let mut previous = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    assert_eq!(unsafe { libc::sigemptyset(&mut mask) }, 0);
    assert_eq!(unsafe { libc::sigaddset(&mut mask, libc::SIGUSR1) }, 0);
    assert_eq!(
        unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, &mask, &mut previous) },
        0
    );

    let fd = unsafe {
        signalfd::signalfd(
            -1,
            (&mask as *const libc::sigset_t).cast(),
            signalfd::SFD_NONBLOCK as i32,
        )
    };
    assert!(fd >= 0, "signalfd failed: {fd}");
    assert_eq!(
        unsafe { libc::pthread_kill(libc::pthread_self(), libc::SIGUSR1) },
        0
    );

    let mut info = signalfd::signalfd_siginfo::default();
    let read = unsafe {
        libc::read(
            fd,
            (&mut info as *mut signalfd::signalfd_siginfo).cast(),
            std::mem::size_of_val(&info),
        )
    };

    assert_eq!(
        unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &previous, std::ptr::null_mut()) },
        0
    );
    assert_eq!(unsafe { libc::close(fd) }, 0);

    assert_eq!(read, std::mem::size_of_val(&info) as isize);
    assert_eq!(info.ssi_signo, libc::SIGUSR1 as u32);
}

#[test]
fn signalfd_layout_and_constants_match_linux() {
    assert_eq!(
        std::mem::size_of::<signalfd::signalfd_siginfo>(),
        std::mem::size_of::<libc::signalfd_siginfo>()
    );
    assert_eq!(
        std::mem::align_of::<signalfd::signalfd_siginfo>(),
        std::mem::align_of::<libc::signalfd_siginfo>()
    );
    assert_eq!(signalfd::SFD_CLOEXEC, libc::SFD_CLOEXEC as u32);
    assert_eq!(signalfd::SFD_NONBLOCK, libc::SFD_NONBLOCK as u32);
}
