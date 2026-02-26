use crate::api_model::ApiModel;

use crate::emitter::core::{TypeIndex, emit_function};

pub(crate) fn emit(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::generated::*;
use crate::ffi;
"#,
    );

    let index = TypeIndex::new(model);
    for f in &model.functions {
        out.push_str(&emit_function(f, model, &index, c_prefix));
    }

    out
}
