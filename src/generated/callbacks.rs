#![allow(dead_code, unused_imports)]
use crate::ffi;
use super::*;
use std::cell::RefCell;
fn string_view_to_string(view: ffi::WGPUStringView) -> String {
    if view.data.is_null() || view.length == 0 {
        return String::new();
    }
    let data = view.data.cast::<u8>();
    let slice = unsafe { std::slice::from_raw_parts(data, view.length) };
    String::from_utf8_lossy(slice).into_owned()
}
pub type Callback = Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>;
pub type DawnLoadCacheDataFunction = Option<
    unsafe extern "C" fn(
        *const std::ffi::c_void,
        usize,
        *mut std::ffi::c_void,
        usize,
        *mut std::ffi::c_void,
    ) -> usize,
>;
pub type DawnStoreCacheDataFunction = Option<
    unsafe extern "C" fn(
        *const std::ffi::c_void,
        usize,
        *const std::ffi::c_void,
        usize,
        *mut std::ffi::c_void,
    ) -> (),
>;
pub type Proc = Option<unsafe extern "C" fn() -> ()>;
pub type BufferMapCallback = Box<dyn FnMut(MapAsyncStatus, String) + Send + 'static>;
pub(crate) unsafe extern "C" fn buffer_map_callback_trampoline(
    status: ffi::WGPUMapAsyncStatus,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(userdata1.cast::<Option<BufferMapCallback>>());
    if let Some(mut callback) = callback.take() {
        callback(status, message);
    }
}
pub type CompilationInfoCallback = Box<
    dyn FnMut(CompilationInfoRequestStatus, &CompilationInfo) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn compilation_info_callback_trampoline(
    status: ffi::WGPUCompilationInfoRequestStatus,
    compilation_info: *const ffi::WGPUCompilationInfo,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let compilation_info = if compilation_info.is_null() {
        CompilationInfo::new()
    } else {
        CompilationInfo::new()
    };
    let mut callback = Box::from_raw(
        userdata1.cast::<Option<CompilationInfoCallback>>(),
    );
    if let Some(mut callback) = callback.take() {
        callback(status, &compilation_info);
    }
}
pub type CreateComputePipelineAsyncCallback = Box<
    dyn FnMut(
        CreatePipelineAsyncStatus,
        Option<ComputePipeline>,
        String,
    ) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn create_compute_pipeline_async_callback_trampoline(
    status: ffi::WGPUCreatePipelineAsyncStatus,
    pipeline: ffi::WGPUComputePipeline,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let pipeline = if pipeline.is_null() {
        None
    } else {
        Some(unsafe { ComputePipeline::from_raw(pipeline) })
    };
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(
        userdata1.cast::<Option<CreateComputePipelineAsyncCallback>>(),
    );
    if let Some(mut callback) = callback.take() {
        callback(status, pipeline, message);
    }
}
pub type CreateRenderPipelineAsyncCallback = Box<
    dyn FnMut(CreatePipelineAsyncStatus, Option<RenderPipeline>, String) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn create_render_pipeline_async_callback_trampoline(
    status: ffi::WGPUCreatePipelineAsyncStatus,
    pipeline: ffi::WGPURenderPipeline,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let pipeline = if pipeline.is_null() {
        None
    } else {
        Some(unsafe { RenderPipeline::from_raw(pipeline) })
    };
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(
        userdata1.cast::<Option<CreateRenderPipelineAsyncCallback>>(),
    );
    if let Some(mut callback) = callback.take() {
        callback(status, pipeline, message);
    }
}
pub type DeviceLostCallback = Box<
    dyn FnMut(Vec<Device>, DeviceLostReason, String) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn device_lost_callback_trampoline(
    device: *const ffi::WGPUDevice,
    reason: ffi::WGPUDeviceLostReason,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let device = if device.is_null() {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(device, 1) }
            .iter()
            .map(|raw| unsafe { Device::from_raw(*raw) })
            .collect()
    };
    let reason = reason.into();
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(userdata1.cast::<Option<DeviceLostCallback>>());
    if let Some(mut callback) = callback.take() {
        callback(device, reason, message);
    }
}
pub type LoggingCallback = Box<dyn FnMut(LoggingType, String) + Send + 'static>;
pub(crate) unsafe extern "C" fn logging_callback_trampoline(
    r#type: ffi::WGPULoggingType,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let r#type = r#type.into();
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(userdata1.cast::<Option<LoggingCallback>>());
    if let Some(mut callback) = callback.take() {
        callback(r#type, message);
    }
}
pub type PopErrorScopeCallback = Box<
    dyn FnMut(PopErrorScopeStatus, ErrorType, String) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn pop_error_scope_callback_trampoline(
    status: ffi::WGPUPopErrorScopeStatus,
    r#type: ffi::WGPUErrorType,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let r#type = r#type.into();
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(userdata1.cast::<Option<PopErrorScopeCallback>>());
    if let Some(mut callback) = callback.take() {
        callback(status, r#type, message);
    }
}
pub type QueueWorkDoneCallback = Box<
    dyn FnMut(QueueWorkDoneStatus, String) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn queue_work_done_callback_trampoline(
    status: ffi::WGPUQueueWorkDoneStatus,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(userdata1.cast::<Option<QueueWorkDoneCallback>>());
    if let Some(mut callback) = callback.take() {
        callback(status, message);
    }
}
pub type RequestAdapterCallback = Box<
    dyn FnMut(RequestAdapterStatus, Option<Adapter>, String) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn request_adapter_callback_trampoline(
    status: ffi::WGPURequestAdapterStatus,
    adapter: ffi::WGPUAdapter,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let adapter = if adapter.is_null() {
        None
    } else {
        Some(unsafe { Adapter::from_raw(adapter) })
    };
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(userdata1.cast::<Option<RequestAdapterCallback>>());
    if let Some(mut callback) = callback.take() {
        callback(status, adapter, message);
    }
}
pub type RequestDeviceCallback = Box<
    dyn FnMut(RequestDeviceStatus, Option<Device>, String) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn request_device_callback_trampoline(
    status: ffi::WGPURequestDeviceStatus,
    device: ffi::WGPUDevice,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let status = status.into();
    let device = if device.is_null() {
        None
    } else {
        Some(unsafe { Device::from_raw(device) })
    };
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(userdata1.cast::<Option<RequestDeviceCallback>>());
    if let Some(mut callback) = callback.take() {
        callback(status, device, message);
    }
}
pub type UncapturedErrorCallback = Box<
    dyn FnMut(Vec<Device>, ErrorType, String) + Send + 'static,
>;
pub(crate) unsafe extern "C" fn uncaptured_error_callback_trampoline(
    device: *const ffi::WGPUDevice,
    r#type: ffi::WGPUErrorType,
    message: ffi::WGPUStringView,
    userdata1: *mut std::ffi::c_void,
    userdata2: *mut std::ffi::c_void,
) {
    let _ = userdata2;
    let device = if device.is_null() {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(device, 1) }
            .iter()
            .map(|raw| unsafe { Device::from_raw(*raw) })
            .collect()
    };
    let r#type = r#type.into();
    let message = string_view_to_string(message);
    let mut callback = Box::from_raw(
        userdata1.cast::<Option<UncapturedErrorCallback>>(),
    );
    if let Some(mut callback) = callback.take() {
        callback(device, r#type, message);
    }
}
pub struct BufferMapCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<BufferMapCallback>>,
}
impl Default for BufferMapCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl BufferMapCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct CompilationInfoCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<CompilationInfoCallback>>,
}
impl Default for CompilationInfoCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl CompilationInfoCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct CreateComputePipelineAsyncCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<CreateComputePipelineAsyncCallback>>,
}
impl Default for CreateComputePipelineAsyncCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl CreateComputePipelineAsyncCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct CreateRenderPipelineAsyncCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<CreateRenderPipelineAsyncCallback>>,
}
impl Default for CreateRenderPipelineAsyncCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl CreateRenderPipelineAsyncCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct DeviceLostCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<DeviceLostCallback>>,
}
impl Default for DeviceLostCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl DeviceLostCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct LoggingCallbackInfo {
    pub callback: std::cell::RefCell<Option<LoggingCallback>>,
}
impl Default for LoggingCallbackInfo {
    fn default() -> Self {
        Self {
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl LoggingCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct PopErrorScopeCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<PopErrorScopeCallback>>,
}
impl Default for PopErrorScopeCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl PopErrorScopeCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct QueueWorkDoneCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<QueueWorkDoneCallback>>,
}
impl Default for QueueWorkDoneCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl QueueWorkDoneCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct RequestAdapterCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<RequestAdapterCallback>>,
}
impl Default for RequestAdapterCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl RequestAdapterCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct RequestDeviceCallbackInfo {
    pub mode: Option<CallbackMode>,
    pub callback: std::cell::RefCell<Option<RequestDeviceCallback>>,
}
impl Default for RequestDeviceCallbackInfo {
    fn default() -> Self {
        Self {
            mode: None,
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl RequestDeviceCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub struct UncapturedErrorCallbackInfo {
    pub callback: std::cell::RefCell<Option<UncapturedErrorCallback>>,
}
impl Default for UncapturedErrorCallbackInfo {
    fn default() -> Self {
        Self {
            callback: std::cell::RefCell::new(None),
        }
    }
}
impl UncapturedErrorCallbackInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
