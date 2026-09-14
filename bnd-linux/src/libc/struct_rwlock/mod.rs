#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __pad1: u64,
    pub __pad2: u64,
    pub __flags: u32,
}
