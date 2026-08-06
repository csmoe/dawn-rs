use crate::api_model::StructureModel;
use crate::emitter::core::*;
use crate::parser::{LengthValue, RecordMember};
use std::collections::{HashMap, HashSet};

pub(crate) fn emit_struct(
    s: &StructureModel,
    index: &TypeIndex,
    c_prefix: &str,
    callback_info_map: &HashMap<String, String>,
    callback_info_mode_map: &HashMap<String, bool>,
    constant_map: &HashMap<String, String>,
) -> String {
    let name = type_name(&s.name);
    let ffi_name = ffi_type_name(&s.name, c_prefix);
    let docs = doc_comment(s.def.comment.as_deref());

    let mut fields = Vec::new();
    let mut default_fields = Vec::new();
    let mut extra_methods = Vec::new();
    let length_fields = length_field_names(&s.def.members);
    let needs_free_members = index.struct_needs_free_members(&s.name);
    let required_members = s
        .def
        .members
        .iter()
        .filter(|member| member_is_required(s, member, index, &length_fields))
        .collect::<Vec<_>>();
    let required_checks = required_members
        .iter()
        .map(|member| {
            let field = safe_ident(&snake_case_name(&member.name));
            format!(
                "assert!(self.{field}.is_some(), \"required field {name}.{field} is missing\");",
                name = type_name(&s.name)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    if s.def.extensible.is_extensible() {
        let ext_enum = format!("{}Extension", type_name(&s.name));
        fields.push(format!(
            r#"pub(crate) extensions: Vec<{ext_enum}>,"#,
            ext_enum = ext_enum
        ));
        default_fields.push("extensions: Vec::new(),".to_string());
        let mut to_ffi_body = emit_struct_to_ffi_body(
            s,
            index,
            c_prefix,
            true,
            callback_info_map,
            callback_info_mode_map,
        );
        if !required_checks.is_empty() {
            to_ffi_body = format!("{required_checks}\n{to_ffi_body}");
        }
        extra_methods.push(format!(
            r#"pub(crate) fn to_ffi(&self) -> (ffi::{ffi_name}, ChainedStructStorage) {{
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::{c_prefix}ChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {{
            next = ext.push_chain(&mut storage, next);
        }}
{to_ffi_body}
    }}"#,
        ));
        extra_methods.push(format!(
            r#"pub fn with_extension(mut self, extension: {ext_enum}) -> Self {{
            self.extensions.push(extension);
            self
        }}"#
        ));
    } else {
        let mut to_ffi_body = emit_struct_to_ffi_body(
            s,
            index,
            c_prefix,
            false,
            callback_info_map,
            callback_info_mode_map,
        );
        if !required_checks.is_empty() {
            to_ffi_body = format!("{required_checks}\n{to_ffi_body}");
        }
        extra_methods.push(
            format!(
                r#"    pub(crate) fn to_ffi(&self) -> (ffi::{ffi_name}, ChainedStructStorage) {{
        let mut storage = ChainedStructStorage::new();
{body}
    }}"#,
                ffi_name = ffi_name,
                body = indent_block(&to_ffi_body, 8)
            )
            .to_string(),
        );
    }

    for member in &s.def.members {
        if length_fields.contains(&member.name) {
            continue;
        }
        let field_name = safe_ident(&snake_case_name(&member.name));
        let field_ty = struct_field_type(member, index);
        let param_ty = builder_param_type(member, index);
        fields.push(format!(
            r#"pub {field_name}: {field_ty},"#,
            field_name = field_name,
            field_ty = field_ty
        ));
        let default_value = member_default_expr(member, index, c_prefix, constant_map)
            .map(|expr| format!("Some({expr})", expr = expr))
            .unwrap_or_else(|| "None".to_string());
        default_fields.push(format!(
            r#"        {field_name}: {default_value},"#,
            field_name = field_name,
            default_value = default_value
        ));

        let _ = param_ty;
    }

    let fields_block = fields.join("\n");
    let default_fields_block = default_fields.join("\n");
    let mut from_ffi_body = emit_struct_from_ffi_body(s, index, true);
    if from_ffi_body.is_empty() {
        from_ffi_body = "let _ = value;\n        Self::default()".to_string();
    } else if needs_free_members {
        let free_fn = free_members_fn_name(&s.name);
        from_ffi_body =
            format!("let result = {from_ffi_body};\nunsafe {{ ffi::{free_fn}(value) }};\nresult");
    }
    let from_ffi = format!(
        r#"    pub(crate) fn from_ffi(value: ffi::{ffi_name}) -> Self {{
{from_ffi_body}
    }}"#,
        ffi_name = ffi_name,
        from_ffi_body = indent_block(&from_ffi_body, 8)
    );
    let mut from_ffi_borrowed_body = emit_struct_from_ffi_body(s, index, false);
    if from_ffi_borrowed_body.is_empty() {
        from_ffi_borrowed_body = "let _ = value;\n        Self::default()".to_string();
    }
    let from_ffi_borrowed = format!(
        r#"    pub(crate) fn from_ffi_borrowed(value: ffi::{ffi_name}) -> Self {{
{from_ffi_borrowed_body}
    }}"#,
        ffi_name = ffi_name,
        from_ffi_borrowed_body = indent_block(&from_ffi_borrowed_body, 8)
    );

    let mut extra_blocks = extra_methods.clone();
    if !from_ffi.is_empty() {
        extra_blocks.push(from_ffi);
    }
    extra_blocks.push(from_ffi_borrowed);
    let extra_methods_block = extra_blocks.join("\n\n");

    let default_impl = if required_members.is_empty() {
        format!(
            r#"impl Default for {name} {{
    fn default() -> Self {{
        Self {{
{defaults}
        }}
    }}
}}

"#,
            name = name,
            defaults = default_fields_block,
        )
    } else {
        String::new()
    };
    let constructor_signature = required_members
        .iter()
        .map(|member| {
            let field = safe_ident(&snake_case_name(&member.name));
            let ty = builder_param_type(member, index);
            format!("{field}: {ty}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    let constructor_body = if required_members.is_empty() {
        "Self::default()".to_string()
    } else {
        let required_names = required_members
            .iter()
            .map(|member| safe_ident(&snake_case_name(&member.name)))
            .collect::<HashSet<_>>();
        let mut constructor_fields = Vec::new();
        if s.def.extensible.is_extensible() {
            constructor_fields.push("extensions: Vec::new(),".to_string());
        }
        for member in &s.def.members {
            if length_fields.contains(&member.name) {
                continue;
            }
            let field = safe_ident(&snake_case_name(&member.name));
            if required_names.contains(&field) {
                constructor_fields.push(format!("{field}: Some({field}),"));
            } else {
                let value = member_default_expr(member, index, c_prefix, constant_map)
                    .map(|expr| format!("Some({expr})"))
                    .unwrap_or_else(|| "None".to_string());
                constructor_fields.push(format!("{field}: {value},"));
            }
        }
        format!(
            "Self {{\n{}\n        }}",
            indent_block(&constructor_fields.join("\n"), 12)
        )
    };

    let struct_block = format!(
        r#"{docs}pub struct {name} {{
{fields}
}}

{default_impl}impl {name} {{
    pub fn new({constructor_signature}) -> Self {{
        {constructor_body}
    }}
{extra_methods}
}}

"#,
        name = name,
        docs = docs,
        fields = fields_block,
        default_impl = default_impl,
        constructor_signature = constructor_signature,
        constructor_body = constructor_body,
        extra_methods = extra_methods_block
    );
    struct_block
}

fn member_is_required(
    structure: &StructureModel,
    member: &RecordMember,
    index: &TypeIndex,
    length_fields: &HashSet<String>,
) -> bool {
    if structure.def.is_output()
        || length_fields.contains(&member.name)
        || member.optional
        || member.default.is_some()
        || index.is_callback_info(&member.member_type)
        || (index.struct_defaultable(&member.member_type) && member.annotation.is_value())
    {
        return false;
    }

    !member.length.as_ref().is_some_and(|length| match length {
        LengthValue::String(name) => structure
            .def
            .members
            .iter()
            .find(|candidate| candidate.name == *name)
            .is_some_and(|count| count.default.is_some()),
        LengthValue::Number(_) => false,
    })
}

pub(crate) fn emit_struct_from_ffi_body(
    s: &StructureModel,
    index: &TypeIndex,
    take_ownership: bool,
) -> String {
    let mut fields = Vec::new();
    let length_fields = length_field_names(&s.def.members);

    if s.def.extensible.is_extensible() {
        let extension_ty = format!("{}Extension", type_name(&s.name));
        let from_chain = if take_ownership {
            "from_chain"
        } else {
            "from_chain_borrowed"
        };
        fields.push(format!(
            "extensions: unsafe {{ {extension_ty}::{from_chain}(value.nextInChain) }},"
        ));
    }

    let raw_objects_are_borrowed = !take_ownership || index.struct_needs_free_members(&s.name);

    for member in &s.def.members {
        if length_fields.contains(&member.name) {
            continue;
        }
        let field_name = safe_ident(&snake_case_name(&member.name));
        let ffi_field = ffi_field_name(&member.name);
        let value_expr = if let Some(length) = &member.length {
            let len_expr = ffi_length_expr(length);
            if is_char_string_list(member) {
                format!(
                    r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| {{
                        if raw.is_null() {{
                            String::new()
                        }} else {{
                            unsafe {{ CStr::from_ptr(*raw) }}.to_string_lossy().into_owned()
                        }}
                    }})
                    .collect(),
            )
        }}"#,
                    ffi_field = ffi_field,
                    len_expr = len_expr
                )
            } else if index.is_object(&member.member_type) {
                let obj = type_name(&member.member_type);
                if member.array_element_optional == Some(true) && raw_objects_are_borrowed {
                    format!(
                        r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| {{
                        if raw.is_null() {{
                            None
                        }} else {{
                            unsafe {{ ffi::wgpu{obj}AddRef(*raw) }};
                            Some(unsafe {{ {obj}::from_raw(*raw) }})
                        }}
                    }})
                    .collect(),
            )
        }}"#,
                        ffi_field = ffi_field,
                        len_expr = len_expr,
                        obj = obj
                    )
                } else if member.array_element_optional == Some(true) {
                    format!(
                        r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| if raw.is_null() {{ None }} else {{ Some(unsafe {{ {obj}::from_raw(*raw) }}) }})
                    .collect(),
            )
        }}"#,
                        ffi_field = ffi_field,
                        len_expr = len_expr,
                        obj = obj
                    )
                } else if raw_objects_are_borrowed {
                    format!(
                        r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| {{
                        unsafe {{ ffi::wgpu{obj}AddRef(*raw) }};
                        unsafe {{ {obj}::from_raw(*raw) }}
                    }})
                    .collect(),
            )
        }}"#,
                        ffi_field = ffi_field,
                        len_expr = len_expr,
                        obj = obj
                    )
                } else {
                    format!(
                        r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| unsafe {{ {obj}::from_raw(*raw) }})
                    .collect(),
            )
        }}"#,
                        ffi_field = ffi_field,
                        len_expr = len_expr,
                        obj = obj
                    )
                }
            } else if index.is_enum(&member.member_type) || index.is_bitmask(&member.member_type) {
                let rust_ty = type_name(&member.member_type);
                format!(
                    r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| {rust_ty}::from(*raw))
                    .collect(),
            )
        }}"#,
                    ffi_field = ffi_field,
                    len_expr = len_expr,
                    rust_ty = rust_ty
                )
            } else if index.struct_extensible(&member.member_type).is_some() {
                let rust_ty = type_name(&member.member_type);
                let from_ffi_fn = if take_ownership && !index.struct_needs_free_members(&s.name) {
                    "from_ffi"
                } else {
                    "from_ffi_borrowed"
                };
                format!(
                    r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| {rust_ty}::{from_ffi_fn}(*raw))
                    .collect(),
            )
        }}"#,
                    ffi_field = ffi_field,
                    len_expr = len_expr,
                    rust_ty = rust_ty,
                    from_ffi_fn = from_ffi_fn
                )
            } else {
                format!(
                    r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}.to_vec())
        }}"#,
                    ffi_field = ffi_field,
                    len_expr = len_expr
                )
            }
        } else if member.member_type == "string view" {
            if member.optional {
                format!(
                    r#"if value.{ffi_field}.data.is_null() || value.{ffi_field}.length == 0 {{
            None
        }} else {{
            Some(string_view_to_string(value.{ffi_field}))
        }}"#,
                    ffi_field = ffi_field
                )
            } else {
                format!(
                    "Some(string_view_to_string(value.{ffi_field}))",
                    ffi_field = ffi_field
                )
            }
        } else if index.is_enum(&member.member_type) || index.is_bitmask(&member.member_type) {
            format!("Some(value.{ffi_field}.into())", ffi_field = ffi_field)
        } else if index.is_callback_info(&member.member_type) {
            "None".to_string()
        } else if index.is_object(&member.member_type) {
            let obj = type_name(&member.member_type);
            if raw_objects_are_borrowed && member.optional {
                format!(
                    r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            unsafe {{ ffi::wgpu{obj}AddRef(value.{ffi_field}) }};
            Some(unsafe {{ {obj}::from_raw(value.{ffi_field}) }})
        }}"#,
                    ffi_field = ffi_field,
                    obj = obj
                )
            } else if raw_objects_are_borrowed {
                format!(
                    r#"{{
            unsafe {{ ffi::wgpu{obj}AddRef(value.{ffi_field}) }};
            Some(unsafe {{ {obj}::from_raw(value.{ffi_field}) }})
        }}"#,
                    ffi_field = ffi_field,
                    obj = obj
                )
            } else if member.optional {
                format!(
                    r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(unsafe {{ {obj}::from_raw(value.{ffi_field}) }})
        }}"#,
                    ffi_field = ffi_field,
                    obj = obj
                )
            } else {
                format!(
                    r#"Some(unsafe {{ {obj}::from_raw(value.{ffi_field}) }})"#,
                    ffi_field = ffi_field,
                    obj = obj
                )
            }
        } else if index.struct_extensible(&member.member_type).is_some()
            && member.annotation.is_value()
        {
            let rust_ty = type_name(&member.member_type);
            let from_ffi_fn = if take_ownership && !index.struct_needs_free_members(&s.name) {
                "from_ffi"
            } else {
                "from_ffi_borrowed"
            };
            format!(
                r#"Some({rust_ty}::{from_ffi_fn}(value.{ffi_field}))"#,
                rust_ty = rust_ty,
                ffi_field = ffi_field,
                from_ffi_fn = from_ffi_fn
            )
        } else if index.struct_extensible(&member.member_type).is_some()
            && (member.annotation.is_const_ptr() || member.annotation.is_mut_ptr())
        {
            let rust_ty = type_name(&member.member_type);
            let from_ffi_fn = if take_ownership && !index.struct_needs_free_members(&s.name) {
                "from_ffi"
            } else {
                "from_ffi_borrowed"
            };
            format!(
                r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some({rust_ty}::{from_ffi_fn}(unsafe {{ *value.{ffi_field} }}))
        }}"#,
                ffi_field = ffi_field,
                rust_ty = rust_ty,
                from_ffi_fn = from_ffi_fn
            )
        } else if member.member_type == "bool" {
            format!("Some(value.{ffi_field} != 0)", ffi_field = ffi_field)
        } else {
            format!("Some(value.{ffi_field})", ffi_field = ffi_field)
        };

        fields.push(format!(
            r#"    {field_name}: {value_expr},"#,
            field_name = field_name,
            value_expr = value_expr
        ));
    }

    if fields.is_empty() {
        return String::new();
    }

    let fields_block = fields.join("\n");
    format!(
        r#"Self {{
{fields}
        }}"#,
        fields = fields_block
    )
}

pub(crate) fn emit_struct_to_ffi_body(
    s: &StructureModel,
    index: &TypeIndex,
    c_prefix: &str,
    is_extensible: bool,
    callback_info_map: &HashMap<String, String>,
    callback_info_mode_map: &HashMap<String, bool>,
) -> String {
    let mut lines = Vec::new();
    let mut length_map: HashMap<String, String> = HashMap::new();
    let mut length_types: HashMap<String, String> = HashMap::new();
    for member in &s.def.members {
        length_types.insert(member.name.clone(), rust_type_for(&member.member_type));
        if let Some(LengthValue::String(name)) = &member.length {
            length_map.insert(name.clone(), member.name.clone());
        }
    }

    lines.push(format!(
        r#"let mut raw: ffi::{ffi_ty} = unsafe {{ std::mem::zeroed() }};"#,
        ffi_ty = ffi_type_name(&s.name, c_prefix)
    ));
    if is_extensible {
        lines.push("raw.nextInChain = next;".to_string());
    }

    for member in &s.def.members {
        let field_name = safe_ident(&snake_case_name(&member.name));
        let ffi_field = ffi_field_name(&member.name);
        if length_map.contains_key(&member.name) {
            let data_name = length_map.get(&member.name).cloned().unwrap();
            let data_ident = safe_ident(&snake_case_name(&data_name));
            let len_expr = format!(
                r#"self.{data}.as_ref().map(|v| v.len()).unwrap_or(0)"#,
                data = data_ident
            );
            let target_ty = rust_type_for(&member.member_type);
            let len_expr = if target_ty == "usize" {
                len_expr
            } else {
                format!("({len}) as {ty}", len = len_expr, ty = target_ty)
            };
            lines.push(format!(
                r#"raw.{ffi_field} = {len_expr};"#,
                ffi_field = ffi_field,
                len_expr = len_expr
            ));
            continue;
        }

        if member.length.is_some() {
            let len_field = if let Some(LengthValue::String(name)) = &member.length {
                Some((name.clone(), ffi_field_name(name)))
            } else {
                None
            };
            let len_assign = len_field.as_ref().map(|(orig, field)| {
                let len_ty = length_types
                    .get(orig)
                    .cloned()
                    .unwrap_or_else(|| "usize".to_string());
                let len_expr = if len_ty == "usize" {
                    "len_value".to_string()
                } else {
                    format!("len_value as {len_ty}", len_ty = len_ty)
                };
                format!(
                    r#"raw.{field} = {len_expr};"#,
                    field = field,
                    len_expr = len_expr
                )
            });

            let mut body = Vec::new();
            body.push("let len_value = values.len();".to_string());

            if is_char_string_list(member) {
                body.push(
                    "let mut c_strings: Vec<std::ffi::CString> = Vec::with_capacity(values.len());"
                        .to_string(),
                );
                body.push("let mut ptrs: Vec<*const std::os::raw::c_char> = Vec::with_capacity(values.len());".to_string());
                body.push("for item in values.iter() {".to_string());
                body.push("    let c_string = std::ffi::CString::new(item.as_str()).unwrap_or_else(|_| std::ffi::CString::new(\"\").unwrap());".to_string());
                body.push("    ptrs.push(c_string.as_ptr());".to_string());
                body.push("    c_strings.push(c_string);".to_string());
                body.push("}".to_string());
                body.push("let ptr = storage.push_vec(ptrs);".to_string());
                body.push("storage.push_any(c_strings);".to_string());
                body.push(format!(r#"raw.{ffi_field} = ptr;"#, ffi_field = ffi_field));
                if let Some(assign) = len_assign {
                    body.push(assign);
                }
            } else if index.is_object(&member.member_type) {
                let mapping = if member.array_element_optional == Some(true) {
                    "v.as_ref().map(|v| v.as_raw()).unwrap_or(std::ptr::null_mut())"
                } else {
                    "v.as_raw()"
                };
                body.push(format!(
                    r#"let raw_vec: Vec<ffi::{prefix}{obj}> = values.iter().map(|v| {mapping}).collect();"#,
                    prefix = c_prefix,
                    obj = type_name(&member.member_type),
                    mapping = mapping,
                ));
                body.push("let ptr = storage.push_vec(raw_vec);".to_string());
                body.push(format!(r#"raw.{ffi_field} = ptr;"#, ffi_field = ffi_field));
                if let Some(assign) = len_assign {
                    body.push(assign);
                }
            } else if index.is_enum(&member.member_type) || index.is_bitmask(&member.member_type) {
                let ffi_ty = ffi_type_name(&member.member_type, c_prefix);
                body.push(format!(
                    r#"let raw_vec: Vec<ffi::{ffi_ty}> = values.iter().map(|v| (*v).into()).collect();"#,
                    ffi_ty = ffi_ty
                ));
                body.push("let ptr = storage.push_vec(raw_vec);".to_string());
                body.push(format!(r#"raw.{ffi_field} = ptr;"#, ffi_field = ffi_field));
                if let Some(assign) = len_assign {
                    body.push(assign);
                }
            } else if index.struct_extensible(&member.member_type).is_some() {
                let ffi_ty = ffi_type_name(&member.member_type, c_prefix);
                body.push(format!(
                    r#"let mut raw_vec: Vec<ffi::{ffi_ty}> = Vec::with_capacity(values.len());"#,
                    ffi_ty = ffi_ty
                ));
                body.push("for item in values.iter() {".to_string());
                body.push("    let (raw_item, storage_item) = item.to_ffi();".to_string());
                body.push("    raw_vec.push(raw_item);".to_string());
                body.push("    storage.push_storage(storage_item);".to_string());
                body.push("}".to_string());
                body.push("let ptr = storage.push_vec(raw_vec);".to_string());
                body.push(format!(r#"raw.{ffi_field} = ptr;"#, ffi_field = ffi_field));
                if let Some(assign) = len_assign {
                    body.push(assign);
                }
            } else if member.member_type == "string view" {
                body.push(format!(
                    r#"let raw_vec: Vec<ffi::{prefix}StringView> = values.iter().map(|value| ffi::{prefix}StringView {{ data: value.as_ptr().cast(), length: value.len() }}).collect();"#,
                    prefix = c_prefix
                ));
                body.push("let ptr = storage.push_vec(raw_vec);".to_string());
                body.push(format!(r#"raw.{ffi_field} = ptr;"#, ffi_field = ffi_field));
                if let Some(assign) = len_assign {
                    body.push(assign);
                }
            } else {
                body.push("let raw_vec = values.to_vec();".to_string());
                body.push("let ptr = storage.push_vec(raw_vec);".to_string());
                body.push(format!(r#"raw.{ffi_field} = ptr;"#, ffi_field = ffi_field));
                if let Some(assign) = len_assign {
                    body.push(assign);
                }
            }

            lines.push(format!(
                r#"if let Some(values) = &self.{field} {{
{body}
}} else {{
    raw.{ffi_field} = std::ptr::null();
    {len_zero}
}}"#,
                field = field_name,
                ffi_field = ffi_field,
                body = indent_block(&body.join("\n"), 4),
                len_zero = if let Some((_, field)) = len_field.as_ref() {
                    format!("raw.{field} = 0;", field = field)
                } else {
                    "let _ = 0;".to_string()
                }
            ));
            continue;
        }

        if member.member_type == "string view" {
            lines.push(format!(
                r#"if let Some(value) = &self.{field} {{
    raw.{ffi_field} = ffi::{prefix}StringView {{ data: value.as_ptr().cast(), length: value.len() }};
}} else {{
    raw.{ffi_field} = ffi::{prefix}StringView {{ data: std::ptr::null(), length: 0 }};
}}"#,
                field = field_name,
                ffi_field = ffi_field,
                prefix = c_prefix
            ));
            continue;
        }

        if member.member_type == "bool" {
            lines.push(format!(
                r#"raw.{ffi_field} = if self.{field}.unwrap_or(false) {{ 1 }} else {{ 0 }};"#,
                ffi_field = ffi_field,
                field = field_name
            ));
            continue;
        }

        if index.is_enum(&member.member_type) || index.is_bitmask(&member.member_type) {
            let ffi_ty = ffi_type_name(&member.member_type, c_prefix);
            lines.push(format!(
                r#"if let Some(value) = self.{field} {{
    raw.{ffi_field} = value.into();
}} else {{
    raw.{ffi_field} = 0 as ffi::{ffi_ty};
}}"#,
                field = field_name,
                ffi_field = ffi_field,
                ffi_ty = ffi_ty
            ));
            continue;
        }

        if index.is_object(&member.member_type) {
            lines.push(format!(
                r#"raw.{ffi_field} = self.{field}.as_ref().map(|v| v.as_raw()).unwrap_or(std::ptr::null_mut());"#,
                ffi_field = ffi_field,
                field = field_name
            ));
            continue;
        }

        if index.is_callback_info(&member.member_type) {
            if let Some(callback_fn_name) = callback_info_map.get(&member.member_type) {
                let callback_ty = callback_type_name(callback_fn_name);
                let ffi_callback_ty = ffi_type_name(callback_fn_name, c_prefix);
                let trampoline = callback_trampoline_name(callback_fn_name);
                let info_name = type_name(&member.member_type);
                let has_mode = callback_info_mode_map
                    .get(&member.member_type)
                    .copied()
                    .unwrap_or(false);
                let mode_line = if has_mode {
                    "    let mode = info.mode.unwrap_or(CallbackMode::AllowProcessEvents);\n"
                } else {
                    ""
                };
                let mode_field = if has_mode {
                    "        mode: mode.into(),\n"
                } else {
                    ""
                };
                let default_mode_field = if has_mode {
                    "        mode: CallbackMode::AllowProcessEvents.into(),\n"
                } else {
                    ""
                };
                let callback_userdata_storage = if callback_info_is_persistent(callback_fn_name) {
                    format!(
                        "        storage.push_callback_userdata(CallbackUserdata::new_mutex::<{callback_ty}>(userdata));\n",
                        callback_ty = callback_ty
                    )
                } else {
                    String::new()
                };
                let callback_box_wrapper = if callback_info_is_persistent(callback_fn_name) {
                    "        let callback_box = Box::new(std::sync::Mutex::new(Some(callback_box)));\n"
                } else {
                    "        let callback_box = Box::new(Some(callback_box));\n"
                };

                lines.push(format!(
                    r#"if let Some(info) = &self.{field} {{
    let mut callback_slot = info.callback.borrow_mut();
    let callback = callback_slot.take();
    let (callback_ptr, userdata1): (ffi::{ffi_callback_ty}, *mut std::ffi::c_void) = if let Some(callback) = callback {{
        let callback_box: {callback_ty} = callback;
{callback_box_wrapper}
        let userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
{callback_userdata_storage}        (Some({trampoline}), userdata)
    }} else {{
        (None, std::ptr::null_mut())
    }};
{mode_line}    raw.{ffi_field} = ffi::{prefix}{info_name} {{
        nextInChain: std::ptr::null_mut(),
{mode_field}        callback: callback_ptr,
        userdata1,
        userdata2: std::ptr::null_mut(),
    }};
}} else {{
    raw.{ffi_field} = ffi::{prefix}{info_name} {{
        nextInChain: std::ptr::null_mut(),
{default_mode_field}        callback: None,
        userdata1: std::ptr::null_mut(),
        userdata2: std::ptr::null_mut(),
    }};
}}"#,
                    field = field_name,
                    callback_ty = callback_ty,
                    ffi_callback_ty = ffi_callback_ty,
                    trampoline = trampoline,
                    callback_userdata_storage = callback_userdata_storage,
                    callback_box_wrapper = callback_box_wrapper,
                    mode_line = mode_line,
                    mode_field = mode_field,
                    default_mode_field = default_mode_field,
                    ffi_field = ffi_field,
                    info_name = info_name,
                    prefix = c_prefix
                ));
            } else {
                lines.push(format!(r#"let _ = &self.{field};"#, field = field_name));
            }
            continue;
        }

        if index.struct_extensible(&member.member_type).is_some() {
            if member.annotation.is_value() {
                lines.push(format!(
                    r#"if let Some(value) = &self.{field} {{
    let (raw_value, storage_value) = value.to_ffi();
    raw.{ffi_field} = raw_value;
    storage.push_storage(storage_value);
}}"#,
                    field = field_name,
                    ffi_field = ffi_field
                ));
                continue;
            }

            if member.annotation.is_const_ptr() {
                lines.push(format!(
                    r#"if let Some(value) = &self.{field} {{
    let (raw_value, storage_value) = value.to_ffi();
    let ptr = storage.push_value(raw_value);
    raw.{ffi_field} = ptr;
    storage.push_storage(storage_value);
}} else {{
    raw.{ffi_field} = std::ptr::null();
}}"#,
                    field = field_name,
                    ffi_field = ffi_field
                ));
                continue;
            }

            if member.annotation.is_mut_ptr() {
                lines.push(format!(
                    r#"if let Some(value) = &self.{field} {{
    let (raw_value, storage_value) = value.to_ffi();
    let ptr = storage.push_value_mut(raw_value);
    raw.{ffi_field} = ptr;
    storage.push_storage(storage_value);
}} else {{
    raw.{ffi_field} = std::ptr::null_mut();
}}"#,
                    field = field_name,
                    ffi_field = ffi_field
                ));
                continue;
            }
        }

        lines.push(format!(
            r#"if let Some(value) = self.{field} {{
    raw.{ffi_field} = value;
}}"#,
            field = field_name,
            ffi_field = ffi_field
        ));
    }

    lines.push("(raw, storage)".to_string());

    lines.join("\n")
}

pub(crate) fn struct_field_type(member: &RecordMember, index: &TypeIndex) -> String {
    if is_char_string_list(member) {
        return "Option<Vec<String>>".to_string();
    }

    let base = rust_type_for(&member.member_type);
    let is_struct = index.struct_extensible(&member.member_type).is_some()
        || index.is_callback_info(&member.member_type);

    let ty = if member.length.is_some() {
        if member.array_element_optional == Some(true) && index.is_object(&member.member_type) {
            format!(r#"Vec<Option<{base}>>"#, base = base)
        } else {
            format!(r#"Vec<{base}>"#, base = base)
        }
    } else if member.member_type.contains('*') {
        base
    } else if member.annotation.is_const_ptr() && !is_struct {
        format!(r#"*const {base}"#, base = base)
    } else if member.annotation.is_mut_ptr() && !is_struct {
        format!(r#"*mut {base}"#, base = base)
    } else {
        base
    };

    format!(r#"Option<{ty}>"#, ty = ty)
}

pub(crate) fn builder_param_type(member: &RecordMember, index: &TypeIndex) -> String {
    if is_char_string_list(member) {
        return "Vec<String>".to_string();
    }

    let base = rust_type_for(&member.member_type);
    let is_struct = index.struct_extensible(&member.member_type).is_some()
        || index.is_callback_info(&member.member_type);

    if member.length.is_some() {
        if member.array_element_optional == Some(true) && index.is_object(&member.member_type) {
            format!(r#"Vec<Option<{base}>>"#, base = base)
        } else {
            format!(r#"Vec<{base}>"#, base = base)
        }
    } else if member.member_type.contains('*') {
        base
    } else if member.annotation.is_const_ptr() && !is_struct {
        format!(r#"*const {base}"#, base = base)
    } else if member.annotation.is_mut_ptr() && !is_struct {
        format!(r#"*mut {base}"#, base = base)
    } else {
        base
    }
}

fn member_default_expr(
    member: &RecordMember,
    index: &TypeIndex,
    c_prefix: &str,
    constant_map: &HashMap<String, String>,
) -> Option<String> {
    if member.no_default == Some(true) {
        return None;
    }
    let default_value = member.default.as_ref()?;
    let ty = &member.member_type;

    let numeric_expr = |value: u64| -> String {
        if ty == "bool" {
            if value == 0 {
                "false".to_string()
            } else {
                "true".to_string()
            }
        } else if index.is_enum(ty) {
            let ffi_ty = ffi_type_name(ty, c_prefix);
            format!(
                "{enum_ty}::from({value} as ffi::{ffi_ty})",
                enum_ty = type_name(ty)
            )
        } else if index.is_bitmask(ty) {
            format!(
                "{mask_ty}::from_bits_retain({value} as u64)",
                mask_ty = type_name(ty)
            )
        } else if ty == "float" || ty == "double" {
            format!("{value}.0")
        } else {
            value.to_string()
        }
    };

    match default_value {
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(num) => num.as_u64().map(numeric_expr),
        Value::String(s) => {
            let lowered = s.trim().to_ascii_lowercase();
            if lowered == "nullptr" || lowered == "null" {
                return None;
            }
            if let Some(num) = parse_numeric_string_u64(s) {
                Some(numeric_expr(num))
            } else if index.is_enum(ty) {
                Some(format!(
                    r#"{}::{}"#,
                    type_name(ty),
                    enum_variant_name_camel(s)
                ))
            } else if index.is_bitmask(ty) {
                Some(format!(r#"{}::{}"#, type_name(ty), bitmask_variant_name(s)))
            } else {
                constant_map.get(s).cloned()
            }
        }
        _ => None,
    }
}

pub(crate) fn length_field_names(members: &[RecordMember]) -> HashSet<String> {
    let mut fields = HashSet::new();
    for member in members {
        if let Some(LengthValue::String(name)) = &member.length {
            fields.insert(name.clone());
        }
    }
    fields
}

pub(crate) fn length_expr_for_data_arg(arg: &RecordMember, data_name: &str) -> String {
    if arg.optional {
        format!(
            r#"{data}.as_deref().map(|v| v.len()).unwrap_or(0)"#,
            data = data_name
        )
    } else {
        format!(r#"{data}.len()"#, data = data_name)
    }
}

pub(crate) fn length_value_expr(length: &LengthValue) -> String {
    match length {
        LengthValue::Number(value) => value.to_string(),
        LengthValue::String(name) => safe_ident(&snake_case_name(name)),
    }
}

pub(crate) fn is_char_string_list(member: &RecordMember) -> bool {
    member.member_type == "char"
        && member.length.is_some()
        && member.annotation.is_const_const_ptr()
}

fn callback_info_is_persistent(callback_name: &str) -> bool {
    matches!(
        callback_name,
        "device lost callback"
            | "uncaptured error callback"
            | "dawn load cache data callback"
            | "dawn store cache data callback"
    )
}
