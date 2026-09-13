#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct __once_flag {
    pub __data: i32,
}
#[repr(C)]
#[cfg(feature = "atomic_wide_counter")]
#[derive(Clone, Copy)]
pub struct __pthread_cond_s {
    pub __wseq: super::atomic_wide_counter::__atomic_wide_counter,
    pub __g1_start: super::atomic_wide_counter::__atomic_wide_counter,
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
    pub __unused_initialized_1: u32,
    pub __unused_initialized_2: u32,
}
#[cfg(feature = "atomic_wide_counter")]
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct __pthread_list_t {
    pub __prev: *mut Self,
    pub __next: *mut Self,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct __pthread_slist_t {
    pub __next: *mut Self,
}
pub type __thrd_t = u64;
pub type __tss_t = u32;
