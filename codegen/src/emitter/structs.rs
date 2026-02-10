use crate::api_model::ApiModel;

use crate::emitter::core::{
    build_callback_info_map, build_callback_info_mode_map, build_constant_map, emit_struct, TypeIndex,
};

pub(crate) fn emit(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use crate::generated::*;
use std::any::Any;
use std::ffi::CStr;

pub(crate) fn string_view_to_string(view: ffi::{prefix}StringView) -> String {{
    if view.data.is_null() || view.length == 0 {{
        return String::new();
    }}
    let data = view.data.cast::<u8>();
    let slice = unsafe {{ std::slice::from_raw_parts(data, view.length) }};
    String::from_utf8_lossy(slice).into_owned()
}}

"#,
        prefix = c_prefix
    ));

    let index = TypeIndex::new(model);
    let constant_map = build_constant_map(model);
    let callback_info_map = build_callback_info_map(model);
    let callback_info_mode_map = build_callback_info_mode_map(model);
    for s in &model.structures {
        out.push_str(&emit_struct(
            s,
            &index,
            c_prefix,
            &callback_info_map,
            &callback_info_mode_map,
            &constant_map,
        ));
    }

    out
}
