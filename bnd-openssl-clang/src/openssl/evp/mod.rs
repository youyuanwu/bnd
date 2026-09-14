#[cfg(feature = "bio")]
windows_link::link!("crypto" "C" fn BIO_f_base64() -> *const super::bio::BIO_METHOD);
#[cfg(feature = "bio")]
windows_link::link!("crypto" "C" fn BIO_f_cipher() -> *const super::bio::BIO_METHOD);
#[cfg(feature = "bio")]
windows_link::link!("crypto" "C" fn BIO_f_md() -> *const super::bio::BIO_METHOD);
#[cfg(feature = "bio")]
windows_link::link!("crypto" "C" fn BIO_f_reliable() -> *const super::bio::BIO_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn BIO_set_cipher(b : *mut super::types::BIO, c : *const super::types::EVP_CIPHER, k : *const u8, i : *const u8, enc : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_ASYM_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_free(cipher : *mut super::types::EVP_ASYM_CIPHER));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_get0_description(cipher : *const super::types::EVP_ASYM_CIPHER) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_get0_name(cipher : *const super::types::EVP_ASYM_CIPHER) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_get0_provider(cipher : *const super::types::EVP_ASYM_CIPHER) -> *mut super::types::OSSL_PROVIDER);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_gettable_ctx_params(ciph : *const super::types::EVP_ASYM_CIPHER) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_is_a(cipher : *const super::types::EVP_ASYM_CIPHER, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_names_do_all(cipher : *const super::types::EVP_ASYM_CIPHER, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_settable_ctx_params(ciph : *const super::types::EVP_ASYM_CIPHER) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ASYM_CIPHER_up_ref(cipher : *mut super::types::EVP_ASYM_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_BytesToKey(r#type : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, salt : *const u8, data : *const u8, datal : i32, count : i32, key : *mut u8, iv : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_buf_noconst(ctx : *mut super::types::EVP_CIPHER_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_cipher(ctx : *const super::types::EVP_CIPHER_CTX) -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_clear_flags(ctx : *mut super::types::EVP_CIPHER_CTX, flags : i32));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_copy(out : *mut super::types::EVP_CIPHER_CTX, r#in : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_ctrl(ctx : *mut super::types::EVP_CIPHER_CTX, r#type : i32, arg : i32, ptr : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_dup(r#in : *const super::types::EVP_CIPHER_CTX) -> *mut super::types::EVP_CIPHER_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_free(c : *mut super::types::EVP_CIPHER_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get0_cipher(ctx : *const super::types::EVP_CIPHER_CTX) -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get1_cipher(ctx : *mut super::types::EVP_CIPHER_CTX) -> *mut super::types::EVP_CIPHER);
#[cfg(all(feature = "asn1", feature = "types", feature = "x509"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_algor(ctx : *mut super::types::EVP_CIPHER_CTX, alg : *mut *mut super::types::X509_ALGOR) -> i32);
#[cfg(all(feature = "asn1", feature = "types", feature = "x509"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_algor_params(ctx : *mut super::types::EVP_CIPHER_CTX, alg : *mut super::types::X509_ALGOR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_app_data(ctx : *const super::types::EVP_CIPHER_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_block_size(ctx : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_cipher_data(ctx : *const super::types::EVP_CIPHER_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_iv_length(ctx : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_key_length(ctx : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_nid(ctx : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_num(ctx : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_original_iv(ctx : *mut super::types::EVP_CIPHER_CTX, buf : *mut core::ffi::c_void, len : usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_params(ctx : *mut super::types::EVP_CIPHER_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_tag_length(ctx : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_get_updated_iv(ctx : *mut super::types::EVP_CIPHER_CTX, buf : *mut core::ffi::c_void, len : usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_gettable_params(ctx : *mut super::types::EVP_CIPHER_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_is_encrypting(ctx : *const super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_iv(ctx : *const super::types::EVP_CIPHER_CTX) -> *const u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_iv_noconst(ctx : *mut super::types::EVP_CIPHER_CTX) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_new() -> *mut super::types::EVP_CIPHER_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_original_iv(ctx : *const super::types::EVP_CIPHER_CTX) -> *const u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_rand_key(ctx : *mut super::types::EVP_CIPHER_CTX, key : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_reset(c : *mut super::types::EVP_CIPHER_CTX) -> i32);
#[cfg(all(feature = "asn1", feature = "types", feature = "x509"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_algor_params(ctx : *mut super::types::EVP_CIPHER_CTX, alg : *const super::types::X509_ALGOR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_app_data(ctx : *mut super::types::EVP_CIPHER_CTX, data : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_cipher_data(ctx : *mut super::types::EVP_CIPHER_CTX, cipher_data : *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_flags(ctx : *mut super::types::EVP_CIPHER_CTX, flags : i32));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_key_length(x : *mut super::types::EVP_CIPHER_CTX, keylen : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_num(ctx : *mut super::types::EVP_CIPHER_CTX, num : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_padding(c : *mut super::types::EVP_CIPHER_CTX, pad : i32) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_set_params(ctx : *mut super::types::EVP_CIPHER_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_settable_params(ctx : *mut super::types::EVP_CIPHER_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_CTX_test_flags(ctx : *const super::types::EVP_CIPHER_CTX, flags : i32) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_asn1_to_param(c : *mut super::types::EVP_CIPHER_CTX, r#type : *mut super::types::ASN1_TYPE) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_can_pipeline(cipher : *const super::types::EVP_CIPHER, enc : i32) -> i32);
windows_link::link!("crypto" "C" fn EVP_CIPHER_do_all(r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
windows_link::link!("crypto" "C" fn EVP_CIPHER_do_all_sorted(r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_free(cipher : *mut super::types::EVP_CIPHER));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get0_description(cipher : *const super::types::EVP_CIPHER) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get0_name(cipher : *const super::types::EVP_CIPHER) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get0_provider(cipher : *const super::types::EVP_CIPHER) -> *const super::types::OSSL_PROVIDER);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_asn1_iv(c : *mut super::types::EVP_CIPHER_CTX, r#type : *mut super::types::ASN1_TYPE) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_block_size(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_flags(cipher : *const super::types::EVP_CIPHER) -> u64);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_iv_length(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_key_length(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_mode(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_nid(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_params(cipher : *mut super::types::EVP_CIPHER, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_get_type(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_gettable_ctx_params(cipher : *const super::types::EVP_CIPHER) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_gettable_params(cipher : *const super::types::EVP_CIPHER) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_impl_ctx_size(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_is_a(cipher : *const super::types::EVP_CIPHER, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_dup(cipher : *const super::types::EVP_CIPHER) -> *mut super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_free(cipher : *mut super::types::EVP_CIPHER));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_get_cleanup(cipher : *const super::types::EVP_CIPHER) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_get_ctrl(cipher : *const super::types::EVP_CIPHER) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_get_do_cipher(cipher : *const super::types::EVP_CIPHER) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_get_get_asn1_params(cipher : *const super::types::EVP_CIPHER) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_get_init(cipher : *const super::types::EVP_CIPHER) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_get_set_asn1_params(cipher : *const super::types::EVP_CIPHER) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_new(cipher_type : i32, block_size : i32, key_len : i32) -> *mut super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_cleanup(cipher : *mut super::types::EVP_CIPHER, cleanup : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_ctrl(cipher : *mut super::types::EVP_CIPHER, ctrl : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_do_cipher(cipher : *mut super::types::EVP_CIPHER, do_cipher : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_flags(cipher : *mut super::types::EVP_CIPHER, flags : u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_get_asn1_params(cipher : *mut super::types::EVP_CIPHER, get_asn1_parameters : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_impl_ctx_size(cipher : *mut super::types::EVP_CIPHER, ctx_size : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_init(cipher : *mut super::types::EVP_CIPHER, init : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_iv_length(cipher : *mut super::types::EVP_CIPHER, iv_len : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_meth_set_set_asn1_params(cipher : *mut super::types::EVP_CIPHER, set_asn1_parameters : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_names_do_all(cipher : *const super::types::EVP_CIPHER, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_param_to_asn1(c : *mut super::types::EVP_CIPHER_CTX, r#type : *mut super::types::ASN1_TYPE) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_set_asn1_iv(c : *mut super::types::EVP_CIPHER_CTX, r#type : *mut super::types::ASN1_TYPE) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CIPHER_settable_ctx_params(cipher : *const super::types::EVP_CIPHER) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CIPHER_up_ref(cipher : *mut super::types::EVP_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_Cipher(c : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, r#in : *const u8, inl : u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherFinal(ctx : *mut super::types::EVP_CIPHER_CTX, outm : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherFinal_ex(ctx : *mut super::types::EVP_CIPHER_CTX, outm : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherInit(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, iv : *const u8, enc : i32) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CipherInit_SKEY(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, skey : *mut super::types::EVP_SKEY, iv : *const u8, iv_len : usize, enc : i32, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherInit_ex(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, r#impl : *mut super::types::ENGINE, key : *const u8, iv : *const u8, enc : i32) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_CipherInit_ex2(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, iv : *const u8, enc : i32, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherPipelineDecryptInit(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, keylen : usize, numpipes : usize, iv : *mut *mut u8, ivlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherPipelineEncryptInit(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, keylen : usize, numpipes : usize, iv : *mut *mut u8, ivlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherPipelineFinal(ctx : *mut super::types::EVP_CIPHER_CTX, outm : *mut *mut u8, outl : *mut usize, outsize : *const usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherPipelineUpdate(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut *mut u8, outl : *mut usize, outsize : *const usize, r#in : *mut *mut u8, inl : *const usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_CipherUpdate(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, outl : *mut i32, r#in : *const u8, inl : i32) -> i32);
windows_link::link!("crypto" "C" fn EVP_DecodeBlock(t : *mut u8, f : *const u8, n : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecodeFinal(ctx : *mut super::types::EVP_ENCODE_CTX, out : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecodeInit(ctx : *mut super::types::EVP_ENCODE_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecodeUpdate(ctx : *mut super::types::EVP_ENCODE_CTX, out : *mut u8, outl : *mut i32, r#in : *const u8, inl : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecryptFinal(ctx : *mut super::types::EVP_CIPHER_CTX, outm : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecryptFinal_ex(ctx : *mut super::types::EVP_CIPHER_CTX, outm : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecryptInit(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, iv : *const u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecryptInit_ex(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, r#impl : *mut super::types::ENGINE, key : *const u8, iv : *const u8) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_DecryptInit_ex2(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, iv : *const u8, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DecryptUpdate(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, outl : *mut i32, r#in : *const u8, inl : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_Digest(data : *const core::ffi::c_void, count : usize, md : *mut u8, size : *mut u32, r#type : *const super::types::EVP_MD, r#impl : *mut super::types::ENGINE) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestFinal(ctx : *mut super::types::EVP_MD_CTX, md : *mut u8, s : *mut u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestFinalXOF(ctx : *mut super::types::EVP_MD_CTX, out : *mut u8, outlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestFinal_ex(ctx : *mut super::types::EVP_MD_CTX, md : *mut u8, s : *mut u32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestInit(ctx : *mut super::types::EVP_MD_CTX, r#type : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestInit_ex(ctx : *mut super::types::EVP_MD_CTX, r#type : *const super::types::EVP_MD, r#impl : *mut super::types::ENGINE) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_DigestInit_ex2(ctx : *mut super::types::EVP_MD_CTX, r#type : *const super::types::EVP_MD, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestSign(ctx : *mut super::types::EVP_MD_CTX, sigret : *mut u8, siglen : *mut usize, tbs : *const u8, tbslen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestSignFinal(ctx : *mut super::types::EVP_MD_CTX, sigret : *mut u8, siglen : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestSignInit(ctx : *mut super::types::EVP_MD_CTX, pctx : *mut *mut super::types::EVP_PKEY_CTX, r#type : *const super::types::EVP_MD, e : *mut super::types::ENGINE, pkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_DigestSignInit_ex(ctx : *mut super::types::EVP_MD_CTX, pctx : *mut *mut super::types::EVP_PKEY_CTX, mdname : *const i8, libctx : *mut super::types::OSSL_LIB_CTX, props : *const i8, pkey : *mut super::types::EVP_PKEY, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestSignUpdate(ctx : *mut super::types::EVP_MD_CTX, data : *const core::ffi::c_void, dsize : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestSqueeze(ctx : *mut super::types::EVP_MD_CTX, out : *mut u8, outlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestUpdate(ctx : *mut super::types::EVP_MD_CTX, d : *const core::ffi::c_void, cnt : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestVerify(ctx : *mut super::types::EVP_MD_CTX, sigret : *const u8, siglen : usize, tbs : *const u8, tbslen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestVerifyFinal(ctx : *mut super::types::EVP_MD_CTX, sig : *const u8, siglen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestVerifyInit(ctx : *mut super::types::EVP_MD_CTX, pctx : *mut *mut super::types::EVP_PKEY_CTX, r#type : *const super::types::EVP_MD, e : *mut super::types::ENGINE, pkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_DigestVerifyInit_ex(ctx : *mut super::types::EVP_MD_CTX, pctx : *mut *mut super::types::EVP_PKEY_CTX, mdname : *const i8, libctx : *mut super::types::OSSL_LIB_CTX, props : *const i8, pkey : *mut super::types::EVP_PKEY, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_DigestVerifyUpdate(ctx : *mut super::types::EVP_MD_CTX, data : *const core::ffi::c_void, dsize : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ENCODE_CTX_copy(dctx : *mut super::types::EVP_ENCODE_CTX, sctx : *const super::types::EVP_ENCODE_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ENCODE_CTX_free(ctx : *mut super::types::EVP_ENCODE_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ENCODE_CTX_new() -> *mut super::types::EVP_ENCODE_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ENCODE_CTX_num(ctx : *mut super::types::EVP_ENCODE_CTX) -> i32);
windows_link::link!("crypto" "C" fn EVP_EncodeBlock(t : *mut u8, f : *const u8, n : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncodeFinal(ctx : *mut super::types::EVP_ENCODE_CTX, out : *mut u8, outl : *mut i32));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncodeInit(ctx : *mut super::types::EVP_ENCODE_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncodeUpdate(ctx : *mut super::types::EVP_ENCODE_CTX, out : *mut u8, outl : *mut i32, r#in : *const u8, inl : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncryptFinal(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncryptFinal_ex(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncryptInit(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, iv : *const u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncryptInit_ex(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, r#impl : *mut super::types::ENGINE, key : *const u8, iv : *const u8) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_EncryptInit_ex2(ctx : *mut super::types::EVP_CIPHER_CTX, cipher : *const super::types::EVP_CIPHER, key : *const u8, iv : *const u8, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_EncryptUpdate(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, outl : *mut i32, r#in : *const u8, inl : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_KEM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_free(wrap : *mut super::types::EVP_KEM));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_get0_description(wrap : *const super::types::EVP_KEM) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_get0_name(wrap : *const super::types::EVP_KEM) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_get0_provider(wrap : *const super::types::EVP_KEM) -> *mut super::types::OSSL_PROVIDER);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEM_gettable_ctx_params(kem : *const super::types::EVP_KEM) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_is_a(wrap : *const super::types::EVP_KEM, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_names_do_all(wrap : *const super::types::EVP_KEM, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEM_settable_ctx_params(kem : *const super::types::EVP_KEM) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEM_up_ref(wrap : *mut super::types::EVP_KEM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, data : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_KEYEXCH);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_free(exchange : *mut super::types::EVP_KEYEXCH));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_get0_description(keyexch : *const super::types::EVP_KEYEXCH) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_get0_name(keyexch : *const super::types::EVP_KEYEXCH) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_get0_provider(exchange : *const super::types::EVP_KEYEXCH) -> *mut super::types::OSSL_PROVIDER);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_gettable_ctx_params(keyexch : *const super::types::EVP_KEYEXCH) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_is_a(keyexch : *const super::types::EVP_KEYEXCH, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_names_do_all(keyexch : *const super::types::EVP_KEYEXCH, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_settable_ctx_params(keyexch : *const super::types::EVP_KEYEXCH) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYEXCH_up_ref(exchange : *mut super::types::EVP_KEYEXCH) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_KEYMGMT);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_free(keymgmt : *mut super::types::EVP_KEYMGMT));
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_gen_gettable_params(keymgmt : *const super::types::EVP_KEYMGMT) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_gen_settable_params(keymgmt : *const super::types::EVP_KEYMGMT) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_get0_description(keymgmt : *const super::types::EVP_KEYMGMT) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_get0_name(keymgmt : *const super::types::EVP_KEYMGMT) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_get0_provider(keymgmt : *const super::types::EVP_KEYMGMT) -> *const super::types::OSSL_PROVIDER);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_gettable_params(keymgmt : *const super::types::EVP_KEYMGMT) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_is_a(keymgmt : *const super::types::EVP_KEYMGMT, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_names_do_all(keymgmt : *const super::types::EVP_KEYMGMT, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_settable_params(keymgmt : *const super::types::EVP_KEYMGMT) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_KEYMGMT_up_ref(keymgmt : *mut super::types::EVP_KEYMGMT) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_dup(src : *const super::types::EVP_MAC_CTX) -> *mut super::types::EVP_MAC_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_free(ctx : *mut super::types::EVP_MAC_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_get0_mac(ctx : *mut super::types::EVP_MAC_CTX) -> *mut super::types::EVP_MAC);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_get_block_size(ctx : *mut super::types::EVP_MAC_CTX) -> usize);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_get_mac_size(ctx : *mut super::types::EVP_MAC_CTX) -> usize);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_get_params(ctx : *mut super::types::EVP_MAC_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_gettable_params(ctx : *mut super::types::EVP_MAC_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_new(mac : *mut super::types::EVP_MAC) -> *mut super::types::EVP_MAC_CTX);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_set_params(ctx : *mut super::types::EVP_MAC_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_CTX_settable_params(ctx : *mut super::types::EVP_MAC_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_fetch(libctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_MAC);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_final(ctx : *mut super::types::EVP_MAC_CTX, out : *mut u8, outl : *mut usize, outsize : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_finalXOF(ctx : *mut super::types::EVP_MAC_CTX, out : *mut u8, outsize : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_free(mac : *mut super::types::EVP_MAC));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_get0_description(mac : *const super::types::EVP_MAC) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_get0_name(mac : *const super::types::EVP_MAC) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_get0_provider(mac : *const super::types::EVP_MAC) -> *const super::types::OSSL_PROVIDER);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_get_params(mac : *mut super::types::EVP_MAC, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_gettable_ctx_params(mac : *const super::types::EVP_MAC) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_gettable_params(mac : *const super::types::EVP_MAC) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_init(ctx : *mut super::types::EVP_MAC_CTX, key : *const u8, keylen : usize, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_init_SKEY(ctx : *mut super::types::EVP_MAC_CTX, skey : *mut super::types::EVP_SKEY, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_is_a(mac : *const super::types::EVP_MAC, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_names_do_all(mac : *const super::types::EVP_MAC, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MAC_settable_ctx_params(mac : *const super::types::EVP_MAC) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_up_ref(mac : *mut super::types::EVP_MAC) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MAC_update(ctx : *mut super::types::EVP_MAC_CTX, data : *const u8, datalen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_clear_flags(ctx : *mut super::types::EVP_MD_CTX, flags : i32));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_copy(out : *mut super::types::EVP_MD_CTX, r#in : *const super::types::EVP_MD_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_copy_ex(out : *mut super::types::EVP_MD_CTX, r#in : *const super::types::EVP_MD_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_ctrl(ctx : *mut super::types::EVP_MD_CTX, cmd : i32, p1 : i32, p2 : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_dup(r#in : *const super::types::EVP_MD_CTX) -> *mut super::types::EVP_MD_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_free(ctx : *mut super::types::EVP_MD_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_get0_md(ctx : *const super::types::EVP_MD_CTX) -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_get0_md_data(ctx : *const super::types::EVP_MD_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_get1_md(ctx : *mut super::types::EVP_MD_CTX) -> *mut super::types::EVP_MD);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_get_params(ctx : *mut super::types::EVP_MD_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_get_pkey_ctx(ctx : *const super::types::EVP_MD_CTX) -> *mut super::types::EVP_PKEY_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_get_size_ex(ctx : *const super::types::EVP_MD_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_gettable_params(ctx : *mut super::types::EVP_MD_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_md(ctx : *const super::types::EVP_MD_CTX) -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_new() -> *mut super::types::EVP_MD_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_reset(ctx : *mut super::types::EVP_MD_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_set_flags(ctx : *mut super::types::EVP_MD_CTX, flags : i32));
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_set_params(ctx : *mut super::types::EVP_MD_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_set_pkey_ctx(ctx : *mut super::types::EVP_MD_CTX, pctx : *mut super::types::EVP_PKEY_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_set_update_fn(ctx : *mut super::types::EVP_MD_CTX, update : *mut u8));
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_settable_params(ctx : *mut super::types::EVP_MD_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_test_flags(ctx : *const super::types::EVP_MD_CTX, flags : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_CTX_update_fn(ctx : *mut super::types::EVP_MD_CTX) -> *mut u8);
windows_link::link!("crypto" "C" fn EVP_MD_do_all(r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
windows_link::link!("crypto" "C" fn EVP_MD_do_all_sorted(r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_free(md : *mut super::types::EVP_MD));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get0_description(md : *const super::types::EVP_MD) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get0_name(md : *const super::types::EVP_MD) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get0_provider(md : *const super::types::EVP_MD) -> *const super::types::OSSL_PROVIDER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get_block_size(md : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get_flags(md : *const super::types::EVP_MD) -> u64);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_get_params(digest : *const super::types::EVP_MD, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get_pkey_type(md : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get_size(md : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_get_type(md : *const super::types::EVP_MD) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_gettable_ctx_params(md : *const super::types::EVP_MD) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_gettable_params(digest : *const super::types::EVP_MD) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_is_a(md : *const super::types::EVP_MD, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_dup(md : *const super::types::EVP_MD) -> *mut super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_free(md : *mut super::types::EVP_MD));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_app_datasize(md : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_cleanup(md : *const super::types::EVP_MD) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_copy(md : *const super::types::EVP_MD) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_ctrl(md : *const super::types::EVP_MD) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_final(md : *const super::types::EVP_MD) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_flags(md : *const super::types::EVP_MD) -> u64);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_init(md : *const super::types::EVP_MD) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_input_blocksize(md : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_result_size(md : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_get_update(md : *const super::types::EVP_MD) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_new(md_type : i32, pkey_type : i32) -> *mut super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_app_datasize(md : *mut super::types::EVP_MD, datasize : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_cleanup(md : *mut super::types::EVP_MD, cleanup : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_copy(md : *mut super::types::EVP_MD, copy : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_ctrl(md : *mut super::types::EVP_MD, ctrl : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_final(md : *mut super::types::EVP_MD, r#final : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_flags(md : *mut super::types::EVP_MD, flags : u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_init(md : *mut super::types::EVP_MD, init : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_input_blocksize(md : *mut super::types::EVP_MD, blocksize : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_result_size(md : *mut super::types::EVP_MD, resultsize : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_meth_set_update(md : *mut super::types::EVP_MD, update : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_names_do_all(md : *const super::types::EVP_MD, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_MD_settable_ctx_params(md : *const super::types::EVP_MD) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_up_ref(md : *mut super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_MD_xof(md : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_OpenFinal(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_OpenInit(ctx : *mut super::types::EVP_CIPHER_CTX, r#type : *const super::types::EVP_CIPHER, ek : *const u8, ekl : i32, iv : *const u8, r#priv : *mut super::types::EVP_PKEY) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PBE_CipherInit(pbe_obj : *mut super::types::ASN1_OBJECT, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, ctx : *mut super::types::EVP_CIPHER_CTX, en_de : i32) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PBE_CipherInit_ex(pbe_obj : *mut super::types::ASN1_OBJECT, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, ctx : *mut super::types::EVP_CIPHER_CTX, en_de : i32, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PBE_alg_add(nid : i32, cipher : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, keygen : EVP_PBE_KEYGEN) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PBE_alg_add_type(pbe_type : i32, pbe_nid : i32, cipher_nid : i32, md_nid : i32, keygen : EVP_PBE_KEYGEN) -> i32);
windows_link::link!("crypto" "C" fn EVP_PBE_cleanup());
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PBE_find(r#type : i32, pbe_nid : i32, pcnid : *mut i32, pmnid : *mut i32, pkeygen : *mut EVP_PBE_KEYGEN) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PBE_find_ex(r#type : i32, pbe_nid : i32, pcnid : *mut i32, pmnid : *mut i32, pkeygen : *mut EVP_PBE_KEYGEN, pkeygen_ex : *mut EVP_PBE_KEYGEN_EX) -> i32);
windows_link::link!("crypto" "C" fn EVP_PBE_get(ptype : *mut i32, ppbe_nid : *mut i32, num : usize) -> i32);
windows_link::link!("crypto" "C" fn EVP_PBE_scrypt(pass : *const i8, passlen : usize, salt : *const u8, saltlen : usize, n : u64, r : u64, p : u64, maxmem : u64, key : *mut u8, keylen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PBE_scrypt_ex(pass : *const i8, passlen : usize, salt : *const u8, saltlen : usize, n : u64, r : u64, p : u64, maxmem : u64, key : *mut u8, keylen : usize, ctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_ctrl(ctx : *mut super::types::EVP_PKEY_CTX, keytype : i32, optype : i32, cmd : i32, p1 : i32, p2 : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_ctrl_str(ctx : *mut super::types::EVP_PKEY_CTX, r#type : *const i8, value : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_ctrl_uint64(ctx : *mut super::types::EVP_PKEY_CTX, keytype : i32, optype : i32, cmd : i32, value : u64) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_dup(ctx : *const super::types::EVP_PKEY_CTX) -> *mut super::types::EVP_PKEY_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_free(ctx : *mut super::types::EVP_PKEY_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get0_libctx(ctx : *mut super::types::EVP_PKEY_CTX) -> *mut super::types::OSSL_LIB_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get0_peerkey(ctx : *mut super::types::EVP_PKEY_CTX) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get0_pkey(ctx : *mut super::types::EVP_PKEY_CTX) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get0_propq(ctx : *const super::types::EVP_PKEY_CTX) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get0_provider(ctx : *const super::types::EVP_PKEY_CTX) -> *const super::types::OSSL_PROVIDER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get1_id(ctx : *mut super::types::EVP_PKEY_CTX, id : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get1_id_len(ctx : *mut super::types::EVP_PKEY_CTX, id_len : *mut usize) -> i32);
#[cfg(all(feature = "asn1", feature = "types", feature = "x509"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_algor(ctx : *mut super::types::EVP_PKEY_CTX, alg : *mut *mut super::types::X509_ALGOR) -> i32);
#[cfg(all(feature = "asn1", feature = "types", feature = "x509"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_algor_params(ctx : *mut super::types::EVP_PKEY_CTX, alg : *mut super::types::X509_ALGOR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_app_data(ctx : *mut super::types::EVP_PKEY_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_cb(ctx : *mut super::types::EVP_PKEY_CTX) -> EVP_PKEY_gen_cb);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_data(ctx : *const super::types::EVP_PKEY_CTX) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_group_name(ctx : *mut super::types::EVP_PKEY_CTX, name : *mut i8, namelen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_keygen_info(ctx : *mut super::types::EVP_PKEY_CTX, idx : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_operation(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_params(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_get_signature_md(ctx : *mut super::types::EVP_PKEY_CTX, md : *mut *mut super::types::EVP_MD) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_gettable_params(ctx : *const super::types::EVP_PKEY_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_hex2ctrl(ctx : *mut super::types::EVP_PKEY_CTX, cmd : i32, hex : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_is_a(ctx : *mut super::types::EVP_PKEY_CTX, keytype : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_md(ctx : *mut super::types::EVP_PKEY_CTX, optype : i32, cmd : i32, md : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_new(pkey : *mut super::types::EVP_PKEY, e : *mut super::types::ENGINE) -> *mut super::types::EVP_PKEY_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_new_from_name(libctx : *mut super::types::OSSL_LIB_CTX, name : *const i8, propquery : *const i8) -> *mut super::types::EVP_PKEY_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_new_from_pkey(libctx : *mut super::types::OSSL_LIB_CTX, pkey : *mut super::types::EVP_PKEY, propquery : *const i8) -> *mut super::types::EVP_PKEY_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_new_id(id : i32, e : *mut super::types::ENGINE) -> *mut super::types::EVP_PKEY_CTX);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set0_keygen_info(ctx : *mut super::types::EVP_PKEY_CTX, dat : *mut i32, datlen : i32));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set1_id(ctx : *mut super::types::EVP_PKEY_CTX, id : *const core::ffi::c_void, len : i32) -> i32);
#[cfg(all(feature = "asn1", feature = "types", feature = "x509"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_algor_params(ctx : *mut super::types::EVP_PKEY_CTX, alg : *const super::types::X509_ALGOR) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_app_data(ctx : *mut super::types::EVP_PKEY_CTX, data : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_cb(ctx : *mut super::types::EVP_PKEY_CTX, cb : EVP_PKEY_gen_cb));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_data(ctx : *mut super::types::EVP_PKEY_CTX, data : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_group_name(ctx : *mut super::types::EVP_PKEY_CTX, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_kem_op(ctx : *mut super::types::EVP_PKEY_CTX, op : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_mac_key(ctx : *mut super::types::EVP_PKEY_CTX, key : *const u8, keylen : i32) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_params(ctx : *mut super::types::EVP_PKEY_CTX, params : *const super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_signature(pctx : *mut super::types::EVP_PKEY_CTX, sig : *const u8, siglen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_set_signature_md(ctx : *mut super::types::EVP_PKEY_CTX, md : *const super::types::EVP_MD) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_settable_params(ctx : *const super::types::EVP_PKEY_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_CTX_str2ctrl(ctx : *mut super::types::EVP_PKEY_CTX, cmd : i32, str : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_Q_keygen(libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8, r#type : *const i8, ...) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_add0(ameth : *const super::types::EVP_PKEY_ASN1_METHOD) -> i32);
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_add_alias(to : i32, from : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_copy(dst : *mut super::types::EVP_PKEY_ASN1_METHOD, src : *const super::types::EVP_PKEY_ASN1_METHOD));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_find(pe : *mut *mut super::types::ENGINE, r#type : i32) -> *const super::types::EVP_PKEY_ASN1_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_find_str(pe : *mut *mut super::types::ENGINE, str : *const i8, len : i32) -> *const super::types::EVP_PKEY_ASN1_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_free(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_get0(idx : i32) -> *const super::types::EVP_PKEY_ASN1_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_get0_info(ppkey_id : *mut i32, pkey_base_id : *mut i32, ppkey_flags : *mut i32, pinfo : *mut *mut i8, ppem_str : *mut *mut i8, ameth : *const super::types::EVP_PKEY_ASN1_METHOD) -> i32);
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_get_count() -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_new(id : i32, flags : i32, pem_str : *const i8, info : *const i8) -> *mut super::types::EVP_PKEY_ASN1_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_check(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, pkey_check : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_ctrl(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, pkey_ctrl : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_free(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, pkey_free : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_get_priv_key(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, get_priv_key : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_get_pub_key(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, get_pub_key : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_item(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, item_verify : *mut u8, item_sign : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_param(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, param_decode : *mut u8, param_encode : *mut u8, param_missing : *mut u8, param_copy : *mut u8, param_cmp : *mut u8, param_print : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_param_check(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, pkey_param_check : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_private(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, priv_decode : *mut u8, priv_encode : *mut u8, priv_print : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_public(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, pub_decode : *mut u8, pub_encode : *mut u8, pub_cmp : *mut u8, pub_print : *mut u8, pkey_size : *mut u8, pkey_bits : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_public_check(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, pkey_pub_check : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_security_bits(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, pkey_security_bits : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_set_priv_key(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, set_priv_key : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_set_pub_key(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, set_pub_key : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_asn1_set_siginf(ameth : *mut super::types::EVP_PKEY_ASN1_METHOD, siginf_set : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_assign(pkey : *mut super::types::EVP_PKEY, r#type : i32, key : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_auth_decapsulate_init(ctx : *mut super::types::EVP_PKEY_CTX, authpub : *mut super::types::EVP_PKEY, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_auth_encapsulate_init(ctx : *mut super::types::EVP_PKEY_CTX, authpriv : *mut super::types::EVP_PKEY, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_can_sign(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_check(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_cmp(a : *const super::types::EVP_PKEY, b : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_cmp_parameters(a : *const super::types::EVP_PKEY, b : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_copy_parameters(to : *mut super::types::EVP_PKEY, from : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_decapsulate(ctx : *mut super::types::EVP_PKEY_CTX, unwrapped : *mut u8, unwrappedlen : *mut usize, wrapped : *const u8, wrappedlen : usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_decapsulate_init(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_decrypt(ctx : *mut super::types::EVP_PKEY_CTX, out : *mut u8, outlen : *mut usize, r#in : *const u8, inlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_decrypt_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_decrypt_init_ex(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_decrypt_old(dec_key : *mut u8, enc_key : *const u8, enc_key_len : i32, private_key : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_derive(ctx : *mut super::types::EVP_PKEY_CTX, key : *mut u8, keylen : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_derive_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_derive_init_ex(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_derive_set_peer(ctx : *mut super::types::EVP_PKEY_CTX, peer : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_derive_set_peer_ex(ctx : *mut super::types::EVP_PKEY_CTX, peer : *mut super::types::EVP_PKEY, validate_peer : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_digestsign_supports_digest(pkey : *mut super::types::EVP_PKEY, libctx : *mut super::types::OSSL_LIB_CTX, name : *const i8, propq : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_dup(pkey : *mut super::types::EVP_PKEY) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_encapsulate(ctx : *mut super::types::EVP_PKEY_CTX, wrappedkey : *mut u8, wrappedkeylen : *mut usize, genkey : *mut u8, genkeylen : *mut usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_encapsulate_init(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_encrypt(ctx : *mut super::types::EVP_PKEY_CTX, out : *mut u8, outlen : *mut usize, r#in : *const u8, inlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_encrypt_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_encrypt_init_ex(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_encrypt_old(enc_key : *mut u8, key : *const u8, key_len : i32, pub_key : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_eq(a : *const super::types::EVP_PKEY, b : *const super::types::EVP_PKEY) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_export(pkey : *const super::types::EVP_PKEY, selection : i32, export_cb : super::core::OSSL_CALLBACK, export_cbarg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_free(pkey : *mut super::types::EVP_PKEY));
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_fromdata(ctx : *mut super::types::EVP_PKEY_CTX, ppkey : *mut *mut super::types::EVP_PKEY, selection : i32, param : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_fromdata_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_fromdata_settable(ctx : *mut super::types::EVP_PKEY_CTX, selection : i32) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_generate(ctx : *mut super::types::EVP_PKEY_CTX, ppkey : *mut *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0(pkey : *const super::types::EVP_PKEY) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_DH(pkey : *const super::types::EVP_PKEY) -> *const super::types::dh_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_DSA(pkey : *const super::types::EVP_PKEY) -> *const super::types::dsa_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_EC_KEY(pkey : *const super::types::EVP_PKEY) -> *const super::types::ec_key_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_RSA(pkey : *const super::types::EVP_PKEY) -> *const super::types::rsa_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_asn1(pkey : *const super::types::EVP_PKEY) -> *const super::types::EVP_PKEY_ASN1_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_description(pkey : *const super::types::EVP_PKEY) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_engine(pkey : *const super::types::EVP_PKEY) -> *mut super::types::ENGINE);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_hmac(pkey : *const super::types::EVP_PKEY, len : *mut usize) -> *const u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_poly1305(pkey : *const super::types::EVP_PKEY, len : *mut usize) -> *const u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_provider(key : *const super::types::EVP_PKEY) -> *const super::types::OSSL_PROVIDER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_siphash(pkey : *const super::types::EVP_PKEY, len : *mut usize) -> *const u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get0_type_name(key : *const super::types::EVP_PKEY) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get1_DH(pkey : *mut super::types::EVP_PKEY) -> *mut super::types::dh_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get1_DSA(pkey : *mut super::types::EVP_PKEY) -> *mut super::types::dsa_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get1_EC_KEY(pkey : *mut super::types::EVP_PKEY) -> *mut super::types::ec_key_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get1_RSA(pkey : *mut super::types::EVP_PKEY) -> *mut super::types::rsa_st);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get1_encoded_public_key(pkey : *mut super::types::EVP_PKEY, ppub : *mut *mut u8) -> usize);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_base_id(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_bits(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_bn_param(pkey : *const super::types::EVP_PKEY, key_name : *const i8, bn : *mut *mut super::types::BIGNUM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_default_digest_name(pkey : *mut super::types::EVP_PKEY, mdname : *mut i8, mdname_sz : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_default_digest_nid(pkey : *mut super::types::EVP_PKEY, pnid : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_ec_point_conv_form(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_ex_data(key : *const super::types::EVP_PKEY, idx : i32) -> *mut core::ffi::c_void);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_field_type(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_group_name(pkey : *const super::types::EVP_PKEY, name : *mut i8, name_sz : usize, gname_len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_id(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_int_param(pkey : *const super::types::EVP_PKEY, key_name : *const i8, out : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_octet_string_param(pkey : *const super::types::EVP_PKEY, key_name : *const i8, buf : *mut u8, max_buf_sz : usize, out_sz : *mut usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_params(pkey : *const super::types::EVP_PKEY, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_raw_private_key(pkey : *const super::types::EVP_PKEY, r#priv : *mut u8, len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_raw_public_key(pkey : *const super::types::EVP_PKEY, r#pub : *mut u8, len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_security_bits(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_size(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_size_t_param(pkey : *const super::types::EVP_PKEY, key_name : *const i8, out : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_get_utf8_string_param(pkey : *const super::types::EVP_PKEY, key_name : *const i8, str : *mut i8, max_buf_sz : usize, out_sz : *mut usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_gettable_params(pkey : *const super::types::EVP_PKEY) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_is_a(pkey : *const super::types::EVP_PKEY, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_keygen(ctx : *mut super::types::EVP_PKEY_CTX, ppkey : *mut *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_keygen_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_add0(pmeth : *const super::types::EVP_PKEY_METHOD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_copy(dst : *mut super::types::EVP_PKEY_METHOD, src : *const super::types::EVP_PKEY_METHOD));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_find(r#type : i32) -> *const super::types::EVP_PKEY_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_free(pmeth : *mut super::types::EVP_PKEY_METHOD));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get0(idx : usize) -> *const super::types::EVP_PKEY_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get0_info(ppkey_id : *mut i32, pflags : *mut i32, meth : *const super::types::EVP_PKEY_METHOD));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_check(pmeth : *const super::types::EVP_PKEY_METHOD, pcheck : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_cleanup(pmeth : *const super::types::EVP_PKEY_METHOD, pcleanup : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_copy(pmeth : *const super::types::EVP_PKEY_METHOD, pcopy : *mut *mut u8));
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_count() -> usize);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_ctrl(pmeth : *const super::types::EVP_PKEY_METHOD, pctrl : *mut *mut u8, pctrl_str : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_decrypt(pmeth : *const super::types::EVP_PKEY_METHOD, pdecrypt_init : *mut *mut u8, pdecrypt : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_derive(pmeth : *const super::types::EVP_PKEY_METHOD, pderive_init : *mut *mut u8, pderive : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_digest_custom(pmeth : *const super::types::EVP_PKEY_METHOD, pdigest_custom : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_digestsign(pmeth : *const super::types::EVP_PKEY_METHOD, digestsign : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_digestverify(pmeth : *const super::types::EVP_PKEY_METHOD, digestverify : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_encrypt(pmeth : *const super::types::EVP_PKEY_METHOD, pencrypt_init : *mut *mut u8, pencryptfn : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_init(pmeth : *const super::types::EVP_PKEY_METHOD, pinit : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_keygen(pmeth : *const super::types::EVP_PKEY_METHOD, pkeygen_init : *mut *mut u8, pkeygen : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_param_check(pmeth : *const super::types::EVP_PKEY_METHOD, pcheck : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_paramgen(pmeth : *const super::types::EVP_PKEY_METHOD, pparamgen_init : *mut *mut u8, pparamgen : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_public_check(pmeth : *const super::types::EVP_PKEY_METHOD, pcheck : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_sign(pmeth : *const super::types::EVP_PKEY_METHOD, psign_init : *mut *mut u8, psign : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_signctx(pmeth : *const super::types::EVP_PKEY_METHOD, psignctx_init : *mut *mut u8, psignctx : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_verify(pmeth : *const super::types::EVP_PKEY_METHOD, pverify_init : *mut *mut u8, pverify : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_verify_recover(pmeth : *const super::types::EVP_PKEY_METHOD, pverify_recover_init : *mut *mut u8, pverify_recover : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_get_verifyctx(pmeth : *const super::types::EVP_PKEY_METHOD, pverifyctx_init : *mut *mut u8, pverifyctx : *mut *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_new(id : i32, flags : i32) -> *mut super::types::EVP_PKEY_METHOD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_remove(pmeth : *const super::types::EVP_PKEY_METHOD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_check(pmeth : *mut super::types::EVP_PKEY_METHOD, check : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_cleanup(pmeth : *mut super::types::EVP_PKEY_METHOD, cleanup : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_copy(pmeth : *mut super::types::EVP_PKEY_METHOD, copy : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_ctrl(pmeth : *mut super::types::EVP_PKEY_METHOD, ctrl : *mut u8, ctrl_str : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_decrypt(pmeth : *mut super::types::EVP_PKEY_METHOD, decrypt_init : *mut u8, decrypt : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_derive(pmeth : *mut super::types::EVP_PKEY_METHOD, derive_init : *mut u8, derive : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_digest_custom(pmeth : *mut super::types::EVP_PKEY_METHOD, digest_custom : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_digestsign(pmeth : *mut super::types::EVP_PKEY_METHOD, digestsign : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_digestverify(pmeth : *mut super::types::EVP_PKEY_METHOD, digestverify : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_encrypt(pmeth : *mut super::types::EVP_PKEY_METHOD, encrypt_init : *mut u8, encryptfn : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_init(pmeth : *mut super::types::EVP_PKEY_METHOD, init : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_keygen(pmeth : *mut super::types::EVP_PKEY_METHOD, keygen_init : *mut u8, keygen : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_param_check(pmeth : *mut super::types::EVP_PKEY_METHOD, check : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_paramgen(pmeth : *mut super::types::EVP_PKEY_METHOD, paramgen_init : *mut u8, paramgen : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_public_check(pmeth : *mut super::types::EVP_PKEY_METHOD, check : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_sign(pmeth : *mut super::types::EVP_PKEY_METHOD, sign_init : *mut u8, sign : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_signctx(pmeth : *mut super::types::EVP_PKEY_METHOD, signctx_init : *mut u8, signctx : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_verify(pmeth : *mut super::types::EVP_PKEY_METHOD, verify_init : *mut u8, verify : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_verify_recover(pmeth : *mut super::types::EVP_PKEY_METHOD, verify_recover_init : *mut u8, verify_recover : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_meth_set_verifyctx(pmeth : *mut super::types::EVP_PKEY_METHOD, verifyctx_init : *mut u8, verifyctx : *mut u8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_missing_parameters(pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_new() -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_new_CMAC_key(e : *mut super::types::ENGINE, r#priv : *const u8, len : usize, cipher : *const super::types::EVP_CIPHER) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_new_mac_key(r#type : i32, e : *mut super::types::ENGINE, key : *const u8, keylen : i32) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_new_raw_private_key(r#type : i32, e : *mut super::types::ENGINE, r#priv : *const u8, len : usize) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_new_raw_private_key_ex(libctx : *mut super::types::OSSL_LIB_CTX, keytype : *const i8, propq : *const i8, r#priv : *const u8, len : usize) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_new_raw_public_key(r#type : i32, e : *mut super::types::ENGINE, r#pub : *const u8, len : usize) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_new_raw_public_key_ex(libctx : *mut super::types::OSSL_LIB_CTX, keytype : *const i8, propq : *const i8, r#pub : *const u8, len : usize) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_pairwise_check(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_param_check(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_param_check_quick(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_parameters_eq(a : *const super::types::EVP_PKEY, b : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_paramgen(ctx : *mut super::types::EVP_PKEY_CTX, ppkey : *mut *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_paramgen_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_print_params(out : *mut super::types::BIO, pkey : *const super::types::EVP_PKEY, indent : i32, pctx : *mut super::types::ASN1_PCTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_print_params_fp(fp : *mut bnd_linux_clang::libc::file::FILE, pkey : *const super::types::EVP_PKEY, indent : i32, pctx : *mut super::types::ASN1_PCTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_print_private(out : *mut super::types::BIO, pkey : *const super::types::EVP_PKEY, indent : i32, pctx : *mut super::types::ASN1_PCTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_print_private_fp(fp : *mut bnd_linux_clang::libc::file::FILE, pkey : *const super::types::EVP_PKEY, indent : i32, pctx : *mut super::types::ASN1_PCTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_print_public(out : *mut super::types::BIO, pkey : *const super::types::EVP_PKEY, indent : i32, pctx : *mut super::types::ASN1_PCTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_print_public_fp(fp : *mut bnd_linux_clang::libc::file::FILE, pkey : *const super::types::EVP_PKEY, indent : i32, pctx : *mut super::types::ASN1_PCTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_private_check(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_public_check(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_public_check_quick(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_save_parameters(pkey : *mut super::types::EVP_PKEY, mode : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set1_DH(pkey : *mut super::types::EVP_PKEY, key : *mut super::types::dh_st) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set1_DSA(pkey : *mut super::types::EVP_PKEY, key : *mut super::types::dsa_st) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set1_EC_KEY(pkey : *mut super::types::EVP_PKEY, key : *mut super::types::ec_key_st) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set1_RSA(pkey : *mut super::types::EVP_PKEY, key : *mut super::types::rsa_st) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set1_encoded_public_key(pkey : *mut super::types::EVP_PKEY, r#pub : *const u8, publen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set1_engine(pkey : *mut super::types::EVP_PKEY, e : *mut super::types::ENGINE) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_bn_param(pkey : *mut super::types::EVP_PKEY, key_name : *const i8, bn : *const super::types::BIGNUM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_ex_data(key : *mut super::types::EVP_PKEY, idx : i32, arg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_int_param(pkey : *mut super::types::EVP_PKEY, key_name : *const i8, r#in : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_octet_string_param(pkey : *mut super::types::EVP_PKEY, key_name : *const i8, buf : *const u8, bsize : usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_params(pkey : *mut super::types::EVP_PKEY, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_size_t_param(pkey : *mut super::types::EVP_PKEY, key_name : *const i8, r#in : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_type(pkey : *mut super::types::EVP_PKEY, r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_type_by_keymgmt(pkey : *mut super::types::EVP_PKEY, keymgmt : *mut super::types::EVP_KEYMGMT) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_type_str(pkey : *mut super::types::EVP_PKEY, str : *const i8, len : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_set_utf8_string_param(pkey : *mut super::types::EVP_PKEY, key_name : *const i8, str : *const i8) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_settable_params(pkey : *const super::types::EVP_PKEY) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_sign(ctx : *mut super::types::EVP_PKEY_CTX, sig : *mut u8, siglen : *mut usize, tbs : *const u8, tbslen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_sign_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_sign_init_ex(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_sign_init_ex2(ctx : *mut super::types::EVP_PKEY_CTX, algo : *mut super::types::EVP_SIGNATURE, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_sign_message_final(ctx : *mut super::types::EVP_PKEY_CTX, sig : *mut u8, siglen : *mut usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_sign_message_init(ctx : *mut super::types::EVP_PKEY_CTX, algo : *mut super::types::EVP_SIGNATURE, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_sign_message_update(ctx : *mut super::types::EVP_PKEY_CTX, r#in : *const u8, inlen : usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_todata(pkey : *const super::types::EVP_PKEY, selection : i32, params : *mut *mut super::types::OSSL_PARAM) -> i32);
windows_link::link!("crypto" "C" fn EVP_PKEY_type(r#type : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_type_names_do_all(pkey : *const super::types::EVP_PKEY, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_up_ref(pkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify(ctx : *mut super::types::EVP_PKEY_CTX, sig : *const u8, siglen : usize, tbs : *const u8, tbslen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_init_ex(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_init_ex2(ctx : *mut super::types::EVP_PKEY_CTX, algo : *mut super::types::EVP_SIGNATURE, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_message_final(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_message_init(ctx : *mut super::types::EVP_PKEY_CTX, algo : *mut super::types::EVP_SIGNATURE, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_message_update(ctx : *mut super::types::EVP_PKEY_CTX, r#in : *const u8, inlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_recover(ctx : *mut super::types::EVP_PKEY_CTX, rout : *mut u8, routlen : *mut usize, sig : *const u8, siglen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_recover_init(ctx : *mut super::types::EVP_PKEY_CTX) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_recover_init_ex(ctx : *mut super::types::EVP_PKEY_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_PKEY_verify_recover_init_ex2(ctx : *mut super::types::EVP_PKEY_CTX, algo : *mut super::types::EVP_SIGNATURE, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_Q_digest(libctx : *mut super::types::OSSL_LIB_CTX, name : *const i8, propq : *const i8, data : *const core::ffi::c_void, datalen : usize, md : *mut u8, mdlen : *mut usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_Q_mac(libctx : *mut super::types::OSSL_LIB_CTX, name : *const i8, propq : *const i8, subalg : *const i8, params : *const super::types::OSSL_PARAM, key : *const core::ffi::c_void, keylen : usize, data : *const u8, datalen : usize, out : *mut u8, outsize : usize, outlen : *mut usize) -> *mut u8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_free(ctx : *mut super::types::EVP_RAND_CTX));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_get0_rand(ctx : *mut super::types::EVP_RAND_CTX) -> *mut super::types::EVP_RAND);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_get_params(ctx : *mut super::types::EVP_RAND_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_gettable_params(ctx : *mut super::types::EVP_RAND_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_new(rand : *mut super::types::EVP_RAND, parent : *mut super::types::EVP_RAND_CTX) -> *mut super::types::EVP_RAND_CTX);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_set_params(ctx : *mut super::types::EVP_RAND_CTX, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_settable_params(ctx : *mut super::types::EVP_RAND_CTX) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_CTX_up_ref(ctx : *mut super::types::EVP_RAND_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_enable_locking(ctx : *mut super::types::EVP_RAND_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_fetch(libctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_RAND);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_free(rand : *mut super::types::EVP_RAND));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_generate(ctx : *mut super::types::EVP_RAND_CTX, out : *mut u8, outlen : usize, strength : u32, prediction_resistance : i32, addin : *const u8, addin_len : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_get0_description(md : *const super::types::EVP_RAND) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_get0_name(rand : *const super::types::EVP_RAND) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_get0_provider(rand : *const super::types::EVP_RAND) -> *const super::types::OSSL_PROVIDER);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_get_params(rand : *mut super::types::EVP_RAND, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_get_state(ctx : *mut super::types::EVP_RAND_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_get_strength(ctx : *mut super::types::EVP_RAND_CTX) -> u32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_gettable_ctx_params(rand : *const super::types::EVP_RAND) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_gettable_params(rand : *const super::types::EVP_RAND) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_instantiate(ctx : *mut super::types::EVP_RAND_CTX, strength : u32, prediction_resistance : i32, pstr : *const u8, pstr_len : usize, params : *mut super::types::OSSL_PARAM) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_is_a(rand : *const super::types::EVP_RAND, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_names_do_all(rand : *const super::types::EVP_RAND, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_nonce(ctx : *mut super::types::EVP_RAND_CTX, out : *mut u8, outlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_reseed(ctx : *mut super::types::EVP_RAND_CTX, prediction_resistance : i32, ent : *const u8, ent_len : usize, addin : *const u8, addin_len : usize) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_RAND_settable_ctx_params(rand : *const super::types::EVP_RAND) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_uninstantiate(ctx : *mut super::types::EVP_RAND_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_up_ref(rand : *mut super::types::EVP_RAND) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_RAND_verify_zeroization(ctx : *mut super::types::EVP_RAND_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, data : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_SIGNATURE);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_free(signature : *mut super::types::EVP_SIGNATURE));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_get0_description(signature : *const super::types::EVP_SIGNATURE) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_get0_name(signature : *const super::types::EVP_SIGNATURE) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_get0_provider(signature : *const super::types::EVP_SIGNATURE) -> *mut super::types::OSSL_PROVIDER);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_gettable_ctx_params(sig : *const super::types::EVP_SIGNATURE) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_is_a(signature : *const super::types::EVP_SIGNATURE, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_names_do_all(signature : *const super::types::EVP_SIGNATURE, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_settable_ctx_params(sig : *const super::types::EVP_SIGNATURE) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SIGNATURE_up_ref(signature : *mut super::types::EVP_SIGNATURE) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_do_all_provided(libctx : *mut super::types::OSSL_LIB_CTX, r#fn : *mut u8, arg : *mut core::ffi::c_void));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_fetch(ctx : *mut super::types::OSSL_LIB_CTX, algorithm : *const i8, properties : *const i8) -> *mut super::types::EVP_SKEYMGMT);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_free(keymgmt : *mut super::types::EVP_SKEYMGMT));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_get0_description(keymgmt : *const super::types::EVP_SKEYMGMT) -> *const i8);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_get0_gen_settable_params(skeymgmt : *const super::types::EVP_SKEYMGMT) -> *const super::types::OSSL_PARAM);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_get0_imp_settable_params(skeymgmt : *const super::types::EVP_SKEYMGMT) -> *const super::types::OSSL_PARAM);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_get0_name(keymgmt : *const super::types::EVP_SKEYMGMT) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_get0_provider(keymgmt : *const super::types::EVP_SKEYMGMT) -> *const super::types::OSSL_PROVIDER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_is_a(keymgmt : *const super::types::EVP_SKEYMGMT, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_names_do_all(keymgmt : *const super::types::EVP_SKEYMGMT, r#fn : *mut u8, data : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEYMGMT_up_ref(keymgmt : *mut super::types::EVP_SKEYMGMT) -> i32);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_SKEY_export(skey : *const super::types::EVP_SKEY, selection : i32, export_cb : super::core::OSSL_CALLBACK, export_cbarg : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_free(skey : *mut super::types::EVP_SKEY));
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_SKEY_generate(libctx : *mut super::types::OSSL_LIB_CTX, skeymgmtname : *const i8, propquery : *const i8, params : *const super::types::OSSL_PARAM) -> *mut super::types::EVP_SKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_get0_key_id(skey : *const super::types::EVP_SKEY) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_get0_provider_name(skey : *const super::types::EVP_SKEY) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_get0_raw_key(skey : *const super::types::EVP_SKEY, key : *mut *mut u8, len : *mut usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_get0_skeymgmt_name(skey : *const super::types::EVP_SKEY) -> *const i8);
#[cfg(all(feature = "core", feature = "types"))]
windows_link::link!("crypto" "C" fn EVP_SKEY_import(libctx : *mut super::types::OSSL_LIB_CTX, skeymgmtname : *const i8, propquery : *const i8, selection : i32, params : *const super::types::OSSL_PARAM) -> *mut super::types::EVP_SKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_import_raw_key(libctx : *mut super::types::OSSL_LIB_CTX, skeymgmtname : *const i8, key : *mut u8, keylen : usize, propquery : *const i8) -> *mut super::types::EVP_SKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_is_a(skey : *const super::types::EVP_SKEY, name : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_to_provider(skey : *mut super::types::EVP_SKEY, libctx : *mut super::types::OSSL_LIB_CTX, prov : *mut super::types::OSSL_PROVIDER, propquery : *const i8) -> *mut super::types::EVP_SKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SKEY_up_ref(skey : *mut super::types::EVP_SKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SealFinal(ctx : *mut super::types::EVP_CIPHER_CTX, out : *mut u8, outl : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SealInit(ctx : *mut super::types::EVP_CIPHER_CTX, r#type : *const super::types::EVP_CIPHER, ek : *mut *mut u8, ekl : *mut i32, iv : *mut u8, pubk : *mut *mut super::types::EVP_PKEY, npubk : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SignFinal(ctx : *mut super::types::EVP_MD_CTX, md : *mut u8, s : *mut u32, pkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_SignFinal_ex(ctx : *mut super::types::EVP_MD_CTX, md : *mut u8, s : *mut u32, pkey : *mut super::types::EVP_PKEY, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_VerifyFinal(ctx : *mut super::types::EVP_MD_CTX, sigbuf : *const u8, siglen : u32, pkey : *mut super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_VerifyFinal_ex(ctx : *mut super::types::EVP_MD_CTX, sigbuf : *const u8, siglen : u32, pkey : *mut super::types::EVP_PKEY, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
windows_link::link!("crypto" "C" fn EVP_add_alg_module());
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_add_cipher(cipher : *const super::types::EVP_CIPHER) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_add_digest(digest : *const super::types::EVP_MD) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_cbc_hmac_sha1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_cbc_hmac_sha256() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_ccm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_gcm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_ocb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_wrap() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_wrap_pad() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_128_xts() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_ccm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_gcm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_ocb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_wrap() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_192_wrap_pad() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_cbc_hmac_sha1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_cbc_hmac_sha256() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_ccm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_gcm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_ocb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_wrap() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_wrap_pad() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aes_256_xts() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_ccm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_gcm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_128_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_ccm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_gcm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_192_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_ccm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_gcm() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_aria_256_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_bf_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_bf_cfb64() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_bf_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_bf_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_blake2b512() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_blake2s256() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_128_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_128_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_128_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_128_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_128_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_128_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_128_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_192_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_192_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_192_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_192_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_192_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_192_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_192_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_256_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_256_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_256_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_256_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_256_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_256_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_camellia_256_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_cast5_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_cast5_cfb64() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_cast5_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_cast5_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_chacha20() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_chacha20_poly1305() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_default_properties_enable_fips(libctx : *mut super::types::OSSL_LIB_CTX, enable : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_default_properties_is_fips_enabled(libctx : *mut super::types::OSSL_LIB_CTX) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_cfb64() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3_cfb1() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3_cfb64() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3_cfb8() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede3_wrap() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede_cfb64() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ede_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_des_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_desx_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_enc_null() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_get1_default_properties(libctx : *mut super::types::OSSL_LIB_CTX) -> *mut i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_get_cipherbyname(name : *const i8) -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_get_digestbyname(name : *const i8) -> *const super::types::EVP_MD);
windows_link::link!("crypto" "C" fn EVP_get_pw_prompt() -> *mut i8);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_md4() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_md5() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_md5_sha1() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_md_null() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc2_40_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc2_64_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc2_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc2_cfb64() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc2_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc2_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc4() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc4_40() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_rc4_hmac_md5() -> *const super::types::EVP_CIPHER);
windows_link::link!("crypto" "C" fn EVP_read_pw_string(buf : *mut i8, length : i32, prompt : *const i8, verify : i32) -> i32);
windows_link::link!("crypto" "C" fn EVP_read_pw_string_min(buf : *mut i8, minlen : i32, maxlen : i32, prompt : *const i8, verify : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_ripemd160() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_seed_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_seed_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_seed_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_seed_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_set_default_properties(libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
windows_link::link!("crypto" "C" fn EVP_set_pw_prompt(prompt : *const i8));
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha1() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha224() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha256() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha384() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha3_224() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha3_256() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha3_384() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha3_512() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha512() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha512_224() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sha512_256() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_shake128() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_shake256() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sm3() -> *const super::types::EVP_MD);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sm4_cbc() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sm4_cfb128() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sm4_ctr() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sm4_ecb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_sm4_ofb() -> *const super::types::EVP_CIPHER);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn EVP_whirlpool() -> *const super::types::EVP_MD);
windows_link::link!("crypto" "C" fn PKCS5_PBE_add());
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn PKCS5_PBE_keyivgen(ctx : *mut super::types::EVP_CIPHER_CTX, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, cipher : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, en_de : i32) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn PKCS5_PBE_keyivgen_ex(cctx : *mut super::types::EVP_CIPHER_CTX, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, cipher : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, en_de : i32, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn PKCS5_PBKDF2_HMAC(pass : *const i8, passlen : i32, salt : *const u8, saltlen : i32, iter : i32, digest : *const super::types::EVP_MD, keylen : i32, out : *mut u8) -> i32);
windows_link::link!("crypto" "C" fn PKCS5_PBKDF2_HMAC_SHA1(pass : *const i8, passlen : i32, salt : *const u8, saltlen : i32, iter : i32, keylen : i32, out : *mut u8) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn PKCS5_v2_PBE_keyivgen(ctx : *mut super::types::EVP_CIPHER_CTX, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, cipher : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, en_de : i32) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn PKCS5_v2_PBE_keyivgen_ex(ctx : *mut super::types::EVP_CIPHER_CTX, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, cipher : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, en_de : i32, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn PKCS5_v2_scrypt_keyivgen(ctx : *mut super::types::EVP_CIPHER_CTX, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, c : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, en_de : i32) -> i32);
#[cfg(all(feature = "asn1", feature = "types"))]
windows_link::link!("crypto" "C" fn PKCS5_v2_scrypt_keyivgen_ex(ctx : *mut super::types::EVP_CIPHER_CTX, pass : *const i8, passlen : i32, param : *mut super::types::ASN1_TYPE, c : *const super::types::EVP_CIPHER, md : *const super::types::EVP_MD, en_de : i32, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn d2i_AutoPrivateKey(a : *mut *mut super::types::EVP_PKEY, pp : *mut *mut u8, length : i64) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn d2i_AutoPrivateKey_ex(a : *mut *mut super::types::EVP_PKEY, pp : *mut *mut u8, length : i64, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn d2i_KeyParams(r#type : i32, a : *mut *mut super::types::EVP_PKEY, pp : *mut *mut u8, length : i64) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn d2i_KeyParams_bio(r#type : i32, a : *mut *mut super::types::EVP_PKEY, r#in : *mut super::types::BIO) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn d2i_PrivateKey(r#type : i32, a : *mut *mut super::types::EVP_PKEY, pp : *mut *mut u8, length : i64) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn d2i_PrivateKey_ex(r#type : i32, a : *mut *mut super::types::EVP_PKEY, pp : *mut *mut u8, length : i64, libctx : *mut super::types::OSSL_LIB_CTX, propq : *const i8) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn d2i_PublicKey(r#type : i32, a : *mut *mut super::types::EVP_PKEY, pp : *mut *mut u8, length : i64) -> *mut super::types::EVP_PKEY);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn i2d_KeyParams(a : *const super::types::EVP_PKEY, pp : *mut *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn i2d_KeyParams_bio(bp : *mut super::types::BIO, pkey : *const super::types::EVP_PKEY) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn i2d_PrivateKey(a : *const super::types::EVP_PKEY, pp : *mut *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("crypto" "C" fn i2d_PublicKey(a : *const super::types::EVP_PKEY, pp : *mut *mut u8) -> i32);
pub const ASN1_PKEY_ALIAS: i32 = 1;
pub const ASN1_PKEY_CTRL_CMS_ENVELOPE: i32 = 7;
pub const ASN1_PKEY_CTRL_CMS_IS_RI_TYPE_SUPPORTED: i32 = 11;
pub const ASN1_PKEY_CTRL_CMS_RI_TYPE: i32 = 8;
pub const ASN1_PKEY_CTRL_CMS_SIGN: i32 = 5;
pub const ASN1_PKEY_CTRL_DEFAULT_MD_NID: i32 = 3;
pub const ASN1_PKEY_CTRL_GET1_TLS_ENCPT: i32 = 10;
pub const ASN1_PKEY_CTRL_PKCS7_ENCRYPT: i32 = 2;
pub const ASN1_PKEY_CTRL_PKCS7_SIGN: i32 = 1;
pub const ASN1_PKEY_CTRL_SET1_TLS_ENCPT: i32 = 9;
pub const ASN1_PKEY_DYNAMIC: i32 = 2;
pub const ASN1_PKEY_SIGPARAM_NULL: i32 = 4;
pub const EVP_AEAD_TLS1_AAD_LEN: i32 = 13;
pub const EVP_CCM8_TLS_TAG_LEN: i32 = 8;
pub const EVP_CCM_TLS_EXPLICIT_IV_LEN: i32 = 8;
pub const EVP_CCM_TLS_FIXED_IV_LEN: i32 = 4;
pub const EVP_CCM_TLS_IV_LEN: i32 = 12;
pub const EVP_CCM_TLS_TAG_LEN: i32 = 16;
pub const EVP_CHACHAPOLY_TLS_TAG_LEN: i32 = 16;
pub const EVP_CIPHER_CTX_FLAG_WRAP_ALLOW: i32 = 1;
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct EVP_CIPHER_INFO {
    pub cipher: *const super::types::EVP_CIPHER,
    pub iv: [u8; 16],
}
#[cfg(feature = "types")]
impl Default for EVP_CIPHER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVP_CIPH_ALWAYS_CALL_INIT: i32 = 32;
pub const EVP_CIPH_CBC_MODE: i32 = 2;
pub const EVP_CIPH_CCM_MODE: i32 = 7;
pub const EVP_CIPH_CFB_MODE: i32 = 3;
pub const EVP_CIPH_CTRL_INIT: i32 = 64;
pub const EVP_CIPH_CTR_MODE: i32 = 5;
pub const EVP_CIPH_CUSTOM_COPY: i32 = 1024;
pub const EVP_CIPH_CUSTOM_IV: i32 = 16;
pub const EVP_CIPH_CUSTOM_IV_LENGTH: i32 = 2048;
pub const EVP_CIPH_CUSTOM_KEY_LENGTH: i32 = 128;
pub const EVP_CIPH_ECB_MODE: i32 = 1;
pub const EVP_CIPH_FLAG_AEAD_CIPHER: i32 = 2097152;
pub const EVP_CIPH_FLAG_CIPHER_WITH_MAC: i32 = 33554432;
pub const EVP_CIPH_FLAG_CTS: i32 = 16384;
pub const EVP_CIPH_FLAG_CUSTOM_ASN1: i32 = 16777216;
pub const EVP_CIPH_FLAG_CUSTOM_CIPHER: i32 = 1048576;
pub const EVP_CIPH_FLAG_DEFAULT_ASN1: i32 = 0;
pub const EVP_CIPH_FLAG_FIPS: i32 = 0;
pub const EVP_CIPH_FLAG_GET_WRAP_CIPHER: i32 = 67108864;
pub const EVP_CIPH_FLAG_INVERSE_CIPHER: i32 = 134217728;
pub const EVP_CIPH_FLAG_LENGTH_BITS: i32 = 8192;
pub const EVP_CIPH_FLAG_NON_FIPS_ALLOW: i32 = 0;
pub const EVP_CIPH_FLAG_PIPELINE: i32 = 8388608;
pub const EVP_CIPH_FLAG_TLS1_1_MULTIBLOCK: i32 = 4194304;
pub const EVP_CIPH_GCM_MODE: i32 = 6;
pub const EVP_CIPH_GCM_SIV_MODE: i32 = 65541;
pub const EVP_CIPH_MODE: i32 = 983047;
pub const EVP_CIPH_NO_PADDING: i32 = 256;
pub const EVP_CIPH_OCB_MODE: i32 = 65539;
pub const EVP_CIPH_OFB_MODE: i32 = 4;
pub const EVP_CIPH_RAND_KEY: i32 = 512;
pub const EVP_CIPH_SIV_MODE: i32 = 65540;
pub const EVP_CIPH_STREAM_CIPHER: i32 = 0;
pub const EVP_CIPH_VARIABLE_LENGTH: i32 = 8;
pub const EVP_CIPH_WRAP_MODE: i32 = 65538;
pub const EVP_CIPH_XTS_MODE: i32 = 65537;
pub const EVP_CTRL_AEAD_GET_TAG: i32 = 16;
pub const EVP_CTRL_AEAD_SET_IVLEN: i32 = 9;
pub const EVP_CTRL_AEAD_SET_IV_FIXED: i32 = 18;
pub const EVP_CTRL_AEAD_SET_MAC_KEY: i32 = 23;
pub const EVP_CTRL_AEAD_SET_TAG: i32 = 17;
pub const EVP_CTRL_AEAD_TLS1_AAD: i32 = 22;
pub const EVP_CTRL_BLOCK_PADDING_MODE: i32 = 33;
pub const EVP_CTRL_CCM_GET_TAG: i32 = 16;
pub const EVP_CTRL_CCM_SET_IVLEN: i32 = 9;
pub const EVP_CTRL_CCM_SET_IV_FIXED: i32 = 18;
pub const EVP_CTRL_CCM_SET_L: i32 = 20;
pub const EVP_CTRL_CCM_SET_MSGLEN: i32 = 21;
pub const EVP_CTRL_CCM_SET_TAG: i32 = 17;
pub const EVP_CTRL_COPY: i32 = 8;
pub const EVP_CTRL_GCM_GET_TAG: i32 = 16;
pub const EVP_CTRL_GCM_IV_GEN: i32 = 19;
pub const EVP_CTRL_GCM_SET_IVLEN: i32 = 9;
pub const EVP_CTRL_GCM_SET_IV_FIXED: i32 = 18;
pub const EVP_CTRL_GCM_SET_IV_INV: i32 = 24;
pub const EVP_CTRL_GCM_SET_TAG: i32 = 17;
pub const EVP_CTRL_GET_IVLEN: i32 = 37;
pub const EVP_CTRL_GET_RC2_KEY_BITS: i32 = 2;
pub const EVP_CTRL_GET_RC5_ROUNDS: i32 = 4;
pub const EVP_CTRL_GET_WRAP_CIPHER: i32 = 41;
pub const EVP_CTRL_INIT: i32 = 0;
pub const EVP_CTRL_KEY_MESH: i32 = 32;
pub const EVP_CTRL_PBE_PRF_NID: i32 = 7;
pub const EVP_CTRL_PROCESS_UNPROTECTED: i32 = 40;
pub const EVP_CTRL_RAND_KEY: i32 = 6;
pub const EVP_CTRL_SBOX_USED: i32 = 31;
pub const EVP_CTRL_SET_KEY_LENGTH: i32 = 1;
pub const EVP_CTRL_SET_PIPELINE_INPUT_BUFS: i32 = 35;
pub const EVP_CTRL_SET_PIPELINE_INPUT_LENS: i32 = 36;
pub const EVP_CTRL_SET_PIPELINE_OUTPUT_BUFS: i32 = 34;
pub const EVP_CTRL_SET_RC2_KEY_BITS: i32 = 3;
pub const EVP_CTRL_SET_RC5_ROUNDS: i32 = 5;
pub const EVP_CTRL_SET_SBOX: i32 = 30;
pub const EVP_CTRL_SET_SPEED: i32 = 39;
pub const EVP_CTRL_SSL3_MASTER_SECRET: i32 = 29;
pub const EVP_CTRL_TLS1_1_MULTIBLOCK_AAD: i32 = 25;
pub const EVP_CTRL_TLS1_1_MULTIBLOCK_DECRYPT: i32 = 27;
pub const EVP_CTRL_TLS1_1_MULTIBLOCK_ENCRYPT: i32 = 26;
pub const EVP_CTRL_TLS1_1_MULTIBLOCK_MAX_BUFSIZE: i32 = 28;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EVP_CTRL_TLS1_1_MULTIBLOCK_PARAM {
    pub out: *mut u8,
    pub inp: *const u8,
    pub len: usize,
    pub interleave: u32,
}
pub const EVP_CTRL_TLSTREE: i32 = 42;
pub const EVP_GCM_TLS_EXPLICIT_IV_LEN: i32 = 8;
pub const EVP_GCM_TLS_FIXED_IV_LEN: i32 = 4;
pub const EVP_GCM_TLS_TAG_LEN: i32 = 16;
pub const EVP_MAX_AEAD_TAG_LENGTH: i32 = 16;
pub const EVP_MAX_BLOCK_LENGTH: i32 = 32;
pub const EVP_MAX_IV_LENGTH: i32 = 16;
pub const EVP_MAX_KEY_LENGTH: i32 = 64;
pub const EVP_MAX_MD_SIZE: i32 = 64;
pub const EVP_MAX_PIPES: i32 = 32;
pub const EVP_MD_CTRL_ALG_CTRL: i32 = 4096;
pub const EVP_MD_CTRL_DIGALGID: i32 = 1;
pub const EVP_MD_CTRL_MICALG: i32 = 2;
pub const EVP_MD_CTRL_TLSTREE: i32 = 4;
pub const EVP_MD_CTRL_XOF_LEN: i32 = 3;
pub const EVP_MD_CTX_FLAG_CLEANED: i32 = 2;
pub const EVP_MD_CTX_FLAG_FINALISE: i32 = 512;
pub const EVP_MD_CTX_FLAG_NON_FIPS_ALLOW: i32 = 8;
pub const EVP_MD_CTX_FLAG_NO_INIT: i32 = 256;
pub const EVP_MD_CTX_FLAG_ONESHOT: i32 = 1;
pub const EVP_MD_CTX_FLAG_PAD_MASK: i32 = 240;
pub const EVP_MD_CTX_FLAG_PAD_PKCS1: i32 = 0;
pub const EVP_MD_CTX_FLAG_PAD_PSS: i32 = 32;
pub const EVP_MD_CTX_FLAG_PAD_X931: i32 = 16;
pub const EVP_MD_CTX_FLAG_REUSE: i32 = 4;
pub const EVP_MD_FLAG_DIGALGID_ABSENT: i32 = 8;
pub const EVP_MD_FLAG_DIGALGID_CUSTOM: i32 = 24;
pub const EVP_MD_FLAG_DIGALGID_MASK: i32 = 24;
pub const EVP_MD_FLAG_DIGALGID_NULL: i32 = 0;
pub const EVP_MD_FLAG_FIPS: i32 = 1024;
pub const EVP_MD_FLAG_ONESHOT: i32 = 1;
pub const EVP_MD_FLAG_XOF: i32 = 2;
pub const EVP_PADDING_ANSI923: i32 = 3;
pub const EVP_PADDING_ISO10126: i32 = 4;
pub const EVP_PADDING_ISO7816_4: i32 = 2;
pub const EVP_PADDING_PKCS7: i32 = 1;
pub const EVP_PADDING_ZERO: i32 = 5;
#[cfg(all(feature = "asn1", feature = "types"))]
pub type EVP_PBE_KEYGEN = Option<
    unsafe extern "C" fn(
        ctx: *mut super::types::EVP_CIPHER_CTX,
        pass: *const i8,
        passlen: i32,
        param: *mut super::types::ASN1_TYPE,
        cipher: *const super::types::EVP_CIPHER,
        md: *const super::types::EVP_MD,
        en_de: i32,
    ) -> i32,
>;
#[cfg(all(feature = "asn1", feature = "types"))]
pub type EVP_PBE_KEYGEN_EX = Option<
    unsafe extern "C" fn(
        ctx: *mut super::types::EVP_CIPHER_CTX,
        pass: *const i8,
        passlen: i32,
        param: *mut super::types::ASN1_TYPE,
        cipher: *const super::types::EVP_CIPHER,
        md: *const super::types::EVP_MD,
        en_de: i32,
        libctx: *mut super::types::OSSL_LIB_CTX,
        propq: *const i8,
    ) -> i32,
>;
pub const EVP_PBE_TYPE_KDF: i32 = 2;
pub const EVP_PBE_TYPE_OUTER: i32 = 0;
pub const EVP_PBE_TYPE_PRF: i32 = 1;
pub const EVP_PKEY_ALG_CTRL: i32 = 4096;
pub const EVP_PKEY_CMAC: i32 = 894;
pub const EVP_PKEY_CTRL_CIPHER: i32 = 12;
pub const EVP_PKEY_CTRL_CMS_DECRYPT: i32 = 10;
pub const EVP_PKEY_CTRL_CMS_ENCRYPT: i32 = 9;
pub const EVP_PKEY_CTRL_CMS_SIGN: i32 = 11;
pub const EVP_PKEY_CTRL_DIGESTINIT: i32 = 7;
pub const EVP_PKEY_CTRL_GET1_ID: i32 = 16;
pub const EVP_PKEY_CTRL_GET1_ID_LEN: i32 = 17;
pub const EVP_PKEY_CTRL_GET_MD: i32 = 13;
pub const EVP_PKEY_CTRL_MD: i32 = 1;
pub const EVP_PKEY_CTRL_PEER_KEY: i32 = 2;
pub const EVP_PKEY_CTRL_PKCS7_DECRYPT: i32 = 4;
pub const EVP_PKEY_CTRL_PKCS7_ENCRYPT: i32 = 3;
pub const EVP_PKEY_CTRL_PKCS7_SIGN: i32 = 5;
pub const EVP_PKEY_CTRL_SET1_ID: i32 = 15;
pub const EVP_PKEY_CTRL_SET_DIGEST_SIZE: i32 = 14;
pub const EVP_PKEY_CTRL_SET_IV: i32 = 8;
pub const EVP_PKEY_CTRL_SET_MAC_KEY: i32 = 6;
pub const EVP_PKEY_DH: i32 = 28;
pub const EVP_PKEY_DHX: i32 = 920;
pub const EVP_PKEY_DSA: i32 = 116;
pub const EVP_PKEY_DSA1: i32 = 67;
pub const EVP_PKEY_DSA2: i32 = 66;
pub const EVP_PKEY_DSA3: i32 = 113;
pub const EVP_PKEY_DSA4: i32 = 70;
pub const EVP_PKEY_EC: i32 = 408;
pub const EVP_PKEY_ED25519: i32 = 1087;
pub const EVP_PKEY_ED448: i32 = 1088;
pub const EVP_PKEY_FLAG_AUTOARGLEN: i32 = 2;
pub const EVP_PKEY_FLAG_SIGCTX_CUSTOM: i32 = 4;
pub const EVP_PKEY_HKDF: i32 = 1036;
pub const EVP_PKEY_HMAC: i32 = 855;
pub const EVP_PKEY_KEYMGMT: i32 = -1;
pub const EVP_PKEY_KEYPAIR: i32 = 135;
pub const EVP_PKEY_KEY_PARAMETERS: i32 = 132;
pub const EVP_PKEY_ML_DSA_44: i32 = 1457;
pub const EVP_PKEY_ML_DSA_65: i32 = 1458;
pub const EVP_PKEY_ML_DSA_87: i32 = 1459;
pub const EVP_PKEY_MO_DECRYPT: i32 = 8;
pub const EVP_PKEY_MO_ENCRYPT: i32 = 4;
pub const EVP_PKEY_MO_SIGN: i32 = 1;
pub const EVP_PKEY_MO_VERIFY: i32 = 2;
pub const EVP_PKEY_NONE: i32 = 0;
pub const EVP_PKEY_OP_ALL: i32 = 65535;
pub const EVP_PKEY_OP_DECAPSULATE: i32 = 8192;
pub const EVP_PKEY_OP_DECRYPT: i32 = 1024;
pub const EVP_PKEY_OP_DERIVE: i32 = 2048;
pub const EVP_PKEY_OP_ENCAPSULATE: i32 = 4096;
pub const EVP_PKEY_OP_ENCRYPT: i32 = 512;
pub const EVP_PKEY_OP_FROMDATA: i32 = 8;
pub const EVP_PKEY_OP_KEYGEN: i32 = 4;
pub const EVP_PKEY_OP_PARAMGEN: i32 = 2;
pub const EVP_PKEY_OP_SIGN: i32 = 16;
pub const EVP_PKEY_OP_SIGNCTX: i32 = 128;
pub const EVP_PKEY_OP_SIGNMSG: i32 = 16384;
pub const EVP_PKEY_OP_TYPE_CRYPT: i32 = 1536;
pub const EVP_PKEY_OP_TYPE_DATA: i32 = 8;
pub const EVP_PKEY_OP_TYPE_DERIVE: i32 = 2048;
pub const EVP_PKEY_OP_TYPE_GEN: i32 = 6;
pub const EVP_PKEY_OP_TYPE_KEM: i32 = 12288;
pub const EVP_PKEY_OP_TYPE_NOGEN: i32 = 65529;
pub const EVP_PKEY_OP_TYPE_SIG: i32 = 49648;
pub const EVP_PKEY_OP_UNDEFINED: i32 = 0;
pub const EVP_PKEY_OP_VERIFY: i32 = 32;
pub const EVP_PKEY_OP_VERIFYCTX: i32 = 256;
pub const EVP_PKEY_OP_VERIFYMSG: i32 = 32768;
pub const EVP_PKEY_OP_VERIFYRECOVER: i32 = 64;
pub const EVP_PKEY_POLY1305: i32 = 1061;
pub const EVP_PKEY_PRIVATE_KEY: i32 = 133;
pub const EVP_PKEY_PUBLIC_KEY: i32 = 134;
pub const EVP_PKEY_RSA: i32 = 6;
pub const EVP_PKEY_RSA2: i32 = 19;
pub const EVP_PKEY_RSA_PSS: i32 = 912;
pub const EVP_PKEY_SCRYPT: i32 = 973;
pub const EVP_PKEY_SIPHASH: i32 = 1062;
pub const EVP_PKEY_SLH_DSA_SHA2_128F: i32 = 1461;
pub const EVP_PKEY_SLH_DSA_SHA2_128S: i32 = 1460;
pub const EVP_PKEY_SLH_DSA_SHA2_192F: i32 = 1463;
pub const EVP_PKEY_SLH_DSA_SHA2_192S: i32 = 1462;
pub const EVP_PKEY_SLH_DSA_SHA2_256F: i32 = 1465;
pub const EVP_PKEY_SLH_DSA_SHA2_256S: i32 = 1464;
pub const EVP_PKEY_SLH_DSA_SHAKE_128F: i32 = 1467;
pub const EVP_PKEY_SLH_DSA_SHAKE_128S: i32 = 1466;
pub const EVP_PKEY_SLH_DSA_SHAKE_192F: i32 = 1469;
pub const EVP_PKEY_SLH_DSA_SHAKE_192S: i32 = 1468;
pub const EVP_PKEY_SLH_DSA_SHAKE_256F: i32 = 1471;
pub const EVP_PKEY_SLH_DSA_SHAKE_256S: i32 = 1470;
pub const EVP_PKEY_SM2: i32 = 1172;
pub const EVP_PKEY_TLS1_PRF: i32 = 1021;
pub const EVP_PKEY_X25519: i32 = 1034;
pub const EVP_PKEY_X448: i32 = 1035;
#[cfg(feature = "types")]
pub type EVP_PKEY_gen_cb =
    Option<unsafe extern "C" fn(ctx: *mut super::types::EVP_PKEY_CTX) -> i32>;
pub const EVP_PKS_DSA: i32 = 512;
pub const EVP_PKS_EC: i32 = 1024;
pub const EVP_PKS_RSA: i32 = 256;
pub const EVP_PKT_ENC: i32 = 32;
pub const EVP_PKT_EXCH: i32 = 64;
pub const EVP_PKT_SIGN: i32 = 16;
pub const EVP_PK_DH: i32 = 4;
pub const EVP_PK_DSA: i32 = 2;
pub const EVP_PK_EC: i32 = 8;
pub const EVP_PK_RSA: i32 = 1;
pub const EVP_RAND_STATE_ERROR: i32 = 2;
pub const EVP_RAND_STATE_READY: i32 = 1;
pub const EVP_RAND_STATE_UNINITIALISED: i32 = 0;
pub const PKCS5_DEFAULT_ITER: i32 = 2048;
pub const PKCS5_SALT_LEN: i32 = 8;
