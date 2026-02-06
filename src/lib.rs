mod api_model;
mod emitter;
mod parser;
mod visitor;

pub(crate) use api_model::*;
pub(crate) use emitter::*;
pub(crate) use visitor::*;

use std::path::Path;
use std::path::PathBuf;

pub(crate) struct GeneratorConfig {
    pub input: PathBuf,
    pub out_dir: PathBuf,
    pub tags: Vec<String>,
}

impl GeneratorConfig {
    pub fn new(input: impl AsRef<Path>, out_dir: impl AsRef<Path>, tags: Vec<String>) -> Self {
        Self {
            input: input.as_ref().to_path_buf(),
            out_dir: out_dir.as_ref().to_path_buf(),
            tags,
        }
    }
}

pub(crate) fn generate_strings<P: AsRef<Path>>(
    input: P,
    tags: &[String],
) -> Result<GeneratedFiles, Box<dyn std::error::Error>> {
    let api = parser::DawnJsonParser::parse_file(input)?;
    let filtered = if tags.is_empty() {
        api
    } else {
        api.filter_by_tags(tags)
    };
    let model = api_model::ApiModel::from_api(&filtered);
    Ok(emitter::generate_strings(&model))
}

pub(crate) fn generate_to_dir<P: AsRef<Path>, Q: AsRef<Path>>(
    input: P,
    out_dir: Q,
    tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let files = generate_strings(input, tags)?;
    files.write_to_dir(out_dir.as_ref())?;
    Ok(())
}

pub(crate) fn generate_with_config(
    config: &GeneratorConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    generate_to_dir(&config.input, &config.out_dir, &config.tags)
}

mod ffi {
    #![allow(clippy::all, nonstandard_style)]
    include!(concat!(env!("OUT_DIR"), "/generated/ffi.rs"));
}

mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}

pub use generated::*;
