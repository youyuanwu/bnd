#[repr(C)]
#[cfg(all(feature = "asn1", feature = "types"))]
#[derive(Clone, Copy, Default)]
pub struct X509_algor_st {
    pub algorithm: *mut super::types::ASN1_OBJECT,
    pub parameter: *mut super::types::ASN1_TYPE,
}
