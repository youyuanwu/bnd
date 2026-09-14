#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct sched_param {
    pub sched_priority: i32,
}
