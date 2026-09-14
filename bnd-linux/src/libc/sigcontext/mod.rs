pub const FP_XSTATE_MAGIC1: u32 = 1179670611;
pub const FP_XSTATE_MAGIC2: u32 = 1179670597;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _fpreg {
    pub significand: [u16; 4],
    pub exponent: u16,
}
impl Default for _fpreg {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct _fpstate {
    pub cwd: super::types::__uint16_t,
    pub swd: super::types::__uint16_t,
    pub ftw: super::types::__uint16_t,
    pub fop: super::types::__uint16_t,
    pub rip: super::types::__uint64_t,
    pub rdp: super::types::__uint64_t,
    pub mxcsr: super::types::__uint32_t,
    pub mxcr_mask: super::types::__uint32_t,
    pub _st: [_fpxreg; 8],
    pub _xmm: [_xmmreg; 16],
    pub __glibc_reserved1: [super::types::__uint32_t; 24],
}
#[cfg(feature = "types")]
impl Default for _fpstate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct _fpx_sw_bytes {
    pub magic1: super::types::__uint32_t,
    pub extended_size: super::types::__uint32_t,
    pub xstate_bv: super::types::__uint64_t,
    pub xstate_size: super::types::__uint32_t,
    pub __glibc_reserved1: [super::types::__uint32_t; 7],
}
#[cfg(feature = "types")]
impl Default for _fpx_sw_bytes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _fpxreg {
    pub significand: [u16; 4],
    pub exponent: u16,
    pub __glibc_reserved1: [u16; 3],
}
impl Default for _fpxreg {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct _xmmreg {
    pub element: [super::types::__uint32_t; 4],
}
#[cfg(feature = "types")]
impl Default for _xmmreg {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct _xsave_hdr {
    pub xstate_bv: super::types::__uint64_t,
    pub __glibc_reserved1: [super::types::__uint64_t; 2],
    pub __glibc_reserved2: [super::types::__uint64_t; 5],
}
#[cfg(feature = "types")]
impl Default for _xsave_hdr {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy, Default)]
pub struct _xstate {
    pub fpstate: _fpstate,
    pub xstate_hdr: _xsave_hdr,
    pub ymmh: _ymmh_state,
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct _ymmh_state {
    pub ymmh_space: [super::types::__uint32_t; 64],
}
#[cfg(feature = "types")]
impl Default for _ymmh_state {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub struct sigcontext {
    pub r8: super::types::__uint64_t,
    pub r9: super::types::__uint64_t,
    pub r10: super::types::__uint64_t,
    pub r11: super::types::__uint64_t,
    pub r12: super::types::__uint64_t,
    pub r13: super::types::__uint64_t,
    pub r14: super::types::__uint64_t,
    pub r15: super::types::__uint64_t,
    pub rdi: super::types::__uint64_t,
    pub rsi: super::types::__uint64_t,
    pub rbp: super::types::__uint64_t,
    pub rbx: super::types::__uint64_t,
    pub rdx: super::types::__uint64_t,
    pub rax: super::types::__uint64_t,
    pub rcx: super::types::__uint64_t,
    pub rsp: super::types::__uint64_t,
    pub rip: super::types::__uint64_t,
    pub eflags: super::types::__uint64_t,
    pub cs: u16,
    pub gs: u16,
    pub fs: u16,
    pub __pad0: u16,
    pub err: super::types::__uint64_t,
    pub trapno: super::types::__uint64_t,
    pub oldmask: super::types::__uint64_t,
    pub cr2: super::types::__uint64_t,
    pub Anonymous: sigcontext_0,
    pub __reserved1: [super::types::__uint64_t; 8],
}
#[cfg(feature = "types")]
impl Default for sigcontext {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "types")]
#[derive(Clone, Copy)]
pub union sigcontext_0 {
    pub fpstate: *mut _fpstate,
    pub __fpstate_word: super::types::__uint64_t,
}
#[cfg(feature = "types")]
impl Default for sigcontext_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
