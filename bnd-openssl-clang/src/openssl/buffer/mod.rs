#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct buf_mem_st {
    pub length: usize,
    pub data: *mut i8,
    pub max: usize,
    pub flags: u64,
}
