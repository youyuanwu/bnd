#[repr(C)]
#[derive(Clone, Copy)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
impl Default for __sigset_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
