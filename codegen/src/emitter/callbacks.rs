use crate::api_model::ApiModel;

use crate::emitter::core::{
    TypeIndex, emit_callback, emit_callback_function, emit_callback_info, emit_function_pointer,
};

pub(crate) fn emit(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use crate::generated::*;
"#,
    );

    let index = TypeIndex::new(model);
    for f in &model.function_pointers {
        out.push_str(&emit_function_pointer(f));
    }
    for c in &model.callback_functions {
        out.push_str(&emit_callback_function(c, &index, c_prefix));
    }
    for c in &model.callback_infos {
        out.push_str(&emit_callback_info(c, &index));
    }
    for c in &model.callbacks {
        out.push_str(&emit_callback(c));
    }

    out
}
