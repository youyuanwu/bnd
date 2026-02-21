#![allow(clippy::unnecessary_mut_passed)]

use bnd_linux::linux::timerfd;
use bnd_posix::posix::time::itimerspec;

#[test]
fn timerfd_create_monotonic() {
    // CLOCK_MONOTONIC = 1
    let tfd = unsafe { timerfd::timerfd_create(1, 0) };
    assert!(tfd >= 0, "timerfd_create failed: {tfd}");
    unsafe { libc::close(tfd) };
}

#[test]
fn timerfd_settime_gettime_roundtrip() {
    let tfd = unsafe { timerfd::timerfd_create(1, 0) };
    assert!(tfd >= 0);

    // Set a 1-second one-shot timer
    let new_value = itimerspec {
        it_interval: bnd_posix::posix::stat::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        it_value: bnd_posix::posix::stat::timespec {
            tv_sec: 1,
            tv_nsec: 0,
        },
    };
    let mut old_value = itimerspec::default();
    let ret = unsafe {
        timerfd::timerfd_settime(
            tfd,
            0,
            &new_value as *const _ as *const _,
            &mut old_value as *mut _ as *mut _,
        )
    };
    assert_eq!(ret, 0, "timerfd_settime failed");

    // Read it back
    let mut cur = itimerspec::default();
    let ret = unsafe { timerfd::timerfd_gettime(tfd, &mut cur as *mut _ as *mut _) };
    assert_eq!(ret, 0, "timerfd_gettime failed");
    // Timer should still have time remaining (we just set it)
    assert!(cur.it_value.tv_sec > 0 || cur.it_value.tv_nsec > 0);

    unsafe { libc::close(tfd) };
}

#[test]
fn tfd_constants() {
    assert_eq!(timerfd::TFD_CLOEXEC, 524288);
    assert_eq!(timerfd::TFD_NONBLOCK, 2048);
    assert_eq!(timerfd::TFD_TIMER_ABSTIME, 1);
    assert_eq!(timerfd::TFD_TIMER_CANCEL_ON_SET, 2);
}
