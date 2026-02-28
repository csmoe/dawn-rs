#[allow(dead_code, unused_imports, nonstandard_style)]
#[rustfmt::skip]
mod ffi;

#[allow(unused)]
#[rustfmt::skip]
mod generated;

pub use generated::*;

#[cfg(feature = "wire")]
pub mod wire;
#[cfg(feature = "wire")]
mod wire_backend;
#[cfg(feature = "wire")]
mod wire_ipc;
#[cfg(feature = "wire")]
mod wire_shim;
