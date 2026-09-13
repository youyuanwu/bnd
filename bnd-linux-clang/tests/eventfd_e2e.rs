use bnd_linux_clang::libc::eventfd;

#[test]
fn eventfd_create_close() {
    let fd = unsafe { eventfd::eventfd(0, 0) };
    assert!(fd >= 0, "eventfd failed: {fd}");
    assert_eq!(unsafe { libc::close(fd) }, 0);
}

#[test]
fn eventfd_write_read_roundtrip() {
    let fd = unsafe { eventfd::eventfd(0, 0) };
    assert!(fd >= 0, "eventfd failed: {fd}");

    assert_eq!(unsafe { eventfd::eventfd_write(fd, 42) }, 0);

    let mut value: eventfd::eventfd_t = 0;
    assert_eq!(unsafe { eventfd::eventfd_read(fd, &mut value) }, 0);
    assert_eq!(value, 42);

    assert_eq!(unsafe { libc::close(fd) }, 0);
}

#[test]
fn eventfd_constants_match_linux() {
    assert_eq!(eventfd::EFD_NONBLOCK, libc::EFD_NONBLOCK as u32);
    assert_eq!(eventfd::EFD_CLOEXEC, libc::EFD_CLOEXEC as u32);
    assert_eq!(eventfd::EFD_SEMAPHORE, libc::EFD_SEMAPHORE as u32);
}
