//! End-to-end tests for OpenSSL error queue bindings against real libcrypto.

use bnd_openssl::openssl::{err, types};

#[test]
fn error_record_layouts_match_native_abi() {
    assert_eq!(std::mem::size_of::<err::lhash_st_ERR_STRING_DATA_0>(), 8);
    assert_eq!(std::mem::align_of::<err::lhash_st_ERR_STRING_DATA_0>(), 8);
    assert_eq!(std::mem::size_of::<err::lhash_st_ERR_STRING_DATA>(), 8);
    assert_eq!(std::mem::align_of::<err::lhash_st_ERR_STRING_DATA>(), 8);
    assert_eq!(
        std::mem::offset_of!(err::lhash_st_ERR_STRING_DATA, dummy),
        0
    );

    let _: unsafe extern "C" fn() -> *mut types::ERR_STATE = err::ERR_get_state;
}

#[test]
fn error_queue_round_trip() {
    const REASON: i32 = 42;

    unsafe {
        err::ERR_clear_error();
        assert_eq!(err::ERR_get_error(), 0);

        err::ERR_new();
        err::ERR_set_debug(
            c"err_e2e.rs".as_ptr(),
            line!() as i32,
            c"error_queue_round_trip".as_ptr(),
        );
        err::ERR_set_error(
            err::ERR_LIB_USER,
            REASON,
            c"generated binding test".as_ptr(),
        );

        let packed = err::ERR_get_error();
        assert_ne!(packed, 0);
        assert_eq!(
            (packed >> err::ERR_LIB_OFFSET) & err::ERR_LIB_MASK as u64,
            err::ERR_LIB_USER as u64
        );
        assert_eq!(packed & err::ERR_REASON_MASK as u64, REASON as u64);
        assert_eq!(err::ERR_get_error(), 0);
    }
}
