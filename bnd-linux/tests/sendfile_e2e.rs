use bnd_linux::libc::{sendfile, types};
use std::io::Write;

#[test]
fn sendfile_uses_posix_types_and_transfers_data() {
    let mut source = tempfile::tempfile().expect("create source file");
    source
        .write_all(b"hello sendfile")
        .expect("write source file");

    let mut pipe_fds = [0; 2];
    assert_eq!(unsafe { libc::pipe(pipe_fds.as_mut_ptr()) }, 0);

    let mut offset: types::off_t = 0;
    let written = unsafe {
        sendfile::sendfile(
            pipe_fds[1],
            std::os::fd::AsRawFd::as_raw_fd(&source),
            &mut offset,
            14,
        )
    };
    assert_eq!(written, 14);
    assert_eq!(offset, 14);

    let mut output = [0; 14];
    assert_eq!(
        unsafe { libc::read(pipe_fds[0], output.as_mut_ptr().cast(), output.len()) },
        14
    );
    assert_eq!(&output, b"hello sendfile");

    unsafe {
        libc::close(pipe_fds[0]);
        libc::close(pipe_fds[1]);
    }
}

#[test]
fn generated_posix_types_match_host_abi() {
    assert_eq!(std::mem::size_of::<types::off_t>(), 8);
    assert_eq!(std::mem::size_of::<types::ssize_t>(), 8);
}
