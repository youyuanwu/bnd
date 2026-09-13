#[repr(C)]
#[cfg(all(feature = "__sigset_t", feature = "setjmp"))]
#[derive(Clone, Copy)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: super::setjmp::__jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: super::__sigset_t::__sigset_t,
}
#[cfg(all(feature = "__sigset_t", feature = "setjmp"))]
impl Default for __jmp_buf_tag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
