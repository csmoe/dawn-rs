//! Keeps callback userdata alive until Dawn delivers the device-lost event.
//!
//! Device release only makes that event ready: AllowProcessEvents and WaitAnyOnly
//! may defer delivery. Dawn stops error/logging callbacks before delivering it.

use crate::generated::CallbackUserdata;
use crate::sys;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub(crate) struct DeviceCallbackState {
    // Keep replaced logging callbacks too: a concurrent invocation may still use
    // the old userdata. No Rust lock is held while calling Dawn or user code.
    userdata: Mutex<Vec<CallbackUserdata>>,
}

impl DeviceCallbackState {
    pub(crate) fn retain(&self, userdata: CallbackUserdata) {
        self.userdata
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(userdata);
    }
}

struct LostContext {
    state: Arc<DeviceCallbackState>,
    original: sys::WGPUDeviceLostCallbackInfo,
}

pub(crate) fn prepare(
    descriptor: &mut sys::WGPUDeviceDescriptor,
    userdata: Vec<CallbackUserdata>,
) -> Arc<DeviceCallbackState> {
    let state = Arc::new(DeviceCallbackState {
        userdata: Mutex::new(userdata),
    });
    let state_ptr = Arc::as_ptr(&state).cast_mut().cast::<c_void>();
    let mut original = descriptor.deviceLostCallbackInfo;
    original.userdata2 = state_ptr;
    let context = Box::new(LostContext {
        state: state.clone(),
        original,
    });
    descriptor.uncapturedErrorCallbackInfo.userdata2 = state_ptr;
    descriptor.deviceLostCallbackInfo.callback = Some(device_lost);
    descriptor.deviceLostCallbackInfo.userdata1 = Box::into_raw(context).cast();
    descriptor.deviceLostCallbackInfo.userdata2 = std::ptr::null_mut();
    // With no user callback there is no reason to defer cleanup.
    if original.callback.is_none() {
        descriptor.deviceLostCallbackInfo.mode = crate::CallbackMode::AllowSpontaneous.into();
    }
    state
}

pub(crate) unsafe fn clone_state(userdata: *mut c_void) -> Arc<DeviceCallbackState> {
    let state = userdata.cast::<DeviceCallbackState>();
    // prepare installs this borrowed pointer; the lost context owns a strong ref
    // until Dawn has finished all error/logging callbacks and the lost callback.
    unsafe {
        Arc::increment_strong_count(state);
        Arc::from_raw(state)
    }
}

unsafe extern "C" fn device_lost(
    device: *const sys::WGPUDevice,
    reason: sys::WGPUDeviceLostReason,
    message: sys::WGPUStringView,
    userdata1: *mut c_void,
    _userdata2: *mut c_void,
) {
    // Unlike logging/error callbacks, the device-lost event fires exactly once,
    // including failed creation and instance shutdown cancellation.
    let context = unsafe { Box::from_raw(userdata1.cast::<LostContext>()) };
    if let Some(callback) = context.original.callback {
        unsafe {
            callback(
                device,
                reason,
                message,
                context.original.userdata1,
                context.original.userdata2,
            );
        }
    }
    drop(context.state);
}
