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
    let send_sync_impl = if requires_external_synchronization(&name) {
        String::new()
    } else {
        format!(
            r#"// Dawn serializes access to API objects internally. Encoder recording objects are
// excluded because their mutable recording state requires external synchronization.
unsafe impl Send for {name} {{}}

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
        let method_docs = doc_comment(method.comment.as_deref());
        if name == "Adapter" && method_name == "create_device" {
            methods.push(emit_adapter_create_device_method(c_prefix));
            continue;
        }
        if name == "Adapter" && method_name == "request_device" {
            methods.push(emit_adapter_request_device_method(c_prefix));
            continue;
        }

        let return_ty = method
            .returns()
            .map(rust_return_type)
            .unwrap_or_else(|| "()".to_string());

        let signature = fn_signature_params(&method.args, model, Some("self"));
        let (arg_prelude, ffi_args, has_callback) =
            emit_ffi_arg_prelude(&method.args, model, index, c_prefix);
        let postlude = emit_out_struct_postlude(&method.args, index);

        let uses_raw_pointer = method
            .args
            .iter()
            .any(|arg| arg.length.is_none() && arg.member_type.contains('*'))
            || method
                .returns()
                .is_some_and(|ret| ret.get_type().contains('*'));
        let safety = if uses_raw_pointer
            || (name == "Buffer"
                && matches!(
                    method_name.as_str(),
                    "get_mapped_range" | "get_const_mapped_range"
                )) {
            "unsafe "
        } else {
            ""
        };
        let safety_docs = if safety.is_empty() {
            String::new()
        } else {
            "    /// # Safety\n    /// All raw pointers must remain valid for the duration required by Dawn.\n"
                .to_string()
        };

        methods.push(format!(
            r#"{method_docs}{safety_docs}    pub {safety}fn {method_name}({signature}) -> {return_ty} {{
{arg_prelude}
{body}
    }}"#,
            method_name = method_name,
            method_docs = if method_docs.is_empty() {
                String::new()
            } else {
                format!("{}\n", indent_block(&method_docs, 4))
            },
            safety_docs = safety_docs,
            safety = safety,
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
    let docs = doc_comment(o.def.comment.as_deref());
    let object_fields = if name == "Device" {
        format!(
            r#"    raw: ffi::{prefix}{name},
    callback_userdata: std::sync::Arc<Vec<CallbackUserdata>>,"#,
            prefix = c_prefix,
            name = name
        )
    } else {
        format!(
            r#"    raw: ffi::{prefix}{name},"#,
            prefix = c_prefix,
            name = name
        )
    };
    let from_raw_body = if name == "Device" {
        r#"Self {
            raw,
            callback_userdata: std::sync::Arc::new(Vec::new()),
        }"#
        .to_string()
    } else {
        "Self { raw }".to_string()
    };
    let device_extra_methods = if name == "Device" {
        r#"
    pub(crate) unsafe fn from_raw_with_callback_userdata(
        raw: ffi::WGPUDevice,
        callback_userdata: Vec<CallbackUserdata>,
    ) -> Self {
        assert!(!raw.is_null(), "Dawn returned a null Device handle");
        Self {
            raw,
            callback_userdata: std::sync::Arc::new(callback_userdata),
        }
    }

    pub(crate) fn with_callback_userdata(mut self, callback_userdata: Vec<CallbackUserdata>) -> Self {
        self.callback_userdata = std::sync::Arc::new(callback_userdata);
        self
    }
"#
        .to_string()
    } else {
        String::new()
    };
    let clone_body = if name == "Device" {
        r#"Self {
            raw: self.raw,
            callback_userdata: self.callback_userdata.clone(),
        }"#
        .to_string()
    } else {
        "Self { raw: self.raw }".to_string()
    };

    format!(
        r#"{docs}#[derive(Debug)]
pub struct {name} {{
{object_fields}
}}

impl {name} {{
    /// Takes ownership of one strong reference returned by the Dawn C API.
    ///
    /// # Safety
    /// `raw` must be non-null and represent an owned reference of the matching object type.
    pub unsafe fn from_raw(raw: crate::sys::{prefix}{name}) -> Self {{
        assert!(!raw.is_null(), "Dawn returned a null {name} handle");
        {from_raw_body}
    }}
{device_extra_methods}

    pub fn as_raw(&self) -> crate::sys::{prefix}{name} {{
        self.raw
    }}

    /// Transfers this wrapper's strong reference to the caller.
    pub fn into_raw(self) -> crate::sys::{prefix}{name} {{
        let this = std::mem::ManuallyDrop::new(self);
        this.raw
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
        {clone_body}
    }}
}}

{send_sync_impl}"#,
        name = name,
        docs = docs,
        methods = methods_block,
        prefix = c_prefix,
        object_fields = object_fields,
        from_raw_body = from_raw_body,
        device_extra_methods = device_extra_methods,
        clone_body = clone_body,
        send_sync_impl = send_sync_impl
    )
}

fn requires_external_synchronization(name: &str) -> bool {
    matches!(
        name,
        "CommandEncoder" | "ComputePassEncoder" | "RenderPassEncoder" | "RenderBundleEncoder"
    )
}

fn emit_adapter_create_device_method(c_prefix: &str) -> String {
    format!(
        r#"    pub fn create_device(&self, descriptor: Option<&DeviceDescriptor>) -> Device {{
        let mut descriptor_storage = ChainedStructStorage::new();
        let mut descriptor_ffi: Option<ffi::{prefix}DeviceDescriptor> = None;
        let descriptor_ptr = if let Some(value) = &descriptor {{
            let (raw, storage) = value.to_ffi();
            descriptor_storage = storage;
            descriptor_ffi = Some(raw);
            if let Some(raw_ref) = descriptor_ffi.as_ref() {{
                std::ptr::from_ref(raw_ref)
            }} else {{
                unreachable!("internal error: descriptor_ffi missing after assignment")
            }}
        }} else {{
            std::ptr::null()
        }};
        let result = unsafe {{ ffi::wgpuAdapterCreateDevice(self.raw, descriptor_ptr) }};
        let callback_userdata = descriptor_storage.take_callback_userdatas();
        unsafe {{ Device::from_raw_with_callback_userdata(result, callback_userdata) }}
    }}"#,
        prefix = c_prefix
    )
}

fn emit_adapter_request_device_method(c_prefix: &str) -> String {
    format!(
        r#"    pub fn request_device(
        &self,
        descriptor: Option<&DeviceDescriptor>,
        callback_mode: CallbackMode,
        callback: impl FnMut(RequestDeviceStatus, Option<Device>, String) + Send + 'static,
    ) -> Future {{
        let mut descriptor_storage = ChainedStructStorage::new();
        let mut descriptor_ffi: Option<ffi::{prefix}DeviceDescriptor> = None;
        let descriptor_ptr = if let Some(value) = &descriptor {{
            let (raw, storage) = value.to_ffi();
            descriptor_storage = storage;
            descriptor_ffi = Some(raw);
            if let Some(raw_ref) = descriptor_ffi.as_ref() {{
                std::ptr::from_ref(raw_ref)
            }} else {{
                unreachable!("internal error: descriptor_ffi missing after assignment")
            }}
        }} else {{
            std::ptr::null()
        }};
        let mut device_callback_userdata = Some(descriptor_storage.take_callback_userdatas());
        let mut callback = callback;
        let callback_box: RequestDeviceCallback = Box::new(move |status, device, message| {{
            let device = device.map(|device| {{
                device.with_callback_userdata(device_callback_userdata.take().unwrap_or_default())
            }});
            callback(status, device, message);
        }});
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::{prefix}RequestDeviceCallbackInfo {{
            nextInChain: std::ptr::null_mut(),
            mode: callback_mode.into(),
            callback: Some(request_device_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        }};
        let result = unsafe {{
            ffi::wgpuAdapterRequestDevice(self.raw, descriptor_ptr, callback_info_ffi)
        }};
        Future::from_ffi(result)
    }}"#,
        prefix = c_prefix
    )
}
