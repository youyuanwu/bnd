use bnd_linux::linux::inotify;

#[test]
fn inotify_init1_returns_valid_fd() {
    let ifd = unsafe { inotify::inotify_init1(0) };
    assert!(ifd >= 0, "inotify_init1 failed: {ifd}");
    unsafe { libc::close(ifd) };
}

#[test]
fn inotify_add_rm_watch() {
    let ifd = unsafe { inotify::inotify_init1(0) };
    assert!(ifd >= 0);

    let path = c"/tmp";
    let wd = unsafe { inotify::inotify_add_watch(ifd, path.as_ptr(), inotify::IN_CREATE as u32) };
    assert!(wd >= 0, "inotify_add_watch failed: {wd}");

    let ret = unsafe { inotify::inotify_rm_watch(ifd, wd) };
    assert_eq!(ret, 0, "inotify_rm_watch failed");

    unsafe { libc::close(ifd) };
}

#[test]
fn in_constants() {
    assert_eq!(inotify::IN_CREATE, 256);
    assert_eq!(inotify::IN_DELETE, 512);
    assert_eq!(inotify::IN_MODIFY, 2);
    assert_eq!(inotify::IN_MOVED_FROM, 64);
    assert_eq!(inotify::IN_MOVED_TO, 128);
    assert_eq!(inotify::IN_ACCESS, 1);
    assert_eq!(inotify::IN_OPEN, 32);
    assert_eq!(inotify::IN_CLOSE_WRITE, 8);
}
