use bnd_linux::linux::sendfile;

#[test]
fn sendfile_between_fds() {
    use std::ffi::CString;

    unsafe {
        // Create a temp file and write data
        let path = CString::new("/tmp/bnd_sendfile_test").unwrap();
        let fd_in = libc::open(
            path.as_ptr(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        );
        assert!(fd_in >= 0, "open source failed");

        let data = b"hello sendfile";
        libc::write(fd_in, data.as_ptr() as *const _, data.len());
        libc::lseek(fd_in, 0, libc::SEEK_SET);

        // Create a pipe for the output
        let mut pipefd = [0i32; 2];
        libc::pipe(pipefd.as_mut_ptr());

        // sendfile from file to pipe
        let mut offset: i64 = 0;
        let n = sendfile::sendfile(pipefd[1], fd_in, &mut offset as *mut _, data.len() as u64);
        assert_eq!(n, data.len() as i64, "sendfile returned {n}");

        // Read from pipe and verify
        let mut buf = [0u8; 64];
        let nread = libc::read(pipefd[0], buf.as_mut_ptr() as *mut _, buf.len());
        assert_eq!(nread as usize, data.len());
        assert_eq!(&buf[..data.len()], data);

        libc::close(fd_in);
        libc::close(pipefd[0]);
        libc::close(pipefd[1]);
        libc::unlink(path.as_ptr());
    }
}
