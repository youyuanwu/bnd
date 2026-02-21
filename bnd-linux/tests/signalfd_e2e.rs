use bnd_linux::linux::signalfd;

#[test]
fn signalfd_siginfo_struct_size() {
    assert_eq!(core::mem::size_of::<signalfd::signalfd_siginfo>(), 128);
}

#[test]
fn sfd_constants() {
    assert_eq!(signalfd::SFD_CLOEXEC, 524288);
    assert_eq!(signalfd::SFD_NONBLOCK, 2048);
}

#[test]
fn signalfd_create() {
    // Block SIGUSR1 via sigprocmask, then create signalfd
    unsafe {
        let mut mask: libc::sigset_t = core::mem::zeroed();
        libc::sigemptyset(&mut mask);
        libc::sigaddset(&mut mask, libc::SIGUSR1);
        libc::sigprocmask(libc::SIG_BLOCK, &mask, core::ptr::null_mut());

        let sfd = signalfd::signalfd(
            -1,
            &mask as *const libc::sigset_t as *const _,
            signalfd::SFD_NONBLOCK as i32,
        );
        assert!(sfd >= 0, "signalfd failed: {sfd}");
        libc::close(sfd);

        // Unblock SIGUSR1
        libc::sigprocmask(libc::SIG_UNBLOCK, &mask, core::ptr::null_mut());
    }
}
