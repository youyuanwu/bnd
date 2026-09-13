#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stack_t {
    pub ss_sp: *mut core::ffi::c_void,
    pub ss_flags: i32,
    pub ss_size: usize,
}
