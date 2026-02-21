use bnd_linux::linux::eventfd;

#[test]
fn eventfd_create_close() {
    let efd = unsafe { eventfd::eventfd(0, 0) };
    assert!(efd >= 0, "eventfd failed: {efd}");
    unsafe { libc::close(efd) };
}

#[test]
fn eventfd_write_read_roundtrip() {
    let efd = unsafe { eventfd::eventfd(0, 0) };
    assert!(efd >= 0);

    let ret = unsafe { eventfd::eventfd_write(efd, 42) };
    assert_eq!(ret, 0, "eventfd_write failed");

    let mut val: eventfd::eventfd_t = 0;
    let ret = unsafe { eventfd::eventfd_read(efd, &mut val) };
    assert_eq!(ret, 0, "eventfd_read failed");
    assert_eq!(val, 42);

    unsafe { libc::close(efd) };
}

#[test]
fn efd_nonblock_constant() {
    assert_eq!(eventfd::EFD_NONBLOCK, 2048);
}

#[test]
fn efd_cloexec_constant() {
    assert_eq!(eventfd::EFD_CLOEXEC, 524288);
}

#[test]
fn efd_semaphore_constant() {
    assert_eq!(eventfd::EFD_SEMAPHORE, 1);
}
