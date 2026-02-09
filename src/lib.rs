#[allow(dead_code, unused_imports, nonstandard_style)]
mod ffi {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/ffi.rs"));
}

mod generated;

pub use generated::*;
