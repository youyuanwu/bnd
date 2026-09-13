windows_link::link!("c" "C" fn bindresvport(__sockfd : i32, __sock_in : *mut sockaddr_in) -> i32);
windows_link::link!("c" "C" fn bindresvport6(__sockfd : i32, __sock_in : *mut sockaddr_in6) -> i32);
windows_link::link!("c" "C" fn htonl(__hostlong : u32) -> u32);
windows_link::link!("c" "C" fn htons(__hostshort : u16) -> u16);
windows_link::link!("c" "C" fn inet_addr(__cp : *const i8) -> in_addr_t);
windows_link::link!("c" "C" fn inet_aton(__cp : *const i8, __inp : *mut in_addr) -> i32);
windows_link::link!("c" "C" fn inet_lnaof(__in : in_addr) -> in_addr_t);
windows_link::link!("c" "C" fn inet_makeaddr(__net : in_addr_t, __host : in_addr_t) -> in_addr);
windows_link::link!("c" "C" fn inet_net_ntop(__af : i32, __cp : *const core::ffi::c_void, __bits : i32, __buf : *mut i8, __len : u64) -> *mut i8);
windows_link::link!("c" "C" fn inet_net_pton(__af : i32, __cp : *const i8, __buf : *mut core::ffi::c_void, __len : u64) -> i32);
windows_link::link!("c" "C" fn inet_neta(__net : in_addr_t, __buf : *mut i8, __len : u64) -> *mut i8);
windows_link::link!("c" "C" fn inet_netof(__in : in_addr) -> in_addr_t);
windows_link::link!("c" "C" fn inet_network(__cp : *const i8) -> in_addr_t);
windows_link::link!("c" "C" fn inet_nsap_addr(__cp : *const i8, __buf : *mut u8, __len : i32) -> u32);
windows_link::link!("c" "C" fn inet_nsap_ntoa(__len : i32, __cp : *const u8, __buf : *mut i8) -> *mut i8);
windows_link::link!("c" "C" fn inet_ntoa(__in : in_addr) -> *mut i8);
#[cfg(all(feature = "posix_types", feature = "posix_unistd"))]
windows_link::link!("c" "C" fn inet_ntop(__af : i32, __cp : *const core::ffi::c_void, __buf : *mut i8, __len : super::unistd::socklen_t) -> *mut i8);
windows_link::link!("c" "C" fn inet_pton(__af : i32, __cp : *const i8, __buf : *mut core::ffi::c_void) -> i32);
windows_link::link!("c" "C" fn ntohl(__netlong : u32) -> u32);
windows_link::link!("c" "C" fn ntohs(__netshort : u16) -> u16);
pub const INET6_ADDRSTRLEN: i32 = 46;
pub const INET_ADDRSTRLEN: i32 = 16;
pub const IN_CLASSA_MAX: i32 = 128;
pub const IN_CLASSA_NET: i32 = -16777216;
pub const IN_CLASSA_NSHIFT: i32 = 24;
pub const IN_CLASSB_MAX: i32 = 65536;
pub const IN_CLASSB_NET: i32 = -65536;
pub const IN_CLASSB_NSHIFT: i32 = 16;
pub const IN_CLASSC_NET: i32 = -256;
pub const IN_CLASSC_NSHIFT: i32 = 8;
pub const IN_LOOPBACKNET: i32 = 127;
pub const IPPORT_BIFFUDP: u32 = 512;
pub const IPPORT_CMDSERVER: u32 = 514;
pub const IPPORT_DAYTIME: u32 = 13;
pub const IPPORT_DISCARD: u32 = 9;
pub const IPPORT_ECHO: u32 = 7;
pub const IPPORT_EFSSERVER: u32 = 520;
pub const IPPORT_EXECSERVER: u32 = 512;
pub const IPPORT_FINGER: u32 = 79;
pub const IPPORT_FTP: u32 = 21;
pub const IPPORT_LOGINSERVER: u32 = 513;
pub const IPPORT_MTP: u32 = 57;
pub const IPPORT_NAMESERVER: u32 = 42;
pub const IPPORT_NETSTAT: u32 = 15;
pub const IPPORT_RESERVED: u32 = 1024;
pub const IPPORT_RJE: u32 = 77;
pub const IPPORT_ROUTESERVER: u32 = 520;
pub const IPPORT_SMTP: u32 = 25;
pub const IPPORT_SUPDUP: u32 = 95;
pub const IPPORT_SYSTAT: u32 = 11;
pub const IPPORT_TELNET: u32 = 23;
pub const IPPORT_TFTP: u32 = 69;
pub const IPPORT_TIMESERVER: u32 = 37;
pub const IPPORT_TTYLINK: u32 = 87;
pub const IPPORT_USERRESERVED: u32 = 5000;
pub const IPPORT_WHOIS: u32 = 43;
pub const IPPORT_WHOSERVER: u32 = 513;
pub const IPPROTO_AH: u32 = 51;
pub const IPPROTO_BEETPH: u32 = 94;
pub const IPPROTO_COMP: u32 = 108;
pub const IPPROTO_DCCP: u32 = 33;
pub const IPPROTO_DSTOPTS: u32 = 60;
pub const IPPROTO_EGP: u32 = 8;
pub const IPPROTO_ENCAP: u32 = 98;
pub const IPPROTO_ESP: u32 = 50;
pub const IPPROTO_ETHERNET: u32 = 143;
pub const IPPROTO_FRAGMENT: u32 = 44;
pub const IPPROTO_GRE: u32 = 47;
pub const IPPROTO_HOPOPTS: u32 = 0;
pub const IPPROTO_ICMP: u32 = 1;
pub const IPPROTO_ICMPV6: u32 = 58;
pub const IPPROTO_IDP: u32 = 22;
pub const IPPROTO_IGMP: u32 = 2;
pub const IPPROTO_IP: u32 = 0;
pub const IPPROTO_IPIP: u32 = 4;
pub const IPPROTO_IPV6: u32 = 41;
pub const IPPROTO_L2TP: u32 = 115;
pub const IPPROTO_MAX: u32 = 263;
pub const IPPROTO_MH: u32 = 135;
pub const IPPROTO_MPLS: u32 = 137;
pub const IPPROTO_MPTCP: u32 = 262;
pub const IPPROTO_MTP: u32 = 92;
pub const IPPROTO_NONE: u32 = 59;
pub const IPPROTO_PIM: u32 = 103;
pub const IPPROTO_PUP: u32 = 12;
pub const IPPROTO_RAW: u32 = 255;
pub const IPPROTO_ROUTING: u32 = 43;
pub const IPPROTO_RSVP: u32 = 46;
pub const IPPROTO_SCTP: u32 = 132;
pub const IPPROTO_SMC: u32 = 256;
pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_TP: u32 = 29;
pub const IPPROTO_UDP: u32 = 17;
pub const IPPROTO_UDPLITE: u32 = 136;
pub const _ARPA_INET_H: i32 = 1;
pub const _NETINET_IN_H: i32 = 1;
#[repr(C, packed(8))]
#[cfg(feature = "posix_socket")]
#[derive(Clone, Copy)]
pub struct group_filter {
    pub gf_interface: u32,
    pub gf_group: super::socket::sockaddr_storage,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_slist: [super::socket::sockaddr_storage; 1],
}
#[cfg(feature = "posix_socket")]
impl Default for group_filter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "posix_socket")]
#[derive(Clone, Copy, Default)]
pub struct group_req {
    pub gr_interface: u32,
    pub gr_group: super::socket::sockaddr_storage,
}
#[repr(C, packed(8))]
#[cfg(feature = "posix_socket")]
#[derive(Clone, Copy, Default)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: super::socket::sockaddr_storage,
    pub gsr_source: super::socket::sockaddr_storage,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct in6_addr {
    pub __in6_u: in6_addr___in6_u,
}
impl Default for in6_addr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union in6_addr___in6_u {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}
impl Default for in6_addr___in6_u {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
pub type in_port_t = u16;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: i32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct ip_msfilter {
    pub imsf_multiaddr: in_addr,
    pub imsf_interface: in_addr,
    pub imsf_fmode: u32,
    pub imsf_numsrc: u32,
    pub imsf_slist: [in_addr; 1],
}
impl Default for ip_msfilter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: u32,
}
impl Default for ipv6_mreq {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}
impl Default for sockaddr_in {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
impl Default for sockaddr_in6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
