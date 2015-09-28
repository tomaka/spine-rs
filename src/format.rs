#![allow(dead_code)]
#![allow(non_snake_case)]

#[cfg(feature = "serde_macros")]
include!("format.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/format.rs"));
