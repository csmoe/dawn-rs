use crate::api_model::{CallbackFunctionModel, CallbackInfoModel, CallbackModel};
use crate::emitter::core::*;
use crate::parser::RecordMember;

pub(crate) fn emit_callback_function(
    c: &CallbackFunctionModel,
    index: &TypeIndex,
    c_prefix: &str,
) -> String {
    let name = callback_type_name(&c.name);
    let args = callback_arg_list(&c.def.args);
    let signature = format!(r#"{args}"#, args = args);
    let trampoline = callback_trampoline_name(&c.name);
    let ffi_args = callback_ffi_arg_list(&c.def.args, index, c_prefix);

    let mut conversions = Vec::new();
    let mut call_args = Vec::new();

    for arg in &c.def.args {
        let arg_name = safe_ident(&snake_case_name(&arg.name));
        let mut call_arg = arg_name.clone();

        if arg.member_type == "string view" {
            conversions.push(format!(
                r#"    let {arg_name} = string_view_to_string({arg_name});"#,
                arg_name = arg_name
            ));
            call_args.push(call_arg);
            continue;
        }

        if index.is_enum(&arg.member_type) || index.is_bitmask(&arg.member_type) {
            conversions.push(format!(
                r#"    let {arg_name} = {arg_name}.into();"#,
                arg_name = arg_name
            ));
            call_args.push(call_arg);
            continue;
        }

        if index.is_object(&arg.member_type) {
            let obj = type_name(&arg.member_type);
            if arg.length.is_some() {
                let len_expr = length_value_expr(arg.length.as_ref().unwrap());
                conversions.push(format!(
                    r#"    let {arg_name} = if {arg_name}.is_null() {{
        Vec::new()
    }} else {{
        unsafe {{ std::slice::from_raw_parts({arg_name}, {len_expr}) }}
            .iter()
            .map(|raw| unsafe {{ {obj}::from_raw(*raw) }})
            .collect()
    }};"#,
                    arg_name = arg_name,
                    len_expr = len_expr,
                    obj = obj
                ));
                call_args.push(call_arg);
                continue;
            }
            if arg.optional {
                conversions.push(format!(
                    r#"    let {arg_name} = if {arg_name}.is_null() {{
        None
    }} else {{
        Some(unsafe {{ {obj}::from_raw({arg_name}) }})
    }};"#,
                    arg_name = arg_name,
                    obj = obj
                ));
            } else {
                conversions.push(format!(
                    r#"    let {arg_name} = unsafe {{ {obj}::from_raw({arg_name}) }};"#,
                    arg_name = arg_name,
                    obj = obj
                ));
            }
            call_args.push(call_arg);
            continue;
        }

        if index.struct_extensible(&arg.member_type).is_some()
            && (arg.annotation.is_const_ptr() || arg.annotation.is_mut_ptr())
        {
            let rust_ty = type_name(&arg.member_type);
            conversions.push(format!(
                r#"    let {arg_name} = if {arg_name}.is_null() {{
        {rust_ty}::new()
    }} else {{
        {rust_ty}::new()
    }};"#,
                arg_name = arg_name,
                rust_ty = rust_ty
            ));
            if arg.annotation.is_const_ptr() {
                call_arg = format!(r#"&{arg_name}"#, arg_name = arg_name);
            } else if arg.annotation.is_mut_ptr() {
                call_arg = format!(r#"&mut {arg_name}"#, arg_name = arg_name);
            }
            call_args.push(call_arg);
            continue;
        }

        if arg.member_type == "bool" {
            conversions.push(format!(
                r#"    let {arg_name} = {arg_name} != 0;"#,
                arg_name = arg_name
            ));
            call_args.push(call_arg);
            continue;
        }

        call_args.push(call_arg);
    }

    let conversions_block = if conversions.is_empty() {
        String::new()
    } else {
        conversions.join("\n")
    };
    let call_args_block = call_args.join(", ");

    format!(
        r#"pub type {name} = Box<dyn FnMut({signature}) + Send + 'static>;

pub(crate) unsafe extern "C" fn {trampoline}({ffi_args}, userdata1: *mut std::ffi::c_void, userdata2: *mut std::ffi::c_void) {{
    let _ = userdata2;
{conversions}
    let mut callback = unsafe {{ Box::from_raw(userdata1.cast::<Option<{name}>>()) }};
    if let Some(mut callback) = callback.take() {{
        callback({call_args});
    }}
}}

"#,
        name = name,
        signature = signature,
        trampoline = trampoline,
        ffi_args = ffi_args,
        conversions = conversions_block,
        call_args = call_args_block
    )
}

pub(crate) fn emit_callback(c: &CallbackModel) -> String {
    let name = callback_type_name(&c.name);
    let args = callback_arg_list(c.def.args());
    if let Some(ret) = c.def.returns() {
        let ret_ty = rust_return_type(ret);
        return format!(
            r#"pub type {name} = Box<dyn FnMut({args}) -> {ret_ty} + Send + 'static>;

"#,
            name = name,
            args = args,
            ret_ty = ret_ty
        );
    }

    format!(
        r#"pub type {name} = Box<dyn FnMut({args}) + Send + 'static>;

"#,
        name = name,
        args = args
    )
}

pub(crate) fn emit_callback_info(c: &CallbackInfoModel, index: &TypeIndex) -> String {
    let name = type_name(&c.name);

    let mut fields = Vec::new();

    for member in &c.def.members {
        let field_name = safe_ident(&snake_case_name(&member.name));
        let field_ty = if member.name == "callback" {
            format!(
                r#"std::cell::RefCell<Option<{}>>"#,
                callback_type_name(&member.member_type)
            )
        } else {
            struct_field_type(member, index)
        };
        let param_ty = builder_param_type(member, index);
        fields.push(format!(
            r#"    pub {field_name}: {field_ty},"#,
            field_name = field_name,
            field_ty = field_ty
        ));

        let _ = param_ty;
    }

    let fields_block = fields.join("\n");
    let mut defaults = Vec::new();
    for member in &c.def.members {
        let field_name = safe_ident(&snake_case_name(&member.name));
        if member.name == "callback" {
            defaults.push(format!(
                r#"            {field_name}: std::cell::RefCell::new(None),"#,
                field_name = field_name
            ));
        } else {
            defaults.push(format!(
                r#"            {field_name}: None,"#,
                field_name = field_name
            ));
        }
    }
    let defaults_block = defaults.join("\n");

    format!(
        r#"pub struct {name} {{
{fields}
}}

impl Default for {name} {{
    fn default() -> Self {{
        Self {{
{defaults}
        }}
    }}
}}

impl {name} {{
    pub fn new() -> Self {{
        Self::default()
    }}
}}

"#,
        name = name,
        fields = fields_block,
        defaults = defaults_block,
    )
}

pub(crate) fn emit_function_pointer(fp: &crate::api_model::FunctionPointerModel) -> String {
    let name = type_name(&fp.name);
    let args = callback_arg_list(fp.def.args());
    let ret = fp
        .def
        .returns()
        .map(rust_return_type)
        .unwrap_or_else(|| "()".to_string());

    format!(
        r#"pub type {name} = Option<unsafe extern "C" fn({args}) -> {ret}>;

"#,
        name = name,
        args = args,
        ret = ret
    )
}

pub(crate) fn callback_arg_list(args: &[RecordMember]) -> String {
    let mut parts = Vec::new();
    let length_fields = length_field_names(args);
    for arg in args {
        if length_fields.contains(&arg.name) {
            continue;
        }
        let arg_ty = callback_param_type(arg);
        parts.push(arg_ty);
    }
    parts.join(", ")
}

pub(crate) fn callback_type_name(name: &str) -> String {
    let mut result = type_name(name);
    if !result.ends_with("Callback") {
        result.push_str("Callback");
    }
    result
}

pub(crate) fn callback_trampoline_name(name: &str) -> String {
    format!(r#"{}_trampoline"#, safe_ident(&snake_case_name(name)))
}

fn callback_ffi_arg_list(args: &[RecordMember], index: &TypeIndex, c_prefix: &str) -> String {
    let mut parts = Vec::new();
    for arg in args {
        let arg_name = safe_ident(&snake_case_name(&arg.name));
        let arg_ty = callback_ffi_arg_type(arg, index, c_prefix);
        parts.push(format!(
            r#"{arg_name}: {arg_ty}"#,
            arg_name = arg_name,
            arg_ty = arg_ty
        ));
    }
    parts.join(", ")
}

fn callback_ffi_arg_type(arg: &RecordMember, index: &TypeIndex, c_prefix: &str) -> String {
    if arg.member_type == "string view" {
        return format!("ffi::{}StringView", c_prefix);
    }

    if arg.member_type == "bool" {
        return format!("ffi::{}Bool", c_prefix);
    }

    if index.is_object(&arg.member_type) {
        let base = format!("ffi::{}{}", c_prefix, type_name(&arg.member_type));
        if arg.annotation.is_mut_ptr() {
            return format!("*mut {base}", base = base);
        }
        if arg.annotation.is_const_ptr() || arg.length.is_some() {
            return format!("*const {base}", base = base);
        }
        return base;
    }

    if index.is_enum(&arg.member_type) || index.is_bitmask(&arg.member_type) {
        let ffi_ty = ffi_type_name(&arg.member_type, c_prefix);
        return format!("ffi::{ffi_ty}", ffi_ty = ffi_ty);
    }

    if index.struct_extensible(&arg.member_type).is_some() {
        let ffi_ty = ffi_type_name(&arg.member_type, c_prefix);
        let base = format!("ffi::{ffi_ty}", ffi_ty = ffi_ty);
        if arg.annotation.is_const_ptr() {
            return format!("*const {base}", base = base);
        }
        if arg.annotation.is_mut_ptr() {
            return format!("*mut {base}", base = base);
        }
        return base;
    }

    rust_type_for(&arg.member_type)
}

fn callback_param_type(arg: &RecordMember) -> String {
    let base = rust_type_for(&arg.member_type);

    let mut ty = if arg.length.is_some() {
        format!(r#"Vec<{base}>"#, base = base)
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
