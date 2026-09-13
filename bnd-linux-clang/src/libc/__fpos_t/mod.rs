#[repr(C)]
#[cfg(all(feature = "__mbstate_t", feature = "types"))]
#[derive(Clone, Copy)]
pub struct __fpos_t {
    pub __pos: super::types::__off_t,
    pub __state: super::__mbstate_t::__mbstate_t,
}
#[cfg(all(feature = "__mbstate_t", feature = "types"))]
impl Default for __fpos_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
