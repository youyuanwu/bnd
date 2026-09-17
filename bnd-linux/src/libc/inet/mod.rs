#[cfg(feature = "in_")]
windows_link::link!("c" "C" fn inet_addr(__cp : *const i8) -> super::in_::in_addr_t);
#[cfg(feature = "in_")]
windows_link::link!("c" "C" fn inet_aton(__cp : *const i8, __inp : *mut super::in_::in_addr) -> i32);
#[cfg(feature = "in_")]
windows_link::link!("c" "C" fn inet_lnaof(__in : super::in_::in_addr) -> super::in_::in_addr_t);
#[cfg(feature = "in_")]
windows_link::link!("c" "C" fn inet_makeaddr(__net : super::in_::in_addr_t, __host : super::in_::in_addr_t) -> super::in_::in_addr);
windows_link::link!("resolv" "C" fn inet_net_ntop(__af : i32, __cp : *const core::ffi::c_void, __bits : i32, __buf : *mut i8, __len : usize) -> *mut i8);
windows_link::link!("resolv" "C" fn inet_net_pton(__af : i32, __cp : *const i8, __buf : *mut core::ffi::c_void, __len : usize) -> i32);
#[cfg(feature = "in_")]
windows_link::link!("resolv" "C" fn inet_neta(__net : super::in_::in_addr_t, __buf : *mut i8, __len : usize) -> *mut i8);
#[cfg(feature = "in_")]
windows_link::link!("c" "C" fn inet_netof(__in : super::in_::in_addr) -> super::in_::in_addr_t);
#[cfg(feature = "in_")]
windows_link::link!("c" "C" fn inet_network(__cp : *const i8) -> super::in_::in_addr_t);
windows_link::link!("c" "C" fn inet_nsap_addr(__cp : *const i8, __buf : *mut u8, __len : i32) -> u32);
windows_link::link!("c" "C" fn inet_nsap_ntoa(__len : i32, __cp : *const u8, __buf : *mut i8) -> *mut i8);
#[cfg(feature = "in_")]
windows_link::link!("c" "C" fn inet_ntoa(__in : super::in_::in_addr) -> *mut i8);
#[cfg(all(feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn inet_ntop(__af : i32, __cp : *const core::ffi::c_void, __buf : *mut i8, __len : super::unistd::socklen_t) -> *const i8);
windows_link::link!("c" "C" fn inet_pton(__af : i32, __cp : *const i8, __buf : *mut core::ffi::c_void) -> i32);
