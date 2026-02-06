#[allow(clippy::all, nonstandard_style, unused)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/generated/ffi.rs"));
}

#[allow(unused)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}

pub use generated::*;
