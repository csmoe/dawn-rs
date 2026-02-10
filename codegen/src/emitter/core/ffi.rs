use crate::api_model::ApiModel;
use crate::emitter::core::*;
use crate::parser::{LengthValue, RecordMember, ReturnType};
use std::collections::HashMap;

pub(crate) fn emit_ffi_arg_prelude(
    args: &[RecordMember],
    model: &ApiModel,
    index: &TypeIndex,
    c_prefix: &str,
) -> (String, Vec<String>, bool) {
    let mut prelude = Vec::new();
    let mut ffi_args = Vec::new();
    let mut has_callback = false;
    let callback_info_map = build_callback_info_map(model);
    let callback_info_mode_map = build_callback_info_mode_map(model);
    let mut length_data_map: HashMap<String, usize> = HashMap::new();
    for (idx, arg) in args.iter().enumerate() {
        if let Some(LengthValue::String(name)) = &arg.length {
            length_data_map.insert(name.clone(), idx);
        }
    }

    for arg in args {
        if let Some(data_index) = length_data_map.get(&arg.name) {
            let data_arg = &args[*data_index];
            let data_name = safe_ident(&snake_case_name(&data_arg.name));
            let len_expr = length_expr_for_data_arg(data_arg, &data_name);
            let target_ty = rust_type_for(&arg.member_type);
            let len_expr = if target_ty == "usize" {
                len_expr
            } else {
                format!(
                    "({len_expr}) as {target_ty}",
                    len_expr = len_expr,
                    target_ty = target_ty
                )
            };
            ffi_args.push(len_expr);
            continue;
        }
        if index.is_callback_info(&arg.member_type) {
            if let Some(callback_fn_name) = callback_info_map.get(&arg.member_type) {
                let callback_ty = callback_type_name(callback_fn_name);
                let trampoline = callback_trampoline_name(callback_fn_name);
                let info_name = type_name(&arg.member_type);
                let name = safe_ident(&snake_case_name(&arg.name));
                let has_mode = callback_info_mode_map
                    .get(&arg.member_type)
                    .copied()
                    .unwrap_or(false);
                let mode_line = if has_mode {
                    format!(
                        "            mode: ffi::{prefix}CallbackMode_{prefix}CallbackMode_AllowSpontaneous,\n",
                        prefix = c_prefix
                    )
                } else {
                    String::new()
                };

                prelude.push(format!(
                    r#"        let callback_box: {callback_ty} = Box::new(callback);"#,
                    callback_ty = callback_ty
                ));
                prelude.push(
                    r#"        let callback_box = Box::new(Some(callback_box));"#.to_string(),
                );
                prelude.push(
                    r#"        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();"#
                        .to_string(),
                );
                prelude.push(format!(
                    r#"        let {name}_ffi = ffi::{c_prefix}{info_name} {{
            nextInChain: std::ptr::null_mut(),
{mode_line}            callback: Some({trampoline}),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        }};"#,
                    name = name,
                    info_name = info_name,
                    trampoline = trampoline,
                    mode_line = mode_line,
                    c_prefix = c_prefix
                ));
                ffi_args.push(format!("{name}_ffi", name = name));
            } else {
                has_callback = true;
                prelude.push("        let _ = callback;".to_string());
                ffi_args.push("std::ptr::null_mut()".to_string());
            }
            continue;
        }

        let name = safe_ident(&snake_case_name(&arg.name));
        if arg.length.is_some() {
            if index.struct_extensible(&arg.member_type).is_some() {
                let ffi_ty = ffi_type_name(&arg.member_type, c_prefix);
                let ptr_expr = if arg.annotation.is_mut_ptr() {
                    format!("{name}_raw.as_mut_ptr()", name = name)
                } else {
                    format!("{name}_raw.as_ptr()", name = name)
                };
                let null_ptr = if arg.annotation.is_mut_ptr() {
                    "std::ptr::null_mut()"
                } else {
                    "std::ptr::null()"
                };
                if arg.optional {
                    prelude.push(format!(
                        r#"        let mut {name}_raw: Vec<ffi::{ffi_ty}> = Vec::new();
        let mut {name}_storage: Vec<ChainedStructStorage> = Vec::new();
        let {name}_ptr = if let Some(value) = {name}.as_deref() {{
            for item in value {{
                let (raw, storage) = item.to_ffi();
                {name}_raw.push(raw);
                {name}_storage.push(storage);
            }}
            {ptr_expr}
        }} else {{
            {null_ptr}
        }};"#,
                        name = name,
                        ffi_ty = ffi_ty,
                        ptr_expr = ptr_expr,
                        null_ptr = null_ptr
                    ));
                } else {
                    prelude.push(format!(
                        r#"        let mut {name}_raw: Vec<ffi::{ffi_ty}> = Vec::new();
        let mut {name}_storage: Vec<ChainedStructStorage> = Vec::new();
        for item in {name}.iter() {{
            let (raw, storage) = item.to_ffi();
            {name}_raw.push(raw);
            {name}_storage.push(storage);
        }}
        let {name}_ptr = {ptr_expr};"#,
                        name = name,
                        ffi_ty = ffi_ty,
                        ptr_expr = ptr_expr
                    ));
                }
                ffi_args.push(format!("{name}_ptr", name = name));
                continue;
            }

            if index.is_object(&arg.member_type) {
                let obj = type_name(&arg.member_type);
                let ptr_expr = if arg.annotation.is_mut_ptr() {
                    format!("{name}_raw.as_mut_ptr()", name = name)
                } else {
                    format!("{name}_raw.as_ptr()", name = name)
                };
                let null_ptr = if arg.annotation.is_mut_ptr() {
                    "std::ptr::null_mut()"
                } else {
                    "std::ptr::null()"
                };
                if arg.optional {
                    prelude.push(format!(
                        r#"        let mut {name}_raw: Vec<ffi::{prefix}{obj}> = Vec::new();
        let {name}_ptr = if let Some(value) = {name}.as_deref() {{
            {name}_raw = value.iter().map(|v| v.as_raw()).collect();
            {ptr_expr}
        }} else {{
            {null_ptr}
        }};"#,
                        name = name,
                        obj = obj,
                        prefix = c_prefix,
                        ptr_expr = ptr_expr,
                        null_ptr = null_ptr
                    ));
                } else {
                    prelude.push(format!(
                        r#"        let mut {name}_raw: Vec<ffi::{prefix}{obj}> = {name}.iter().map(|v| v.as_raw()).collect();
        let {name}_ptr = {ptr_expr};"#,
                        name = name,
                        obj = obj,
                        prefix = c_prefix,
                        ptr_expr = ptr_expr
                    ));
                }
                ffi_args.push(format!("{name}_ptr", name = name));
                continue;
            }

            let null_ptr = if arg.annotation.is_mut_ptr() {
                "std::ptr::null_mut()"
            } else {
                "std::ptr::null()"
            };
            if arg.optional {
                let deref = if arg.annotation.is_mut_ptr() {
                    "as_deref_mut()"
                } else {
                    "as_deref()"
                };
                let ptr_expr = if arg.annotation.is_mut_ptr() {
                    "value.as_mut_ptr()"
                } else {
                    "value.as_ptr()"
                };
                prelude.push(format!(
                    r#"        let {name}_ptr = if let Some(value) = {name}.{deref} {{
            {ptr_expr}
        }} else {{
            {null_ptr}
        }};"#,
                    name = name,
                    deref = deref,
                    ptr_expr = ptr_expr,
                    null_ptr = null_ptr
                ));
            } else {
                let ptr_expr = if arg.annotation.is_mut_ptr() {
                    format!("{name}.as_mut_ptr()", name = name)
                } else {
                    format!("{name}.as_ptr()", name = name)
                };
                prelude.push(format!(
                    r#"        let {name}_ptr = {ptr_expr};"#,
                    name = name,
                    ptr_expr = ptr_expr
                ));
            }
            ffi_args.push(format!("{name}_ptr", name = name));
            continue;
        }
        if index.is_object(&arg.member_type) {
            if arg.optional {
                prelude.push(format!(
                    r#"        let {name}_raw = {name}.as_ref().map(|v| v.as_raw()).unwrap_or(std::ptr::null_mut());"#,
                    name = name
                ));
                ffi_args.push(format!("{name}_raw", name = name));
            } else {
                ffi_args.push(format!("{name}.as_raw()", name = name));
            }
            continue;
        }

        if index.is_enum(&arg.member_type) || index.is_bitmask(&arg.member_type) {
            let ffi_ty = ffi_type_name(&arg.member_type, c_prefix);
            prelude.push(format!(
                r#"        let {name}_ffi: ffi::{ffi_ty} = {name}.into();"#,
                name = name,
                ffi_ty = ffi_ty
            ));
            ffi_args.push(format!("{name}_ffi", name = name));
            continue;
        }

        if arg.member_type == "string view" {
            if arg.optional {
                prelude.push(format!(
                    r#"        let {name}_ffi = if let Some(value) = &{name} {{
            ffi::{prefix}StringView {{ data: value.as_ptr().cast(), length: value.len() }}
        }} else {{
            ffi::{prefix}StringView {{ data: std::ptr::null(), length: 0 }}
        }};"#,
                    name = name,
                    prefix = c_prefix
                ));
            } else {
                prelude.push(format!(
                    r#"        let {name}_ffi = ffi::{prefix}StringView {{
            data: {name}.as_ptr().cast(),
            length: {name}.len(),
        }};"#,
                    name = name,
                    prefix = c_prefix
                ));
            }
            ffi_args.push(format!("{name}_ffi", name = name));
            continue;
        }

        if arg.member_type == "bool" {
            prelude.push(format!(
                r#"        let {name}_ffi: ffi::{prefix}Bool = if {name} {{ 1 }} else {{ 0 }};"#,
                name = name,
                prefix = c_prefix
            ));
            ffi_args.push(format!("{name}_ffi", name = name));
            continue;
        }

        if index.struct_extensible(&arg.member_type).is_some() {
            if arg.optional {
                if arg.annotation.is_const_ptr() || arg.annotation.is_mut_ptr() {
                    let (ptr_ctor, null_ptr) = if arg.annotation.is_mut_ptr() {
                        ("std::ptr::addr_of_mut!", "std::ptr::null_mut()")
                    } else {
                        ("std::ptr::addr_of!", "std::ptr::null()")
                    };
                    prelude.push(format!(
                        r#"        let mut {name}_storage = ChainedStructStorage::new();
        let {name}_ptr = if let Some(value) = &{name} {{
            let ({name}_ffi, storage) = value.to_ffi();
            {name}_storage = storage;
            {ptr_ctor}({name}_ffi)
        }} else {{
            {null_ptr}
        }};"#,
                        name = name,
                        ptr_ctor = ptr_ctor,
                        null_ptr = null_ptr
                    ));
                    ffi_args.push(format!("{name}_ptr", name = name));
                    continue;
                }
                prelude.push(format!(r#"        let _ = {name};"#, name = name));
                ffi_args.push("std::ptr::null()".to_string());
                continue;
            }

            if arg.annotation.is_const_ptr() {
                prelude.push(format!(
                    r#"        let ({name}_ffi, _{name}_storage) = {name}.to_ffi();"#,
                    name = name
                ));
                prelude.push(format!(
                    r#"        let {name}_ptr = std::ptr::addr_of!({name}_ffi);"#,
                    name = name
                ));
                ffi_args.push(format!("{name}_ptr", name = name));
                continue;
            }

            if arg.annotation.is_mut_ptr() {
                prelude.push(format!(
                    r#"        let (mut {name}_ffi, _{name}_storage) = {name}.to_ffi();"#,
                    name = name
                ));
                prelude.push(format!(
                    r#"        let {name}_ptr = std::ptr::addr_of_mut!({name}_ffi);"#,
                    name = name
                ));
                ffi_args.push(format!("{name}_ptr", name = name));
                continue;
            }

            prelude.push(format!(
                r#"        let ({name}_ffi, _{name}_storage) = {name}.to_ffi();"#,
                name = name
            ));
            ffi_args.push(format!("{name}_ffi", name = name));
            continue;
        }

        ffi_args.push(name);
    }

    (prelude.join("\n"), ffi_args, has_callback)
}

pub(crate) fn emit_out_struct_postlude(args: &[RecordMember], index: &TypeIndex) -> String {
    let mut lines = Vec::new();
    for arg in args {
        if arg.optional {
            continue;
        }
        if !arg.annotation.is_mut_ptr() {
            continue;
        }
        if index.struct_extensible(&arg.member_type).is_none() {
            continue;
        }
        let name = safe_ident(&snake_case_name(&arg.name));
        let ty = type_name(&arg.member_type);
        lines.push(format!(
            r#"        *{name} = {ty}::from_ffi({name}_ffi);"#,
            name = name,
            ty = ty
        ));
        let _ = index.struct_needs_free_members(&arg.member_type);
    }
    lines.join("\n")
}

pub(crate) fn emit_return_conversion(ret: Option<&ReturnType>, index: &TypeIndex, name: &str) -> String {
    let Some(ret) = ret else {
        return "        let _ = ();".to_string();
    };

    let ty = ret.get_type();
    if index.is_object(ty) {
        let obj = type_name(ty);
        if ret.is_optional() {
            return format!(
                r#"        if {name}.is_null() {{
            None
        }} else {{
            Some(unsafe {{ {obj}::from_raw({name}) }})
        }}"#,
                name = name,
                obj = obj
            );
        }
        return format!(
            r#"        unsafe {{ {obj}::from_raw({name}) }}"#,
            name = name,
            obj = obj
        );
    }

    if index.is_enum(ty) || index.is_bitmask(ty) {
        return format!(r#"        {name}.into()"#, name = name);
    }

    if index.struct_extensible(ty).is_some() {
        return format!(
            r#"        {ty}::from_ffi({name})"#,
            ty = type_name(ty),
            name = name
        );
    }

    if ret.get_type() == "void" {
        return "        let _ = ();".to_string();
    }

    if ret.get_type() == "bool" {
        return format!(r#"        {name} != 0"#, name = name);
    }

    format!(r#"        {name}"#, name = name)
}

pub(crate) fn ffi_field_name(name: &str) -> String {
    if !name.contains(' ') && name.chars().any(|c| c.is_ascii_uppercase()) {
        return name.to_string();
    }
    let parts: Vec<&str> = name.split_whitespace().collect();
    if parts.is_empty() {
        return String::new();
    }
    let mut out = if parts[0].len() == 1 && parts[0].chars().all(|c| c.is_ascii_uppercase()) {
        parts[0].to_string()
    } else {
        parts[0].to_lowercase()
    };
    for part in parts.iter().skip(1) {
        if part.eq_ignore_ascii_case("id") {
            out.push_str("ID");
            continue;
        }
        if part.len() == 1 {
            out.push_str(&part.to_ascii_uppercase());
            continue;
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.push(first.to_ascii_uppercase());
            out.extend(chars);
        }
    }
    if out == "type" {
        "type_".to_string()
    } else {
        out
    }
}

pub(crate) fn ffi_length_expr(length: &LengthValue) -> String {
    match length {
        LengthValue::Number(value) => format!("{value}usize", value = value),
        LengthValue::String(name) => format!("value.{} as usize", ffi_field_name(name)),
    }
}

pub(crate) fn free_members_fn_name(struct_name: &str) -> String {
    format!("wgpu{}FreeMembers", type_name(struct_name))
}
