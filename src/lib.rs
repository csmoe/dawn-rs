#[allow(dead_code, unused_imports, nonstandard_style)]
#[rustfmt::skip]
mod ffi;

/// Raw, unsafe Dawn C API generated from the matching release header.
pub mod sys {
    pub use crate::ffi::*;
}

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
