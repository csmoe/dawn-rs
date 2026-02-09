pub mod api_model;
pub mod emitter;
pub mod parser;
pub mod visitor;

pub use api_model::ApiModel;
pub use emitter::{generate_strings, generate_strings_with_ffi_consts, GeneratedFiles};
pub use parser::DawnJsonParser;

use std::path::Path;

pub fn generate_ffi_string(
    api_header: &Path,
    clang_args: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut builder = bindgen::Builder::default().header(api_header.to_string_lossy());
    for arg in clang_args {
        builder = builder.clang_arg(arg);
    }
    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;
    Ok(bindings.to_string())
}
