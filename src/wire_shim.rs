use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

// Rust-side wire shim intentionally mirrors Dawn's WireHelper pattern:
// - Dawn C++ wire runtime performs command serialization/deserialization.
// - Rust only forwards opaque wire bytes over transport and triggers HandleCommands.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WireInstanceHandle {
    pub id: u32,
    pub generation: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReservedWireInstance {
    pub instance: *mut c_void,
    pub handle: WireInstanceHandle,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReservedWireSurface {
    pub surface: *mut c_void,
    pub instance_handle: WireInstanceHandle,
    pub handle: WireInstanceHandle,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReservedWireTexture {
    pub texture: *mut c_void,
    pub handle: WireInstanceHandle,
    pub device_handle: WireInstanceHandle,
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
    ) -> ReservedWireInstance;
    fn dawn_rs_wire_client_reserve_surface(
        client: *mut DawnRsWireClientOpaque,
        instance: *mut c_void,
    ) -> ReservedWireSurface;
    fn dawn_rs_wire_client_get_device_handle(
        client: *mut DawnRsWireClientOpaque,
        device: *mut c_void,
    ) -> WireInstanceHandle;
    fn dawn_rs_wire_client_reserve_bgra8_texture_2d(
        client: *mut DawnRsWireClientOpaque,
        device: *mut c_void,
        width: u32,
        height: u32,
    ) -> ReservedWireTexture;

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
        handle: WireInstanceHandle,
    ) -> bool;
    fn dawn_rs_wire_server_inject_surface(
        server: *mut DawnRsWireServerOpaque,
        surface: *mut c_void,
        handle: WireInstanceHandle,
        instance_handle: WireInstanceHandle,
    ) -> bool;
    fn dawn_rs_wire_server_inject_texture(
        server: *mut DawnRsWireServerOpaque,
        texture: *mut c_void,
        handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool;
    fn dawn_rs_wire_server_inject_buffer(
        server: *mut DawnRsWireServerOpaque,
        buffer: *mut c_void,
        handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool;
    fn dawn_rs_wire_server_get_device(
        server: *mut DawnRsWireServerOpaque,
        handle: WireInstanceHandle,
    ) -> *mut c_void;
    fn dawn_rs_wire_server_inject_iosurface_texture(
        server: *mut DawnRsWireServerOpaque,
        io_surface: *mut c_void,
        width: u32,
        height: u32,
        texture_handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool;
    fn dawn_rs_wire_server_inject_dxgi_texture(
        server: *mut DawnRsWireServerOpaque,
        shared_handle: *mut c_void,
        use_keyed_mutex: bool,
        width: u32,
        height: u32,
        texture_handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool;
    fn dawn_rs_wire_server_inject_dmabuf_texture(
        server: *mut DawnRsWireServerOpaque,
        fd: i32,
        drm_format: u32,
        drm_modifier: u64,
        stride: u32,
        offset: u64,
        width: u32,
        height: u32,
        texture_handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool;
    fn dawn_rs_wire_set_client_procs();
    fn dawn_rs_wire_set_native_procs();
    fn dawn_rs_wire_clear_procs();
}

struct CallbackState {
    closed: AtomicBool,
    on_flush: Mutex<Box<dyn FnMut(&[u8]) + Send + 'static>>,
    on_error: Mutex<Box<dyn FnMut(&str) + Send + 'static>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProcTableMode {
    WireClient,
    Native,
}

#[derive(Debug)]
struct ProcTableLease {
    mode: ProcTableMode,
}

#[derive(Debug, Default)]
struct ProcTableState {
    mode: Option<ProcTableMode>,
    refs: usize,
}

fn proc_table_state() -> &'static Mutex<ProcTableState> {
    static STATE: OnceLock<Mutex<ProcTableState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(ProcTableState::default()))
}

fn set_client_procs() {
    unsafe { dawn_rs_wire_set_client_procs() }
}

fn set_native_procs() {
    unsafe { dawn_rs_wire_set_native_procs() }
}

fn clear_procs() {
    unsafe { dawn_rs_wire_clear_procs() }
}

impl ProcTableLease {
    fn acquire(mode: ProcTableMode) -> Result<Self, String> {
        let mut state = proc_table_state()
            .lock()
            .map_err(|_| "wire proc table lock poisoned".to_string())?;
        match state.mode {
            None => {
                match mode {
                    ProcTableMode::WireClient => set_client_procs(),
                    ProcTableMode::Native => set_native_procs(),
                }
                state.mode = Some(mode);
                state.refs = 1;
                Ok(Self { mode })
            }
            Some(current) if current == mode => {
                state.refs += 1;
                Ok(Self { mode })
            }
            Some(current) => Err(format!(
                "wire proc table mode conflict: active={current:?}, requested={mode:?}"
            )),
        }
    }
}

impl Drop for ProcTableLease {
    fn drop(&mut self) {
        let Ok(mut state) = proc_table_state().lock() else {
            return;
        };
        if state.mode != Some(self.mode) || state.refs == 0 {
            return;
        }
        state.refs -= 1;
        if state.refs == 0 {
            clear_procs();
            state.mode = None;
        }
    }
}

extern "C" fn on_flush_trampoline(userdata: *mut c_void, data: *const u8, size: usize) {
    if userdata.is_null() {
        return;
    }
    if size == 0 || data.is_null() || size > (64 * 1024 * 1024) {
        return;
    }
    let state = unsafe { &*(userdata as *const CallbackState) };
    if state.closed.load(Ordering::Relaxed) {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(data, size) };
    if let Ok(mut cb) = state.on_flush.lock() {
        (cb)(bytes);
    }
}

extern "C" fn on_error_trampoline(userdata: *mut c_void, data: *const u8, size: usize) {
    if userdata.is_null() {
        return;
    }
    if size == 0 || data.is_null() || size > (64 * 1024) {
        return;
    }
    let state = unsafe { &*(userdata as *const CallbackState) };
    if state.closed.load(Ordering::Relaxed) {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(data, size) };
    if let Ok(msg) = std::str::from_utf8(bytes) {
        if let Ok(mut cb) = state.on_error.lock() {
            (cb)(msg);
        }
    }
}

pub struct WireHelperClient {
    raw: *mut DawnRsWireClientOpaque,
    state: *mut CallbackState,
    _proc_table: ProcTableLease,
}

unsafe impl Send for WireHelperClient {}

impl WireHelperClient {
    pub fn new<F, E>(max_allocation_size: usize, on_flush: F, on_error: E) -> Result<Self, String>
    where
        F: FnMut(&[u8]) + Send + 'static,
        E: FnMut(&str) + Send + 'static,
    {
        let proc_table = ProcTableLease::acquire(ProcTableMode::WireClient)?;
        let state = Box::new(CallbackState {
            closed: AtomicBool::new(false),
            on_flush: Mutex::new(Box::new(on_flush)),
            on_error: Mutex::new(Box::new(on_error)),
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
            _proc_table: proc_table,
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

    pub fn reserve_instance(&mut self) -> ReservedWireInstance {
        unsafe { dawn_rs_wire_client_reserve_instance(self.raw) }
    }

    pub fn reserve_surface(&mut self, instance: *mut c_void) -> ReservedWireSurface {
        unsafe { dawn_rs_wire_client_reserve_surface(self.raw, instance) }
    }

    pub fn get_device_handle(&mut self, device: *mut c_void) -> WireInstanceHandle {
        unsafe { dawn_rs_wire_client_get_device_handle(self.raw, device) }
    }

    pub fn reserve_bgra8_texture_2d(
        &mut self,
        device: *mut c_void,
        width: u32,
        height: u32,
    ) -> ReservedWireTexture {
        unsafe { dawn_rs_wire_client_reserve_bgra8_texture_2d(self.raw, device, width, height) }
    }

    pub unsafe fn reserved_instance_to_instance(reserved: ReservedWireInstance) -> crate::Instance {
        unsafe { crate::Instance::from_raw(reserved.instance.cast()) }
    }

    pub unsafe fn reserved_surface_to_surface(reserved: ReservedWireSurface) -> crate::Surface {
        unsafe { crate::Surface::from_raw(reserved.surface.cast()) }
    }

    pub unsafe fn reserved_texture_to_texture(reserved: ReservedWireTexture) -> crate::Texture {
        unsafe { crate::Texture::from_raw(reserved.texture.cast()) }
    }
}

impl Drop for WireHelperClient {
    fn drop(&mut self) {
        unsafe {
            (*self.state).closed.store(true, Ordering::Relaxed);
            dawn_rs_wire_client_destroy(self.raw);
            drop(Box::from_raw(self.state));
        }
    }
}

pub struct WireHelperServer {
    raw: *mut DawnRsWireServerOpaque,
    state: *mut CallbackState,
    _proc_table: ProcTableLease,
}

unsafe impl Send for WireHelperServer {}

impl WireHelperServer {
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
        let proc_table = ProcTableLease::acquire(ProcTableMode::Native)?;
        let state = Box::new(CallbackState {
            closed: AtomicBool::new(false),
            on_flush: Mutex::new(Box::new(on_flush)),
            on_error: Mutex::new(Box::new(on_error)),
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
            _proc_table: proc_table,
        })
    }

    pub fn handle_commands(&mut self, data: &[u8]) -> bool {
        unsafe { dawn_rs_wire_server_handle_commands(self.raw, data.as_ptr(), data.len()) }
    }

    pub fn flush(&mut self) -> bool {
        unsafe { dawn_rs_wire_server_flush(self.raw) }
    }

    pub fn inject_instance(&mut self, instance: *mut c_void, handle: WireInstanceHandle) -> bool {
        unsafe { dawn_rs_wire_server_inject_instance(self.raw, instance, handle) }
    }

    pub fn inject_surface(
        &mut self,
        surface: *mut c_void,
        handle: WireInstanceHandle,
        instance_handle: WireInstanceHandle,
    ) -> bool {
        unsafe { dawn_rs_wire_server_inject_surface(self.raw, surface, handle, instance_handle) }
    }

    pub fn inject_texture(
        &mut self,
        texture: *mut c_void,
        handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool {
        unsafe { dawn_rs_wire_server_inject_texture(self.raw, texture, handle, device_handle) }
    }

    pub fn inject_buffer(
        &mut self,
        buffer: *mut c_void,
        handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool {
        unsafe { dawn_rs_wire_server_inject_buffer(self.raw, buffer, handle, device_handle) }
    }

    pub fn get_device(&mut self, handle: WireInstanceHandle) -> *mut c_void {
        unsafe { dawn_rs_wire_server_get_device(self.raw, handle) }
    }

    pub fn inject_iosurface_texture(
        &mut self,
        io_surface: *mut c_void,
        width: u32,
        height: u32,
        texture_handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool {
        unsafe {
            dawn_rs_wire_server_inject_iosurface_texture(
                self.raw,
                io_surface,
                width,
                height,
                texture_handle,
                device_handle,
            )
        }
    }

    pub fn inject_dxgi_texture(
        &mut self,
        shared_handle: *mut c_void,
        use_keyed_mutex: bool,
        width: u32,
        height: u32,
        texture_handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool {
        unsafe {
            dawn_rs_wire_server_inject_dxgi_texture(
                self.raw,
                shared_handle,
                use_keyed_mutex,
                width,
                height,
                texture_handle,
                device_handle,
            )
        }
    }

    pub fn inject_dmabuf_texture(
        &mut self,
        fd: i32,
        drm_format: u32,
        drm_modifier: u64,
        stride: u32,
        offset: u64,
        width: u32,
        height: u32,
        texture_handle: WireInstanceHandle,
        device_handle: WireInstanceHandle,
    ) -> bool {
        unsafe {
            dawn_rs_wire_server_inject_dmabuf_texture(
                self.raw,
                fd,
                drm_format,
                drm_modifier,
                stride,
                offset,
                width,
                height,
                texture_handle,
                device_handle,
            )
        }
    }
}

impl Drop for WireHelperServer {
    fn drop(&mut self) {
        unsafe {
            (*self.state).closed.store(true, Ordering::Relaxed);
            dawn_rs_wire_server_destroy(self.raw);
            drop(Box::from_raw(self.state));
        }
    }
}
