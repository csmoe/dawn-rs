#[allow(dead_code, unused_imports, nonstandard_style)]
mod ffi {
    include!("./generated/ffi.rs");
}

#[allow(unused)]
mod generated;

pub use generated::*;
