#[repr(C)]
#[derive(Clone, Copy)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Default for __fsid_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
