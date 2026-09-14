windows_link::link!("crypto" "C" fn ERR_add_error_data(num : i32, ...));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn ERR_add_error_mem_bio(sep : *const i8, bio : *mut super::types::BIO));
windows_link::link!("crypto" "C" fn ERR_add_error_txt(sepr : *const i8, txt : *const i8));
windows_link::link!("crypto" "C" fn ERR_add_error_vdata(num : i32, args : *mut core::ffi::c_void));
windows_link::link!("crypto" "C" fn ERR_clear_error());
windows_link::link!("crypto" "C" fn ERR_clear_last_mark() -> i32);
windows_link::link!("crypto" "C" fn ERR_count_to_mark() -> i32);
windows_link::link!("crypto" "C" fn ERR_error_string(e : u64, buf : *mut i8) -> *mut i8);
windows_link::link!("crypto" "C" fn ERR_error_string_n(e : u64, buf : *mut i8, len : usize));
windows_link::link!("crypto" "C" fn ERR_func_error_string(e : u64) -> *const i8);
windows_link::link!("crypto" "C" fn ERR_get_error() -> u64);
windows_link::link!("crypto" "C" fn ERR_get_error_all(file : *mut *mut i8, line : *mut i32, func : *mut *mut i8, data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_get_error_line(file : *mut *mut i8, line : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_get_error_line_data(file : *mut *mut i8, line : *mut i32, data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_get_next_error_library() -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn ERR_get_state() -> *mut super::types::ERR_STATE);
windows_link::link!("crypto" "C" fn ERR_lib_error_string(e : u64) -> *const i8);
windows_link::link!("crypto" "C" fn ERR_load_strings(lib : i32, str : *mut ERR_STRING_DATA) -> i32);
windows_link::link!("crypto" "C" fn ERR_load_strings_const(str : *const ERR_STRING_DATA) -> i32);
windows_link::link!("crypto" "C" fn ERR_new());
windows_link::link!("crypto" "C" fn ERR_peek_error() -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_error_all(file : *mut *mut i8, line : *mut i32, func : *mut *mut i8, data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_error_data(data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_error_func(func : *mut *mut i8) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_error_line(file : *mut *mut i8, line : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_error_line_data(file : *mut *mut i8, line : *mut i32, data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_last_error() -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_last_error_all(file : *mut *mut i8, line : *mut i32, func : *mut *mut i8, data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_last_error_data(data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_last_error_func(func : *mut *mut i8) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_last_error_line(file : *mut *mut i8, line : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_peek_last_error_line_data(file : *mut *mut i8, line : *mut i32, data : *mut *mut i8, flags : *mut i32) -> u64);
windows_link::link!("crypto" "C" fn ERR_pop() -> i32);
windows_link::link!("crypto" "C" fn ERR_pop_to_mark() -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn ERR_print_errors(bp : *mut super::types::BIO));
windows_link::link!("crypto" "C" fn ERR_print_errors_cb(cb : *mut u8, u : *mut core::ffi::c_void));
windows_link::link!("crypto" "C" fn ERR_print_errors_fp(fp : *mut bnd_linux::libc::file::FILE));
windows_link::link!("crypto" "C" fn ERR_reason_error_string(e : u64) -> *const i8);
windows_link::link!("crypto" "C" fn ERR_remove_state(pid : u64));
windows_link::link!("crypto" "C" fn ERR_remove_thread_state(param0 : *mut core::ffi::c_void));
windows_link::link!("crypto" "C" fn ERR_set_debug(file : *const i8, line : i32, func : *const i8));
windows_link::link!("crypto" "C" fn ERR_set_error(lib : i32, reason : i32, fmt : *const i8, ...));
windows_link::link!("crypto" "C" fn ERR_set_error_data(data : *mut i8, flags : i32));
windows_link::link!("crypto" "C" fn ERR_set_mark() -> i32);
windows_link::link!("crypto" "C" fn ERR_unload_strings(lib : i32, str : *mut ERR_STRING_DATA) -> i32);
windows_link::link!("crypto" "C" fn ERR_vset_error(lib : i32, reason : i32, fmt : *const i8, args : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn OSSL_ERR_STATE_free(es : *mut super::types::ERR_STATE));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn OSSL_ERR_STATE_new() -> *mut super::types::ERR_STATE);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn OSSL_ERR_STATE_restore(es : *const super::types::ERR_STATE));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn OSSL_ERR_STATE_save(es : *mut super::types::ERR_STATE));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn OSSL_ERR_STATE_save_to_mark(es : *mut super::types::ERR_STATE));
pub const ERR_FLAG_CLEAR: i32 = 2;
pub const ERR_FLAG_MARK: i32 = 1;
pub const ERR_LIB_ASN1: i32 = 13;
pub const ERR_LIB_ASYNC: i32 = 51;
pub const ERR_LIB_BIO: i32 = 32;
pub const ERR_LIB_BN: i32 = 3;
pub const ERR_LIB_BUF: i32 = 7;
pub const ERR_LIB_CMP: i32 = 58;
pub const ERR_LIB_CMS: i32 = 46;
pub const ERR_LIB_COMP: i32 = 41;
pub const ERR_LIB_CONF: i32 = 14;
pub const ERR_LIB_CRMF: i32 = 56;
pub const ERR_LIB_CRYPTO: i32 = 15;
pub const ERR_LIB_CT: i32 = 50;
pub const ERR_LIB_DH: i32 = 5;
pub const ERR_LIB_DSA: i32 = 10;
pub const ERR_LIB_DSO: i32 = 37;
pub const ERR_LIB_EC: i32 = 16;
pub const ERR_LIB_ECDH: i32 = 43;
pub const ERR_LIB_ECDSA: i32 = 42;
pub const ERR_LIB_ENGINE: i32 = 38;
pub const ERR_LIB_ESS: i32 = 54;
pub const ERR_LIB_EVP: i32 = 6;
pub const ERR_LIB_FIPS: i32 = 45;
pub const ERR_LIB_HMAC: i32 = 48;
pub const ERR_LIB_HTTP: i32 = 61;
pub const ERR_LIB_KDF: i32 = 52;
pub const ERR_LIB_MASK: i32 = 255;
pub const ERR_LIB_NONE: i32 = 1;
pub const ERR_LIB_OBJ: i32 = 8;
pub const ERR_LIB_OCSP: i32 = 39;
pub const ERR_LIB_OFFSET: i32 = 23;
pub const ERR_LIB_OSSL_DECODER: i32 = 60;
pub const ERR_LIB_OSSL_ENCODER: i32 = 59;
pub const ERR_LIB_OSSL_STORE: i32 = 44;
pub const ERR_LIB_PEM: i32 = 9;
pub const ERR_LIB_PKCS12: i32 = 35;
pub const ERR_LIB_PKCS7: i32 = 33;
pub const ERR_LIB_PROP: i32 = 55;
pub const ERR_LIB_PROV: i32 = 57;
pub const ERR_LIB_RAND: i32 = 36;
pub const ERR_LIB_RSA: i32 = 4;
pub const ERR_LIB_SM2: i32 = 53;
pub const ERR_LIB_SSL: i32 = 20;
pub const ERR_LIB_SYS: i32 = 2;
pub const ERR_LIB_TS: i32 = 47;
pub const ERR_LIB_UI: i32 = 40;
pub const ERR_LIB_USER: i32 = 128;
pub const ERR_LIB_X509: i32 = 11;
pub const ERR_LIB_X509V3: i32 = 34;
pub const ERR_MAX_DATA_SIZE: i32 = 1024;
pub const ERR_NUM_ERRORS: i32 = 16;
pub const ERR_REASON_MASK: i32 = 8388607;
pub const ERR_RFLAGS_MASK: i32 = 31;
pub const ERR_RFLAGS_OFFSET: i32 = 18;
pub const ERR_RFLAG_COMMON: i32 = 524288;
pub const ERR_RFLAG_FATAL: i32 = 262144;
pub const ERR_R_ASN1_LIB: i32 = 524301;
pub const ERR_R_BIO_LIB: i32 = 524320;
pub const ERR_R_BN_LIB: i32 = 524291;
pub const ERR_R_BUF_LIB: i32 = 524295;
pub const ERR_R_CMP_LIB: i32 = 524346;
pub const ERR_R_CMS_LIB: i32 = 524334;
pub const ERR_R_CONF_LIB: i32 = 524302;
pub const ERR_R_CRYPTO_LIB: i32 = 524303;
pub const ERR_R_CT_LIB: i32 = 524338;
pub const ERR_R_DH_LIB: i32 = 524293;
pub const ERR_R_DISABLED: i32 = 786692;
pub const ERR_R_DSA_LIB: i32 = 524298;
pub const ERR_R_DSO_LIB: i32 = 524325;
pub const ERR_R_ECDSA_LIB: i32 = 524330;
pub const ERR_R_EC_LIB: i32 = 524304;
pub const ERR_R_ENGINE_LIB: i32 = 524326;
pub const ERR_R_ESS_LIB: i32 = 524342;
pub const ERR_R_EVP_LIB: i32 = 524294;
pub const ERR_R_FATAL: i32 = 786432;
pub const ERR_R_FETCH_FAILED: i32 = 524557;
pub const ERR_R_INIT_FAIL: i32 = 786693;
pub const ERR_R_INTERNAL_ERROR: i32 = 786691;
pub const ERR_R_INTERRUPTED_OR_CANCELLED: i32 = 524553;
pub const ERR_R_INVALID_PROPERTY_DEFINITION: i32 = 524558;
pub const ERR_R_INVALID_PROVIDER_FUNCTIONS: i32 = 786696;
pub const ERR_R_MALLOC_FAILURE: i32 = 786688;
pub const ERR_R_MISSING_ASN1_EOS: i32 = 524555;
pub const ERR_R_NESTED_ASN1_ERROR: i32 = 524554;
pub const ERR_R_OBJ_LIB: i32 = 524296;
pub const ERR_R_OPERATION_FAIL: i32 = 786695;
pub const ERR_R_OSSL_DECODER_LIB: i32 = 524348;
pub const ERR_R_OSSL_ENCODER_LIB: i32 = 524347;
pub const ERR_R_OSSL_STORE_LIB: i32 = 524332;
pub const ERR_R_PASSED_INVALID_ARGUMENT: i32 = 524550;
pub const ERR_R_PASSED_NULL_PARAMETER: i32 = 786690;
pub const ERR_R_PEM_LIB: i32 = 524297;
pub const ERR_R_PKCS12_LIB: i32 = 524323;
pub const ERR_R_PKCS7_LIB: i32 = 524321;
pub const ERR_R_PROV_LIB: i32 = 524345;
pub const ERR_R_RAND_LIB: i32 = 524324;
pub const ERR_R_RSA_LIB: i32 = 524292;
pub const ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED: i32 = 786689;
pub const ERR_R_SSL_LIB: i32 = 524308;
pub const ERR_R_SYS_LIB: i32 = 524290;
pub const ERR_R_TS_LIB: i32 = 524335;
pub const ERR_R_UI_LIB: i32 = 524328;
pub const ERR_R_UNABLE_TO_GET_READ_LOCK: i32 = 786703;
pub const ERR_R_UNABLE_TO_GET_WRITE_LOCK: i32 = 786704;
pub const ERR_R_UNSUPPORTED: i32 = 524556;
pub const ERR_R_X509V3_LIB: i32 = 524322;
pub const ERR_R_X509_LIB: i32 = 524299;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ERR_STRING_DATA {
    pub error: u64,
    pub string: *const i8,
}
pub const ERR_SYSTEM_FLAG: u32 = 2147483648;
pub const ERR_SYSTEM_MASK: u32 = 2147483647;
pub const ERR_TXT_MALLOCED: i32 = 1;
pub const ERR_TXT_STRING: i32 = 2;
pub const SYS_F_ACCEPT: i32 = 0;
pub const SYS_F_BIND: i32 = 0;
pub const SYS_F_CLOSE: i32 = 0;
pub const SYS_F_CONNECT: i32 = 0;
pub const SYS_F_FCNTL: i32 = 0;
pub const SYS_F_FFLUSH: i32 = 0;
pub const SYS_F_FOPEN: i32 = 0;
pub const SYS_F_FREAD: i32 = 0;
pub const SYS_F_FSTAT: i32 = 0;
pub const SYS_F_GETADDRINFO: i32 = 0;
pub const SYS_F_GETHOSTBYNAME: i32 = 0;
pub const SYS_F_GETNAMEINFO: i32 = 0;
pub const SYS_F_GETSERVBYNAME: i32 = 0;
pub const SYS_F_GETSOCKNAME: i32 = 0;
pub const SYS_F_GETSOCKOPT: i32 = 0;
pub const SYS_F_IOCTL: i32 = 0;
pub const SYS_F_IOCTLSOCKET: i32 = 0;
pub const SYS_F_LISTEN: i32 = 0;
pub const SYS_F_OPEN: i32 = 0;
pub const SYS_F_OPENDIR: i32 = 0;
pub const SYS_F_SENDFILE: i32 = 0;
pub const SYS_F_SETSOCKOPT: i32 = 0;
pub const SYS_F_SOCKET: i32 = 0;
pub const SYS_F_STAT: i32 = 0;
pub const SYS_F_WSASTARTUP: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct err_state_st {
    pub err_flags: [i32; 16],
    pub err_marks: [i32; 16],
    pub err_buffer: [u64; 16],
    pub err_data: [*mut i8; 16],
    pub err_data_size: [usize; 16],
    pub err_data_flags: [i32; 16],
    pub err_file: [*mut i8; 16],
    pub err_line: [i32; 16],
    pub err_func: [*mut i8; 16],
    pub top: i32,
    pub bottom: i32,
}
impl Default for err_state_st {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type lh_ERR_STRING_DATA_compfunc =
    Option<unsafe extern "C" fn(a: *const ERR_STRING_DATA, b: *const ERR_STRING_DATA) -> i32>;
pub type lh_ERR_STRING_DATA_doallfunc = Option<unsafe extern "C" fn(a: *mut ERR_STRING_DATA)>;
pub type lh_ERR_STRING_DATA_hashfunc =
    Option<unsafe extern "C" fn(a: *const ERR_STRING_DATA) -> u64>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct lhash_st_ERR_STRING_DATA {
    pub dummy: lhash_st_ERR_STRING_DATA_0,
}
impl Default for lhash_st_ERR_STRING_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union lhash_st_ERR_STRING_DATA_0 {
    pub d1: *mut core::ffi::c_void,
    pub d2: u64,
    pub d3: i32,
}
impl Default for lhash_st_ERR_STRING_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
