use crate::api_model::{
    ApiModel, BitmaskModel, CallbackFunctionModel, CallbackInfoModel, CallbackModel, ConstantModel,
    EnumModel, FunctionModel, ObjectModel, StructureModel,
};
use crate::parser::{EnumValueDef, LengthValue, RecordMember, ReturnType};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub struct GeneratedFiles {
    pub enums: String,
    pub structs: String,
    pub extensions: String,
    pub objects: String,
    pub callbacks: String,
    pub functions: String,
    pub constants: String,
    pub mod_rs: String,
}

impl GeneratedFiles {
    pub fn write_to_dir(&self, out_dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(out_dir)?;
        fs::write(out_dir.join("enums.rs"), &self.enums)?;
        fs::write(out_dir.join("structs.rs"), &self.structs)?;
        fs::write(out_dir.join("extensions.rs"), &self.extensions)?;
        fs::write(out_dir.join("objects.rs"), &self.objects)?;
        fs::write(out_dir.join("callbacks.rs"), &self.callbacks)?;
        fs::write(out_dir.join("functions.rs"), &self.functions)?;
        fs::write(out_dir.join("constants.rs"), &self.constants)?;
        fs::write(out_dir.join("mod.rs"), &self.mod_rs)?;
        Ok(())
    }
}

pub fn generate_strings(model: &ApiModel) -> GeneratedFiles {
    generate_strings_with_ffi_consts(model, None)
}

pub fn generate_strings_with_ffi_consts(
    model: &ApiModel,
    ffi_consts: Option<&HashSet<String>>,
) -> GeneratedFiles {
    let c_prefix = model.c_prefix.clone();
    let enums = format_rust_source(&emit_enums(model, &c_prefix, ffi_consts));
    let structs = format_rust_source(&emit_structs(model, &c_prefix));
    let extensions = format_rust_source(&emit_extensions(model, &c_prefix));
    let objects = format_rust_source(&emit_objects(model, &c_prefix));
    let callbacks = format_rust_source(&emit_callbacks(model, &c_prefix));
    let functions = format_rust_source(&emit_functions(model, &c_prefix));
    let constants = format_rust_source(&emit_constants(model));
    let mod_rs = format_rust_source(&emit_mod_rs());

    GeneratedFiles {
        enums,
        structs,
        extensions,
        objects,
        callbacks,
        functions,
        constants,
        mod_rs,
    }
}

pub fn generate_to_dir(model: &ApiModel, out_dir: &Path) -> std::io::Result<()> {
    let files = generate_strings(model);
    files.write_to_dir(out_dir)
}

fn emit_mod_rs() -> String {
    let content = r#"#![allow(dead_code, unused_imports)]

mod enums;
mod structs;
mod extensions;
mod objects;
mod callbacks;
mod functions;
mod constants;

pub use enums::*;
pub use structs::*;
pub use extensions::*;
pub use objects::*;
pub use callbacks::*;
pub use functions::*;
pub use constants::*;
"#;

    content
        .replacen(
            "#![allow(dead_code, unused_imports)]",
            "#[allow(dead_code, unused_imports)]",
            1,
        )
        .to_string()
}

fn format_rust_source(source: &str) -> String {
    match syn::parse_file(source) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => source.to_string(),
    }
}

fn emit_enums(model: &ApiModel, c_prefix: &str, ffi_consts: Option<&HashSet<String>>) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use bitflags::bitflags;

"#,
    );

    for e in &model.enums {
        out.push_str(&emit_enum(e, c_prefix, ffi_consts));
    }
    for b in &model.bitmasks {
        out.push_str(&emit_bitmask(b, c_prefix, ffi_consts));
    }

    out
}

fn emit_structs(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;
use std::any::Any;
use std::ffi::CStr;

fn string_view_to_string(view: ffi::{prefix}StringView) -> String {{
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

fn emit_extensions(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;
use std::any::Any;

pub(crate) struct ChainedStructStorage {{
    entries: Vec<Box<ffi::{prefix}ChainedStruct>>,
    buffers: Vec<Box<dyn Any>>,
    nested: Vec<ChainedStructStorage>,
}}

impl ChainedStructStorage {{
    pub(crate) fn new() -> Self {{
        Self {{
            entries: Vec::new(),
            buffers: Vec::new(),
            nested: Vec::new(),
        }}
    }}

    pub(crate) fn push(
        &mut self,
        s_type: ffi::{prefix}SType,
        next: *mut ffi::{prefix}ChainedStruct,
    ) -> *mut ffi::{prefix}ChainedStruct {{
        let mut node = Box::new(ffi::{prefix}ChainedStruct {{ next, sType: s_type }});
        let ptr = std::ptr::from_mut(node.as_mut());
        self.entries.push(node);
        ptr
    }}

    pub(crate) fn push_value<T: 'static>(&mut self, value: T) -> *const T {{
        let boxed = Box::new(value);
        let ptr = std::ptr::from_ref(boxed.as_ref());
        self.buffers.push(boxed);
        ptr
    }}

    pub(crate) fn push_value_mut<T: 'static>(&mut self, value: T) -> *mut T {{
        let mut boxed = Box::new(value);
        let ptr = std::ptr::from_mut(boxed.as_mut());
        self.buffers.push(boxed);
        ptr
    }}

    pub(crate) fn push_vec<T: 'static>(&mut self, value: Vec<T>) -> *const T {{
        let ptr = value.as_ptr();
        self.buffers.push(Box::new(value));
        ptr
    }}

    pub(crate) fn push_vec_mut<T: 'static>(&mut self, value: Vec<T>) -> *mut T {{
        let mut value = value;
        let ptr = value.as_mut_ptr();
        self.buffers.push(Box::new(value));
        ptr
    }}

    pub(crate) fn push_any<T: 'static>(&mut self, value: T) {{
        self.buffers.push(Box::new(value));
    }}

    pub(crate) fn push_storage(&mut self, storage: ChainedStructStorage) {{
        self.nested.push(storage);
    }}
}}

"#,
        prefix = c_prefix
    ));

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

    let stype_map = build_stype_map(model, c_prefix);

    for root in root_names {
        let variants = roots.get(&root).cloned().unwrap_or_default();
        let enum_name = format!("{}Extension", type_name(&root));

        let mut variant_lines = Vec::new();
        let mut from_lines = Vec::new();
        for s in variants.iter() {
            let ty = type_name(&s.name);
            variant_lines.push(format!(r#"{ty}({ty}),"#));
            from_lines.push(format!(
                r#"impl std::convert::From<{ty}> for {enum_name} {{
                    fn from(ext: {ty}) -> Self {{
                        {enum_name}::{ty}(ext)
                    }}
                }}"#
            ));
        }
        let variant_block = variant_lines.join("\n");
        let from_block = from_lines.join("\n");

        let has_variants = !variants.is_empty();
        let impl_block = if has_variants {
            format!(
                r#"impl {enum_name} {{
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::{prefix}ChainedStruct,
    ) -> *mut ffi::{prefix}ChainedStruct {{
        match self {{
{push_arms}
        }}
    }}
}}

"#,
                enum_name = enum_name,
                prefix = c_prefix,
                push_arms = {
                    let mut arms = Vec::new();
                    for s in variants.iter() {
                        let ty = type_name(&s.name);
                        let stype_const = stype_map.get(&s.name).cloned().unwrap_or_else(|| {
                            format!(
                                r#"{}::{}"#,
                                type_name("s type"),
                                enum_variant_name_camel(&s.name)
                            )
                        });
                        arms.push(format!(
                            r#"            {enum_name}::{ty}(value) => {{
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = {stype_const} as ffi::{prefix}SType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::{prefix}ChainedStruct>()
            }}"#,
                            enum_name = enum_name,
                            ty = ty,
                            stype_const = stype_const,
                            prefix = c_prefix
                        ));
                    }
                    arms.join("\n")
                }
            )
        } else {
            format!(
                r#"impl {enum_name} {{
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::{prefix}ChainedStruct,
    ) -> *mut ffi::{prefix}ChainedStruct {{
        let _ = self;
        let _ = storage;
        next
    }}
}}

"#,
                enum_name = enum_name,
                prefix = c_prefix
            )
        };

        out.push_str(&format!(
            r#"#[allow(dead_code)]
pub enum {enum_name} {{
{variants}
}}

{from_block}

{impl_block}
"#,
            enum_name = enum_name,
            variants = variant_block,
            impl_block = impl_block
        ));
    }

    out
}

fn emit_objects(model: &ApiModel, c_prefix: &str) -> String {
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
        out.push_str(&emit_object(
            o,
            constructors.get(&o.name),
            model,
            &index,
            c_prefix,
        ));
    }

    out
}

fn emit_callbacks(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;
use std::cell::RefCell;

fn string_view_to_string(view: ffi::{prefix}StringView) -> String {{
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

    for fp in &model.function_pointers {
        out.push_str(&emit_function_pointer(fp));
    }

    for c in &model.callback_functions {
        out.push_str(&emit_callback_function(c, &index, c_prefix));
    }

    for c in &model.callbacks {
        out.push_str(&emit_callback(c));
    }

    for c in &model.callback_infos {
        out.push_str(&emit_callback_info(c, &index));
    }

    out
}

fn emit_functions(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;

"#,
    );

    let index = TypeIndex::new(model);
    for f in &model.functions {
        out.push_str(&emit_function(f, model, &index, c_prefix));
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

fn emit_enum(e: &EnumModel, c_prefix: &str, ffi_consts: Option<&HashSet<String>>) -> String {
    let name = type_name(&e.name);
    let mut variants = Vec::new();
    let mut from_arms = Vec::new();
    let ffi_type = ffi_type_name(&e.name, c_prefix);
    let mut first_variant: Option<String> = None;

    for v in &e.def.values {
        let variant = enum_variant_name_camel(&v.name);
        let const_variant = ffi_enum_value_name(&v.name);
        let const_name = ffi_enum_const_name(&ffi_type, &const_variant);
        if !ffi_const_is_available(ffi_consts, &const_name) {
            continue;
        }
        variants.push(format!(
            r#"    {variant} = ffi::{const_name} as u32,"#,
            variant = variant,
            const_name = const_name
        ));
        from_arms.push(format!(
            r#"            ffi::{const_name} => {name}::{variant},"#,
            const_name = const_name,
            name = name,
            variant = variant
        ));
        if first_variant.is_none() {
            first_variant = Some(variant);
        }
    }

    let variants_block = variants.join("\n");
    let from_arms_block = from_arms.join("\n");
    let fallback_variant = first_variant.unwrap_or_else(|| "Undefined".to_string());

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

fn emit_bitmask(b: &BitmaskModel, c_prefix: &str, ffi_consts: Option<&HashSet<String>>) -> String {
    let name = type_name(&b.name);
    let mut variants = Vec::new();
    let ffi_type = ffi_type_name(&b.name, c_prefix);

    for v in &b.def.values {
        let variant = bitmask_variant_name(&v.name);
        let const_variant = ffi_enum_value_name(&v.name);
        let const_name = ffi_bitmask_const_name(&ffi_type, &const_variant);
        if !ffi_const_is_available(ffi_consts, &const_name) {
            continue;
        }
        variants.push(format!(
            r#"        const {variant} = ffi::{const_name} as u64;"#,
            variant = variant,
            const_name = const_name
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

fn ffi_const_is_available(ffi_consts: Option<&HashSet<String>>, name: &str) -> bool {
    ffi_consts.map_or(true, |set| set.contains(name))
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

fn emit_struct(
    s: &StructureModel,
    index: &TypeIndex,
    c_prefix: &str,
    callback_info_map: &HashMap<String, String>,
    callback_info_mode_map: &HashMap<String, bool>,
    constant_map: &HashMap<String, String>,
) -> String {
    let name = type_name(&s.name);
    let ffi_name = ffi_type_name(&s.name, c_prefix);

    let mut fields = Vec::new();
    let mut default_fields = Vec::new();
    let mut extra_methods = Vec::new();
    let length_fields = length_field_names(&s.def.members);
    let needs_free_members = index.struct_needs_free_members(&s.name);

    if s.def.extensible.is_extensible() {
        let ext_enum = format!("{}Extension", type_name(&s.name));
        fields.push(format!(
            r#"pub(crate) extensions: Vec<{ext_enum}>,"#,
            ext_enum = ext_enum
        ));
        default_fields.push("extensions: Vec::new(),".to_string());
        let to_ffi_body = emit_struct_to_ffi_body(
            s,
            index,
            c_prefix,
            true,
            callback_info_map,
            callback_info_mode_map,
        );
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
        let to_ffi_body = emit_struct_to_ffi_body(
            s,
            index,
            c_prefix,
            false,
            callback_info_map,
            callback_info_mode_map,
        );
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

    if needs_free_members {
        fields.push(format!(
            r#"#[doc(hidden)]
    pub(crate) _free_members: Option<ffi::{ffi_name}>,"#,
            ffi_name = ffi_name
        ));
        default_fields.push("        _free_members: None,".to_string());
    }

    let fields_block = fields.join("\n");
    let default_fields_block = default_fields.join("\n");
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
    if needs_free_members {
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
    let drop_impl = if needs_free_members {
        let free_fn = free_members_fn_name(&s.name);
        format!(
            r#"impl Drop for {name} {{
    fn drop(&mut self) {{
        if let Some(value) = self._free_members.take() {{
            unsafe {{ ffi::{free_fn}(value) }};
        }}
    }}
}}

"#,
            name = name,
            free_fn = free_fn
        )
    } else {
        String::new()
    };

    let struct_block = format!(
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
{extra_methods}
}}

"#,
        name = name,
        fields = fields_block,
        defaults = default_fields_block,
        extra_methods = extra_methods_block
    );
    format!(
        "{struct_block}{drop_impl}",
        struct_block = struct_block,
        drop_impl = drop_impl
    )
}

fn emit_object(
    o: &ObjectModel,
    constructor: Option<&FunctionModel>,
    model: &ApiModel,
    index: &TypeIndex,
    c_prefix: &str,
) -> String {
    let name = type_name(&o.name);
    let mut methods = Vec::new();
    let no_autolock = o.def.no_autolock.unwrap_or(false);
    let marker_field = if no_autolock {
        "    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,\n"
    } else {
        ""
    };
    let marker_init = if no_autolock {
        ", _not_sync: std::marker::PhantomData"
    } else {
        ""
    };
    let send_sync_impl = if no_autolock {
        String::new()
    } else {
        format!(
            r#"unsafe impl Send for {name} {{}}

unsafe impl Sync for {name} {{}}

"#,
            name = name
        )
    };

    if let Some(func) = constructor {
        let signature = fn_signature_params(&func.def.args, model, None);
        let (arg_prelude, ffi_args, has_callback) =
            emit_ffi_arg_prelude(&func.def.args, model, index, c_prefix);
        let func_name = ffi_fn_name(&func.name, c_prefix);
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
            emit_ffi_arg_prelude(&method.args, model, index, c_prefix);
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
                    let func_name = ffi_fn_name(&format!("{} {}", o.name, method.name), c_prefix);
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
                    let func_name = ffi_fn_name(&format!("{} {}", o.name, method.name), c_prefix);
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
    raw: ffi::{prefix}{name},
{marker_field}}}

impl {name} {{
    pub(crate) unsafe fn from_raw(raw: ffi::{prefix}{name}) -> Self {{
        Self {{ raw{marker_init} }}
    }}

    pub fn as_raw(&self) -> ffi::{prefix}{name} {{
        self.raw
    }}

{methods}
}}

impl Drop for {name} {{
    fn drop(&mut self) {{
        if self.as_raw().is_null() {{
            return;
        }}
        unsafe {{ ffi::wgpu{name}Release(self.raw) }};
    }}
}}

impl Clone for {name} {{
    fn clone(&self) -> Self {{
        unsafe {{ ffi::wgpu{name}AddRef(self.raw) }};
        Self {{ raw: self.raw{marker_init} }}
    }}
}}

{send_sync_impl}"#,
        name = name,
        methods = methods_block,
        prefix = c_prefix,
        marker_field = marker_field,
        marker_init = marker_init,
        send_sync_impl = send_sync_impl
    )
}

fn emit_callback_function(c: &CallbackFunctionModel, index: &TypeIndex, c_prefix: &str) -> String {
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

fn emit_function(f: &FunctionModel, model: &ApiModel, index: &TypeIndex, c_prefix: &str) -> String {
    let name = safe_ident(&snake_case_name(&f.name));
    let return_ty = f
        .def
        .returns()
        .map(|ret| rust_return_type(ret))
        .unwrap_or_else(|| "()".to_string());

    let signature = fn_signature_params(&f.def.args, model, None);
    let (arg_prelude, ffi_args, has_callback) =
        emit_ffi_arg_prelude(&f.def.args, model, index, c_prefix);
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

fn emit_constant(c: &ConstantModel) -> Option<String> {
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

fn build_constant_map(model: &ApiModel) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for c in &model.constants {
        map.insert(c.name.clone(), shouty_snake_case_name(&c.name));
    }
    map
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

fn fn_signature_params(args: &[RecordMember], model: &ApiModel, receiver: Option<&str>) -> String {
    let mut parts = Vec::new();
    let callback_info_map = build_callback_info_map(model);
    let callback_fn_map = build_callback_function_map(model);
    let length_fields = length_field_names(args);

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

        if length_fields.contains(&arg.name) {
            continue;
        }

        let param_name = safe_ident(&snake_case_name(&arg.name));
        let param_ty = rust_param_type(arg);
        let needs_mut = arg.length.is_some() && arg.annotation.is_mut_ptr();
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
        let _ = index.struct_needs_free_members(&arg.member_type);
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
                Some(format!(
                    r#"{}::{}"#,
                    type_name(ty),
                    enum_variant_name_camel(s)
                ))
            } else if index.is_bitmask(ty) {
                Some(format!(r#"{}::{}"#, type_name(ty), bitmask_variant_name(s)))
            } else if let Some(const_name) = constant_map.get(s) {
                Some(const_name.clone())
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

fn ffi_enum_const_name(ffi_type: &str, variant: &str) -> String {
    format!("{ffi_type}_{ffi_type}_{variant}")
}

fn ffi_bitmask_const_name(ffi_type: &str, variant: &str) -> String {
    format!("{ffi_type}_{variant}")
}

fn ffi_enum_value_name(name: &str) -> String {
    if name.trim().eq_ignore_ascii_case("srgb") {
        return "SRGB".to_string();
    }
    let mut out = String::new();
    for word in name.split_whitespace() {
        let mut parts = Vec::new();
        for part in word.split('-') {
            if part.is_empty() {
                continue;
            }
            let value = if part.chars().all(|c| c.is_ascii_digit()) {
                part.to_string()
            } else if part
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            {
                part.to_string()
            } else {
                let mut chars = part.chars();
                let mut s = String::new();
                if let Some(first) = chars.next() {
                    s.push(first.to_ascii_uppercase());
                    s.push_str(&chars.as_str().to_ascii_lowercase());
                }
                s
            };
            parts.push(value);
        }
        out.push_str(&parts.join("_"));
    }
    for (from, to) in acronym_fixes() {
        out = out.replace(from, to);
    }
    let extra_fixes = [
        ("Opengles", "OpenGLES"),
        ("Opengl", "OpenGL"),
        ("Gpu", "GPU"),
        ("Cpu", "CPU"),
        ("Winui", "WinUI"),
    ];
    for (from, to) in extra_fixes {
        out = out.replace(from, to);
    }
    out
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

fn ffi_type_name(canonical: &str, c_prefix: &str) -> String {
    if canonical == "INTERNAL_HAVE_EMDAWNWEBGPU_HEADER" {
        return format!("{c_prefix}{canonical}");
    }
    format!("{}{}", c_prefix, camel_case_with_acronyms(canonical))
}

fn ffi_fn_name(canonical: &str, c_prefix: &str) -> String {
    format!(
        "{}{}",
        c_prefix.to_lowercase(),
        camel_case_with_acronyms(canonical)
    )
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
        ("Html", "HTML"),
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

fn build_stype_map(model: &ApiModel, c_prefix: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let stype_enum_name = "s type";
    let enum_name = type_name(stype_enum_name);
    if let Some(stype_enum) = model.enums.iter().find(|e| e.name == stype_enum_name) {
        for value in &stype_enum.def.values {
            let variant = enum_variant_name_camel(&value.name);
            map.insert(value.name.clone(), format!(r#"{enum_name}::{variant}"#));
        }
    }
    let _ = c_prefix;
    map
}

fn length_value_expr(length: &LengthValue) -> String {
    match length {
        LengthValue::Number(value) => value.to_string(),
        LengthValue::String(name) => safe_ident(&snake_case_name(name)),
    }
}

fn is_char_string_list(member: &RecordMember) -> bool {
    member.member_type == "char"
        && member.length.is_some()
        && member.annotation.is_const_const_ptr()
}

fn struct_needs_free_members(s: &StructureModel) -> bool {
    let is_out = s.def.extensible.is_output()
        || s.def.chained.as_deref() == Some("out")
        || s.def.out == Some(true);
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
    let length_fields = length_field_names(&s.def.members);

    if s.def.extensible.is_extensible() {
        fields.push("extensions: Vec::new(),".to_string());
    }

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

        fields.push(format!(
            r#"    {field_name}: {value_expr},"#,
            field_name = field_name,
            value_expr = value_expr
        ));
    }

    if index.struct_needs_free_members(&s.name) {
        fields.push("    _free_members: Some(value),".to_string());
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

fn length_field_names(members: &[RecordMember]) -> HashSet<String> {
    let mut fields = HashSet::new();
    for member in members {
        if let Some(LengthValue::String(name)) = &member.length {
            fields.insert(name.clone());
        }
    }
    fields
}

fn length_expr_for_data_arg(arg: &RecordMember, data_name: &str) -> String {
    if arg.optional {
        format!(
            r#"{data}.as_deref().map(|v| v.len()).unwrap_or(0)"#,
            data = data_name
        )
    } else {
        format!(r#"{data}.len()"#, data = data_name)
    }
}

fn emit_struct_to_ffi_body(
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
                body.push(format!(
                    r#"let raw_vec: Vec<ffi::{prefix}{obj}> = values.iter().map(|v| v.as_raw()).collect();"#,
                    prefix = c_prefix,
                    obj = type_name(&member.member_type)
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
                    "    let mode = info.mode.unwrap_or(CallbackMode::AllowSpontaneous);\n"
                } else {
                    ""
                };
                let mode_field = if has_mode {
                    "        mode: mode.into(),\n"
                } else {
                    ""
                };
                let default_mode_field = if has_mode {
                    "        mode: CallbackMode::AllowSpontaneous.into(),\n"
                } else {
                    ""
                };

                lines.push(format!(
                    r#"if let Some(info) = &self.{field} {{
    let mut callback_slot = info.callback.borrow_mut();
    let callback = callback_slot.take();
    let (callback_ptr, userdata1): (ffi::{ffi_callback_ty}, *mut std::ffi::c_void) = if let Some(callback) = callback {{
        let callback_box: {callback_ty} = callback;
        let callback_box = Box::new(Some(callback_box));
        let userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        (Some({trampoline}), userdata)
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
