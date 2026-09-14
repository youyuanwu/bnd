#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct iovec {
    pub iov_base: *mut core::ffi::c_void,
    pub iov_len: usize,
}
