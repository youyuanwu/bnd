#[repr(C)]
#[cfg(all(feature = "struct_timespec", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct itimerspec {
    pub it_interval: super::struct_timespec::timespec,
    pub it_value: super::struct_timespec::timespec,
}
