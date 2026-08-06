use crate::api_model::{CallbackFunctionModel, CallbackInfoModel, CallbackModel};
use crate::emitter::core::*;
use crate::parser::RecordMember;

pub(crate) fn emit_callback_function(
    c: &CallbackFunctionModel,
    index: &TypeIndex,
    c_prefix: &str,
) -> String {
    let name = callback_type_name(&c.name);
    let args = callback_arg_list_for_callback(&c.name, &c.def.args);
    let signature = args.to_string();
    let trampoline = callback_trampoline_name(&c.name);
    let ffi_args = callback_ffi_arg_list(&c.def.args, index, c_prefix);
    let return_ty = c
        .def
        .returns()
        .map(rust_return_type)
        .unwrap_or_else(|| "()".to_string());
    let trampoline_return = if return_ty == "()" {
        String::new()
    } else {
        format!(" -> {return_ty}")
    };
    let default_return = if return_ty == "()" {
        String::new()
    } else {
        "    Default::default()\n".to_string()
    };

    let mut conversions = Vec::new();
    let mut call_args = Vec::new();
    let length_fields = length_field_names(&c.def.args);

    for arg in &c.def.args {
        if length_fields.contains(&arg.name) {
            continue;
        }
        let arg_name = safe_ident(&snake_case_name(&arg.name));
        let mut call_arg = arg_name.clone();

        if let Some(length) = &arg.length
            && !index.is_object(&arg.member_type)
        {
            let len_expr = length_value_expr(length);
            if arg.annotation.is_mut_ptr() {
                conversions.push(format!(
                    r#"    let mut {arg_name}_empty = [];
    let {arg_name} = if {arg_name}.is_null() {{
        &mut {arg_name}_empty[..]
    }} else {{
        unsafe {{ std::slice::from_raw_parts_mut({arg_name}, {len_expr}) }}
    }};"#,
                    arg_name = arg_name,
                    len_expr = len_expr
                ));
            } else {
                conversions.push(format!(
                    r#"    let {arg_name} = if {arg_name}.is_null() {{
        &[]
    }} else {{
        unsafe {{ std::slice::from_raw_parts({arg_name}, {len_expr}) }}
    }};"#,
                    arg_name = arg_name,
                    len_expr = len_expr
                ));
            }
            call_args.push(call_arg);
            continue;
        }

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
            if callback_object_is_borrowed(&c.name, arg) {
                if let Some(length) = &arg.length {
                    let len_expr = length_value_expr(length);
                    conversions.push(format!(
                        r#"    let {arg_name}_owner = if {arg_name}.is_null() {{
        None
    }} else {{
        unsafe {{ std::slice::from_raw_parts({arg_name}, {len_expr}) }}
            .first()
            .copied()
            .filter(|raw| !raw.is_null())
            .map(|raw| unsafe {{ {obj}::from_raw(raw) }})
    }};
    let {arg_name} = {arg_name}_owner.as_ref();"#,
                        arg_name = arg_name,
                        len_expr = len_expr,
                        obj = obj
                    ));
                } else if arg.optional {
                    conversions.push(format!(
                        r#"    let {arg_name}_owner = if {arg_name}.is_null() {{
        None
    }} else {{
        Some(unsafe {{ {obj}::from_raw({arg_name}) }})
    }};
    let {arg_name} = {arg_name}_owner.as_ref();"#,
                        arg_name = arg_name,
                        obj = obj
                    ));
                } else {
                    conversions.push(format!(
                        r#"    let {arg_name}_owner = Some(unsafe {{ {obj}::from_raw({arg_name}) }});
    let {arg_name} = {arg_name}_owner.as_ref().expect("required callback object must not be null");"#,
                        arg_name = arg_name,
                        obj = obj
                    ));
                }
                call_args.push(call_arg);
                continue;
            }
            if let Some(length) = &arg.length {
                let len_expr = length_value_expr(length);
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
            if arg.optional {
                conversions.push(format!(
                    r#"    let mut {arg_name}_value = if {arg_name}.is_null() {{
        None
    }} else {{
        Some({rust_ty}::from_ffi_borrowed(unsafe {{ *{arg_name} }}))
    }};
    let {arg_name} = {arg_name}_value.{borrow_method}();"#,
                    arg_name = arg_name,
                    rust_ty = rust_ty,
                    borrow_method = if arg.annotation.is_mut_ptr() {
                        "as_mut"
                    } else {
                        "as_ref"
                    }
                ));
            } else {
                conversions.push(format!(
                    r#"    if {arg_name}.is_null() {{
        return Default::default();
    }}
    let mut {arg_name} = {rust_ty}::from_ffi_borrowed(unsafe {{ *{arg_name} }});"#,
                    arg_name = arg_name,
                    rust_ty = rust_ty
                ));
            }
            if arg.annotation.is_const_ptr() {
                if !arg.optional {
                    call_arg = format!(r#"&{arg_name}"#, arg_name = arg_name);
                }
            } else if arg.annotation.is_mut_ptr() && !arg.optional {
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
    let forget_borrowed_objects = borrowed_object_owner_names(&c.name, &c.def.args)
        .into_iter()
        .map(|arg_name| {
            format!(
                r#"    if let Some(value) = {arg_name}_owner {{
        std::mem::forget(value);
    }}"#,
                arg_name = arg_name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let callback_dispatch = if callback_is_repeating(&c.name) && return_ty == "()" {
        format!(
            r#"    let callback_slot = unsafe {{ &*userdata1.cast::<std::sync::Mutex<Option<{name}>>>() }};
    let mut callback = {{
        let mut slot = callback_slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        slot.take()
    }};
    if let Some(callback) = callback.as_mut() {{
        invoke_callback_safely(|| callback({call_args}));
    }}
    let mut slot = callback_slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = callback;"#,
            name = name,
            call_args = call_args_block
        )
    } else if callback_is_repeating(&c.name) {
        format!(
            r#"    let callback_slot = unsafe {{ &*userdata1.cast::<std::sync::Mutex<Option<{name}>>>() }};
    let mut callback = {{
        let mut slot = callback_slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        slot.take()
    }};
    let result = callback
        .as_mut()
        .map(|callback| invoke_callback_safely(|| callback({call_args})))
        .unwrap_or_default();
    let mut slot = callback_slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = callback;
    return result;"#,
            name = name,
            call_args = call_args_block
        )
    } else if return_ty == "()" {
        format!(
            r#"    let mut callback = unsafe {{ Box::from_raw(userdata1.cast::<Option<{name}>>()) }};
    if let Some(mut callback) = callback.take() {{
        invoke_callback_safely(|| callback({call_args}));
    }}"#,
            name = name,
            call_args = call_args_block
        )
    } else {
        format!(
            r#"    let mut callback = unsafe {{ Box::from_raw(userdata1.cast::<Option<{name}>>()) }};
    if let Some(mut callback) = callback.take() {{
        return invoke_callback_safely(|| callback({call_args}));
    }}"#,
            name = name,
            call_args = call_args_block
        )
    };

    format!(
        r#"pub type {name} = Box<dyn FnMut({signature}) -> {return_ty} + Send + 'static>;

pub(crate) unsafe extern "C" fn {trampoline}({ffi_args}, userdata1: *mut std::ffi::c_void, userdata2: *mut std::ffi::c_void){trampoline_return} {{
    let _ = userdata2;
{conversions}
{callback_dispatch}
{forget_borrowed_objects}
{default_return}
}}

"#,
        name = name,
        signature = signature,
        return_ty = return_ty,
        trampoline = trampoline,
        ffi_args = ffi_args,
        trampoline_return = trampoline_return,
        conversions = conversions_block,
        callback_dispatch = callback_dispatch,
        forget_borrowed_objects = forget_borrowed_objects,
        default_return = default_return
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
    callback_arg_list_for_callback("", args)
}

pub(crate) fn callback_arg_list_for_callback(callback_name: &str, args: &[RecordMember]) -> String {
    let mut parts = Vec::new();
    let length_fields = length_field_names(args);
    for arg in args {
        if length_fields.contains(&arg.name) {
            continue;
        }
        let arg_ty = callback_param_type(callback_name, arg);
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

    if arg.annotation.is_mut_ptr() {
        return format!("*mut {}", rust_type_for(&arg.member_type));
    }

    if arg.annotation.is_const_ptr() || arg.length.is_some() {
        return format!("*const {}", rust_type_for(&arg.member_type));
    }

    rust_type_for(&arg.member_type)
}

fn callback_param_type(callback_name: &str, arg: &RecordMember) -> String {
    let base = rust_type_for(&arg.member_type);

    if callback_object_is_borrowed(callback_name, arg) {
        return format!(r#"Option<&{base}>"#, base = base);
    }

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

fn callback_object_is_borrowed(callback_name: &str, arg: &RecordMember) -> bool {
    matches!(
        callback_name,
        "device lost" | "device lost callback" | "uncaptured error" | "uncaptured error callback"
    ) && arg.member_type == "device"
}

fn callback_is_repeating(callback_name: &str) -> bool {
    matches!(
        callback_name,
        "device lost"
            | "device lost callback"
            | "uncaptured error"
            | "uncaptured error callback"
            | "dawn load cache data"
            | "dawn load cache data callback"
            | "dawn store cache data"
            | "dawn store cache data callback"
    )
}

fn borrowed_object_owner_names(callback_name: &str, args: &[RecordMember]) -> Vec<String> {
    args.iter()
        .filter(|arg| callback_object_is_borrowed(callback_name, arg))
        .map(|arg| safe_ident(&snake_case_name(&arg.name)))
        .collect()
}
