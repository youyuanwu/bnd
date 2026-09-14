#[cfg(feature = "types")]
windows_link::link!("c" "C" fn sendfile(__out_fd : i32, __in_fd : i32, __offset : *mut super::types::off_t, __count : usize) -> super::types::ssize_t);
