windows_link::link!("c" "C" fn sigstack(__ss : *mut sigstack, __oss : *mut sigstack) -> i32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct sigstack {
    pub ss_sp: *mut core::ffi::c_void,
    pub ss_onstack: i32,
}
