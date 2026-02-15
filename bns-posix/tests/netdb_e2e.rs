//! End-to-end tests for Netdb bindings against real libc.

use bns_posix::PosixFile::Netdb;
use bns_posix::PosixFile::Socket;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

#[test]
fn ai_flag_constants() {
    assert_eq!(Netdb::AI_PASSIVE, 1);
    assert_eq!(Netdb::AI_CANONNAME, 2);
    assert_eq!(Netdb::AI_NUMERICHOST, 4);
    assert_eq!(Netdb::AI_V4MAPPED, 8);
    assert_eq!(Netdb::AI_ADDRCONFIG, 32);
}

#[test]
fn eai_error_constants() {
    assert_eq!(Netdb::EAI_NONAME, -2);
    assert_eq!(Netdb::EAI_AGAIN, -3);
    assert_eq!(Netdb::EAI_FAIL, -4);
    assert_eq!(Netdb::EAI_FAMILY, -6);
    assert_eq!(Netdb::EAI_MEMORY, -10);
}

// ---------------------------------------------------------------------------
// Struct layout
// ---------------------------------------------------------------------------

#[test]
fn addrinfo_struct_size() {
    assert_eq!(
        core::mem::size_of::<Netdb::addrinfo>(),
        48,
        "struct addrinfo should be 48 bytes on x86_64"
    );
}

#[test]
fn hostent_struct_size() {
    assert_eq!(
        core::mem::size_of::<Netdb::hostent>(),
        32,
        "struct hostent should be 32 bytes on x86_64"
    );
}

#[test]
fn servent_struct_size() {
    assert_eq!(
        core::mem::size_of::<Netdb::servent>(),
        32,
        "struct servent should be 32 bytes on x86_64"
    );
}

#[test]
fn protoent_struct_size() {
    assert_eq!(
        core::mem::size_of::<Netdb::protoent>(),
        24,
        "struct protoent should be 24 bytes on x86_64"
    );
}

#[test]
fn netent_struct_size() {
    assert_eq!(
        core::mem::size_of::<Netdb::netent>(),
        24,
        "struct netent should be 24 bytes on x86_64"
    );
}

// ---------------------------------------------------------------------------
// Lookup functions
// ---------------------------------------------------------------------------

#[test]
fn getprotobyname_tcp() {
    let name = c"tcp";
    let entry = unsafe { Netdb::getprotobyname(name.as_ptr()) };
    assert!(!entry.is_null(), "getprotobyname(\"tcp\") should succeed");
    let proto = unsafe { (*entry).p_proto };
    assert_eq!(proto, 6, "TCP protocol number should be 6");
}

#[test]
fn getprotobyname_udp() {
    let name = c"udp";
    let entry = unsafe { Netdb::getprotobyname(name.as_ptr()) };
    assert!(!entry.is_null(), "getprotobyname(\"udp\") should succeed");
    let proto = unsafe { (*entry).p_proto };
    assert_eq!(proto, 17, "UDP protocol number should be 17");
}

#[test]
#[allow(clippy::field_reassign_with_default)]
fn getaddrinfo_localhost() {
    let node = c"127.0.0.1";
    let mut hints = Netdb::addrinfo::default();
    hints.ai_family = Socket::PF_INET;
    hints.ai_socktype = Socket::SOCK_STREAM as i32;

    let mut result: *const Netdb::addrinfo = core::ptr::null();
    let rc = unsafe {
        Netdb::getaddrinfo(
            node.as_ptr(),
            core::ptr::null(),
            &hints as *const _,
            &mut result as *mut _ as *const *const Netdb::addrinfo,
        )
    };
    assert_eq!(rc, 0, "getaddrinfo should succeed for 127.0.0.1");
    assert!(!result.is_null());

    let ai = unsafe { &*result };
    assert_eq!(ai.ai_family, Socket::PF_INET);
    assert_eq!(ai.ai_socktype, Socket::SOCK_STREAM as i32);

    unsafe { Netdb::freeaddrinfo(result) };
}
