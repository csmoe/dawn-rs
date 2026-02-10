use crate::api_model::ApiModel;

use crate::emitter::core::emit_constant;

pub(crate) fn emit(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::generated::*;
use crate::ffi;
"#,
    );

    for c in &model.constants {
        if let Some(line) = emit_constant(c) {
            out.push_str(&line);
            out.push('\n');
        }
    }

    out
}
