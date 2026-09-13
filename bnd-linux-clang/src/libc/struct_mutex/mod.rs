#[repr(C)]
#[cfg(feature = "thread_shared_types")]
#[derive(Clone, Copy, Default)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __unused: i16,
    pub __list: super::thread_shared_types::__pthread_list_t,
}
