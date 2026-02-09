#[cfg(target_os = "macos")]
fn main() {
    use dawn_rs::*;
    use objc2::rc::autoreleasepool;
    use objc2::runtime::AnyObject;
    use objc2::{class, msg_send};
    use std::time::Instant;
    use winit::event::{Event, WindowEvent};
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::WindowBuilder;
    use winit::platform::macos::WindowExtMacOS;

    autoreleasepool(|_| {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Dawn Triangle")
            .build(&event_loop)
            .expect("create window");

        let ns_view = window.ns_view() as *mut AnyObject;
        let layer: *mut AnyObject = unsafe { msg_send![class!(CAMetalLayer), layer] };
        unsafe {
            let _: () = msg_send![ns_view, setWantsLayer: true];
            let _: () = msg_send![ns_view, setLayer: layer];
        }

        let instance = Instance::new(None);
        let mut surface_desc = SurfaceDescriptor::new();
        let mut metal_layer = SurfaceSourceMetalLayer::new();
        metal_layer.layer = Some(layer.cast());
        surface_desc.push_extension(SurfaceDescriptorExtension::SurfaceSourceMetalLayer(
            metal_layer,
        ));
        let surface = instance.create_surface(&surface_desc);

        let adapter = instance.create_adapter_for_rendering();
        let device = adapter.create_device(None);
        let queue = device.get_queue();

        let size = window.inner_size();
        let mut config = SurfaceConfiguration::new();
        config.device = Some(device.clone());
        config.format = Some(TextureFormat::Bgra8Unorm);
        config.usage = Some(TextureUsage::RENDER_ATTACHMENT);
        config.width = Some(size.width);
        config.height = Some(size.height);
        surface.configure(&config);

        let shader_source = ShaderSourceWGSL {
            code: Some(
                r#"
@vertex
fn vs_main(@builtin(vertex_index) index : u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    let pos = positions[index];
    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.2, 0.3, 1.0);
}
"#
                .to_string(),
            ),
        };
        let mut shader_desc = ShaderModuleDescriptor::new();
        shader_desc.push_extension(ShaderModuleDescriptorExtension::ShaderSourceWGSL(
            shader_source,
        ));
        let shader = device.create_shader_module(&shader_desc);

        let mut vertex_state = VertexState::new();
        vertex_state.module = Some(shader.clone());
        vertex_state.entry_point = Some("vs_main".to_string());

        let mut fragment_state = FragmentState::new();
        fragment_state.module = Some(shader);
        fragment_state.entry_point = Some("fs_main".to_string());
        let mut color_target = ColorTargetState::new();
        color_target.format = Some(TextureFormat::Bgra8Unorm);
        fragment_state.targets = Some(vec![color_target]);

        let mut pipeline_desc = RenderPipelineDescriptor::new();
        pipeline_desc.vertex = Some(vertex_state);
        pipeline_desc.fragment = Some(fragment_state);
        pipeline_desc.primitive = Some(PrimitiveState::new());
        pipeline_desc.multisample = Some(MultisampleState::new());
        let pipeline = device.create_render_pipeline(&pipeline_desc);

        let mut last_redraw = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        config.width = Some(size.width);
                        config.height = Some(size.height);
                        surface.configure(&config);
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    if last_redraw.elapsed().as_millis() > 16 {
                        window.request_redraw();
                        last_redraw = Instant::now();
                    }
                }
                Event::RedrawRequested(_) => {
                    let mut surface_texture = SurfaceTexture::new();
                    surface.get_current_texture(&mut surface_texture);
                    let texture = match surface_texture.texture {
                        Some(tex) => tex,
                        None => return,
                    };
                    let view = texture.create_view(None);

                    let mut color_attachment = RenderPassColorAttachment::new();
                    color_attachment.view = Some(view);
                    color_attachment.load_op = Some(LoadOp::Clear);
                    color_attachment.store_op = Some(StoreOp::Store);
                    color_attachment.clear_value = Some(Color {
                        r: Some(0.1),
                        g: Some(0.1),
                        b: Some(0.12),
                        a: Some(1.0),
                    });

                    let mut render_pass_desc = RenderPassDescriptor::new();
                    render_pass_desc.color_attachments = Some(vec![color_attachment]);

                    let encoder = device.create_command_encoder(None);
                    let pass = encoder.begin_render_pass(&render_pass_desc);
                    pass.set_pipeline(pipeline.clone());
                    pass.draw(3, 1, 0, 0);
                    pass.end();

                    let command_buffer = encoder.finish(None);
                    queue.submit(&[command_buffer]);
                    let _ = surface.present();
                }
                _ => {}
            }
        });
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
