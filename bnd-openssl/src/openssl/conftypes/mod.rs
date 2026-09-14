#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct conf_method_st {
    pub name: *const i8,
    pub create: *mut u8,
    pub init: *mut u8,
    pub destroy: *mut u8,
    pub destroy_data: *mut u8,
    pub load_bio: *mut u8,
    pub dump: *mut u8,
    pub is_number: *mut u8,
    pub to_int: *mut u8,
    pub load: *mut u8,
}
#[repr(C)]
#[cfg(all(feature = "conf", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct conf_st {
    pub meth: *mut super::conf::CONF_METHOD,
    pub meth_data: *mut core::ffi::c_void,
    pub data: *mut core::ffi::c_void,
    pub flag_dollarid: i32,
    pub flag_abspath: i32,
    pub includedir: *mut i8,
    pub libctx: *mut super::types::OSSL_LIB_CTX,
}
