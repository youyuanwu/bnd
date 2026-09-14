#[repr(C)]
#[derive(Clone, Copy)]
pub union __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: __atomic_wide_counter_0,
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct __atomic_wide_counter_0 {
    pub __low: u32,
    pub __high: u32,
}
