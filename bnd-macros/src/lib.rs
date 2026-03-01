#![no_std]

/// Declares an external function and links the named shared library.
///
/// Emits `#[link(name = "...")]` so the linker automatically pulls in
/// the library (e.g. `-lcrypto`).
///
/// # Example
///
/// ```ignore
/// bnd_macros::link!("crypto" "C" fn EVP_DigestInit(ctx: *mut EVP_MD_CTX, type_: *const EVP_MD) -> i32);
/// ```
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

/// Declares an external function without emitting any link attributes.
///
/// The library name is accepted for syntax compatibility but ignored.
/// The consumer is responsible for ensuring the symbol is available at
/// link time (e.g. via a build script, `-l` in RUSTFLAGS, or static
/// archive).
///
/// # Example
///
/// ```ignore
/// bnd_macros::link_raw!("c" "C" fn creat(__file: *const i8, __mode: u32) -> i32);
/// ```
#[macro_export]
macro_rules! link_raw {
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}
