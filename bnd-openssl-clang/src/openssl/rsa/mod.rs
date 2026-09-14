#[repr(C)]
#[cfg(all(feature = "asn1", feature = "types", feature = "x509"))]
#[derive(Clone, Copy, Default)]
pub struct rsa_pss_params_st {
    pub hashAlgorithm: *mut super::types::X509_ALGOR,
    pub maskGenAlgorithm: *mut super::types::X509_ALGOR,
    pub saltLength: *mut super::types::ASN1_INTEGER,
    pub trailerField: *mut super::types::ASN1_INTEGER,
    pub maskHash: *mut super::types::X509_ALGOR,
}
