//! Chromium-style wire demo (modern desktop path):
//! - `browser` process orchestrates session and owns top-level window semantics.
//! - `gpu` process runs Dawn wire server and executes GPU API work from the wire client.
//! - Cross-process render targets are shared via platform-native handles
//!   (IOSurface / DXGI handle / DMA-BUF-X11 path) and synchronized through wire control flow.
//! - OS compositor still performs final composition to screen.

use dawn_rs::wire::{Server as WireServer, ServerOptions as WireServerOptions};
#[cfg(target_os = "macos")]
use dawn_rs::wire::WireTextureReservation;
use dawn_rs::{
    BackendType, BufferUsage, Color, ColorTargetState, FragmentState, Instance, LoadOp,
    MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    RequestAdapterStatus, ShaderModuleDescriptor, ShaderSourceWGSL, TextureFormat,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};
use dawn_wgpu::wire_backend::{IpcWireBackend, WireInitOptions};
#[cfg(target_os = "macos")]
use interprocess::local_socket::traits::Stream as _;
#[cfg(target_os = "macos")]
use interprocess::local_socket::{
    GenericFilePath, GenericNamespaced, ListenerOptions, Name, Stream as IpcStream, prelude::*,
};
#[cfg(target_os = "macos")]
use mach2::bootstrap::{bootstrap_look_up, bootstrap_port, bootstrap_register};
#[cfg(target_os = "macos")]
use mach2::kern_return::KERN_SUCCESS;
#[cfg(target_os = "macos")]
use mach2::mach_port::{
    mach_port_allocate, mach_port_deallocate, mach_port_destroy, mach_port_insert_right,
};
#[cfg(target_os = "macos")]
use mach2::message::{
    MACH_MSG_PORT_DESCRIPTOR, MACH_MSG_TIMEOUT_NONE, MACH_MSG_TYPE_COPY_SEND,
    MACH_MSG_TYPE_MAKE_SEND, MACH_MSG_TYPE_MOVE_SEND, MACH_MSGH_BITS, MACH_MSGH_BITS_COMPLEX,
    MACH_RCV_MSG, MACH_SEND_MSG, mach_msg, mach_msg_body_t, mach_msg_header_t,
    mach_msg_port_descriptor_t,
};
#[cfg(target_os = "macos")]
use mach2::port::{MACH_PORT_NULL, MACH_PORT_RIGHT_RECEIVE, mach_port_t};
#[cfg(target_os = "macos")]
use mach2::traps::mach_task_self;
#[cfg(target_os = "macos")]
use objc2::{
    class, msg_send,
    runtime::{AnyClass, AnyObject},
    sel,
};
#[cfg(target_os = "macos")]
use objc2_core_foundation::{
    CFDictionary, CFMutableDictionary, CFNumber, CFRetained, CFString, CFType,
};
#[cfg(target_os = "macos")]
use objc2_io_surface::{
    IOSurfaceRef, kIOSurfaceAllocSize, kIOSurfaceBytesPerElement, kIOSurfaceBytesPerRow,
    kIOSurfaceHeight, kIOSurfacePixelFormat, kIOSurfaceWidth,
};
use std::env;
use std::error::Error;
#[cfg(target_os = "macos")]
use std::ffi::CString;
#[cfg(target_os = "macos")]
use std::io::{Read, Write};
#[cfg(all(unix, not(target_os = "macos")))]
use std::ffi::CString;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(any(target_os = "windows", target_os = "macos", all(unix, not(target_os = "macos"))))]
use std::sync::mpsc;
#[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
use std::thread::JoinHandle;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
#[cfg(all(unix, not(target_os = "macos")))]
use winit::raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
#[cfg(target_os = "macos")]
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
#[cfg(any(target_os = "windows", target_os = "macos", all(unix, not(target_os = "macos"))))]
use winit::window::{Window, WindowAttributes};
use winit::window::WindowId;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;
const MAX_VERTEX_BYTES: u64 = 256 * 1024;
#[cfg(target_os = "macos")]
const CTRL_MSG_RESERVE_TEXTURE: u8 = 1;
#[cfg(target_os = "macos")]
const CTRL_MSG_RESERVE_TEXTURE_ACK: u8 = 2;

#[cfg(target_os = "macos")]
#[derive(Clone, Copy)]
struct SharedWireTextureSpec {
    reservation: WireTextureReservation,
}

#[cfg(target_os = "macos")]
fn control_to_ipc_name(name: &str) -> std::io::Result<Name<'static>> {
    if GenericNamespaced::is_supported() {
        name.to_string()
            .to_ns_name::<GenericNamespaced>()
            .map_err(std::io::Error::other)
    } else {
        format!("/tmp/{name}.sock")
            .to_fs_name::<GenericFilePath>()
            .map_err(std::io::Error::other)
    }
}

#[cfg(target_os = "macos")]
fn control_connect_with_retry(
    name: &str,
    attempts: usize,
    delay: Duration,
) -> std::io::Result<IpcStream> {
    let ipc_name = control_to_ipc_name(name)?;
    let mut last_err: Option<std::io::Error> = None;
    for _ in 0..attempts.max(1) {
        match IpcStream::connect(ipc_name.clone()) {
            Ok(stream) => return Ok(stream),
            Err(e) => {
                last_err = Some(e);
                thread::sleep(delay);
            }
        }
    }
    Err(last_err.unwrap_or_else(|| std::io::Error::other("failed to connect control socket")))
}

#[cfg(target_os = "macos")]
fn control_bind_and_accept_with_stop(
    name: &str,
    stop: Option<&Arc<AtomicBool>>,
) -> std::io::Result<IpcStream> {
    use interprocess::local_socket::traits::Listener as _;
    use interprocess::local_socket::ListenerNonblockingMode;
    let listener = ListenerOptions::new()
        .name(control_to_ipc_name(name)?)
        .reclaim_name(true)
        .create_sync()?;
    listener.set_nonblocking(ListenerNonblockingMode::Accept)?;
    loop {
        if let Some(flag) = stop
            && flag.load(Ordering::Relaxed)
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "stopped while waiting control accept",
            ));
        }
        match listener.accept() {
            Ok(stream) => return Ok(stream),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(target_os = "macos")]
fn write_shared_texture_spec(
    stream: &mut IpcStream,
    spec: SharedWireTextureSpec,
) -> std::io::Result<()> {
    let mut payload = Vec::with_capacity(40);
    payload.extend_from_slice(&spec.reservation.texture_handle().id().to_le_bytes());
    payload.extend_from_slice(&spec.reservation.texture_handle().generation().to_le_bytes());
    payload.extend_from_slice(&spec.reservation.device_handle().id().to_le_bytes());
    payload.extend_from_slice(&spec.reservation.device_handle().generation().to_le_bytes());
    payload.extend_from_slice(&spec.reservation.width().to_le_bytes());
    payload.extend_from_slice(&spec.reservation.height().to_le_bytes());
    payload.extend_from_slice(&spec.reservation.format_raw().to_le_bytes());
    payload.extend_from_slice(&spec.reservation.usage_bits().to_le_bytes());
    stream.write_all(&[CTRL_MSG_RESERVE_TEXTURE])?;
    stream.write_all(&(payload.len() as u32).to_le_bytes())?;
    stream.write_all(&payload)?;
    stream.flush()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn read_shared_texture_spec(stream: &mut IpcStream) -> std::io::Result<Option<SharedWireTextureSpec>> {
    let mut tag = [0u8; 1];
    match stream.read_exact(&mut tag) {
        Ok(()) => {}
        Err(e)
            if e.kind() == std::io::ErrorKind::TimedOut
                || e.kind() == std::io::ErrorKind::WouldBlock =>
        {
            return Ok(None);
        }
        Err(e) => return Err(e),
    }
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;
    let len = u32::from_le_bytes(len_bytes) as usize;
    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload)?;
    if tag[0] != CTRL_MSG_RESERVE_TEXTURE || len != 36 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid control reserve texture message",
        ));
    }
    let mut at = 0usize;
    let read_u32 = |buf: &[u8], at: &mut usize| -> u32 {
        let v = u32::from_le_bytes([buf[*at], buf[*at + 1], buf[*at + 2], buf[*at + 3]]);
        *at += 4;
        v
    };
    let texture_id = read_u32(&payload, &mut at);
    let texture_generation = read_u32(&payload, &mut at);
    let device_id = read_u32(&payload, &mut at);
    let device_generation = read_u32(&payload, &mut at);
    let width = read_u32(&payload, &mut at);
    let height = read_u32(&payload, &mut at);
    let format = read_u32(&payload, &mut at);
    let usage = u64::from_le_bytes([
        payload[at],
        payload[at + 1],
        payload[at + 2],
        payload[at + 3],
        payload[at + 4],
        payload[at + 5],
        payload[at + 6],
        payload[at + 7],
    ]);
    let reservation = WireTextureReservation::new(
        dawn_rs::wire::WireHandle::new(texture_id, texture_generation),
        dawn_rs::wire::WireHandle::new(device_id, device_generation),
        width,
        height,
        format,
        usage,
    );
    Ok(Some(SharedWireTextureSpec { reservation }))
}

#[cfg(target_os = "macos")]
fn write_control_ack(stream: &mut IpcStream, ok: bool) -> std::io::Result<()> {
    stream.write_all(&[CTRL_MSG_RESERVE_TEXTURE_ACK])?;
    stream.write_all(&1u32.to_le_bytes())?;
    stream.write_all(&[if ok { 1 } else { 0 }])?;
    stream.flush()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn read_control_ack(stream: &mut IpcStream) -> std::io::Result<bool> {
    let mut tag = [0u8; 1];
    stream.read_exact(&mut tag)?;
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;
    let len = u32::from_le_bytes(len_bytes) as usize;
    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload)?;
    if tag[0] != CTRL_MSG_RESERVE_TEXTURE_ACK || len != 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid control ack message",
        ));
    }
    Ok(payload[0] != 0)
}

#[cfg(target_os = "macos")]
fn read_control_ack_with_retry(
    stream: &mut IpcStream,
    attempts: usize,
    delay: Duration,
) -> std::io::Result<bool> {
    let mut last_err: Option<std::io::Error> = None;
    for _ in 0..attempts.max(1) {
        match read_control_ack(stream) {
            Ok(v) => return Ok(v),
            Err(e)
                if e.kind() == std::io::ErrorKind::TimedOut
                    || e.kind() == std::io::ErrorKind::WouldBlock =>
            {
                last_err = Some(e);
                thread::sleep(delay);
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_err.unwrap_or_else(|| std::io::Error::other("control ack timeout")))
}

#[cfg(target_os = "macos")]
fn create_iosurface(width: u32, height: u32) -> Option<CFRetained<IOSurfaceRef>> {
    const PIXEL_FORMAT_BGRA: i32 = 0x4247_5241;
    let width_cf = CFNumber::new_i32(width as i32);
    let height_cf = CFNumber::new_i32(height as i32);
    let bpe_cf = CFNumber::new_i32(4);
    let bpr_cf = CFNumber::new_i32((width.saturating_mul(4)) as i32);
    let alloc_cf = CFNumber::new_i32((width.saturating_mul(height).saturating_mul(4)) as i32);
    let fmt_cf = CFNumber::new_i32(PIXEL_FORMAT_BGRA);
    let props = CFMutableDictionary::<CFString, CFType>::with_capacity(6);
    unsafe {
        props.set(kIOSurfaceWidth, width_cf.as_ref());
        props.set(kIOSurfaceHeight, height_cf.as_ref());
        props.set(kIOSurfaceBytesPerElement, bpe_cf.as_ref());
        props.set(kIOSurfaceBytesPerRow, bpr_cf.as_ref());
        props.set(kIOSurfaceAllocSize, alloc_cf.as_ref());
        props.set(kIOSurfacePixelFormat, fmt_cf.as_ref());
    }
    let props_ref = unsafe {
        &*(props.as_ref() as *const CFDictionary<CFString, CFType> as *const CFDictionary)
    };
    unsafe { IOSurfaceRef::new(props_ref) }
}
static NYA_SHADER: &str = r#"
struct VsOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec2<f32>, @location(1) color: vec3<f32>) -> VsOut {
  var out: VsOut;
  out.pos = vec4<f32>(pos, 0.0, 1.0);
  out.color = color;
  return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color, 1.0);
}
"#;


#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MachPortSendMsg {
    header: mach_msg_header_t,
    body: mach_msg_body_t,
    port: mach_msg_port_descriptor_t,
    payload: u32,
}

#[cfg(target_os = "macos")]
fn mach_send_iosurface_port(
    service_name: &str,
    io_surface: &IOSurfaceRef,
) -> Result<(), Box<dyn Error>> {
    let mut service_port: mach_port_t = MACH_PORT_NULL;
    let service_cstr = CString::new(service_name)?;
    let kr = unsafe {
        bootstrap_look_up(
            bootstrap_port,
            service_cstr.as_ptr(),
            &mut service_port as *mut mach_port_t,
        )
    };
    if kr != KERN_SUCCESS || service_port == MACH_PORT_NULL {
        return Err(format!("bootstrap_look_up failed for {service_name}: kr={kr}").into());
    }

    let iosurface_port = io_surface.create_mach_port();
    if iosurface_port == MACH_PORT_NULL {
        return Err("IOSurface create_mach_port returned null".into());
    }
    let mut msg = MachPortSendMsg::default();
    msg.header.msgh_bits = MACH_MSGH_BITS_COMPLEX | MACH_MSGH_BITS(MACH_MSG_TYPE_COPY_SEND, 0);
    msg.header.msgh_size = std::mem::size_of::<MachPortSendMsg>() as u32;
    msg.header.msgh_remote_port = service_port;
    msg.header.msgh_local_port = MACH_PORT_NULL;
    msg.header.msgh_id = 0x1978;
    msg.body.msgh_descriptor_count = 1;
    msg.port.name = iosurface_port;
    msg.port.disposition = MACH_MSG_TYPE_MOVE_SEND as u8;
    msg.port.type_ = MACH_MSG_PORT_DESCRIPTOR as u8;
    msg.payload = io_surface.id();

    let mr = unsafe {
        mach_msg(
            &mut msg.header,
            MACH_SEND_MSG,
            msg.header.msgh_size,
            0,
            MACH_PORT_NULL,
            MACH_MSG_TIMEOUT_NONE,
            MACH_PORT_NULL,
        )
    };
    unsafe {
        let _ = mach_port_deallocate(mach_task_self(), service_port);
    }
    if mr != KERN_SUCCESS {
        return Err(format!("mach_msg send failed: mr={mr}").into());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn mach_send_iosurface_port_with_retry(
    service_name: &str,
    io_surface: &IOSurfaceRef,
    attempts: usize,
    delay: Duration,
) -> Result<(), Box<dyn Error>> {
    let mut last_err: Option<String> = None;
    for _ in 0..attempts.max(1) {
        match mach_send_iosurface_port(service_name, io_surface) {
            Ok(()) => return Ok(()),
            Err(err) => {
                last_err = Some(err.to_string());
                thread::sleep(delay);
            }
        }
    }
    Err(last_err
        .unwrap_or_else(|| "mach_send_iosurface_port failed".to_string())
        .into())
}

#[cfg(target_os = "macos")]
fn mach_receive_iosurface_port(
    service_name: &str,
) -> Result<CFRetained<IOSurfaceRef>, Box<dyn Error>> {
    let mut recv_port: mach_port_t = MACH_PORT_NULL;
    let kr_alloc = unsafe {
        mach_port_allocate(
            mach_task_self(),
            MACH_PORT_RIGHT_RECEIVE,
            &mut recv_port as *mut mach_port_t,
        )
    };
    if kr_alloc != KERN_SUCCESS || recv_port == MACH_PORT_NULL {
        return Err(format!("mach_port_allocate failed: kr={kr_alloc}").into());
    }
    let kr_insert = unsafe {
        mach_port_insert_right(
            mach_task_self(),
            recv_port,
            recv_port,
            MACH_MSG_TYPE_MAKE_SEND,
        )
    };
    if kr_insert != KERN_SUCCESS {
        unsafe {
            let _ = mach_port_destroy(mach_task_self(), recv_port);
        }
        return Err(format!("mach_port_insert_right failed: kr={kr_insert}").into());
    }
    let c_name = CString::new(service_name)?;
    let kr_register =
        unsafe { bootstrap_register(bootstrap_port, c_name.as_ptr() as *mut i8, recv_port) };
    if kr_register != KERN_SUCCESS {
        unsafe {
            let _ = mach_port_destroy(mach_task_self(), recv_port);
        }
        return Err(
            format!("bootstrap_register failed for {service_name}: kr={kr_register}").into(),
        );
    }

    let mut recv_buf = [0u8; 1024];
    let header_ptr = recv_buf.as_mut_ptr().cast::<mach_msg_header_t>();
    unsafe {
        (*header_ptr).msgh_local_port = recv_port;
        (*header_ptr).msgh_size = recv_buf.len() as u32;
    }
    let mr = unsafe {
        mach_msg(
            header_ptr,
            MACH_RCV_MSG,
            0,
            recv_buf.len() as u32,
            recv_port,
            MACH_MSG_TIMEOUT_NONE,
            MACH_PORT_NULL,
        )
    };
    unsafe {
        let _ = mach_port_destroy(mach_task_self(), recv_port);
    }
    if mr != KERN_SUCCESS {
        return Err(format!("mach_msg recv failed: mr={mr}").into());
    }
    let recv_msg = unsafe { &*(recv_buf.as_ptr().cast::<MachPortSendMsg>()) };
    let recv_surface_port = recv_msg.port.name;
    if recv_surface_port == MACH_PORT_NULL {
        return Err("received null iosurface mach port".into());
    }
    let surface = IOSurfaceRef::lookup_from_mach_port(recv_surface_port)
        .ok_or("IOSurface lookup_from_mach_port failed")?;
    unsafe {
        let _ = mach_port_deallocate(mach_task_self(), recv_surface_port);
    }
    Ok(surface)
}

fn glyph_5x7(ch: char) -> [u8; 7] {
    match ch {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0F],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x1F],
        'J' => [0x1F, 0x02, 0x02, 0x02, 0x12, 0x12, 0x0C],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
        ':' => [0x00, 0x0C, 0x0C, 0x00, 0x0C, 0x0C, 0x00],
        '-' => [0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00],
        '1' => [0x04, 0x0C, 0x14, 0x04, 0x04, 0x04, 0x1F],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
#[link(name = "X11")]
unsafe extern "C" {
    #[link_name = "XDisplayString"]
    fn x11_display_string(display: *mut std::ffi::c_void) -> *const std::os::raw::c_char;
}

#[cfg(target_os = "windows")]
struct BrowserWindowHost {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

#[cfg(all(unix, not(target_os = "macos")))]
struct BrowserWindowHost {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

#[cfg(target_os = "windows")]
impl BrowserWindowHost {
    fn spawn() -> Result<Self, Box<dyn Error>> {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = stop.clone();
        let (tx, rx) = mpsc::channel::<Result<(), String>>();
        let thread = thread::spawn(move || {
            let event_loop = match EventLoop::new() {
                Ok(loop_) => loop_,
                Err(err) => {
                    let _ = tx.send(Err(format!("create event loop failed: {err}")));
                    return;
                }
            };
            let mut app = MainHostAppWin32::new(tx, stop_thread);
            let _ = event_loop.run_app(&mut app);
        });
        rx
            .recv_timeout(Duration::from_secs(5))
            .map_err(|e| format!("wait browser window host failed: {e}"))??;
        Ok(Self {
            stop,
            thread: Some(thread),
        })
    }
}

#[cfg(target_os = "windows")]
impl Drop for BrowserWindowHost {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(target_os = "windows")]
struct MainHostAppWin32 {
    tx: Option<mpsc::Sender<Result<(), String>>>,
    stop: Arc<AtomicBool>,
    window: Option<Window>,
}

#[cfg(target_os = "windows")]
impl MainHostAppWin32 {
    fn new(tx: mpsc::Sender<Result<(), String>>, stop: Arc<AtomicBool>) -> Self {
        Self {
            tx: Some(tx),
            stop,
            window: None,
        }
    }
}

#[cfg(target_os = "windows")]
impl ApplicationHandler for MainHostAppWin32 {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let window = match event_loop.create_window(
            WindowAttributes::default()
                .with_title("Nya Cat Main Process")
                .with_inner_size(winit::dpi::PhysicalSize::new(WIDTH, HEIGHT)),
        ) {
            Ok(w) => w,
            Err(err) => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Err(format!("create host window failed: {err}")));
                }
                event_loop.exit();
                return;
            }
        };
        let window_handle = match window.window_handle() {
            Ok(h) => h,
            Err(_) => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Err("host window handle unavailable".to_string()));
                }
                event_loop.exit();
                return;
            }
        };
        match window_handle.as_raw() {
            RawWindowHandle::Win32(handle) => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Ok(()));
                }
            }
            _ => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Err("host expected Win32 handle".to_string()));
                }
                event_loop.exit();
                return;
            }
        }
        self.window = Some(window);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(16),
        ));
        if self.stop.load(Ordering::Relaxed) {
            event_loop.exit();
        }
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if window.id() != id {
            return;
        }
        if let WindowEvent::CloseRequested = event {
            self.stop.store(true, Ordering::Relaxed);
            event_loop.exit();
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl BrowserWindowHost {
    fn spawn() -> Result<Self, Box<dyn Error>> {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = stop.clone();
        let (tx, rx) = mpsc::channel::<Result<(), String>>();
        let thread = thread::spawn(move || {
            let event_loop = match EventLoop::new() {
                Ok(loop_) => loop_,
                Err(err) => {
                    let _ = tx.send(Err(format!("create event loop failed: {err}")));
                    return;
                }
            };
            let mut app = MainHostAppLinux::new(tx, stop_thread);
            let _ = event_loop.run_app(&mut app);
        });
        rx
            .recv_timeout(Duration::from_secs(5))
            .map_err(|e| format!("wait browser window host failed: {e}"))??;
        Ok(Self {
            stop,
            thread: Some(thread),
        })
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl Drop for BrowserWindowHost {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
struct MainHostAppLinux {
    tx: Option<mpsc::Sender<Result<(), String>>>,
    stop: Arc<AtomicBool>,
    window: Option<Window>,
}

#[cfg(all(unix, not(target_os = "macos")))]
impl MainHostAppLinux {
    fn new(tx: mpsc::Sender<Result<(), String>>, stop: Arc<AtomicBool>) -> Self {
        Self {
            tx: Some(tx),
            stop,
            window: None,
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl ApplicationHandler for MainHostAppLinux {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let window = match event_loop.create_window(
            WindowAttributes::default()
                .with_title("Nya Cat Main Process")
                .with_inner_size(winit::dpi::PhysicalSize::new(WIDTH, HEIGHT)),
        ) {
            Ok(w) => w,
            Err(err) => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Err(format!("create host window failed: {err}")));
                }
                event_loop.exit();
                return;
            }
        };
        let window_handle = match window.window_handle() {
            Ok(h) => h,
            Err(_) => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Err("host window handle unavailable".to_string()));
                }
                event_loop.exit();
                return;
            }
        };
        let display_handle = match window.display_handle() {
            Ok(h) => h,
            Err(_) => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Err("host display handle unavailable".to_string()));
                }
                event_loop.exit();
                return;
            }
        };
        match (display_handle.as_raw(), window_handle.as_raw()) {
            (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(win)) => {
                let name_ptr = match display.display {
                    Some(p) => unsafe { x11_display_string(p.as_ptr().cast()) },
                    None => std::ptr::null(),
                };
                if name_ptr.is_null() {
                    if let Some(tx) = self.tx.take() {
                        let _ = tx.send(Err("XDisplayString returned null".to_string()));
                    }
                    event_loop.exit();
                    return;
                }
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Ok(()));
                }
            }
            _ => {
                if let Some(tx) = self.tx.take() {
                    let _ = tx.send(Err(
                        "host only supports X11 for external surface path".to_string(),
                    ));
                }
                event_loop.exit();
                return;
            }
        }
        self.window = Some(window);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(16),
        ));
        if self.stop.load(Ordering::Relaxed) {
            event_loop.exit();
        }
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if window.id() != id {
            return;
        }
        if let WindowEvent::CloseRequested = event {
            self.stop.store(true, Ordering::Relaxed);
            event_loop.exit();
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("nya-cat-wire error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    if let Some(mode) = args.next()
        && mode == "--gpu-process"
    {
        let socket_name = args.next().ok_or("missing socket name")?;
        let parent_pid = args.next().and_then(|v| v.parse::<u32>().ok());
        #[cfg(target_os = "macos")]
        let mut iosurface_port_service: Option<String> = None;
        #[cfg(target_os = "macos")]
        let mut control_socket_name: Option<String> = None;
        while let Some(arg) = args.next() {
            #[cfg(target_os = "macos")]
            if arg == "--iosurface-port-service" {
                iosurface_port_service = args.next();
            }
            #[cfg(target_os = "macos")]
            if arg == "--control-socket" {
                control_socket_name = args.next();
            }
        }
        return run_gpu_process(
            &socket_name,
            parent_pid,
            #[cfg(target_os = "macos")]
            iosurface_port_service,
            #[cfg(target_os = "macos")]
            control_socket_name,
        );
    }
    run_browser_process()
}

fn run_browser_process_loop(
    stop: Option<Arc<AtomicBool>>,
    #[cfg(target_os = "macos")] iosurface_notify: Option<mpsc::Sender<mach_port_t>>,
) -> Result<(), Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let reconnect_delay = Duration::from_millis(400);
    let mut session_id: u64 = 0;
    loop {
        if let Some(stop_flag) = stop.as_ref()
            && stop_flag.load(Ordering::Relaxed)
        {
            return Ok(());
        }
        let socket_name = format!("dawn-wgpu-rainbow-wire-{stamp}-{session_id}");
        session_id = session_id.saturating_add(1);
        match run_browser_session(
            &socket_name,
            stop.as_ref().cloned(),
            #[cfg(target_os = "macos")]
            iosurface_notify.clone(),
        ) {
            Ok(()) => {
                eprintln!("browser: gpu process disconnected, restarting session");
                thread::sleep(reconnect_delay);
            }
            Err(err) => {
                eprintln!("browser: session failed: {err}");
                thread::sleep(reconnect_delay);
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn run_browser_process() -> Result<(), Box<dyn Error>> {
    run_browser_process_loop(
        None,
        #[cfg(target_os = "macos")]
        None,
    )
}

#[cfg(target_os = "macos")]
fn run_browser_process() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "macos")]
    fn make_display_layer() -> Result<*mut AnyObject, Box<dyn Error>> {
        unsafe {
            let layer_class: &AnyClass = AnyClass::get(c"CAIOSurfaceLayer")
                .unwrap_or(class!(CALayer));
            let layer: *mut AnyObject = msg_send![layer_class, layer];
            if layer.is_null() {
                return Err("failed to create display layer".into());
            }
            let _: () = msg_send![layer, setOpaque: true];
            let _: () = msg_send![layer, setNeedsDisplayOnBoundsChange: true];
            Ok(layer)
        }
    }

    #[cfg(target_os = "macos")]
    fn attach_iosurface_to_window(
        window: &Window,
        display_layer: *mut AnyObject,
        surface: &IOSurfaceRef,
    ) -> Result<(), Box<dyn Error>> {
        let raw = window.window_handle()?.as_raw();
        let ns_view: *mut AnyObject = match raw {
            RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast::<AnyObject>(),
            _ => return Err("non-AppKit window handle on macOS".into()),
        };
        unsafe {
            let _: () = msg_send![ns_view, setWantsLayer: true];
            let _: () = msg_send![ns_view, setLayer: display_layer];
            let _: () = msg_send![class!(CATransaction), begin];
            let _: () = msg_send![class!(CATransaction), setDisableActions: true];
            let content_ptr = (surface as *const IOSurfaceRef)
                .cast_mut()
                .cast::<AnyObject>();
            let _: () = msg_send![display_layer, setContents: content_ptr];
            let _: () = msg_send![display_layer, setContentsScale: window.scale_factor()];
            if msg_send![display_layer, respondsToSelector: sel!(setContentsChanged)] {
                let _: () = msg_send![display_layer, setContentsChanged];
            }
            let _: () = msg_send![class!(CATransaction), commit];
            let _: () = msg_send![ns_view, setNeedsDisplay: true];
        }
        Ok(())
    }

    struct BrowserShellApp {
        stop: Arc<AtomicBool>,
        window: Option<Window>,
        display_layer: *mut AnyObject,
        iosurface_rx: mpsc::Receiver<mach_port_t>,
        current_surface: Option<CFRetained<IOSurfaceRef>>,
    }
    impl ApplicationHandler for BrowserShellApp {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            if self.window.is_some() {
                return;
            }
            if let Ok(window) = event_loop.create_window(
                WindowAttributes::default()
                    .with_title("Nya Cat Browser Process")
                    .with_inner_size(winit::dpi::PhysicalSize::new(WIDTH, HEIGHT)),
            ) {
                if let Some(surface) = self.current_surface.as_ref()
                    && let Err(err) =
                        attach_iosurface_to_window(&window, self.display_layer, surface.as_ref())
                {
                    eprintln!("browser: attach IOSurface on resume failed: {err}");
                }
                self.window = Some(window);
            } else {
                event_loop.exit();
            }
        }
        fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(16),
            ));
            if self.stop.load(Ordering::Relaxed) {
                event_loop.exit();
                return;
            }
            while let Ok(port) = self.iosurface_rx.try_recv() {
                if let Some(surface) = IOSurfaceRef::lookup_from_mach_port(port) {
                    let id = surface.id();
                    if let Some(window) = self.window.as_ref() {
                        if let Err(err) =
                            attach_iosurface_to_window(window, self.display_layer, surface.as_ref())
                        {
                            eprintln!("browser: attach IOSurface to window failed: {err}");
                        } else {
                            eprintln!("browser: attached IOSurface id={id} to window");
                        }
                    }
                    self.current_surface = Some(surface);
                    eprintln!("browser: received shared IOSurface id={id} on main thread");
                }
                unsafe {
                    let _ = mach_port_deallocate(mach_task_self(), port);
                }
            }
            if let (Some(window), Some(surface)) = (self.window.as_ref(), self.current_surface.as_ref()) {
                if let Err(err) =
                    attach_iosurface_to_window(window, self.display_layer, surface.as_ref())
                {
                    eprintln!("browser: periodic attach failed: {err}");
                }
            }
            if let Some(window) = self.window.as_ref() {
                window.request_redraw();
            }
        }
        fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
            if let Some(window) = self.window.as_ref() {
                if window.id() != id {
                    return;
                }
            }
            if let WindowEvent::CloseRequested = event {
                self.stop.store(true, Ordering::Relaxed);
                event_loop.exit();
                return;
            }
            match event {
                WindowEvent::Resized(_) | WindowEvent::RedrawRequested => {
                    if let (Some(window), Some(surface)) =
                        (self.window.as_ref(), self.current_surface.as_ref())
                    {
                        if let Err(err) =
                            attach_iosurface_to_window(window, self.display_layer, surface.as_ref())
                        {
                            eprintln!("browser: attach on window event failed: {err}");
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let stop = Arc::new(AtomicBool::new(false));
    let (ios_tx, ios_rx) = mpsc::channel::<mach_port_t>();
    let worker_stop = stop.clone();
    let worker = thread::spawn(move || {
        if let Err(err) = run_browser_process_loop(
            Some(worker_stop),
            Some(ios_tx),
        ) {
            eprintln!("browser worker error: {err}");
        }
    });

    let event_loop = EventLoop::new()?;
    let display_layer = make_display_layer()?;
    let mut app = BrowserShellApp {
        stop: stop.clone(),
        window: None,
        display_layer,
        iosurface_rx: ios_rx,
        current_surface: None,
    };
    event_loop.run_app(&mut app)?;
    stop.store(true, Ordering::Relaxed);
    let _ = worker.join();
    Ok(())
}

fn create_nya_shader(device: &dawn_rs::Device) -> dawn_rs::ShaderModule {
    let mut desc = ShaderModuleDescriptor::new();
    desc = desc.with_extension(
        ShaderSourceWGSL {
            code: Some(NYA_SHADER.to_string()),
        }
        .into(),
    );
    device.create_shader_module(&desc)
}

fn create_nya_pipeline(
    device: &dawn_rs::Device,
    shader: dawn_rs::ShaderModule,
    format: TextureFormat,
) -> RenderPipeline {
    let mut vertex = VertexState::new();
    vertex.module = Some(shader.clone());
    vertex.entry_point = Some("vs_main".to_string());
    let mut pos_attr = VertexAttribute::new();
    pos_attr.format = Some(VertexFormat::Float32X2);
    pos_attr.offset = Some(0);
    pos_attr.shader_location = Some(0);
    let mut color_attr = VertexAttribute::new();
    color_attr.format = Some(VertexFormat::Float32X3);
    color_attr.offset = Some(8);
    color_attr.shader_location = Some(1);
    let mut vb = VertexBufferLayout::new();
    vb.array_stride = Some(20);
    vb.step_mode = Some(VertexStepMode::Vertex);
    vb.attributes = Some(vec![pos_attr, color_attr]);
    vertex.buffers = Some(vec![vb]);

    let mut fragment = FragmentState::new();
    fragment.module = Some(shader);
    fragment.entry_point = Some("fs_main".to_string());

    let mut color_target = ColorTargetState::new();
    color_target.format = Some(format);
    fragment.targets = Some(vec![color_target]);

    let mut desc = RenderPipelineDescriptor::new();
    desc.layout = Some(device.create_pipeline_layout(&PipelineLayoutDescriptor::new()));
    desc.vertex = Some(vertex);
    desc.fragment = Some(fragment);
    desc.primitive = Some(PrimitiveState::new());
    desc.multisample = Some(MultisampleState::new());
    device.create_render_pipeline(&desc)
}

fn push_rect(v: &mut Vec<f32>, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 3]) {
    let [r, g, b] = color;
    v.extend_from_slice(&[x0, y0, r, g, b, x1, y0, r, g, b, x1, y1, r, g, b]);
    v.extend_from_slice(&[x0, y0, r, g, b, x1, y1, r, g, b, x0, y1, r, g, b]);
}

fn build_nya_vertices(phase: f32) -> Vec<f32> {
    let mut v = Vec::with_capacity(6000);

    push_rect(&mut v, -1.0, -1.0, 1.0, 1.0, [1.0, 0.67, 0.03]);

    let cat_x = 0.22 + 0.12 * (phase * 1.4).sin();
    let bob = 0.07 * (phase * 5.0).sin();
    let tail_wag = 0.10 * (phase * 8.0).sin();
    let trail_end = cat_x - 0.20;
    let rainbow = [
        [0.93, 0.10, 0.10],
        [1.00, 0.55, 0.08],
        [0.98, 0.92, 0.10],
        [0.10, 0.85, 0.30],
        [0.10, 0.45, 0.95],
        [0.45, 0.00, 0.85],
    ];
    for (i, c) in rainbow.iter().enumerate() {
        let y0 = bob - 0.35 + i as f32 * 0.08;
        let y1 = y0 + 0.08;
        let x_wave = 0.10 * (phase * 8.0 + i as f32).sin();
        push_rect(&mut v, -1.0, y0, trail_end + x_wave, y1, *c);
    }

    push_rect(
        &mut v,
        cat_x - 0.30 + tail_wag,
        bob - 0.05,
        cat_x - 0.18 + tail_wag,
        bob + 0.08,
        [0.42, 0.44, 0.46],
    );
    push_rect(
        &mut v,
        cat_x - 0.29 + tail_wag,
        bob - 0.04,
        cat_x - 0.19 + tail_wag,
        bob + 0.07,
        [0.10, 0.10, 0.12],
    );

    push_rect(
        &mut v,
        cat_x - 0.18,
        bob - 0.18,
        cat_x + 0.14,
        bob + 0.16,
        [0.10, 0.10, 0.12],
    );
    push_rect(
        &mut v,
        cat_x - 0.16,
        bob - 0.16,
        cat_x + 0.12,
        bob + 0.14,
        [0.95, 0.70, 0.84],
    );

    let sprinkles = [
        (-0.12, 0.08),
        (-0.06, 0.00),
        (0.02, 0.10),
        (0.07, -0.02),
        (0.00, -0.08),
    ];
    for (sx, sy) in sprinkles {
        push_rect(
            &mut v,
            cat_x + sx,
            bob + sy,
            cat_x + sx + 0.02,
            bob + sy + 0.02,
            [0.90, 0.20, 0.50],
        );
    }

    push_rect(
        &mut v,
        cat_x + 0.10,
        bob - 0.12,
        cat_x + 0.36,
        bob + 0.14,
        [0.42, 0.44, 0.46],
    );
    push_rect(
        &mut v,
        cat_x + 0.13,
        bob + 0.10,
        cat_x + 0.18,
        bob + 0.20,
        [0.42, 0.44, 0.46],
    );
    push_rect(
        &mut v,
        cat_x + 0.29,
        bob + 0.10,
        cat_x + 0.34,
        bob + 0.20,
        [0.42, 0.44, 0.46],
    );
    push_rect(
        &mut v,
        cat_x + 0.16,
        bob + 0.02,
        cat_x + 0.20,
        bob + 0.06,
        [1.0, 1.0, 1.0],
    );
    push_rect(
        &mut v,
        cat_x + 0.26,
        bob + 0.02,
        cat_x + 0.30,
        bob + 0.06,
        [1.0, 1.0, 1.0],
    );
    push_rect(
        &mut v,
        cat_x + 0.18,
        bob + 0.03,
        cat_x + 0.19,
        bob + 0.04,
        [0.0, 0.0, 0.0],
    );
    push_rect(
        &mut v,
        cat_x + 0.28,
        bob + 0.03,
        cat_x + 0.29,
        bob + 0.04,
        [0.0, 0.0, 0.0],
    );
    push_rect(
        &mut v,
        cat_x + 0.20,
        bob - 0.05,
        cat_x + 0.26,
        bob - 0.03,
        [0.05, 0.05, 0.05],
    );
    push_rect(
        &mut v,
        cat_x + 0.14,
        bob - 0.06,
        cat_x + 0.18,
        bob - 0.01,
        [0.98, 0.63, 0.74],
    );
    push_rect(
        &mut v,
        cat_x + 0.28,
        bob - 0.06,
        cat_x + 0.32,
        bob - 0.01,
        [0.98, 0.63, 0.74],
    );

    let foot_y = bob - 0.20 + 0.01 * (phase * 12.0).sin();
    push_rect(
        &mut v,
        cat_x + 0.12,
        foot_y,
        cat_x + 0.20,
        foot_y + 0.06,
        [0.10, 0.10, 0.12],
    );
    push_rect(
        &mut v,
        cat_x + 0.28,
        foot_y,
        cat_x + 0.36,
        foot_y + 0.06,
        [0.10, 0.10, 0.12],
    );

    push_rect(&mut v, -0.98, 0.78, 0.98, 0.96, [0.06, 0.04, 0.04]);
    push_quote_vertices(
        &mut v,
        "GOD IS LOVE. 1 JOHN 4:8",
        -0.94,
        0.82,
        0.016,
        [0.97, 0.96, 0.92],
    );

    v
}

fn push_quote_vertices(v: &mut Vec<f32>, text: &str, x: f32, y: f32, scale: f32, color: [f32; 3]) {
    let mut cursor_x = x;
    for ch in text.chars() {
        let rows = glyph_5x7(ch);
        for (ry, mask) in rows.iter().enumerate() {
            for rx in 0..5 {
                if (mask >> (4 - rx)) & 1 == 1 {
                    let x0 = cursor_x + rx as f32 * scale;
                    let y0 = y + (6.0 - ry as f32) * scale;
                    push_rect(v, x0, y0, x0 + scale, y0 + scale, color);
                }
            }
        }
        cursor_x += if ch == ' ' { scale * 3.0 } else { scale * 6.0 };
    }
}

fn run_browser_session(
    socket_name: &str,
    stop: Option<Arc<AtomicBool>>,
    #[cfg(target_os = "macos")] iosurface_notify: Option<mpsc::Sender<mach_port_t>>,
) -> Result<(), Box<dyn Error>> {
    let perf_log = env::var("DAWN_WGPU_PERF_LOG")
        .ok()
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    #[cfg(target_os = "macos")]
    let control_socket_name = format!(
        "dr-ctrl-{}-{:08x}",
        std::process::id(),
        (SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64) & 0xffff_ffff
    );
    #[cfg(target_os = "macos")]
    let iosurface_port_service: String = {
        let stamp = (SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64) & 0xffff_ffff;
        format!("dr-isp-{}-{stamp:08x}", std::process::id())
    };
    #[cfg(target_os = "windows")]
    let main_surface_host: Option<BrowserWindowHost> = Some(BrowserWindowHost::spawn()?);

    #[cfg(all(unix, not(target_os = "macos")))]
    let main_surface_host: Option<BrowserWindowHost> = Some(BrowserWindowHost::spawn()?);
    let mut child = spawn_gpu_process(
        socket_name,
        #[cfg(target_os = "macos")]
        Some(&iosurface_port_service),
        #[cfg(target_os = "macos")]
        Some(&control_socket_name),
    )?;
    let wire_backend = IpcWireBackend::connect_name_with_options(
        socket_name,
        WireInitOptions {
            reserve_surface: false,
            connect_attempts: 300,
            connect_delay: Duration::from_millis(10),
        },
    )?;
    #[cfg(target_os = "macos")]
    let mut control_stream = control_bind_and_accept_with_stop(&control_socket_name, stop.as_ref())?;
    #[cfg(target_os = "macos")]
    let _ = control_stream.set_recv_timeout(Some(Duration::from_millis(1)));
    #[cfg(target_os = "macos")]
    eprintln!("browser: local IOSurface shared texture prepared for wire Reserve+Inject");
    let instance = wire_backend.dawn_instance();
    let _keep_wire_backend_alive = &wire_backend;
    let mut flush_wire = || {
        wire_backend.pump();
    };

    let adapter = request_adapter_with_fallback(&instance, &mut flush_wire)?;
    let mut adapter_info = dawn_rs::AdapterInfo::new();
    let info_status = adapter.get_info(&mut adapter_info);
    eprintln!(
        "browser: adapter info status={info_status:?} backend={:?} vendor=\"{}\" vendor_id={} device_id={} arch=\"{}\" device=\"{}\" name=\"{}\"",
        adapter_info
            .backend_type
            .unwrap_or(dawn_rs::BackendType::Undefined),
        adapter_info.vendor.as_deref().unwrap_or(""),
        adapter_info.vendor_id.unwrap_or_default(),
        adapter_info.device_id.unwrap_or_default(),
        adapter_info.architecture.as_deref().unwrap_or(""),
        adapter_info.device.as_deref().unwrap_or(""),
        adapter_info.description.as_deref().unwrap_or("")
    );
    let device = request_device_sync(
        &instance,
        &adapter,
        adapter_info.backend_type,
        &mut flush_wire,
    )?;
    let queue = device.get_queue();

    let format = dawn_rs::TextureFormat::Bgra8Unorm;
    let shader = create_nya_shader(&device);
    let pipeline = create_nya_pipeline(&device, shader, format);
    let mut wire_render_target: Option<dawn_rs::Texture> = None;
    #[cfg(target_os = "macos")]
    let should_reserve_shared_wire_texture = cfg!(target_os = "macos");
    #[cfg(target_os = "macos")]
    if should_reserve_shared_wire_texture {
        if let Ok(reserved) = wire_backend.reserve_texture(
            &device,
            WIDTH,
            HEIGHT,
            dawn_rs::TextureFormat::Bgra8Unorm,
            dawn_rs::TextureUsage::TEXTURE_BINDING
                | dawn_rs::TextureUsage::COPY_SRC
                | dawn_rs::TextureUsage::COPY_DST
                | dawn_rs::TextureUsage::RENDER_ATTACHMENT,
        ) {
            let spec = SharedWireTextureSpec {
                reservation: reserved.reservation(),
            };
            write_shared_texture_spec(&mut control_stream, spec)?;
            let ack_ok = read_control_ack_with_retry(
                &mut control_stream,
                400,
                Duration::from_millis(2),
            )?;
            if !ack_ok {
                return Err("gpu process rejected shared texture injection".into());
            }
            #[cfg(target_os = "macos")]
            if let Some(notify) = iosurface_notify.as_ref() {
                let received_surface = mach_receive_iosurface_port(&iosurface_port_service)?;
                eprintln!(
                    "browser: received IOSurface mach port from gpu id={} ({}x{})",
                    received_surface.id(),
                    received_surface.width(),
                    received_surface.height()
                );
                let port = received_surface.create_mach_port();
                if port != MACH_PORT_NULL && notify.send(port).is_err() {
                    unsafe {
                        let _ = mach_port_deallocate(mach_task_self(), port);
                    }
                }
            }
            wire_render_target = Some(reserved.texture.clone());
        }
    }
    if wire_render_target.is_none() {
        let mut tex_desc = dawn_rs::TextureDescriptor::new();
        tex_desc.format = Some(format);
        tex_desc.dimension = Some(dawn_rs::TextureDimension::D2);
        tex_desc.size = Some(dawn_rs::Extent3D {
            width: Some(WIDTH),
            height: Some(HEIGHT),
            depth_or_array_layers: Some(1),
        });
        tex_desc.mip_level_count = Some(1);
        tex_desc.sample_count = Some(1);
        tex_desc.usage = Some(
            dawn_rs::TextureUsage::RENDER_ATTACHMENT
                | dawn_rs::TextureUsage::COPY_SRC
                | dawn_rs::TextureUsage::TEXTURE_BINDING,
        );
        wire_render_target = Some(device.create_texture(&tex_desc));
    }
    let wire_render_target = wire_render_target.ok_or("failed to create wire render target")?;
    let mut vb_desc = dawn_rs::BufferDescriptor::new();
    vb_desc.size = Some(MAX_VERTEX_BYTES);
    vb_desc.usage = Some(BufferUsage::VERTEX | BufferUsage::COPY_DST);
    let vertex_buffer = device
        .create_buffer(&vb_desc)
        .ok_or("create vertex buffer failed")?;

    let start = Instant::now();
    let mut perf_window_start = Instant::now();
    let mut perf_window_frames: u64 = 0;
    let mut perf_window_frame_time_sum = Duration::ZERO;
    let child_exit_status = loop {
        if let Some(stop_flag) = stop.as_ref()
            && stop_flag.load(Ordering::Relaxed)
        {
            let _ = child.kill();
            let status = child.wait()?;
            break status;
        }
        let _keep_wire_backend_alive = &wire_backend;
        if let Some(status) = child.try_wait()? {
            break status;
        }
        let frame_begin = Instant::now();
        let phase = start.elapsed().as_secs_f32();
        let vertices = build_nya_vertices(phase);
        let vertex_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr().cast::<u8>(),
                vertices.len() * std::mem::size_of::<f32>(),
            )
        };
        if vertex_bytes.len() as u64 > MAX_VERTEX_BYTES {
            return Err("nya vertex data overflow".into());
        }
        let view = wire_render_target.create_view(None);

        let mut color_attachment = RenderPassColorAttachment::new();
        color_attachment.view = Some(view);
        color_attachment.load_op = Some(LoadOp::Clear);
        color_attachment.store_op = Some(dawn_rs::StoreOp::Store);
        color_attachment.clear_value = Some(Color {
            r: Some(1.0),
            g: Some(0.67),
            b: Some(0.03),
            a: Some(1.0),
        });
        let mut pass_desc = RenderPassDescriptor::new();
        pass_desc.color_attachments = Some(vec![color_attachment]);

        let encoder = device.create_command_encoder(None);
        encoder.write_buffer(vertex_buffer.clone(), 0, vertex_bytes);
        let pass = encoder.begin_render_pass(&pass_desc);
        pass.set_pipeline(pipeline.clone());
        pass.set_vertex_buffer(0, Some(vertex_buffer.clone()), 0, vertex_bytes.len() as u64);
        pass.draw((vertices.len() / 5) as u32, 1, 0, 0);
        pass.end();
        queue.submit(&[encoder.finish(None)]);
        if perf_log {
            perf_window_frames = perf_window_frames.saturating_add(1);
            perf_window_frame_time_sum += frame_begin.elapsed();
            let elapsed = perf_window_start.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let fps = perf_window_frames as f64 / elapsed.as_secs_f64();
                let avg_ms = if perf_window_frames > 0 {
                    (perf_window_frame_time_sum.as_secs_f64() * 1000.0) / perf_window_frames as f64
                } else {
                    0.0
                };
                eprintln!("perf: fps={fps:.1} avg_frame_ms={avg_ms:.2}");
                perf_window_start = Instant::now();
                perf_window_frames = 0;
                perf_window_frame_time_sum = Duration::ZERO;
            }
        }

        flush_wire();
        instance.process_events();
        thread::sleep(Duration::from_millis(16));
    };
    eprintln!("browser: gpu child exited with status={child_exit_status}");

    if child.try_wait()?.is_none() {
        let _ = child.kill();
        let _ = child.wait();
    }
    if !child_exit_status.success() {
        return Err(format!("gpu child exited abnormally: {child_exit_status}").into());
    }
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    drop(main_surface_host);
    Ok(())
}

fn run_gpu_process(
    socket_name: &str,
    parent_pid: Option<u32>,
    #[cfg(target_os = "macos")] iosurface_port_service: Option<String>,
    #[cfg(target_os = "macos")]
    control_socket_name: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut app = GpuProcessRuntime::new(
        socket_name.to_string(),
        parent_pid,
        #[cfg(target_os = "macos")]
        control_socket_name,
        #[cfg(target_os = "macos")]
        iosurface_port_service,
    );
    app.init()?;
    loop {
        if !app.tick()? {
            break;
        }
        thread::sleep(Duration::from_millis(8));
    }
    Ok(())
}

struct GpuProcessRuntime {
    socket_name: String,
    wire_server: Option<WireServer>,
    #[cfg(target_os = "macos")]
    iosurface_port_service: Option<String>,
    #[cfg(target_os = "macos")]
    control_socket_name: Option<String>,
    #[cfg(target_os = "macos")]
    control_stream: Option<IpcStream>,
    #[cfg(unix)]
    parent_pid: Option<u32>,
    #[cfg(target_os = "macos")]
    created_iosurface: Option<CFRetained<IOSurfaceRef>>,
    shared_texture_injected: bool,
}

impl GpuProcessRuntime {
    fn new(
        socket_name: String,
        parent_pid: Option<u32>,
        #[cfg(target_os = "macos")]
        control_socket_name: Option<String>,
        #[cfg(target_os = "macos")] iosurface_port_service: Option<String>,
    ) -> Self {
        #[cfg(not(unix))]
        let _ = parent_pid;
        Self {
            socket_name,
            wire_server: None,
            #[cfg(target_os = "macos")]
            iosurface_port_service,
            #[cfg(target_os = "macos")]
            control_socket_name,
            #[cfg(target_os = "macos")]
            control_stream: None,
            #[cfg(unix)]
            parent_pid,
            #[cfg(target_os = "macos")]
            created_iosurface: None,
            shared_texture_injected: false,
        }
    }

    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let wire_server = WireServer::accept_and_inject_native(
            &self.socket_name,
            None,
            WireServerOptions {
                expect_surface: false,
                use_spontaneous_callbacks: true,
                max_allocation_size: 0,
                transport: dawn_rs::wire::TransportOptions::default(),
            },
        )?;

        self.wire_server = Some(wire_server);
        Ok(())
    }

    fn tick(&mut self) -> Result<bool, Box<dyn Error>> {
        #[cfg(unix)]
        if let Some(parent_pid) = self.parent_pid {
            let current_ppid = unsafe { libc::getppid() } as u32;
            if current_ppid != parent_pid {
                eprintln!(
                    "gpu: parent changed (expected {parent_pid}, got {current_ppid}), exiting"
                );
                return Ok(false);
            }
        }
        if let Some(server) = self.wire_server.as_ref() {
            server.flush();
        }
        #[cfg(target_os = "macos")]
        if self.control_stream.is_none()
            && let Some(name) = self.control_socket_name.as_deref()
            && let Ok(stream) = control_connect_with_retry(name, 1, Duration::from_millis(0))
        {
            let _ = stream.set_recv_timeout(Some(Duration::from_millis(1)));
            self.control_stream = Some(stream);
        }
        #[cfg(target_os = "macos")]
        if let (Some(server), Some(stream)) = (self.wire_server.as_ref(), self.control_stream.as_mut()) {
            loop {
                let message = match read_shared_texture_spec(stream) {
                    Ok(Some(spec)) => spec,
                    Ok(None) => break,
                    Err(err) => {
                        if matches!(
                            err.kind(),
                            std::io::ErrorKind::UnexpectedEof
                                | std::io::ErrorKind::BrokenPipe
                                | std::io::ErrorKind::ConnectionReset
                        ) {
                            self.control_stream = None;
                        } else {
                            eprintln!("gpu: control read failed: {err}");
                        }
                        break;
                    }
                };
                let mut ok = false;
                let mut needs_send_surface = false;
                #[cfg(target_os = "macos")]
                {
                    let needs_new_surface = self.created_iosurface.as_ref().is_none_or(|surf| {
                        surf.width() as u32 != message.reservation.width()
                            || surf.height() as u32 != message.reservation.height()
                    });
                    if needs_new_surface
                        && let Some(new_surface) = create_iosurface(
                            message.reservation.width(),
                            message.reservation.height(),
                        )
                    {
                        needs_send_surface = true;
                        self.created_iosurface = Some(new_surface);
                    }
                    if let Some(io_surface) = self.created_iosurface.as_ref() {
                        ok = server
                            .inject_iosurface_texture(
                                (io_surface.as_ref() as *const IOSurfaceRef).cast_mut().cast(),
                                message.reservation,
                            )
                            .is_ok();
                    }
                }
                #[cfg(target_os = "windows")]
                {
                    let _ = message;
                }
                #[cfg(target_os = "linux")]
                {
                    let _ = message;
                }
                if let Err(err) = write_control_ack(stream, ok) {
                    eprintln!("gpu: control ack write failed: {err}");
                    break;
                }
                #[cfg(target_os = "macos")]
                if ok && needs_send_surface
                    && let (Some(service_name), Some(io_surface)) = (
                        self.iosurface_port_service.as_deref(),
                        self.created_iosurface.as_ref(),
                    )
                {
                    if let Err(err) = mach_send_iosurface_port_with_retry(
                        service_name,
                        io_surface.as_ref(),
                        200,
                        Duration::from_millis(5),
                    ) {
                        eprintln!("gpu: send IOSurface to browser failed: {err}");
                    } else {
                        eprintln!(
                            "gpu: sent IOSurface mach port to browser id={}",
                            io_surface.id()
                        );
                    }
                }
                self.shared_texture_injected |= ok;
            }
        }
        Ok(true)
    }
}

impl Drop for GpuProcessRuntime {
    fn drop(&mut self) {
        let _ = self.wire_server.take();
    }
}

fn request_adapter_sync(
    instance: &Instance,
    options: Option<RequestAdapterOptions>,
    mut flush_wire: impl FnMut(),
) -> Result<dawn_rs::Adapter, String> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<dawn_rs::Adapter, String>>();
    let _future = instance.request_adapter(options.as_ref(), move |status, adapter, message| {
        if status != RequestAdapterStatus::Success {
            let _ = tx.send(Err(format!("{status:?}: {message}")));
            return;
        }
        match adapter {
            Some(adapter) => {
                let _ = tx.send(Ok(adapter));
            }
            None => {
                let _ = tx.send(Err("request_adapter returned None".to_string()));
            }
        }
    });

    loop {
        match rx.try_recv() {
            Ok(result) => return result,
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                flush_wire();
                instance.process_events();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err("request_adapter callback disconnected".to_string());
            }
        }
    }
}

fn request_adapter_with_fallback(
    instance: &Instance,
    mut flush_wire: impl FnMut(),
) -> Result<dawn_rs::Adapter, String> {
    let mut attempts: Vec<(&'static str, RequestAdapterOptions)> = Vec::new();
    attempts.push(("auto", RequestAdapterOptions::new()));
    let mut metal = RequestAdapterOptions::new();
    metal.backend_type = Some(BackendType::Metal);
    attempts.push(("metal", metal));

    let mut last_err = String::from("no adapter attempts");
    for (label, options) in attempts {
        match request_adapter_sync(instance, Some(options), &mut flush_wire) {
            Ok(adapter) => {
                eprintln!("browser: selected adapter via {label}");
                return Ok(adapter);
            }
            Err(err) => {
                eprintln!("browser: adapter attempt {label} failed: {err}");
                last_err = err;
            }
        }
    }
    Err(last_err)
}

fn request_device_sync(
    instance: &Instance,
    adapter: &dawn_rs::Adapter,
    adapter_backend: Option<BackendType>,
    mut flush_wire: impl FnMut(),
) -> Result<dawn_rs::Device, String> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<dawn_rs::Device, String>>();
    let mut device_desc = dawn_rs::DeviceDescriptor::new();
    #[cfg(target_os = "macos")]
    {
        if adapter_backend == Some(BackendType::Metal) {
            device_desc.required_features = Some(vec![
                dawn_rs::FeatureName::SharedTextureMemoryIOSurface,
            ]);
        }
    }
    let uncaptured = dawn_rs::UncapturedErrorCallbackInfo::new();
    *uncaptured.callback.borrow_mut() = Some(Box::new(move |_devices, error_type, message| {
        eprintln!("browser: uncaptured device error type={error_type:?} message={message}");
    }));
    device_desc.uncaptured_error_callback_info = Some(uncaptured);
    let _future = adapter.request_device(Some(&device_desc), move |status, device, message| {
        if status != dawn_rs::RequestDeviceStatus::Success {
            let _ = tx.send(Err(format!("{status:?}: {message}")));
            return;
        }
        match device {
            Some(device) => {
                let _ = tx.send(Ok(device));
            }
            None => {
                let _ = tx.send(Err("request_device returned None".to_string()));
            }
        }
    });

    loop {
        match rx.try_recv() {
            Ok(result) => return result,
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                flush_wire();
                instance.process_events();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err("request_device callback disconnected".to_string());
            }
        }
    }
}

fn spawn_gpu_process(
    socket_name: &str,
    #[cfg(target_os = "macos")] iosurface_port_service: Option<&str>,
    #[cfg(target_os = "macos")]
    control_socket_name: Option<&str>,
) -> Result<Child, Box<dyn Error>> {
    let current = env::current_exe()?;
    let mut cmd = Command::new(current);
    if let Ok(exe) = env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        cmd.current_dir(exe_dir);
    }
    eprintln!("browser: spawning gpu process with mode=Native");
    cmd.arg("--gpu-process")
        .arg(socket_name)
        .arg(std::process::id().to_string());
    #[cfg(target_os = "macos")]
    if let Some(service_name) = iosurface_port_service {
        cmd.arg("--iosurface-port-service").arg(service_name);
    }
    #[cfg(target_os = "macos")]
    if let Some(control_name) = control_socket_name {
        cmd.arg("--control-socket").arg(control_name);
    }
    #[cfg(unix)]
    cmd.arg0("rainbow_cat_gpu_child");
    Ok(cmd.spawn()?)
}
