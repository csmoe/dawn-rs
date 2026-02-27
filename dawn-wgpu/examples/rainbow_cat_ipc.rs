use dawn_rs::wire_ipc::OutboundPacket;
use dawn_rs::wire_shim::{WireHelperServer, WireInstanceHandle};
use dawn_wgpu::wire_backend::{IpcWireBackend, WireInitOptions};
use dawn_wgpu::wire_server;
use dawn_rs::{
    BackendType, BufferUsage, Color, ColorTargetState, FragmentState, Instance, LoadOp,
    MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    RequestAdapterStatus, ShaderModuleDescriptor, ShaderSourceWGSL,
    SurfaceGetCurrentTextureStatus, SurfaceTexture, TextureFormat, TextureUsage, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};
#[cfg(target_os = "macos")]
use raw_window_metal::Layer;
use std::env;
use std::error::Error;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GpuMode {
    Native,
    AngleSwiftShader,
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
        let use_angle_swiftshader = args.any(|v| v == "--angle-swiftshader");
        return run_gpu_process(&socket_name, parent_pid, use_angle_swiftshader);
    }
    run_main_process()
}

fn run_main_process() -> Result<(), Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let reconnect_delay = Duration::from_millis(400);
    let force_swiftshader = env::var("DAWN_WGPU_FORCE_SWIFTSHADER")
        .ok()
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    let mut mode = if force_swiftshader {
        eprintln!("main: DAWN_WGPU_FORCE_SWIFTSHADER=1, start with SwiftShader");
        GpuMode::AngleSwiftShader
    } else {
        GpuMode::Native
    };
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
                            "main: native mode unstable ({} fast disconnects), fallback to ANGLE SwiftShader",
                            native_unstable_disconnects
                        );
                        mode = GpuMode::AngleSwiftShader;
                    }
                }
                thread::sleep(reconnect_delay);
            }
            Err(err) => {
                eprintln!("main: session failed: {err}");
                let err_text = err.to_string();
                if mode == GpuMode::Native {
                    eprintln!("main: fallback to ANGLE SwiftShader on next restart");
                    mode = GpuMode::AngleSwiftShader;
                } else if err_text.contains("TINT_BUILD_SPV_WRITER") {
                    if force_swiftshader {
                        return Err(
                            "SwiftShader forced, but Dawn build lacks TINT SPIR-V writer support"
                                .into(),
                        );
                    }
                    eprintln!("main: SwiftShader unsupported in this Dawn build, fallback to Native");
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

    v
}

fn run_main_session(socket_name: &str, mode: GpuMode) -> Result<(), Box<dyn Error>> {
    let mut child = spawn_gpu_process(socket_name, mode)?;
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
    let mut vb_desc = dawn_rs::BufferDescriptor::new();
    vb_desc.size = Some(MAX_VERTEX_BYTES);
    vb_desc.usage = Some(BufferUsage::VERTEX | BufferUsage::COPY_DST);
    let vertex_buffer = device
        .create_buffer(&vb_desc)
        .ok_or("create vertex buffer failed")?;

    let start = Instant::now();
    let child_exit_status = loop {
        let _keep_wire_backend_alive = &wire_backend;
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if mode == GpuMode::AngleSwiftShader
            && spirv_writer_missing.load(Ordering::Relaxed)
        {
            return Err("TINT_BUILD_SPV_WRITER is not defined".into());
        }

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
        pass.end();
        queue.submit(&[encoder.finish(None)]);
        let _ = surface.present();

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
    use_angle_swiftshader: bool,
) -> Result<(), Box<dyn Error>> {
    if use_angle_swiftshader {
        // Set before any Dawn/ANGLE initialization in this process.
        unsafe {
            env::set_var("ANGLE_DEFAULT_PLATFORM", "swiftshader");
        }
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
    let (reader_stream, writer_stream, reserved) = wire_server::accept_wire_client(socket_name)?;

    let event_loop = EventLoop::new()?;
    let mut app = GpuProcessApp::new(
        reader_stream,
        writer_stream,
        reserved.instance,
        reserved.surface,
        reserved.surface_instance,
        parent_pid,
        use_angle_swiftshader,
    );
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct GpuProcessApp {
    reader_stream: Option<interprocess::local_socket::Stream>,
    writer_stream: Option<interprocess::local_socket::Stream>,
    instance_handle: WireInstanceHandle,
    surface_handle: WireInstanceHandle,
    surface_instance_handle: WireInstanceHandle,
    window: Option<Window>,
    pump: Option<wire_server::WireServerPump>,
    server: Option<Arc<Mutex<WireHelperServer>>>,
    _native_instance: Option<Instance>,
    _native_surface: Option<dawn_rs::Surface>,
    handle_wire_on_main_thread: bool,
    parent_pid: Option<u32>,
    #[cfg(target_os = "macos")]
    metal_layer: Option<Layer>,
}

impl GpuProcessApp {
    fn new(
        reader_stream: interprocess::local_socket::Stream,
        writer_stream: interprocess::local_socket::Stream,
        instance_handle: WireInstanceHandle,
        surface_handle: WireInstanceHandle,
        surface_instance_handle: WireInstanceHandle,
        parent_pid: Option<u32>,
        use_angle_swiftshader: bool,
    ) -> Self {
        Self {
            reader_stream: Some(reader_stream),
            writer_stream: Some(writer_stream),
            instance_handle,
            surface_handle,
            surface_instance_handle,
            window: None,
            pump: None,
            server: None,
            _native_instance: None,
            _native_surface: None,
            handle_wire_on_main_thread: use_angle_swiftshader,
            parent_pid,
            #[cfg(target_os = "macos")]
            metal_layer: None,
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

        let reader_stream = self.reader_stream.take().ok_or("missing reader stream")?;
        let mut writer_stream = self.writer_stream.take().ok_or("missing writer stream")?;

        let (to_writer_tx, to_writer_rx) = mpsc::channel::<OutboundPacket>();
        let server = Arc::new(Mutex::new(WireHelperServer::new_native(
            0,
            true,
            {
                let to_writer_tx = to_writer_tx.clone();
                move |bytes: &[u8]| {
                    let _ = to_writer_tx.send(OutboundPacket::Wire(bytes.to_vec()));
                }
            },
            |msg: &str| eprintln!("wire server error: {msg}"),
        )?));

        let native_instance = Instance::new(None);
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

        let native_surface = native_instance.create_surface(&surface_desc);

        let injected_instance = {
            let mut guard = server.lock().map_err(|_| "wire server lock poisoned")?;
            guard.inject_instance(native_instance.as_raw().cast(), self.instance_handle)
        };
        let injected_surface = {
            let mut guard = server.lock().map_err(|_| "wire server lock poisoned")?;
            guard.inject_surface(
                native_surface.as_raw().cast(),
                self.surface_handle,
                self.surface_instance_handle,
            )
        };
        let ok = injected_instance && injected_surface;
        wire_server::send_reserve_ack(&mut writer_stream, ok)?;
        if !ok {
            return Err("failed to inject instance/surface into wire server".into());
        }

        let pump = wire_server::WireServerPump::start(
            reader_stream,
            writer_stream,
            to_writer_tx,
            to_writer_rx,
            server.clone(),
            self.handle_wire_on_main_thread,
        );

        self.window = Some(window);
        self.server = Some(server);
        self._native_instance = Some(native_instance);
        self._native_surface = Some(native_surface);
        self.pump = Some(pump);
        Ok(())
    }
}

impl Drop for GpuProcessApp {
    fn drop(&mut self) {
        // Join wire threads first, then release native objects while native procs
        // are still leased by the wire server, and finally drop the server itself.
        let _ = self.pump.take();
        let _ = self._native_surface.take();
        let _ = self._native_instance.take();
        let _ = self.server.take();
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
        if let Some(pump) = self.pump.as_ref() {
            if self.handle_wire_on_main_thread
                && let Some(server) = self.server.as_ref()
                && let Err(err) = pump.process_pending(server)
            {
                eprintln!("gpu process failed to handle wire command on main thread: {err}");
                event_loop.exit();
                return;
            }
            if pump.is_disconnected() {
                eprintln!("gpu process: wire transport disconnected, exiting");
                event_loop.exit();
                return;
            }
        }
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
        GpuMode::AngleSwiftShader => {
            let mut gles_surface = RequestAdapterOptions::new();
            gles_surface.compatible_surface = Some(surface.clone());
            gles_surface.backend_type = Some(BackendType::OpenGLes);
            attempts.push(("opengles(surface)", gles_surface));

            let mut gles = RequestAdapterOptions::new();
            gles.backend_type = Some(BackendType::OpenGLes);
            attempts.push(("opengles(no-surface)", gles));

            let mut vk_surface = RequestAdapterOptions::new();
            vk_surface.compatible_surface = Some(surface.clone());
            vk_surface.backend_type = Some(BackendType::Vulkan);
            attempts.push(("vulkan(surface)", vk_surface));

            let mut vk = RequestAdapterOptions::new();
            vk.backend_type = Some(BackendType::Vulkan);
            attempts.push(("vulkan(no-surface)", vk));

            let mut gl_surface = RequestAdapterOptions::new();
            gl_surface.compatible_surface = Some(surface.clone());
            gl_surface.backend_type = Some(BackendType::OpenGL);
            attempts.push(("opengl(surface)", gl_surface));

            let mut gl = RequestAdapterOptions::new();
            gl.backend_type = Some(BackendType::OpenGL);
            attempts.push(("opengl(no-surface)", gl));

            // Recovery path: if software backends are unavailable for this surface/platform,
            // fall back to the native-compatible adapter so the session can recover.
            let mut auto = RequestAdapterOptions::new();
            auto.compatible_surface = Some(surface.clone());
            attempts.push(("auto(surface,recover)", auto));

            let mut metal = RequestAdapterOptions::new();
            metal.compatible_surface = Some(surface.clone());
            metal.backend_type = Some(BackendType::Metal);
            attempts.push(("metal(surface,recover)", metal));
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

fn spawn_gpu_process(socket_name: &str, mode: GpuMode) -> Result<Child, Box<dyn Error>> {
    let current = env::current_exe()?;
    let mut cmd = Command::new(current);
    eprintln!("main: spawning gpu_process with mode={mode:?}");
    if mode == GpuMode::AngleSwiftShader {
        if let Ok(exe) = env::current_exe()
            && let Some(exe_dir) = exe.parent()
        {
            let icd = exe_dir.join("vk_swiftshader_icd.json");
            if icd.is_file() {
                cmd.env("VK_ICD_FILENAMES", &icd);
                cmd.env("VK_DRIVER_FILES", &icd);
            }
            #[cfg(target_os = "macos")]
            {
                let mut paths = vec![exe_dir.to_string_lossy().into_owned()];
                if let Some(existing) = env::var_os("DYLD_LIBRARY_PATH") {
                    let existing = existing.to_string_lossy();
                    if !existing.is_empty() {
                        paths.push(existing.into_owned());
                    }
                }
                cmd.env("DYLD_LIBRARY_PATH", paths.join(":"));
            }
        }
    }
    cmd.arg("--gpu-process")
        .arg(socket_name)
        .arg(std::process::id().to_string());
    if mode == GpuMode::AngleSwiftShader {
        cmd.arg("--angle-swiftshader");
    }
    #[cfg(unix)]
    cmd.arg0("rainbow_cat_gpu_child");
    Ok(cmd.spawn()?)
}
