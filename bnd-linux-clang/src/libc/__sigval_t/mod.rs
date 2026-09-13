pub type __sigval_t = sigval;
#[repr(C)]
#[derive(Clone, Copy)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut core::ffi::c_void,
}
impl Default for sigval {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
