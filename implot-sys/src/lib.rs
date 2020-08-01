#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// TODO(4bb4) change this to include the bindings we hand-generate
// once that is happening
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
