#[allow(dead_code, unused_imports, nonstandard_style)]
#[rustfmt::skip]
mod ffi;

#[allow(unused)]
#[rustfmt::skip]
mod generated;

pub use generated::*;

#[cfg(feature = "wire")]
pub mod wire_shim;
