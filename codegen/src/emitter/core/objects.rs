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
        let safety = if func
            .def
            .args
            .iter()
            .any(|a| index.input_requires_unsafe(&a.member_type))
        {
            "unsafe "
        } else {
            ""
        };
        let safety_docs = if safety.is_empty() {
            ""
        } else {
            "    /// # Safety\n    /// External pointers and handles in the descriptor must remain valid for all Dawn use.\n"
        };
        methods.push(format!(
            r#"{safety_docs}    pub {safety}fn new({signature}) -> Self {{
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

        if name == "Device" && method_name == "set_logging_callback" {
            methods.push(
                r#"    /// Installs a repeating logging callback, retained through device loss.
    /// Panics for devices adopted from raw handles without callback ownership.
    pub fn set_logging_callback(
        &self,
        callback: impl FnMut(LoggingType, String) + Send + 'static,
    ) {
        let state = self.callback_state.as_ref()
            .expect("logging callbacks require a Device created by dawn-rs");
        let callback: LoggingCallback = Box::new(callback);
        let userdata = Box::into_raw(Box::new(std::sync::Mutex::new(Some(callback))))
            .cast::<std::ffi::c_void>();
        state.retain(CallbackUserdata::new_mutex::<LoggingCallback>(userdata));
        let info = ffi::WGPULoggingCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            callback: Some(logging_callback_trampoline),
            userdata1: userdata,
            userdata2: std::ptr::null_mut(),
        };
        unsafe { ffi::wgpuDeviceSetLoggingCallback(self.raw, info) };
    }"#
                .to_string(),
            );
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

        let uses_raw_pointer = method.args.iter().any(|arg| {
            index.input_requires_unsafe(&arg.member_type)
                || (arg.member_type == "void" && arg.length.is_none() && !arg.annotation.is_value())
        }) || method
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
            "    /// # Safety\n    /// All external pointers and handles, including those in nested descriptors and extensions, must be valid and correctly synchronized for every use by Dawn. Host allocations must remain alive until GPU use ends.\n"
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
    callback_state: Option<std::sync::Arc<crate::device_callbacks::DeviceCallbackState>>,"#,
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
            callback_state: None,
        }"#
        .to_string()
    } else {
        "Self { raw }".to_string()
    };
    let device_extra_methods = if name == "Device" {
        r#"
    pub(crate) fn with_callback_state(
        mut self,
        state: std::sync::Arc<crate::device_callbacks::DeviceCallbackState>,
    ) -> Self {
        self.callback_state = Some(state);
        self
    }

    pub(crate) unsafe fn borrow_for_callback(
        raw: ffi::WGPUDevice,
        userdata2: *mut std::ffi::c_void,
    ) -> Self {
        let state = unsafe { crate::device_callbacks::clone_state(userdata2) };
        unsafe { Self::from_raw(raw) }.with_callback_state(state)
    }

    pub(crate) fn forget_borrowed(self) {
        let this = std::mem::ManuallyDrop::new(self);
        // Only the native handle is borrowed; the callback-state Arc is owned.
        unsafe { drop(std::ptr::read(&this.callback_state)); }
    }

"#
        .to_string()
    } else {
        String::new()
    };
    let clone_body = if name == "Device" {
        r#"Self {
            raw: self.raw,
            callback_state: self.callback_state.clone(),
        }"#
        .to_string()
    } else {
        "Self { raw: self.raw }".to_string()
    };

    let into_raw_cleanup = if name == "Device" {
        "        // The lost-event context retains callback state after ownership transfer.\n        unsafe { drop(std::ptr::read(&this.callback_state)); }\n"
    } else {
        ""
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
{into_raw_cleanup}        this.raw
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
        r#"    /// # Safety
    /// External pointers and handles in descriptor extensions must remain valid for all Dawn use.
    pub unsafe fn create_device(&self, descriptor: Option<&DeviceDescriptor>) -> Device {{
        let default_descriptor = DeviceDescriptor::new();
        let (mut descriptor_ffi, mut storage) = descriptor.unwrap_or(&default_descriptor).to_ffi();
        let state = crate::device_callbacks::prepare(&mut descriptor_ffi, storage.take_callback_userdatas());
        let result = unsafe {{ ffi::{prefix_lower}AdapterCreateDevice(self.raw, &descriptor_ffi) }};
        unsafe {{ Device::from_raw(result) }}.with_callback_state(state)
    }}"#,
        prefix_lower = c_prefix.to_lowercase(),
    )
}

fn emit_adapter_request_device_method(c_prefix: &str) -> String {
    format!(
        r#"    /// # Safety
    /// External pointers and handles in descriptor extensions must remain valid for all Dawn use.
    pub unsafe fn request_device(
        &self,
        descriptor: Option<&DeviceDescriptor>,
        callback_mode: CallbackMode,
        callback: impl FnMut(RequestDeviceStatus, Option<Device>, String) + Send + 'static,
    ) -> Future {{
        let default_descriptor = DeviceDescriptor::new();
        let (mut descriptor_ffi, mut storage) = descriptor.unwrap_or(&default_descriptor).to_ffi();
        let state = crate::device_callbacks::prepare(&mut descriptor_ffi, storage.take_callback_userdatas());
        let mut callback = callback;
        let callback_box: RequestDeviceCallback = Box::new(move |status, device, message| {{
            let device = device.map(|device| device.with_callback_state(state.clone()));
            callback(status, device, message);
        }});
        let callback_userdata = Box::into_raw(Box::new(Some(callback_box))).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::{prefix}RequestDeviceCallbackInfo {{
            nextInChain: std::ptr::null_mut(),
            mode: callback_mode.into(),
            callback: Some(request_device_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        }};
        let result = unsafe {{
            ffi::{prefix_lower}AdapterRequestDevice(self.raw, &descriptor_ffi, callback_info_ffi)
        }};
        Future::from_ffi(result)
    }}"#,
        prefix = c_prefix,
        prefix_lower = c_prefix.to_lowercase(),
    )
}
