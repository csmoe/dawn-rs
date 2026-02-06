pub mod api_model;
pub mod emitter;
#[allow(clippy::all, nonstandard_style)]
pub mod ffi;
pub mod generated;
pub mod parser;
pub mod visitor;

pub use api_model::*;
pub use emitter::*;
pub use generated::*;
pub use visitor::*;

use std::path::Path;

pub fn generate<P: AsRef<Path>, Q: AsRef<Path>>(
    input: P,
    out_dir: Q,
    tags: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let api = parser::DawnJsonParser::parse_file(input)?;
    let filtered = if tags.is_empty() {
        api
    } else {
        api.filter_by_tags(tags)
    };
    let model = api_model::ApiModel::from_api(&filtered);
    emitter::generate_to_dir(&model, out_dir.as_ref())?;
    Ok(())
}
