#![allow(clippy::unnecessary_mut_passed)]

use bnd_linux::linux::epoll;

#[test]
fn epoll_create1_returns_valid_fd() {
    let epfd = unsafe { epoll::epoll_create1(0) };
    assert!(epfd >= 0, "epoll_create1 failed: {epfd}");
    unsafe { libc::close(epfd) };
}

#[test]
fn epoll_ctl_add_eventfd() {
    let epfd = unsafe { epoll::epoll_create1(0) };
    assert!(epfd >= 0);

    let efd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK) };
    assert!(efd >= 0);

    let mut ev = epoll::epoll_event {
        events: epoll::EPOLLIN,
        ..Default::default()
    };
    ev.data.Value.fd = efd;

    let ret = unsafe { epoll::epoll_ctl(epfd, epoll::EPOLL_CTL_ADD, efd, &mut ev) };
    assert_eq!(ret, 0, "epoll_ctl ADD failed");

    unsafe {
        libc::close(efd);
        libc::close(epfd);
    };
}

#[test]
fn epoll_wait_eventfd_readable() {
    let epfd = unsafe { epoll::epoll_create1(0) };
    assert!(epfd >= 0);

    let efd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK) };
    assert!(efd >= 0);

    let mut ev = epoll::epoll_event {
        events: epoll::EPOLLIN,
        ..Default::default()
    };
    ev.data.Value.fd = efd;
    unsafe { epoll::epoll_ctl(epfd, epoll::EPOLL_CTL_ADD, efd, &mut ev) };

    // Write to eventfd to make it readable
    let val: u64 = 1;
    unsafe { libc::write(efd, &val as *const u64 as *const _, 8) };

    let mut events = [epoll::epoll_event::default(); 4];
    let n = unsafe { epoll::epoll_wait(epfd, events.as_mut_ptr(), 4, 100) };
    assert_eq!(n, 1, "expected 1 event, got {n}");
    assert_ne!(events[0].events & epoll::EPOLLIN, 0, "expected EPOLLIN");

    unsafe {
        libc::close(efd);
        libc::close(epfd);
    };
}

#[test]
fn epoll_event_constants() {
    assert_eq!(epoll::EPOLLIN, 0x001);
    assert_eq!(epoll::EPOLLPRI, 0x002);
    assert_eq!(epoll::EPOLLOUT, 0x004);
    assert_eq!(epoll::EPOLLERR, 0x008);
    assert_eq!(epoll::EPOLLHUP, 0x010);
    assert_eq!(epoll::EPOLLRDHUP, 0x2000);
}

#[test]
fn epoll_ctl_constants() {
    assert_eq!(epoll::EPOLL_CTL_ADD, 1);
    assert_eq!(epoll::EPOLL_CTL_DEL, 2);
    assert_eq!(epoll::EPOLL_CTL_MOD, 3);
}

#[test]
fn epoll_event_struct_size() {
    assert_eq!(core::mem::size_of::<epoll::epoll_event>(), 12);
    assert_eq!(core::mem::size_of::<epoll::epoll_data>(), 8);
}
