use crate::api_model::{
    ApiModel, BitmaskModel, CallbackFunctionModel, CallbackInfoModel, CallbackModel, ConstantModel,
    EnumModel, FunctionModel, ObjectModel, StructureModel,
};
use crate::parser::{EnumValueDef, LengthValue, RecordMember, ReturnType};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn generate_to_dir(model: &ApiModel, out_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(out_dir)?;

    let enums = emit_enums(model);
    let structs = emit_structs(model);
    let extensions = emit_extensions(model);
    let objects = emit_objects(model);
    let callbacks = emit_callbacks(model);
    let functions = emit_functions(model);
    let constants = emit_constants(model);

    fs::write(out_dir.join("enums.rs"), enums)?;
    fs::write(out_dir.join("structs.rs"), structs)?;
    fs::write(out_dir.join("extensions.rs"), extensions)?;
    fs::write(out_dir.join("objects.rs"), objects)?;
    fs::write(out_dir.join("callbacks.rs"), callbacks)?;
    fs::write(out_dir.join("functions.rs"), functions)?;
    fs::write(out_dir.join("constants.rs"), constants)?;

    let mod_rs = emit_mod_rs();
    fs::write(out_dir.join("mod.rs"), mod_rs)?;

    Ok(())
}

fn emit_mod_rs() -> String {
    let content = r#"#![allow(dead_code, unused_imports)]

pub mod enums;
pub mod structs;
pub mod extensions;
pub mod objects;
pub mod callbacks;
pub mod functions;
pub mod constants;

pub use enums::*;
pub use structs::*;
pub use extensions::*;
pub use objects::*;
pub use callbacks::*;
pub use functions::*;
pub use constants::*;
"#;

    content.to_string()
}

fn emit_enums(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use bitflags::bitflags;

"#,
    );

    for e in &model.enums {
        out.push_str(&emit_enum(e));
    }
    for b in &model.bitmasks {
        out.push_str(&emit_bitmask(b));
    }

    out
}

fn emit_structs(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;
use std::ffi::CStr;

fn string_view_to_string(view: ffi::WGPUStringView) -> String {
    if view.data.is_null() || view.length == 0 {
        return String::new();
    }
    let data = view.data.cast::<u8>();
    let slice = unsafe { std::slice::from_raw_parts(data, view.length) };
    String::from_utf8_lossy(slice).into_owned()
}

"#,
    );

    let index = TypeIndex::new(model);
    for s in &model.structures {
        out.push_str(&emit_struct(s, &index));
    }

    out
}

fn emit_extensions(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;

pub struct ChainedStructStorage {
    entries: Vec<Box<ffi::WGPUChainedStruct>>,
}

impl ChainedStructStorage {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn push(
        &mut self,
        s_type: ffi::WGPUSType,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let mut node = Box::new(ffi::WGPUChainedStruct { next, sType: s_type });
        let ptr = std::ptr::from_mut(node.as_mut());
        self.entries.push(node);
        ptr
    }
}

"#,
    );

    let mut roots: HashMap<String, Vec<&StructureModel>> = HashMap::new();
    let mut extensible_roots: Vec<String> = Vec::new();
    for s in &model.structures {
        if s.def.extensible.is_extensible() {
            extensible_roots.push(s.name.clone());
        }
        for root in &s.def.chain_roots {
            roots.entry(root.clone()).or_default().push(s);
        }
    }

    let mut root_names: Vec<String> = extensible_roots;
    root_names.sort();
    root_names.dedup();

    let stype_map = build_stype_map(model);

    for root in root_names {
        let variants = roots.get(&root).cloned().unwrap_or_default();
        let enum_name = format!("{}Extension", type_name(&root));

        let mut variant_lines = Vec::new();
        for s in variants.iter() {
            let ty = type_name(&s.name);
            variant_lines.push(format!(r#"    {ty}({ty}),"#, ty = ty));
        }
        let variant_block = variant_lines.join("\n");

        let has_variants = !variants.is_empty();
        let impl_block = if has_variants {
            format!(
                r#"impl {enum_name} {{
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {{
        match self {{
{push_arms}
        }}
    }}
}}

"#,
                enum_name = enum_name,
                push_arms = {
                    let mut arms = Vec::new();
                    for s in variants.iter() {
                        let ty = type_name(&s.name);
                        let stype_const = stype_map
                            .get(&s.name)
                            .cloned()
                            .unwrap_or_else(|| {
                                format!(
                                    r#"{}::{}"#,
                                    type_name("s type"),
                                    enum_variant_name_camel(&s.name)
                                )
                            });
                        arms.push(format!(
                            r#"            {enum_name}::{ty}(_) => storage.push({stype_const} as ffi::WGPUSType, next),"#,
                            enum_name = enum_name,
                            ty = ty,
                            stype_const = stype_const
                        ));
                    }
                    arms.join("\n")
                }
            )
        } else {
            format!(
                r#"impl {enum_name} {{
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {{
        let _ = self;
        let _ = storage;
        next
    }}
}}

"#,
                enum_name = enum_name
            )
        };

        out.push_str(&format!(
            r#"#[allow(dead_code)]
pub enum {enum_name} {{
{variants}
}}

{impl_block}
"#,
            enum_name = enum_name,
            variants = variant_block,
            impl_block = impl_block
        ));
    }

    out
}

fn emit_objects(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;

"#,
    );

    let constructors = build_constructor_map(model);
    let index = TypeIndex::new(model);
    for o in &model.objects {
        out.push_str(&emit_object(o, constructors.get(&o.name), model, &index));
    }

    out
}

fn emit_callbacks(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;

fn string_view_to_string(view: ffi::WGPUStringView) -> String {
    if view.data.is_null() || view.length == 0 {
        return String::new();
    }
    let data = view.data.cast::<u8>();
    let slice = unsafe { std::slice::from_raw_parts(data, view.length) };
    String::from_utf8_lossy(slice).into_owned()
}

"#,
    );

    let index = TypeIndex::new(model);

    for fp in &model.function_pointers {
        out.push_str(&emit_function_pointer(fp));
    }

    for c in &model.callback_functions {
        out.push_str(&emit_callback_function(c, &index));
    }

    for c in &model.callbacks {
        out.push_str(&emit_callback(c));
    }

    for c in &model.callback_infos {
        out.push_str(&emit_callback_info(c, &index));
    }

    out
}

fn emit_functions(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;

"#,
    );

    let index = TypeIndex::new(model);
    for f in &model.functions {
        out.push_str(&emit_function(f, model, &index));
    }

    out
}

fn emit_constants(model: &ApiModel) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use super::*;

"#,
    );

    for c in &model.constants {
        if let Some(block) = emit_constant(c) {
            out.push_str(&block);
        }
    }

    out
}

fn emit_enum(e: &EnumModel) -> String {
    let name = type_name(&e.name);
    let mut variants = Vec::new();
    let mut from_arms = Vec::new();
    let ffi_type = ffi_type_name(&e.name);

    for v in &e.def.values {
        let variant = enum_variant_name_camel(&v.name);
        let value = enum_value_u32(v);
        variants.push(format!(
            r#"    {variant} = {value},"#,
            variant = variant,
            value = value
        ));
        from_arms.push(format!(
            r#"            {value} => {name}::{variant},"#,
            value = value,
            name = name,
            variant = variant
        ));
    }

    let variants_block = variants.join("\n");
    let from_arms_block = from_arms.join("\n");
    let fallback_variant = variants
        .first()
        .map(|v| v.split_whitespace().nth(0).unwrap_or("Undefined"))
        .unwrap_or("Undefined");

    format!(
        r#"#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum {name} {{
{variants}
}}

impl From<ffi::{ffi_type}> for {name} {{
    fn from(value: ffi::{ffi_type}) -> Self {{
        match value as u32 {{
{from_arms}
            _ => {name}::{fallback},
        }}
    }}
}}

impl From<{name}> for ffi::{ffi_type} {{
    fn from(value: {name}) -> Self {{
        value as ffi::{ffi_type}
    }}
}}

"#,
        name = name,
        variants = variants_block,
        ffi_type = ffi_type,
        from_arms = from_arms_block,
        fallback = fallback_variant
    )
}

fn emit_bitmask(b: &BitmaskModel) -> String {
    let name = type_name(&b.name);
    let mut variants = Vec::new();
    let ffi_type = ffi_type_name(&b.name);

    for v in &b.def.values {
        let variant = bitmask_variant_name(&v.name);
        let value = enum_value_u64(v);
        variants.push(format!(
            r#"        const {variant} = {value};"#,
            variant = variant,
            value = value
        ));
    }

    let variants_block = variants.join("\n");

    format!(
        r#"bitflags! {{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct {name}: u64 {{
{variants}
    }}
}}

impl From<ffi::{ffi_type}> for {name} {{
    fn from(value: ffi::{ffi_type}) -> Self {{
        {name}::from_bits_truncate(value as u64)
    }}
}}

impl From<{name}> for ffi::{ffi_type} {{
    fn from(value: {name}) -> Self {{
        value.bits() as ffi::{ffi_type}
    }}
}}

"#,
        name = name,
        variants = variants_block,
        ffi_type = ffi_type
    )
}

fn enum_value_u32(value: &EnumValueDef) -> String {
    enum_value_u64(value).to_string()
}

fn enum_value_u64(value: &EnumValueDef) -> u64 {
    let base = match &value.value {
        Value::Number(num) => num.as_u64().unwrap_or(0),
        Value::String(s) => parse_numeric_string_u64(s).unwrap_or(0),
        _ => 0,
    };
    let offset = enum_tag_offset(&value.tags);
    base + offset as u64
}

fn enum_tag_offset(tags: &[String]) -> u32 {
    let has_compat = tags.iter().any(|t| t == "compat");
    let has_emscripten = tags.iter().any(|t| t == "emscripten");
    let has_dawn = tags.iter().any(|t| t == "dawn");
    if has_compat {
        return 0x0002_0000;
    }
    if has_emscripten {
        return 0x0004_0000;
    }
    if has_dawn {
        return 0x0005_0000;
    }
    if tags.iter().any(|t| t == "native") {
        return 0x0001_0000;
    }
    0
}

fn parse_numeric_string_u64(value: &str) -> Option<u64> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).ok()
    } else if let Some(hex) = trimmed.strip_prefix("0X") {
        u64::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u64>().ok()
    }
}

fn emit_struct(s: &StructureModel, index: &TypeIndex) -> String {
    let name = type_name(&s.name);
    let ffi_name = ffi_type_name(&s.name);

    let mut fields = Vec::new();
    let mut default_fields = Vec::new();
    let mut builder_methods = Vec::new();
    let mut extra_methods = Vec::new();

    if s.def.extensible.is_extensible() {
        let ext_enum = format!("{}Extension", type_name(&s.name));
        fields.push(format!(
            r#"    pub extensions: Vec<{ext_enum}>,"#,
            ext_enum = ext_enum
        ));
        default_fields.push("        extensions: Vec::new(),".to_string());
        extra_methods.push(format!(
            r#"    pub fn with_extension(mut self, extension: {ext_enum}) -> Self {{
        self.extensions.push(extension);
        self
    }}"#,
            ext_enum = ext_enum
        ));
        extra_methods.push(
            r#"    pub fn to_ffi(&self) -> (ffi::{ffi_name}, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::{ffi_name} = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }"#.replace("{ffi_name}", &ffi_name),
        );
    } else {
        extra_methods.push(
            r#"    pub fn to_ffi(&self) -> ffi::{ffi_name} {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }"#.replace("{ffi_name}", &ffi_name),
        );
    }

    for member in &s.def.members {
        let field_name = safe_ident(&snake_case_name(&member.name));
        let field_ty = struct_field_type(member, index);
        let param_ty = builder_param_type(member, index);
        fields.push(format!(
            r#"    pub {field_name}: {field_ty},"#,
            field_name = field_name,
            field_ty = field_ty
        ));
        let default_value = member_default_expr(member, index)
            .map(|expr| format!("Some({expr})", expr = expr))
            .unwrap_or_else(|| "None".to_string());
        default_fields.push(format!(
            r#"        {field_name}: {default_value},"#,
            field_name = field_name,
            default_value = default_value
        ));

        builder_methods.push(format!(
            r#"    pub fn {field_name}(mut self, value: {param_ty}) -> Self {{
        self.{field_name} = Some(value);
        self
    }}"#,
            field_name = field_name,
            param_ty = param_ty
        ));
    }

    let fields_block = fields.join("\n");
    let default_fields_block = default_fields.join("\n");
    let builder_methods_block = builder_methods.join("\n\n");

    let mut from_ffi_body = emit_struct_from_ffi_body(s, index);
    if from_ffi_body.is_empty() {
        from_ffi_body = "let _ = value;\n        Self::default()".to_string();
    }
    let from_ffi = format!(
        r#"    pub(crate) fn from_ffi(value: ffi::{ffi_name}) -> Self {{
{from_ffi_body}
    }}"#,
        ffi_name = ffi_name,
        from_ffi_body = indent_block(&from_ffi_body, 8)
    );

    let mut free_members = String::new();
    if index.struct_needs_free_members(&s.name) {
        let free_fn = free_members_fn_name(&s.name);
        free_members = format!(
            r#"    pub(crate) fn free_members(value: ffi::{ffi_name}) {{
        unsafe {{ ffi::{free_fn}(value) }};
    }}"#,
            ffi_name = ffi_name,
            free_fn = free_fn
        );
    }

    let mut extra_blocks = extra_methods.clone();
    if !from_ffi.is_empty() {
        extra_blocks.push(from_ffi);
    }
    if !free_members.is_empty() {
        extra_blocks.push(free_members);
    }
    let extra_methods_block = extra_blocks.join("\n\n");

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
{builder_methods}
{extra_methods}
}}

"#,
        name = name,
        fields = fields_block,
        defaults = default_fields_block,
        builder_methods = builder_methods_block,
        extra_methods = extra_methods_block
    )
}

fn emit_object(
    o: &ObjectModel,
    constructor: Option<&FunctionModel>,
    model: &ApiModel,
    index: &TypeIndex,
) -> String {
    let name = type_name(&o.name);
    let mut methods = Vec::new();

    if let Some(func) = constructor {
        let signature = fn_signature_params(&func.def.args, model, None);
        let (arg_prelude, ffi_args, has_callback) =
            emit_ffi_arg_prelude(&func.def.args, model, index);
        let func_name = ffi_fn_name(&func.name);
        let args = ffi_args.join(", ");
        let ffi_call = format!(
            "        let result = unsafe {{ ffi::{func}({args}) }};",
            func = func_name,
            args = args
        );
        let ret = emit_return_conversion(func.def.returns(), index, "result");
        let postlude = emit_out_struct_postlude(&func.def.args, index);
        let body = if postlude.is_empty() {
            format!("{}\n{}", ffi_call, ret)
        } else {
            format!("{}\n{}\n{}", ffi_call, postlude, ret)
        };
        methods.push(format!(
            r#"    pub fn new({signature}) -> Self {{
{arg_prelude}
{body}
    }}"#,
            signature = signature,
            arg_prelude = indent_block(&arg_prelude, 8),
            body = if has_callback {
                "        unimplemented!()".to_string()
            } else {
                body
            }
        ));
    }

    for method in &o.def.methods {
        let method_name = safe_ident(&snake_case_name(&method.name));
        let return_ty = method
            .returns()
            .map(|ret| rust_return_type(ret))
            .unwrap_or_else(|| "()".to_string());

        let signature = fn_signature_params(&method.args, model, Some("self"));
        let (arg_prelude, ffi_args, has_callback) =
            emit_ffi_arg_prelude(&method.args, model, index);
        let postlude = emit_out_struct_postlude(&method.args, index);

        methods.push(format!(
            r#"    pub fn {method_name}({signature}) -> {return_ty} {{
{arg_prelude}
{body}
    }}"#,
            method_name = method_name,
            signature = signature,
            return_ty = return_ty,
            arg_prelude = indent_block(&arg_prelude, 8),
            body = if has_callback {
                "        unimplemented!()".to_string()
            } else {
                let args = if ffi_args.is_empty() {
                    "".to_string()
                } else {
                    format!(", {}", ffi_args.join(", "))
                };
                if method
                    .returns()
                    .map(|ret| ret.get_type() == "void")
                    .unwrap_or(true)
                {
                    let func_name = ffi_fn_name(&format!("{} {}", o.name, method.name));
                    let postlude = if postlude.is_empty() {
                        String::new()
                    } else {
                        format!("\n{postlude}", postlude = postlude)
                    };
                    format!(
                        "        unsafe {{ ffi::{func}(self.raw{args}) }};{postlude}\n        ()",
                        func = func_name,
                        args = args,
                        postlude = postlude
                    )
                } else {
                    let func_name = ffi_fn_name(&format!("{} {}", o.name, method.name));
                    let ffi_call = format!(
                        "        let result = unsafe {{ ffi::{func}(self.raw{args}) }};",
                        func = func_name,
                        args = args
                    );
                    let ret = emit_return_conversion(method.returns(), index, "result");
                    if postlude.is_empty() {
                        format!("{}\n{}", ffi_call, ret)
                    } else {
                        format!("{}\n{}\n{}", ffi_call, postlude, ret)
                    }
                }
            }
        ));
    }

    let methods_block = methods.join("\n\n");

    format!(
        r#"#[derive(Debug)]
pub struct {name} {{
    raw: ffi::WGPU{name},
}}

impl {name} {{
    pub(crate) unsafe fn from_raw(raw: ffi::WGPU{name}) -> Self {{
        Self {{ raw }}
    }}

    pub fn as_raw(&self) -> ffi::WGPU{name} {{
        self.raw
    }}

{methods}
}}

impl Drop for {name} {{
    fn drop(&mut self) {{
        unsafe {{ ffi::wgpu{name}Release(self.raw) }};
    }}
}}

impl Clone for {name} {{
    fn clone(&self) -> Self {{
        unsafe {{ ffi::wgpu{name}AddRef(self.raw) }};
        Self {{ raw: self.raw }}
    }}
}}

"#,
        name = name,
        methods = methods_block
    )
}

fn emit_callback_function(c: &CallbackFunctionModel, index: &TypeIndex) -> String {
    let name = callback_type_name(&c.name);
    let args = callback_arg_list(&c.def.args);
    let signature = format!(r#"{args}"#, args = args);
    let trampoline = callback_trampoline_name(&c.name);
    let ffi_args = callback_ffi_arg_list(&c.def.args, index);

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
    let mut callback = Box::from_raw(userdata1.cast::<Option<{name}>>());
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

fn emit_callback(c: &CallbackModel) -> String {
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

fn emit_callback_info(c: &CallbackInfoModel, index: &TypeIndex) -> String {
    let name = type_name(&c.name);

    let mut fields = Vec::new();
    let mut builder_methods = Vec::new();

    for member in &c.def.members {
        let field_name = safe_ident(&snake_case_name(&member.name));
        let field_ty = struct_field_type(member, index);
        let param_ty = builder_param_type(member, index);
        fields.push(format!(
            r#"    pub {field_name}: {field_ty},"#,
            field_name = field_name,
            field_ty = field_ty
        ));

        builder_methods.push(format!(
            r#"    pub fn {field_name}(mut self, value: {param_ty}) -> Self {{
        self.{field_name} = Some(value);
        self
    }}"#,
            field_name = field_name,
            param_ty = param_ty
        ));
    }

    let fields_block = fields.join("\n");
    let builder_methods_block = builder_methods.join("\n\n");

    format!(
        r#"#[derive(Default)]
pub struct {name} {{
{fields}
}}

impl {name} {{
    pub fn new() -> Self {{
        Self::default()
    }}
{builder_methods}
}}

"#,
        name = name,
        fields = fields_block,
        builder_methods = builder_methods_block
    )
}

fn emit_function_pointer(fp: &crate::api_model::FunctionPointerModel) -> String {
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

fn emit_function(f: &FunctionModel, model: &ApiModel, index: &TypeIndex) -> String {
    let name = safe_ident(&snake_case_name(&f.name));
    let return_ty = f
        .def
        .returns()
        .map(|ret| rust_return_type(ret))
        .unwrap_or_else(|| "()".to_string());

    let signature = fn_signature_params(&f.def.args, model, None);
    let (arg_prelude, ffi_args, has_callback) = emit_ffi_arg_prelude(&f.def.args, model, index);
    let postlude = emit_out_struct_postlude(&f.def.args, index);

    format!(
        r#"pub fn {name}({signature}) -> {return_ty} {{
{arg_prelude}
{body}
}}

"#,
        name = name,
        signature = signature,
        return_ty = return_ty,
        arg_prelude = indent_block(&arg_prelude, 4),
        body = if has_callback {
            "    unimplemented!()".to_string()
        } else {
            if f.def.returns().map(|ret| ret.get_type() == "void").unwrap_or(true) {
                let func_name = ffi_fn_name(&f.name);
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
                let func_name = ffi_fn_name(&f.name);
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

fn emit_constant(c: &ConstantModel) -> Option<String> {
    let name = shouty_snake_case_name(&c.name);
    let ty = rust_type_for(&c.def.const_type);
    let value = match &c.def.value {
        Value::Number(num) => num.to_string(),
        Value::String(s) => {
            if let Some(parsed) = parse_numeric_string(s) {
                parsed.to_string()
            } else {
                return None;
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

fn build_constructor_map(model: &ApiModel) -> HashMap<String, FunctionModel> {
    let mut map = HashMap::new();
    for func in &model.functions {
        if let Some(obj_name) = func.name.strip_prefix("create ") {
            map.insert(obj_name.to_string(), func.clone());
        }
    }
    map
}

fn callback_arg_list(args: &[RecordMember]) -> String {
    let mut parts = Vec::new();
    for arg in args {
        let arg_ty = rust_param_type(arg);
        parts.push(arg_ty);
    }
    parts.join(", ")
}

fn callback_type_name(name: &str) -> String {
    let mut result = type_name(name);
    if !result.ends_with("Callback") {
        result.push_str("Callback");
    }
    result
}

fn callback_trampoline_name(name: &str) -> String {
    format!(r#"{}_trampoline"#, safe_ident(&snake_case_name(name)))
}

fn callback_ffi_arg_list(args: &[RecordMember], index: &TypeIndex) -> String {
    let mut parts = Vec::new();
    for arg in args {
        let arg_name = safe_ident(&snake_case_name(&arg.name));
        let arg_ty = callback_ffi_arg_type(arg, index);
        parts.push(format!(r#"{arg_name}: {arg_ty}"#, arg_name = arg_name, arg_ty = arg_ty));
    }
    parts.join(", ")
}

fn callback_ffi_arg_type(arg: &RecordMember, index: &TypeIndex) -> String {
    if arg.member_type == "string view" {
        return "ffi::WGPUStringView".to_string();
    }

    if arg.member_type == "bool" {
        return "ffi::WGPUBool".to_string();
    }

    if index.is_object(&arg.member_type) {
        let base = format!("ffi::WGPU{}", type_name(&arg.member_type));
        if arg.annotation.is_mut_ptr() {
            return format!("*mut {base}", base = base);
        }
        if arg.annotation.is_const_ptr() || arg.length.is_some() {
            return format!("*const {base}", base = base);
        }
        return base;
    }

    if index.is_enum(&arg.member_type) || index.is_bitmask(&arg.member_type) {
        let ffi_ty = ffi_type_name(&arg.member_type);
        return format!("ffi::{ffi_ty}", ffi_ty = ffi_ty);
    }

    if index.struct_extensible(&arg.member_type).is_some() {
        let ffi_ty = ffi_type_name(&arg.member_type);
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

fn fn_signature_params(args: &[RecordMember], model: &ApiModel, receiver: Option<&str>) -> String {
    let mut parts = Vec::new();
    let callback_info_map = build_callback_info_map(model);
    let callback_fn_map = build_callback_function_map(model);

    if let Some(recv) = receiver {
        if recv == "self" {
            parts.push("&self".to_string());
        }
    }

    for arg in args {
        if let Some(callback_fn_name) = callback_info_map.get(&arg.member_type) {
            let callback_args = callback_fn_map
                .get(callback_fn_name)
                .map(|c| callback_arg_list(&c.def.args))
                .unwrap_or_else(|| "".to_string());
            parts.push(format!(
                r#"callback: impl FnMut({callback_args}) + Send + 'static"#,
                callback_args = callback_args
            ));
            continue;
        }

        let param_name = safe_ident(&snake_case_name(&arg.name));
        let param_ty = rust_param_type(arg);
        let needs_mut = arg.length.is_some() && arg.annotation.is_mut_ptr() && !arg.optional;
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

fn rust_return_type(ret: &ReturnType) -> String {
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

// args_usage_list removed: superseded by emit_ffi_arg_prelude.

struct TypeIndex {
    objects: HashMap<String, ()>,
    enums: HashMap<String, ()>,
    bitmasks: HashMap<String, ()>,
    structs: HashMap<String, bool>,
    callback_infos: HashMap<String, ()>,
    structs_need_free: HashMap<String, ()>,
}

impl TypeIndex {
    fn new(model: &ApiModel) -> Self {
        let mut objects = HashMap::new();
        let mut enums = HashMap::new();
        let mut bitmasks = HashMap::new();
        let mut structs = HashMap::new();
        let mut callback_infos = HashMap::new();
        let mut structs_need_free = HashMap::new();

        for o in &model.objects {
            objects.insert(o.name.clone(), ());
        }
        for e in &model.enums {
            enums.insert(e.name.clone(), ());
        }
        for b in &model.bitmasks {
            bitmasks.insert(b.name.clone(), ());
        }
        for s in &model.structures {
            structs.insert(s.name.clone(), s.def.extensible.is_extensible());
            if struct_needs_free_members(s) {
                structs_need_free.insert(s.name.clone(), ());
            }
        }
        for c in &model.callback_infos {
            callback_infos.insert(c.name.clone(), ());
        }

        Self {
            objects,
            enums,
            bitmasks,
            structs,
            callback_infos,
            structs_need_free,
        }
    }

    fn is_object(&self, name: &str) -> bool {
        self.objects.contains_key(name)
    }

    fn is_enum(&self, name: &str) -> bool {
        self.enums.contains_key(name)
    }

    fn is_bitmask(&self, name: &str) -> bool {
        self.bitmasks.contains_key(name)
    }

    fn struct_extensible(&self, name: &str) -> Option<bool> {
        self.structs.get(name).copied()
    }

    fn is_callback_info(&self, name: &str) -> bool {
        self.callback_infos.contains_key(name)
    }

    fn struct_needs_free_members(&self, name: &str) -> bool {
        self.structs_need_free.contains_key(name)
    }
}

fn emit_ffi_arg_prelude(
    args: &[RecordMember],
    model: &ApiModel,
    index: &TypeIndex,
) -> (String, Vec<String>, bool) {
    let mut prelude = Vec::new();
    let mut ffi_args = Vec::new();
    let mut has_callback = false;
    let callback_info_map = build_callback_info_map(model);
    let callback_info_mode_map = build_callback_info_mode_map(model);

    for arg in args {
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
                    "            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,\n"
                } else {
                    ""
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
                    r#"        let {name}_ffi = ffi::WGPU{info_name} {{
            nextInChain: std::ptr::null_mut(),
{mode_line}            callback: Some({trampoline}),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        }};"#,
                    name = name,
                    info_name = info_name,
                    trampoline = trampoline,
                    mode_line = mode_line
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
            if index.is_object(&arg.member_type) && !arg.optional {
                let obj = type_name(&arg.member_type);
                prelude.push(format!(
                    r#"        let {name}_raw: Vec<ffi::WGPU{obj}> = {name}.iter().map(|v| v.raw).collect();"#,
                    name = name,
                    obj = obj
                ));
                if arg.annotation.is_mut_ptr() {
                    ffi_args.push(format!("{name}_raw.as_mut_ptr()", name = name));
                } else {
                    ffi_args.push(format!("{name}_raw.as_ptr()", name = name));
                }
                continue;
            }
            if arg.optional {
                prelude.push(format!(r#"        let _ = {name};"#, name = name));
                if arg.annotation.is_mut_ptr() {
                    ffi_args.push("std::ptr::null_mut()".to_string());
                } else {
                    ffi_args.push("std::ptr::null()".to_string());
                }
            } else {
                if arg.annotation.is_mut_ptr() {
                    ffi_args.push(format!("{name}.as_mut_ptr()", name = name));
                } else {
                    ffi_args.push(format!("{name}.as_ptr()", name = name));
                }
            }
            continue;
        }
        if index.is_object(&arg.member_type) {
            if arg.optional {
                prelude.push(format!(
                    r#"        let {name}_raw = {name}.as_ref().map(|v| v.raw).unwrap_or(std::ptr::null_mut());"#,
                    name = name
                ));
                ffi_args.push(format!("{name}_raw", name = name));
            } else {
                ffi_args.push(format!("{name}.raw", name = name));
            }
            continue;
        }

        if index.is_enum(&arg.member_type) || index.is_bitmask(&arg.member_type) {
            let ffi_ty = ffi_type_name(&arg.member_type);
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
            ffi::WGPUStringView {{ data: value.as_ptr().cast(), length: value.len() }}
        }} else {{
            ffi::WGPUStringView {{ data: std::ptr::null(), length: 0 }}
        }};"#,
                    name = name
                ));
            } else {
                prelude.push(format!(
                    r#"        let {name}_ffi = ffi::WGPUStringView {{
            data: {name}.as_ptr().cast(),
            length: {name}.len(),
        }};"#,
                    name = name
                ));
            }
            ffi_args.push(format!("{name}_ffi", name = name));
            continue;
        }

        if arg.member_type == "bool" {
            prelude.push(format!(
                r#"        let {name}_ffi: ffi::WGPUBool = if {name} {{ 1 }} else {{ 0 }};"#,
                name = name
            ));
            ffi_args.push(format!("{name}_ffi", name = name));
            continue;
        }

        if let Some(is_extensible) = index.struct_extensible(&arg.member_type) {
            if arg.optional {
                prelude.push(format!(r#"        let _ = {name};"#, name = name));
                ffi_args.push("std::ptr::null()".to_string());
                continue;
            }

            if arg.annotation.is_const_ptr() {
                if is_extensible {
                    prelude.push(format!(
                        r#"        let ({name}_ffi, _{name}_storage) = {name}.to_ffi();"#,
                        name = name
                    ));
                } else {
                    prelude.push(format!(r#"        let {name}_ffi = {name}.to_ffi();"#, name = name));
                }
                prelude.push(format!(
                    r#"        let {name}_ptr = std::ptr::addr_of!({name}_ffi);"#,
                    name = name
                ));
                ffi_args.push(format!("{name}_ptr", name = name));
                continue;
            }

            if arg.annotation.is_mut_ptr() {
                if is_extensible {
                    prelude.push(format!(
                        r#"        let (mut {name}_ffi, _{name}_storage) = {name}.to_ffi();"#,
                        name = name
                    ));
                } else {
                    prelude.push(format!(r#"        let mut {name}_ffi = {name}.to_ffi();"#, name = name));
                }
                prelude.push(format!(
                    r#"        let {name}_ptr = std::ptr::addr_of_mut!({name}_ffi);"#,
                    name = name
                ));
                ffi_args.push(format!("{name}_ptr", name = name));
                continue;
            }

            if is_extensible {
                prelude.push(format!(
                    r#"        let ({name}_ffi, _{name}_storage) = {name}.to_ffi();"#,
                    name = name
                ));
            } else {
                prelude.push(format!(r#"        let {name}_ffi = {name}.to_ffi();"#, name = name));
            }
            ffi_args.push(format!("{name}_ffi", name = name));
            continue;
        }

        ffi_args.push(name);
    }

    (prelude.join("\n"), ffi_args, has_callback)
}

fn emit_out_struct_postlude(args: &[RecordMember], index: &TypeIndex) -> String {
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
        if index.struct_needs_free_members(&arg.member_type) {
            let free_fn = free_members_fn_name(&arg.member_type);
            lines.push(format!(
                r#"        unsafe {{ ffi::{free_fn}({name}_ffi) }};"#,
                free_fn = free_fn,
                name = name
            ));
        }
    }
    lines.join("\n")
}

fn emit_return_conversion(ret: Option<&ReturnType>, index: &TypeIndex, name: &str) -> String {
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
        return format!(r#"        unsafe {{ {obj}::from_raw({name}) }}"#, name = name, obj = obj);
    }

    if index.is_enum(ty) || index.is_bitmask(ty) {
        return format!(r#"        {name}.into()"#, name = name);
    }

    if index.struct_extensible(ty).is_some() {
        return format!(r#"        {ty}::new()"#, ty = type_name(ty));
    }

    if ret.get_type() == "void" {
        return "        let _ = ();".to_string();
    }

    if ret.get_type() == "bool" {
        return format!(r#"        {name} != 0"#, name = name);
    }

    format!(r#"        {name}"#, name = name)
}

fn indent_block(text: &str, spaces: usize) -> String {
    if text.is_empty() {
        return String::new();
    }
    let pad = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{pad}{line}", pad = pad, line = line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn rust_param_type(arg: &RecordMember) -> String {
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

fn struct_field_type(member: &RecordMember, index: &TypeIndex) -> String {
    if is_char_string_list(member) {
        return "Option<Vec<String>>".to_string();
    }

    let base = rust_type_for(&member.member_type);
    let is_struct = index.struct_extensible(&member.member_type).is_some()
        || index.is_callback_info(&member.member_type);

    let ty = if member.length.is_some() {
        format!(r#"Vec<{base}>"#, base = base)
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

fn builder_param_type(member: &RecordMember, index: &TypeIndex) -> String {
    if is_char_string_list(member) {
        return "Vec<String>".to_string();
    }

    let base = rust_type_for(&member.member_type);
    let is_struct = index.struct_extensible(&member.member_type).is_some()
        || index.is_callback_info(&member.member_type);

    if member.length.is_some() {
        format!(r#"Vec<{base}>"#, base = base)
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

fn member_default_expr(member: &RecordMember, index: &TypeIndex) -> Option<String> {
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
            let ffi_ty = ffi_type_name(ty);
            format!("{enum_ty}::from({value} as ffi::{ffi_ty})", enum_ty = type_name(ty))
        } else if index.is_bitmask(ty) {
            format!(
                "{mask_ty}::from_bits_truncate({value} as u64)",
                mask_ty = type_name(ty)
            )
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
                Some(format!(r#"{}::{}"#, type_name(ty), enum_variant_name_camel(s)))
            } else if index.is_bitmask(ty) {
                Some(format!(r#"{}::{}"#, type_name(ty), bitmask_variant_name(s)))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn rust_type_for(name: &str) -> String {
    match name {
        "void" => "std::ffi::c_void".to_string(),
        "void *" => "*mut std::ffi::c_void".to_string(),
        "void const *" => "*const std::ffi::c_void".to_string(),
        "bool" => "bool".to_string(),
        "int" => "i32".to_string(),
        "uint" => "u32".to_string(),
        "uint8_t" => "u8".to_string(),
        "uint16_t" => "u16".to_string(),
        "uint32_t" => "u32".to_string(),
        "uint64_t" => "u64".to_string(),
        "int8_t" => "i8".to_string(),
        "int16_t" => "i16".to_string(),
        "int32_t" => "i32".to_string(),
        "int64_t" => "i64".to_string(),
        "size_t" => "usize".to_string(),
        "float" => "f32".to_string(),
        "double" => "f64".to_string(),
        "char" => "std::os::raw::c_char".to_string(),
        "string view" => "String".to_string(),
        other => type_name(other),
    }
}

fn type_name(name: &str) -> String {
    let raw = camel_case_with_acronyms(name);
    normalize_digit_prefix_camel(&raw)
}

fn enum_variant_name_camel(name: &str) -> String {
    let raw = camel_case_with_acronyms(name);
    normalize_digit_prefix_camel(&raw)
}

fn bitmask_variant_name(name: &str) -> String {
    let raw = shouty_snake_case_name(name);
    if let Some(first) = raw.chars().next() {
        if first.is_ascii_digit() {
            return normalize_digit_prefix(&raw);
        }
    }
    raw
}

fn safe_ident(name: &str) -> String {
    let keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn",
    ];

    if keywords.contains(&name) {
        format!(r#"r#{name}"#, name = name)
    } else {
        name.to_string()
    }
}

fn parse_numeric_string(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix("0x") {
        u32::from_str_radix(hex, 16).ok()
    } else if let Some(hex) = trimmed.strip_prefix("0X") {
        u32::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u32>().ok()
    }
}

fn normalize_digit_prefix_camel(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return value.to_string();
    };
    if !first.is_ascii_digit() {
        return value.to_string();
    }
    let Some(second) = chars.next() else {
        return value.to_string();
    };
    let mut rest: String = chars.collect();
    rest.insert(0, first);
    let mut out = String::new();
    out.push(second.to_ascii_uppercase());
    out.push_str(&rest);
    out
}

fn normalize_digit_prefix(raw: &str) -> String {
    let mut chars = raw.chars().peekable();
    let mut digits = String::new();
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            digits.push(*c);
            chars.next();
        } else {
            break;
        }
    }

    let rest: String = chars.collect();
    if let Some(stripped) = rest.strip_prefix('D') {
        format!("D{digits}{stripped}", digits = digits, stripped = stripped)
    } else {
        format!("D{digits}{rest}", digits = digits, rest = rest)
    }
}

fn normalize_digit_prefix_snake(raw: &str) -> String {
    let mut chars = raw.chars();
    let Some(first) = chars.next() else {
        return raw.to_string();
    };
    if !first.is_ascii_digit() {
        return raw.to_string();
    }
    let Some(second) = chars.next() else {
        return raw.to_string();
    };
    let mut rest: String = chars.collect();
    rest.insert(0, first);
    let mut out = String::new();
    out.push(second);
    out.push_str(&rest);
    out
}

fn uppercase_after_digits(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut prev_digit = false;
    for ch in value.chars() {
        if prev_digit && ch.is_ascii_lowercase() {
            out.push(ch.to_ascii_uppercase());
        } else {
            out.push(ch);
        }
        prev_digit = ch.is_ascii_digit();
    }
    out
}

fn ffi_type_name(canonical: &str) -> String {
    format!("WGPU{}", camel_case_with_acronyms(canonical))
}

fn ffi_fn_name(canonical: &str) -> String {
    format!("wgpu{}", camel_case_with_acronyms(canonical))
}

fn camel_case_name(name: &str) -> String {
    name.to_upper_camel_case()
}

fn snake_case_name(name: &str) -> String {
    let raw = name.to_snake_case();
    normalize_digit_prefix_snake(&raw)
}

fn shouty_snake_case_name(name: &str) -> String {
    let raw = name.to_shouty_snake_case();
    if let Some(first) = raw.chars().next() {
        if first.is_ascii_digit() {
            return normalize_digit_prefix(&raw);
        }
    }
    raw
}

fn camel_case_with_acronyms(name: &str) -> String {
    let mut out = camel_case_name(name);
    for (from, to) in acronym_fixes() {
        out = out.replace(from, to);
    }
    uppercase_after_digits(&out)
}

fn acronym_fixes() -> &'static [(&'static str, &'static str)] {
    &[
        ("WebGpu", "WebGPU"),
        ("WebXr", "WebXR"),
        ("Webgpu", "WebGPU"),
        ("Webxr", "WebXR"),
        ("Wgpu", "WGPU"),
        ("Wgsl", "WGSL"),
        ("Hlsl", "HLSL"),
        ("Spirv", "SPIRV"),
        ("Dxgi", "DXGI"),
        ("Mtl", "MTL"),
        ("IoSurface", "IOSurface"),
        ("Hwnd", "HWND"),
        ("Uwp", "UWP"),
        ("WinUi", "WinUI"),
        ("Oom", "OOM"),
        ("Fd", "FD"),
        ("D3d12", "D3D12"),
        ("D3d11", "D3D11"),
        ("D3d", "D3D"),
        ("Vk", "Vk"),
        ("Xcb", "XCB"),
        ("OpenGl", "OpenGL"),
        ("OpenGles", "OpenGLES"),
        ("Egl", "EGL"),
    ]
}

fn build_callback_info_map(model: &ApiModel) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for info in &model.callback_infos {
        if let Some(callback_member) = info
            .def
            .members
            .iter()
            .find(|member| member.name == "callback")
        {
            map.insert(info.name.clone(), callback_member.member_type.clone());
        }
    }
    map
}

fn build_callback_function_map(model: &ApiModel) -> HashMap<String, CallbackFunctionModel> {
    let mut map = HashMap::new();
    for callback in &model.callback_functions {
        map.insert(callback.name.clone(), callback.clone());
    }
    map
}

fn build_callback_info_mode_map(model: &ApiModel) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for info in &model.callback_infos {
        let has_mode = info.def.members.iter().any(|member| member.name == "mode");
        map.insert(info.name.clone(), has_mode);
    }
    map
}

fn build_stype_map(model: &ApiModel) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let stype_enum_name = "s type";
    let enum_name = type_name(stype_enum_name);
    if let Some(stype_enum) = model.enums.iter().find(|e| e.name == stype_enum_name) {
        for value in &stype_enum.def.values {
            let variant = enum_variant_name_camel(&value.name);
            map.insert(value.name.clone(), format!(r#"{enum_name}::{variant}"#));
        }
    }
    map
}

fn length_value_expr(length: &LengthValue) -> String {
    match length {
        LengthValue::Number(value) => value.to_string(),
        LengthValue::String(name) => safe_ident(&snake_case_name(name)),
    }
}

fn is_char_string_list(member: &RecordMember) -> bool {
    member.member_type == "char" && member.length.is_some() && member.annotation.is_const_const_ptr()
}

fn struct_needs_free_members(s: &StructureModel) -> bool {
    let is_out = s.def.extensible.is_output() || s.def.chained.as_deref() == Some("out") || s.def.out == Some(true);
    if !is_out {
        return false;
    }
    s.def.members.iter().any(|member| {
        if member.member_type == "string view" || is_char_string_list(member) {
            return true;
        }
        if member.length.is_some() {
            return true;
        }
        !member.annotation.is_value()
    })
}

fn ffi_field_name(name: &str) -> String {
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

fn ffi_length_expr(length: &LengthValue) -> String {
    match length {
        LengthValue::Number(value) => format!("{value}usize", value = value),
        LengthValue::String(name) => format!("value.{} as usize", ffi_field_name(name)),
    }
}

fn free_members_fn_name(struct_name: &str) -> String {
    format!("wgpu{}FreeMembers", type_name(struct_name))
}

fn emit_struct_from_ffi_body(s: &StructureModel, index: &TypeIndex) -> String {
    let mut fields = Vec::new();

    if s.def.extensible.is_extensible() {
        fields.push("extensions: Vec::new(),".to_string());
    }

    for member in &s.def.members {
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
                format!(
                    r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some(
                unsafe {{ std::slice::from_raw_parts(value.{ffi_field}, {len_expr}) }}
                    .iter()
                    .map(|raw| {rust_ty}::from_ffi(*raw))
                    .collect(),
            )
        }}"#,
                    ffi_field = ffi_field,
                    len_expr = len_expr,
                    rust_ty = rust_ty
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
                format!("Some(string_view_to_string(value.{ffi_field}))", ffi_field = ffi_field)
            }
        } else if index.is_enum(&member.member_type) || index.is_bitmask(&member.member_type) {
            format!("Some(value.{ffi_field}.into())", ffi_field = ffi_field)
        } else if index.is_callback_info(&member.member_type) {
            "None".to_string()
        } else if index.is_object(&member.member_type) {
            let obj = type_name(&member.member_type);
            if member.optional {
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
            format!(
                r#"Some({rust_ty}::from_ffi(value.{ffi_field}))"#,
                rust_ty = rust_ty,
                ffi_field = ffi_field
            )
        } else if index.struct_extensible(&member.member_type).is_some()
            && (member.annotation.is_const_ptr() || member.annotation.is_mut_ptr())
        {
            let rust_ty = type_name(&member.member_type);
            format!(
                r#"if value.{ffi_field}.is_null() {{
            None
        }} else {{
            Some({rust_ty}::from_ffi(unsafe {{ *value.{ffi_field} }}))
        }}"#,
                ffi_field = ffi_field,
                rust_ty = rust_ty
            )
        } else if member.member_type == "bool" {
            format!("Some(value.{ffi_field} != 0)", ffi_field = ffi_field)
        } else {
            format!("Some(value.{ffi_field})", ffi_field = ffi_field)
        };

        fields.push(format!(r#"    {field_name}: {value_expr},"#, field_name = field_name, value_expr = value_expr));
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
