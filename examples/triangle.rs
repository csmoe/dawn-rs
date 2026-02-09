#[cfg(target_os = "macos")]
use objc2::rc::autoreleasepool;
#[cfg(target_os = "macos")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
#[cfg(target_os = "macos")]
use raw_window_metal::Layer;
#[cfg(target_os = "macos")]
use std::time::Instant;
#[cfg(target_os = "macos")]
use winit::application::ApplicationHandler;
#[cfg(target_os = "macos")]
use winit::event::WindowEvent;
#[cfg(target_os = "macos")]
use winit::event_loop::{ActiveEventLoop, EventLoop};
#[cfg(target_os = "macos")]
use winit::window::{Window, WindowAttributes, WindowId};

static SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(input.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.2, 0.3, 1.0);
}
"#;

#[cfg(target_os = "macos")]
fn main() {
    autoreleasepool(|_| {
        let event_loop = EventLoop::new().expect("create event loop");

        let mut app = TriangleApp::new(SHADER);

        event_loop.run_app(&mut app).expect("run app");
    });
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("triangle example is only implemented for macOS");
}

trait InstanceExt {
    fn create_adapter_for_rendering(&self) -> dawn_rs::Adapter;
}

impl InstanceExt for dawn_rs::Instance {
    fn create_adapter_for_rendering(&self) -> dawn_rs::Adapter {
        let adapter = self.request_adapter_sync();
        adapter
    }
}

trait InstanceRequestAdapterSync {
    fn request_adapter_sync(&self) -> dawn_rs::Adapter;
}

struct SendAdapter(dawn_rs::Adapter);
unsafe impl Send for SendAdapter {}

struct TriangleApp {
    shader_source: String,
    window: Option<Window>,
    instance: Option<dawn_rs::Instance>,
    surface: Option<dawn_rs::Surface>,
    device: Option<dawn_rs::Device>,
    queue: Option<dawn_rs::Queue>,
    pipeline: Option<dawn_rs::RenderPipeline>,
    config: Option<dawn_rs::SurfaceConfiguration>,
    vertex_buffer: Option<dawn_rs::Buffer>,
    vertex_buffer_size: u64,
    metal_layer: Option<Layer>,
    last_redraw: Instant,
}

impl TriangleApp {
    fn new(shader_source: &str) -> Self {
        Self {
            shader_source: shader_source.to_string(),
            window: None,
            instance: None,
            surface: None,
            device: None,
            queue: None,
            pipeline: None,
            config: None,
            vertex_buffer: None,
            vertex_buffer_size: 0,
            metal_layer: None,
            last_redraw: Instant::now(),
        }
    }

    fn init(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default().with_title("Dawn Triangle"))
            .expect("create window");

        let window_handle = window.window_handle().expect("window handle");
        let layer = match window_handle.as_raw() {
            RawWindowHandle::AppKit(handle) => unsafe { Layer::from_ns_view(handle.ns_view) },
            _ => panic!("expected AppKit window handle"),
        };
        let layer_ptr = layer.as_ptr().as_ptr();

        let instance = dawn_rs::Instance::new(None);
        let mut surface_desc = dawn_rs::SurfaceDescriptor::new();
        let mut metal_layer = dawn_rs::SurfaceSourceMetalLayer::new();
        metal_layer.layer = Some(layer_ptr.cast());
        surface_desc.push_extension(
            dawn_rs::SurfaceDescriptorExtension::SurfaceSourceMetalLayer(metal_layer),
        );
        let surface = instance.create_surface(&surface_desc);

        let adapter = instance.create_adapter_for_rendering();
        let device = adapter.create_device(None);
        let queue = device.get_queue();

        let vertex_data: [f32; 6] = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];
        let vertex_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                vertex_data.as_ptr().cast::<u8>(),
                vertex_data.len() * std::mem::size_of::<f32>(),
            )
        };
        let mut vertex_buffer_desc = dawn_rs::BufferDescriptor::new();
        vertex_buffer_desc.size = Some(vertex_bytes.len() as u64);
        vertex_buffer_desc.usage =
            Some(dawn_rs::BufferUsage::VERTEX | dawn_rs::BufferUsage::COPY_DST);
        let vertex_buffer = device
            .create_buffer(&vertex_buffer_desc)
            .expect("create vertex buffer");
        let upload_encoder = device.create_command_encoder(None);
        upload_encoder.write_buffer(vertex_buffer.clone(), 0, vertex_bytes);
        let upload_commands = upload_encoder.finish(None);
        queue.submit(&[upload_commands]);

        let mut capabilities = dawn_rs::SurfaceCapabilities::new();
        let _ = surface.get_capabilities(adapter.clone(), &mut capabilities);
        let format = capabilities
            .formats
            .as_ref()
            .and_then(|formats| {
                formats
                    .iter()
                    .copied()
                    .find(|f| *f == dawn_rs::TextureFormat::Bgra8UnormSrgb)
                    .or_else(|| formats.first().copied())
            })
            .unwrap_or(dawn_rs::TextureFormat::Bgra8Unorm);
        let present_mode = capabilities
            .present_modes
            .as_ref()
            .and_then(|modes| {
                modes
                    .iter()
                    .copied()
                    .find(|m| *m == dawn_rs::PresentMode::Fifo)
                    .or_else(|| modes.first().copied())
            })
            .unwrap_or(dawn_rs::PresentMode::Fifo);
        let alpha_mode = capabilities
            .alpha_modes
            .as_ref()
            .and_then(|modes| {
                modes
                    .iter()
                    .copied()
                    .find(|m| *m == dawn_rs::CompositeAlphaMode::Auto)
                    .or_else(|| modes.first().copied())
            })
            .unwrap_or(dawn_rs::CompositeAlphaMode::Auto);

        let size = window.inner_size();
        let mut config = dawn_rs::SurfaceConfiguration::new();
        config.device = Some(device.clone());
        config.format = Some(format);
        config.usage = Some(dawn_rs::TextureUsage::RENDER_ATTACHMENT);
        config.width = Some(size.width);
        config.height = Some(size.height);
        config.present_mode = Some(present_mode);
        config.alpha_mode = Some(alpha_mode);
        surface.configure(&config);

        let shader_source = dawn_rs::ShaderSourceWGSL {
            code: Some(self.shader_source.clone()),
        };
        let mut shader_desc = dawn_rs::ShaderModuleDescriptor::new();
        shader_desc.push_extension(dawn_rs::ShaderModuleDescriptorExtension::ShaderSourceWGSL(
            shader_source,
        ));
        let shader = device.create_shader_module(&shader_desc);
        let (shader_tx, shader_rx) = std::sync::mpsc::channel::<String>();
        let _future = shader.get_compilation_info(move |status, info| {
            if status != dawn_rs::CompilationInfoRequestStatus::Success {
                let _ = shader_tx.send(format!("Shader compilation info status: {:?}", status));
                return;
            }
            let mut output = String::new();
            if let Some(messages) = &info.messages {
                for message in messages.iter() {
                    let line = message.line_num.unwrap_or(0);
                    let col = message.line_pos.unwrap_or(0);
                    let kind = message
                        .r#type
                        .unwrap_or(dawn_rs::CompilationMessageType::Info);
                    let text = message.message.clone().unwrap_or_default();
                    output.push_str(&format!("[{:?}] {}:{} {}\n", kind, line, col, text));
                }
            }
            let _ = shader_tx.send(output);
        });
        loop {
            match shader_rx.try_recv() {
                Ok(output) => {
                    if !output.trim().is_empty() {
                        eprintln!("Shader compilation messages:\n{}", output);
                    }
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    instance.process_events();
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }
        }

        let mut vertex_state = dawn_rs::VertexState::new();
        vertex_state.module = Some(shader.clone());
        vertex_state.entry_point = Some("vs_main".to_string());
        let mut vertex_attribute = dawn_rs::VertexAttribute::new();
        vertex_attribute.format = Some(dawn_rs::VertexFormat::Float32X2);
        vertex_attribute.offset = Some(0);
        vertex_attribute.shader_location = Some(0);
        let mut vertex_buffer_layout = dawn_rs::VertexBufferLayout::new();
        vertex_buffer_layout.array_stride = Some(8);
        vertex_buffer_layout.step_mode = Some(dawn_rs::VertexStepMode::Vertex);
        vertex_buffer_layout.attributes = Some(vec![vertex_attribute]);
        vertex_state.buffers = Some(vec![vertex_buffer_layout]);

        let mut fragment_state = dawn_rs::FragmentState::new();
        fragment_state.module = Some(shader);
        fragment_state.entry_point = Some("fs_main".to_string());
        let mut color_target = dawn_rs::ColorTargetState::new();
        color_target.format = Some(format);
        fragment_state.targets = Some(vec![color_target]);

        let mut pipeline_desc = dawn_rs::RenderPipelineDescriptor::new();
        pipeline_desc.vertex = Some(vertex_state);
        pipeline_desc.fragment = Some(fragment_state);
        pipeline_desc.primitive = Some(dawn_rs::PrimitiveState::new());
        pipeline_desc.multisample = Some(dawn_rs::MultisampleState::new());
        let pipeline = device.create_render_pipeline(&pipeline_desc);
        let _future =
            device.create_render_pipeline_async(&pipeline_desc, |status, _pipeline, message| {
                if status != dawn_rs::CreatePipelineAsyncStatus::Success {
                    eprintln!("Pipeline async error: {:?}: {}", status, message);
                }
            });

        self.window = Some(window);
        self.instance = Some(instance);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.pipeline = Some(pipeline);
        self.config = Some(config);
        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_buffer_size = vertex_bytes.len() as u64;
        self.metal_layer = Some(layer);
    }

    fn redraw(&mut self) {
        let (surface, device, queue, pipeline, config, vertex_buffer, vertex_buffer_size) = match (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.queue.as_ref(),
            self.pipeline.as_ref(),
            self.config.as_ref(),
            self.vertex_buffer.as_ref(),
        ) {
            (
                Some(surface),
                Some(device),
                Some(queue),
                Some(pipeline),
                Some(config),
                Some(vertex_buffer),
            ) => (
                surface,
                device,
                queue,
                pipeline,
                config,
                vertex_buffer,
                self.vertex_buffer_size,
            ),
            _ => return,
        };

        let mut surface_texture = dawn_rs::SurfaceTexture::new();
        surface.get_current_texture(&mut surface_texture);
        match surface_texture.status {
            Some(
                dawn_rs::SurfaceGetCurrentTextureStatus::SuccessOptimal
                | dawn_rs::SurfaceGetCurrentTextureStatus::SuccessSuboptimal,
            ) => {}
            Some(
                dawn_rs::SurfaceGetCurrentTextureStatus::Outdated
                | dawn_rs::SurfaceGetCurrentTextureStatus::Lost,
            ) => {
                surface.configure(config);
                return;
            }
            _ => {
                return;
            }
        }
        let texture = match surface_texture.texture {
            Some(tex) => tex,
            None => return,
        };
        let view = texture.create_view(None);

        let mut color_attachment = dawn_rs::RenderPassColorAttachment::new();
        color_attachment.view = Some(view);
        color_attachment.load_op = Some(dawn_rs::LoadOp::Clear);
        color_attachment.store_op = Some(dawn_rs::StoreOp::Store);
        color_attachment.clear_value = Some(dawn_rs::Color {
            r: Some(0.1),
            g: Some(0.1),
            b: Some(0.12),
            a: Some(1.0),
        });

        let mut render_pass_desc = dawn_rs::RenderPassDescriptor::new();
        render_pass_desc.color_attachments = Some(vec![color_attachment]);

        let encoder = device.create_command_encoder(None);
        let pass = encoder.begin_render_pass(&render_pass_desc);
        pass.set_pipeline(pipeline.clone());
        pass.set_vertex_buffer(0, Some(vertex_buffer.clone()), 0, vertex_buffer_size);
        pass.draw(3, 1, 0, 0);
        pass.end();

        let command_buffer = encoder.finish(None);
        queue.submit(&[command_buffer]);
        let present_status = surface.present();
        if present_status != dawn_rs::Status::Success {
            eprintln!("Surface present status: {:?}", present_status);
        }
        let _ = config;
    }

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        let Some(surface) = self.surface.as_ref() else {
            return;
        };
        let Some(config) = self.config.as_mut() else {
            return;
        };
        config.width = Some(size.width);
        config.height = Some(size.height);
        surface.configure(config);
    }
}

impl ApplicationHandler for TriangleApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.init(event_loop);
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if window.id() != id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                _event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.resize(size);
            }
            WindowEvent::RedrawRequested => {
                self.redraw();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            if self.last_redraw.elapsed().as_millis() > 16 {
                window.request_redraw();
                self.last_redraw = Instant::now();
            }
        }
    }
}
impl InstanceRequestAdapterSync for dawn_rs::Instance {
    fn request_adapter_sync(&self) -> dawn_rs::Adapter {
        let (tx, rx) = std::sync::mpsc::channel::<Result<SendAdapter, String>>();
        let _future = self.request_adapter(None, move |status, adapter, message| {
            if status != dawn_rs::RequestAdapterStatus::Success {
                let _ = tx.send(Err(message));
                return;
            }
            let adapter = match adapter {
                Some(adapter) => adapter,
                None => {
                    let _ = tx.send(Err("No adapter".to_string()));
                    return;
                }
            };
            let _ = tx.send(Ok(SendAdapter(adapter)));
        });
        loop {
            match rx.try_recv() {
                Ok(Ok(adapter)) => return adapter.0,
                Ok(Err(message)) => panic!("Request adapter failed: {}", message),
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    self.process_events();
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    panic!("Request adapter callback disconnected");
                }
            }
        }
    }
}
