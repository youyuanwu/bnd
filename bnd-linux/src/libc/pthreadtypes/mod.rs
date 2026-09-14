#[repr(C)]
#[derive(Clone, Copy)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union pthread_barrier_t {
    pub __size: [i8; 32],
    pub __align: i64,
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union pthread_barrierattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "atomic_wide_counter", feature = "thread_shared_types"))]
#[derive(Clone, Copy)]
pub union pthread_cond_t {
    pub __data: super::thread_shared_types::__pthread_cond_s,
    pub __size: [i8; 48],
    pub __align: i64,
}
#[cfg(all(feature = "atomic_wide_counter", feature = "thread_shared_types"))]
impl Default for pthread_cond_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union pthread_condattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type pthread_key_t = u32;
#[repr(C)]
#[cfg(all(feature = "struct_mutex", feature = "thread_shared_types"))]
#[derive(Clone, Copy)]
pub union pthread_mutex_t {
    pub __data: super::struct_mutex::__pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[cfg(all(feature = "struct_mutex", feature = "thread_shared_types"))]
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type pthread_once_t = i32;
#[repr(C)]
#[cfg(feature = "struct_rwlock")]
#[derive(Clone, Copy)]
pub union pthread_rwlock_t {
    pub __data: super::struct_rwlock::__pthread_rwlock_arch_t,
    pub __size: [i8; 56],
    pub __align: i64,
}
#[cfg(feature = "struct_rwlock")]
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union pthread_rwlockattr_t {
    pub __size: [i8; 8],
    pub __align: i64,
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type pthread_spinlock_t = i32;
pub type pthread_t = u64;
