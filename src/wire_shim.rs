use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DawnRsWireHandle {
    pub id: u32,
    pub generation: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DawnRsWireReservedInstance {
    pub instance: *mut c_void,
    pub handle: DawnRsWireHandle,
}

#[repr(C)]
struct DawnRsWireSerializerCallbacks {
    userdata: *mut c_void,
    on_flush: Option<extern "C" fn(*mut c_void, *const u8, usize)>,
    on_error: Option<extern "C" fn(*mut c_void, *const u8, usize)>,
    max_allocation_size: usize,
}

#[repr(C)]
struct DawnRsWireClientOpaque {
    _private: [u8; 0],
}

#[repr(C)]
struct DawnRsWireServerOpaque {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn dawn_rs_wire_client_create(
        callbacks: *const DawnRsWireSerializerCallbacks,
    ) -> *mut DawnRsWireClientOpaque;
    fn dawn_rs_wire_client_destroy(client: *mut DawnRsWireClientOpaque);
    fn dawn_rs_wire_client_handle_commands(
        client: *mut DawnRsWireClientOpaque,
        data: *const u8,
        size: usize,
    ) -> bool;
    fn dawn_rs_wire_client_flush(client: *mut DawnRsWireClientOpaque) -> bool;
    fn dawn_rs_wire_client_disconnect(client: *mut DawnRsWireClientOpaque);
    fn dawn_rs_wire_client_reserve_instance(
        client: *mut DawnRsWireClientOpaque,
    ) -> DawnRsWireReservedInstance;

    fn dawn_rs_wire_server_create_native(
        callbacks: *const DawnRsWireSerializerCallbacks,
        use_spontaneous_callbacks: bool,
    ) -> *mut DawnRsWireServerOpaque;
    fn dawn_rs_wire_server_destroy(server: *mut DawnRsWireServerOpaque);
    fn dawn_rs_wire_server_handle_commands(
        server: *mut DawnRsWireServerOpaque,
        data: *const u8,
        size: usize,
    ) -> bool;
    fn dawn_rs_wire_server_flush(server: *mut DawnRsWireServerOpaque) -> bool;
    fn dawn_rs_wire_server_inject_instance(
        server: *mut DawnRsWireServerOpaque,
        instance: *mut c_void,
        handle: DawnRsWireHandle,
    ) -> bool;
    fn dawn_rs_wire_set_client_procs();
    fn dawn_rs_wire_set_native_procs();
    fn dawn_rs_wire_clear_procs();
}

struct CallbackState {
    on_flush: Box<dyn FnMut(&[u8]) + Send + 'static>,
    on_error: Box<dyn FnMut(&str) + Send + 'static>,
}

extern "C" fn on_flush_trampoline(userdata: *mut c_void, data: *const u8, size: usize) {
    if userdata.is_null() || data.is_null() {
        return;
    }
    let state = unsafe { &mut *(userdata as *mut CallbackState) };
    let bytes = unsafe { std::slice::from_raw_parts(data, size) };
    (state.on_flush)(bytes);
}

extern "C" fn on_error_trampoline(userdata: *mut c_void, data: *const u8, size: usize) {
    if userdata.is_null() || data.is_null() {
        return;
    }
    let state = unsafe { &mut *(userdata as *mut CallbackState) };
    let bytes = unsafe { std::slice::from_raw_parts(data, size) };
    if let Ok(msg) = std::str::from_utf8(bytes) {
        (state.on_error)(msg);
    }
}

pub struct WireClientShim {
    raw: *mut DawnRsWireClientOpaque,
    state: *mut CallbackState,
}

unsafe impl Send for WireClientShim {}

impl WireClientShim {
    pub fn new<F, E>(max_allocation_size: usize, on_flush: F, on_error: E) -> Result<Self, String>
    where
        F: FnMut(&[u8]) + Send + 'static,
        E: FnMut(&str) + Send + 'static,
    {
        let state = Box::new(CallbackState {
            on_flush: Box::new(on_flush),
            on_error: Box::new(on_error),
        });
        let state_ptr = Box::into_raw(state);
        let callbacks = DawnRsWireSerializerCallbacks {
            userdata: state_ptr.cast(),
            on_flush: Some(on_flush_trampoline),
            on_error: Some(on_error_trampoline),
            max_allocation_size,
        };
        let raw = unsafe { dawn_rs_wire_client_create(&callbacks) };
        if raw.is_null() {
            unsafe {
                drop(Box::from_raw(state_ptr));
            }
            return Err("failed to create Dawn wire client shim".to_string());
        }
        Ok(Self {
            raw,
            state: state_ptr,
        })
    }

    pub fn handle_commands(&mut self, data: &[u8]) -> bool {
        unsafe { dawn_rs_wire_client_handle_commands(self.raw, data.as_ptr(), data.len()) }
    }

    pub fn flush(&mut self) -> bool {
        unsafe { dawn_rs_wire_client_flush(self.raw) }
    }

    pub fn disconnect(&mut self) {
        unsafe { dawn_rs_wire_client_disconnect(self.raw) }
    }

    pub fn reserve_instance(&mut self) -> DawnRsWireReservedInstance {
        unsafe { dawn_rs_wire_client_reserve_instance(self.raw) }
    }

    pub unsafe fn reserved_instance_to_instance(
        reserved: DawnRsWireReservedInstance,
    ) -> crate::Instance {
        unsafe { crate::Instance::from_raw(reserved.instance.cast()) }
    }
}

impl Drop for WireClientShim {
    fn drop(&mut self) {
        unsafe {
            dawn_rs_wire_client_destroy(self.raw);
            drop(Box::from_raw(self.state));
        }
    }
}

pub struct WireServerShim {
    raw: *mut DawnRsWireServerOpaque,
    state: *mut CallbackState,
}

unsafe impl Send for WireServerShim {}

impl WireServerShim {
    pub fn new_native<F, E>(
        max_allocation_size: usize,
        use_spontaneous_callbacks: bool,
        on_flush: F,
        on_error: E,
    ) -> Result<Self, String>
    where
        F: FnMut(&[u8]) + Send + 'static,
        E: FnMut(&str) + Send + 'static,
    {
        let state = Box::new(CallbackState {
            on_flush: Box::new(on_flush),
            on_error: Box::new(on_error),
        });
        let state_ptr = Box::into_raw(state);
        let callbacks = DawnRsWireSerializerCallbacks {
            userdata: state_ptr.cast(),
            on_flush: Some(on_flush_trampoline),
            on_error: Some(on_error_trampoline),
            max_allocation_size,
        };
        let raw =
            unsafe { dawn_rs_wire_server_create_native(&callbacks, use_spontaneous_callbacks) };
        if raw.is_null() {
            unsafe {
                drop(Box::from_raw(state_ptr));
            }
            return Err("failed to create Dawn wire server shim".to_string());
        }
        Ok(Self {
            raw,
            state: state_ptr,
        })
    }

    pub fn handle_commands(&mut self, data: &[u8]) -> bool {
        unsafe { dawn_rs_wire_server_handle_commands(self.raw, data.as_ptr(), data.len()) }
    }

    pub fn flush(&mut self) -> bool {
        unsafe { dawn_rs_wire_server_flush(self.raw) }
    }

    pub fn inject_instance(&mut self, instance: *mut c_void, handle: DawnRsWireHandle) -> bool {
        unsafe { dawn_rs_wire_server_inject_instance(self.raw, instance, handle) }
    }
}

impl Drop for WireServerShim {
    fn drop(&mut self) {
        unsafe {
            dawn_rs_wire_server_destroy(self.raw);
            drop(Box::from_raw(self.state));
        }
    }
}

pub fn set_client_procs() {
    unsafe { dawn_rs_wire_set_client_procs() }
}

pub fn set_native_procs() {
    unsafe { dawn_rs_wire_set_native_procs() }
}

pub fn clear_procs() {
    unsafe { dawn_rs_wire_clear_procs() }
}
