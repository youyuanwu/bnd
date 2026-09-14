pub type cookie_close_function_t =
    Option<unsafe extern "C" fn(__cookie: *mut core::ffi::c_void) -> i32>;
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy, Default)]
pub struct cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
#[cfg(feature = "types")]
pub type cookie_read_function_t = Option<
    unsafe extern "C" fn(
        __cookie: *mut core::ffi::c_void,
        __buf: *mut i8,
        __nbytes: usize,
    ) -> super::types::__ssize_t,
>;
#[cfg(feature = "types")]
pub type cookie_seek_function_t = Option<
    unsafe extern "C" fn(
        __cookie: *mut core::ffi::c_void,
        __pos: *mut super::types::__off64_t,
        __w: i32,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type cookie_write_function_t = Option<
    unsafe extern "C" fn(
        __cookie: *mut core::ffi::c_void,
        __buf: *const i8,
        __nbytes: usize,
    ) -> super::types::__ssize_t,
>;
