#[repr(C)]
#[derive(Clone, Copy)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: __mbstate_t_0,
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union __mbstate_t_0 {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
impl Default for __mbstate_t_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
