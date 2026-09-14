pub type SSL_COMP = ssl_comp_st;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ssl_comp_st(pub u8);
