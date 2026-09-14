pub type ASN1_VALUE = ASN1_VALUE_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ASN1_VALUE_st(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct asn1_string_st {
    pub length: i32,
    pub r#type: i32,
    pub data: *mut u8,
    pub flags: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct asn1_string_table_st {
    pub nid: i32,
    pub minsize: i64,
    pub maxsize: i64,
    pub mask: u64,
    pub flags: u64,
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct asn1_type_st {
    pub r#type: i32,
    pub value: asn1_type_st_0,
}
#[cfg(feature = "types")]
impl Default for asn1_type_st {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub union asn1_type_st_0 {
    pub ptr: *mut i8,
    pub boolean: super::types::ASN1_BOOLEAN,
    pub asn1_string: *mut super::types::ASN1_STRING,
    pub object: *mut super::types::ASN1_OBJECT,
    pub integer: *mut super::types::ASN1_INTEGER,
    pub enumerated: *mut super::types::ASN1_ENUMERATED,
    pub bit_string: *mut super::types::ASN1_BIT_STRING,
    pub octet_string: *mut super::types::ASN1_OCTET_STRING,
    pub printablestring: *mut super::types::ASN1_PRINTABLESTRING,
    pub t61string: *mut super::types::ASN1_T61STRING,
    pub ia5string: *mut super::types::ASN1_IA5STRING,
    pub generalstring: *mut super::types::ASN1_GENERALSTRING,
    pub bmpstring: *mut super::types::ASN1_BMPSTRING,
    pub universalstring: *mut super::types::ASN1_UNIVERSALSTRING,
    pub utctime: *mut super::types::ASN1_UTCTIME,
    pub generalizedtime: *mut super::types::ASN1_GENERALIZEDTIME,
    pub visiblestring: *mut super::types::ASN1_VISIBLESTRING,
    pub utf8string: *mut super::types::ASN1_UTF8STRING,
    pub set: *mut super::types::ASN1_STRING,
    pub sequence: *mut super::types::ASN1_STRING,
    pub asn1_value: *mut ASN1_VALUE,
}
#[cfg(feature = "types")]
impl Default for asn1_type_st_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
