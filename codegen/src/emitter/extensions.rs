use crate::api_model::ApiModel;
use crate::api_model::StructureModel;
use std::collections::HashMap;

use crate::emitter::core::{build_stype_map, enum_variant_name_camel, type_name};

pub(crate) fn emit(model: &ApiModel, c_prefix: &str) -> String {
    emit_extensions(model, c_prefix)
}

pub(crate) fn emit_extensions(model: &ApiModel, c_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        r#"#![allow(dead_code, unused_imports)]

use crate::ffi;
use crate::generated::*;
use std::any::Any;
use std::fmt;

pub(crate) struct CallbackUserdata {{
    ptr: *mut std::ffi::c_void,
    drop_fn: unsafe fn(*mut std::ffi::c_void),
}}

impl CallbackUserdata {{
    pub(crate) fn new<T: Send + 'static>(ptr: *mut std::ffi::c_void) -> Self {{
        unsafe fn drop_box<T>(ptr: *mut std::ffi::c_void) {{
            if !ptr.is_null() {{
                unsafe {{ drop(Box::from_raw(ptr.cast::<Option<T>>())); }}
            }}
        }}

        Self {{
            ptr,
            drop_fn: drop_box::<T>,
        }}
    }}

    pub(crate) fn new_mutex<T: Send + 'static>(ptr: *mut std::ffi::c_void) -> Self {{
        unsafe fn drop_box<T>(ptr: *mut std::ffi::c_void) {{
            if !ptr.is_null() {{
                unsafe {{
                    drop(Box::from_raw(
                        ptr.cast::<std::sync::Mutex<Option<T>>>(),
                    ));
                }}
            }}
        }}

        Self {{
            ptr,
            drop_fn: drop_box::<T>,
        }}
    }}
}}

impl Drop for CallbackUserdata {{
    fn drop(&mut self) {{
        unsafe {{ (self.drop_fn)(self.ptr) }};
    }}
}}

impl fmt::Debug for CallbackUserdata {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        f.debug_struct("CallbackUserdata")
            .field("ptr", &self.ptr)
            .finish_non_exhaustive()
    }}
}}

unsafe impl Send for CallbackUserdata {{}}
unsafe impl Sync for CallbackUserdata {{}}

pub(crate) struct ChainedStructStorage {{
    entries: Vec<Box<ffi::{prefix}ChainedStruct>>,
    buffers: Vec<Box<dyn Any>>,
    nested: Vec<ChainedStructStorage>,
    callback_userdata: Vec<CallbackUserdata>,
}}

impl ChainedStructStorage {{
    pub(crate) fn new() -> Self {{
        Self {{
            entries: Vec::new(),
            buffers: Vec::new(),
            nested: Vec::new(),
            callback_userdata: Vec::new(),
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

    pub(crate) fn push_callback_userdata(&mut self, userdata: CallbackUserdata) {{
        self.callback_userdata.push(userdata);
    }}

    pub(crate) fn take_callback_userdatas(&mut self) -> Vec<CallbackUserdata> {{
        let mut userdatas = std::mem::take(&mut self.callback_userdata);
        for nested in &mut self.nested {{
            userdatas.extend(nested.take_callback_userdatas());
        }}
        userdatas
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
            let from_chain_arms = variants
                .iter()
                .map(|s| {
                    let ty = type_name(&s.name);
                    let stype_const = stype_map.get(&s.name).cloned().unwrap_or_else(|| {
                        format!(
                            r#"{}::{}"#,
                            type_name("s type"),
                            enum_variant_name_camel(&s.name)
                        )
                    });
                    format!(
                        r#"                {stype_const} => Some({enum_name}::{ty}(
                    {ty}::from_ffi(unsafe {{ *current.cast::<ffi::{prefix}{ty}>() }}),
                )),"#,
                        prefix = c_prefix,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            let from_chain_borrowed_arms = variants
                .iter()
                .map(|s| {
                    let ty = type_name(&s.name);
                    let stype_const = stype_map.get(&s.name).cloned().unwrap_or_else(|| {
                        format!(
                            r#"{}::{}"#,
                            type_name("s type"),
                            enum_variant_name_camel(&s.name)
                        )
                    });
                    format!(
                        r#"                {stype_const} => Some({enum_name}::{ty}(
                    {ty}::from_ffi_borrowed(unsafe {{ *current.cast::<ffi::{prefix}{ty}>() }}),
                )),"#,
                        prefix = c_prefix,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
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

    pub(crate) unsafe fn from_chain(
        current: *mut ffi::{prefix}ChainedStruct,
    ) -> Vec<Self> {{
        unsafe {{ Self::from_chain_with(current, true) }}
    }}

    pub(crate) unsafe fn from_chain_borrowed(
        current: *mut ffi::{prefix}ChainedStruct,
    ) -> Vec<Self> {{
        unsafe {{ Self::from_chain_with(current, false) }}
    }}

    unsafe fn from_chain_with(
        mut current: *mut ffi::{prefix}ChainedStruct,
        take_ownership: bool,
    ) -> Vec<Self> {{
        let mut extensions = Vec::new();
        while !current.is_null() {{
            let chain = unsafe {{ &*current }};
            let next = chain.next;
            let s_type = SType::from(chain.sType);
            let extension = if take_ownership {{
                match s_type {{
{from_chain_arms}
                    _ => None,
                }}
            }} else {{
                match s_type {{
{from_chain_borrowed_arms}
                    _ => None,
                }}
            }};
            if let Some(extension) = extension {{
                extensions.push(extension);
            }}
            current = next;
        }}
        extensions
    }}
}}

"#,
                enum_name = enum_name,
                prefix = c_prefix,
                from_chain_arms = from_chain_arms,
                from_chain_borrowed_arms = from_chain_borrowed_arms,
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
                raw.chain.sType = {stype_const}.into();
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

    pub(crate) unsafe fn from_chain(
        _current: *mut ffi::{prefix}ChainedStruct,
    ) -> Vec<Self> {{
        Vec::new()
    }}

    pub(crate) unsafe fn from_chain_borrowed(
        _current: *mut ffi::{prefix}ChainedStruct,
    ) -> Vec<Self> {{
        Vec::new()
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
