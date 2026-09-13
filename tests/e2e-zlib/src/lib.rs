#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

mod bindings;
pub use bindings::*;

extern crate bnd_macros as windows_link;
