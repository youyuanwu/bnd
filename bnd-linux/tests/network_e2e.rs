use bnd_linux::libc::{r#in, inet, netdb, socket, socket_type, unistd};

#[test]
fn socketpair_roundtrip() {
    let mut sockets = [0; 2];
    assert_eq!(
        unsafe {
            socket::socketpair(
                socket::AF_UNIX,
                socket_type::SOCK_STREAM as i32,
                0,
                sockets.as_mut_ptr(),
            )
        },
        0
    );

    let input = b"socket";
    assert_eq!(
        unsafe { unistd::write(sockets[0], input.as_ptr().cast(), input.len()) },
        input.len() as i64
    );
    let mut output = [0u8; 6];
    assert_eq!(
        unsafe { unistd::read(sockets[1], output.as_mut_ptr().cast(), output.len()) },
        output.len() as i64
    );
    assert_eq!(&output, input);

    unsafe {
        unistd::close(sockets[0]);
        unistd::close(sockets[1]);
    }
}

#[test]
fn inet_and_getaddrinfo_roundtrip() {
    let mut address = r#in::in_addr::default();
    assert_eq!(
        unsafe {
            inet::inet_pton(
                socket::AF_INET,
                c"127.0.0.1".as_ptr(),
                (&mut address as *mut r#in::in_addr).cast(),
            )
        },
        1
    );

    let mut text = [0i8; 16];
    assert!(
        !unsafe {
            inet::inet_ntop(
                socket::AF_INET,
                (&address as *const r#in::in_addr).cast(),
                text.as_mut_ptr(),
                text.len() as u32,
            )
        }
        .is_null()
    );
    assert_eq!(
        unsafe { std::ffi::CStr::from_ptr(text.as_ptr()) }.to_bytes(),
        b"127.0.0.1"
    );

    let mut network_text = [0i8; 16];
    assert!(
        !unsafe {
            inet::inet_neta(
                u32::from_be_bytes([127, 0, 0, 0]),
                network_text.as_mut_ptr(),
                network_text.len(),
            )
        }
        .is_null()
    );

    let hints = netdb::addrinfo {
        ai_flags: netdb::AI_NUMERICHOST,
        ai_family: socket::AF_INET,
        ..Default::default()
    };
    let mut result = std::ptr::null_mut();
    assert_eq!(
        unsafe { netdb::getaddrinfo(c"127.0.0.1".as_ptr(), std::ptr::null(), &hints, &mut result) },
        0
    );
    assert!(!result.is_null());
    unsafe { netdb::freeaddrinfo(result) };
}
