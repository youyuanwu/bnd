//! Build support for locating the shared native test fixture.

/// Directory containing the native fixture built by `build.rs`.
pub fn lib_dir() -> &'static str {
    env!("OUT_DIR")
}
