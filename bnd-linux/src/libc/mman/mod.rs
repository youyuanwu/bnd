windows_link::link!("c" "C" fn madvise(__addr : *mut core::ffi::c_void, __len : usize, __advice : i32) -> i32);
windows_link::link!("c" "C" fn mincore(__start : *mut core::ffi::c_void, __len : usize, __vec : *mut u8) -> i32);
windows_link::link!("c" "C" fn mlock(__addr : *const core::ffi::c_void, __len : usize) -> i32);
windows_link::link!("c" "C" fn mlockall(__flags : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn mmap(__addr : *mut core::ffi::c_void, __len : usize, __prot : i32, __flags : i32, __fd : i32, __offset : super::types::__off_t) -> *mut core::ffi::c_void);
windows_link::link!("c" "C" fn mprotect(__addr : *mut core::ffi::c_void, __len : usize, __prot : i32) -> i32);
windows_link::link!("c" "C" fn msync(__addr : *mut core::ffi::c_void, __len : usize, __flags : i32) -> i32);
windows_link::link!("c" "C" fn munlock(__addr : *const core::ffi::c_void, __len : usize) -> i32);
windows_link::link!("c" "C" fn munlockall() -> i32);
windows_link::link!("c" "C" fn munmap(__addr : *mut core::ffi::c_void, __len : usize) -> i32);
windows_link::link!("c" "C" fn posix_madvise(__addr : *mut core::ffi::c_void, __len : usize, __advice : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn shm_open(__name : *const i8, __oflag : i32, __mode : super::types::mode_t) -> i32);
windows_link::link!("c" "C" fn shm_unlink(__name : *const i8) -> i32);
pub const MAP_32BIT: i32 = 64;
pub const MAP_ABOVE4G: i32 = 128;
pub const SHADOW_STACK_SET_TOKEN: i32 = 1;
