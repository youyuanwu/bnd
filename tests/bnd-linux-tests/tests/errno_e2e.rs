//! End-to-end tests for errno bindings against real libc.

use bnd_linux::libc::{errno, errno_base};

#[test]
fn basic_errno_constants() {
    assert_eq!(errno_base::EPERM, 1);
    assert_eq!(errno_base::ENOENT, 2);
    assert_eq!(errno_base::ESRCH, 3);
    assert_eq!(errno_base::EINTR, 4);
    assert_eq!(errno_base::EIO, 5);
    assert_eq!(errno_base::EBADF, 9);
    assert_eq!(errno_base::ENOMEM, 12);
    assert_eq!(errno_base::EACCES, 13);
    assert_eq!(errno_base::EEXIST, 17);
    assert_eq!(errno_base::EINVAL, 22);
}

#[test]
fn network_errno_constants() {
    assert_eq!(errno::ECONNREFUSED, 111);
    assert_eq!(errno::ECONNRESET, 104);
    assert_eq!(errno::EADDRINUSE, 98);
    assert_eq!(errno::ETIMEDOUT, 110);
    assert_eq!(errno::ENETUNREACH, 101);
}

#[test]
fn errno_location_returns_valid_pointer() {
    unsafe {
        let ptr = errno::__errno_location();
        assert!(
            !ptr.is_null(),
            "__errno_location should return a non-null pointer"
        );

        // Reading errno should not crash — value is whatever it happens to be
        let _val = *ptr;
    }
}

#[test]
fn errno_set_and_read() {
    unsafe {
        let ptr = errno::__errno_location();

        // Set errno to a known value
        *ptr = 0;
        assert_eq!(*ptr, 0);

        // Set to EINVAL and read back
        *ptr = errno_base::EINVAL;
        assert_eq!(*ptr, errno_base::EINVAL);

        // Reset
        *ptr = 0;
    }
}

#[test]
fn errno_reflects_failed_syscall() {
    // Open a nonexistent file — this will set errno to ENOENT
    unsafe {
        let ptr = errno::__errno_location();
        *ptr = 0; // clear

        // Use libc open via our unistd bindings to trigger ENOENT
        let path = c"/nonexistent/path/that/does/not/exist";
        let fd = bnd_linux::libc::unistd::access(path.as_ptr(), 0);
        assert_eq!(fd, -1, "access() should fail for nonexistent path");

        let err = *errno::__errno_location();
        assert_eq!(
            err,
            errno_base::ENOENT,
            "errno should be ENOENT after failed access()"
        );
    }
}
