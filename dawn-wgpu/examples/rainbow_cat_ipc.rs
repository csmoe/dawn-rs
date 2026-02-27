use dawn_wgpu::create_instance;
use interprocess::TryClone;
use interprocess::local_socket::{
    GenericFilePath, GenericNamespaced, ListenerOptions, Stream, prelude::*,
};
use pollster::block_on;
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use wgpu::SurfaceError;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

const SHADER: &str = r#"
struct Uniforms {
    time: f32,
}

@group(0) @binding(0) var<uniform> u: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4<f32> {
    var p = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );
    return vec4<f32>(p[vid], 0.0, 1.0);
}

fn rect(p: vec2<f32>, x0: f32, y0: f32, x1: f32, y1: f32) -> bool {
    return p.x >= x0 && p.x < x1 && p.y >= y0 && p.y < y1;
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    // Pixel grid (nearest-neighbor look)
    let cell = 10.0;
    let g = floor(pos.xy / vec2<f32>(cell, cell));

    let black = vec3<f32>(0.00, 0.00, 0.00);
    let yellow = vec3<f32>(1.00, 0.73, 0.02);
    let red = vec3<f32>(1.00, 0.08, 0.05);
    let orange = vec3<f32>(1.00, 0.55, 0.02);
    let rainbow_yellow = vec3<f32>(1.00, 0.95, 0.00);
    let green = vec3<f32>(0.12, 0.92, 0.18);
    let blue = vec3<f32>(0.10, 0.58, 1.00);
    let purple = vec3<f32>(0.46, 0.22, 1.00);
    let crust = vec3<f32>(0.96, 0.76, 0.62);
    let jam = vec3<f32>(0.98, 0.66, 0.90);
    let sprinkle = vec3<f32>(1.00, 0.20, 0.55);
    let cat = vec3<f32>(0.55, 0.55, 0.57);
    let blush = vec3<f32>(1.00, 0.66, 0.72);
    let white = vec3<f32>(1.00, 1.00, 1.00);

    var col = yellow;

    // Layout anchor (close to classic nyan-cat framing)
    let ox = 54.0;
    let oy = 16.0;
    let wave = floor(0.5 + 1.5 * sin(u.time * 5.0 + g.x * 0.2));

    // Rainbow trail
    if (g.x >= 1.0 && g.x < ox - 2.0) {
        let ry = g.y - (oy + 2.0) - wave;
        if (ry >= 0.0 && ry < 18.0) {
            if (ry < 3.0) {
                col = red;
            } else if (ry < 6.0) {
                col = orange;
            } else if (ry < 9.0) {
                col = rainbow_yellow;
            } else if (ry < 12.0) {
                col = green;
            } else if (ry < 15.0) {
                col = blue;
            } else {
                col = purple;
            }
        }
    }

    // Tail (outline then fill)
    if (rect(g, ox - 8.0, oy + 9.0, ox - 1.0, oy + 14.0)) {
        col = black;
    }
    if (rect(g, ox - 7.0, oy + 10.0, ox - 2.0, oy + 13.0)) {
        col = cat;
    }

    // Poptart body
    if (rect(g, ox, oy, ox + 28.0, oy + 22.0)) {
        col = black;
    }
    if (rect(g, ox + 1.0, oy + 1.0, ox + 27.0, oy + 21.0)) {
        col = crust;
    }
    if (rect(g, ox + 4.0, oy + 3.0, ox + 24.0, oy + 19.0)) {
        col = jam;
    }

    // Sprinkles
    if (
        rect(g, ox + 6.0, oy + 6.0, ox + 7.0, oy + 7.0) ||
        rect(g, ox + 10.0, oy + 5.0, ox + 11.0, oy + 6.0) ||
        rect(g, ox + 16.0, oy + 7.0, ox + 17.0, oy + 8.0) ||
        rect(g, ox + 21.0, oy + 6.0, ox + 22.0, oy + 7.0) ||
        rect(g, ox + 12.0, oy + 11.0, ox + 13.0, oy + 12.0) ||
        rect(g, ox + 8.0, oy + 14.0, ox + 9.0, oy + 15.0) ||
        rect(g, ox + 18.0, oy + 14.0, ox + 19.0, oy + 15.0)
    ) {
        col = sprinkle;
    }

    // Cat body/head outline
    if (rect(g, ox + 26.0, oy + 7.0, ox + 43.0, oy + 22.0)) {
        col = black;
    }
    // Ears
    if (rect(g, ox + 27.0, oy + 4.0, ox + 31.0, oy + 8.0) || rect(g, ox + 37.0, oy + 4.0, ox + 41.0, oy + 8.0)) {
        col = black;
    }

    // Cat fill
    if (rect(g, ox + 27.0, oy + 8.0, ox + 42.0, oy + 21.0)) {
        col = cat;
    }
    if (rect(g, ox + 28.0, oy + 5.0, ox + 30.0, oy + 8.0) || rect(g, ox + 38.0, oy + 5.0, ox + 40.0, oy + 8.0)) {
        col = cat;
    }

    // Eyes
    if (rect(g, ox + 31.0, oy + 11.0, ox + 33.0, oy + 13.0) || rect(g, ox + 37.0, oy + 11.0, ox + 39.0, oy + 13.0)) {
        col = white;
    }
    if (rect(g, ox + 31.0, oy + 12.0, ox + 32.0, oy + 13.0) || rect(g, ox + 37.0, oy + 12.0, ox + 38.0, oy + 13.0)) {
        col = black;
    }

    // Mouth
    if (rect(g, ox + 34.0, oy + 14.0, ox + 36.0, oy + 15.0) ||
        rect(g, ox + 33.0, oy + 15.0, ox + 34.0, oy + 16.0) ||
        rect(g, ox + 36.0, oy + 15.0, ox + 37.0, oy + 16.0)) {
        col = black;
    }

    // Blush
    if (rect(g, ox + 29.0, oy + 14.0, ox + 31.0, oy + 16.0) || rect(g, ox + 39.0, oy + 14.0, ox + 41.0, oy + 16.0)) {
        col = blush;
    }

    // Legs with tiny stepping animation
    let step = floor(0.5 + 1.0 * sin(u.time * 7.0));
    if (rect(g, ox + 29.0, oy + 21.0 + step, ox + 32.0, oy + 23.0 + step) ||
        rect(g, ox + 35.0, oy + 21.0 - step, ox + 38.0, oy + 23.0 - step) ||
        rect(g, ox + 40.0, oy + 21.0 + step, ox + 42.0, oy + 23.0 + step) ||
        rect(g, ox + 25.0, oy + 21.0 - step, ox + 27.0, oy + 23.0 - step)) {
        col = black;
    }
    if (rect(g, ox + 30.0, oy + 21.0 + step, ox + 31.0, oy + 22.0 + step) ||
        rect(g, ox + 36.0, oy + 21.0 - step, ox + 37.0, oy + 22.0 - step) ||
        rect(g, ox + 40.0, oy + 21.0 + step, ox + 41.0, oy + 22.0 + step) ||
        rect(g, ox + 25.0, oy + 21.0 - step, ox + 26.0, oy + 22.0 - step)) {
        col = cat;
    }

    return vec4<f32>(col, 1.0);
}
"#;

fn main() {
    if let Err(e) = run() {
        eprintln!("rainbow_cat_ipc error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    if let Some(mode) = args.next()
        && mode == "--ipc-server"
    {
        let name = args.next().ok_or("missing socket name for --ipc-server")?;
        return run_server(&name);
    }
    run_client()
}

fn run_client() -> Result<(), Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let sock_name = format!("dawn-wgpu-rainbow-window-{stamp}");
    let mut child = spawn_server(&sock_name)?;

    let name = to_ipc_name(&sock_name)?;
    let mut stream = connect_with_retry(name)?;
    let start = Instant::now();
    while child.try_wait()?.is_none() {
        let phase = start.elapsed().as_secs_f32();
        write_message(&mut stream, &IpcMessage::AnimationPhase { phase })?;
        stream.flush()?;
        thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

fn run_server(sock_name: &str) -> Result<(), Box<dyn Error>> {
    let name = to_ipc_name(sock_name)?;
    let listener = ListenerOptions::new()
        .name(name)
        .reclaim_name(true)
        .create_sync()?;
    let stream = listener.accept()?;
    let mut reader_stream = stream.try_clone()?;
    let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));

    let phase = Arc::new(Mutex::new(0.0f32));
    let running = Arc::new(AtomicBool::new(true));
    let phase_for_reader = phase.clone();
    let running_for_reader = running.clone();
    let reader_thread = thread::spawn(move || -> Result<(), String> {
        while running_for_reader.load(Ordering::Relaxed) {
            let message = match read_message(&mut reader_stream) {
                Ok(message) => message,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    continue;
                }
                Err(e) => return Err(e.to_string()),
            };
            match message {
                IpcMessage::AnimationPhase { phase: value } => {
                    if let Ok(mut guard) = phase_for_reader.lock() {
                        *guard = value;
                    }
                }
                IpcMessage::Shutdown => break,
            }
        }
        Ok(())
    });

    let event_loop = EventLoop::new()?;
    let mut app = RainbowCatApp::new(phase.clone());
    event_loop.run_app(&mut app)?;
    running.store(false, Ordering::Relaxed);

    match reader_thread.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => Err("ipc reader panicked".into()),
    }
}

struct RainbowCatApp {
    phase: Arc<Mutex<f32>>,
    state: Option<State>,
}

impl RainbowCatApp {
    fn new(phase: Arc<Mutex<f32>>) -> Self {
        Self { phase, state: None }
    }
}

impl ApplicationHandler for RainbowCatApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Rainbow Cat IPC (Server Render)")
                    .with_inner_size(winit::dpi::PhysicalSize::new(WIDTH, HEIGHT)),
            )
            .expect("create window");
        self.state = Some(State::new(window, self.phase.clone()));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
                state.window.request_redraw();
            }
            WindowEvent::RedrawRequested => state.render(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_ref() {
            state.window.request_redraw();
        }
    }
}

struct State {
    window: Window,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    phase: Arc<Mutex<f32>>,
}

impl State {
    fn new(window: Window, phase: Arc<Mutex<f32>>) -> Self {
        let instance = create_instance(&wgpu::InstanceDescriptor::default());
        let surface = unsafe {
            let target =
                wgpu::SurfaceTargetUnsafe::from_window(&window).expect("create surface target");
            instance
                .create_surface_unsafe(target)
                .expect("create surface")
        };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("request adapter");
        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("rainbow-cat-device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .expect("request device");

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("rainbow-cat-shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rainbow-cat-uniform"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rainbow-cat-bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rainbow-cat-bg"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("rainbow-cat-layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rainbow-cat-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            pipeline,
            uniform_buffer,
            bind_group,
            phase,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self) {
        let phase = self.phase.lock().map(|v| *v).unwrap_or_default();
        let mut uniform = [0u8; 16];
        uniform[..4].copy_from_slice(&phase.to_le_bytes());
        self.queue.write_buffer(&self.uniform_buffer, 0, &uniform);

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Timeout) => return,
            Err(SurfaceError::Outdated | SurfaceError::Lost) => {
                let size = self.window.inner_size();
                self.resize(size.width, size.height);
                return;
            }
            Err(err) => panic!("surface error: {err:?}"),
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("rainbow-cat-encoder"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("rainbow-cat-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

fn spawn_server(sock_name: &str) -> Result<Child, Box<dyn Error>> {
    let current = env::current_exe()?;
    let child = Command::new(current)
        .arg("--ipc-server")
        .arg(sock_name)
        .spawn()?;
    Ok(child)
}

fn connect_with_retry(
    name: interprocess::local_socket::Name<'static>,
) -> Result<Stream, Box<dyn Error>> {
    for _ in 0..300 {
        match Stream::connect(name.clone()) {
            Ok(stream) => return Ok(stream),
            Err(_) => thread::sleep(Duration::from_millis(10)),
        }
    }
    Err("failed to connect ipc server".into())
}

fn to_ipc_name(name: &str) -> Result<interprocess::local_socket::Name<'static>, Box<dyn Error>> {
    if GenericNamespaced::is_supported() {
        Ok(name.to_string().to_ns_name::<GenericNamespaced>()?)
    } else {
        Ok(format!("/tmp/{name}.sock").to_fs_name::<GenericFilePath>()?)
    }
}

#[derive(Debug)]
enum IpcMessage {
    AnimationPhase { phase: f32 },
    Shutdown,
}

fn write_message<W: Write>(writer: &mut W, message: &IpcMessage) -> std::io::Result<()> {
    match message {
        IpcMessage::AnimationPhase { phase } => {
            writer.write_all(&[1])?;
            writer.write_all(&phase.to_le_bytes())?;
        }
        IpcMessage::Shutdown => writer.write_all(&[2])?,
    }
    Ok(())
}

fn read_message<R: Read>(reader: &mut R) -> std::io::Result<IpcMessage> {
    let mut tag = [0u8; 1];
    reader.read_exact(&mut tag)?;
    match tag[0] {
        1 => {
            let mut b = [0u8; 4];
            reader.read_exact(&mut b)?;
            Ok(IpcMessage::AnimationPhase {
                phase: f32::from_le_bytes(b),
            })
        }
        2 => Ok(IpcMessage::Shutdown),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unknown ipc message",
        )),
    }
}
