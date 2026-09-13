#[cfg(feature = "sockaddr")]
windows_link::link!("c" "C" fn bindresvport(__sockfd : i32, __sock_in : *mut sockaddr_in) -> i32);
windows_link::link!("c" "C" fn htonl(__hostlong : u32) -> u32);
windows_link::link!("c" "C" fn htons(__hostshort : u16) -> u16);
windows_link::link!("c" "C" fn ntohl(__netlong : u32) -> u32);
windows_link::link!("c" "C" fn ntohs(__netshort : u16) -> u16);
pub const INADDR_ALLHOSTS_GROUP: in_addr_t = 3758096385;
pub const INADDR_ALLRTRS_GROUP: in_addr_t = 3758096386;
pub const INADDR_ALLSNOOPERS_GROUP: in_addr_t = 3758096490;
pub const INADDR_ANY: in_addr_t = 0;
pub const INADDR_BROADCAST: in_addr_t = 4294967295;
pub const INADDR_DUMMY: in_addr_t = 3221225480;
pub const INADDR_LOOPBACK: in_addr_t = 2130706433;
pub const INADDR_MAX_LOCAL_GROUP: in_addr_t = 3758096639;
pub const INADDR_NONE: in_addr_t = 4294967295;
pub const INADDR_UNSPEC_GROUP: in_addr_t = 3758096384;
pub const INET6_ADDRSTRLEN: i32 = 46;
pub const INET_ADDRSTRLEN: i32 = 16;
pub const IN_CLASSA_HOST: u32 = 16777215;
pub const IN_CLASSA_MAX: i32 = 128;
pub const IN_CLASSA_NET: u32 = 4278190080;
pub const IN_CLASSA_NSHIFT: i32 = 24;
pub const IN_CLASSB_HOST: u32 = 65535;
pub const IN_CLASSB_MAX: i32 = 65536;
pub const IN_CLASSB_NET: u32 = 4294901760;
pub const IN_CLASSB_NSHIFT: i32 = 16;
pub const IN_CLASSC_HOST: u32 = 255;
pub const IN_CLASSC_NET: u32 = 4294967040;
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
pub const IPV6_2292DSTOPTS: i32 = 4;
pub const IPV6_2292HOPLIMIT: i32 = 8;
pub const IPV6_2292HOPOPTS: i32 = 3;
pub const IPV6_2292PKTINFO: i32 = 2;
pub const IPV6_2292PKTOPTIONS: i32 = 6;
pub const IPV6_2292RTHDR: i32 = 5;
pub const IPV6_ADDRFORM: i32 = 1;
pub const IPV6_ADDR_PREFERENCES: i32 = 72;
pub const IPV6_ADD_MEMBERSHIP: i32 = 20;
pub const IPV6_AUTHHDR: i32 = 10;
pub const IPV6_AUTOFLOWLABEL: i32 = 70;
pub const IPV6_CHECKSUM: i32 = 7;
pub const IPV6_DONTFRAG: i32 = 62;
pub const IPV6_DROP_MEMBERSHIP: i32 = 21;
pub const IPV6_DSTOPTS: i32 = 59;
pub const IPV6_FREEBIND: i32 = 78;
pub const IPV6_HDRINCL: i32 = 36;
pub const IPV6_HOPLIMIT: i32 = 52;
pub const IPV6_HOPOPTS: i32 = 54;
pub const IPV6_IPSEC_POLICY: i32 = 34;
pub const IPV6_JOIN_ANYCAST: i32 = 27;
pub const IPV6_JOIN_GROUP: i32 = 20;
pub const IPV6_LEAVE_ANYCAST: i32 = 28;
pub const IPV6_LEAVE_GROUP: i32 = 21;
pub const IPV6_MINHOPCOUNT: i32 = 73;
pub const IPV6_MTU: i32 = 24;
pub const IPV6_MTU_DISCOVER: i32 = 23;
pub const IPV6_MULTICAST_ALL: i32 = 29;
pub const IPV6_MULTICAST_HOPS: i32 = 18;
pub const IPV6_MULTICAST_IF: i32 = 17;
pub const IPV6_MULTICAST_LOOP: i32 = 19;
pub const IPV6_NEXTHOP: i32 = 9;
pub const IPV6_ORIGDSTADDR: i32 = 74;
pub const IPV6_PATHMTU: i32 = 61;
pub const IPV6_PKTINFO: i32 = 50;
pub const IPV6_PMTUDISC_DO: i32 = 2;
pub const IPV6_PMTUDISC_DONT: i32 = 0;
pub const IPV6_PMTUDISC_INTERFACE: i32 = 4;
pub const IPV6_PMTUDISC_OMIT: i32 = 5;
pub const IPV6_PMTUDISC_PROBE: i32 = 3;
pub const IPV6_PMTUDISC_WANT: i32 = 1;
pub const IPV6_RECVDSTOPTS: i32 = 58;
pub const IPV6_RECVERR: i32 = 25;
pub const IPV6_RECVERR_RFC4884: i32 = 31;
pub const IPV6_RECVFRAGSIZE: i32 = 77;
pub const IPV6_RECVHOPLIMIT: i32 = 51;
pub const IPV6_RECVHOPOPTS: i32 = 53;
pub const IPV6_RECVORIGDSTADDR: i32 = 74;
pub const IPV6_RECVPATHMTU: i32 = 60;
pub const IPV6_RECVPKTINFO: i32 = 49;
pub const IPV6_RECVRTHDR: i32 = 56;
pub const IPV6_RECVTCLASS: i32 = 66;
pub const IPV6_ROUTER_ALERT: i32 = 22;
pub const IPV6_ROUTER_ALERT_ISOLATE: i32 = 30;
pub const IPV6_RTHDR: i32 = 57;
pub const IPV6_RTHDRDSTOPTS: i32 = 55;
pub const IPV6_RTHDR_LOOSE: i32 = 0;
pub const IPV6_RTHDR_STRICT: i32 = 1;
pub const IPV6_RTHDR_TYPE_0: i32 = 0;
pub const IPV6_RXDSTOPTS: i32 = 59;
pub const IPV6_RXHOPOPTS: i32 = 54;
pub const IPV6_TCLASS: i32 = 67;
pub const IPV6_TRANSPARENT: i32 = 75;
pub const IPV6_UNICAST_HOPS: i32 = 16;
pub const IPV6_UNICAST_IF: i32 = 76;
pub const IPV6_V6ONLY: i32 = 26;
pub const IPV6_XFRM_POLICY: i32 = 35;
pub const IP_ADD_MEMBERSHIP: i32 = 35;
pub const IP_ADD_SOURCE_MEMBERSHIP: i32 = 39;
pub const IP_BIND_ADDRESS_NO_PORT: i32 = 24;
pub const IP_BLOCK_SOURCE: i32 = 38;
pub const IP_CHECKSUM: i32 = 23;
pub const IP_DEFAULT_MULTICAST_LOOP: i32 = 1;
pub const IP_DEFAULT_MULTICAST_TTL: i32 = 1;
pub const IP_DROP_MEMBERSHIP: i32 = 36;
pub const IP_DROP_SOURCE_MEMBERSHIP: i32 = 40;
pub const IP_FREEBIND: i32 = 15;
pub const IP_HDRINCL: i32 = 3;
pub const IP_IPSEC_POLICY: i32 = 16;
pub const IP_LOCAL_PORT_RANGE: i32 = 51;
pub const IP_MAX_MEMBERSHIPS: i32 = 20;
pub const IP_MINTTL: i32 = 21;
pub const IP_MSFILTER: i32 = 41;
pub const IP_MTU: i32 = 14;
pub const IP_MTU_DISCOVER: i32 = 10;
pub const IP_MULTICAST_ALL: i32 = 49;
pub const IP_MULTICAST_IF: i32 = 32;
pub const IP_MULTICAST_LOOP: i32 = 34;
pub const IP_MULTICAST_TTL: i32 = 33;
pub const IP_NODEFRAG: i32 = 22;
pub const IP_OPTIONS: i32 = 4;
pub const IP_ORIGDSTADDR: i32 = 20;
pub const IP_PASSSEC: i32 = 18;
pub const IP_PKTINFO: i32 = 8;
pub const IP_PKTOPTIONS: i32 = 9;
pub const IP_PMTUDISC: i32 = 10;
pub const IP_PMTUDISC_DO: i32 = 2;
pub const IP_PMTUDISC_DONT: i32 = 0;
pub const IP_PMTUDISC_INTERFACE: i32 = 4;
pub const IP_PMTUDISC_OMIT: i32 = 5;
pub const IP_PMTUDISC_PROBE: i32 = 3;
pub const IP_PMTUDISC_WANT: i32 = 1;
pub const IP_PROTOCOL: i32 = 52;
pub const IP_RECVERR: i32 = 11;
pub const IP_RECVERR_RFC4884: i32 = 26;
pub const IP_RECVFRAGSIZE: i32 = 25;
pub const IP_RECVOPTS: i32 = 6;
pub const IP_RECVORIGDSTADDR: i32 = 20;
pub const IP_RECVRETOPTS: i32 = 7;
pub const IP_RECVTOS: i32 = 13;
pub const IP_RECVTTL: i32 = 12;
pub const IP_RETOPTS: i32 = 7;
pub const IP_ROUTER_ALERT: i32 = 5;
pub const IP_TOS: i32 = 1;
pub const IP_TRANSPARENT: i32 = 19;
pub const IP_TTL: i32 = 2;
pub const IP_UNBLOCK_SOURCE: i32 = 37;
pub const IP_UNICAST_IF: i32 = 50;
pub const IP_XFRM_POLICY: i32 = 17;
pub const MCAST_BLOCK_SOURCE: i32 = 43;
pub const MCAST_EXCLUDE: i32 = 0;
pub const MCAST_INCLUDE: i32 = 1;
pub const MCAST_JOIN_GROUP: i32 = 42;
pub const MCAST_JOIN_SOURCE_GROUP: i32 = 46;
pub const MCAST_LEAVE_GROUP: i32 = 45;
pub const MCAST_LEAVE_SOURCE_GROUP: i32 = 47;
pub const MCAST_MSFILTER: i32 = 48;
pub const MCAST_UNBLOCK_SOURCE: i32 = 44;
pub const SOL_ICMPV6: i32 = 58;
pub const SOL_IP: i32 = 0;
pub const SOL_IPV6: i32 = 41;
#[repr(C)]
#[cfg(all(feature = "sockaddr", feature = "socket"))]
#[derive(Clone, Copy)]
pub struct group_filter {
    pub gf_interface: u32,
    pub gf_group: super::socket::sockaddr_storage,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_slist: [super::socket::sockaddr_storage; 1],
}
#[cfg(all(feature = "sockaddr", feature = "socket"))]
impl Default for group_filter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "sockaddr", feature = "socket"))]
#[derive(Clone, Copy, Default)]
pub struct group_req {
    pub gr_interface: u32,
    pub gr_group: super::socket::sockaddr_storage,
}
#[repr(C)]
#[cfg(all(feature = "sockaddr", feature = "socket"))]
#[derive(Clone, Copy, Default)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: super::socket::sockaddr_storage,
    pub gsr_source: super::socket::sockaddr_storage,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in6_addr {
    pub __in6_u: in6_addr_0,
}
impl Default for in6_addr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union in6_addr_0 {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}
impl Default for in6_addr_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct in_pktinfo {
    pub ipi_ifindex: i32,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}
pub type in_port_t = u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: i32,
}
#[repr(C)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ip_opts {
    pub ip_dst: in_addr,
    pub ip_opts: [i8; 40],
}
impl Default for ip_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[repr(C)]
#[cfg(feature = "sockaddr")]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: super::sockaddr::sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}
#[cfg(feature = "sockaddr")]
impl Default for sockaddr_in {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "sockaddr")]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: super::sockaddr::sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
#[cfg(feature = "sockaddr")]
impl Default for sockaddr_in6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
