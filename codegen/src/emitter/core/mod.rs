use crate::api_model::ConstantModel;
use serde_json::Value;
mod callbacks;
mod ffi;
mod functions;
mod helpers;
mod index;
mod names;
mod numbers;
mod objects;
mod structs;

pub(crate) use callbacks::*;
pub(crate) use ffi::*;
pub(crate) use functions::*;
pub(crate) use helpers::*;
pub(crate) use index::*;
pub(crate) use names::*;
pub(crate) use numbers::*;
pub(crate) use objects::*;
pub(crate) use structs::*;

pub(crate) fn emit_constant(c: &ConstantModel) -> Option<String> {
    let name = shouty_snake_case_name(&c.name);
    let ty = rust_type_for(&c.def.const_type);
    let value = match &c.def.value {
        Value::Number(num) => num.to_string(),
        Value::String(s) => {
            if let Some(parsed) = parse_numeric_string(s) {
                parsed.to_string()
            } else if let Some(parsed) = parse_numeric_string_u64(s) {
                parsed.to_string()
            } else {
                match s.as_str() {
                    "UINT32_MAX" => "u32::MAX".to_string(),
                    "UINT64_MAX" => "u64::MAX".to_string(),
                    "SIZE_MAX" => "usize::MAX".to_string(),
                    "NAN" => {
                        if ty == "f32" {
                            "f32::NAN".to_string()
                        } else {
                            "f64::NAN".to_string()
                        }
                    }
                    _ => return None,
                }
            }
        }
        _ => return None,
    };

    Some(format!(
        r#"pub const {name}: {ty} = {value};

"#,
        name = name,
        ty = ty,
        value = value
    ))
}

// args_usage_list removed: superseded by emit_ffi_arg_prelude.

pub(crate) fn ffi_fn_name(canonical: &str, c_prefix: &str) -> String {
    format!(
        "{}{}",
        c_prefix.to_lowercase(),
        camel_case_with_acronyms(canonical)
    )
}
