#[cfg(feature = "types")]
pub type OSSL_CALLBACK = Option<
    unsafe extern "C" fn(params: *mut super::types::OSSL_PARAM, arg: *mut core::ffi::c_void) -> i32,
>;
pub type OSSL_CORE_BIO = ossl_core_bio_st;
pub type OSSL_CORE_HANDLE = ossl_core_handle_st;
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy, Default)]
pub struct ossl_algorithm_st {
    pub algorithm_names: *const i8,
    pub property_definition: *const i8,
    pub implementation: *const super::types::OSSL_DISPATCH,
    pub algorithm_description: *const i8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_core_bio_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_core_handle_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_dispatch_st {
    pub function_id: i32,
    pub function: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_item_st {
    pub id: u32,
    pub ptr: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_param_st {
    pub key: *const i8,
    pub data_type: u32,
    pub data: *mut core::ffi::c_void,
    pub data_size: usize,
    pub return_size: usize,
}
