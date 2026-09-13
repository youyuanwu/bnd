#![allow(clippy::unnecessary_mut_passed)]

use bnd_linux_clang::libc::linux::{epoll, eventfd};

#[test]
fn epoll_waits_for_generated_eventfd() {
    let epfd = unsafe { epoll::epoll_create1(0) };
    assert!(epfd >= 0, "epoll_create1 failed: {epfd}");

    let eventfd = unsafe { eventfd::eventfd(0, eventfd::EFD_NONBLOCK as i32) };
    assert!(eventfd >= 0, "eventfd failed: {eventfd}");

    let mut event = epoll::epoll_event {
        events: epoll::EPOLLIN,
        ..Default::default()
    };
    event.data.fd = eventfd;

    assert_eq!(
        unsafe { epoll::epoll_ctl(epfd, epoll::EPOLL_CTL_ADD, eventfd, &mut event) },
        0
    );
    assert_eq!(unsafe { eventfd::eventfd_write(eventfd, 1) }, 0);

    let mut events = [epoll::epoll_event::default(); 1];
    assert_eq!(
        unsafe { epoll::epoll_wait(epfd, events.as_mut_ptr(), 1, 100) },
        1
    );
    assert_ne!(events[0].events & epoll::EPOLLIN, 0);

    unsafe {
        libc::close(eventfd);
        libc::close(epfd);
    }
}

#[test]
fn epoll_layout_and_constants_match_linux() {
    assert_eq!(std::mem::size_of::<epoll::epoll_data_t>(), 8);
    assert_eq!(std::mem::size_of::<epoll::epoll_event>(), 12);
    assert_eq!(epoll::EPOLL_CTL_ADD, libc::EPOLL_CTL_ADD);
    assert_eq!(epoll::EPOLL_CTL_DEL, libc::EPOLL_CTL_DEL);
    assert_eq!(epoll::EPOLL_CTL_MOD, libc::EPOLL_CTL_MOD);
}
