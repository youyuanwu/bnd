//! End-to-end tests for Stat bindings against real libc.

use bnd_linux::libc::{fcntl, struct_stat, struct_timespec, unistd};

use std::ffi::CString;

fn tmp_path(name: &str) -> CString {
    CString::new(format!("/tmp/bnd_linux_e2e_{name}_{}", std::process::id())).unwrap()
}

#[test]
fn stat_file_size() {
    let path = tmp_path("stat_size");
    let fd = unsafe { fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0);
    let data = b"0123456789";
    unsafe { unistd::write(fd, data.as_ptr() as *const core::ffi::c_void, data.len()) };
    unsafe { unistd::close(fd) };

    let mut st = struct_stat::stat::default();
    let rc = unsafe { struct_stat::stat(path.as_ptr(), &mut st) };
    assert_eq!(rc, 0, "stat failed");
    assert_eq!(st.st_size, 10);

    unsafe { unistd::unlink(path.as_ptr()) };
}

#[test]
fn stat_is_regular_file() {
    let path = tmp_path("stat_reg");
    let fd = unsafe { fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0);
    unsafe { unistd::close(fd) };

    let mut st = struct_stat::stat::default();
    let rc = unsafe { struct_stat::stat(path.as_ptr(), &mut st) };
    assert_eq!(rc, 0);
    assert_eq!(
        st.st_mode & 0o170000,
        0o100000,
        "expected S_IFREG, got mode {:#o}",
        st.st_mode
    );

    unsafe { unistd::unlink(path.as_ptr()) };
}

#[test]
fn stat_struct_size() {
    assert_eq!(
        std::mem::size_of::<struct_stat::stat>(),
        144,
        "struct stat should be 144 bytes on x86_64 Linux"
    );
}

#[test]
fn timespec_struct_size() {
    assert_eq!(
        std::mem::size_of::<struct_timespec::timespec>(),
        16,
        "struct timespec should be 16 bytes"
    );
}
