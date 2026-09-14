use bnd_linux::libc::inotify;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

#[repr(align(8))]
struct EventBuffer([u8; 4096]);

#[test]
fn inotify_reports_created_file() {
    let fd = unsafe { inotify::inotify_init1(0) };
    assert!(fd >= 0, "inotify_init1 failed: {fd}");

    let directory = tempfile::tempdir().expect("create watched directory");
    let path = CString::new(directory.path().as_os_str().as_bytes()).expect("path contains NUL");
    let watch = unsafe { inotify::inotify_add_watch(fd, path.as_ptr(), inotify::IN_CREATE as u32) };
    assert!(watch >= 0, "inotify_add_watch failed: {watch}");

    std::fs::write(directory.path().join("created.txt"), b"created").expect("create watched file");

    let mut buffer = EventBuffer([0; 4096]);
    let read = unsafe {
        libc::read(
            fd,
            buffer.0.as_mut_ptr().cast(),
            std::mem::size_of_val(&buffer.0),
        )
    };
    assert!(
        read >= std::mem::size_of::<inotify::inotify_event>() as isize,
        "inotify read failed: {read}"
    );

    let event = unsafe { &*buffer.0.as_ptr().cast::<inotify::inotify_event>() };
    assert_eq!(event.wd, watch);
    assert_ne!(event.mask & inotify::IN_CREATE as u32, 0);
    assert!(event.len > 0);

    let name_offset = std::mem::offset_of!(inotify::inotify_event, name);
    let name_end = name_offset + event.len as usize;
    assert!(name_end <= read as usize);
    let name = &buffer.0[name_offset..name_end];
    let name_len = name
        .iter()
        .position(|&byte| byte == 0)
        .expect("event name NUL");
    assert_eq!(&name[..name_len], b"created.txt");

    assert_eq!(unsafe { inotify::inotify_rm_watch(fd, watch) }, 0);
    assert_eq!(unsafe { libc::close(fd) }, 0);
}

#[test]
fn inotify_layout_and_constants_match_linux() {
    assert_eq!(std::mem::size_of::<inotify::inotify_event>(), 16);
    assert_eq!(std::mem::align_of::<inotify::inotify_event>(), 4);
    assert_eq!(inotify::IN_CLOEXEC, libc::IN_CLOEXEC as u32);
    assert_eq!(inotify::IN_NONBLOCK, libc::IN_NONBLOCK as u32);
    assert_eq!(inotify::IN_CREATE, libc::IN_CREATE as i32);
    assert_eq!(inotify::IN_CLOSE, libc::IN_CLOSE as i32);
    assert_eq!(inotify::IN_MOVE, libc::IN_MOVE as i32);
    assert_eq!(inotify::IN_ALL_EVENTS, libc::IN_ALL_EVENTS as i32);
}
