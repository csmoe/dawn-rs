use crate::api_model::{ApiModel, BitmaskModel, EnumModel};

use crate::emitter::core::{
    bitmask_variant_name, enum_variant_name_camel, ffi_bitmask_const_name, ffi_enum_const_name,
    ffi_enum_value_name, ffi_type_name, type_name,
};

pub(crate) fn emit(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use bitflags::bitflags;

"#,
    );

    for e in &model.enums {
        out.push_str(&emit_enum(e, c_prefix));
    }
    for b in &model.bitmasks {
        out.push_str(&emit_bitmask(b, c_prefix));
    }

    out
}

pub(crate) fn emit_enum(e: &EnumModel, c_prefix: &str) -> String {
    let name = type_name(&e.name);
    let mut variants = Vec::new();
    let mut from_arms = Vec::new();
    let ffi_type = ffi_type_name(&e.name, c_prefix);
    let mut first_variant: Option<String> = None;

    for v in &e.def.values {
        let variant = enum_variant_name_camel(&v.name);
        let const_variant = ffi_enum_value_name(&v.name);
        let const_name = ffi_enum_const_name(&ffi_type, &const_variant);
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

pub(crate) fn emit_bitmask(b: &BitmaskModel, c_prefix: &str) -> String {
    let name = type_name(&b.name);
    let ffi_type = ffi_type_name(&b.name, c_prefix);
    let mut variants = Vec::new();

    for v in &b.def.values {
        let variant = bitmask_variant_name(&v.name);
        let const_variant = ffi_enum_value_name(&v.name);
        let const_name = ffi_bitmask_const_name(&ffi_type, &const_variant);
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
