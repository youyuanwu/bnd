#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn __cmsg_nxthdr(__mhdr : *mut msghdr, __cmsg : *mut cmsghdr) -> *mut cmsghdr);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn accept(__fd : i32, __addr : *mut sockaddr, __addr_len : *mut super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn bind(__fd : i32, __addr : *const sockaddr, __len : super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn connect(__fd : i32, __addr : *const sockaddr, __len : super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn getpeername(__fd : i32, __addr : *mut sockaddr, __len : *mut super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn getsockname(__fd : i32, __addr : *mut sockaddr, __len : *mut super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn getsockopt(__fd : i32, __level : i32, __optname : i32, __optval : *mut core::ffi::c_void, __optlen : *mut super::unistd::socklen_t) -> i32);
windows_link::link!("c" "C" fn isfdtype(__fd : i32, __fdtype : i32) -> i32);
windows_link::link!("c" "C" fn listen(__fd : i32, __n : i32) -> i32);
#[cfg(feature = "posix_types")]
windows_link::link!("c" "C" fn recv(__fd : i32, __buf : *mut core::ffi::c_void, __n : u64, __flags : i32) -> super::types::ssize_t);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn recvfrom(__fd : i32, __buf : *mut core::ffi::c_void, __n : u64, __flags : i32, __addr : *mut sockaddr, __addr_len : *mut super::unistd::socklen_t) -> super::types::ssize_t);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn recvmsg(__fd : i32, __message : *mut msghdr, __flags : i32) -> super::types::ssize_t);
#[cfg(feature = "posix_types")]
windows_link::link!("c" "C" fn send(__fd : i32, __buf : *const core::ffi::c_void, __n : u64, __flags : i32) -> super::types::ssize_t);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn sendmsg(__fd : i32, __message : *const msghdr, __flags : i32) -> super::types::ssize_t);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn sendto(__fd : i32, __buf : *const core::ffi::c_void, __n : u64, __flags : i32, __addr : *const sockaddr, __addr_len : super::unistd::socklen_t) -> super::types::ssize_t);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn setsockopt(__fd : i32, __level : i32, __optname : i32, __optval : *const core::ffi::c_void, __optlen : super::unistd::socklen_t) -> i32);
windows_link::link!("c" "C" fn shutdown(__fd : i32, __how : i32) -> i32);
windows_link::link!("c" "C" fn sockatmark(__fd : i32) -> i32);
windows_link::link!("c" "C" fn socket(__domain : i32, __type : i32, __protocol : i32) -> i32);
windows_link::link!("c" "C" fn socketpair(__domain : i32, __type : i32, __protocol : i32, __fds : *mut i32) -> i32);
pub const MSG_BATCH: u32 = 262144;
pub const MSG_CMSG_CLOEXEC: u32 = 1073741824;
pub const MSG_CONFIRM: u32 = 2048;
pub const MSG_CTRUNC: u32 = 8;
pub const MSG_DONTROUTE: u32 = 4;
pub const MSG_DONTWAIT: u32 = 64;
pub const MSG_EOR: u32 = 128;
pub const MSG_ERRQUEUE: u32 = 8192;
pub const MSG_FASTOPEN: u32 = 536870912;
pub const MSG_FIN: u32 = 512;
pub const MSG_MORE: u32 = 32768;
pub const MSG_NOSIGNAL: u32 = 16384;
pub const MSG_OOB: u32 = 1;
pub const MSG_PEEK: u32 = 2;
pub const MSG_PROXY: u32 = 16;
pub const MSG_RST: u32 = 4096;
pub const MSG_SOCK_DEVMEM: u32 = 33554432;
pub const MSG_SYN: u32 = 1024;
pub const MSG_TRUNC: u32 = 32;
pub const MSG_WAITALL: u32 = 256;
pub const MSG_WAITFORONE: u32 = 65536;
pub const MSG_ZEROCOPY: u32 = 67108864;
pub const PF_ALG: i32 = 38;
pub const PF_APPLETALK: i32 = 5;
pub const PF_ASH: i32 = 18;
pub const PF_ATMPVC: i32 = 8;
pub const PF_ATMSVC: i32 = 20;
pub const PF_AX25: i32 = 3;
pub const PF_BLUETOOTH: i32 = 31;
pub const PF_BRIDGE: i32 = 7;
pub const PF_CAIF: i32 = 37;
pub const PF_CAN: i32 = 29;
pub const PF_DECnet: i32 = 12;
pub const PF_ECONET: i32 = 19;
pub const PF_IB: i32 = 27;
pub const PF_IEEE802154: i32 = 36;
pub const PF_INET: i32 = 2;
pub const PF_INET6: i32 = 10;
pub const PF_IPX: i32 = 4;
pub const PF_IRDA: i32 = 23;
pub const PF_ISDN: i32 = 34;
pub const PF_IUCV: i32 = 32;
pub const PF_KCM: i32 = 41;
pub const PF_KEY: i32 = 15;
pub const PF_LLC: i32 = 26;
pub const PF_LOCAL: i32 = 1;
pub const PF_MAX: i32 = 46;
pub const PF_MCTP: i32 = 45;
pub const PF_MPLS: i32 = 28;
pub const PF_NETBEUI: i32 = 13;
pub const PF_NETLINK: i32 = 16;
pub const PF_NETROM: i32 = 6;
pub const PF_NFC: i32 = 39;
pub const PF_PACKET: i32 = 17;
pub const PF_PHONET: i32 = 35;
pub const PF_PPPOX: i32 = 24;
pub const PF_QIPCRTR: i32 = 42;
pub const PF_RDS: i32 = 21;
pub const PF_ROSE: i32 = 11;
pub const PF_RXRPC: i32 = 33;
pub const PF_SECURITY: i32 = 14;
pub const PF_SMC: i32 = 43;
pub const PF_SNA: i32 = 22;
pub const PF_TIPC: i32 = 30;
pub const PF_UNSPEC: i32 = 0;
pub const PF_VSOCK: i32 = 40;
pub const PF_WANPIPE: i32 = 25;
pub const PF_X25: i32 = 9;
pub const PF_XDP: i32 = 44;
pub const SCM_RIGHTS: u32 = 1;
pub const SHUT_RD: u32 = 0;
pub const SHUT_RDWR: u32 = 2;
pub const SHUT_WR: u32 = 1;
pub const SOCK_CLOEXEC: u32 = 524288;
pub const SOCK_DCCP: u32 = 6;
pub const SOCK_DGRAM: u32 = 2;
pub const SOCK_NONBLOCK: u32 = 2048;
pub const SOCK_PACKET: u32 = 10;
pub const SOCK_RAW: u32 = 3;
pub const SOCK_RDM: u32 = 4;
pub const SOCK_SEQPACKET: u32 = 5;
pub const SOCK_STREAM: u32 = 1;
pub const SOL_AAL: i32 = 265;
pub const SOL_ALG: i32 = 279;
pub const SOL_ATM: i32 = 264;
pub const SOL_BLUETOOTH: i32 = 274;
pub const SOL_CAIF: i32 = 278;
pub const SOL_DCCP: i32 = 269;
pub const SOL_DECNET: i32 = 261;
pub const SOL_IRDA: i32 = 266;
pub const SOL_IUCV: i32 = 277;
pub const SOL_KCM: i32 = 281;
pub const SOL_LLC: i32 = 268;
pub const SOL_MCTP: i32 = 285;
pub const SOL_MPTCP: i32 = 284;
pub const SOL_NETBEUI: i32 = 267;
pub const SOL_NETLINK: i32 = 270;
pub const SOL_NFC: i32 = 280;
pub const SOL_PACKET: i32 = 263;
pub const SOL_PNPIPE: i32 = 275;
pub const SOL_PPPOL2TP: i32 = 273;
pub const SOL_RAW: i32 = 255;
pub const SOL_RDS: i32 = 276;
pub const SOL_RXRPC: i32 = 272;
pub const SOL_SMC: i32 = 286;
pub const SOL_TIPC: i32 = 271;
pub const SOL_TLS: i32 = 282;
pub const SOL_VSOCK: i32 = 287;
pub const SOL_X25: i32 = 262;
pub const SOL_XDP: i32 = 283;
pub const SOMAXCONN: i32 = 4096;
pub const _SYS_SOCKET_H: i32 = 1;
pub const __iovec_defined: i32 = 1;
pub type __socket_type = u32;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct cmsghdr {
    pub cmsg_len: u64,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
    pub __cmsg_data: *mut u8,
}
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct iovec {
    pub iov_base: *mut core::ffi::c_void,
    pub iov_len: u64,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct linger {
    pub l_onoff: i32,
    pub l_linger: i32,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
#[derive(Clone, Copy, Default)]
pub struct msghdr {
    pub msg_name: *mut core::ffi::c_void,
    pub msg_namelen: super::unistd::socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: u64,
    pub msg_control: *mut core::ffi::c_void,
    pub msg_controllen: u64,
    pub msg_flags: i32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [i8; 14],
}
impl Default for sockaddr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[derive(Clone, Copy)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __ss_padding: [i8; 118],
    pub __ss_align: u64,
}
impl Default for sockaddr_storage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
