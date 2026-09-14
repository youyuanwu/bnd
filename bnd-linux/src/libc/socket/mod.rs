#[cfg(all(feature = "struct_iovec", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn __cmsg_nxthdr(__mhdr : *mut msghdr, __cmsg : *mut cmsghdr) -> *mut cmsghdr);
#[cfg(all(feature = "sockaddr", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn accept(__fd : i32, __addr : *mut sockaddr, __addr_len : *mut super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "sockaddr", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn bind(__fd : i32, __addr : *const sockaddr, __len : super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "sockaddr", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn connect(__fd : i32, __addr : *const sockaddr, __len : super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "sockaddr", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn getpeername(__fd : i32, __addr : *mut sockaddr, __len : *mut super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "sockaddr", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn getsockname(__fd : i32, __addr : *mut sockaddr, __len : *mut super::unistd::socklen_t) -> i32);
#[cfg(all(feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn getsockopt(__fd : i32, __level : i32, __optname : i32, __optval : *mut core::ffi::c_void, __optlen : *mut super::unistd::socklen_t) -> i32);
windows_link::link!("c" "C" fn isfdtype(__fd : i32, __fdtype : i32) -> i32);
windows_link::link!("c" "C" fn listen(__fd : i32, __n : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn recv(__fd : i32, __buf : *mut core::ffi::c_void, __n : usize, __flags : i32) -> super::types::ssize_t);
#[cfg(all(feature = "sockaddr", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn recvfrom(__fd : i32, __buf : *mut core::ffi::c_void, __n : usize, __flags : i32, __addr : *mut sockaddr, __addr_len : *mut super::unistd::socklen_t) -> super::types::ssize_t);
#[cfg(all(feature = "struct_iovec", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn recvmsg(__fd : i32, __message : *mut msghdr, __flags : i32) -> super::types::ssize_t);
#[cfg(feature = "types")]
windows_link::link!("c" "C" fn send(__fd : i32, __buf : *const core::ffi::c_void, __n : usize, __flags : i32) -> super::types::ssize_t);
#[cfg(all(feature = "struct_iovec", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn sendmsg(__fd : i32, __message : *const msghdr, __flags : i32) -> super::types::ssize_t);
#[cfg(all(feature = "sockaddr", feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn sendto(__fd : i32, __buf : *const core::ffi::c_void, __n : usize, __flags : i32, __addr : *const sockaddr, __addr_len : super::unistd::socklen_t) -> super::types::ssize_t);
#[cfg(all(feature = "types", feature = "unistd"))]
windows_link::link!("c" "C" fn setsockopt(__fd : i32, __level : i32, __optname : i32, __optval : *const core::ffi::c_void, __optlen : super::unistd::socklen_t) -> i32);
windows_link::link!("c" "C" fn shutdown(__fd : i32, __how : i32) -> i32);
windows_link::link!("c" "C" fn sockatmark(__fd : i32) -> i32);
windows_link::link!("c" "C" fn socket(__domain : i32, __type : i32, __protocol : i32) -> i32);
windows_link::link!("c" "C" fn socketpair(__domain : i32, __type : i32, __protocol : i32, __fds : *mut i32) -> i32);
pub const AF_ALG: i32 = 38;
pub const AF_APPLETALK: i32 = 5;
pub const AF_ASH: i32 = 18;
pub const AF_ATMPVC: i32 = 8;
pub const AF_ATMSVC: i32 = 20;
pub const AF_AX25: i32 = 3;
pub const AF_BLUETOOTH: i32 = 31;
pub const AF_BRIDGE: i32 = 7;
pub const AF_CAIF: i32 = 37;
pub const AF_CAN: i32 = 29;
pub const AF_DECnet: i32 = 12;
pub const AF_ECONET: i32 = 19;
pub const AF_FILE: i32 = 1;
pub const AF_IB: i32 = 27;
pub const AF_IEEE802154: i32 = 36;
pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 10;
pub const AF_IPX: i32 = 4;
pub const AF_IRDA: i32 = 23;
pub const AF_ISDN: i32 = 34;
pub const AF_IUCV: i32 = 32;
pub const AF_KCM: i32 = 41;
pub const AF_KEY: i32 = 15;
pub const AF_LLC: i32 = 26;
pub const AF_LOCAL: i32 = 1;
pub const AF_MAX: i32 = 46;
pub const AF_MCTP: i32 = 45;
pub const AF_MPLS: i32 = 28;
pub const AF_NETBEUI: i32 = 13;
pub const AF_NETLINK: i32 = 16;
pub const AF_NETROM: i32 = 6;
pub const AF_NFC: i32 = 39;
pub const AF_PACKET: i32 = 17;
pub const AF_PHONET: i32 = 35;
pub const AF_PPPOX: i32 = 24;
pub const AF_QIPCRTR: i32 = 42;
pub const AF_RDS: i32 = 21;
pub const AF_ROSE: i32 = 11;
pub const AF_ROUTE: i32 = 16;
pub const AF_RXRPC: i32 = 33;
pub const AF_SECURITY: i32 = 14;
pub const AF_SMC: i32 = 43;
pub const AF_SNA: i32 = 22;
pub const AF_TIPC: i32 = 30;
pub const AF_UNIX: i32 = 1;
pub const AF_UNSPEC: i32 = 0;
pub const AF_VSOCK: i32 = 40;
pub const AF_WANPIPE: i32 = 25;
pub const AF_X25: i32 = 9;
pub const AF_XDP: i32 = 44;
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
pub const PF_FILE: i32 = 1;
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
pub const PF_ROUTE: i32 = 16;
pub const PF_RXRPC: i32 = 33;
pub const PF_SECURITY: i32 = 14;
pub const PF_SMC: i32 = 43;
pub const PF_SNA: i32 = 22;
pub const PF_TIPC: i32 = 30;
pub const PF_UNIX: i32 = 1;
pub const PF_UNSPEC: i32 = 0;
pub const PF_VSOCK: i32 = 40;
pub const PF_WANPIPE: i32 = 25;
pub const PF_X25: i32 = 9;
pub const PF_XDP: i32 = 44;
pub const SCM_DEVMEM_DMABUF: i32 = 79;
pub const SCM_DEVMEM_LINEAR: i32 = 78;
pub const SCM_INQ: i32 = 84;
pub const SCM_RIGHTS: u32 = 1;
pub const SCM_TIMESTAMP: i32 = 29;
pub const SCM_TIMESTAMPING: i32 = 37;
pub const SCM_TIMESTAMPING_OPT_STATS: i32 = 54;
pub const SCM_TIMESTAMPING_PKTINFO: i32 = 58;
pub const SCM_TIMESTAMPNS: i32 = 35;
pub const SCM_TS_OPT_ID: i32 = 81;
pub const SCM_TXTIME: i32 = 61;
pub const SCM_WIFI_STATUS: i32 = 41;
pub const SHUT_RD: u32 = 0;
pub const SHUT_RDWR: u32 = 2;
pub const SHUT_WR: u32 = 1;
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
pub const SOL_SOCKET: i32 = 1;
pub const SOL_TIPC: i32 = 271;
pub const SOL_TLS: i32 = 282;
pub const SOL_VSOCK: i32 = 287;
pub const SOL_X25: i32 = 262;
pub const SOL_XDP: i32 = 283;
pub const SOMAXCONN: i32 = 4096;
pub const SO_ACCEPTCONN: i32 = 30;
pub const SO_ATTACH_BPF: i32 = 50;
pub const SO_ATTACH_FILTER: i32 = 26;
pub const SO_ATTACH_REUSEPORT_CBPF: i32 = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: i32 = 52;
pub const SO_BINDTODEVICE: i32 = 25;
pub const SO_BINDTOIFINDEX: i32 = 62;
pub const SO_BPF_EXTENSIONS: i32 = 48;
pub const SO_BROADCAST: i32 = 6;
pub const SO_BSDCOMPAT: i32 = 14;
pub const SO_BUF_LOCK: i32 = 72;
pub const SO_BUSY_POLL: i32 = 46;
pub const SO_BUSY_POLL_BUDGET: i32 = 70;
pub const SO_CNX_ADVICE: i32 = 53;
pub const SO_COOKIE: i32 = 57;
pub const SO_DEBUG: i32 = 1;
pub const SO_DETACH_BPF: i32 = 27;
pub const SO_DETACH_FILTER: i32 = 27;
pub const SO_DETACH_REUSEPORT_BPF: i32 = 68;
pub const SO_DEVMEM_DMABUF: i32 = 79;
pub const SO_DEVMEM_DONTNEED: i32 = 80;
pub const SO_DEVMEM_LINEAR: i32 = 78;
pub const SO_DOMAIN: i32 = 39;
pub const SO_DONTROUTE: i32 = 5;
pub const SO_ERROR: i32 = 4;
pub const SO_GET_FILTER: i32 = 26;
pub const SO_INCOMING_CPU: i32 = 49;
pub const SO_INCOMING_NAPI_ID: i32 = 56;
pub const SO_INQ: i32 = 84;
pub const SO_KEEPALIVE: i32 = 9;
pub const SO_LINGER: i32 = 13;
pub const SO_LOCK_FILTER: i32 = 44;
pub const SO_MARK: i32 = 36;
pub const SO_MAX_PACING_RATE: i32 = 47;
pub const SO_MEMINFO: i32 = 55;
pub const SO_NETNS_COOKIE: i32 = 71;
pub const SO_NOFCS: i32 = 43;
pub const SO_NO_CHECK: i32 = 11;
pub const SO_OOBINLINE: i32 = 10;
pub const SO_PASSCRED: i32 = 16;
pub const SO_PASSPIDFD: i32 = 76;
pub const SO_PASSRIGHTS: i32 = 83;
pub const SO_PASSSEC: i32 = 34;
pub const SO_PEEK_OFF: i32 = 42;
pub const SO_PEERCRED: i32 = 17;
pub const SO_PEERGROUPS: i32 = 59;
pub const SO_PEERNAME: i32 = 28;
pub const SO_PEERPIDFD: i32 = 77;
pub const SO_PEERSEC: i32 = 31;
pub const SO_PREFER_BUSY_POLL: i32 = 69;
pub const SO_PRIORITY: i32 = 12;
pub const SO_PROTOCOL: i32 = 38;
pub const SO_RCVBUF: i32 = 8;
pub const SO_RCVBUFFORCE: i32 = 33;
pub const SO_RCVLOWAT: i32 = 18;
pub const SO_RCVMARK: i32 = 75;
pub const SO_RCVPRIORITY: i32 = 82;
pub const SO_RCVTIMEO: i32 = 20;
pub const SO_RCVTIMEO_NEW: i32 = 66;
pub const SO_RCVTIMEO_OLD: i32 = 20;
pub const SO_RESERVE_MEM: i32 = 73;
pub const SO_REUSEADDR: i32 = 2;
pub const SO_REUSEPORT: i32 = 15;
pub const SO_RXQ_OVFL: i32 = 40;
pub const SO_SECURITY_AUTHENTICATION: i32 = 22;
pub const SO_SECURITY_ENCRYPTION_NETWORK: i32 = 24;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: i32 = 23;
pub const SO_SELECT_ERR_QUEUE: i32 = 45;
pub const SO_SNDBUF: i32 = 7;
pub const SO_SNDBUFFORCE: i32 = 32;
pub const SO_SNDLOWAT: i32 = 19;
pub const SO_SNDTIMEO: i32 = 21;
pub const SO_SNDTIMEO_NEW: i32 = 67;
pub const SO_SNDTIMEO_OLD: i32 = 21;
pub const SO_TIMESTAMP: i32 = 29;
pub const SO_TIMESTAMPING: i32 = 37;
pub const SO_TIMESTAMPING_NEW: i32 = 65;
pub const SO_TIMESTAMPING_OLD: i32 = 37;
pub const SO_TIMESTAMPNS: i32 = 35;
pub const SO_TIMESTAMPNS_NEW: i32 = 64;
pub const SO_TIMESTAMPNS_OLD: i32 = 35;
pub const SO_TIMESTAMP_NEW: i32 = 63;
pub const SO_TIMESTAMP_OLD: i32 = 29;
pub const SO_TXREHASH: i32 = 74;
pub const SO_TXTIME: i32 = 61;
pub const SO_TYPE: i32 = 3;
pub const SO_WIFI_STATUS: i32 = 41;
pub const SO_ZEROCOPY: i32 = 60;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
    pub __cmsg_data: [u8; 0],
}
impl Default for cmsghdr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct linger {
    pub l_onoff: i32,
    pub l_linger: i32,
}
#[repr(C)]
#[cfg(all(feature = "struct_iovec", feature = "types", feature = "unistd"))]
#[derive(Clone, Copy, Default)]
pub struct msghdr {
    pub msg_name: *mut core::ffi::c_void,
    pub msg_namelen: super::unistd::socklen_t,
    pub msg_iov: *mut super::struct_iovec::iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut core::ffi::c_void,
    pub msg_controllen: usize,
    pub msg_flags: i32,
}
#[repr(C)]
#[cfg(feature = "sockaddr")]
#[derive(Clone, Copy)]
pub struct sockaddr {
    pub sa_family: super::sockaddr::sa_family_t,
    pub sa_data: [i8; 14],
}
#[cfg(feature = "sockaddr")]
impl Default for sockaddr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "sockaddr")]
#[derive(Clone, Copy)]
pub struct sockaddr_storage {
    pub ss_family: super::sockaddr::sa_family_t,
    pub __ss_padding: [i8; 118],
    pub __ss_align: u64,
}
#[cfg(feature = "sockaddr")]
impl Default for sockaddr_storage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
