#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_tlsext_max_fragment_length(ctx : *mut super::types::SSL_CTX, mode : u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_CTX_set_tlsext_ticket_key_evp_cb(ctx : *mut super::types::SSL_CTX, fp : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_check_chain(s : *mut super::types::SSL, x : *mut super::types::X509, pk : *mut super::types::EVP_PKEY, chain : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_export_keying_material(s : *mut super::types::SSL, out : *mut u8, olen : usize, label : *const i8, llen : usize, context : *const u8, contextlen : usize, use_context : i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_export_keying_material_early(s : *mut super::types::SSL, out : *mut u8, olen : usize, label : *const i8, llen : usize, context : *const u8, contextlen : usize) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get1_builtin_sigalgs(libctx : *mut super::types::OSSL_LIB_CTX) -> *mut i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_peer_signature_type_nid(s : *const super::types::SSL, pnid : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_servername(s : *const super::types::SSL, r#type : i32) -> *const i8);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_servername_type(s : *const super::types::SSL) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_shared_sigalgs(s : *mut super::types::SSL, idx : i32, psign : *mut i32, phash : *mut i32, psignandhash : *mut i32, rsig : *mut u8, rhash : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_sigalgs(s : *mut super::types::SSL, idx : i32, psign : *mut i32, phash : *mut i32, psignandhash : *mut i32, rsig : *mut u8, rhash : *mut u8) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_get_signature_type_nid(s : *const super::types::SSL, pnid : *mut i32) -> i32);
#[cfg(feature = "types")]
windows_link::link!("ssl" "C" fn SSL_set_tlsext_max_fragment_length(ssl : *mut super::types::SSL, mode : u8) -> i32);
pub const OPENSSL_TLS_SECURITY_LEVEL: i32 = 2;
pub const SSL_TLSEXT_ERR_ALERT_FATAL: i32 = 2;
pub const SSL_TLSEXT_ERR_ALERT_WARNING: i32 = 1;
pub const SSL_TLSEXT_ERR_NOACK: i32 = 3;
pub const SSL_TLSEXT_ERR_OK: i32 = 0;
pub const TLS13_AD_CERTIFICATE_REQUIRED: i32 = 116;
pub const TLS13_AD_MISSING_EXTENSION: i32 = 109;
pub const TLS1_1_VERSION_MAJOR: i32 = 3;
pub const TLS1_1_VERSION_MINOR: i32 = 2;
pub const TLS1_2_VERSION_MAJOR: i32 = 3;
pub const TLS1_2_VERSION_MINOR: i32 = 3;
pub const TLS1_3_CK_AES_128_CCM_8_SHA256: i32 = 50336517;
pub const TLS1_3_CK_AES_128_CCM_SHA256: i32 = 50336516;
pub const TLS1_3_CK_AES_128_GCM_SHA256: i32 = 50336513;
pub const TLS1_3_CK_AES_256_GCM_SHA384: i32 = 50336514;
pub const TLS1_3_CK_CHACHA20_POLY1305_SHA256: i32 = 50336515;
pub const TLS1_3_CK_SHA256_SHA256: i32 = 50380980;
pub const TLS1_3_CK_SHA384_SHA384: i32 = 50380981;
pub const TLS1_3_RFC_AES_128_CCM_8_SHA256: *const u8 = [
    84, 76, 83, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 67, 77, 95, 56, 95, 83, 72, 65, 50, 53, 54,
    0,
]
.as_ptr();
pub const TLS1_3_RFC_AES_128_CCM_SHA256: *const u8 = [
    84, 76, 83, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_3_RFC_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 65, 69, 83, 95, 49, 50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_3_RFC_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 65, 69, 83, 95, 50, 53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_3_RFC_CHACHA20_POLY1305_SHA256: *const u8 = [
    84, 76, 83, 95, 67, 72, 65, 67, 72, 65, 50, 48, 95, 80, 79, 76, 89, 49, 51, 48, 53, 95, 83, 72,
    65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_3_RFC_SHA256_SHA256: *const u8 = [
    84, 76, 83, 95, 83, 72, 65, 50, 53, 54, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_3_RFC_SHA384_SHA384: *const u8 = [
    84, 76, 83, 95, 83, 72, 65, 51, 56, 52, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_AD_ACCESS_DENIED: i32 = 49;
pub const TLS1_AD_BAD_CERTIFICATE_HASH_VALUE: i32 = 114;
pub const TLS1_AD_BAD_CERTIFICATE_STATUS_RESPONSE: i32 = 113;
pub const TLS1_AD_CERTIFICATE_UNOBTAINABLE: i32 = 111;
pub const TLS1_AD_DECODE_ERROR: i32 = 50;
pub const TLS1_AD_DECRYPTION_FAILED: i32 = 21;
pub const TLS1_AD_DECRYPT_ERROR: i32 = 51;
pub const TLS1_AD_EXPORT_RESTRICTION: i32 = 60;
pub const TLS1_AD_INAPPROPRIATE_FALLBACK: i32 = 86;
pub const TLS1_AD_INSUFFICIENT_SECURITY: i32 = 71;
pub const TLS1_AD_INTERNAL_ERROR: i32 = 80;
pub const TLS1_AD_NO_APPLICATION_PROTOCOL: i32 = 120;
pub const TLS1_AD_NO_RENEGOTIATION: i32 = 100;
pub const TLS1_AD_PROTOCOL_VERSION: i32 = 70;
pub const TLS1_AD_RECORD_OVERFLOW: i32 = 22;
pub const TLS1_AD_UNKNOWN_CA: i32 = 48;
pub const TLS1_AD_UNKNOWN_PSK_IDENTITY: i32 = 115;
pub const TLS1_AD_UNRECOGNIZED_NAME: i32 = 112;
pub const TLS1_AD_UNSUPPORTED_EXTENSION: i32 = 110;
pub const TLS1_AD_USER_CANCELLED: i32 = 90;
pub const TLS1_CK_ADH_WITH_AES_128_GCM_SHA256: i32 = 50331814;
pub const TLS1_CK_ADH_WITH_AES_128_SHA: i32 = 50331700;
pub const TLS1_CK_ADH_WITH_AES_128_SHA256: i32 = 50331756;
pub const TLS1_CK_ADH_WITH_AES_256_GCM_SHA384: i32 = 50331815;
pub const TLS1_CK_ADH_WITH_AES_256_SHA: i32 = 50331706;
pub const TLS1_CK_ADH_WITH_AES_256_SHA256: i32 = 50331757;
pub const TLS1_CK_ADH_WITH_CAMELLIA_128_CBC_SHA: i32 = 50331718;
pub const TLS1_CK_ADH_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50331839;
pub const TLS1_CK_ADH_WITH_CAMELLIA_256_CBC_SHA: i32 = 50331785;
pub const TLS1_CK_ADH_WITH_CAMELLIA_256_CBC_SHA256: i32 = 50331845;
pub const TLS1_CK_ADH_WITH_SEED_SHA: i32 = 50331803;
pub const TLS1_CK_DHE_DSS_WITH_AES_128_GCM_SHA256: i32 = 50331810;
pub const TLS1_CK_DHE_DSS_WITH_AES_128_SHA: i32 = 50331698;
pub const TLS1_CK_DHE_DSS_WITH_AES_128_SHA256: i32 = 50331712;
pub const TLS1_CK_DHE_DSS_WITH_AES_256_GCM_SHA384: i32 = 50331811;
pub const TLS1_CK_DHE_DSS_WITH_AES_256_SHA: i32 = 50331704;
pub const TLS1_CK_DHE_DSS_WITH_AES_256_SHA256: i32 = 50331754;
pub const TLS1_CK_DHE_DSS_WITH_ARIA_128_GCM_SHA256: i32 = 50380886;
pub const TLS1_CK_DHE_DSS_WITH_ARIA_256_GCM_SHA384: i32 = 50380887;
pub const TLS1_CK_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA: i32 = 50331716;
pub const TLS1_CK_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50331837;
pub const TLS1_CK_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA: i32 = 50331783;
pub const TLS1_CK_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256: i32 = 50331843;
pub const TLS1_CK_DHE_DSS_WITH_SEED_SHA: i32 = 50331801;
pub const TLS1_CK_DHE_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 50331791;
pub const TLS1_CK_DHE_PSK_WITH_AES_128_CBC_SHA: i32 = 50331792;
pub const TLS1_CK_DHE_PSK_WITH_AES_128_CBC_SHA256: i32 = 50331826;
pub const TLS1_CK_DHE_PSK_WITH_AES_128_CCM: i32 = 50380966;
pub const TLS1_CK_DHE_PSK_WITH_AES_128_CCM_8: i32 = 50380970;
pub const TLS1_CK_DHE_PSK_WITH_AES_128_GCM_SHA256: i32 = 50331818;
pub const TLS1_CK_DHE_PSK_WITH_AES_256_CBC_SHA: i32 = 50331793;
pub const TLS1_CK_DHE_PSK_WITH_AES_256_CBC_SHA384: i32 = 50331827;
pub const TLS1_CK_DHE_PSK_WITH_AES_256_CCM: i32 = 50380967;
pub const TLS1_CK_DHE_PSK_WITH_AES_256_CCM_8: i32 = 50380971;
pub const TLS1_CK_DHE_PSK_WITH_AES_256_GCM_SHA384: i32 = 50331819;
pub const TLS1_CK_DHE_PSK_WITH_ARIA_128_GCM_SHA256: i32 = 50380908;
pub const TLS1_CK_DHE_PSK_WITH_ARIA_256_GCM_SHA384: i32 = 50380909;
pub const TLS1_CK_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380950;
pub const TLS1_CK_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380951;
pub const TLS1_CK_DHE_PSK_WITH_CHACHA20_POLY1305: i32 = 50384045;
pub const TLS1_CK_DHE_PSK_WITH_NULL_SHA: i32 = 50331693;
pub const TLS1_CK_DHE_PSK_WITH_NULL_SHA256: i32 = 50331828;
pub const TLS1_CK_DHE_PSK_WITH_NULL_SHA384: i32 = 50331829;
pub const TLS1_CK_DHE_PSK_WITH_RC4_128_SHA: i32 = 50331790;
pub const TLS1_CK_DHE_RSA_WITH_AES_128_CCM: i32 = 50380958;
pub const TLS1_CK_DHE_RSA_WITH_AES_128_CCM_8: i32 = 50380962;
pub const TLS1_CK_DHE_RSA_WITH_AES_128_GCM_SHA256: i32 = 50331806;
pub const TLS1_CK_DHE_RSA_WITH_AES_128_SHA: i32 = 50331699;
pub const TLS1_CK_DHE_RSA_WITH_AES_128_SHA256: i32 = 50331751;
pub const TLS1_CK_DHE_RSA_WITH_AES_256_CCM: i32 = 50380959;
pub const TLS1_CK_DHE_RSA_WITH_AES_256_CCM_8: i32 = 50380963;
pub const TLS1_CK_DHE_RSA_WITH_AES_256_GCM_SHA384: i32 = 50331807;
pub const TLS1_CK_DHE_RSA_WITH_AES_256_SHA: i32 = 50331705;
pub const TLS1_CK_DHE_RSA_WITH_AES_256_SHA256: i32 = 50331755;
pub const TLS1_CK_DHE_RSA_WITH_ARIA_128_GCM_SHA256: i32 = 50380882;
pub const TLS1_CK_DHE_RSA_WITH_ARIA_256_GCM_SHA384: i32 = 50380883;
pub const TLS1_CK_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA: i32 = 50331717;
pub const TLS1_CK_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50331838;
pub const TLS1_CK_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA: i32 = 50331784;
pub const TLS1_CK_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256: i32 = 50331844;
pub const TLS1_CK_DHE_RSA_WITH_CHACHA20_POLY1305: i32 = 50384042;
pub const TLS1_CK_DHE_RSA_WITH_SEED_SHA: i32 = 50331802;
pub const TLS1_CK_DH_DSS_WITH_AES_128_GCM_SHA256: i32 = 50331812;
pub const TLS1_CK_DH_DSS_WITH_AES_128_SHA: i32 = 50331696;
pub const TLS1_CK_DH_DSS_WITH_AES_128_SHA256: i32 = 50331710;
pub const TLS1_CK_DH_DSS_WITH_AES_256_GCM_SHA384: i32 = 50331813;
pub const TLS1_CK_DH_DSS_WITH_AES_256_SHA: i32 = 50331702;
pub const TLS1_CK_DH_DSS_WITH_AES_256_SHA256: i32 = 50331752;
pub const TLS1_CK_DH_DSS_WITH_ARIA_128_GCM_SHA256: i32 = 50380888;
pub const TLS1_CK_DH_DSS_WITH_ARIA_256_GCM_SHA384: i32 = 50380889;
pub const TLS1_CK_DH_DSS_WITH_CAMELLIA_128_CBC_SHA: i32 = 50331714;
pub const TLS1_CK_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50331835;
pub const TLS1_CK_DH_DSS_WITH_CAMELLIA_256_CBC_SHA: i32 = 50331781;
pub const TLS1_CK_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256: i32 = 50331841;
pub const TLS1_CK_DH_DSS_WITH_SEED_SHA: i32 = 50331799;
pub const TLS1_CK_DH_RSA_WITH_AES_128_GCM_SHA256: i32 = 50331808;
pub const TLS1_CK_DH_RSA_WITH_AES_128_SHA: i32 = 50331697;
pub const TLS1_CK_DH_RSA_WITH_AES_128_SHA256: i32 = 50331711;
pub const TLS1_CK_DH_RSA_WITH_AES_256_GCM_SHA384: i32 = 50331809;
pub const TLS1_CK_DH_RSA_WITH_AES_256_SHA: i32 = 50331703;
pub const TLS1_CK_DH_RSA_WITH_AES_256_SHA256: i32 = 50331753;
pub const TLS1_CK_DH_RSA_WITH_ARIA_128_GCM_SHA256: i32 = 50380884;
pub const TLS1_CK_DH_RSA_WITH_ARIA_256_GCM_SHA384: i32 = 50380885;
pub const TLS1_CK_DH_RSA_WITH_CAMELLIA_128_CBC_SHA: i32 = 50331715;
pub const TLS1_CK_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50331836;
pub const TLS1_CK_DH_RSA_WITH_CAMELLIA_256_CBC_SHA: i32 = 50331782;
pub const TLS1_CK_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256: i32 = 50331842;
pub const TLS1_CK_DH_RSA_WITH_SEED_SHA: i32 = 50331800;
pub const TLS1_CK_DH_anon_WITH_ARIA_128_GCM_SHA256: i32 = 50380890;
pub const TLS1_CK_DH_anon_WITH_ARIA_256_GCM_SHA384: i32 = 50380891;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: i32 = 50380809;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_128_CCM: i32 = 50380972;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_128_CCM_8: i32 = 50380974;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: i32 = 50380843;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_128_SHA256: i32 = 50380835;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: i32 = 50380810;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_256_CCM: i32 = 50380973;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_256_CCM_8: i32 = 50380975;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: i32 = 50380844;
pub const TLS1_CK_ECDHE_ECDSA_WITH_AES_256_SHA384: i32 = 50380836;
pub const TLS1_CK_ECDHE_ECDSA_WITH_ARIA_128_GCM_SHA256: i32 = 50380892;
pub const TLS1_CK_ECDHE_ECDSA_WITH_ARIA_256_GCM_SHA384: i32 = 50380893;
pub const TLS1_CK_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380914;
pub const TLS1_CK_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380915;
pub const TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305: i32 = 50384041;
pub const TLS1_CK_ECDHE_ECDSA_WITH_DES_192_CBC3_SHA: i32 = 50380808;
pub const TLS1_CK_ECDHE_ECDSA_WITH_NULL_SHA: i32 = 50380806;
pub const TLS1_CK_ECDHE_ECDSA_WITH_RC4_128_SHA: i32 = 50380807;
pub const TLS1_CK_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 50380852;
pub const TLS1_CK_ECDHE_PSK_WITH_AES_128_CBC_SHA: i32 = 50380853;
pub const TLS1_CK_ECDHE_PSK_WITH_AES_128_CBC_SHA256: i32 = 50380855;
pub const TLS1_CK_ECDHE_PSK_WITH_AES_256_CBC_SHA: i32 = 50380854;
pub const TLS1_CK_ECDHE_PSK_WITH_AES_256_CBC_SHA384: i32 = 50380856;
pub const TLS1_CK_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380954;
pub const TLS1_CK_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380955;
pub const TLS1_CK_ECDHE_PSK_WITH_CHACHA20_POLY1305: i32 = 50384044;
pub const TLS1_CK_ECDHE_PSK_WITH_NULL_SHA: i32 = 50380857;
pub const TLS1_CK_ECDHE_PSK_WITH_NULL_SHA256: i32 = 50380858;
pub const TLS1_CK_ECDHE_PSK_WITH_NULL_SHA384: i32 = 50380859;
pub const TLS1_CK_ECDHE_PSK_WITH_RC4_128_SHA: i32 = 50380851;
pub const TLS1_CK_ECDHE_RSA_WITH_AES_128_CBC_SHA: i32 = 50380819;
pub const TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256: i32 = 50380847;
pub const TLS1_CK_ECDHE_RSA_WITH_AES_128_SHA256: i32 = 50380839;
pub const TLS1_CK_ECDHE_RSA_WITH_AES_256_CBC_SHA: i32 = 50380820;
pub const TLS1_CK_ECDHE_RSA_WITH_AES_256_GCM_SHA384: i32 = 50380848;
pub const TLS1_CK_ECDHE_RSA_WITH_AES_256_SHA384: i32 = 50380840;
pub const TLS1_CK_ECDHE_RSA_WITH_ARIA_128_GCM_SHA256: i32 = 50380896;
pub const TLS1_CK_ECDHE_RSA_WITH_ARIA_256_GCM_SHA384: i32 = 50380897;
pub const TLS1_CK_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380918;
pub const TLS1_CK_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380919;
pub const TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305: i32 = 50384040;
pub const TLS1_CK_ECDHE_RSA_WITH_DES_192_CBC3_SHA: i32 = 50380818;
pub const TLS1_CK_ECDHE_RSA_WITH_NULL_SHA: i32 = 50380816;
pub const TLS1_CK_ECDHE_RSA_WITH_RC4_128_SHA: i32 = 50380817;
pub const TLS1_CK_ECDH_ECDSA_WITH_AES_128_CBC_SHA: i32 = 50380804;
pub const TLS1_CK_ECDH_ECDSA_WITH_AES_128_GCM_SHA256: i32 = 50380845;
pub const TLS1_CK_ECDH_ECDSA_WITH_AES_128_SHA256: i32 = 50380837;
pub const TLS1_CK_ECDH_ECDSA_WITH_AES_256_CBC_SHA: i32 = 50380805;
pub const TLS1_CK_ECDH_ECDSA_WITH_AES_256_GCM_SHA384: i32 = 50380846;
pub const TLS1_CK_ECDH_ECDSA_WITH_AES_256_SHA384: i32 = 50380838;
pub const TLS1_CK_ECDH_ECDSA_WITH_ARIA_128_GCM_SHA256: i32 = 50380894;
pub const TLS1_CK_ECDH_ECDSA_WITH_ARIA_256_GCM_SHA384: i32 = 50380895;
pub const TLS1_CK_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380916;
pub const TLS1_CK_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380917;
pub const TLS1_CK_ECDH_ECDSA_WITH_DES_192_CBC3_SHA: i32 = 50380803;
pub const TLS1_CK_ECDH_ECDSA_WITH_NULL_SHA: i32 = 50380801;
pub const TLS1_CK_ECDH_ECDSA_WITH_RC4_128_SHA: i32 = 50380802;
pub const TLS1_CK_ECDH_RSA_WITH_AES_128_CBC_SHA: i32 = 50380814;
pub const TLS1_CK_ECDH_RSA_WITH_AES_128_GCM_SHA256: i32 = 50380849;
pub const TLS1_CK_ECDH_RSA_WITH_AES_128_SHA256: i32 = 50380841;
pub const TLS1_CK_ECDH_RSA_WITH_AES_256_CBC_SHA: i32 = 50380815;
pub const TLS1_CK_ECDH_RSA_WITH_AES_256_GCM_SHA384: i32 = 50380850;
pub const TLS1_CK_ECDH_RSA_WITH_AES_256_SHA384: i32 = 50380842;
pub const TLS1_CK_ECDH_RSA_WITH_ARIA_128_GCM_SHA256: i32 = 50380898;
pub const TLS1_CK_ECDH_RSA_WITH_ARIA_256_GCM_SHA384: i32 = 50380899;
pub const TLS1_CK_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380920;
pub const TLS1_CK_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380921;
pub const TLS1_CK_ECDH_RSA_WITH_DES_192_CBC3_SHA: i32 = 50380813;
pub const TLS1_CK_ECDH_RSA_WITH_NULL_SHA: i32 = 50380811;
pub const TLS1_CK_ECDH_RSA_WITH_RC4_128_SHA: i32 = 50380812;
pub const TLS1_CK_ECDH_anon_WITH_AES_128_CBC_SHA: i32 = 50380824;
pub const TLS1_CK_ECDH_anon_WITH_AES_256_CBC_SHA: i32 = 50380825;
pub const TLS1_CK_ECDH_anon_WITH_DES_192_CBC3_SHA: i32 = 50380823;
pub const TLS1_CK_ECDH_anon_WITH_NULL_SHA: i32 = 50380821;
pub const TLS1_CK_ECDH_anon_WITH_RC4_128_SHA: i32 = 50380822;
pub const TLS1_CK_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 50331787;
pub const TLS1_CK_PSK_WITH_AES_128_CBC_SHA: i32 = 50331788;
pub const TLS1_CK_PSK_WITH_AES_128_CBC_SHA256: i32 = 50331822;
pub const TLS1_CK_PSK_WITH_AES_128_CCM: i32 = 50380964;
pub const TLS1_CK_PSK_WITH_AES_128_CCM_8: i32 = 50380968;
pub const TLS1_CK_PSK_WITH_AES_128_GCM_SHA256: i32 = 50331816;
pub const TLS1_CK_PSK_WITH_AES_256_CBC_SHA: i32 = 50331789;
pub const TLS1_CK_PSK_WITH_AES_256_CBC_SHA384: i32 = 50331823;
pub const TLS1_CK_PSK_WITH_AES_256_CCM: i32 = 50380965;
pub const TLS1_CK_PSK_WITH_AES_256_CCM_8: i32 = 50380969;
pub const TLS1_CK_PSK_WITH_AES_256_GCM_SHA384: i32 = 50331817;
pub const TLS1_CK_PSK_WITH_ARIA_128_GCM_SHA256: i32 = 50380906;
pub const TLS1_CK_PSK_WITH_ARIA_256_GCM_SHA384: i32 = 50380907;
pub const TLS1_CK_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380948;
pub const TLS1_CK_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380949;
pub const TLS1_CK_PSK_WITH_CHACHA20_POLY1305: i32 = 50384043;
pub const TLS1_CK_PSK_WITH_NULL_SHA: i32 = 50331692;
pub const TLS1_CK_PSK_WITH_NULL_SHA256: i32 = 50331824;
pub const TLS1_CK_PSK_WITH_NULL_SHA384: i32 = 50331825;
pub const TLS1_CK_PSK_WITH_RC4_128_SHA: i32 = 50331786;
pub const TLS1_CK_RSA_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 50331795;
pub const TLS1_CK_RSA_PSK_WITH_AES_128_CBC_SHA: i32 = 50331796;
pub const TLS1_CK_RSA_PSK_WITH_AES_128_CBC_SHA256: i32 = 50331830;
pub const TLS1_CK_RSA_PSK_WITH_AES_128_GCM_SHA256: i32 = 50331820;
pub const TLS1_CK_RSA_PSK_WITH_AES_256_CBC_SHA: i32 = 50331797;
pub const TLS1_CK_RSA_PSK_WITH_AES_256_CBC_SHA384: i32 = 50331831;
pub const TLS1_CK_RSA_PSK_WITH_AES_256_GCM_SHA384: i32 = 50331821;
pub const TLS1_CK_RSA_PSK_WITH_ARIA_128_GCM_SHA256: i32 = 50380910;
pub const TLS1_CK_RSA_PSK_WITH_ARIA_256_GCM_SHA384: i32 = 50380911;
pub const TLS1_CK_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50380952;
pub const TLS1_CK_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 50380953;
pub const TLS1_CK_RSA_PSK_WITH_CHACHA20_POLY1305: i32 = 50384046;
pub const TLS1_CK_RSA_PSK_WITH_NULL_SHA: i32 = 50331694;
pub const TLS1_CK_RSA_PSK_WITH_NULL_SHA256: i32 = 50331832;
pub const TLS1_CK_RSA_PSK_WITH_NULL_SHA384: i32 = 50331833;
pub const TLS1_CK_RSA_PSK_WITH_RC4_128_SHA: i32 = 50331794;
pub const TLS1_CK_RSA_WITH_AES_128_CCM: i32 = 50380956;
pub const TLS1_CK_RSA_WITH_AES_128_CCM_8: i32 = 50380960;
pub const TLS1_CK_RSA_WITH_AES_128_GCM_SHA256: i32 = 50331804;
pub const TLS1_CK_RSA_WITH_AES_128_SHA: i32 = 50331695;
pub const TLS1_CK_RSA_WITH_AES_128_SHA256: i32 = 50331708;
pub const TLS1_CK_RSA_WITH_AES_256_CCM: i32 = 50380957;
pub const TLS1_CK_RSA_WITH_AES_256_CCM_8: i32 = 50380961;
pub const TLS1_CK_RSA_WITH_AES_256_GCM_SHA384: i32 = 50331805;
pub const TLS1_CK_RSA_WITH_AES_256_SHA: i32 = 50331701;
pub const TLS1_CK_RSA_WITH_AES_256_SHA256: i32 = 50331709;
pub const TLS1_CK_RSA_WITH_ARIA_128_GCM_SHA256: i32 = 50380880;
pub const TLS1_CK_RSA_WITH_ARIA_256_GCM_SHA384: i32 = 50380881;
pub const TLS1_CK_RSA_WITH_CAMELLIA_128_CBC_SHA: i32 = 50331713;
pub const TLS1_CK_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 50331834;
pub const TLS1_CK_RSA_WITH_CAMELLIA_256_CBC_SHA: i32 = 50331780;
pub const TLS1_CK_RSA_WITH_CAMELLIA_256_CBC_SHA256: i32 = 50331840;
pub const TLS1_CK_RSA_WITH_NULL_SHA256: i32 = 50331707;
pub const TLS1_CK_RSA_WITH_SEED_SHA: i32 = 50331798;
pub const TLS1_CK_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA: i32 = 50380828;
pub const TLS1_CK_SRP_SHA_DSS_WITH_AES_128_CBC_SHA: i32 = 50380831;
pub const TLS1_CK_SRP_SHA_DSS_WITH_AES_256_CBC_SHA: i32 = 50380834;
pub const TLS1_CK_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA: i32 = 50380827;
pub const TLS1_CK_SRP_SHA_RSA_WITH_AES_128_CBC_SHA: i32 = 50380830;
pub const TLS1_CK_SRP_SHA_RSA_WITH_AES_256_CBC_SHA: i32 = 50380833;
pub const TLS1_CK_SRP_SHA_WITH_3DES_EDE_CBC_SHA: i32 = 50380826;
pub const TLS1_CK_SRP_SHA_WITH_AES_128_CBC_SHA: i32 = 50380829;
pub const TLS1_CK_SRP_SHA_WITH_AES_256_CBC_SHA: i32 = 50380832;
pub const TLS1_FINISH_MAC_LENGTH: i32 = 12;
pub const TLS1_RFC_ADH_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50,
    56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_AES_128_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50,
    56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_AES_128_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50,
    56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53,
    54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_AES_256_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53,
    54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_AES_256_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53,
    54, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76,
    73, 65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76,
    73, 65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76,
    73, 65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76,
    73, 65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ADH_WITH_SEED_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 83, 69, 69, 68, 95, 67,
    66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_AES_128_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_AES_128_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_AES_256_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_AES_256_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50,
    56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53,
    54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_DSS_WITH_SEED_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 83, 69, 69, 68, 95, 67, 66,
    67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 51, 68, 69, 83, 95, 69, 68,
    69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_128_CCM: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_128_CCM_8: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 68, 72, 69, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 67, 77, 95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_256_CCM: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_256_CCM_8: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 68, 72, 69, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 67, 77, 95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50,
    56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53,
    54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 72, 65, 67, 72, 65, 50,
    48, 95, 80, 79, 76, 89, 49, 51, 48, 53, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_NULL_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72,
    65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_NULL_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72,
    65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_NULL_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72,
    65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_PSK_WITH_RC4_128_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 82, 67, 52, 95, 49, 50, 56,
    95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_128_CCM: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_128_CCM_8: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 67, 77, 95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_128_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_128_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_256_CCM: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_256_CCM_8: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 67, 77, 95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_256_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_AES_256_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50,
    56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53,
    54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_CHACHA20_POLY1305: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 72, 65, 67, 72, 65, 50,
    48, 95, 80, 79, 76, 89, 49, 51, 48, 53, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DHE_RSA_WITH_SEED_SHA: *const u8 = [
    84, 76, 83, 95, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 83, 69, 69, 68, 95, 67, 66,
    67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_DH_DSS_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50, 56,
    95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DH_DSS_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53, 54,
    95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DH_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50, 56,
    95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DH_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53, 54,
    95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_DH_anon_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49,
    50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_DH_anon_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50,
    53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_128_CCM: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 49, 50, 56, 95, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_128_CCM_8: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 49, 50, 56, 95, 67, 67, 77, 95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 49, 50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_128_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_256_CCM: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 50, 53, 54, 95, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_256_CCM_8: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 50, 53, 54, 95, 67, 67, 77, 95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 50, 53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_AES_256_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73,
    65, 95, 49, 50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73,
    65, 95, 50, 53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77,
    69, 76, 76, 73, 65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77,
    69, 76, 76, 73, 65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_CHACHA20_POLY1305: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 67, 72, 65,
    67, 72, 65, 50, 48, 95, 80, 79, 76, 89, 49, 51, 48, 53, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_DES_192_CBC3_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 51, 68, 69,
    83, 95, 69, 68, 69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_NULL_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 78, 85, 76,
    76, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_ECDSA_WITH_RC4_128_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 82, 67, 52,
    95, 49, 50, 56, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 51, 68, 69, 83, 95,
    69, 68, 69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49,
    50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49,
    50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50,
    53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50,
    53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76,
    76, 73, 65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76,
    76, 73, 65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 72, 65, 67, 72,
    65, 50, 48, 95, 80, 79, 76, 89, 49, 51, 48, 53, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_NULL_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95,
    83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_NULL_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95,
    83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_NULL_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95,
    83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_PSK_WITH_RC4_128_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 82, 67, 52, 95, 49,
    50, 56, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49,
    50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49,
    50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_AES_128_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49,
    50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50,
    53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50,
    53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_AES_256_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50,
    53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95,
    49, 50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95,
    50, 53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76,
    76, 73, 65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76,
    76, 73, 65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_CHACHA20_POLY1305: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 72, 65, 67, 72,
    65, 50, 48, 95, 80, 79, 76, 89, 49, 51, 48, 53, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_DES_192_CBC3_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 51, 68, 69, 83, 95,
    69, 68, 69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_NULL_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95,
    83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDHE_RSA_WITH_RC4_128_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 69, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 82, 67, 52, 95, 49,
    50, 56, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_ECDSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65,
    95, 49, 50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_ECDSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 69, 67, 68, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65,
    95, 50, 53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49,
    50, 56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50,
    53, 54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_anon_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95,
    49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_anon_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95,
    50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_anon_WITH_DES_192_CBC3_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 51, 68, 69, 83,
    95, 69, 68, 69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_anon_WITH_NULL_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76,
    95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_ECDH_anon_WITH_RC4_128_SHA: *const u8 = [
    84, 76, 83, 95, 69, 67, 68, 72, 95, 97, 110, 111, 110, 95, 87, 73, 84, 72, 95, 82, 67, 52, 95,
    49, 50, 56, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 51, 68, 69, 83, 95, 69, 68, 69, 95, 67, 66,
    67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 66, 67,
    95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 66, 67,
    95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_128_CCM: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 67, 77,
    0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_128_CCM_8: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 67, 77,
    95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 71, 67, 77,
    95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 66, 67,
    95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 66, 67,
    95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_256_CCM: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 67, 77,
    0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_256_CCM_8: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 67, 77,
    95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 71, 67, 77,
    95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50, 56, 95, 71, 67,
    77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53, 54, 95, 71, 67,
    77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73, 65, 95, 49, 50,
    56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73, 65, 95, 50, 53,
    54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 72, 65, 67, 72, 65, 50, 48, 95, 80, 79,
    76, 89, 49, 51, 48, 53, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_NULL_SHA: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_NULL_SHA256: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72, 65, 50, 53, 54,
    0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_NULL_SHA384: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72, 65, 51, 56, 52,
    0,
]
.as_ptr();
pub const TLS1_RFC_PSK_WITH_RC4_128_SHA: *const u8 = [
    84, 76, 83, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 82, 67, 52, 95, 49, 50, 56, 95, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 51, 68, 69, 83, 95, 69, 68,
    69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50,
    56, 95, 71, 67, 77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53,
    54, 95, 71, 67, 77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73,
    65, 95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 67, 72, 65, 67, 72, 65, 50,
    48, 95, 80, 79, 76, 89, 49, 51, 48, 53, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_NULL_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72,
    65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_NULL_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72,
    65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_NULL_SHA384: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72,
    65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_PSK_WITH_RC4_128_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 80, 83, 75, 95, 87, 73, 84, 72, 95, 82, 67, 52, 95, 49, 50, 56,
    95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_128_CCM: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 67, 77,
    0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_128_CCM_8: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 67, 77,
    95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 71, 67, 77,
    95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_128_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 66, 67,
    95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_128_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56, 95, 67, 66, 67,
    95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_256_CCM: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 67, 77,
    0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_256_CCM_8: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 67, 77,
    95, 56, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 71, 67, 77,
    95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_256_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 66, 67,
    95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_AES_256_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54, 95, 67, 66, 67,
    95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 49, 50, 56, 95, 71, 67,
    77, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 82, 73, 65, 95, 50, 53, 54, 95, 71, 67,
    77, 95, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73, 65, 95, 49, 50,
    56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73, 65, 95, 49, 50,
    56, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73, 65, 95, 50, 53,
    54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 67, 65, 77, 69, 76, 76, 73, 65, 95, 50, 53,
    54, 95, 67, 66, 67, 95, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_NULL_SHA256: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 78, 85, 76, 76, 95, 83, 72, 65, 50, 53, 54,
    0,
]
.as_ptr();
pub const TLS1_RFC_RSA_WITH_SEED_SHA: *const u8 = [
    84, 76, 83, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 83, 69, 69, 68, 95, 67, 66, 67, 95, 83, 72,
    65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 51, 68, 69,
    83, 95, 69, 68, 69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_DSS_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_DSS_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 68, 83, 83, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 51, 68, 69,
    83, 95, 69, 68, 69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_RSA_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 49, 50, 56, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_RSA_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 82, 83, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83,
    95, 50, 53, 54, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 87, 73, 84, 72, 95, 51, 68, 69, 83, 95, 69, 68,
    69, 95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_WITH_AES_128_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 49, 50, 56,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_RFC_SRP_SHA_WITH_AES_256_CBC_SHA: *const u8 = [
    84, 76, 83, 95, 83, 82, 80, 95, 83, 72, 65, 95, 87, 73, 84, 72, 95, 65, 69, 83, 95, 50, 53, 54,
    95, 67, 66, 67, 95, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_AES_128_GCM_SHA256: *const u8 = [
    65, 68, 72, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_AES_128_SHA: *const u8 =
    [65, 68, 72, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_ADH_WITH_AES_128_SHA256: *const u8 = [
    65, 68, 72, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_AES_256_GCM_SHA384: *const u8 = [
    65, 68, 72, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_AES_256_SHA: *const u8 =
    [65, 68, 72, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_ADH_WITH_AES_256_SHA256: *const u8 = [
    65, 68, 72, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    65, 68, 72, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    65, 68, 72, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    65, 68, 72, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    65, 68, 72, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ADH_WITH_SEED_SHA: *const u8 =
    [65, 68, 72, 45, 83, 69, 69, 68, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_AES_128_GCM_SHA256: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_AES_128_SHA: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_AES_128_SHA256: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_AES_256_GCM_SHA384: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_AES_256_SHA: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_AES_256_SHA256: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51,
    56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_RC4_128_SHA: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_DSS_WITH_SEED_SHA: *const u8 = [
    68, 72, 69, 45, 68, 83, 83, 45, 83, 69, 69, 68, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 51, 68, 69, 83, 45, 69, 68, 69, 45, 67, 66, 67, 45, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_128_CCM: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_128_CCM_8: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_128_GCM_SHA256: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_256_CCM: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_256_CCM_8: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_AES_256_GCM_SHA384: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51,
    56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 51,
    56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 67, 72, 65, 67, 72, 65, 50, 48, 45, 80, 79, 76, 89, 49, 51, 48,
    53, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_NULL_SHA: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_NULL_SHA256: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_NULL_SHA384: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_PSK_WITH_RC4_128_SHA: *const u8 = [
    68, 72, 69, 45, 80, 83, 75, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_128_CCM: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_128_CCM_8: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_128_SHA: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_128_SHA256: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_256_CCM: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_256_CCM_8: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_256_SHA: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_AES_256_SHA256: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51,
    56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_CHACHA20_POLY1305: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 67, 72, 65, 67, 72, 65, 50, 48, 45, 80, 79, 76, 89, 49, 51, 48,
    53, 0,
]
.as_ptr();
pub const TLS1_TXT_DHE_RSA_WITH_SEED_SHA: *const u8 = [
    68, 72, 69, 45, 82, 83, 65, 45, 83, 69, 69, 68, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_AES_128_GCM_SHA256: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54,
    0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_AES_128_SHA: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_AES_128_SHA256: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_AES_256_GCM_SHA384: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52,
    0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_AES_256_SHA: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_AES_256_SHA256: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_DSS_WITH_SEED_SHA: *const u8 = [
    68, 72, 45, 68, 83, 83, 45, 83, 69, 69, 68, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54,
    0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_AES_128_SHA: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_AES_128_SHA256: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52,
    0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_AES_256_SHA: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_AES_256_SHA256: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_RSA_WITH_SEED_SHA: *const u8 = [
    68, 72, 45, 82, 83, 65, 45, 83, 69, 69, 68, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_anon_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    65, 68, 72, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_DH_anon_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    65, 68, 72, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_128_CCM: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_128_CCM_8: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83,
    72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_128_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_256_CCM: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_256_CCM_8: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83,
    72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_AES_256_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45,
    83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45,
    83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45,
    83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45,
    83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_CHACHA20_POLY1305: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 67, 72, 65, 67, 72, 65, 50, 48, 45, 80, 79, 76,
    89, 49, 51, 48, 53, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_DES_192_CBC3_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 68, 69, 83, 45, 67, 66, 67, 51, 45, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_NULL_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_ECDSA_WITH_RC4_128_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 69, 67, 68, 83, 65, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 51, 68, 69, 83, 45, 69, 68, 69, 45, 67, 66, 67, 45, 83,
    72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65,
    50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65,
    51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72,
    65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72,
    65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 67, 72, 65, 67, 72, 65, 50, 48, 45, 80, 79, 76, 89, 49,
    51, 48, 53, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_NULL_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_NULL_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_NULL_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_PSK_WITH_RC4_128_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 80, 83, 75, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_AES_128_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65,
    50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_AES_128_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_AES_256_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65,
    51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_AES_256_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54,
    0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52,
    0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72,
    65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72,
    65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_CHACHA20_POLY1305: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 67, 72, 65, 67, 72, 65, 50, 48, 45, 80, 79, 76, 89, 49,
    51, 48, 53, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_DES_192_CBC3_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 68, 69, 83, 45, 67, 66, 67, 51, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_NULL_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDHE_RSA_WITH_RC4_128_SHA: *const u8 = [
    69, 67, 68, 72, 69, 45, 82, 83, 65, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_AES_128_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72,
    65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_AES_128_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54,
    0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_AES_256_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72,
    65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_AES_256_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 51, 56, 52,
    0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83,
    72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83,
    72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83,
    72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83,
    72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_DES_192_CBC3_SHA: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 68, 69, 83, 45, 67, 66, 67, 51, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_NULL_SHA: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_ECDSA_WITH_RC4_128_SHA: *const u8 = [
    69, 67, 68, 72, 45, 69, 67, 68, 83, 65, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_AES_128_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_AES_128_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_AES_256_CBC_SHA: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51,
    56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_AES_256_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65,
    50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65,
    51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_DES_192_CBC3_SHA: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 68, 69, 83, 45, 67, 66, 67, 51, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_NULL_SHA: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_RSA_WITH_RC4_128_SHA: *const u8 = [
    69, 67, 68, 72, 45, 82, 83, 65, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_anon_WITH_AES_128_CBC_SHA: *const u8 = [
    65, 69, 67, 68, 72, 45, 65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_anon_WITH_AES_256_CBC_SHA: *const u8 = [
    65, 69, 67, 68, 72, 45, 65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_anon_WITH_DES_192_CBC3_SHA: *const u8 = [
    65, 69, 67, 68, 72, 45, 68, 69, 83, 45, 67, 66, 67, 51, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_ECDH_anon_WITH_NULL_SHA: *const u8 =
    [65, 69, 67, 68, 72, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_ECDH_anon_WITH_RC4_128_SHA: *const u8 =
    [65, 69, 67, 68, 72, 45, 82, 67, 52, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    80, 83, 75, 45, 51, 68, 69, 83, 45, 69, 68, 69, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_128_CCM: *const u8 =
    [80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 0].as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_128_CCM_8: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_128_GCM_SHA256: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_256_CCM: *const u8 =
    [80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 0].as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_256_CCM_8: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 56, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_AES_256_GCM_SHA384: *const u8 = [
    80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    80, 83, 75, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    80, 83, 75, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    80, 83, 75, 45, 67, 72, 65, 67, 72, 65, 50, 48, 45, 80, 79, 76, 89, 49, 51, 48, 53, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_NULL_SHA: *const u8 =
    [80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_PSK_WITH_NULL_SHA256: *const u8 = [
    80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_NULL_SHA384: *const u8 = [
    80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_PSK_WITH_RC4_128_SHA: *const u8 =
    [80, 83, 75, 45, 82, 67, 52, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 51, 68, 69, 83, 45, 69, 68, 69, 45, 67, 66, 67, 45, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_AES_128_CBC_SHA: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_AES_128_CBC_SHA256: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_AES_128_GCM_SHA256: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53,
    54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_AES_256_CBC_SHA: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_AES_256_CBC_SHA384: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_AES_256_GCM_SHA384: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56,
    52, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51,
    56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50,
    53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 51,
    56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_CHACHA20_POLY1305: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 67, 72, 65, 67, 72, 65, 50, 48, 45, 80, 79, 76, 89, 49, 51, 48,
    53, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_NULL_SHA: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_NULL_SHA256: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_NULL_SHA384: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 78, 85, 76, 76, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_PSK_WITH_RC4_128_SHA: *const u8 = [
    82, 83, 65, 45, 80, 83, 75, 45, 82, 67, 52, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_128_CCM: *const u8 =
    [65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_128_CCM_8: *const u8 =
    [65, 69, 83, 49, 50, 56, 45, 67, 67, 77, 56, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_128_GCM_SHA256: *const u8 = [
    65, 69, 83, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_128_SHA: *const u8 =
    [65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_128_SHA256: *const u8 =
    [65, 69, 83, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_256_CCM: *const u8 =
    [65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_256_CCM_8: *const u8 =
    [65, 69, 83, 50, 53, 54, 45, 67, 67, 77, 56, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_256_GCM_SHA384: *const u8 = [
    65, 69, 83, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_256_SHA: *const u8 =
    [65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_AES_256_SHA256: *const u8 =
    [65, 69, 83, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_ARIA_128_GCM_SHA256: *const u8 = [
    65, 82, 73, 65, 49, 50, 56, 45, 71, 67, 77, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_ARIA_256_GCM_SHA384: *const u8 = [
    65, 82, 73, 65, 50, 53, 54, 45, 71, 67, 77, 45, 83, 72, 65, 51, 56, 52, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_CAMELLIA_128_CBC_SHA: *const u8 = [
    67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_CAMELLIA_128_CBC_SHA256: *const u8 = [
    67, 65, 77, 69, 76, 76, 73, 65, 49, 50, 56, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_CAMELLIA_256_CBC_SHA: *const u8 = [
    67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_CAMELLIA_256_CBC_SHA256: *const u8 = [
    67, 65, 77, 69, 76, 76, 73, 65, 50, 53, 54, 45, 83, 72, 65, 50, 53, 54, 0,
]
.as_ptr();
pub const TLS1_TXT_RSA_WITH_NULL_SHA256: *const u8 =
    [78, 85, 76, 76, 45, 83, 72, 65, 50, 53, 54, 0].as_ptr();
pub const TLS1_TXT_RSA_WITH_SEED_SHA: *const u8 = [83, 69, 69, 68, 45, 83, 72, 65, 0].as_ptr();
pub const TLS1_TXT_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 68, 83, 83, 45, 51, 68, 69, 83, 45, 69, 68, 69, 45, 67, 66, 67, 45, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_DSS_WITH_AES_128_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 68, 83, 83, 45, 65, 69, 83, 45, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_DSS_WITH_AES_256_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 68, 83, 83, 45, 65, 69, 83, 45, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 82, 83, 65, 45, 51, 68, 69, 83, 45, 69, 68, 69, 45, 67, 66, 67, 45, 83, 72, 65,
    0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_RSA_WITH_AES_128_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 82, 83, 65, 45, 65, 69, 83, 45, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_RSA_WITH_AES_256_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 82, 83, 65, 45, 65, 69, 83, 45, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_WITH_3DES_EDE_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 51, 68, 69, 83, 45, 69, 68, 69, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_WITH_AES_128_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 65, 69, 83, 45, 49, 50, 56, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_TXT_SRP_SHA_WITH_AES_256_CBC_SHA: *const u8 = [
    83, 82, 80, 45, 65, 69, 83, 45, 50, 53, 54, 45, 67, 66, 67, 45, 83, 72, 65, 0,
]
.as_ptr();
pub const TLS1_VERSION_MAJOR: i32 = 3;
pub const TLS1_VERSION_MINOR: i32 = 1;
pub const TLSEXT_ECPOINTFORMAT_ansiX962_compressed_char2: i32 = 2;
pub const TLSEXT_ECPOINTFORMAT_ansiX962_compressed_prime: i32 = 1;
pub const TLSEXT_ECPOINTFORMAT_first: i32 = 0;
pub const TLSEXT_ECPOINTFORMAT_last: i32 = 2;
pub const TLSEXT_ECPOINTFORMAT_uncompressed: i32 = 0;
pub const TLSEXT_MAXLEN_host_name: i32 = 255;
pub const TLSEXT_NAMETYPE_host_name: i32 = 0;
pub const TLSEXT_STATUSTYPE_ocsp: i32 = 1;
pub const TLSEXT_TYPE_application_layer_protocol_negotiation: i32 = 16;
pub const TLSEXT_TYPE_cert_type: i32 = 9;
pub const TLSEXT_TYPE_certificate_authorities: i32 = 47;
pub const TLSEXT_TYPE_client_authz: i32 = 7;
pub const TLSEXT_TYPE_client_cert_type: i32 = 19;
pub const TLSEXT_TYPE_client_certificate_url: i32 = 2;
pub const TLSEXT_TYPE_compress_certificate: i32 = 27;
pub const TLSEXT_TYPE_cookie: i32 = 44;
pub const TLSEXT_TYPE_early_data: i32 = 42;
pub const TLSEXT_TYPE_ec_point_formats: i32 = 11;
pub const TLSEXT_TYPE_elliptic_curves: i32 = 10;
pub const TLSEXT_TYPE_encrypt_then_mac: i32 = 22;
pub const TLSEXT_TYPE_extended_master_secret: i32 = 23;
pub const TLSEXT_TYPE_key_share: i32 = 51;
pub const TLSEXT_TYPE_max_fragment_length: i32 = 1;
pub const TLSEXT_TYPE_next_proto_neg: i32 = 13172;
pub const TLSEXT_TYPE_padding: i32 = 21;
pub const TLSEXT_TYPE_post_handshake_auth: i32 = 49;
pub const TLSEXT_TYPE_psk: i32 = 41;
pub const TLSEXT_TYPE_psk_kex_modes: i32 = 45;
pub const TLSEXT_TYPE_quic_transport_parameters: i32 = 57;
pub const TLSEXT_TYPE_renegotiate: i32 = 65281;
pub const TLSEXT_TYPE_server_authz: i32 = 8;
pub const TLSEXT_TYPE_server_cert_type: i32 = 20;
pub const TLSEXT_TYPE_server_name: i32 = 0;
pub const TLSEXT_TYPE_session_ticket: i32 = 35;
pub const TLSEXT_TYPE_signature_algorithms: i32 = 13;
pub const TLSEXT_TYPE_signature_algorithms_cert: i32 = 50;
pub const TLSEXT_TYPE_signed_certificate_timestamp: i32 = 18;
pub const TLSEXT_TYPE_srp: i32 = 12;
pub const TLSEXT_TYPE_status_request: i32 = 5;
pub const TLSEXT_TYPE_supported_groups: i32 = 10;
pub const TLSEXT_TYPE_supported_versions: i32 = 43;
pub const TLSEXT_TYPE_truncated_hmac: i32 = 4;
pub const TLSEXT_TYPE_trusted_ca_keys: i32 = 3;
pub const TLSEXT_TYPE_use_srtp: i32 = 14;
pub const TLSEXT_TYPE_user_mapping: i32 = 6;
pub const TLSEXT_cert_type_1609dot2: i32 = 3;
pub const TLSEXT_cert_type_pgp: i32 = 1;
pub const TLSEXT_cert_type_rpk: i32 = 2;
pub const TLSEXT_cert_type_x509: i32 = 0;
pub const TLSEXT_comp_cert_brotli: i32 = 2;
pub const TLSEXT_comp_cert_limit: i32 = 4;
pub const TLSEXT_comp_cert_none: i32 = 0;
pub const TLSEXT_comp_cert_zlib: i32 = 1;
pub const TLSEXT_comp_cert_zstd: i32 = 3;
pub const TLSEXT_curve_P_256: i32 = 23;
pub const TLSEXT_curve_P_384: i32 = 24;
pub const TLSEXT_hash_gostr3411: i32 = 237;
pub const TLSEXT_hash_gostr34112012_256: i32 = 238;
pub const TLSEXT_hash_gostr34112012_512: i32 = 239;
pub const TLSEXT_hash_md5: i32 = 1;
pub const TLSEXT_hash_none: i32 = 0;
pub const TLSEXT_hash_num: i32 = 10;
pub const TLSEXT_hash_sha1: i32 = 2;
pub const TLSEXT_hash_sha224: i32 = 3;
pub const TLSEXT_hash_sha256: i32 = 4;
pub const TLSEXT_hash_sha384: i32 = 5;
pub const TLSEXT_hash_sha512: i32 = 6;
pub const TLSEXT_max_fragment_length_1024: i32 = 2;
pub const TLSEXT_max_fragment_length_2048: i32 = 3;
pub const TLSEXT_max_fragment_length_4096: i32 = 4;
pub const TLSEXT_max_fragment_length_512: i32 = 1;
pub const TLSEXT_max_fragment_length_DISABLED: i32 = 0;
pub const TLSEXT_max_fragment_length_UNSPECIFIED: i32 = 255;
pub const TLSEXT_nid_unknown: i32 = 16777216;
pub const TLSEXT_signature_anonymous: i32 = 0;
pub const TLSEXT_signature_dsa: i32 = 2;
pub const TLSEXT_signature_ecdsa: i32 = 3;
pub const TLSEXT_signature_gostr34102001: i32 = 237;
pub const TLSEXT_signature_gostr34102012_256: i32 = 238;
pub const TLSEXT_signature_gostr34102012_512: i32 = 239;
pub const TLSEXT_signature_num: i32 = 7;
pub const TLSEXT_signature_rsa: i32 = 1;
pub const TLS_ANY_VERSION: i32 = 65536;
pub const TLS_CT_DSS_FIXED_DH: i32 = 4;
pub const TLS_CT_DSS_SIGN: i32 = 2;
pub const TLS_CT_ECDSA_FIXED_ECDH: i32 = 66;
pub const TLS_CT_ECDSA_SIGN: i32 = 64;
pub const TLS_CT_GOST01_SIGN: i32 = 22;
pub const TLS_CT_GOST12_512_SIGN: i32 = 239;
pub const TLS_CT_GOST12_IANA_512_SIGN: i32 = 68;
pub const TLS_CT_GOST12_IANA_SIGN: i32 = 67;
pub const TLS_CT_GOST12_LEGACY_512_SIGN: i32 = 239;
pub const TLS_CT_GOST12_LEGACY_SIGN: i32 = 238;
pub const TLS_CT_GOST12_SIGN: i32 = 238;
pub const TLS_CT_NUMBER: i32 = 12;
pub const TLS_CT_RSA_FIXED_DH: i32 = 3;
pub const TLS_CT_RSA_FIXED_ECDH: i32 = 65;
pub const TLS_CT_RSA_SIGN: i32 = 1;
pub const TLS_MAX_VERSION: i32 = 772;
pub const TLS_MD_CLIENT_FINISH_CONST: *const u8 = [
    99, 108, 105, 101, 110, 116, 32, 102, 105, 110, 105, 115, 104, 101, 100, 0,
]
.as_ptr();
pub const TLS_MD_CLIENT_FINISH_CONST_SIZE: i32 = 15;
pub const TLS_MD_CLIENT_WRITE_KEY_CONST: *const u8 = [
    99, 108, 105, 101, 110, 116, 32, 119, 114, 105, 116, 101, 32, 107, 101, 121, 0,
]
.as_ptr();
pub const TLS_MD_CLIENT_WRITE_KEY_CONST_SIZE: i32 = 16;
pub const TLS_MD_EXTENDED_MASTER_SECRET_CONST: *const u8 = [
    101, 120, 116, 101, 110, 100, 101, 100, 32, 109, 97, 115, 116, 101, 114, 32, 115, 101, 99, 114,
    101, 116, 0,
]
.as_ptr();
pub const TLS_MD_EXTENDED_MASTER_SECRET_CONST_SIZE: i32 = 22;
pub const TLS_MD_IV_BLOCK_CONST: *const u8 = [73, 86, 32, 98, 108, 111, 99, 107, 0].as_ptr();
pub const TLS_MD_IV_BLOCK_CONST_SIZE: i32 = 8;
pub const TLS_MD_KEY_EXPANSION_CONST: *const u8 = [
    107, 101, 121, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 0,
]
.as_ptr();
pub const TLS_MD_KEY_EXPANSION_CONST_SIZE: i32 = 13;
pub const TLS_MD_MASTER_SECRET_CONST: *const u8 = [
    109, 97, 115, 116, 101, 114, 32, 115, 101, 99, 114, 101, 116, 0,
]
.as_ptr();
pub const TLS_MD_MASTER_SECRET_CONST_SIZE: i32 = 13;
pub const TLS_MD_MAX_CONST_SIZE: i32 = 22;
pub const TLS_MD_SERVER_FINISH_CONST: *const u8 = [
    115, 101, 114, 118, 101, 114, 32, 102, 105, 110, 105, 115, 104, 101, 100, 0,
]
.as_ptr();
pub const TLS_MD_SERVER_FINISH_CONST_SIZE: i32 = 15;
pub const TLS_MD_SERVER_WRITE_KEY_CONST: *const u8 = [
    115, 101, 114, 118, 101, 114, 32, 119, 114, 105, 116, 101, 32, 107, 101, 121, 0,
]
.as_ptr();
pub const TLS_MD_SERVER_WRITE_KEY_CONST_SIZE: i32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tls_session_ticket_ext_st {
    pub length: u16,
    pub data: *mut core::ffi::c_void,
}
