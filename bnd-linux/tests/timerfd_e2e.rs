use bnd_linux::libc::{struct_itimerspec, struct_timespec, timerfd};

#[test]
fn timerfd_reports_expiration() {
    let fd = unsafe { timerfd::timerfd_create(libc::CLOCK_MONOTONIC, 0) };
    assert!(fd >= 0, "timerfd_create failed: {fd}");

    let mut current = struct_itimerspec::itimerspec::default();
    assert_eq!(unsafe { timerfd::timerfd_gettime(fd, &mut current) }, 0);
    assert_eq!(current.it_value.tv_sec, 0);
    assert_eq!(current.it_value.tv_nsec, 0);

    let timeout = struct_itimerspec::itimerspec {
        it_interval: struct_timespec::timespec::default(),
        it_value: struct_timespec::timespec {
            tv_sec: 0,
            tv_nsec: 1_000_000,
        },
    };
    assert_eq!(
        unsafe { timerfd::timerfd_settime(fd, 0, &timeout, std::ptr::null_mut()) },
        0
    );

    let mut expirations = 0u64;
    assert_eq!(
        unsafe {
            libc::read(
                fd,
                (&mut expirations as *mut u64).cast(),
                std::mem::size_of_val(&expirations),
            )
        },
        std::mem::size_of_val(&expirations) as isize
    );
    assert_eq!(expirations, 1);
    assert_eq!(unsafe { libc::close(fd) }, 0);
}

#[test]
fn timerfd_layout_and_constants_match_linux() {
    assert_eq!(
        std::mem::size_of::<struct_itimerspec::itimerspec>(),
        std::mem::size_of::<libc::itimerspec>()
    );
    assert_eq!(
        std::mem::align_of::<struct_itimerspec::itimerspec>(),
        std::mem::align_of::<libc::itimerspec>()
    );
    assert_eq!(timerfd::TFD_CLOEXEC, libc::TFD_CLOEXEC as u32);
    assert_eq!(timerfd::TFD_NONBLOCK, libc::TFD_NONBLOCK as u32);
    assert_eq!(timerfd::TFD_TIMER_ABSTIME, libc::TFD_TIMER_ABSTIME as u32);
    assert_eq!(
        timerfd::TFD_TIMER_CANCEL_ON_SET,
        libc::TFD_TIMER_CANCEL_ON_SET as u32
    );
}
