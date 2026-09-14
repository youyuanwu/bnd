windows_link::link!("crypto" "C" fn SHA1(d : *const u8, n : usize, md : *mut u8) -> *mut u8);
windows_link::link!("crypto" "C" fn SHA1_Final(md : *mut u8, c : *mut SHA_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA1_Init(c : *mut SHA_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA1_Transform(c : *mut SHA_CTX, data : *const u8));
windows_link::link!("crypto" "C" fn SHA1_Update(c : *mut SHA_CTX, data : *const core::ffi::c_void, len : usize) -> i32);
windows_link::link!("crypto" "C" fn SHA224(d : *const u8, n : usize, md : *mut u8) -> *mut u8);
windows_link::link!("crypto" "C" fn SHA224_Final(md : *mut u8, c : *mut SHA256_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA224_Init(c : *mut SHA256_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA224_Update(c : *mut SHA256_CTX, data : *const core::ffi::c_void, len : usize) -> i32);
windows_link::link!("crypto" "C" fn SHA256(d : *const u8, n : usize, md : *mut u8) -> *mut u8);
windows_link::link!("crypto" "C" fn SHA256_Final(md : *mut u8, c : *mut SHA256_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA256_Init(c : *mut SHA256_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA256_Transform(c : *mut SHA256_CTX, data : *const u8));
windows_link::link!("crypto" "C" fn SHA256_Update(c : *mut SHA256_CTX, data : *const core::ffi::c_void, len : usize) -> i32);
windows_link::link!("crypto" "C" fn SHA384(d : *const u8, n : usize, md : *mut u8) -> *mut u8);
windows_link::link!("crypto" "C" fn SHA384_Final(md : *mut u8, c : *mut SHA512_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA384_Init(c : *mut SHA512_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA384_Update(c : *mut SHA512_CTX, data : *const core::ffi::c_void, len : usize) -> i32);
windows_link::link!("crypto" "C" fn SHA512(d : *const u8, n : usize, md : *mut u8) -> *mut u8);
windows_link::link!("crypto" "C" fn SHA512_Final(md : *mut u8, c : *mut SHA512_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA512_Init(c : *mut SHA512_CTX) -> i32);
windows_link::link!("crypto" "C" fn SHA512_Transform(c : *mut SHA512_CTX, data : *const u8));
windows_link::link!("crypto" "C" fn SHA512_Update(c : *mut SHA512_CTX, data : *const core::ffi::c_void, len : usize) -> i32);
pub const SHA224_DIGEST_LENGTH: i32 = 28;
pub const SHA256_192_DIGEST_LENGTH: i32 = 24;
pub const SHA256_CBLOCK: i32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHA256_CTX {
    pub h: [u32; 8],
    pub Nl: u32,
    pub Nh: u32,
    pub data: [u32; 16],
    pub num: u32,
    pub md_len: u32,
}
impl Default for SHA256_CTX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHA256_DIGEST_LENGTH: i32 = 32;
pub const SHA384_DIGEST_LENGTH: i32 = 48;
pub const SHA512_CBLOCK: i32 = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHA512_CTX {
    pub h: [u64; 8],
    pub Nl: u64,
    pub Nh: u64,
    pub u: SHA512_CTX_0,
    pub num: u32,
    pub md_len: u32,
}
impl Default for SHA512_CTX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SHA512_CTX_0 {
    pub d: [u64; 16],
    pub p: [u8; 128],
}
impl Default for SHA512_CTX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHA512_DIGEST_LENGTH: i32 = 64;
pub const SHA_CBLOCK: i32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHA_CTX {
    pub h0: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub Nl: u32,
    pub Nh: u32,
    pub data: [u32; 16],
    pub num: u32,
}
impl Default for SHA_CTX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SHA_DIGEST_LENGTH: i32 = 20;
pub const SHA_LAST_BLOCK: i32 = 56;
pub const SHA_LBLOCK: i32 = 16;
