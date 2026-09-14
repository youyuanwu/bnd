#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct __locale_data(pub u8);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const i8; 13],
}
impl Default for __locale_struct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type __locale_t = *mut __locale_struct;
