//! End-to-end tests for Socket bindings against real libc.

use bns_posix::PosixFile::Inet;
use bns_posix::PosixFile::Socket;
use bns_posix::PosixFile::Unistd;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

#[test]
fn sock_type_constants() {
    assert_eq!(Socket::SOCK_STREAM, 1);
    assert_eq!(Socket::SOCK_DGRAM, 2);
    assert_eq!(Socket::SOCK_RAW, 3);
    assert_eq!(Socket::SOCK_SEQPACKET, 5);
}

#[test]
fn pf_inet_constants() {
    assert_eq!(Socket::PF_INET, 2);
    assert_eq!(Socket::PF_INET6, 10);
    assert_eq!(Socket::PF_LOCAL, 1);
    assert_eq!(Socket::PF_UNSPEC, 0);
}

#[test]
fn shutdown_constants() {
    assert_eq!(Socket::SHUT_RD, 0);
    assert_eq!(Socket::SHUT_WR, 1);
    assert_eq!(Socket::SHUT_RDWR, 2);
}

#[test]
fn msg_flag_constants() {
    assert_eq!(Socket::MSG_OOB, 1);
    assert_eq!(Socket::MSG_PEEK, 2);
    assert_eq!(Socket::MSG_DONTROUTE, 4);
    assert_eq!(Socket::MSG_DONTWAIT, 64);
    assert_eq!(Socket::MSG_WAITALL, 256);
}

// ---------------------------------------------------------------------------
// Struct layout
// ---------------------------------------------------------------------------

#[test]
fn sockaddr_struct_size() {
    assert_eq!(
        core::mem::size_of::<Socket::sockaddr>(),
        16,
        "struct sockaddr should be 16 bytes"
    );
}

#[test]
fn sockaddr_storage_struct_size() {
    assert_eq!(
        core::mem::size_of::<Socket::sockaddr_storage>(),
        128,
        "struct sockaddr_storage should be 128 bytes"
    );
}

#[test]
fn msghdr_struct_size() {
    assert_eq!(
        core::mem::size_of::<Socket::msghdr>(),
        56,
        "struct msghdr should be 56 bytes on x86_64"
    );
}

#[test]
fn iovec_struct_size() {
    assert_eq!(
        core::mem::size_of::<Socket::iovec>(),
        16,
        "struct iovec should be 16 bytes"
    );
}

#[test]
fn linger_struct_size() {
    assert_eq!(
        core::mem::size_of::<Socket::linger>(),
        8,
        "struct linger should be 8 bytes"
    );
}

// ---------------------------------------------------------------------------
// Syscall smoke tests
// ---------------------------------------------------------------------------

#[test]
fn socket_create_tcp() {
    let fd = unsafe { Socket::socket(Socket::PF_INET, Socket::SOCK_STREAM as i32, 0) };
    assert!(fd >= 0, "socket(PF_INET, SOCK_STREAM, 0) failed: {fd}");
    unsafe { Unistd::close(fd) };
}

#[test]
fn socket_create_udp() {
    let fd = unsafe { Socket::socket(Socket::PF_INET, Socket::SOCK_DGRAM as i32, 0) };
    assert!(fd >= 0, "socket(PF_INET, SOCK_DGRAM, 0) failed: {fd}");
    unsafe { Unistd::close(fd) };
}

#[test]
fn socketpair_unix() {
    let mut fds = [0i32; 2];
    let rc = unsafe {
        Socket::socketpair(
            Socket::PF_LOCAL,
            Socket::SOCK_STREAM as i32,
            0,
            fds.as_mut_ptr() as *const i32,
        )
    };
    assert_eq!(rc, 0, "socketpair failed");
    assert!(fds[0] >= 0);
    assert!(fds[1] >= 0);
    unsafe {
        Unistd::close(fds[0]);
        Unistd::close(fds[1]);
    };
}

#[test]
#[allow(clippy::field_reassign_with_default)]
fn getsockname_after_bind() {
    let fd = unsafe { Socket::socket(Socket::PF_INET, Socket::SOCK_STREAM as i32, 0) };
    assert!(fd >= 0);

    let mut addr = Inet::sockaddr_in::default();
    addr.sin_family = Socket::PF_INET as u16;
    addr.sin_port = 0;
    addr.sin_addr.s_addr = unsafe { Inet::htonl(0x7f000001) }; // 127.0.0.1

    let rc = unsafe {
        Socket::bind(
            fd,
            &addr as *const _ as *const Socket::sockaddr,
            core::mem::size_of::<Inet::sockaddr_in>() as u32,
        )
    };
    assert_eq!(rc, 0, "bind to loopback:0 failed");

    let mut out = Inet::sockaddr_in::default();
    let mut len = core::mem::size_of::<Inet::sockaddr_in>() as u32;
    let rc = unsafe {
        Socket::getsockname(
            fd,
            &mut out as *mut _ as *const Socket::sockaddr,
            &mut len as *mut u32 as *const u32,
        )
    };
    assert_eq!(rc, 0, "getsockname failed");
    assert_eq!(out.sin_family, Socket::PF_INET as u16);
    assert_ne!(out.sin_port, 0, "kernel should assign a port");

    unsafe { Unistd::close(fd) };
}

#[test]
#[allow(clippy::field_reassign_with_default)]
fn listen_on_tcp_socket() {
    let fd = unsafe { Socket::socket(Socket::PF_INET, Socket::SOCK_STREAM as i32, 0) };
    assert!(fd >= 0);

    let mut addr = Inet::sockaddr_in::default();
    addr.sin_family = Socket::PF_INET as u16;
    addr.sin_port = 0;
    addr.sin_addr.s_addr = unsafe { Inet::htonl(0x7f000001) };

    let rc = unsafe {
        Socket::bind(
            fd,
            &addr as *const _ as *const Socket::sockaddr,
            core::mem::size_of::<Inet::sockaddr_in>() as u32,
        )
    };
    assert_eq!(rc, 0, "bind failed");

    let rc = unsafe { Socket::listen(fd, 5) };
    assert_eq!(rc, 0, "listen failed");

    unsafe { Unistd::close(fd) };
}

#[test]
fn setsockopt_reuseaddr() {
    let fd = unsafe { Socket::socket(Socket::PF_INET, Socket::SOCK_STREAM as i32, 0) };
    assert!(fd >= 0);

    let optval: i32 = 1;
    let rc = unsafe {
        Socket::setsockopt(
            fd,
            1, // SOL_SOCKET
            2, // SO_REUSEADDR
            &optval as *const _ as *const core::ffi::c_void,
            core::mem::size_of::<i32>() as u32,
        )
    };
    assert_eq!(rc, 0, "setsockopt SO_REUSEADDR failed");

    unsafe { Unistd::close(fd) };
}

#[test]
fn send_recv_socketpair() {
    let mut fds = [0i32; 2];
    let rc = unsafe {
        Socket::socketpair(
            Socket::PF_LOCAL,
            Socket::SOCK_STREAM as i32,
            0,
            fds.as_mut_ptr() as *const i32,
        )
    };
    assert_eq!(rc, 0);

    let msg = b"hello socket";
    let sent = unsafe {
        Socket::send(
            fds[0],
            msg.as_ptr() as *const core::ffi::c_void,
            msg.len() as u64,
            0,
        )
    };
    assert_eq!(sent, msg.len() as i64);

    let mut buf = [0u8; 64];
    let recvd = unsafe {
        Socket::recv(
            fds[1],
            buf.as_mut_ptr() as *const core::ffi::c_void,
            buf.len() as u64,
            0,
        )
    };
    assert_eq!(recvd, msg.len() as i64);
    assert_eq!(&buf[..msg.len()], msg);

    unsafe {
        Unistd::close(fds[0]);
        Unistd::close(fds[1]);
    };
}
