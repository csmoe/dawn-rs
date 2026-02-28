use dawn_rs::wire::{
    Server as WireServer, ServerOptions as WireServerOptions, with_native_runtime,
};
use dawn_rs::{
    BackendType, BufferUsage, Color, ColorTargetState, FragmentState, Instance, LoadOp,
    MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    RequestAdapterStatus, ShaderModuleDescriptor, ShaderSourceWGSL, SurfaceGetCurrentTextureStatus,
    SurfaceTexture, TextureFormat, TextureUsage, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexState, VertexStepMode,
};
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
use dawn_rs::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, SamplerBindingLayout,
    SamplerBindingType, ShaderStage, TextureBindingLayout, TextureSampleType, TextureViewDimension,
};
use dawn_wgpu::wire_backend::{IpcWireBackend, WireInitOptions};
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
use objc2_core_foundation::{
    CFDictionary, CFMutableDictionary, CFNumber, CFRetained, CFString, CFType,
};
#[cfg(target_os = "macos")]
use objc2_io_surface::{
    IOSurfaceLockOptions, IOSurfaceRef, kIOSurfaceBytesPerElement, kIOSurfaceHeight,
    kIOSurfacePixelFormat, kIOSurfaceWidth,
};
#[cfg(target_os = "macos")]
use raw_window_metal::Layer;
use std::env;
use std::error::Error;
#[cfg(target_os = "macos")]
use std::ffi::CString;
#[cfg(target_os = "windows")]
use std::ffi::c_void;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command};
#[cfg(target_os = "windows")]
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
#[cfg(all(unix, not(target_os = "macos")))]
use winit::raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowAttributes, WindowId};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;
const MAX_VERTEX_BYTES: u64 = 256 * 1024;
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

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
static SHARED_TEXTURE_PREVIEW_SHADER: &str = r#"
struct VsOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
  var pos = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -3.0),
    vec2<f32>(-1.0,  1.0),
    vec2<f32>( 3.0,  1.0)
  );
  var uv = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 2.0),
    vec2<f32>(0.0, 0.0),
    vec2<f32>(2.0, 0.0)
  );
  var out: VsOut;
  out.pos = vec4<f32>(pos[vi], 0.0, 1.0);
  out.uv = uv[vi];
  return out;
}

@group(0) @binding(0) var tex_sampler: sampler;
@group(0) @binding(1) var tex_src: texture_2d<f32>;

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
  return textureSample(tex_src, tex_sampler, in.uv);
}
"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GpuMode {
    Native,
    SwiftShaderVk,
}

#[cfg(target_os = "macos")]
struct LocalMacIosurface {
    io_surface: CFRetained<IOSurfaceRef>,
}

#[cfg(target_os = "macos")]
fn create_local_iosurface_shared_texture(width: u32, height: u32) -> Option<LocalMacIosurface> {
    const PIXEL_FORMAT_BGRA: i32 = 0x4247_5241;
    let width_cf = CFNumber::new_i32(width as i32);
    let height_cf = CFNumber::new_i32(height as i32);
    let bpe_cf = CFNumber::new_i32(4);
    let fmt_cf = CFNumber::new_i32(PIXEL_FORMAT_BGRA);
    let props = CFMutableDictionary::<CFString, CFType>::with_capacity(4);
    unsafe {
        props.set(kIOSurfaceWidth, width_cf.as_ref());
        props.set(kIOSurfaceHeight, height_cf.as_ref());
        props.set(kIOSurfaceBytesPerElement, bpe_cf.as_ref());
        props.set(kIOSurfacePixelFormat, fmt_cf.as_ref());
    }
    let props_ref = unsafe {
        &*(props.as_ref() as *const CFDictionary<CFString, CFType> as *const CFDictionary)
    };
    let io_surface = unsafe { IOSurfaceRef::new(props_ref) }?;
    draw_quote_and_cat_into_iosurface(&io_surface, width, height);
    eprintln!(
        "main: created local IOSurface id={} size={}x{}",
        io_surface.id(),
        width,
        height
    );
    Some(LocalMacIosurface { io_surface })
}

#[cfg(target_os = "macos")]
fn draw_quote_and_cat_into_iosurface(io_surface: &IOSurfaceRef, width: u32, height: u32) {
    let lock_ok = unsafe { io_surface.lock(IOSurfaceLockOptions::empty(), std::ptr::null_mut()) }
        == KERN_SUCCESS;
    if !lock_ok {
        eprintln!("main: IOSurface lock failed");
        return;
    }
    let base_ptr = io_surface.base_address().as_ptr();
    let row_bytes = io_surface.bytes_per_row();
    if base_ptr.is_null() || row_bytes == 0 {
        let _ = unsafe { io_surface.unlock(IOSurfaceLockOptions::empty(), std::ptr::null_mut()) };
        return;
    }
    let buf_len = row_bytes.saturating_mul(height as usize);
    let pixels = unsafe { std::slice::from_raw_parts_mut(base_ptr.cast::<u8>(), buf_len) };

    for y in 0..height as usize {
        for x in 0..width as usize {
            let idx = y * row_bytes + x * 4;
            pixels[idx] = 0x1A;
            pixels[idx + 1] = 0x10;
            pixels[idx + 2] = 0x10;
            pixels[idx + 3] = 0xFF;
        }
    }

    let mut put_rect = |x0: i32, y0: i32, x1: i32, y1: i32, bgr: [u8; 3]| {
        let x0 = x0.clamp(0, width as i32);
        let y0 = y0.clamp(0, height as i32);
        let x1 = x1.clamp(0, width as i32);
        let y1 = y1.clamp(0, height as i32);
        for y in y0..y1 {
            for x in x0..x1 {
                let idx = (y as usize) * row_bytes + (x as usize) * 4;
                pixels[idx] = bgr[0];
                pixels[idx + 1] = bgr[1];
                pixels[idx + 2] = bgr[2];
                pixels[idx + 3] = 0xFF;
            }
        }
    };

    let rainbow_h = (height as i32 / 14).max(4);
    let rainbow_y = (height as i32 * 2 / 3).max(40);
    let stripe_colors = [
        [0x10, 0x10, 0xEF],
        [0x10, 0x90, 0xFF],
        [0x10, 0xE0, 0xF0],
        [0x20, 0xD0, 0x20],
        [0xE0, 0x70, 0x20],
        [0x90, 0x10, 0x90],
    ];
    for (i, c) in stripe_colors.iter().enumerate() {
        put_rect(
            20,
            rainbow_y + i as i32 * rainbow_h,
            width as i32 / 2,
            rainbow_y + (i as i32 + 1) * rainbow_h,
            *c,
        );
    }
    let cx = (width as i32 * 2 / 3).max(200);
    let cy = (height as i32 * 3 / 4).max(180);
    put_rect(cx - 90, cy - 60, cx + 30, cy + 45, [0xC8, 0xB8, 0xB8]);
    put_rect(cx - 80, cy - 50, cx + 20, cy + 35, [0xE6, 0xB6, 0xE6]);
    put_rect(cx + 20, cy - 70, cx + 120, cy + 35, [0x70, 0x70, 0x70]);
    put_rect(cx + 40, cy - 95, cx + 65, cy - 70, [0x70, 0x70, 0x70]);
    put_rect(cx + 85, cy - 95, cx + 110, cy - 70, [0x70, 0x70, 0x70]);
    put_rect(cx + 45, cy - 35, cx + 60, cy - 20, [0xFF, 0xFF, 0xFF]);
    put_rect(cx + 82, cy - 35, cx + 97, cy - 20, [0xFF, 0xFF, 0xFF]);
    put_rect(cx + 52, cy - 30, cx + 56, cy - 26, [0x10, 0x10, 0x10]);
    put_rect(cx + 89, cy - 30, cx + 93, cy - 26, [0x10, 0x10, 0x10]);

    let mut draw_glyph = |ch: char, x: i32, y: i32, scale: i32, bgr: [u8; 3]| {
        let rows = glyph_5x7(ch);
        for (ry, mask) in rows.iter().enumerate() {
            for rx in 0..5 {
                if (mask >> (4 - rx)) & 1 == 1 {
                    put_rect(
                        x + rx * scale,
                        y + ry as i32 * scale,
                        x + (rx + 1) * scale,
                        y + (ry as i32 + 1) * scale,
                        bgr,
                    );
                }
            }
        }
    };

    let quote = "GOD IS LOVE. 1 JOHN 4:8";
    let scale = 4;
    let mut px = 24i32;
    let py = 24i32;
    for ch in quote.chars() {
        draw_glyph(ch, px, py, scale, [0xF0, 0xF0, 0xF0]);
        px += if ch == ' ' { scale * 3 } else { scale * 6 };
    }

    let _ = unsafe { io_surface.unlock(IOSurfaceLockOptions::empty(), std::ptr::null_mut()) };
}

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

#[cfg(target_os = "windows")]
const PROCESS_DUP_HANDLE: u32 = 0x0040;
#[cfg(target_os = "windows")]
const DUPLICATE_SAME_ACCESS: u32 = 0x0000_0002;

#[cfg(target_os = "windows")]
unsafe extern "system" {
    fn OpenProcess(desired_access: u32, inherit_handle: i32, process_id: u32) -> *mut c_void;
    fn GetCurrentProcess() -> *mut c_void;
    fn DuplicateHandle(
        source_process_handle: *mut c_void,
        source_handle: *mut c_void,
        target_process_handle: *mut c_void,
        target_handle: *mut *mut c_void,
        desired_access: u32,
        inherit_handle: i32,
        options: u32,
    ) -> i32;
    fn CloseHandle(handle: *mut c_void) -> i32;
}

#[cfg(target_os = "windows")]
fn duplicate_handle_to_gpu_process(
    source_handle_value: usize,
    target_pid: u32,
) -> Result<*mut c_void, Box<dyn Error>> {
    let target_process = unsafe { OpenProcess(PROCESS_DUP_HANDLE, 0, target_pid) };
    if target_process.is_null() {
        return Err("OpenProcess(PROCESS_DUP_HANDLE) failed".into());
    }
    let mut duplicated: *mut c_void = ptr::null_mut();
    let ok = unsafe {
        DuplicateHandle(
            GetCurrentProcess(),
            source_handle_value as *mut c_void,
            target_process,
            &mut duplicated,
            0,
            0,
            DUPLICATE_SAME_ACCESS,
        )
    };
    let _ = unsafe { CloseHandle(target_process) };
    if ok == 0 || duplicated.is_null() {
        return Err("DuplicateHandle to gpu process failed".into());
    }
    Ok(duplicated)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("rainbow_cat_ipc error: {e}");
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
        let mut use_swiftshader_vk = false;
        #[cfg(target_os = "macos")]
        let mut iosurface_port_service: Option<String> = None;
        let mut control_socket_name: Option<String> = None;
        while let Some(arg) = args.next() {
            if arg == "--swiftshader-vk" || arg == "--angle-swiftshader" {
                use_swiftshader_vk = true;
            }
            #[cfg(target_os = "macos")]
            if arg == "--iosurface-port-service" {
                iosurface_port_service = args.next();
            }
            if arg == "--control-socket" {
                control_socket_name = args.next();
            }
        }
        return run_gpu_process(
            &socket_name,
            parent_pid,
            use_swiftshader_vk,
            #[cfg(target_os = "macos")]
            iosurface_port_service,
            control_socket_name,
        );
    }
    run_main_process()
}

fn run_main_process() -> Result<(), Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let reconnect_delay = Duration::from_millis(400);
    let mut mode = GpuMode::Native;
    let mut native_unstable_disconnects = 0u32;
    let mut session_id: u64 = 0;
    loop {
        let socket_name = format!("dawn-wgpu-rainbow-wire-{stamp}-{session_id}");
        session_id = session_id.saturating_add(1);
        let session_start = Instant::now();
        match run_main_session(&socket_name, mode) {
            Ok(()) => {
                eprintln!("main: gpu_process disconnected, restarting session");
                if mode == GpuMode::Native {
                    if session_start.elapsed() < Duration::from_secs(5) {
                        native_unstable_disconnects = native_unstable_disconnects.saturating_add(1);
                    } else {
                        native_unstable_disconnects = 0;
                    }
                    if native_unstable_disconnects >= 2 {
                        eprintln!(
                            "main: native mode unstable ({} fast disconnects), fallback to SwiftShader(Vulkan)",
                            native_unstable_disconnects
                        );
                        mode = GpuMode::SwiftShaderVk;
                    }
                }
                thread::sleep(reconnect_delay);
            }
            Err(err) => {
                eprintln!("main: session failed: {err}");
                let err_text = err.to_string();
                if mode == GpuMode::Native {
                    eprintln!("main: fallback to SwiftShader(Vulkan) on next restart");
                    mode = GpuMode::SwiftShaderVk;
                } else if err_text.contains("TINT_BUILD_SPV_WRITER") {
                    eprintln!(
                        "main: SwiftShader unsupported in this Dawn build, fallback to Native"
                    );
                    mode = GpuMode::Native;
                } else if err_text.contains("No supported adapters")
                    || err_text.contains("no suitable adapter")
                {
                    eprintln!("main: SwiftShader(Vulkan) adapter unavailable, fallback to Native");
                    mode = GpuMode::Native;
                }
                thread::sleep(reconnect_delay);
            }
        }
    }
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

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
fn create_shared_texture_preview_pipeline(
    device: &dawn_rs::Device,
    format: TextureFormat,
) -> (RenderPipeline, BindGroupLayout) {
    let mut sampler_entry = BindGroupLayoutEntry::new();
    sampler_entry.binding = Some(0);
    sampler_entry.visibility = Some(ShaderStage::FRAGMENT);
    let mut sampler_layout = SamplerBindingLayout::new();
    sampler_layout.r#type = Some(SamplerBindingType::Filtering);
    sampler_entry.sampler = Some(sampler_layout);

    let mut texture_entry = BindGroupLayoutEntry::new();
    texture_entry.binding = Some(1);
    texture_entry.visibility = Some(ShaderStage::FRAGMENT);
    let mut texture_layout = TextureBindingLayout::new();
    texture_layout.sample_type = Some(TextureSampleType::Float);
    texture_layout.view_dimension = Some(TextureViewDimension::D2);
    texture_layout.multisampled = Some(false);
    texture_entry.texture = Some(texture_layout);

    let mut bgl_desc = BindGroupLayoutDescriptor::new();
    bgl_desc.entries = Some(vec![sampler_entry, texture_entry]);
    let bgl = device.create_bind_group_layout(&bgl_desc);

    let mut shader_desc = ShaderModuleDescriptor::new();
    shader_desc = shader_desc.with_extension(
        ShaderSourceWGSL {
            code: Some(SHARED_TEXTURE_PREVIEW_SHADER.to_string()),
        }
        .into(),
    );
    let shader = device.create_shader_module(&shader_desc);

    let mut vertex = VertexState::new();
    vertex.module = Some(shader.clone());
    vertex.entry_point = Some("vs_main".to_string());
    vertex.buffers = Some(vec![]);

    let mut fragment = FragmentState::new();
    fragment.module = Some(shader);
    fragment.entry_point = Some("fs_main".to_string());
    let mut color_target = ColorTargetState::new();
    color_target.format = Some(format);
    fragment.targets = Some(vec![color_target]);

    let mut pl_desc = PipelineLayoutDescriptor::new();
    pl_desc.bind_group_layouts = Some(vec![bgl.clone()]);

    let mut desc = RenderPipelineDescriptor::new();
    desc.layout = Some(device.create_pipeline_layout(&pl_desc));
    desc.vertex = Some(vertex);
    desc.fragment = Some(fragment);
    desc.primitive = Some(PrimitiveState::new());
    desc.multisample = Some(MultisampleState::new());
    (device.create_render_pipeline(&desc), bgl)
}

fn push_rect(v: &mut Vec<f32>, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 3]) {
    let [r, g, b] = color;
    v.extend_from_slice(&[x0, y0, r, g, b, x1, y0, r, g, b, x1, y1, r, g, b]);
    v.extend_from_slice(&[x0, y0, r, g, b, x1, y1, r, g, b, x0, y1, r, g, b]);
}

fn build_nya_vertices(phase: f32) -> Vec<f32> {
    let mut v = Vec::with_capacity(6000);

    push_rect(&mut v, -1.0, -1.0, 1.0, 1.0, [1.0, 0.67, 0.03]);

    let cat_x = 0.25 + 0.03 * (phase * 1.8).sin();
    let bob = 0.03 * (phase * 6.0).sin();
    let tail_wag = 0.03 * (phase * 9.0).sin();
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
        let x_wave = 0.03 * (phase * 10.0 + i as f32).sin();
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

fn run_main_session(socket_name: &str, mode: GpuMode) -> Result<(), Box<dyn Error>> {
    let perf_log = env::var("DAWN_WGPU_PERF_LOG")
        .ok()
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    #[cfg(target_os = "macos")]
    let iosurface_port_service = {
        let stamp = (SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64) & 0xffff_ffff;
        format!("dr-isp-{}-{stamp:08x}", std::process::id())
    };
    let mut child = spawn_gpu_process(
        socket_name,
        mode,
        #[cfg(target_os = "macos")]
        Some(&iosurface_port_service),
        None,
    )?;
    #[cfg(target_os = "macos")]
    let mut local_mac_iosurface_owner: Option<LocalMacIosurface> = None;
    #[cfg(target_os = "macos")]
    if let Some(owner) = create_local_iosurface_shared_texture(WIDTH, HEIGHT) {
        #[cfg(target_os = "macos")]
        if let Err(err) = mach_send_iosurface_port_with_retry(
            &iosurface_port_service,
            owner.io_surface.as_ref(),
            200,
            Duration::from_millis(10),
        ) {
            eprintln!("main: mach send IOSurface port failed: {err}");
        } else {
            eprintln!("main: sent IOSurface mach port to gpu process via {iosurface_port_service}");
        }
        local_mac_iosurface_owner = Some(owner);
    }
    #[cfg(target_os = "macos")]
    eprintln!("main: local IOSurface shared texture prepared for wire Reserve+Inject");
    #[cfg(target_os = "macos")]
    let _keep_local_mac_iosurface_owner_alive = &local_mac_iosurface_owner;
    let wire_backend = IpcWireBackend::connect_name_with_options(
        socket_name,
        WireInitOptions {
            reserve_surface: true,
            connect_attempts: 300,
            connect_delay: Duration::from_millis(10),
        },
    )?;
    let instance = wire_backend.dawn_instance();
    let surface = wire_backend
        .dawn_surface()
        .ok_or("wire backend did not reserve surface")?;
    let _keep_wire_backend_alive = &wire_backend;
    let mut flush_wire = || {
        wire_backend.pump();
    };

    let adapter = request_adapter_with_fallback(&instance, &surface, mode, &mut flush_wire)?;
    let mut adapter_info = dawn_rs::AdapterInfo::new();
    let info_status = adapter.get_info(&mut adapter_info);
    eprintln!(
        "main: adapter info status={info_status:?} mode={mode:?} backend={:?} vendor=\"{}\" vendor_id={} device_id={} arch=\"{}\" device=\"{}\" name=\"{}\"",
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
    let (device, spirv_writer_missing) = request_device_sync(&instance, &adapter, &mut flush_wire)?;
    let queue = device.get_queue();

    let mut capabilities = dawn_rs::SurfaceCapabilities::new();
    let status = surface.get_capabilities(adapter, &mut capabilities);
    if status != dawn_rs::Status::Success {
        return Err(format!("get_capabilities failed: {status:?}").into());
    }

    let format = capabilities
        .formats
        .as_ref()
        .and_then(|f| f.first().copied())
        .unwrap_or(dawn_rs::TextureFormat::Bgra8Unorm);
    let present_mode = capabilities
        .present_modes
        .as_ref()
        .and_then(|m| m.first().copied())
        .unwrap_or(dawn_rs::PresentMode::Fifo);
    let alpha_mode = capabilities
        .alpha_modes
        .as_ref()
        .and_then(|m| m.first().copied())
        .unwrap_or(dawn_rs::CompositeAlphaMode::Auto);

    let mut config = dawn_rs::SurfaceConfiguration::new();
    config.device = Some(device.clone());
    config.format = Some(format);
    config.usage = Some(TextureUsage::RENDER_ATTACHMENT);
    config.width = Some(WIDTH);
    config.height = Some(HEIGHT);
    config.present_mode = Some(present_mode);
    config.alpha_mode = Some(alpha_mode);
    surface.configure(&config);
    let shader = create_nya_shader(&device);
    let pipeline = create_nya_pipeline(&device, shader, format);
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    let (shared_preview_pipeline, shared_preview_bgl) =
        create_shared_texture_preview_pipeline(&device, format);
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    let mut injected_wire_texture_bind_group: Option<dawn_rs::BindGroup> = None;
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    let should_reserve_shared_wire_texture = false;
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
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
            let texture_view = reserved.texture.create_view(None);
            let sampler = device.create_sampler(Some(&dawn_rs::SamplerDescriptor::new()));
            let mut bg_desc = dawn_rs::BindGroupDescriptor::new();
            bg_desc.layout = Some(shared_preview_bgl.clone());
            let mut sampler_entry = dawn_rs::BindGroupEntry::new();
            sampler_entry.binding = Some(0);
            sampler_entry.sampler = Some(sampler);
            let mut texture_entry = dawn_rs::BindGroupEntry::new();
            texture_entry.binding = Some(1);
            texture_entry.texture_view = Some(texture_view);
            bg_desc.entries = Some(vec![sampler_entry, texture_entry]);
            injected_wire_texture_bind_group = Some(device.create_bind_group(&bg_desc));
        }
    }
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
        let _keep_wire_backend_alive = &wire_backend;
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if mode == GpuMode::SwiftShaderVk && spirv_writer_missing.load(Ordering::Relaxed) {
            return Err("TINT_BUILD_SPV_WRITER is not defined".into());
        }

        let frame_begin = Instant::now();
        let mut st = SurfaceTexture::new();
        surface.get_current_texture(&mut st);
        match st.status {
            Some(
                SurfaceGetCurrentTextureStatus::SuccessOptimal
                | SurfaceGetCurrentTextureStatus::SuccessSuboptimal,
            ) => {}
            Some(
                SurfaceGetCurrentTextureStatus::Outdated | SurfaceGetCurrentTextureStatus::Lost,
            ) => {
                surface.configure(&config);
                flush_wire();
                instance.process_events();
                thread::sleep(Duration::from_millis(8));
                continue;
            }
            _ => {
                flush_wire();
                instance.process_events();
                thread::sleep(Duration::from_millis(8));
                continue;
            }
        }

        let Some(texture) = st.texture else {
            continue;
        };
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
        let view = texture.create_view(None);

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
        #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
        if let Some(bg) = injected_wire_texture_bind_group.as_ref() {
            pass.set_scissor_rect(16, 16, WIDTH.saturating_sub(32), HEIGHT / 3);
            pass.set_pipeline(shared_preview_pipeline.clone());
            pass.set_bind_group(0, Some(bg.clone()), &[]);
            pass.draw(3, 1, 0, 0);
            pass.set_scissor_rect(0, 0, WIDTH, HEIGHT);
        }
        pass.end();
        queue.submit(&[encoder.finish(None)]);
        let _ = surface.present();
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
                eprintln!("perf: fps={fps:.1} avg_frame_ms={avg_ms:.2} mode={mode:?}");
                perf_window_start = Instant::now();
                perf_window_frames = 0;
                perf_window_frame_time_sum = Duration::ZERO;
            }
        }

        flush_wire();
        instance.process_events();
        thread::sleep(Duration::from_millis(16));
    };
    eprintln!("main: gpu child exited with status={child_exit_status}");

    if child.try_wait()?.is_none() {
        let _ = child.kill();
        let _ = child.wait();
    }
    if !child_exit_status.success() {
        return Err(format!("gpu child exited abnormally: {child_exit_status}").into());
    }
    Ok(())
}

fn run_gpu_process(
    socket_name: &str,
    parent_pid: Option<u32>,
    use_swiftshader_vk: bool,
    #[cfg(target_os = "macos")] iosurface_port_service: Option<String>,
    _control_socket_name: Option<String>,
) -> Result<(), Box<dyn Error>> {
    if use_swiftshader_vk {
        if let Ok(current_exe) = env::current_exe() {
            let mut icd: PathBuf = current_exe;
            icd.set_file_name("vk_swiftshader_icd.json");
            if icd.is_file() {
                unsafe {
                    env::set_var("VK_ICD_FILENAMES", icd.as_os_str());
                    env::set_var("VK_DRIVER_FILES", icd.as_os_str());
                }
            }
        }
    }
    #[cfg(target_os = "macos")]
    let received_iosurface = if let Some(service_name) = iosurface_port_service.as_deref() {
        match mach_receive_iosurface_port(service_name) {
            Ok(surface) => {
                eprintln!(
                    "gpu process: received IOSurface mach port id={} ({}x{})",
                    surface.id(),
                    surface.width(),
                    surface.height()
                );
                Some(surface)
            }
            Err(err) => {
                eprintln!("gpu process: failed to receive IOSurface mach port: {err}");
                None
            }
        }
    } else {
        None
    };
    let event_loop = EventLoop::new()?;
    let mut app = GpuProcessApp::new(
        socket_name.to_string(),
        parent_pid,
        #[cfg(target_os = "macos")]
        received_iosurface,
    );
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct GpuProcessApp {
    socket_name: String,
    window: Option<Window>,
    wire_server: Option<WireServer>,
    _native_instance: Option<Instance>,
    _native_surface: Option<dawn_rs::Surface>,
    #[cfg(unix)]
    parent_pid: Option<u32>,
    #[cfg(target_os = "macos")]
    metal_layer: Option<Layer>,
    #[cfg(target_os = "macos")]
    received_iosurface: Option<CFRetained<IOSurfaceRef>>,
    shared_texture_injected: bool,
}

impl GpuProcessApp {
    fn new(
        socket_name: String,
        parent_pid: Option<u32>,
        #[cfg(target_os = "macos")] received_iosurface: Option<CFRetained<IOSurfaceRef>>,
    ) -> Self {
        #[cfg(not(unix))]
        let _ = parent_pid;
        Self {
            socket_name,
            window: None,
            wire_server: None,
            _native_instance: None,
            _native_surface: None,
            #[cfg(unix)]
            parent_pid,
            #[cfg(target_os = "macos")]
            metal_layer: None,
            #[cfg(target_os = "macos")]
            received_iosurface,
            shared_texture_injected: false,
        }
    }

    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<(), Box<dyn Error>> {
        let window = event_loop.create_window(
            WindowAttributes::default()
                .with_title("Rainbow Cat GPU Process (Wire Surface)")
                .with_inner_size(winit::dpi::PhysicalSize::new(WIDTH, HEIGHT)),
        )?;

        let window_handle = window.window_handle().map_err(|_| "window handle")?;
        #[cfg(all(unix, not(target_os = "macos")))]
        let display_handle = window.display_handle().map_err(|_| "display handle")?;

        let mut surface_desc = dawn_rs::SurfaceDescriptor::new();

        #[cfg(target_os = "macos")]
        {
            let layer = match window_handle.as_raw() {
                RawWindowHandle::AppKit(handle) => unsafe { Layer::from_ns_view(handle.ns_view) },
                _ => return Err("expected AppKit handle".into()),
            };
            let mut metal_layer = dawn_rs::SurfaceSourceMetalLayer::new();
            metal_layer.layer = Some(layer.as_ptr().as_ptr().cast());
            surface_desc = surface_desc.with_extension(metal_layer.into());
            self.metal_layer = Some(layer);
        }

        #[cfg(target_os = "windows")]
        {
            let win = match window_handle.as_raw() {
                RawWindowHandle::Win32(handle) => handle,
                _ => return Err("expected Win32 handle".into()),
            };
            let mut win32_layer = dawn_rs::SurfaceSourceWindowsHWND::new();
            win32_layer.hwnd = Some((win.hwnd.get() as *mut std::ffi::c_void).cast());
            win32_layer.hinstance = win
                .hinstance
                .map(|h| (h.get() as *mut std::ffi::c_void).cast());
            surface_desc = surface_desc.with_extension(win32_layer.into());
        }

        #[cfg(all(unix, not(target_os = "macos")))]
        {
            match (display_handle.as_raw(), window_handle.as_raw()) {
                (RawDisplayHandle::Wayland(disp), RawWindowHandle::Wayland(win)) => {
                    let mut wayland = dawn_rs::SurfaceSourceWaylandSurface::new();
                    wayland.display = Some(disp.display.as_ptr().cast());
                    wayland.surface = Some(win.surface.as_ptr().cast());
                    surface_desc = surface_desc.with_extension(wayland.into());
                }
                (RawDisplayHandle::Xlib(disp), RawWindowHandle::Xlib(win)) => {
                    let mut xlib = dawn_rs::SurfaceSourceXlibWindow::new();
                    xlib.display = disp.display.map(|p| p.as_ptr().cast());
                    xlib.window = Some(win.window);
                    surface_desc = surface_desc.with_extension(xlib.into());
                }
                (RawDisplayHandle::Xcb(disp), RawWindowHandle::Xcb(win)) => {
                    let mut xcb = dawn_rs::SurfaceSourceXCBWindow::new();
                    xcb.connection = disp.connection.map(|p| p.as_ptr().cast());
                    xcb.window = Some(win.window.get());
                    surface_desc = surface_desc.with_extension(xcb.into());
                }
                _ => return Err("unsupported Linux handle pair".into()),
            }
        }

        let (native_instance, native_surface) = with_native_runtime(|| {
            let native_instance = Instance::new(None);
            let native_surface = native_instance.create_surface(&surface_desc);
            (native_instance, native_surface)
        })?;
        let wire_server = WireServer::accept_and_inject(
            &self.socket_name,
            &native_instance,
            Some(&native_surface),
            WireServerOptions {
                expect_surface: true,
                use_spontaneous_callbacks: true,
                max_allocation_size: 0,
                transport: dawn_rs::wire::TransportOptions::default(),
            },
        )?;

        self.window = Some(window);
        self.wire_server = Some(wire_server);
        self._native_instance = Some(native_instance);
        self._native_surface = Some(native_surface);
        Ok(())
    }
}

impl Drop for GpuProcessApp {
    fn drop(&mut self) {
        let _ = self.wire_server.take();
        let _ = self._native_surface.take();
        let _ = self._native_instance.take();
    }
}

impl ApplicationHandler for GpuProcessApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none()
            && let Err(err) = self.init(event_loop)
        {
            eprintln!("gpu process init failed: {err}");
            event_loop.exit();
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
            eprintln!("gpu process: window close requested");
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(8),
        ));
        #[cfg(unix)]
        if let Some(parent_pid) = self.parent_pid {
            let current_ppid = unsafe { libc::getppid() } as u32;
            if current_ppid != parent_pid {
                eprintln!(
                    "gpu process: parent changed (expected {parent_pid}, got {current_ppid}), exiting"
                );
                event_loop.exit();
                return;
            }
        }
        if let Some(server) = self.wire_server.as_ref() {
            server.flush();
        }
        let _ = self.shared_texture_injected;
        #[cfg(target_os = "macos")]
        let _ = &self.received_iosurface;
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
    surface: &dawn_rs::Surface,
    mode: GpuMode,
    mut flush_wire: impl FnMut(),
) -> Result<dawn_rs::Adapter, String> {
    let mut attempts: Vec<(&'static str, RequestAdapterOptions)> = Vec::new();
    match mode {
        GpuMode::Native => {
            let mut auto = RequestAdapterOptions::new();
            auto.compatible_surface = Some(surface.clone());
            attempts.push(("auto(surface)", auto));

            let mut metal = RequestAdapterOptions::new();
            metal.compatible_surface = Some(surface.clone());
            metal.backend_type = Some(BackendType::Metal);
            attempts.push(("metal(surface)", metal));
        }
        GpuMode::SwiftShaderVk => {
            let mut vk_surface = RequestAdapterOptions::new();
            vk_surface.compatible_surface = Some(surface.clone());
            vk_surface.backend_type = Some(BackendType::Vulkan);
            vk_surface.force_fallback_adapter = Some(true);
            attempts.push(("vulkan(surface)", vk_surface));

            let mut vk = RequestAdapterOptions::new();
            vk.backend_type = Some(BackendType::Vulkan);
            vk.force_fallback_adapter = Some(true);
            attempts.push(("vulkan(no-surface)", vk));
        }
    }

    let mut last_err = String::from("no adapter attempts");
    for (label, options) in attempts {
        match request_adapter_sync(instance, Some(options), &mut flush_wire) {
            Ok(adapter) => {
                eprintln!("main: selected adapter via {label}");
                return Ok(adapter);
            }
            Err(err) => {
                eprintln!("main: adapter attempt {label} failed: {err}");
                last_err = err;
            }
        }
    }
    Err(last_err)
}

fn request_device_sync(
    instance: &Instance,
    adapter: &dawn_rs::Adapter,
    mut flush_wire: impl FnMut(),
) -> Result<(dawn_rs::Device, Arc<AtomicBool>), String> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<dawn_rs::Device, String>>();
    let spirv_writer_missing = Arc::new(AtomicBool::new(false));
    let spirv_writer_missing_cb = spirv_writer_missing.clone();
    let mut device_desc = dawn_rs::DeviceDescriptor::new();
    #[cfg(target_os = "macos")]
    {
        device_desc.required_features = Some(vec![
            dawn_rs::FeatureName::SharedTextureMemoryIOSurface,
            dawn_rs::FeatureName::SharedFenceMTLSharedEvent,
        ]);
    }
    let uncaptured = dawn_rs::UncapturedErrorCallbackInfo::new();
    *uncaptured.callback.borrow_mut() = Some(Box::new(move |_devices, error_type, message| {
        eprintln!("main: uncaptured device error type={error_type:?} message={message}");
        if message.contains("TINT_BUILD_SPV_WRITER is not defined") {
            spirv_writer_missing_cb.store(true, Ordering::Relaxed);
        }
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
            Ok(result) => return result.map(|device| (device, spirv_writer_missing.clone())),
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
    mode: GpuMode,
    #[cfg(target_os = "macos")] iosurface_port_service: Option<&str>,
    control_socket_name: Option<&str>,
) -> Result<Child, Box<dyn Error>> {
    let current = env::current_exe()?;
    let mut cmd = Command::new(current);
    if let Ok(exe) = env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        cmd.current_dir(exe_dir);
    }
    eprintln!("main: spawning gpu_process with mode={mode:?}");
    if mode == GpuMode::SwiftShaderVk
        && let Ok(exe) = env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        let icd = exe_dir.join("vk_swiftshader_icd.json");
        if icd.is_file() {
            cmd.env("VK_ICD_FILENAMES", &icd);
            cmd.env("VK_DRIVER_FILES", &icd);
        }
    }
    cmd.arg("--gpu-process")
        .arg(socket_name)
        .arg(std::process::id().to_string());
    #[cfg(target_os = "macos")]
    if let Some(service_name) = iosurface_port_service {
        cmd.arg("--iosurface-port-service").arg(service_name);
    }
    if let Some(control_name) = control_socket_name {
        cmd.arg("--control-socket").arg(control_name);
    }
    if mode == GpuMode::SwiftShaderVk {
        cmd.arg("--swiftshader-vk");
    }
    #[cfg(unix)]
    cmd.arg0("rainbow_cat_gpu_child");
    Ok(cmd.spawn()?)
}
