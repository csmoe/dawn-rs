use crate::api_model::{ApiModel, FunctionModel, ObjectModel};
use crate::emitter::core::*;

pub(crate) fn emit_object(
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
        "_not_sync: std::marker::PhantomData<std::cell::Cell<()>>,"
    } else {
        ""
    };
    let marker_init = if no_autolock {
        ", _not_sync: std::marker::PhantomData"
    } else {
        ""
    };
    let send_sync_impl = if no_autolock {
        format!(r#"unsafe impl Send for {name} {{}}"#)
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
