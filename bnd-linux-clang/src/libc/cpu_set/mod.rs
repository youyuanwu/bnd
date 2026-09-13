windows_link::link!("c" "C" fn __sched_cpualloc(__count : usize) -> *mut cpu_set_t);
windows_link::link!("c" "C" fn __sched_cpucount(__setsize : usize, __setp : *const cpu_set_t) -> i32);
windows_link::link!("c" "C" fn __sched_cpufree(__set : *mut cpu_set_t));
pub type __cpu_mask = u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
impl Default for cpu_set_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
