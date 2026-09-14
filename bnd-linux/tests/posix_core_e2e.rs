use bnd_linux::libc::{
    cookie_io_functions_t, dirent, dlfcn, errno, mman, mman_linux, stat, stdio, struct_stat,
    struct_timespec, time, unistd,
};
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;

unsafe extern "C" fn cookie_write(
    cookie: *mut core::ffi::c_void,
    buffer: *const i8,
    len: usize,
) -> i64 {
    let output = unsafe { &mut *cookie.cast::<Vec<u8>>() };
    let bytes = unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), len) };
    output.extend_from_slice(bytes);
    len as i64
}

unsafe extern "C" fn cookie_close(_cookie: *mut core::ffi::c_void) -> i32 {
    0
}

#[test]
fn unistd_pipe_and_stat_roundtrip() {
    let mut pipe_fds = [0; 2];
    assert_eq!(unsafe { unistd::pipe(pipe_fds.as_mut_ptr()) }, 0);

    let input = b"posix";
    assert_eq!(
        unsafe { unistd::write(pipe_fds[1], input.as_ptr().cast(), input.len()) },
        input.len() as i64
    );
    let mut output = [0u8; 5];
    assert_eq!(
        unsafe { unistd::read(pipe_fds[0], output.as_mut_ptr().cast(), output.len()) },
        output.len() as i64
    );
    assert_eq!(&output, input);

    assert_eq!(unsafe { unistd::close(pipe_fds[0]) }, 0);
    assert_eq!(unsafe { unistd::close(pipe_fds[1]) }, 0);
    assert_eq!(unsafe { unistd::getpid() }, unsafe { libc::getpid() });

    let file = tempfile::tempfile().expect("create stat test file");
    let mut generated = struct_stat::stat::default();
    assert_eq!(unsafe { stat::fstat(file.as_raw_fd(), &mut generated) }, 0);
    assert_eq!(
        std::mem::size_of::<struct_stat::stat>(),
        std::mem::size_of::<libc::stat>()
    );
}

#[test]
fn mmap_and_time_roundtrip() {
    let len = 4096;
    let address = unsafe {
        mman::mmap(
            std::ptr::null_mut(),
            len,
            mman_linux::PROT_READ | mman_linux::PROT_WRITE,
            mman_linux::MAP_PRIVATE | mman_linux::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert_ne!(address, libc::MAP_FAILED);
    let memory = unsafe { std::slice::from_raw_parts_mut(address.cast::<u8>(), len) };
    memory[..4].copy_from_slice(b"mmap");
    assert_eq!(&memory[..4], b"mmap");
    assert_eq!(unsafe { mman::munmap(address, len) }, 0);

    let mut now = struct_timespec::timespec::default();
    assert_eq!(
        unsafe { time::clock_gettime(time::CLOCK_MONOTONIC, &mut now) },
        0
    );
    assert!(now.tv_sec > 0);
}

#[test]
fn dirent_stdio_dlfcn_and_errno_are_callable() {
    let directory = tempfile::tempdir().expect("create directory");
    let directory_path =
        std::ffi::CString::new(directory.path().as_os_str().as_bytes()).expect("path contains NUL");
    let stream = unsafe { dirent::opendir(directory_path.as_ptr()) };
    assert!(!stream.is_null());
    assert!(!unsafe { dirent::readdir(stream) }.is_null());
    assert_eq!(unsafe { dirent::closedir(stream) }, 0);

    let file = tempfile::NamedTempFile::new().expect("create stdio file");
    let file_path =
        std::ffi::CString::new(file.path().as_os_str().as_bytes()).expect("path contains NUL");
    let stream = unsafe { stdio::fopen(file_path.as_ptr(), c"rb".as_ptr()) };
    assert!(!stream.is_null());
    assert_eq!(unsafe { stdio::fclose(stream) }, 0);

    let handle = unsafe { dlfcn::dlopen(std::ptr::null(), dlfcn::RTLD_NOW) };
    assert!(!handle.is_null());
    assert!(!unsafe { dlfcn::dlsym(handle, c"getpid".as_ptr()) }.is_null());
    assert_eq!(unsafe { dlfcn::dlclose(handle) }, 0);

    assert_eq!(unsafe { errno::__errno_location() }, unsafe {
        libc::__errno_location()
    });

    assert!(!unsafe { unistd::crypt(c"password".as_ptr(), c"aa".as_ptr()) }.is_null());
}

#[test]
fn fopencookie_invokes_generated_callback_fields() {
    let mut output = Box::new(Vec::new());
    let callbacks = cookie_io_functions_t::cookie_io_functions_t {
        read: None,
        write: Some(cookie_write),
        seek: None,
        close: Some(cookie_close),
    };
    let stream = unsafe {
        stdio::fopencookie(
            (&mut *output as *mut Vec<u8>).cast(),
            c"w".as_ptr(),
            callbacks,
        )
    };
    assert!(!stream.is_null());

    let input = b"cookie";
    assert_eq!(
        unsafe { stdio::fwrite(input.as_ptr().cast(), 1, input.len(), stream) },
        input.len() as u64
    );
    assert_eq!(unsafe { stdio::fclose(stream) }, 0);
    assert_eq!(&*output, input);
}

#[test]
fn sscanf_uses_c11_symbol_redirect() {
    #[repr(C)]
    struct Output {
        value: f32,
        guard: u32,
    }

    let mut output = Output {
        value: 0.0,
        guard: 0xfeed_beef,
    };
    assert_eq!(
        unsafe { stdio::sscanf(c"0x1p0s".as_ptr(), c"%as".as_ptr(), &mut output.value) },
        1
    );
    assert_eq!(output.value, 1.0);
    assert_eq!(output.guard, 0xfeed_beef);
}
