use crate::api_model::{ApiModel, FunctionModel};
use crate::emitter::core::*;
use crate::parser::{RecordMember, ReturnType};

pub(crate) fn emit_function(
    f: &FunctionModel,
    model: &ApiModel,
    index: &TypeIndex,
    c_prefix: &str,
) -> String {
    let name = safe_ident(&snake_case_name(&f.name));
    let docs = doc_comment(f.def.comment.as_deref());
    let uses_raw_pointer = f
        .def
        .args
        .iter()
        .any(|arg| arg.length.is_none() && arg.member_type.contains('*'))
        || f.def
            .returns()
            .is_some_and(|ret| ret.get_type().contains('*'));
    let safety = if uses_raw_pointer { "unsafe " } else { "" };
    let safety_docs = if uses_raw_pointer {
        "/// # Safety\n/// All raw pointers must remain valid for the duration required by Dawn.\n"
    } else {
        ""
    };
    let return_ty = f
        .def
        .returns()
        .map(rust_return_type)
        .unwrap_or_else(|| "()".to_string());

    let signature = fn_signature_params(&f.def.args, model, None);
    let (arg_prelude, ffi_args, has_callback) =
        emit_ffi_arg_prelude(&f.def.args, model, index, c_prefix);
    let postlude = emit_out_struct_postlude(&f.def.args, index);

    format!(
        r#"{docs}{safety_docs}pub {safety}fn {name}({signature}) -> {return_ty} {{
{arg_prelude}
{body}
}}

"#,
        name = name,
        docs = docs,
        safety_docs = safety_docs,
        safety = safety,
        signature = signature,
        return_ty = return_ty,
        arg_prelude = indent_block(&arg_prelude, 4),
        body = if has_callback {
            "    unimplemented!()".to_string()
        } else {
            if f.def
                .returns()
                .map(|ret| ret.get_type() == "void")
                .unwrap_or(true)
            {
                let func_name = ffi_fn_name(&f.name, c_prefix);
                let postlude = if postlude.is_empty() {
                    String::new()
                } else {
                    format!("\n{postlude}", postlude = postlude)
                };
                format!(
                    "    unsafe {{ ffi::{func}({args}) }};{postlude}\n    ()",
                    func = func_name,
                    args = ffi_args.join(", "),
                    postlude = postlude
                )
            } else {
                let func_name = ffi_fn_name(&f.name, c_prefix);
                let ffi_call = format!(
                    "    let result = unsafe {{ ffi::{func}({args}) }};",
                    func = func_name,
                    args = ffi_args.join(", ")
                );
                let ret = emit_return_conversion(f.def.returns(), index, "result");
                if postlude.is_empty() {
                    format!("{}\n{}", ffi_call, ret)
                } else {
                    format!("{}\n{}\n{}", ffi_call, postlude, ret)
                }
            }
        }
    )
}

pub(crate) fn fn_signature_params(
    args: &[RecordMember],
    model: &ApiModel,
    receiver: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    let callback_info_map = build_callback_info_map(model);
    let callback_info_mode_map = build_callback_info_mode_map(model);
    let callback_fn_map = build_callback_function_map(model);
    let length_fields = length_field_names(args);

    if let Some(recv) = receiver
        && recv == "self"
    {
        parts.push("&self".to_string());
    }

    for arg in args {
        if let Some(callback_fn_name) = callback_info_map.get(&arg.member_type) {
            if callback_info_mode_map
                .get(&arg.member_type)
                .copied()
                .unwrap_or(false)
            {
                let name = safe_ident(&snake_case_name(&arg.name));
                parts.push(format!("{name}_mode: CallbackMode"));
            }
            let callback_args = callback_fn_map
                .get(callback_fn_name)
                .map(|c| callback_arg_list(&c.def.args))
                .unwrap_or_default();
            parts.push(format!(
                r#"callback: impl FnMut({callback_args}) + Send + 'static"#,
                callback_args = callback_args
            ));
            continue;
        }

        if length_fields.contains(&arg.name) {
            continue;
        }

        let param_name = safe_ident(&snake_case_name(&arg.name));
        let param_ty = rust_param_type(arg);
        let needs_mut = arg.annotation.is_mut_ptr() && (arg.length.is_some() || arg.optional);
        let param_name = if needs_mut {
            format!(r#"mut {param_name}"#, param_name = param_name)
        } else {
            param_name
        };
        parts.push(format!(
            r#"{param_name}: {param_ty}"#,
            param_name = param_name,
            param_ty = param_ty
        ));
    }

    parts.join(", ")
}

pub(crate) fn rust_return_type(ret: &ReturnType) -> String {
    let base = if ret.get_type() == "void" {
        "()".to_string()
    } else {
        rust_type_for(ret.get_type())
    };
    if ret.is_optional() {
        format!(r#"Option<{base}>"#, base = base)
    } else {
        base
    }
}

pub(crate) fn rust_param_type(arg: &RecordMember) -> String {
    let base = if arg.length.is_some() && arg.member_type == "void" {
        "u8".to_string()
    } else {
        rust_type_for(&arg.member_type)
    };

    let mut ty = if arg.length.is_some() {
        if arg.annotation.is_mut_ptr() {
            format!(r#"&mut [{base}]"#, base = base)
        } else {
            format!(r#"&[{base}]"#, base = base)
        }
    } else if arg.member_type.contains('*') {
        base
    } else if arg.annotation.is_const_ptr() {
        format!(r#"&{base}"#, base = base)
    } else if arg.annotation.is_mut_ptr() {
        format!(r#"&mut {base}"#, base = base)
    } else {
        base
    };

    if arg.optional {
        ty = format!(r#"Option<{ty}>"#, ty = ty);
    }

    ty
}
