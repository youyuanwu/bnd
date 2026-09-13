use bnd_linux_clang::libc::xattr;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

#[test]
fn xattr_set_get_list_remove_roundtrip() {
    let file = tempfile::NamedTempFile::new().expect("create xattr test file");
    let path = CString::new(file.path().as_os_str().as_bytes()).expect("path contains NUL");
    let name = c"user.bnd_test";
    let value = b"generated xattr";

    let result = unsafe {
        xattr::setxattr(
            path.as_ptr(),
            name.as_ptr(),
            value.as_ptr().cast(),
            value.len(),
            0,
        )
    };
    if result == -1 {
        let error = std::io::Error::last_os_error();
        let errno = error.raw_os_error();
        if errno == Some(libc::ENOTSUP) || errno == Some(libc::EOPNOTSUPP) {
            eprintln!("skipping xattr roundtrip: temporary filesystem does not support xattrs");
            return;
        }
        panic!("setxattr failed: {error}");
    }

    let mut output = [0u8; 64];
    let read = unsafe {
        xattr::getxattr(
            path.as_ptr(),
            name.as_ptr(),
            output.as_mut_ptr().cast(),
            output.len(),
        )
    };
    assert_eq!(read as usize, value.len());
    assert_eq!(&output[..read as usize], value);

    let mut names = [0i8; 128];
    let listed = unsafe { xattr::listxattr(path.as_ptr(), names.as_mut_ptr(), names.len()) };
    assert!(listed > 0, "listxattr failed: {listed}");
    let names = unsafe { std::slice::from_raw_parts(names.as_ptr().cast::<u8>(), listed as usize) };
    assert!(
        names
            .split(|&byte| byte == 0)
            .any(|entry| entry == name.to_bytes())
    );

    assert_eq!(
        unsafe { xattr::removexattr(path.as_ptr(), name.as_ptr()) },
        0
    );
    assert_eq!(
        unsafe { xattr::getxattr(path.as_ptr(), name.as_ptr(), std::ptr::null_mut(), 0) },
        -1
    );
}

#[test]
fn xattr_constants_match_linux() {
    assert_eq!(xattr::XATTR_CREATE, libc::XATTR_CREATE as u32);
    assert_eq!(xattr::XATTR_REPLACE, libc::XATTR_REPLACE as u32);
}
