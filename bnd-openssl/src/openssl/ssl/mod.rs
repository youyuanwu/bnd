#[cfg(feature = "bio")]
windows_link::link!("ssl" "C" fn BIO_f_ssl() -> *const super::bio::BIO_METHOD);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn BIO_new_buffer_ssl_connect(ctx : *mut super::types::SSL_CTX) -> *mut super::types::BIO);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn BIO_new_ssl(ctx : *mut super::types::SSL_CTX, client : i32) -> *mut super::types::BIO);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn BIO_new_ssl_connect(ctx : *mut super::types::SSL_CTX) -> *mut super::types::BIO);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn BIO_ssl_copy_session_id(to : *mut super::types::BIO, from : *mut super::types::BIO) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn BIO_ssl_shutdown(ssl_bio : *mut super::types::BIO));
windows_link::link!("ssl" "C" fn DTLS_client_method() -> *const SSL_METHOD);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn DTLS_get_data_mtu(s : *const super::types::SSL) -> usize);
windows_link::link!("ssl" "C" fn DTLS_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn DTLS_server_method() -> *const SSL_METHOD);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn DTLS_set_timer_cb(s : *mut super::types::SSL, cb : DTLS_timer_cb));
windows_link::link!("ssl" "C" fn DTLSv1_2_client_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn DTLSv1_2_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn DTLSv1_2_server_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn DTLSv1_client_method() -> *const SSL_METHOD);
#[cfg(all(feature = "bio", feature = "types"))]
windows_link::link!("ssl" "C" fn DTLSv1_listen(s : *mut super::types::SSL, client : *mut super::bio::BIO_ADDR) -> i32);
windows_link::link!("ssl" "C" fn DTLSv1_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn DTLSv1_server_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn OPENSSL_cipher_name(rfc_name : *const i8) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn OPENSSL_init_ssl(opts : u64, settings : *const super::types::OPENSSL_INIT_SETTINGS) -> i32);
windows_link::link!("ssl" "C" fn OSSL_default_cipher_list() -> *const i8);
windows_link::link!("ssl" "C" fn OSSL_default_ciphersuites() -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn PEM_read_SSL_SESSION(out : *mut bnd_linux::libc::file::FILE, x : *mut *mut SSL_SESSION, cb : super::types::pem_password_cb, u : *mut core::ffi::c_void) -> *mut SSL_SESSION);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn PEM_read_bio_SSL_SESSION(out : *mut super::types::BIO, x : *mut *mut SSL_SESSION, cb : super::types::pem_password_cb, u : *mut core::ffi::c_void) -> *mut SSL_SESSION);
windows_link::link!("ssl" "C" fn PEM_write_SSL_SESSION(out : *mut bnd_linux::libc::file::FILE, x : *const SSL_SESSION) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn PEM_write_bio_SSL_SESSION(out : *mut super::types::BIO, x : *const SSL_SESSION) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SRP_Calc_A_param(s : *mut super::types::SSL) -> i32);
windows_link::link!("ssl" "C" fn SSL_CIPHER_description(param0 : *const SSL_CIPHER, buf : *mut i8, size : i32) -> *mut i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CIPHER_find(ssl : *mut super::types::SSL, ptr : *const u8) -> *const SSL_CIPHER);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_auth_nid(c : *const SSL_CIPHER) -> i32);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_bits(c : *const SSL_CIPHER, alg_bits : *mut i32) -> i32);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_cipher_nid(c : *const SSL_CIPHER) -> i32);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_digest_nid(c : *const SSL_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_handshake_digest(c : *const SSL_CIPHER) -> *const super::types::EVP_MD);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_id(c : *const SSL_CIPHER) -> u32);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_kx_nid(c : *const SSL_CIPHER) -> i32);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_name(c : *const SSL_CIPHER) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_protocol_id(c : *const SSL_CIPHER) -> u16);
windows_link::link!("ssl" "C" fn SSL_CIPHER_get_version(c : *const SSL_CIPHER) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_CIPHER_is_aead(c : *const SSL_CIPHER) -> i32);
windows_link::link!("ssl" "C" fn SSL_CIPHER_standard_name(c : *const SSL_CIPHER) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_COMP_add_compression_method(id : i32, cm : *mut super::types::COMP_METHOD) -> i32);
#[cfg(feature = "comp")]
windows_link::link!("ssl" "C" fn SSL_COMP_get0_name(comp : *const super::comp::SSL_COMP) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_COMP_get_compression_methods() -> *mut core::ffi::c_void);
#[cfg(feature = "comp")]
windows_link::link!("ssl" "C" fn SSL_COMP_get_id(comp : *const super::comp::SSL_COMP) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_COMP_get_name(comp : *const super::types::COMP_METHOD) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_COMP_set0_compression_methods(meths : *mut core::ffi::c_void) -> *mut core::ffi::c_void);
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_clear_flags(cctx : *mut SSL_CONF_CTX, flags : u32) -> u32);
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_finish(cctx : *mut SSL_CONF_CTX) -> i32);
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_free(cctx : *mut SSL_CONF_CTX));
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_new() -> *mut SSL_CONF_CTX);
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_set1_prefix(cctx : *mut SSL_CONF_CTX, pre : *const i8) -> i32);
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_set_flags(cctx : *mut SSL_CONF_CTX, flags : u32) -> u32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_set_ssl(cctx : *mut SSL_CONF_CTX, ssl : *mut super::types::SSL));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CONF_CTX_set_ssl_ctx(cctx : *mut SSL_CONF_CTX, ctx : *mut super::types::SSL_CTX));
windows_link::link!("ssl" "C" fn SSL_CONF_cmd(cctx : *mut SSL_CONF_CTX, cmd : *const i8, value : *const i8) -> i32);
windows_link::link!("ssl" "C" fn SSL_CONF_cmd_argv(cctx : *mut SSL_CONF_CTX, pargc : *mut i32, pargv : *mut *mut *mut i8) -> i32);
windows_link::link!("ssl" "C" fn SSL_CONF_cmd_value_type(cctx : *mut SSL_CONF_CTX, cmd : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_SRP_CTX_free(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_SRP_CTX_init(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_add1_to_CA_list(ctx : *mut super::types::SSL_CTX, x : *const super::types::X509) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_add_client_CA(ctx : *mut super::types::SSL_CTX, x : *mut super::types::X509) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_add_client_custom_ext(ctx : *mut super::types::SSL_CTX, ext_type : u32, add_cb : custom_ext_add_cb, free_cb : custom_ext_free_cb, add_arg : *mut core::ffi::c_void, parse_cb : custom_ext_parse_cb, parse_arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_add_custom_ext(ctx : *mut super::types::SSL_CTX, ext_type : u32, context : u32, add_cb : SSL_custom_ext_add_cb_ex, free_cb : SSL_custom_ext_free_cb_ex, add_arg : *mut core::ffi::c_void, parse_cb : SSL_custom_ext_parse_cb_ex, parse_arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_add_server_custom_ext(ctx : *mut super::types::SSL_CTX, ext_type : u32, add_cb : custom_ext_add_cb, free_cb : custom_ext_free_cb, add_arg : *mut core::ffi::c_void, parse_cb : custom_ext_parse_cb, parse_arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_add_session(ctx : *mut super::types::SSL_CTX, session : *mut SSL_SESSION) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_callback_ctrl(param0 : *mut super::types::SSL_CTX, param1 : i32, param2 : *mut u8) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_check_private_key(ctx : *const super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_clear_options(ctx : *mut super::types::SSL_CTX, op : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_compress_certs(ctx : *mut super::types::SSL_CTX, alg : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_config(ctx : *mut super::types::SSL_CTX, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_ct_is_enabled(ctx : *const super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_ctrl(ctx : *mut super::types::SSL_CTX, cmd : i32, larg : i64, parg : *mut core::ffi::c_void) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_dane_clear_flags(ctx : *mut super::types::SSL_CTX, flags : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_dane_enable(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_dane_mtype_set(ctx : *mut super::types::SSL_CTX, md : *const super::types::EVP_MD, mtype : u8, ord : u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_dane_set_flags(ctx : *mut super::types::SSL_CTX, flags : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_enable_ct(ctx : *mut super::types::SSL_CTX, validation_mode : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_flush_sessions(ctx : *mut super::types::SSL_CTX, tm : i64));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_flush_sessions_ex(ctx : *mut super::types::SSL_CTX, tm : bnd_linux::libc::time_t::time_t));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_free(param0 : *mut super::types::SSL_CTX));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_CA_list(ctx : *const super::types::SSL_CTX) -> *const core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_certificate(ctx : *const super::types::SSL_CTX) -> *mut super::types::X509);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_client_cert_type(ctx : *const super::types::SSL_CTX, t : *mut *mut u8, len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_ctlog_store(ctx : *const super::types::SSL_CTX) -> *const super::types::CTLOG_STORE);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_param(ctx : *mut super::types::SSL_CTX) -> *mut super::types::X509_VERIFY_PARAM);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_privatekey(ctx : *const super::types::SSL_CTX) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_security_ex_data(ctx : *const super::types::SSL_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get0_server_cert_type(s : *const super::types::SSL_CTX, t : *mut *mut u8, len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get1_compressed_cert(ctx : *mut super::types::SSL_CTX, alg : i32, data : *mut *mut u8, orig_len : *mut usize) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_cert_store(param0 : *const super::types::SSL_CTX) -> *mut super::types::X509_STORE);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_ciphers(ctx : *const super::types::SSL_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_client_CA_list(s : *const super::types::SSL_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_client_cert_cb(ctx : *mut super::types::SSL_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_default_passwd_cb(ctx : *mut super::types::SSL_CTX) -> super::types::pem_password_cb);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_default_passwd_cb_userdata(ctx : *mut super::types::SSL_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_domain_flags(ctx : *const super::types::SSL_CTX, domain_flags : *mut u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_ex_data(ssl : *const super::types::SSL_CTX, idx : i32) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_info_callback(ctx : *mut super::types::SSL_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_keylog_callback(ctx : *const super::types::SSL_CTX) -> SSL_CTX_keylog_cb_func);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_max_early_data(ctx : *const super::types::SSL_CTX) -> u32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_num_tickets(ctx : *const super::types::SSL_CTX) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_options(ctx : *const super::types::SSL_CTX) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_quiet_shutdown(ctx : *const super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_record_padding_callback_arg(ctx : *const super::types::SSL_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_recv_max_early_data(ctx : *const super::types::SSL_CTX) -> u32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_security_callback(ctx : *const super::types::SSL_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_security_level(ctx : *const super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_ssl_method(ctx : *const super::types::SSL_CTX) -> *const SSL_METHOD);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_timeout(ctx : *const super::types::SSL_CTX) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_verify_callback(ctx : *const super::types::SSL_CTX) -> SSL_verify_cb);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_verify_depth(ctx : *const super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_get_verify_mode(ctx : *const super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_has_client_custom_ext(ctx : *const super::types::SSL_CTX, ext_type : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_load_verify_dir(ctx : *mut super::types::SSL_CTX, capath : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_load_verify_file(ctx : *mut super::types::SSL_CTX, cafile : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_load_verify_locations(ctx : *mut super::types::SSL_CTX, cafile : *const i8, capath : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_load_verify_store(ctx : *mut super::types::SSL_CTX, castore : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_new(meth : *const SSL_METHOD) -> *mut super::types::SSL_CTX);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_new_ex(libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8, meth : *const SSL_METHOD) -> *mut super::types::SSL_CTX);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_remove_session(ctx : *mut super::types::SSL_CTX, session : *mut SSL_SESSION) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_sess_get_get_cb(ctx : *mut super::types::SSL_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_sess_get_new_cb(ctx : *mut super::types::SSL_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_sess_get_remove_cb(ctx : *mut super::types::SSL_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_sess_set_get_cb(ctx : *mut super::types::SSL_CTX, get_session_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_sess_set_new_cb(ctx : *mut super::types::SSL_CTX, new_session_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_sess_set_remove_cb(ctx : *mut super::types::SSL_CTX, remove_session_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_sessions(ctx : *mut super::types::SSL_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set0_CA_list(ctx : *mut super::types::SSL_CTX, name_list : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set0_ctlog_store(ctx : *mut super::types::SSL_CTX, logs : *mut super::types::CTLOG_STORE));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set0_security_ex_data(ctx : *mut super::types::SSL_CTX, ex : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set0_tmp_dh_pkey(ctx : *mut super::types::SSL_CTX, dhpkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set1_cert_comp_preference(ctx : *mut super::types::SSL_CTX, algs : *mut i32, len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set1_cert_store(param0 : *mut super::types::SSL_CTX, param1 : *mut super::types::X509_STORE));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set1_client_cert_type(ctx : *mut super::types::SSL_CTX, val : *const u8, len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set1_compressed_cert(ctx : *mut super::types::SSL_CTX, algorithm : i32, comp_data : *mut u8, comp_length : usize, orig_length : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set1_param(ctx : *mut super::types::SSL_CTX, vpm : *mut super::types::X509_VERIFY_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set1_server_cert_type(ctx : *mut super::types::SSL_CTX, val : *const u8, len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_allow_early_data_cb(ctx : *mut super::types::SSL_CTX, cb : SSL_allow_early_data_cb_fn, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_alpn_protos(ctx : *mut super::types::SSL_CTX, protos : *const u8, protos_len : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_alpn_select_cb(ctx : *mut super::types::SSL_CTX, cb : SSL_CTX_alpn_select_cb_func, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_async_callback(ctx : *mut super::types::SSL_CTX, callback : SSL_async_callback_fn) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_async_callback_arg(ctx : *mut super::types::SSL_CTX, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_block_padding(ctx : *mut super::types::SSL_CTX, block_size : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_block_padding_ex(ctx : *mut super::types::SSL_CTX, app_block_size : usize, hs_block_size : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_cert_cb(c : *mut super::types::SSL_CTX, cb : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_cert_store(param0 : *mut super::types::SSL_CTX, param1 : *mut super::types::X509_STORE));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_cert_verify_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_cipher_list(param0 : *mut super::types::SSL_CTX, str : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_ciphersuites(ctx : *mut super::types::SSL_CTX, str : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_client_CA_list(ctx : *mut super::types::SSL_CTX, name_list : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_client_cert_cb(ctx : *mut super::types::SSL_CTX, client_cert_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_client_cert_engine(ctx : *mut super::types::SSL_CTX, e : *mut super::types::ENGINE) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_client_hello_cb(c : *mut super::types::SSL_CTX, cb : SSL_client_hello_cb_fn, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_cookie_generate_cb(ctx : *mut super::types::SSL_CTX, app_gen_cookie_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_cookie_verify_cb(ctx : *mut super::types::SSL_CTX, app_verify_cookie_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_ct_validation_callback(ctx : *mut super::types::SSL_CTX, callback : ssl_ct_validation_cb, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_ctlog_list_file(ctx : *mut super::types::SSL_CTX, path : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_ctlog_list_file(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_passwd_cb(ctx : *mut super::types::SSL_CTX, cb : super::types::pem_password_cb));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_passwd_cb_userdata(ctx : *mut super::types::SSL_CTX, u : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_read_buffer_len(ctx : *mut super::types::SSL_CTX, len : usize));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_verify_dir(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_verify_file(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_verify_paths(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_default_verify_store(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_domain_flags(ctx : *mut super::types::SSL_CTX, domain_flags : u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_ex_data(ssl : *mut super::types::SSL_CTX, idx : i32, data : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_generate_session_id(ctx : *mut super::types::SSL_CTX, cb : GEN_SESSION_CB) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_info_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_keylog_callback(ctx : *mut super::types::SSL_CTX, cb : SSL_CTX_keylog_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_max_early_data(ctx : *mut super::types::SSL_CTX, max_early_data : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_msg_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_new_pending_conn_cb(c : *mut super::types::SSL_CTX, cb : SSL_new_pending_conn_cb_fn, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_next_proto_select_cb(s : *mut super::types::SSL_CTX, cb : SSL_CTX_npn_select_cb_func, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_next_protos_advertised_cb(s : *mut super::types::SSL_CTX, cb : SSL_CTX_npn_advertised_cb_func, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_not_resumable_session_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_num_tickets(ctx : *mut super::types::SSL_CTX, num_tickets : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_options(ctx : *mut super::types::SSL_CTX, op : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_post_handshake_auth(ctx : *mut super::types::SSL_CTX, val : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_psk_client_callback(ctx : *mut super::types::SSL_CTX, cb : SSL_psk_client_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_psk_find_session_callback(ctx : *mut super::types::SSL_CTX, cb : SSL_psk_find_session_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_psk_server_callback(ctx : *mut super::types::SSL_CTX, cb : SSL_psk_server_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_psk_use_session_callback(ctx : *mut super::types::SSL_CTX, cb : SSL_psk_use_session_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_purpose(ctx : *mut super::types::SSL_CTX, purpose : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_quiet_shutdown(ctx : *mut super::types::SSL_CTX, mode : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_record_padding_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_record_padding_callback_arg(ctx : *mut super::types::SSL_CTX, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_recv_max_early_data(ctx : *mut super::types::SSL_CTX, recv_max_early_data : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_security_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_security_level(ctx : *mut super::types::SSL_CTX, level : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_session_id_context(ctx : *mut super::types::SSL_CTX, sid_ctx : *const u8, sid_ctx_len : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_session_ticket_cb(ctx : *mut super::types::SSL_CTX, gen_cb : SSL_CTX_generate_session_ticket_fn, dec_cb : SSL_CTX_decrypt_session_ticket_fn, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_srp_cb_arg(ctx : *mut super::types::SSL_CTX, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_srp_client_pwd_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_srp_password(ctx : *mut super::types::SSL_CTX, password : *mut i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_srp_strength(ctx : *mut super::types::SSL_CTX, strength : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_srp_username(ctx : *mut super::types::SSL_CTX, name : *mut i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_srp_username_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_srp_verify_param_callback(ctx : *mut super::types::SSL_CTX, cb : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_ssl_version(ctx : *mut super::types::SSL_CTX, meth : *const SSL_METHOD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_stateless_cookie_generate_cb(ctx : *mut super::types::SSL_CTX, gen_stateless_cookie_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_stateless_cookie_verify_cb(ctx : *mut super::types::SSL_CTX, verify_stateless_cookie_cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_timeout(ctx : *mut super::types::SSL_CTX, t : i64) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_tmp_dh_callback(ctx : *mut super::types::SSL_CTX, dh : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_trust(ctx : *mut super::types::SSL_CTX, trust : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_verify(ctx : *mut super::types::SSL_CTX, mode : i32, callback : SSL_verify_cb));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_verify_depth(ctx : *mut super::types::SSL_CTX, depth : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_up_ref(ctx : *mut super::types::SSL_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_PrivateKey(ctx : *mut super::types::SSL_CTX, pkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_PrivateKey_ASN1(pk : i32, ctx : *mut super::types::SSL_CTX, d : *const u8, len : i64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_PrivateKey_file(ctx : *mut super::types::SSL_CTX, file : *const i8, r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_RSAPrivateKey(ctx : *mut super::types::SSL_CTX, rsa : *mut super::types::RSA) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_RSAPrivateKey_ASN1(ctx : *mut super::types::SSL_CTX, d : *const u8, len : i64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_RSAPrivateKey_file(ctx : *mut super::types::SSL_CTX, file : *const i8, r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_cert_and_key(ctx : *mut super::types::SSL_CTX, x509 : *mut super::types::X509, privatekey : *mut super::types::EVP_PKEY, chain : *mut core::ffi::c_void, r#override : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_certificate(ctx : *mut super::types::SSL_CTX, x : *mut super::types::X509) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_certificate_ASN1(ctx : *mut super::types::SSL_CTX, len : i32, d : *const u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_certificate_chain_file(ctx : *mut super::types::SSL_CTX, file : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_certificate_file(ctx : *mut super::types::SSL_CTX, file : *const i8, r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_psk_identity_hint(ctx : *mut super::types::SSL_CTX, identity_hint : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_serverinfo(ctx : *mut super::types::SSL_CTX, serverinfo : *const u8, serverinfo_length : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_serverinfo_ex(ctx : *mut super::types::SSL_CTX, version : u32, serverinfo : *const u8, serverinfo_length : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_use_serverinfo_file(ctx : *mut super::types::SSL_CTX, file : *const i8) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_dup(src : *const SSL_SESSION) -> *mut SSL_SESSION);
windows_link::link!("ssl" "C" fn SSL_SESSION_free(ses : *mut SSL_SESSION));
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_alpn_selected(s : *const SSL_SESSION, alpn : *mut *mut u8, len : *mut usize));
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_cipher(s : *const SSL_SESSION) -> *const SSL_CIPHER);
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_hostname(s : *const SSL_SESSION) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_id_context(s : *const SSL_SESSION, len : *mut u32) -> *const u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_peer(s : *mut SSL_SESSION) -> *mut super::types::X509);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_peer_rpk(s : *mut SSL_SESSION) -> *mut super::types::EVP_PKEY);
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_ticket(s : *const SSL_SESSION, tick : *mut *mut u8, len : *mut usize));
windows_link::link!("ssl" "C" fn SSL_SESSION_get0_ticket_appdata(ss : *mut SSL_SESSION, data : *mut *mut core::ffi::c_void, len : *mut usize) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_compress_id(s : *const SSL_SESSION) -> u32);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_ex_data(ss : *const SSL_SESSION, idx : i32) -> *mut core::ffi::c_void);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_id(s : *const SSL_SESSION, len : *mut u32) -> *const u8);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_master_key(sess : *const SSL_SESSION, out : *mut u8, outlen : usize) -> usize);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_max_early_data(s : *const SSL_SESSION) -> u32);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_max_fragment_length(sess : *const SSL_SESSION) -> u8);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_protocol_version(s : *const SSL_SESSION) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_ticket_lifetime_hint(s : *const SSL_SESSION) -> u64);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_time(s : *const SSL_SESSION) -> i64);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_time_ex(s : *const SSL_SESSION) -> bnd_linux::libc::time_t::time_t);
windows_link::link!("ssl" "C" fn SSL_SESSION_get_timeout(s : *const SSL_SESSION) -> i64);
windows_link::link!("ssl" "C" fn SSL_SESSION_has_ticket(s : *const SSL_SESSION) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_is_resumable(s : *const SSL_SESSION) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_new() -> *mut SSL_SESSION);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_SESSION_print(fp : *mut super::types::BIO, ses : *const SSL_SESSION) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_print_fp(fp : *mut bnd_linux::libc::file::FILE, ses : *const SSL_SESSION) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_SESSION_print_keylog(bp : *mut super::types::BIO, x : *const SSL_SESSION) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set1_alpn_selected(s : *mut SSL_SESSION, alpn : *const u8, len : usize) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set1_hostname(s : *mut SSL_SESSION, hostname : *const i8) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set1_id(s : *mut SSL_SESSION, sid : *const u8, sid_len : u32) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set1_id_context(s : *mut SSL_SESSION, sid_ctx : *const u8, sid_ctx_len : u32) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set1_master_key(sess : *mut SSL_SESSION, r#in : *const u8, len : usize) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set1_ticket_appdata(ss : *mut SSL_SESSION, data : *const core::ffi::c_void, len : usize) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set_cipher(s : *mut SSL_SESSION, cipher : *const SSL_CIPHER) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set_ex_data(ss : *mut SSL_SESSION, idx : i32, data : *mut core::ffi::c_void) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set_max_early_data(s : *mut SSL_SESSION, max_early_data : u32) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set_protocol_version(s : *mut SSL_SESSION, version : i32) -> i32);
windows_link::link!("ssl" "C" fn SSL_SESSION_set_time(s : *mut SSL_SESSION, t : i64) -> i64);
windows_link::link!("ssl" "C" fn SSL_SESSION_set_time_ex(s : *mut SSL_SESSION, t : bnd_linux::libc::time_t::time_t) -> bnd_linux::libc::time_t::time_t);
windows_link::link!("ssl" "C" fn SSL_SESSION_set_timeout(s : *mut SSL_SESSION, t : i64) -> i64);
windows_link::link!("ssl" "C" fn SSL_SESSION_up_ref(ses : *mut SSL_SESSION) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_SRP_CTX_free(ctx : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_SRP_CTX_init(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_accept(ssl : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_accept_connection(ssl : *mut super::types::SSL, flags : u64) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_accept_stream(s : *mut super::types::SSL, flags : u64) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_add1_host(s : *mut super::types::SSL, host : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_add1_to_CA_list(ssl : *mut super::types::SSL, x : *const super::types::X509) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_add_client_CA(ssl : *mut super::types::SSL, x : *mut super::types::X509) -> i32);
windows_link::link!("ssl" "C" fn SSL_add_dir_cert_subjects_to_stack(stackcas : *mut core::ffi::c_void, dir : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_add_expected_rpk(s : *mut super::types::SSL, rpk : *mut super::types::EVP_PKEY) -> i32);
windows_link::link!("ssl" "C" fn SSL_add_file_cert_subjects_to_stack(stackcas : *mut core::ffi::c_void, file : *const i8) -> i32);
windows_link::link!("ssl" "C" fn SSL_add_ssl_module());
windows_link::link!("ssl" "C" fn SSL_add_store_cert_subjects_to_stack(stackcas : *mut core::ffi::c_void, uri : *const i8) -> i32);
windows_link::link!("ssl" "C" fn SSL_alert_desc_string(value : i32) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_alert_desc_string_long(value : i32) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_alert_type_string(value : i32) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_alert_type_string_long(value : i32) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_alloc_buffers(ssl : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_bytes_to_cipher_list(s : *mut super::types::SSL, bytes : *const u8, len : usize, isv2format : i32, sk : *mut *mut core::ffi::c_void, scsvs : *mut *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_callback_ctrl(param0 : *mut super::types::SSL, param1 : i32, param2 : *mut u8) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_certs_clear(s : *mut super::types::SSL));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_check_private_key(ctx : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_clear(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_clear_options(s : *mut super::types::SSL, op : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get0_ciphers(s : *mut super::types::SSL, out : *mut *mut u8) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get0_compression_methods(s : *mut super::types::SSL, out : *mut *mut u8) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get0_ext(s : *mut super::types::SSL, r#type : u32, out : *mut *mut u8, outlen : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get0_legacy_version(s : *mut super::types::SSL) -> u32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get0_random(s : *mut super::types::SSL, out : *mut *mut u8) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get0_session_id(s : *mut super::types::SSL, out : *mut *mut u8) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get1_extensions_present(s : *mut super::types::SSL, out : *mut *mut i32, outlen : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_get_extension_order(s : *mut super::types::SSL, exts : *mut u16, num_exts : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_hello_isv2(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_client_version(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_compress_certs(ssl : *mut super::types::SSL, alg : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_config(s : *mut super::types::SSL, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_connect(ssl : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_copy_session_id(to : *mut super::types::SSL, from : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_ct_is_enabled(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_ctrl(ssl : *mut super::types::SSL, cmd : i32, larg : i64, parg : *mut core::ffi::c_void) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_dane_clear_flags(ssl : *mut super::types::SSL, flags : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_dane_enable(s : *mut super::types::SSL, basedomain : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_dane_set_flags(ssl : *mut super::types::SSL, flags : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_dane_tlsa_add(s : *mut super::types::SSL, usage : u8, selector : u8, mtype : u8, data : *const u8, dlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_do_handshake(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_dup(ssl : *mut super::types::SSL) -> *mut super::types::SSL);
windows_link::link!("ssl" "C" fn SSL_dup_CA_list(sk : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_enable_ct(s : *mut super::types::SSL, validation_mode : i32) -> i32);
windows_link::link!("ssl" "C" fn SSL_extension_supported(ext_type : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_free(ssl : *mut super::types::SSL));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_free_buffers(ssl : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_CA_list(s : *const super::types::SSL) -> *const core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_alpn_selected(ssl : *const super::types::SSL, data : *mut *mut u8, len : *mut u32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_client_cert_type(s : *const super::types::SSL, t : *mut *mut u8, len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_connection(s : *mut super::types::SSL) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_dane(ssl : *mut super::types::SSL) -> *mut super::types::SSL_DANE);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_dane_authority(s : *mut super::types::SSL, mcert : *mut *mut super::types::X509, mspki : *mut *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_dane_tlsa(s : *mut super::types::SSL, usage : *mut u8, selector : *mut u8, mtype : *mut u8, data : *mut *mut u8, dlen : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_domain(s : *mut super::types::SSL) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_group_name(s : *mut super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_listener(s : *mut super::types::SSL) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_next_proto_negotiated(s : *const super::types::SSL, data : *mut *mut u8, len : *mut u32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_param(ssl : *mut super::types::SSL) -> *mut super::types::X509_VERIFY_PARAM);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_peer_CA_list(s : *const super::types::SSL) -> *const core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_peer_certificate(s : *const super::types::SSL) -> *mut super::types::X509);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_peer_rpk(s : *const super::types::SSL) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_peer_scts(s : *mut super::types::SSL) -> *const core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_peername(s : *mut super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_security_ex_data(s : *const super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_server_cert_type(s : *const super::types::SSL, t : *mut *mut u8, len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get0_verified_chain(s : *const super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get1_compressed_cert(ssl : *mut super::types::SSL, alg : i32, data : *mut *mut u8, orig_len : *mut usize) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get1_peer_certificate(s : *const super::types::SSL) -> *mut super::types::X509);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get1_session(ssl : *mut super::types::SSL) -> *mut SSL_SESSION);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get1_supported_ciphers(s : *mut super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_SSL_CTX(ssl : *const super::types::SSL) -> *mut super::types::SSL_CTX);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_accept_connection_queue_len(ssl : *mut super::types::SSL) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_accept_stream_queue_len(s : *mut super::types::SSL) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_all_async_fds(s : *mut super::types::SSL, fds : *mut i32, numfds : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_async_status(s : *mut super::types::SSL, status : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_blocking_mode(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_certificate(ssl : *const super::types::SSL) -> *mut super::types::X509);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_changed_async_fds(s : *mut super::types::SSL, addfd : *mut i32, numaddfds : *mut usize, delfd : *mut i32, numdelfds : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_cipher_list(s : *const super::types::SSL, n : i32) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_ciphers(s : *const super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_client_CA_list(s : *const super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_client_ciphers(s : *const super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_client_random(ssl : *const super::types::SSL, out : *mut u8, outlen : usize) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_conn_close_info(ssl : *mut super::types::SSL, info : *mut SSL_CONN_CLOSE_INFO, info_len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_current_cipher(s : *const super::types::SSL) -> *const SSL_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_current_compression(s : *const super::types::SSL) -> *const super::types::COMP_METHOD);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_current_expansion(s : *const super::types::SSL) -> *const super::types::COMP_METHOD);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_default_passwd_cb(s : *mut super::types::SSL) -> super::types::pem_password_cb);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_default_passwd_cb_userdata(s : *mut super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_default_timeout(s : *const super::types::SSL) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_domain_flags(ssl : *const super::types::SSL, domain_flags : *mut u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_early_data_status(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_error(s : *const super::types::SSL, ret_code : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_event_timeout(s : *mut super::types::SSL, tv : *mut bnd_linux::libc::struct_timeval::timeval, is_infinite : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_ex_data(ssl : *const super::types::SSL, idx : i32) -> *mut core::ffi::c_void);
windows_link::link!("ssl" "C" fn SSL_get_ex_data_X509_STORE_CTX_idx() -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_fd(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_finished(s : *const super::types::SSL, buf : *mut core::ffi::c_void, count : usize) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_handshake_rtt(s : *const super::types::SSL, rtt : *mut u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_info_callback(ssl : *const super::types::SSL) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_key_update_type(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_max_early_data(s : *const super::types::SSL) -> u32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_negotiated_client_cert_type(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_negotiated_server_cert_type(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_num_tickets(s : *const super::types::SSL) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_options(s : *const super::types::SSL) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_peer_cert_chain(s : *const super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_peer_finished(s : *const super::types::SSL, buf : *mut core::ffi::c_void, count : usize) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_pending_cipher(s : *const super::types::SSL) -> *const SSL_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_privatekey(ssl : *const super::types::SSL) -> *mut super::types::evp_pkey_st);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_psk_identity(s : *const super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_psk_identity_hint(s : *const super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_quiet_shutdown(ssl : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_rbio(s : *const super::types::SSL) -> *mut super::types::BIO);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_read_ahead(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_record_padding_callback_arg(ssl : *const super::types::SSL) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_recv_max_early_data(s : *const super::types::SSL) -> u32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_rfd(s : *const super::types::SSL) -> i32);
#[cfg(all(feature = "bio", feature = "types"))]
windows_link::link!("ssl" "C" fn SSL_get_rpoll_descriptor(s : *mut super::types::SSL, desc : *mut super::bio::BIO_POLL_DESCRIPTOR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_security_callback(s : *const super::types::SSL) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_security_level(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_server_random(ssl : *const super::types::SSL, out : *mut u8, outlen : usize) -> usize);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_session(ssl : *const super::types::SSL) -> *mut SSL_SESSION);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_shared_ciphers(s : *const super::types::SSL, buf : *mut i8, size : i32) -> *mut i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_shutdown(ssl : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_srp_N(s : *mut super::types::SSL) -> *mut super::types::BIGNUM);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_srp_g(s : *mut super::types::SSL) -> *mut super::types::BIGNUM);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_srp_userinfo(s : *mut super::types::SSL) -> *mut i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_srp_username(s : *mut super::types::SSL) -> *mut i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_ssl_method(s : *const super::types::SSL) -> *const SSL_METHOD);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_state(ssl : *const super::types::SSL) -> OSSL_HANDSHAKE_STATE);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_stream_id(s : *mut super::types::SSL) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_stream_read_error_code(ssl : *mut super::types::SSL, app_error_code : *mut u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_stream_read_state(ssl : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_stream_type(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_stream_write_error_code(ssl : *mut super::types::SSL, app_error_code : *mut u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_stream_write_state(ssl : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_value_uint(s : *mut super::types::SSL, class_ : u32, id : u32, v : *mut u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_verify_callback(s : *const super::types::SSL) -> SSL_verify_cb);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_verify_depth(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_verify_mode(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_verify_result(ssl : *const super::types::SSL) -> i64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_version(s : *const super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_wbio(s : *const super::types::SSL) -> *mut super::types::BIO);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_wfd(s : *const super::types::SSL) -> i32);
#[cfg(all(feature = "bio", feature = "types"))]
windows_link::link!("ssl" "C" fn SSL_get_wpoll_descriptor(s : *mut super::types::SSL, desc : *mut super::bio::BIO_POLL_DESCRIPTOR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_group_to_name(s : *mut super::types::SSL, id : i32) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_handle_events(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_has_matching_session_id(s : *const super::types::SSL, id : *const u8, id_len : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_has_pending(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_in_before(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_in_init(s : *const super::types::SSL) -> i32);
#[cfg(all(feature = "bio", feature = "types"))]
windows_link::link!("ssl" "C" fn SSL_inject_net_dgram(s : *mut super::types::SSL, buf : *const u8, buf_len : usize, peer : *const super::bio::BIO_ADDR, local : *const super::bio::BIO_ADDR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_connection(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_domain(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_dtls(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_init_finished(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_listener(ssl : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_quic(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_server(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_stream_local(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_is_tls(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_key_update(s : *mut super::types::SSL, updatetype : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_listen(ssl : *mut super::types::SSL) -> i32);
windows_link::link!("ssl" "C" fn SSL_load_client_CA_file(file : *const i8) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_load_client_CA_file_ex(file : *const i8, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_net_read_desired(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_net_write_desired(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_new(ctx : *mut super::types::SSL_CTX) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_new_domain(ctx : *mut super::types::SSL_CTX, flags : u64) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_new_from_listener(ssl : *mut super::types::SSL, flags : u64) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_new_listener(ctx : *mut super::types::SSL_CTX, flags : u64) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_new_listener_from(ssl : *mut super::types::SSL, flags : u64) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_new_session_ticket(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_new_stream(s : *mut super::types::SSL, flags : u64) -> *mut super::types::SSL);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_peek(ssl : *mut super::types::SSL, buf : *mut core::ffi::c_void, num : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_peek_ex(ssl : *mut super::types::SSL, buf : *mut core::ffi::c_void, num : usize, readbytes : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_pending(s : *const super::types::SSL) -> i32);
#[cfg(all(feature = "bio", feature = "types"))]
windows_link::link!("ssl" "C" fn SSL_poll(items : *mut SSL_POLL_ITEM, num_items : usize, stride : usize, timeout : *const bnd_linux::libc::struct_timeval::timeval, flags : u64, result_count : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_read(ssl : *mut super::types::SSL, buf : *mut core::ffi::c_void, num : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_read_early_data(s : *mut super::types::SSL, buf : *mut core::ffi::c_void, num : usize, readbytes : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_read_ex(ssl : *mut super::types::SSL, buf : *mut core::ffi::c_void, num : usize, readbytes : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_renegotiate(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_renegotiate_abbreviated(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_renegotiate_pending(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_rstate_string(s : *const super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_rstate_string_long(s : *const super::types::SSL) -> *const i8);
windows_link::link!("ssl" "C" fn SSL_select_next_proto(out : *mut *mut u8, outlen : *mut u8, r#in : *const u8, inlen : u32, client : *const u8, client_len : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_sendfile(s : *mut super::types::SSL, fd : i32, offset : bnd_linux::libc::types::off_t, size : usize, flags : i32) -> bnd_linux::libc::types::ssize_t);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_session_reused(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set0_CA_list(s : *mut super::types::SSL, name_list : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set0_rbio(s : *mut super::types::SSL, rbio : *mut super::types::BIO));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set0_security_ex_data(s : *mut super::types::SSL, ex : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set0_tmp_dh_pkey(s : *mut super::types::SSL, dhpkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set0_wbio(s : *mut super::types::SSL, wbio : *mut super::types::BIO));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set1_cert_comp_preference(ssl : *mut super::types::SSL, algs : *mut i32, len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set1_client_cert_type(s : *mut super::types::SSL, val : *const u8, len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set1_compressed_cert(ssl : *mut super::types::SSL, algorithm : i32, comp_data : *mut u8, comp_length : usize, orig_length : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set1_host(s : *mut super::types::SSL, host : *const i8) -> i32);
#[cfg(all(feature = "bio", feature = "types"))]
windows_link::link!("ssl" "C" fn SSL_set1_initial_peer_addr(s : *mut super::types::SSL, peer_addr : *const super::bio::BIO_ADDR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set1_param(ssl : *mut super::types::SSL, vpm : *mut super::types::X509_VERIFY_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set1_server_cert_type(s : *mut super::types::SSL, val : *const u8, len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_SSL_CTX(ssl : *mut super::types::SSL, ctx : *mut super::types::SSL_CTX) -> *mut super::types::SSL_CTX);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_accept_state(s : *mut super::types::SSL));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_allow_early_data_cb(s : *mut super::types::SSL, cb : SSL_allow_early_data_cb_fn, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_alpn_protos(ssl : *mut super::types::SSL, protos : *const u8, protos_len : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_async_callback(s : *mut super::types::SSL, callback : SSL_async_callback_fn) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_async_callback_arg(s : *mut super::types::SSL, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_bio(s : *mut super::types::SSL, rbio : *mut super::types::BIO, wbio : *mut super::types::BIO));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_block_padding(ssl : *mut super::types::SSL, block_size : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_block_padding_ex(ssl : *mut super::types::SSL, app_block_size : usize, hs_block_size : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_blocking_mode(s : *mut super::types::SSL, blocking : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_cert_cb(s : *mut super::types::SSL, cb : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_cipher_list(s : *mut super::types::SSL, str : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_ciphersuites(s : *mut super::types::SSL, str : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_client_CA_list(s : *mut super::types::SSL, name_list : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_connect_state(s : *mut super::types::SSL));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_ct_validation_callback(s : *mut super::types::SSL, callback : ssl_ct_validation_cb, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_debug(s : *mut super::types::SSL, debug : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_default_passwd_cb(s : *mut super::types::SSL, cb : super::types::pem_password_cb));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_default_passwd_cb_userdata(s : *mut super::types::SSL, u : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_default_read_buffer_len(s : *mut super::types::SSL, len : usize));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_default_stream_mode(s : *mut super::types::SSL, mode : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_ex_data(ssl : *mut super::types::SSL, idx : i32, data : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_fd(s : *mut super::types::SSL, fd : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_generate_session_id(s : *mut super::types::SSL, cb : GEN_SESSION_CB) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_hostflags(s : *mut super::types::SSL, flags : u32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_incoming_stream_policy(s : *mut super::types::SSL, policy : i32, aec : u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_info_callback(ssl : *mut super::types::SSL, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_max_early_data(s : *mut super::types::SSL, max_early_data : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_msg_callback(ssl : *mut super::types::SSL, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_not_resumable_session_callback(ssl : *mut super::types::SSL, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_num_tickets(s : *mut super::types::SSL, num_tickets : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_options(s : *mut super::types::SSL, op : u64) -> u64);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_post_handshake_auth(s : *mut super::types::SSL, val : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_psk_client_callback(ssl : *mut super::types::SSL, cb : SSL_psk_client_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_psk_find_session_callback(s : *mut super::types::SSL, cb : SSL_psk_find_session_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_psk_server_callback(ssl : *mut super::types::SSL, cb : SSL_psk_server_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_psk_use_session_callback(s : *mut super::types::SSL, cb : SSL_psk_use_session_cb_func));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_purpose(ssl : *mut super::types::SSL, purpose : i32) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("ssl" "C" fn SSL_set_quic_tls_cbs(s : *mut super::types::SSL, qtdis : *const super::types::OSSL_DISPATCH, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_quic_tls_early_data_enabled(s : *mut super::types::SSL, enabled : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_quic_tls_transport_params(s : *mut super::types::SSL, params : *const u8, params_len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_quiet_shutdown(ssl : *mut super::types::SSL, mode : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_read_ahead(s : *mut super::types::SSL, yes : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_record_padding_callback(ssl : *mut super::types::SSL, cb : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_record_padding_callback_arg(ssl : *mut super::types::SSL, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_recv_max_early_data(s : *mut super::types::SSL, recv_max_early_data : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_rfd(s : *mut super::types::SSL, fd : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_security_callback(s : *mut super::types::SSL, cb : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_security_level(s : *mut super::types::SSL, level : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_session(to : *mut super::types::SSL, session : *mut SSL_SESSION) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_session_id_context(ssl : *mut super::types::SSL, sid_ctx : *const u8, sid_ctx_len : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_session_secret_cb(s : *mut super::types::SSL, session_secret_cb : tls_session_secret_cb_fn, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_session_ticket_ext(s : *mut super::types::SSL, ext_data : *mut core::ffi::c_void, ext_len : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_session_ticket_ext_cb(s : *mut super::types::SSL, cb : tls_session_ticket_ext_cb_fn, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_shutdown(ssl : *mut super::types::SSL, mode : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_srp_server_param(s : *mut super::types::SSL, n : *const super::types::BIGNUM, g : *const super::types::BIGNUM, sa : *mut super::types::BIGNUM, v : *mut super::types::BIGNUM, info : *mut i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_srp_server_param_pw(s : *mut super::types::SSL, user : *const i8, pass : *const i8, grp : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_ssl_method(s : *mut super::types::SSL, method : *const SSL_METHOD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_tmp_dh_callback(ssl : *mut super::types::SSL, dh : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_trust(ssl : *mut super::types::SSL, trust : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_value_uint(s : *mut super::types::SSL, class_ : u32, id : u32, v : u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_verify(s : *mut super::types::SSL, mode : i32, callback : SSL_verify_cb));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_verify_depth(s : *mut super::types::SSL, depth : i32));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_verify_result(ssl : *mut super::types::SSL, v : i64));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_wfd(s : *mut super::types::SSL, fd : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_shutdown(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_shutdown_ex(ssl : *mut super::types::SSL, flags : u64, args : *const SSL_SHUTDOWN_EX_ARGS, args_len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_srp_server_param_with_username(s : *mut super::types::SSL, ad : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_state_string(s : *const super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_state_string_long(s : *const super::types::SSL) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_stateless(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_stream_conclude(ssl : *mut super::types::SSL, flags : u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_stream_reset(ssl : *mut super::types::SSL, args : *const SSL_STREAM_RESET_ARGS, args_len : usize) -> i32);
windows_link::link!("ssl" "C" fn SSL_test_functions() -> *const openssl_ssl_test_functions);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_trace(write_p : i32, version : i32, content_type : i32, buf : *const core::ffi::c_void, len : usize, ssl : *mut super::types::SSL, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_up_ref(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_PrivateKey(ssl : *mut super::types::SSL, pkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_PrivateKey_ASN1(pk : i32, ssl : *mut super::types::SSL, d : *const u8, len : i64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_PrivateKey_file(ssl : *mut super::types::SSL, file : *const i8, r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_RSAPrivateKey(ssl : *mut super::types::SSL, rsa : *mut super::types::RSA) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_RSAPrivateKey_ASN1(ssl : *mut super::types::SSL, d : *const u8, len : i64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_RSAPrivateKey_file(ssl : *mut super::types::SSL, file : *const i8, r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_cert_and_key(ssl : *mut super::types::SSL, x509 : *mut super::types::X509, privatekey : *mut super::types::EVP_PKEY, chain : *mut core::ffi::c_void, r#override : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_certificate(ssl : *mut super::types::SSL, x : *mut super::types::X509) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_certificate_ASN1(ssl : *mut super::types::SSL, d : *const u8, len : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_certificate_chain_file(ssl : *mut super::types::SSL, file : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_certificate_file(ssl : *mut super::types::SSL, file : *const i8, r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_use_psk_identity_hint(s : *mut super::types::SSL, identity_hint : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_verify_client_post_handshake(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_version(ssl : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_waiting_for_async(s : *mut super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_want(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_write(ssl : *mut super::types::SSL, buf : *const core::ffi::c_void, num : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_write_early_data(s : *mut super::types::SSL, buf : *const core::ffi::c_void, num : usize, written : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_write_ex(s : *mut super::types::SSL, buf : *const core::ffi::c_void, num : usize, written : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_write_ex2(s : *mut super::types::SSL, buf : *const core::ffi::c_void, num : usize, flags : u64, written : *mut usize) -> i32);
windows_link::link!("ssl" "C" fn TLS_client_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLS_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLS_server_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_1_client_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_1_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_1_server_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_2_client_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_2_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_2_server_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_client_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn TLSv1_server_method() -> *const SSL_METHOD);
windows_link::link!("ssl" "C" fn d2i_SSL_SESSION(a : *mut *mut SSL_SESSION, pp : *mut *mut u8, length : i64) -> *mut SSL_SESSION);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn d2i_SSL_SESSION_ex(a : *mut *mut SSL_SESSION, pp : *mut *mut u8, length : i64, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> *mut SSL_SESSION);
windows_link::link!("ssl" "C" fn i2d_SSL_SESSION(r#in : *const SSL_SESSION, pp : *mut *mut u8) -> i32);
pub const CERT_PKEY_CA_PARAM: i32 = 128;
pub const CERT_PKEY_CA_SIGNATURE: i32 = 32;
pub const CERT_PKEY_CERT_TYPE: i32 = 1024;
pub const CERT_PKEY_EE_PARAM: i32 = 64;
pub const CERT_PKEY_EE_SIGNATURE: i32 = 16;
pub const CERT_PKEY_EXPLICIT_SIGN: i32 = 256;
pub const CERT_PKEY_ISSUER_NAME: i32 = 512;
pub const CERT_PKEY_RPK: i32 = 4096;
pub const CERT_PKEY_SIGN: i32 = 2;
pub const CERT_PKEY_SUITEB: i32 = 2048;
pub const CERT_PKEY_VALID: i32 = 1;
pub const DTLS_CTRL_GET_LINK_MIN_MTU: i32 = 121;
pub const DTLS_CTRL_GET_TIMEOUT: i32 = 73;
pub const DTLS_CTRL_HANDLE_TIMEOUT: i32 = 74;
pub const DTLS_CTRL_SET_LINK_MTU: i32 = 120;
pub const DTLS_ST_CR_HELLO_VERIFY_REQUEST: OSSL_HANDSHAKE_STATE = 2;
pub const DTLS_ST_SW_HELLO_VERIFY_REQUEST: OSSL_HANDSHAKE_STATE = 23;
#[cfg(feature = "types")]
pub type DTLS_timer_cb =
    Option<unsafe extern "C" fn(s: *mut super::types::SSL, timer_us: u32) -> u32>;
#[cfg(feature = "types")]
pub type GEN_SESSION_CB =
    Option<unsafe extern "C" fn(ssl: *mut super::types::SSL, id: *mut u8, id_len: *mut u32) -> i32>;
pub const OPENSSL_INIT_LOAD_SSL_STRINGS: i32 = 2097152;
pub const OPENSSL_INIT_NO_LOAD_SSL_STRINGS: i32 = 1048576;
pub const OPENSSL_INIT_SSL_DEFAULT: i64 = 2097154;
pub const OPENSSL_NPN_NEGOTIATED: i32 = 1;
pub const OPENSSL_NPN_NO_OVERLAP: i32 = 2;
pub const OPENSSL_NPN_UNSUPPORTED: i32 = 0;
pub type OSSL_HANDSHAKE_STATE = u32;
pub const OSSL_RECORD_PROTECTION_LEVEL_APPLICATION: i32 = 3;
pub const OSSL_RECORD_PROTECTION_LEVEL_EARLY: i32 = 1;
pub const OSSL_RECORD_PROTECTION_LEVEL_HANDSHAKE: i32 = 2;
pub const OSSL_RECORD_PROTECTION_LEVEL_NONE: i32 = 0;
pub const PSK_MAX_IDENTITY_LEN: i32 = 256;
pub const PSK_MAX_PSK_LEN: i32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SRTP_PROTECTION_PROFILE {
    pub name: *const i8,
    pub id: u64,
}
pub const SSL_ACCEPT_CONNECTION_NO_BLOCK: u64 = 1;
pub const SSL_ACCEPT_STREAM_NO_BLOCK: u32 = 1;
pub const SSL_AD_ACCESS_DENIED: i32 = 49;
pub const SSL_AD_BAD_CERTIFICATE: i32 = 42;
pub const SSL_AD_BAD_CERTIFICATE_HASH_VALUE: i32 = 114;
pub const SSL_AD_BAD_CERTIFICATE_STATUS_RESPONSE: i32 = 113;
pub const SSL_AD_BAD_RECORD_MAC: i32 = 20;
pub const SSL_AD_CERTIFICATE_EXPIRED: i32 = 45;
pub const SSL_AD_CERTIFICATE_REQUIRED: i32 = 116;
pub const SSL_AD_CERTIFICATE_REVOKED: i32 = 44;
pub const SSL_AD_CERTIFICATE_UNKNOWN: i32 = 46;
pub const SSL_AD_CERTIFICATE_UNOBTAINABLE: i32 = 111;
pub const SSL_AD_CLOSE_NOTIFY: i32 = 0;
pub const SSL_AD_DECODE_ERROR: i32 = 50;
pub const SSL_AD_DECOMPRESSION_FAILURE: i32 = 30;
pub const SSL_AD_DECRYPTION_FAILED: i32 = 21;
pub const SSL_AD_DECRYPT_ERROR: i32 = 51;
pub const SSL_AD_EXPORT_RESTRICTION: i32 = 60;
pub const SSL_AD_HANDSHAKE_FAILURE: i32 = 40;
pub const SSL_AD_ILLEGAL_PARAMETER: i32 = 47;
pub const SSL_AD_INAPPROPRIATE_FALLBACK: i32 = 86;
pub const SSL_AD_INSUFFICIENT_SECURITY: i32 = 71;
pub const SSL_AD_INTERNAL_ERROR: i32 = 80;
pub const SSL_AD_MISSING_EXTENSION: i32 = 109;
pub const SSL_AD_NO_APPLICATION_PROTOCOL: i32 = 120;
pub const SSL_AD_NO_CERTIFICATE: i32 = 41;
pub const SSL_AD_NO_RENEGOTIATION: i32 = 100;
pub const SSL_AD_PROTOCOL_VERSION: i32 = 70;
pub const SSL_AD_REASON_OFFSET: i32 = 1000;
pub const SSL_AD_RECORD_OVERFLOW: i32 = 22;
pub const SSL_AD_UNEXPECTED_MESSAGE: i32 = 10;
pub const SSL_AD_UNKNOWN_CA: i32 = 48;
pub const SSL_AD_UNKNOWN_PSK_IDENTITY: i32 = 115;
pub const SSL_AD_UNRECOGNIZED_NAME: i32 = 112;
pub const SSL_AD_UNSUPPORTED_CERTIFICATE: i32 = 43;
pub const SSL_AD_UNSUPPORTED_EXTENSION: i32 = 110;
pub const SSL_AD_USER_CANCELLED: i32 = 90;
pub const SSL_ASYNC_NO_JOBS: i32 = 6;
pub const SSL_ASYNC_PAUSED: i32 = 5;
pub const SSL_BUILD_CHAIN_FLAG_CHECK: i32 = 4;
pub const SSL_BUILD_CHAIN_FLAG_CLEAR_ERROR: i32 = 16;
pub const SSL_BUILD_CHAIN_FLAG_IGNORE_ERROR: i32 = 8;
pub const SSL_BUILD_CHAIN_FLAG_NO_ROOT: i32 = 2;
pub const SSL_BUILD_CHAIN_FLAG_UNTRUSTED: i32 = 1;
pub const SSL_CB_ACCEPT_EXIT: i32 = 8194;
pub const SSL_CB_ACCEPT_LOOP: i32 = 8193;
pub const SSL_CB_ALERT: i32 = 16384;
pub const SSL_CB_CONNECT_EXIT: i32 = 4098;
pub const SSL_CB_CONNECT_LOOP: i32 = 4097;
pub const SSL_CB_EXIT: i32 = 2;
pub const SSL_CB_HANDSHAKE_DONE: i32 = 32;
pub const SSL_CB_HANDSHAKE_START: i32 = 16;
pub const SSL_CB_LOOP: i32 = 1;
pub const SSL_CB_READ: i32 = 4;
pub const SSL_CB_READ_ALERT: i32 = 16388;
pub const SSL_CB_WRITE: i32 = 8;
pub const SSL_CB_WRITE_ALERT: i32 = 16392;
pub const SSL_CERT_FLAG_BROKEN_PROTOCOL: i32 = 268435456;
pub const SSL_CERT_FLAG_SUITEB_128_LOS: i32 = 196608;
pub const SSL_CERT_FLAG_SUITEB_128_LOS_ONLY: i32 = 65536;
pub const SSL_CERT_FLAG_SUITEB_192_LOS: i32 = 131072;
pub const SSL_CERT_FLAG_TLS_STRICT: u32 = 1;
pub const SSL_CERT_SET_FIRST: i32 = 1;
pub const SSL_CERT_SET_NEXT: i32 = 2;
pub const SSL_CERT_SET_SERVER: i32 = 3;
pub type SSL_CIPHER = ssl_cipher_st;
pub const SSL_CLIENT_HELLO_CB: i32 = 7;
pub const SSL_CLIENT_HELLO_ERROR: i32 = 0;
pub const SSL_CLIENT_HELLO_RETRY: i32 = -1;
pub const SSL_CLIENT_HELLO_SUCCESS: i32 = 1;
pub type SSL_CONF_CTX = ssl_conf_ctx_st;
pub const SSL_CONF_FLAG_CERTIFICATE: i32 = 32;
pub const SSL_CONF_FLAG_CLIENT: i32 = 4;
pub const SSL_CONF_FLAG_CMDLINE: i32 = 1;
pub const SSL_CONF_FLAG_FILE: i32 = 2;
pub const SSL_CONF_FLAG_REQUIRE_PRIVATE: i32 = 64;
pub const SSL_CONF_FLAG_SERVER: i32 = 8;
pub const SSL_CONF_FLAG_SHOW_ERRORS: i32 = 16;
pub const SSL_CONF_TYPE_DIR: i32 = 3;
pub const SSL_CONF_TYPE_FILE: i32 = 2;
pub const SSL_CONF_TYPE_NONE: i32 = 4;
pub const SSL_CONF_TYPE_STORE: i32 = 5;
pub const SSL_CONF_TYPE_STRING: i32 = 1;
pub const SSL_CONF_TYPE_UNKNOWN: i32 = 0;
pub const SSL_CONN_CLOSE_FLAG_LOCAL: u32 = 1;
pub const SSL_CONN_CLOSE_FLAG_TRANSPORT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SSL_CONN_CLOSE_INFO {
    pub error_code: u64,
    pub frame_type: u64,
    pub reason: *const i8,
    pub reason_len: usize,
    pub flags: u32,
}
pub const SSL_COOKIE_LENGTH: i32 = 4096;
pub const SSL_CTRL_BUILD_CERT_CHAIN: i32 = 105;
pub const SSL_CTRL_CERT_FLAGS: i32 = 99;
pub const SSL_CTRL_CHAIN: i32 = 88;
pub const SSL_CTRL_CHAIN_CERT: i32 = 89;
pub const SSL_CTRL_CLEAR_CERT_FLAGS: i32 = 100;
pub const SSL_CTRL_CLEAR_EXTRA_CHAIN_CERTS: i32 = 83;
pub const SSL_CTRL_CLEAR_MODE: i32 = 78;
pub const SSL_CTRL_CLEAR_NUM_RENEGOTIATIONS: i32 = 11;
pub const SSL_CTRL_EXTRA_CHAIN_CERT: i32 = 14;
pub const SSL_CTRL_GET0_IMPLEMENTED_GROUPS: i32 = 139;
pub const SSL_CTRL_GET_CHAIN_CERTS: i32 = 115;
pub const SSL_CTRL_GET_CHAIN_CERT_STORE: i32 = 138;
pub const SSL_CTRL_GET_CLIENT_CERT_REQUEST: i32 = 9;
pub const SSL_CTRL_GET_CLIENT_CERT_TYPES: i32 = 103;
pub const SSL_CTRL_GET_CURVES: i32 = 90;
pub const SSL_CTRL_GET_EC_POINT_FORMATS: i32 = 111;
pub const SSL_CTRL_GET_EXTMS_SUPPORT: i32 = 122;
pub const SSL_CTRL_GET_EXTRA_CHAIN_CERTS: i32 = 82;
pub const SSL_CTRL_GET_FLAGS: i32 = 13;
pub const SSL_CTRL_GET_GROUPS: i32 = 90;
pub const SSL_CTRL_GET_IANA_GROUPS: i32 = 135;
pub const SSL_CTRL_GET_MAX_CERT_LIST: i32 = 50;
pub const SSL_CTRL_GET_MAX_PROTO_VERSION: i32 = 131;
pub const SSL_CTRL_GET_MIN_PROTO_VERSION: i32 = 130;
pub const SSL_CTRL_GET_NEGOTIATED_GROUP: i32 = 134;
pub const SSL_CTRL_GET_NUM_RENEGOTIATIONS: i32 = 10;
pub const SSL_CTRL_GET_PEER_SIGNATURE_NAME: i32 = 141;
pub const SSL_CTRL_GET_PEER_SIGNATURE_NID: i32 = 108;
pub const SSL_CTRL_GET_PEER_TMP_KEY: i32 = 109;
pub const SSL_CTRL_GET_RAW_CIPHERLIST: i32 = 110;
pub const SSL_CTRL_GET_READ_AHEAD: i32 = 40;
pub const SSL_CTRL_GET_RI_SUPPORT: i32 = 76;
pub const SSL_CTRL_GET_SERVER_TMP_KEY: i32 = 109;
pub const SSL_CTRL_GET_SESS_CACHE_MODE: i32 = 45;
pub const SSL_CTRL_GET_SESS_CACHE_SIZE: i32 = 43;
pub const SSL_CTRL_GET_SHARED_CURVE: i32 = 93;
pub const SSL_CTRL_GET_SHARED_GROUP: i32 = 93;
pub const SSL_CTRL_GET_SIGNATURE_NAME: i32 = 140;
pub const SSL_CTRL_GET_SIGNATURE_NID: i32 = 132;
pub const SSL_CTRL_GET_TLSEXT_STATUS_REQ_CB: i32 = 128;
pub const SSL_CTRL_GET_TLSEXT_STATUS_REQ_CB_ARG: i32 = 129;
pub const SSL_CTRL_GET_TLSEXT_STATUS_REQ_EXTS: i32 = 66;
pub const SSL_CTRL_GET_TLSEXT_STATUS_REQ_IDS: i32 = 68;
pub const SSL_CTRL_GET_TLSEXT_STATUS_REQ_OCSP_RESP: i32 = 70;
pub const SSL_CTRL_GET_TLSEXT_STATUS_REQ_TYPE: i32 = 127;
pub const SSL_CTRL_GET_TLSEXT_TICKET_KEYS: i32 = 58;
pub const SSL_CTRL_GET_TMP_KEY: i32 = 133;
pub const SSL_CTRL_GET_TOTAL_RENEGOTIATIONS: i32 = 12;
pub const SSL_CTRL_GET_VERIFY_CERT_STORE: i32 = 137;
pub const SSL_CTRL_MODE: i32 = 33;
pub const SSL_CTRL_SELECT_CURRENT_CERT: i32 = 116;
pub const SSL_CTRL_SESS_ACCEPT: i32 = 24;
pub const SSL_CTRL_SESS_ACCEPT_GOOD: i32 = 25;
pub const SSL_CTRL_SESS_ACCEPT_RENEGOTIATE: i32 = 26;
pub const SSL_CTRL_SESS_CACHE_FULL: i32 = 31;
pub const SSL_CTRL_SESS_CB_HIT: i32 = 28;
pub const SSL_CTRL_SESS_CONNECT: i32 = 21;
pub const SSL_CTRL_SESS_CONNECT_GOOD: i32 = 22;
pub const SSL_CTRL_SESS_CONNECT_RENEGOTIATE: i32 = 23;
pub const SSL_CTRL_SESS_HIT: i32 = 27;
pub const SSL_CTRL_SESS_MISSES: i32 = 29;
pub const SSL_CTRL_SESS_NUMBER: i32 = 20;
pub const SSL_CTRL_SESS_TIMEOUTS: i32 = 30;
pub const SSL_CTRL_SET_CHAIN_CERT_STORE: i32 = 107;
pub const SSL_CTRL_SET_CLIENT_CERT_TYPES: i32 = 104;
pub const SSL_CTRL_SET_CLIENT_SIGALGS: i32 = 101;
pub const SSL_CTRL_SET_CLIENT_SIGALGS_LIST: i32 = 102;
pub const SSL_CTRL_SET_CURRENT_CERT: i32 = 117;
pub const SSL_CTRL_SET_CURVES: i32 = 91;
pub const SSL_CTRL_SET_CURVES_LIST: i32 = 92;
pub const SSL_CTRL_SET_DH_AUTO: i32 = 118;
pub const SSL_CTRL_SET_GROUPS: i32 = 91;
pub const SSL_CTRL_SET_GROUPS_LIST: i32 = 92;
pub const SSL_CTRL_SET_MAX_CERT_LIST: i32 = 51;
pub const SSL_CTRL_SET_MAX_PIPELINES: i32 = 126;
pub const SSL_CTRL_SET_MAX_PROTO_VERSION: i32 = 124;
pub const SSL_CTRL_SET_MAX_SEND_FRAGMENT: i32 = 52;
pub const SSL_CTRL_SET_MIN_PROTO_VERSION: i32 = 123;
pub const SSL_CTRL_SET_MSG_CALLBACK: i32 = 15;
pub const SSL_CTRL_SET_MSG_CALLBACK_ARG: i32 = 16;
pub const SSL_CTRL_SET_MTU: i32 = 17;
pub const SSL_CTRL_SET_NOT_RESUMABLE_SESS_CB: i32 = 79;
pub const SSL_CTRL_SET_READ_AHEAD: i32 = 41;
pub const SSL_CTRL_SET_RETRY_VERIFY: i32 = 136;
pub const SSL_CTRL_SET_SESS_CACHE_MODE: i32 = 44;
pub const SSL_CTRL_SET_SESS_CACHE_SIZE: i32 = 42;
pub const SSL_CTRL_SET_SIGALGS: i32 = 97;
pub const SSL_CTRL_SET_SIGALGS_LIST: i32 = 98;
pub const SSL_CTRL_SET_SPLIT_SEND_FRAGMENT: i32 = 125;
pub const SSL_CTRL_SET_SRP_ARG: i32 = 78;
pub const SSL_CTRL_SET_SRP_GIVE_CLIENT_PWD_CB: i32 = 77;
pub const SSL_CTRL_SET_SRP_VERIFY_PARAM_CB: i32 = 76;
pub const SSL_CTRL_SET_TLSEXT_DEBUG_ARG: i32 = 57;
pub const SSL_CTRL_SET_TLSEXT_DEBUG_CB: i32 = 56;
pub const SSL_CTRL_SET_TLSEXT_HOSTNAME: i32 = 55;
pub const SSL_CTRL_SET_TLSEXT_SERVERNAME_ARG: i32 = 54;
pub const SSL_CTRL_SET_TLSEXT_SERVERNAME_CB: i32 = 53;
pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB: i32 = 63;
pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB_ARG: i32 = 64;
pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_EXTS: i32 = 67;
pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_IDS: i32 = 69;
pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_OCSP_RESP: i32 = 71;
pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_TYPE: i32 = 65;
pub const SSL_CTRL_SET_TLSEXT_TICKET_KEYS: i32 = 59;
pub const SSL_CTRL_SET_TLSEXT_TICKET_KEY_CB: i32 = 72;
pub const SSL_CTRL_SET_TLS_EXT_SRP_PASSWORD: i32 = 81;
pub const SSL_CTRL_SET_TLS_EXT_SRP_STRENGTH: i32 = 80;
pub const SSL_CTRL_SET_TLS_EXT_SRP_USERNAME: i32 = 79;
pub const SSL_CTRL_SET_TLS_EXT_SRP_USERNAME_CB: i32 = 75;
pub const SSL_CTRL_SET_TMP_DH: i32 = 3;
pub const SSL_CTRL_SET_TMP_DH_CB: i32 = 6;
pub const SSL_CTRL_SET_TMP_ECDH: i32 = 4;
pub const SSL_CTRL_SET_VERIFY_CERT_STORE: i32 = 106;
#[cfg(feature = "types")]
pub type SSL_CTX_alpn_select_cb_func = Option<
    unsafe extern "C" fn(
        ssl: *mut super::types::SSL,
        out: *mut *mut u8,
        outlen: *mut u8,
        r#in: *const u8,
        inlen: u32,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_CTX_decrypt_session_ticket_fn = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        ss: *mut SSL_SESSION,
        keyname: *const u8,
        keyname_length: usize,
        status: SSL_TICKET_STATUS,
        arg: *mut core::ffi::c_void,
    ) -> SSL_TICKET_RETURN,
>;
#[cfg(feature = "types")]
pub type SSL_CTX_generate_session_ticket_fn =
    Option<unsafe extern "C" fn(s: *mut super::types::SSL, arg: *mut core::ffi::c_void) -> i32>;
#[cfg(feature = "types")]
pub type SSL_CTX_keylog_cb_func =
    Option<unsafe extern "C" fn(ssl: *const super::types::SSL, line: *const i8)>;
#[cfg(feature = "types")]
pub type SSL_CTX_npn_advertised_cb_func = Option<
    unsafe extern "C" fn(
        ssl: *mut super::types::SSL,
        out: *mut *mut u8,
        outlen: *mut u32,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_CTX_npn_select_cb_func = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        out: *mut *mut u8,
        outlen: *mut u8,
        r#in: *const u8,
        inlen: u32,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
pub const SSL_CT_VALIDATION_PERMISSIVE: u32 = 0;
pub const SSL_CT_VALIDATION_STRICT: u32 = 1;
pub const SSL_DEFAULT_CIPHER_LIST: *const u8 = [
    65, 76, 76, 58, 33, 67, 79, 77, 80, 76, 69, 77, 69, 78, 84, 79, 70, 68, 69, 70, 65, 85, 76, 84,
    58, 33, 101, 78, 85, 76, 76, 0,
]
.as_ptr();
pub const SSL_DEFAULT_STREAM_MODE_AUTO_BIDI: i32 = 1;
pub const SSL_DEFAULT_STREAM_MODE_AUTO_UNI: i32 = 2;
pub const SSL_DEFAULT_STREAM_MODE_NONE: i32 = 0;
pub const SSL_DOMAIN_FLAG_BLOCKING: u32 = 8;
pub const SSL_DOMAIN_FLAG_LEGACY_BLOCKING: u32 = 16;
pub const SSL_DOMAIN_FLAG_MULTI_THREAD: u32 = 2;
pub const SSL_DOMAIN_FLAG_SINGLE_THREAD: u32 = 1;
pub const SSL_DOMAIN_FLAG_THREAD_ASSISTED: u32 = 4;
pub const SSL_EARLY_DATA_ACCEPTED: i32 = 2;
pub const SSL_EARLY_DATA_NOT_SENT: i32 = 0;
pub const SSL_EARLY_DATA_REJECTED: i32 = 1;
pub const SSL_ERROR_NONE: i32 = 0;
pub const SSL_ERROR_SSL: i32 = 1;
pub const SSL_ERROR_SYSCALL: i32 = 5;
pub const SSL_ERROR_WANT_ACCEPT: i32 = 8;
pub const SSL_ERROR_WANT_ASYNC: i32 = 9;
pub const SSL_ERROR_WANT_ASYNC_JOB: i32 = 10;
pub const SSL_ERROR_WANT_CLIENT_HELLO_CB: i32 = 11;
pub const SSL_ERROR_WANT_CONNECT: i32 = 7;
pub const SSL_ERROR_WANT_READ: i32 = 2;
pub const SSL_ERROR_WANT_RETRY_VERIFY: i32 = 12;
pub const SSL_ERROR_WANT_WRITE: i32 = 3;
pub const SSL_ERROR_WANT_X509_LOOKUP: i32 = 4;
pub const SSL_ERROR_ZERO_RETURN: i32 = 6;
pub const SSL_EXT_CLIENT_HELLO: i32 = 128;
pub const SSL_EXT_DTLS_ONLY: i32 = 2;
pub const SSL_EXT_IGNORE_ON_RESUMPTION: i32 = 64;
pub const SSL_EXT_SSL3_ALLOWED: i32 = 8;
pub const SSL_EXT_TLS1_2_AND_BELOW_ONLY: i32 = 16;
pub const SSL_EXT_TLS1_2_SERVER_HELLO: i32 = 256;
pub const SSL_EXT_TLS1_3_CERTIFICATE: i32 = 4096;
pub const SSL_EXT_TLS1_3_CERTIFICATE_COMPRESSION: i32 = 32768;
pub const SSL_EXT_TLS1_3_CERTIFICATE_REQUEST: i32 = 16384;
pub const SSL_EXT_TLS1_3_ENCRYPTED_EXTENSIONS: i32 = 1024;
pub const SSL_EXT_TLS1_3_HELLO_RETRY_REQUEST: i32 = 2048;
pub const SSL_EXT_TLS1_3_NEW_SESSION_TICKET: i32 = 8192;
pub const SSL_EXT_TLS1_3_ONLY: i32 = 32;
pub const SSL_EXT_TLS1_3_RAW_PUBLIC_KEY: i32 = 65536;
pub const SSL_EXT_TLS1_3_SERVER_HELLO: i32 = 512;
pub const SSL_EXT_TLS_IMPLEMENTATION_ONLY: i32 = 4;
pub const SSL_EXT_TLS_ONLY: i32 = 1;
pub const SSL_FILETYPE_ASN1: i32 = 2;
pub const SSL_FILETYPE_PEM: i32 = 1;
pub const SSL_INCOMING_STREAM_POLICY_ACCEPT: i32 = 1;
pub const SSL_INCOMING_STREAM_POLICY_AUTO: i32 = 0;
pub const SSL_INCOMING_STREAM_POLICY_REJECT: i32 = 2;
pub const SSL_KEY_UPDATE_NONE: i32 = -1;
pub const SSL_KEY_UPDATE_NOT_REQUESTED: i32 = 0;
pub const SSL_KEY_UPDATE_REQUESTED: i32 = 1;
pub const SSL_LISTENER_FLAG_NO_VALIDATE: u64 = 2;
pub const SSL_MAC_FLAG_READ_MAC_STREAM: i32 = 1;
pub const SSL_MAC_FLAG_READ_MAC_TLSTREE: i32 = 4;
pub const SSL_MAC_FLAG_WRITE_MAC_STREAM: i32 = 2;
pub const SSL_MAC_FLAG_WRITE_MAC_TLSTREE: i32 = 8;
pub const SSL_MAX_CERT_LIST_DEFAULT: i32 = 102400;
pub const SSL_MAX_KEY_ARG_LENGTH: i32 = 8;
pub const SSL_MAX_PIPELINES: i32 = 32;
pub const SSL_MAX_SID_CTX_LENGTH: i32 = 32;
pub const SSL_MAX_SSL_SESSION_ID_LENGTH: i32 = 32;
pub type SSL_METHOD = ssl_method_st;
pub const SSL_MIN_RSA_MODULUS_LENGTH_IN_BYTES: i32 = 64;
pub const SSL_MODE_ACCEPT_MOVING_WRITE_BUFFER: u32 = 2;
pub const SSL_MODE_ASYNC: u32 = 256;
pub const SSL_MODE_AUTO_RETRY: u32 = 4;
pub const SSL_MODE_DTLS_SCTP_LABEL_LENGTH_BUG: u32 = 1024;
pub const SSL_MODE_ENABLE_PARTIAL_WRITE: u32 = 1;
pub const SSL_MODE_NO_AUTO_CHAIN: u32 = 8;
pub const SSL_MODE_RELEASE_BUFFERS: u32 = 16;
pub const SSL_MODE_SEND_CLIENTHELLO_TIME: u32 = 32;
pub const SSL_MODE_SEND_FALLBACK_SCSV: u32 = 128;
pub const SSL_MODE_SEND_SERVERHELLO_TIME: u32 = 64;
pub const SSL_NOTHING: i32 = 1;
pub const SSL_OP_ALL: u64 = 2147485776;
pub const SSL_OP_ALLOW_CLIENT_RENEGOTIATION: u64 = 256;
pub const SSL_OP_ALLOW_NO_DHE_KEX: u64 = 1024;
pub const SSL_OP_ALLOW_UNSAFE_LEGACY_RENEGOTIATION: u64 = 262144;
pub const SSL_OP_CIPHER_SERVER_PREFERENCE: u64 = 4194304;
pub const SSL_OP_CISCO_ANYCONNECT: u64 = 32768;
pub const SSL_OP_CLEANSE_PLAINTEXT: u64 = 2;
pub const SSL_OP_COOKIE_EXCHANGE: u64 = 8192;
pub const SSL_OP_CRYPTOPRO_TLSEXT_BUG: u64 = 2147483648;
pub const SSL_OP_DISABLE_TLSEXT_CA_NAMES: u64 = 512;
pub const SSL_OP_DONT_INSERT_EMPTY_FRAGMENTS: u64 = 2048;
pub const SSL_OP_ENABLE_KTLS: u64 = 8;
pub const SSL_OP_ENABLE_KTLS_TX_ZEROCOPY_SENDFILE: u64 = 17179869184;
pub const SSL_OP_ENABLE_MIDDLEBOX_COMPAT: u64 = 1048576;
pub const SSL_OP_EPHEMERAL_RSA: i32 = 0;
pub const SSL_OP_IGNORE_UNEXPECTED_EOF: u64 = 128;
pub const SSL_OP_LEGACY_SERVER_CONNECT: u64 = 4;
pub const SSL_OP_MICROSOFT_BIG_SSLV3_BUFFER: i32 = 0;
pub const SSL_OP_MICROSOFT_SESS_ID_BUG: i32 = 0;
pub const SSL_OP_MSIE_SSLV2_RSA_PADDING: i32 = 0;
pub const SSL_OP_NETSCAPE_CA_DN_BUG: i32 = 0;
pub const SSL_OP_NETSCAPE_CHALLENGE_BUG: i32 = 0;
pub const SSL_OP_NETSCAPE_DEMO_CIPHER_CHANGE_BUG: i32 = 0;
pub const SSL_OP_NETSCAPE_REUSE_CIPHER_CHANGE_BUG: i32 = 0;
pub const SSL_OP_NO_ANTI_REPLAY: u64 = 16777216;
pub const SSL_OP_NO_COMPRESSION: u64 = 131072;
pub const SSL_OP_NO_DTLS_MASK: u64 = 201326592;
pub const SSL_OP_NO_DTLSv1: u64 = 67108864;
pub const SSL_OP_NO_DTLSv1_2: u64 = 134217728;
pub const SSL_OP_NO_ENCRYPT_THEN_MAC: u64 = 524288;
pub const SSL_OP_NO_EXTENDED_MASTER_SECRET: u64 = 1;
pub const SSL_OP_NO_QUERY_MTU: u64 = 4096;
pub const SSL_OP_NO_RENEGOTIATION: u64 = 1073741824;
pub const SSL_OP_NO_RX_CERTIFICATE_COMPRESSION: u64 = 8589934592;
pub const SSL_OP_NO_SESSION_RESUMPTION_ON_RENEGOTIATION: u64 = 65536;
pub const SSL_OP_NO_SSL_MASK: u64 = 1040187392;
pub const SSL_OP_NO_SSLv2: i32 = 0;
pub const SSL_OP_NO_SSLv3: u64 = 33554432;
pub const SSL_OP_NO_TICKET: u64 = 16384;
pub const SSL_OP_NO_TLSv1: u64 = 67108864;
pub const SSL_OP_NO_TLSv1_1: u64 = 268435456;
pub const SSL_OP_NO_TLSv1_2: u64 = 134217728;
pub const SSL_OP_NO_TLSv1_3: u64 = 536870912;
pub const SSL_OP_NO_TX_CERTIFICATE_COMPRESSION: u64 = 4294967296;
pub const SSL_OP_PKCS1_CHECK_1: i32 = 0;
pub const SSL_OP_PKCS1_CHECK_2: i32 = 0;
pub const SSL_OP_PREFER_NO_DHE_KEX: u64 = 34359738368;
pub const SSL_OP_PRIORITIZE_CHACHA: u64 = 2097152;
pub const SSL_OP_SAFARI_ECDHE_ECDSA_BUG: u64 = 64;
pub const SSL_OP_SINGLE_DH_USE: i32 = 0;
pub const SSL_OP_SINGLE_ECDH_USE: i32 = 0;
pub const SSL_OP_SSLEAY_080_CLIENT_DH_BUG: i32 = 0;
pub const SSL_OP_SSLREF2_REUSE_CERT_TYPE_BUG: i32 = 0;
pub const SSL_OP_TLSEXT_PADDING: u64 = 16;
pub const SSL_OP_TLS_BLOCK_PADDING_BUG: i32 = 0;
pub const SSL_OP_TLS_D5_BUG: i32 = 0;
pub const SSL_OP_TLS_ROLLBACK_BUG: u64 = 8388608;
pub const SSL_POLL_EVENT_E: u32 = 54;
pub const SSL_POLL_EVENT_EC: u32 = 4;
pub const SSL_POLL_EVENT_ECD: u32 = 8;
pub const SSL_POLL_EVENT_EL: u32 = 2;
pub const SSL_POLL_EVENT_ER: u32 = 16;
pub const SSL_POLL_EVENT_EW: u32 = 32;
pub const SSL_POLL_EVENT_F: u32 = 1;
pub const SSL_POLL_EVENT_I: u32 = 1792;
pub const SSL_POLL_EVENT_IC: u32 = 256;
pub const SSL_POLL_EVENT_IS: u32 = 1536;
pub const SSL_POLL_EVENT_ISB: u32 = 512;
pub const SSL_POLL_EVENT_ISE: u32 = 1540;
pub const SSL_POLL_EVENT_ISU: u32 = 1024;
pub const SSL_POLL_EVENT_NONE: i32 = 0;
pub const SSL_POLL_EVENT_OS: u32 = 6144;
pub const SSL_POLL_EVENT_OSB: u32 = 2048;
pub const SSL_POLL_EVENT_OSE: u32 = 6148;
pub const SSL_POLL_EVENT_OSU: u32 = 4096;
pub const SSL_POLL_EVENT_R: u32 = 64;
pub const SSL_POLL_EVENT_RE: u32 = 80;
pub const SSL_POLL_EVENT_RW: u32 = 192;
pub const SSL_POLL_EVENT_RWE: u32 = 240;
pub const SSL_POLL_EVENT_W: u32 = 128;
pub const SSL_POLL_EVENT_WE: u32 = 160;
pub const SSL_POLL_FLAG_NO_HANDLE_EVENTS: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "bio", feature = "types"))]
#[derive(Clone, Copy)]
pub struct SSL_POLL_ITEM {
    pub desc: super::bio::BIO_POLL_DESCRIPTOR,
    pub events: u64,
    pub revents: u64,
}
#[cfg(all(feature = "bio", feature = "types"))]
impl Default for SSL_POLL_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SSL_READING: i32 = 3;
pub const SSL_READ_EARLY_DATA_ERROR: i32 = 0;
pub const SSL_READ_EARLY_DATA_FINISH: i32 = 2;
pub const SSL_READ_EARLY_DATA_SUCCESS: i32 = 1;
pub const SSL_RECEIVED_SHUTDOWN: i32 = 2;
pub const SSL_RETRY_VERIFY: i32 = 8;
pub const SSL_SECOP_CA_KEY: i32 = 393233;
pub const SSL_SECOP_CA_MD: i32 = 393234;
pub const SSL_SECOP_CIPHER_CHECK: i32 = 65539;
pub const SSL_SECOP_CIPHER_SHARED: i32 = 65538;
pub const SSL_SECOP_CIPHER_SUPPORTED: i32 = 65537;
pub const SSL_SECOP_COMPRESSION: i32 = 15;
pub const SSL_SECOP_CURVE_CHECK: i32 = 131078;
pub const SSL_SECOP_CURVE_SHARED: i32 = 131077;
pub const SSL_SECOP_CURVE_SUPPORTED: i32 = 131076;
pub const SSL_SECOP_EE_KEY: i32 = 393232;
pub const SSL_SECOP_OTHER_CERT: i32 = 393216;
pub const SSL_SECOP_OTHER_CIPHER: i32 = 65536;
pub const SSL_SECOP_OTHER_CURVE: i32 = 131072;
pub const SSL_SECOP_OTHER_DH: i32 = 196608;
pub const SSL_SECOP_OTHER_NONE: i32 = 0;
pub const SSL_SECOP_OTHER_PKEY: i32 = 262144;
pub const SSL_SECOP_OTHER_SIGALG: i32 = 327680;
pub const SSL_SECOP_OTHER_TYPE: u32 = 4294901760;
pub const SSL_SECOP_PEER: i32 = 4096;
pub const SSL_SECOP_PEER_CA_KEY: i32 = 397329;
pub const SSL_SECOP_PEER_CA_MD: i32 = 397330;
pub const SSL_SECOP_PEER_EE_KEY: i32 = 397328;
pub const SSL_SECOP_SIGALG_CHECK: i32 = 327693;
pub const SSL_SECOP_SIGALG_MASK: i32 = 327694;
pub const SSL_SECOP_SIGALG_SHARED: i32 = 327692;
pub const SSL_SECOP_SIGALG_SUPPORTED: i32 = 327691;
pub const SSL_SECOP_TICKET: i32 = 10;
pub const SSL_SECOP_TMP_DH: i32 = 262151;
pub const SSL_SECOP_VERSION: i32 = 9;
pub const SSL_SENT_SHUTDOWN: i32 = 1;
pub const SSL_SERVERINFOV1: i32 = 1;
pub const SSL_SERVERINFOV2: i32 = 2;
pub type SSL_SESSION = ssl_session_st;
pub const SSL_SESSION_ASN1_VERSION: i32 = 1;
pub const SSL_SESSION_CACHE_MAX_SIZE_DEFAULT: i32 = 20480;
pub const SSL_SESS_CACHE_BOTH: i32 = 3;
pub const SSL_SESS_CACHE_CLIENT: i32 = 1;
pub const SSL_SESS_CACHE_NO_AUTO_CLEAR: i32 = 128;
pub const SSL_SESS_CACHE_NO_INTERNAL: i32 = 768;
pub const SSL_SESS_CACHE_NO_INTERNAL_LOOKUP: i32 = 256;
pub const SSL_SESS_CACHE_NO_INTERNAL_STORE: i32 = 512;
pub const SSL_SESS_CACHE_OFF: i32 = 0;
pub const SSL_SESS_CACHE_SERVER: i32 = 2;
pub const SSL_SESS_CACHE_UPDATE_TIME: i32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SSL_SHUTDOWN_EX_ARGS {
    pub quic_error_code: u64,
    pub quic_reason: *const i8,
}
pub const SSL_SHUTDOWN_FLAG_NO_BLOCK: u32 = 4;
pub const SSL_SHUTDOWN_FLAG_NO_STREAM_FLUSH: u32 = 2;
pub const SSL_SHUTDOWN_FLAG_RAPID: u32 = 1;
pub const SSL_SHUTDOWN_FLAG_WAIT_PEER: u32 = 8;
pub const SSL_STREAM_FLAG_ADVANCE: u32 = 4;
pub const SSL_STREAM_FLAG_NO_BLOCK: u32 = 2;
pub const SSL_STREAM_FLAG_UNI: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SSL_STREAM_RESET_ARGS {
    pub quic_error_code: u64,
}
pub const SSL_STREAM_STATE_CONN_CLOSED: i32 = 6;
pub const SSL_STREAM_STATE_FINISHED: i32 = 3;
pub const SSL_STREAM_STATE_NONE: i32 = 0;
pub const SSL_STREAM_STATE_OK: i32 = 1;
pub const SSL_STREAM_STATE_RESET_LOCAL: i32 = 4;
pub const SSL_STREAM_STATE_RESET_REMOTE: i32 = 5;
pub const SSL_STREAM_STATE_WRONG_DIR: i32 = 2;
pub const SSL_STREAM_TYPE_BIDI: u32 = 3;
pub const SSL_STREAM_TYPE_NONE: i32 = 0;
pub const SSL_STREAM_TYPE_READ: u32 = 1;
pub const SSL_STREAM_TYPE_WRITE: u32 = 2;
pub const SSL_ST_ACCEPT: i32 = 8192;
pub const SSL_ST_CONNECT: i32 = 4096;
pub const SSL_ST_MASK: i32 = 4095;
pub const SSL_ST_READ_BODY: i32 = 241;
pub const SSL_ST_READ_DONE: i32 = 242;
pub const SSL_ST_READ_HEADER: i32 = 240;
pub const SSL_TICKET_EMPTY: i32 = 3;
pub const SSL_TICKET_FATAL_ERR_MALLOC: i32 = 0;
pub const SSL_TICKET_FATAL_ERR_OTHER: i32 = 1;
pub const SSL_TICKET_NONE: i32 = 2;
pub const SSL_TICKET_NO_DECRYPT: i32 = 4;
pub type SSL_TICKET_RETURN = i32;
pub const SSL_TICKET_RETURN_ABORT: i32 = 0;
pub const SSL_TICKET_RETURN_IGNORE: i32 = 1;
pub const SSL_TICKET_RETURN_IGNORE_RENEW: i32 = 2;
pub const SSL_TICKET_RETURN_USE: i32 = 3;
pub const SSL_TICKET_RETURN_USE_RENEW: i32 = 4;
pub type SSL_TICKET_STATUS = i32;
pub const SSL_TICKET_SUCCESS: i32 = 5;
pub const SSL_TICKET_SUCCESS_RENEW: i32 = 6;
pub const SSL_TXT_3DES: *const u8 = [51, 68, 69, 83, 0].as_ptr();
pub const SSL_TXT_ADH: *const u8 = [65, 68, 72, 0].as_ptr();
pub const SSL_TXT_AECDH: *const u8 = [65, 69, 67, 68, 72, 0].as_ptr();
pub const SSL_TXT_AES: *const u8 = [65, 69, 83, 0].as_ptr();
pub const SSL_TXT_AES128: *const u8 = [65, 69, 83, 49, 50, 56, 0].as_ptr();
pub const SSL_TXT_AES256: *const u8 = [65, 69, 83, 50, 53, 54, 0].as_ptr();
pub const SSL_TXT_AES_CCM: *const u8 = [65, 69, 83, 67, 67, 77, 0].as_ptr();
pub const SSL_TXT_AES_CCM_8: *const u8 = [65, 69, 83, 67, 67, 77, 56, 0].as_ptr();
pub const SSL_TXT_AES_GCM: *const u8 = [65, 69, 83, 71, 67, 77, 0].as_ptr();
pub const SSL_TXT_ALL: *const u8 = [65, 76, 76, 0].as_ptr();
pub const SSL_TXT_ARIA: *const u8 = [65, 82, 73, 65, 0].as_ptr();
pub const SSL_TXT_ARIA128: *const u8 = [65, 82, 73, 65, 49, 50, 56, 0].as_ptr();
pub const SSL_TXT_ARIA256: *const u8 = [65, 82, 73, 65, 50, 53, 54, 0].as_ptr();
pub const SSL_TXT_ARIA_GCM: *const u8 = [65, 82, 73, 65, 71, 67, 77, 0].as_ptr();
pub const SSL_TXT_CAMELLIA: *const u8 = [67, 65, 77, 69, 76, 76, 73, 65, 0].as_ptr();
pub const SSL_TXT_CAMELLIA128: *const u8 = [67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 0].as_ptr();
pub const SSL_TXT_CAMELLIA256: *const u8 = [67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 0].as_ptr();
pub const SSL_TXT_CBC: *const u8 = [67, 66, 67, 0].as_ptr();
pub const SSL_TXT_CHACHA20: *const u8 = [67, 72, 65, 67, 72, 65, 50, 48, 0].as_ptr();
pub const SSL_TXT_CMPALL: *const u8 = [
    67, 79, 77, 80, 76, 69, 77, 69, 78, 84, 79, 70, 65, 76, 76, 0,
]
.as_ptr();
pub const SSL_TXT_CMPDEF: *const u8 = [
    67, 79, 77, 80, 76, 69, 77, 69, 78, 84, 79, 70, 68, 69, 70, 65, 85, 76, 84, 0,
]
.as_ptr();
pub const SSL_TXT_DES: *const u8 = [68, 69, 83, 0].as_ptr();
pub const SSL_TXT_DH: *const u8 = [68, 72, 0].as_ptr();
pub const SSL_TXT_DHE: *const u8 = [68, 72, 69, 0].as_ptr();
pub const SSL_TXT_DSS: *const u8 = [68, 83, 83, 0].as_ptr();
pub const SSL_TXT_ECDH: *const u8 = [69, 67, 68, 72, 0].as_ptr();
pub const SSL_TXT_ECDHE: *const u8 = [69, 67, 68, 72, 69, 0].as_ptr();
pub const SSL_TXT_ECDSA: *const u8 = [69, 67, 68, 83, 65, 0].as_ptr();
pub const SSL_TXT_EDH: *const u8 = [69, 68, 72, 0].as_ptr();
pub const SSL_TXT_EECDH: *const u8 = [69, 69, 67, 68, 72, 0].as_ptr();
pub const SSL_TXT_FIPS: *const u8 = [70, 73, 80, 83, 0].as_ptr();
pub const SSL_TXT_GOST: *const u8 = [71, 79, 83, 84, 56, 57, 0].as_ptr();
pub const SSL_TXT_GOST12: *const u8 = [71, 79, 83, 84, 49, 50, 0].as_ptr();
pub const SSL_TXT_GOST2012_GOST8912_GOST8912: *const u8 = [
    71, 79, 83, 84, 50, 48, 49, 50, 45, 71, 79, 83, 84, 56, 57, 49, 50, 45, 71, 79, 83, 84, 56, 57,
    49, 50, 0,
]
.as_ptr();
pub const SSL_TXT_GOST89MAC: *const u8 = [71, 79, 83, 84, 56, 57, 77, 65, 67, 0].as_ptr();
pub const SSL_TXT_GOST89MAC12: *const u8 = [71, 79, 83, 84, 56, 57, 77, 65, 67, 49, 50, 0].as_ptr();
pub const SSL_TXT_GOST94: *const u8 = [71, 79, 83, 84, 57, 52, 0].as_ptr();
pub const SSL_TXT_HIGH: *const u8 = [72, 73, 71, 72, 0].as_ptr();
pub const SSL_TXT_IDEA: *const u8 = [73, 68, 69, 65, 0].as_ptr();
pub const SSL_TXT_LOW: *const u8 = [76, 79, 87, 0].as_ptr();
pub const SSL_TXT_MD5: *const u8 = [77, 68, 53, 0].as_ptr();
pub const SSL_TXT_MEDIUM: *const u8 = [77, 69, 68, 73, 85, 77, 0].as_ptr();
pub const SSL_TXT_NULL: *const u8 = [78, 85, 76, 76, 0].as_ptr();
pub const SSL_TXT_PSK: *const u8 = [80, 83, 75, 0].as_ptr();
pub const SSL_TXT_RC2: *const u8 = [82, 67, 50, 0].as_ptr();
pub const SSL_TXT_RC4: *const u8 = [82, 67, 52, 0].as_ptr();
pub const SSL_TXT_RSA: *const u8 = [82, 83, 65, 0].as_ptr();
pub const SSL_TXT_SEED: *const u8 = [83, 69, 69, 68, 0].as_ptr();
pub const SSL_TXT_SHA: *const u8 = [83, 72, 65, 0].as_ptr();
pub const SSL_TXT_SHA1: *const u8 = [83, 72, 65, 49, 0].as_ptr();
pub const SSL_TXT_SHA256: *const u8 = [83, 72, 65, 50, 53, 54, 0].as_ptr();
pub const SSL_TXT_SHA384: *const u8 = [83, 72, 65, 51, 56, 52, 0].as_ptr();
pub const SSL_TXT_SRP: *const u8 = [83, 82, 80, 0].as_ptr();
pub const SSL_TXT_SSLV3: *const u8 = [83, 83, 76, 118, 51, 0].as_ptr();
pub const SSL_TXT_TLSV1: *const u8 = [84, 76, 83, 118, 49, 0].as_ptr();
pub const SSL_TXT_TLSV1_1: *const u8 = [84, 76, 83, 118, 49, 46, 49, 0].as_ptr();
pub const SSL_TXT_TLSV1_2: *const u8 = [84, 76, 83, 118, 49, 46, 50, 0].as_ptr();
pub const SSL_TXT_aDH: *const u8 = [97, 68, 72, 0].as_ptr();
pub const SSL_TXT_aDSS: *const u8 = [97, 68, 83, 83, 0].as_ptr();
pub const SSL_TXT_aECDH: *const u8 = [97, 69, 67, 68, 72, 0].as_ptr();
pub const SSL_TXT_aECDSA: *const u8 = [97, 69, 67, 68, 83, 65, 0].as_ptr();
pub const SSL_TXT_aGOST: *const u8 = [97, 71, 79, 83, 84, 0].as_ptr();
pub const SSL_TXT_aGOST01: *const u8 = [97, 71, 79, 83, 84, 48, 49, 0].as_ptr();
pub const SSL_TXT_aGOST12: *const u8 = [97, 71, 79, 83, 84, 49, 50, 0].as_ptr();
pub const SSL_TXT_aGOST94: *const u8 = [97, 71, 79, 83, 84, 57, 52, 0].as_ptr();
pub const SSL_TXT_aNULL: *const u8 = [97, 78, 85, 76, 76, 0].as_ptr();
pub const SSL_TXT_aPSK: *const u8 = [97, 80, 83, 75, 0].as_ptr();
pub const SSL_TXT_aRSA: *const u8 = [97, 82, 83, 65, 0].as_ptr();
pub const SSL_TXT_aSRP: *const u8 = [97, 83, 82, 80, 0].as_ptr();
pub const SSL_TXT_eNULL: *const u8 = [101, 78, 85, 76, 76, 0].as_ptr();
pub const SSL_TXT_kDH: *const u8 = [107, 68, 72, 0].as_ptr();
pub const SSL_TXT_kDHE: *const u8 = [107, 68, 72, 69, 0].as_ptr();
pub const SSL_TXT_kDHEPSK: *const u8 = [107, 68, 72, 69, 80, 83, 75, 0].as_ptr();
pub const SSL_TXT_kDHd: *const u8 = [107, 68, 72, 100, 0].as_ptr();
pub const SSL_TXT_kDHr: *const u8 = [107, 68, 72, 114, 0].as_ptr();
pub const SSL_TXT_kECDH: *const u8 = [107, 69, 67, 68, 72, 0].as_ptr();
pub const SSL_TXT_kECDHE: *const u8 = [107, 69, 67, 68, 72, 69, 0].as_ptr();
pub const SSL_TXT_kECDHEPSK: *const u8 = [107, 69, 67, 68, 72, 69, 80, 83, 75, 0].as_ptr();
pub const SSL_TXT_kECDHe: *const u8 = [107, 69, 67, 68, 72, 101, 0].as_ptr();
pub const SSL_TXT_kECDHr: *const u8 = [107, 69, 67, 68, 72, 114, 0].as_ptr();
pub const SSL_TXT_kEDH: *const u8 = [107, 69, 68, 72, 0].as_ptr();
pub const SSL_TXT_kEECDH: *const u8 = [107, 69, 69, 67, 68, 72, 0].as_ptr();
pub const SSL_TXT_kGOST: *const u8 = [107, 71, 79, 83, 84, 0].as_ptr();
pub const SSL_TXT_kGOST18: *const u8 = [107, 71, 79, 83, 84, 49, 56, 0].as_ptr();
pub const SSL_TXT_kPSK: *const u8 = [107, 80, 83, 75, 0].as_ptr();
pub const SSL_TXT_kRSA: *const u8 = [107, 82, 83, 65, 0].as_ptr();
pub const SSL_TXT_kRSAPSK: *const u8 = [107, 82, 83, 65, 80, 83, 75, 0].as_ptr();
pub const SSL_TXT_kSRP: *const u8 = [107, 83, 82, 80, 0].as_ptr();
pub const SSL_VALUE_CLASS_FEATURE_NEGOTIATED: i32 = 3;
pub const SSL_VALUE_CLASS_FEATURE_PEER_REQUEST: i32 = 2;
pub const SSL_VALUE_CLASS_FEATURE_REQUEST: i32 = 1;
pub const SSL_VALUE_CLASS_GENERIC: i32 = 0;
pub const SSL_VALUE_EVENT_HANDLING_MODE: i32 = 6;
pub const SSL_VALUE_EVENT_HANDLING_MODE_EXPLICIT: i32 = 2;
pub const SSL_VALUE_EVENT_HANDLING_MODE_IMPLICIT: i32 = 1;
pub const SSL_VALUE_EVENT_HANDLING_MODE_INHERIT: i32 = 0;
pub const SSL_VALUE_NONE: i32 = 0;
pub const SSL_VALUE_QUIC_IDLE_TIMEOUT: i32 = 5;
pub const SSL_VALUE_QUIC_MAX_PENDING_CONNS: i32 = 16;
pub const SSL_VALUE_QUIC_STREAM_BIDI_LOCAL_AVAIL: i32 = 1;
pub const SSL_VALUE_QUIC_STREAM_BIDI_REMOTE_AVAIL: i32 = 2;
pub const SSL_VALUE_QUIC_STREAM_UNI_LOCAL_AVAIL: i32 = 3;
pub const SSL_VALUE_QUIC_STREAM_UNI_REMOTE_AVAIL: i32 = 4;
pub const SSL_VALUE_STREAM_WRITE_BUF_AVAIL: i32 = 9;
pub const SSL_VALUE_STREAM_WRITE_BUF_SIZE: i32 = 7;
pub const SSL_VALUE_STREAM_WRITE_BUF_USED: i32 = 8;
pub const SSL_VERIFY_CLIENT_ONCE: i32 = 4;
pub const SSL_VERIFY_FAIL_IF_NO_PEER_CERT: i32 = 2;
pub const SSL_VERIFY_NONE: i32 = 0;
pub const SSL_VERIFY_PEER: i32 = 1;
pub const SSL_VERIFY_POST_HANDSHAKE: i32 = 8;
pub const SSL_WRITE_FLAG_CONCLUDE: u32 = 1;
pub const SSL_WRITING: i32 = 2;
pub const SSL_X509_LOOKUP: i32 = 4;
#[cfg(feature = "types")]
pub type SSL_allow_early_data_cb_fn =
    Option<unsafe extern "C" fn(s: *mut super::types::SSL, arg: *mut core::ffi::c_void) -> i32>;
#[cfg(feature = "types")]
pub type SSL_async_callback_fn =
    Option<unsafe extern "C" fn(s: *mut super::types::SSL, arg: *mut core::ffi::c_void) -> i32>;
#[cfg(feature = "types")]
pub type SSL_client_hello_cb_fn = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        al: *mut i32,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_custom_ext_add_cb_ex = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        ext_type: u32,
        context: u32,
        out: *mut *mut u8,
        outlen: *mut usize,
        x: *mut super::types::X509,
        chainidx: usize,
        al: *mut i32,
        add_arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_custom_ext_free_cb_ex = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        ext_type: u32,
        context: u32,
        out: *const u8,
        add_arg: *mut core::ffi::c_void,
    ),
>;
#[cfg(feature = "types")]
pub type SSL_custom_ext_parse_cb_ex = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        ext_type: u32,
        context: u32,
        r#in: *const u8,
        inlen: usize,
        x: *mut super::types::X509,
        chainidx: usize,
        al: *mut i32,
        parse_arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_new_pending_conn_cb_fn = Option<
    unsafe extern "C" fn(
        ctx: *mut super::types::SSL_CTX,
        new_ssl: *mut super::types::SSL,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_psk_client_cb_func = Option<
    unsafe extern "C" fn(
        ssl: *mut super::types::SSL,
        hint: *const i8,
        identity: *mut i8,
        max_identity_len: u32,
        psk: *mut u8,
        max_psk_len: u32,
    ) -> u32,
>;
#[cfg(feature = "types")]
pub type SSL_psk_find_session_cb_func = Option<
    unsafe extern "C" fn(
        ssl: *mut super::types::SSL,
        identity: *const u8,
        identity_len: usize,
        sess: *mut *mut SSL_SESSION,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_psk_server_cb_func = Option<
    unsafe extern "C" fn(
        ssl: *mut super::types::SSL,
        identity: *const i8,
        psk: *mut u8,
        max_psk_len: u32,
    ) -> u32,
>;
#[cfg(feature = "types")]
pub type SSL_psk_use_session_cb_func = Option<
    unsafe extern "C" fn(
        ssl: *mut super::types::SSL,
        md: *const super::types::EVP_MD,
        id: *mut *mut u8,
        idlen: *mut usize,
        sess: *mut *mut SSL_SESSION,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type SSL_verify_cb = Option<
    unsafe extern "C" fn(preverify_ok: i32, x509_ctx: *mut super::types::X509_STORE_CTX) -> i32,
>;
#[cfg(feature = "tls1")]
pub type TLS_SESSION_TICKET_EXT = super::tls1::tls_session_ticket_ext_st;
pub type TLS_SIGALGS = tls_sigalgs_st;
pub const TLS_ST_BEFORE: OSSL_HANDSHAKE_STATE = 0;
pub const TLS_ST_CR_CERT: OSSL_HANDSHAKE_STATE = 4;
pub const TLS_ST_CR_CERT_REQ: OSSL_HANDSHAKE_STATE = 8;
pub const TLS_ST_CR_CERT_STATUS: OSSL_HANDSHAKE_STATE = 6;
pub const TLS_ST_CR_CERT_VRFY: OSSL_HANDSHAKE_STATE = 43;
pub const TLS_ST_CR_CHANGE: OSSL_HANDSHAKE_STATE = 11;
pub const TLS_ST_CR_COMP_CERT: OSSL_HANDSHAKE_STATE = 5;
pub const TLS_ST_CR_ENCRYPTED_EXTENSIONS: OSSL_HANDSHAKE_STATE = 42;
pub const TLS_ST_CR_FINISHED: OSSL_HANDSHAKE_STATE = 12;
pub const TLS_ST_CR_HELLO_REQ: OSSL_HANDSHAKE_STATE = 45;
pub const TLS_ST_CR_KEY_EXCH: OSSL_HANDSHAKE_STATE = 7;
pub const TLS_ST_CR_KEY_UPDATE: OSSL_HANDSHAKE_STATE = 49;
pub const TLS_ST_CR_SESSION_TICKET: OSSL_HANDSHAKE_STATE = 10;
pub const TLS_ST_CR_SRVR_DONE: OSSL_HANDSHAKE_STATE = 9;
pub const TLS_ST_CR_SRVR_HELLO: OSSL_HANDSHAKE_STATE = 3;
pub const TLS_ST_CW_CERT: OSSL_HANDSHAKE_STATE = 14;
pub const TLS_ST_CW_CERT_VRFY: OSSL_HANDSHAKE_STATE = 17;
pub const TLS_ST_CW_CHANGE: OSSL_HANDSHAKE_STATE = 18;
pub const TLS_ST_CW_CLNT_HELLO: OSSL_HANDSHAKE_STATE = 13;
pub const TLS_ST_CW_COMP_CERT: OSSL_HANDSHAKE_STATE = 15;
pub const TLS_ST_CW_END_OF_EARLY_DATA: OSSL_HANDSHAKE_STATE = 52;
pub const TLS_ST_CW_FINISHED: OSSL_HANDSHAKE_STATE = 20;
pub const TLS_ST_CW_KEY_EXCH: OSSL_HANDSHAKE_STATE = 16;
pub const TLS_ST_CW_KEY_UPDATE: OSSL_HANDSHAKE_STATE = 47;
pub const TLS_ST_CW_NEXT_PROTO: OSSL_HANDSHAKE_STATE = 19;
pub const TLS_ST_EARLY_DATA: OSSL_HANDSHAKE_STATE = 50;
pub const TLS_ST_OK: OSSL_HANDSHAKE_STATE = 1;
pub const TLS_ST_PENDING_EARLY_DATA_END: OSSL_HANDSHAKE_STATE = 51;
pub const TLS_ST_SR_CERT: OSSL_HANDSHAKE_STATE = 30;
pub const TLS_ST_SR_CERT_VRFY: OSSL_HANDSHAKE_STATE = 33;
pub const TLS_ST_SR_CHANGE: OSSL_HANDSHAKE_STATE = 35;
pub const TLS_ST_SR_CLNT_HELLO: OSSL_HANDSHAKE_STATE = 22;
pub const TLS_ST_SR_COMP_CERT: OSSL_HANDSHAKE_STATE = 31;
pub const TLS_ST_SR_END_OF_EARLY_DATA: OSSL_HANDSHAKE_STATE = 53;
pub const TLS_ST_SR_FINISHED: OSSL_HANDSHAKE_STATE = 36;
pub const TLS_ST_SR_KEY_EXCH: OSSL_HANDSHAKE_STATE = 32;
pub const TLS_ST_SR_KEY_UPDATE: OSSL_HANDSHAKE_STATE = 48;
pub const TLS_ST_SR_NEXT_PROTO: OSSL_HANDSHAKE_STATE = 34;
pub const TLS_ST_SW_CERT: OSSL_HANDSHAKE_STATE = 25;
pub const TLS_ST_SW_CERT_REQ: OSSL_HANDSHAKE_STATE = 28;
pub const TLS_ST_SW_CERT_STATUS: OSSL_HANDSHAKE_STATE = 38;
pub const TLS_ST_SW_CERT_VRFY: OSSL_HANDSHAKE_STATE = 44;
pub const TLS_ST_SW_CHANGE: OSSL_HANDSHAKE_STATE = 39;
pub const TLS_ST_SW_COMP_CERT: OSSL_HANDSHAKE_STATE = 26;
pub const TLS_ST_SW_ENCRYPTED_EXTENSIONS: OSSL_HANDSHAKE_STATE = 41;
pub const TLS_ST_SW_FINISHED: OSSL_HANDSHAKE_STATE = 40;
pub const TLS_ST_SW_HELLO_REQ: OSSL_HANDSHAKE_STATE = 21;
pub const TLS_ST_SW_KEY_EXCH: OSSL_HANDSHAKE_STATE = 27;
pub const TLS_ST_SW_KEY_UPDATE: OSSL_HANDSHAKE_STATE = 46;
pub const TLS_ST_SW_SESSION_TICKET: OSSL_HANDSHAKE_STATE = 37;
pub const TLS_ST_SW_SRVR_DONE: OSSL_HANDSHAKE_STATE = 29;
pub const TLS_ST_SW_SRVR_HELLO: OSSL_HANDSHAKE_STATE = 24;
#[cfg(feature = "types")]
pub type custom_ext_add_cb = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        ext_type: u32,
        out: *mut *mut u8,
        outlen: *mut usize,
        al: *mut i32,
        add_arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type custom_ext_free_cb = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        ext_type: u32,
        out: *const u8,
        add_arg: *mut core::ffi::c_void,
    ),
>;
#[cfg(feature = "types")]
pub type custom_ext_parse_cb = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        ext_type: u32,
        r#in: *const u8,
        inlen: usize,
        al: *mut i32,
        parse_arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct lhash_st_SSL_SESSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct openssl_ssl_test_functions(pub u8);
pub type sk_SRTP_PROTECTION_PROFILE_compfunc = Option<
    unsafe extern "C" fn(
        a: *const *const SRTP_PROTECTION_PROFILE,
        b: *const *const SRTP_PROTECTION_PROFILE,
    ) -> i32,
>;
pub type sk_SRTP_PROTECTION_PROFILE_copyfunc =
    Option<unsafe extern "C" fn(a: *const SRTP_PROTECTION_PROFILE) -> *mut SRTP_PROTECTION_PROFILE>;
pub type sk_SRTP_PROTECTION_PROFILE_freefunc =
    Option<unsafe extern "C" fn(a: *mut SRTP_PROTECTION_PROFILE)>;
pub type sk_SSL_CIPHER_compfunc =
    Option<unsafe extern "C" fn(a: *const *const SSL_CIPHER, b: *const *const SSL_CIPHER) -> i32>;
pub type sk_SSL_CIPHER_copyfunc =
    Option<unsafe extern "C" fn(a: *const SSL_CIPHER) -> *mut SSL_CIPHER>;
pub type sk_SSL_CIPHER_freefunc = Option<unsafe extern "C" fn(a: *mut SSL_CIPHER)>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_cipher_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_conf_ctx_st(pub u8);
#[cfg(feature = "types")]
pub type ssl_crock_st = *mut super::types::ssl_st;
#[cfg(feature = "types")]
pub type ssl_ct_validation_cb = Option<
    unsafe extern "C" fn(
        ctx: *const super::types::CT_POLICY_EVAL_CTX,
        scts: *const core::ffi::c_void,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_session_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stack_st_SRTP_PROTECTION_PROFILE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stack_st_SSL_CIPHER(pub u8);
#[cfg(feature = "types")]
pub type tls_session_secret_cb_fn = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        secret: *mut core::ffi::c_void,
        secret_len: *mut i32,
        peer_ciphers: *mut core::ffi::c_void,
        cipher: *mut *mut SSL_CIPHER,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[cfg(feature = "types")]
pub type tls_session_ticket_ext_cb_fn = Option<
    unsafe extern "C" fn(
        s: *mut super::types::SSL,
        data: *const u8,
        len: i32,
        arg: *mut core::ffi::c_void,
    ) -> i32,
>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tls_sigalgs_st(pub u8);
