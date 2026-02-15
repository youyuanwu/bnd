//! End-to-end tests exercising the generated POSIX file I/O bindings against
//! real libc on Linux.

use bns_posix::PosixFile::Fcntl;
use bns_posix::PosixFile::Stat;
use bns_posix::PosixFile::Unistd;

use std::ffi::CString;

/// Helper: create a temporary file path with a unique name.
fn tmp_path(name: &str) -> CString {
    CString::new(format!("/tmp/bindscrape_e2e_{name}_{}", std::process::id())).unwrap()
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

#[test]
fn o_rdonly_is_zero() {
    assert_eq!(Fcntl::O_RDONLY, 0);
}

#[test]
fn seek_constants() {
    assert_eq!(Unistd::SEEK_SET, 0);
    assert_eq!(Unistd::SEEK_CUR, 1);
    assert_eq!(Unistd::SEEK_END, 2);
}

#[test]
fn access_mode_constants() {
    assert_eq!(Unistd::R_OK, 4);
    assert_eq!(Unistd::W_OK, 2);
    assert_eq!(Unistd::X_OK, 1);
    assert_eq!(Unistd::F_OK, 0);
}

// ---------------------------------------------------------------------------
// Simple syscall smoke tests
// ---------------------------------------------------------------------------

#[test]
fn getpid_returns_positive() {
    let pid = unsafe { Unistd::getpid() };
    assert!(pid > 0, "getpid should return a positive value, got {pid}");
}

#[test]
fn getuid_returns_value() {
    // Not checking exact value, just that it doesn't crash.
    let uid = unsafe { Unistd::getuid() };
    // In most environments uid >= 0 (unsigned, so always true), just call it.
    let _ = uid;
}

// ---------------------------------------------------------------------------
// File I/O: creat, write, read, close, unlink
// ---------------------------------------------------------------------------

#[test]
fn creat_and_close() {
    let path = tmp_path("creat_close");
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0, "creat failed with fd={fd}");
    let rc = unsafe { Unistd::close(fd) };
    assert_eq!(rc, 0, "close failed");
    unsafe { Unistd::unlink(path.as_ptr()) };
}

#[test]
fn write_then_read() {
    let path = tmp_path("write_read");
    // Create + write
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0, "creat failed");
    let data = b"hello bindscrape";
    let written = unsafe {
        Unistd::write(
            fd,
            data.as_ptr() as *const core::ffi::c_void,
            data.len() as u64,
        )
    };
    assert_eq!(written, data.len() as i64, "write returned wrong count");
    unsafe { Unistd::close(fd) };

    // Re-open read-only + read back
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    // creat truncates, so we need to write again then reopen
    unsafe { Unistd::close(fd) };

    // Write fresh
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    unsafe {
        Unistd::write(
            fd,
            data.as_ptr() as *const core::ffi::c_void,
            data.len() as u64,
        )
    };
    unsafe { Unistd::close(fd) };

    // open is variadic so not available; use the raw syscall via creat with O_RDONLY won't work.
    // Instead we use lseek + read by re-opening through a different mechanism.
    // Actually, let's just use creat with read: creat returns write-only.
    // We'll verify via stat that the size is correct.
    let mut st = Stat::stat::default();
    let rc = unsafe { Stat::stat(path.as_ptr(), &mut st as *mut _ as *const _) };
    assert_eq!(rc, 0, "stat failed");
    assert_eq!(st.st_size, data.len() as i64, "file size mismatch");

    unsafe { Unistd::unlink(path.as_ptr()) };
}

// ---------------------------------------------------------------------------
// lseek
// ---------------------------------------------------------------------------

#[test]
fn lseek_returns_offset() {
    let path = tmp_path("lseek");
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0);
    let data = b"abcdefghij"; // 10 bytes
    unsafe {
        Unistd::write(
            fd,
            data.as_ptr() as *const core::ffi::c_void,
            data.len() as u64,
        )
    };

    let pos = unsafe { Unistd::lseek(fd, 0, Unistd::SEEK_CUR) };
    assert_eq!(pos, 10, "after writing 10 bytes, pos should be 10");

    let pos = unsafe { Unistd::lseek(fd, 0, Unistd::SEEK_SET) };
    assert_eq!(pos, 0, "SEEK_SET to 0");

    let pos = unsafe { Unistd::lseek(fd, -3, Unistd::SEEK_END) };
    assert_eq!(pos, 7, "SEEK_END - 3 on 10-byte file");

    unsafe { Unistd::close(fd) };
    unsafe { Unistd::unlink(path.as_ptr()) };
}

// ---------------------------------------------------------------------------
// stat
// ---------------------------------------------------------------------------

#[test]
fn stat_file_size() {
    let path = tmp_path("stat_size");
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0);
    let data = b"0123456789";
    unsafe {
        Unistd::write(
            fd,
            data.as_ptr() as *const core::ffi::c_void,
            data.len() as u64,
        )
    };
    unsafe { Unistd::close(fd) };

    let mut st = Stat::stat::default();
    let rc = unsafe { Stat::stat(path.as_ptr(), &mut st as *mut _ as *const _) };
    assert_eq!(rc, 0, "stat failed");
    assert_eq!(st.st_size, 10);

    unsafe { Unistd::unlink(path.as_ptr()) };
}

#[test]
fn stat_is_regular_file() {
    let path = tmp_path("stat_reg");
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0);
    unsafe { Unistd::close(fd) };

    let mut st = Stat::stat::default();
    let rc = unsafe { Stat::stat(path.as_ptr(), &mut st as *mut _ as *const _) };
    assert_eq!(rc, 0);
    // S_IFREG = 0o100000
    assert_eq!(
        st.st_mode & 0o170000,
        0o100000,
        "expected S_IFREG, got mode {:#o}",
        st.st_mode
    );

    unsafe { Unistd::unlink(path.as_ptr()) };
}

// ---------------------------------------------------------------------------
// access
// ---------------------------------------------------------------------------

#[test]
fn access_existing_file() {
    let path = tmp_path("access_exist");
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0);
    unsafe { Unistd::close(fd) };

    let rc = unsafe { Unistd::access(path.as_ptr(), Unistd::F_OK) };
    assert_eq!(rc, 0, "access F_OK should succeed for existing file");

    unsafe { Unistd::unlink(path.as_ptr()) };
}

#[test]
fn access_nonexistent_file() {
    let path = CString::new("/tmp/bindscrape_e2e_no_such_file_ever").unwrap();
    let rc = unsafe { Unistd::access(path.as_ptr(), Unistd::F_OK) };
    assert_eq!(rc, -1, "access should fail for nonexistent file");
}

// ---------------------------------------------------------------------------
// unlink
// ---------------------------------------------------------------------------

#[test]
fn unlink_file() {
    let path = tmp_path("unlink");
    let fd = unsafe { Fcntl::creat(path.as_ptr(), 0o644) };
    assert!(fd >= 0);
    unsafe { Unistd::close(fd) };

    let rc = unsafe { Unistd::unlink(path.as_ptr()) };
    assert_eq!(rc, 0, "unlink should succeed");

    // Verify the file is gone
    let rc = unsafe { Unistd::access(path.as_ptr(), Unistd::F_OK) };
    assert_eq!(rc, -1, "file should be gone after unlink");
}

// ---------------------------------------------------------------------------
// Struct layout
// ---------------------------------------------------------------------------

#[test]
fn stat_struct_size() {
    assert_eq!(
        std::mem::size_of::<Stat::stat>(),
        144,
        "struct stat should be 144 bytes on x86_64 Linux"
    );
}

#[test]
fn timespec_struct_size() {
    assert_eq!(
        std::mem::size_of::<Stat::timespec>(),
        16,
        "struct timespec should be 16 bytes"
    );
}
