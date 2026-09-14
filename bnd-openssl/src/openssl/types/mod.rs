#[cfg(feature = "asn1")]
pub type ASN1_BIT_STRING = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_BMPSTRING = super::asn1::asn1_string_st;
pub type ASN1_BOOLEAN = i32;
#[cfg(feature = "asn1")]
pub type ASN1_ENUMERATED = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_GENERALIZEDTIME = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_GENERALSTRING = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_IA5STRING = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_INTEGER = super::asn1::asn1_string_st;
pub type ASN1_ITEM = ASN1_ITEM_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ASN1_ITEM_st(pub u8);
pub type ASN1_NULL = i32;
pub type ASN1_OBJECT = asn1_object_st;
#[cfg(feature = "asn1")]
pub type ASN1_OCTET_STRING = super::asn1::asn1_string_st;
pub type ASN1_PCTX = asn1_pctx_st;
#[cfg(feature = "asn1")]
pub type ASN1_PRINTABLESTRING = super::asn1::asn1_string_st;
pub type ASN1_SCTX = asn1_sctx_st;
#[cfg(feature = "asn1")]
pub type ASN1_STRING = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_STRING_TABLE = super::asn1::asn1_string_table_st;
#[cfg(feature = "asn1")]
pub type ASN1_T61STRING = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_TIME = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_TYPE = super::asn1::asn1_type_st;
#[cfg(feature = "asn1")]
pub type ASN1_UNIVERSALSTRING = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_UTCTIME = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_UTF8STRING = super::asn1::asn1_string_st;
#[cfg(feature = "asn1")]
pub type ASN1_VISIBLESTRING = super::asn1::asn1_string_st;
pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUTHORITY_KEYID_st(pub u8);
pub type BIGNUM = bignum_st;
pub type BIO = bio_st;
pub type BN_BLINDING = bn_blinding_st;
pub type BN_CTX = bignum_ctx;
pub type BN_GENCB = bn_gencb_st;
pub type BN_MONT_CTX = bn_mont_ctx_st;
pub type BN_RECP_CTX = bn_recp_ctx_st;
#[cfg(feature = "buffer")]
pub type BUF_MEM = super::buffer::buf_mem_st;
pub type COMP_CTX = comp_ctx_st;
pub type COMP_METHOD = comp_method_st;
#[cfg(all(feature = "conf", feature = "conftypes"))]
pub type CONF = super::conftypes::conf_st;
#[cfg(feature = "crypto")]
pub type CRYPTO_EX_DATA = super::crypto::crypto_ex_data_st;
pub type CTLOG = ctlog_st;
pub type CTLOG_STORE = ctlog_store_st;
pub type CT_POLICY_EVAL_CTX = ct_policy_eval_ctx_st;
pub type DH = dh_st;
pub type DH_METHOD = dh_method;
pub type DIST_POINT = DIST_POINT_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DIST_POINT_st(pub u8);
pub type DSA = dsa_st;
pub type DSA_METHOD = dsa_method;
pub type EC_KEY = ec_key_st;
pub type EC_KEY_METHOD = ec_key_method_st;
pub type ENGINE = engine_st;
#[cfg(feature = "err")]
pub type ERR_STATE = super::err::err_state_st;
pub type EVP_ASYM_CIPHER = evp_asym_cipher_st;
pub type EVP_CIPHER = evp_cipher_st;
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
pub type EVP_ENCODE_CTX = evp_Encode_Ctx_st;
pub type EVP_KDF = evp_kdf_st;
pub type EVP_KDF_CTX = evp_kdf_ctx_st;
pub type EVP_KEM = evp_kem_st;
pub type EVP_KEYEXCH = evp_keyexch_st;
pub type EVP_KEYMGMT = evp_keymgmt_st;
pub type EVP_MAC = evp_mac_st;
pub type EVP_MAC_CTX = evp_mac_ctx_st;
pub type EVP_MD = evp_md_st;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type EVP_PKEY = evp_pkey_st;
pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type EVP_PKEY_METHOD = evp_pkey_method_st;
pub type EVP_RAND = evp_rand_st;
pub type EVP_RAND_CTX = evp_rand_ctx_st;
pub type EVP_SIGNATURE = evp_signature_st;
pub type EVP_SKEY = evp_skey_st;
pub type EVP_SKEYMGMT = evp_skeymgmt_st;
pub type HMAC_CTX = hmac_ctx_st;
pub type ISSUING_DIST_POINT = ISSUING_DIST_POINT_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ISSUING_DIST_POINT_st(pub u8);
pub type NAME_CONSTRAINTS = NAME_CONSTRAINTS_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NAME_CONSTRAINTS_st(pub u8);
pub type OCSP_RESPID = ocsp_responder_id_st;
pub type OCSP_RESPONSE = ocsp_response_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
#[cfg(feature = "core")]
pub type OSSL_ALGORITHM = super::core::ossl_algorithm_st;
pub type OSSL_DECODER = ossl_decoder_st;
pub type OSSL_DECODER_CTX = ossl_decoder_ctx_st;
#[cfg(feature = "core")]
pub type OSSL_DISPATCH = super::core::ossl_dispatch_st;
pub type OSSL_ENCODER = ossl_encoder_st;
pub type OSSL_ENCODER_CTX = ossl_encoder_ctx_st;
pub type OSSL_HTTP_REQ_CTX = ossl_http_req_ctx_st;
#[cfg(feature = "core")]
pub type OSSL_ITEM = super::core::ossl_item_st;
pub type OSSL_LIB_CTX = ossl_lib_ctx_st;
#[cfg(feature = "core")]
pub type OSSL_PARAM = super::core::ossl_param_st;
pub type OSSL_PARAM_BLD = ossl_param_bld_st;
pub type OSSL_PROVIDER = ossl_provider_st;
pub type OSSL_SELF_TEST = ossl_self_test_st;
pub type OSSL_STORE_INFO = ossl_store_info_st;
pub type OSSL_STORE_SEARCH = ossl_store_search_st;
pub type PKCS8_PRIV_KEY_INFO = pkcs8_priv_key_info_st;
pub type RAND_DRBG = rand_drbg_st;
#[cfg(feature = "rand")]
pub type RAND_METHOD = super::rand::rand_meth_st;
pub type RSA = rsa_st;
pub type RSA_METHOD = rsa_meth_st;
#[cfg(all(feature = "asn1", feature = "rsa", feature = "x509"))]
pub type RSA_PSS_PARAMS = super::rsa::rsa_pss_params_st;
pub type SCT = sct_st;
pub type SCT_CTX = sct_ctx_st;
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
pub type SSL_DANE = ssl_dane_st;
pub type UI = ui_st;
pub type UI_METHOD = ui_method_st;
pub type X509 = x509_st;
pub type X509V3_CTX = v3_ext_ctx;
#[cfg(all(feature = "asn1", feature = "x509"))]
pub type X509_ALGOR = super::x509::X509_algor_st;
pub type X509_CRL = X509_crl_st;
pub type X509_CRL_METHOD = x509_crl_method_st;
pub type X509_LOOKUP = x509_lookup_st;
pub type X509_LOOKUP_METHOD = x509_lookup_method_st;
pub type X509_NAME = X509_name_st;
pub type X509_OBJECT = x509_object_st;
pub type X509_POLICY_CACHE = X509_POLICY_CACHE_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_POLICY_CACHE_st(pub u8);
pub type X509_POLICY_LEVEL = X509_POLICY_LEVEL_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_POLICY_LEVEL_st(pub u8);
pub type X509_POLICY_NODE = X509_POLICY_NODE_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_POLICY_NODE_st(pub u8);
pub type X509_POLICY_TREE = X509_POLICY_TREE_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_POLICY_TREE_st(pub u8);
pub type X509_PUBKEY = X509_pubkey_st;
pub type X509_REVOKED = x509_revoked_st;
pub type X509_SIG_INFO = x509_sig_info_st;
pub type X509_STORE = x509_store_st;
pub type X509_STORE_CTX = x509_store_ctx_st;
pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_VERIFY_PARAM_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_crl_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_name_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct X509_pubkey_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct asn1_object_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct asn1_pctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct asn1_sctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct bignum_ctx(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct bignum_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct bio_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct bn_blinding_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct bn_gencb_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct bn_mont_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct bn_recp_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct comp_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct comp_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ct_policy_eval_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ctlog_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ctlog_store_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct dh_method(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct dh_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct dsa_method(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct dsa_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ec_key_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ec_key_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct engine_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_Encode_Ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_asym_cipher_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_cipher_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_cipher_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_kdf_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_kdf_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_kem_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_keyexch_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_keymgmt_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_mac_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_mac_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_md_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_md_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_pkey_asn1_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_pkey_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_pkey_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_pkey_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_rand_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_rand_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_signature_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_skey_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct evp_skeymgmt_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct hmac_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ocsp_responder_id_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ocsp_response_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_decoder_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_decoder_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_encoder_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_encoder_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_http_req_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_init_settings_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_lib_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_param_bld_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_provider_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_self_test_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_store_info_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ossl_store_search_st(pub u8);
pub type pem_password_cb = Option<
    unsafe extern "C" fn(
        buf: *mut i8,
        size: i32,
        rwflag: i32,
        userdata: *mut core::ffi::c_void,
    ) -> i32,
>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct pkcs8_priv_key_info_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct rand_drbg_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct rsa_meth_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct rsa_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct sct_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct sct_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_dane_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stack_st_BIGNUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stack_st_BIGNUM_const(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ui_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ui_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct v3_ext_ctx(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_crl_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_lookup_method_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_lookup_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_object_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_revoked_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_sig_info_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_store_ctx_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct x509_store_st(pub u8);
