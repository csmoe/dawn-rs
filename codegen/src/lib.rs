pub mod api_model;
pub mod emitter;
pub mod parser;
pub mod visitor;

pub use api_model::ApiModel;
pub use emitter::{GeneratedFiles, format_rust_source, generate_strings};
pub use parser::DawnJsonParser;

use std::path::Path;

pub fn generate_ffi_string(
    api_header: &Path,
    clang_args: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let api_header_str = api_header.to_string_lossy().replace('\\', "\\\\");
    let wrapper = format!(
        r#"#include "{api_header_str}"
        #include <dawn/dawn_proc_table.h>
       "#,
    );
    let mut builder = bindgen::Builder::default()
        .header_contents("dawn_rs_bindgen_wrapper.h", &wrapper)
        .allowlist_item("WGPU.*")
        .allowlist_item("wgpu.*")
        .allowlist_item("DawnProcTable")
        .allowlist_item("dawnProcSetProcs");
    for arg in clang_args {
        builder = builder.clang_arg(arg);
    }
    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;
    Ok(bindings.to_string())
}
