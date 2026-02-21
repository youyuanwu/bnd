use bnd_linux::linux::xattr;
use std::ffi::CString;

fn with_tmpfile(f: impl FnOnce(&CString)) {
    let path = CString::new("/tmp/bnd_xattr_test").unwrap();
    unsafe {
        let fd = libc::open(
            path.as_ptr(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        );
        assert!(fd >= 0);
        libc::close(fd);
    }
    f(&path);
    unsafe { libc::unlink(path.as_ptr()) };
}

#[test]
fn setxattr_getxattr_roundtrip() {
    with_tmpfile(|path| {
        let name = c"user.test";
        let value = b"hello";

        let ret = unsafe {
            xattr::setxattr(
                path.as_ptr(),
                name.as_ptr(),
                value.as_ptr() as *const _,
                value.len() as u64,
                0,
            )
        };
        assert_eq!(ret, 0, "setxattr failed");

        let mut buf = [0u8; 64];
        let n = unsafe {
            xattr::getxattr(
                path.as_ptr(),
                name.as_ptr(),
                buf.as_mut_ptr() as *mut _,
                buf.len() as u64,
            )
        };
        assert_eq!(n, value.len() as i64);
        assert_eq!(&buf[..value.len()], value);
    });
}

#[test]
fn listxattr_contains_attr() {
    with_tmpfile(|path| {
        let name = c"user.myattr";
        let value = b"val";

        unsafe {
            xattr::setxattr(
                path.as_ptr(),
                name.as_ptr(),
                value.as_ptr() as *const _,
                value.len() as u64,
                0,
            );
        }

        let mut buf = [0u8; 256];
        let n = unsafe {
            xattr::listxattr(path.as_ptr(), buf.as_mut_ptr() as *mut _, buf.len() as u64)
        };
        assert!(n > 0, "listxattr returned {n}");

        let list = String::from_utf8_lossy(&buf[..n as usize]);
        assert!(
            list.contains("user.myattr"),
            "listxattr output does not contain user.myattr: {list:?}"
        );
    });
}

#[test]
fn removexattr_removes_attr() {
    with_tmpfile(|path| {
        let name = c"user.removeme";
        let value = b"gone";

        unsafe {
            xattr::setxattr(
                path.as_ptr(),
                name.as_ptr(),
                value.as_ptr() as *const _,
                value.len() as u64,
                0,
            );
        }

        let ret = unsafe { xattr::removexattr(path.as_ptr(), name.as_ptr()) };
        assert_eq!(ret, 0, "removexattr failed");

        // getxattr should now fail with ENODATA
        let mut buf = [0u8; 64];
        let n = unsafe {
            xattr::getxattr(
                path.as_ptr(),
                name.as_ptr(),
                buf.as_mut_ptr() as *mut _,
                buf.len() as u64,
            )
        };
        assert!(n < 0, "getxattr should fail after removexattr");
    });
}

#[test]
fn xattr_constants() {
    assert_eq!(xattr::XATTR_CREATE, 1);
    assert_eq!(xattr::XATTR_REPLACE, 2);
}
