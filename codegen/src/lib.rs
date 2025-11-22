//! Generate raw bindings from dawn.json
//!
//! [dawn codegen.md](https://github.com/google/dawn/blob/main/docs/dawn/codegen.md)
//!
//! TODO: download dawn api specs and binaries from https://github.com/google/dawn's releases.

pub mod generator;

mod parser;

pub use parser::*;
