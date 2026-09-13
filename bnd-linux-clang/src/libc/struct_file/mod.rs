#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _IO_codecvt(pub u8);
pub type _IO_lock_t = core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _IO_marker(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _IO_wide_data(pub u8);
#[repr(C, align(8))]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct __FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut Self,
    pub _fileno: i32,
    pub _bitfield: [u8; 3],
    pub _short_backupbuf: [i8; 1],
    pub _old_offset: super::types::__off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut _IO_lock_t,
    pub _offset: super::types::__off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut Self,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub _prevchain: *mut *mut Self,
    pub _mode: i32,
    pub _unused3: i32,
    pub _total_written: super::types::__uint64_t,
    pub _unused2: [i8; 8],
}
#[cfg(feature = "types")]
impl Default for __FILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
