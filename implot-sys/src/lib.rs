#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// just for linking for tests
#[cfg(test)]
use imgui_sys as _;

include!("bindings.rs");
