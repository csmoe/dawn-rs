use dawn_rs::wire_shim::{
    DawnRsWireHandle, WireClientShim, WireServerShim, clear_procs, set_client_procs,
    set_native_procs,
};
use dawn_rs::wire_types::{WireCommand, WirePacket, WireValue, build_packet};
use dawn_rs::{
    BufferDescriptor, BufferUsage, Color, Device, Extent3D, FragmentState, Instance, LoadOp,
    MapAsyncStatus, MapMode, MultisampleState, PipelineLayoutDescriptor,
    PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor,
    RequestAdapterStatus, ShaderModuleDescriptor, ShaderSourceWGSL, StoreOp, TexelCopyBufferInfo,
    TexelCopyBufferLayout, TexelCopyTextureInfo, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsage, VertexState,
};
use interprocess::TryClone;
use interprocess::local_socket::{
    GenericFilePath, GenericNamespaced, ListenerOptions, Stream, prelude::*,
};
use serde::{Deserialize, Serialize};
#[cfg(target_os = "macos")]
use raw_window_metal::Layer;
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
#[cfg(all(unix, not(target_os = "macos")))]
use winit::raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowAttributes, WindowId};

const SHADER: &str = r#"
@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.65),
        vec2<f32>(-0.65, -0.55),
        vec2<f32>(0.65, -0.55)
    );
    return vec4<f32>(pos[idx], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.15, 0.75, 0.35, 1.0);
}
"#;

fn main() {
    if let Err(e) = run() {
        eprintln!("triangle-ipc error: {e}");
        std::process::exit(1);
    }
}

struct ProcTableGuard;

impl ProcTableGuard {
    fn client() -> Self {
        set_client_procs();
        Self
    }

    fn native() -> Self {
        set_native_procs();
        Self
    }
}

impl Drop for ProcTableGuard {
    fn drop(&mut self) {
        clear_procs();
    }
}

struct ChildGuard {
    child: Option<Child>,
}

impl ChildGuard {
    fn new(child: Child) -> Self {
        Self { child: Some(child) }
    }

    fn wait_success(mut self) -> Result<(), Box<dyn Error>> {
        let status = self.child.take().ok_or("child already taken")?.wait()?;
        if !status.success() {
            return Err(format!("server exited with status {status}").into());
        }
        Ok(())
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

struct PumpGuard {
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::Sender<WirePacket>>,
    writer_thread: Option<JoinHandle<Result<(), String>>>,
    reader_thread: Option<JoinHandle<Result<(), String>>>,
}

impl PumpGuard {
    fn new(
        stop: Arc<AtomicBool>,
        tx: mpsc::Sender<WirePacket>,
        writer_thread: JoinHandle<Result<(), String>>,
        reader_thread: JoinHandle<Result<(), String>>,
    ) -> Self {
        Self {
            stop,
            tx: Some(tx),
            writer_thread: Some(writer_thread),
            reader_thread: Some(reader_thread),
        }
    }

    fn shutdown_and_join(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.tx.take();
        if let Some(writer_thread) = self.writer_thread.take() {
            match writer_thread.join() {
                Ok(Err(e)) => eprintln!("wire writer thread error: {e}"),
                Err(_) => eprintln!("wire writer thread panic"),
                Ok(Ok(())) => {}
            }
        }
        if let Some(reader_thread) = self.reader_thread.take() {
            match reader_thread.join() {
                Ok(Err(e)) => eprintln!("wire reader thread error: {e}"),
                Err(_) => eprintln!("wire reader thread panic"),
                Ok(Ok(())) => {}
            }
        }
    }
}

impl Drop for PumpGuard {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.tx.take();
        if let Some(writer_thread) = self.writer_thread.take() {
            let _ = writer_thread.join();
        }
        if let Some(reader_thread) = self.reader_thread.take() {
            let _ = reader_thread.join();
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    if let Some(mode) = args.next() {
        if mode == "--ipc-server" {
            let name = args.next().ok_or("missing socket name for --ipc-server")?;
            let width = args
                .next()
                .ok_or("missing width for --ipc-server")?
                .parse::<u32>()?;
            let height = args
                .next()
                .ok_or("missing height for --ipc-server")?
                .parse::<u32>()?;
            return run_server(&name, width, height);
        }
    }
    run_client()
}

fn run_client() -> Result<(), Box<dyn Error>> {
    let width = 512u32;
    let height = 512u32;
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let sock_name = format!("dawn-rs-triangle-wire-{stamp}");

    let current_exe = env::current_exe()?;
    let child = Command::new(current_exe)
        .arg("--ipc-server")
        .arg(&sock_name)
        .arg(width.to_string())
        .arg(height.to_string())
        .spawn()?;
    let child = ChildGuard::new(child);

    let name = to_ipc_name(&sock_name)?;
    let stream = connect_with_retry(name)?;
    let mut reader_stream = stream.try_clone()?;
    let mut writer_stream = stream.try_clone()?;
    let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));
    let _ = writer_stream.set_send_timeout(Some(Duration::from_millis(200)));

    let _procs_guard = ProcTableGuard::client();

    let (to_writer_tx, to_writer_rx) = mpsc::channel::<WirePacket>();
    let chunk_id = Arc::new(std::sync::atomic::AtomicU64::new(1));
    let client = Arc::new(Mutex::new(WireClientShim::new(
        0,
        {
            let to_writer_tx = to_writer_tx.clone();
            let chunk_id = chunk_id.clone();
            move |bytes: &[u8]| {
                let id = chunk_id.fetch_add(1, Ordering::Relaxed);
                if let Ok(packet) = build_chunked_packet(id, bytes) {
                    let _ = to_writer_tx.send(packet);
                }
            }
        },
        |msg: &str| eprintln!("wire client error: {msg}"),
    )?));

    let reserved = {
        let mut guard = client.lock().map_err(|_| "wire client lock poisoned")?;
        guard.reserve_instance()
    };
    if reserved.instance.is_null() {
        return Err("wire client reserve_instance returned null instance".into());
    }

    write_u32(&mut writer_stream, reserved.handle.id)?;
    write_u32(&mut writer_stream, reserved.handle.generation)?;
    writer_stream.flush()?;

    let mut ack = [0u8; 1];
    reader_stream.read_exact(&mut ack)?;
    if ack[0] != 1 {
        return Err("server rejected wire instance injection".into());
    }

    let stop = Arc::new(AtomicBool::new(false));
    let writer_stop = stop.clone();
    let writer_thread = thread::spawn(move || -> Result<(), String> {
        loop {
            match to_writer_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(packet) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        break;
                    }
                    write_packet(&mut writer_stream, &packet).map_err(|e| e.to_string())?;
                    writer_stream.flush().map_err(|e| e.to_string())?;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        break;
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
        Ok(())
    });

    let reader_stop = stop.clone();
    let client_for_reader = client.clone();
    let reader_thread = thread::spawn(move || -> Result<(), String> {
        while !reader_stop.load(Ordering::Relaxed) {
            let packet = match read_packet(&mut reader_stream) {
                Ok(packet) => packet,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    continue;
                }
                Err(e) => return Err(e.to_string()),
            };
            let frame = decode_chunked_packet(packet)?;
            let mut guard = client_for_reader
                .lock()
                .map_err(|_| "wire client lock poisoned".to_string())?;
            if !guard.handle_commands(&frame) {
                return Err("wire client failed to handle commands".to_string());
            }
        }
        Ok(())
    });
    let flusher_stop = stop.clone();
    let client_for_flusher = client.clone();
    let flusher_thread = thread::spawn(move || -> Result<(), String> {
        while !flusher_stop.load(Ordering::Relaxed) {
            {
                let mut guard = client_for_flusher
                    .lock()
                    .map_err(|_| "wire client lock poisoned".to_string())?;
                let _ = guard.flush();
            }
            thread::sleep(Duration::from_millis(1));
        }
        Ok(())
    });

    let wire_instance = unsafe { WireClientShim::reserved_instance_to_instance(reserved) };

    let pump = PumpGuard::new(stop, to_writer_tx, writer_thread, reader_thread);

    let (checksum, nonzero_pixels) =
        render_triangle_rgba_with_instance(&wire_instance, width, height)?;
    println!(
        "triangle rendered via wire+ipc: {width}x{height}, checksum={checksum}, nonzero_pixels={nonzero_pixels}"
    );

    {
        let mut guard = client.lock().map_err(|_| "wire client lock poisoned")?;
        let _ = guard.flush();
        guard.disconnect();
    }
    drop(stream);
    pump.shutdown_and_join();
    match flusher_thread.join() {
        Ok(Err(e)) => eprintln!("wire flusher thread error: {e}"),
        Err(_) => eprintln!("wire flusher thread panic"),
        Ok(Ok(())) => {}
    }
    child.wait_success()?;
    Ok(())
}

fn run_server(sock_name: &str, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let name = to_ipc_name(sock_name)?;
    let listener = ListenerOptions::new()
        .name(name)
        .reclaim_name(true)
        .create_sync()?;

    let stream = listener.accept()?;
    let mut reader_stream = stream.try_clone()?;
    let mut writer_stream = stream.try_clone()?;
    let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));
    let _ = writer_stream.set_send_timeout(Some(Duration::from_millis(200)));

    let (to_writer_tx, to_writer_rx) = mpsc::channel::<WirePacket>();
    let chunk_id = Arc::new(std::sync::atomic::AtomicU64::new(1));
    let server = Arc::new(Mutex::new(WireServerShim::new_native(
        0,
        true,
        {
            let to_writer_tx = to_writer_tx.clone();
            let chunk_id = chunk_id.clone();
            move |bytes: &[u8]| {
                let id = chunk_id.fetch_add(1, Ordering::Relaxed);
                if let Ok(packet) = build_chunked_packet(id, bytes) {
                    let _ = to_writer_tx.send(packet);
                }
            }
        },
        |msg: &str| eprintln!("wire server error: {msg}"),
    )?));

    let _procs_guard = ProcTableGuard::native();
    let native_instance = Instance::new(None);

    let handle = DawnRsWireHandle {
        id: read_u32(&mut reader_stream)?,
        generation: read_u32(&mut reader_stream)?,
    };
    let injected = {
        let mut guard = server.lock().map_err(|_| "wire server lock poisoned")?;
        guard.inject_instance(native_instance.as_raw().cast(), handle)
    };
    writer_stream.write_all(&[if injected { 1 } else { 0 }])?;
    writer_stream.flush()?;
    if !injected {
        return Err("failed to inject native instance into wire server".into());
    }

    let stop = Arc::new(AtomicBool::new(false));
    let writer_stop = stop.clone();
    let writer_thread = thread::spawn(move || -> Result<(), String> {
        loop {
            match to_writer_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(packet) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        break;
                    }
                    write_packet(&mut writer_stream, &packet).map_err(|e| e.to_string())?;
                    writer_stream.flush().map_err(|e| e.to_string())?;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        break;
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
        Ok(())
    });

    let reader_stop = stop.clone();
    let server_for_reader = server.clone();
    let reader_thread = thread::spawn(move || -> Result<(), String> {
        while !reader_stop.load(Ordering::Relaxed) {
            let packet = match read_packet(&mut reader_stream) {
                Ok(packet) => packet,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    continue;
                }
                Err(e) => return Err(e.to_string()),
            };
            let frame = decode_chunked_packet(packet)?;
            let mut guard = server_for_reader
                .lock()
                .map_err(|_| "wire server lock poisoned".to_string())?;
            if !guard.handle_commands(&frame) {
                return Err("wire server failed to handle commands".to_string());
            }
            let _ = guard.flush();
        }
        Ok(())
    });

    let pump = PumpGuard::new(stop, to_writer_tx, writer_thread, reader_thread);
    render_triangle_window(width, height)?;
    drop(stream);
    pump.shutdown_and_join();
    Ok(())
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
    Err("failed to connect to ipc server".into())
}

fn request_adapter_sync(instance: &Instance) -> Result<dawn_rs::Adapter, String> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<dawn_rs::Adapter, String>>();
    let _future = instance.request_adapter(None, move |status, adapter, message| {
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
            Err(std::sync::mpsc::TryRecvError::Empty) => instance.process_events(),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err("request_adapter callback disconnected".to_string());
            }
        }
    }
}

fn request_device_sync(
    instance: &Instance,
    adapter: &dawn_rs::Adapter,
) -> Result<dawn_rs::Device, String> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<dawn_rs::Device, String>>();
    let _future = adapter.request_device(None, move |status, device, message| {
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
            Err(std::sync::mpsc::TryRecvError::Empty) => instance.process_events(),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err("request_device callback disconnected".to_string());
            }
        }
    }
}

fn render_triangle_rgba_with_instance(
    instance: &Instance,
    width: u32,
    height: u32,
) -> Result<(u64, u32), Box<dyn Error>> {
    let adapter = request_adapter_sync(instance)?;
    let device = request_device_sync(instance, &adapter)?;
    let queue = device.get_queue();

    let texture = create_render_target(&device, width, height);
    let texture_view = texture.create_view(None);

    let shader = create_shader(&device);
    let pipeline = create_pipeline(&device, shader, TextureFormat::Rgba8Unorm);

    let mut color_attachment = RenderPassColorAttachment::new();
    color_attachment.view = Some(texture_view);
    color_attachment.load_op = Some(LoadOp::Clear);
    color_attachment.store_op = Some(StoreOp::Store);
    color_attachment.clear_value = Some(Color {
        r: Some(0.0),
        g: Some(0.0),
        b: Some(0.0),
        a: Some(1.0),
    });

    let mut render_pass = RenderPassDescriptor::new();
    render_pass.color_attachments = Some(vec![color_attachment]);

    let bytes_per_row = align_up(width * 4, 256);
    let readback_size = u64::from(bytes_per_row) * u64::from(height);
    let mut readback_desc = BufferDescriptor::new();
    readback_desc.size = Some(readback_size);
    readback_desc.usage = Some(BufferUsage::COPY_DST | BufferUsage::MAP_READ);
    let readback = device
        .create_buffer(&readback_desc)
        .ok_or("failed to create readback buffer")?;

    let encoder = device.create_command_encoder(None);
    let pass = encoder.begin_render_pass(&render_pass);
    pass.set_pipeline(pipeline);
    pass.draw(3, 1, 0, 0);
    pass.end();

    let mut src = TexelCopyTextureInfo::new();
    src.texture = Some(texture);

    let mut layout = TexelCopyBufferLayout::new();
    layout.bytes_per_row = Some(bytes_per_row);
    layout.rows_per_image = Some(height);

    let mut dst = TexelCopyBufferInfo::new();
    dst.layout = Some(layout);
    dst.buffer = Some(readback.clone());

    let mut copy_size = Extent3D::new();
    copy_size.width = Some(width);
    copy_size.height = Some(height);
    copy_size.depth_or_array_layers = Some(1);

    encoder.copy_texture_to_buffer(&src, &dst, &copy_size);

    let commands = encoder.finish(None);
    queue.submit(&[commands]);

    let (tx, rx) = std::sync::mpsc::channel::<Result<(), String>>();
    let _future = readback.map_async(
        MapMode::READ,
        0,
        readback_size as usize,
        move |status, message| {
            if status == MapAsyncStatus::Success {
                let _ = tx.send(Ok(()));
            } else {
                let _ = tx.send(Err(format!("{status:?}: {message}")));
            }
        },
    );

    let started = Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(())) => break,
            Ok(Err(message)) => return Err(format!("map_async failed: {message}").into()),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                instance.process_events();
                if started.elapsed() > Duration::from_secs(30) {
                    return Err("map_async timed out".into());
                }
                thread::sleep(Duration::from_millis(1));
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err("map_async callback disconnected".into());
            }
        }
    }

    let data_ptr = readback
        .get_const_mapped_range(0, readback_size as usize)
        .cast::<u8>();
    if data_ptr.is_null() {
        return Err("readback buffer returned null mapping".into());
    }
    let mapped = unsafe { std::slice::from_raw_parts(data_ptr, readback_size as usize) };

    let mut checksum = 0u64;
    let mut nonzero_pixels = 0u32;
    for y in 0..height as usize {
        let row_start = y * bytes_per_row as usize;
        let row = &mapped[row_start..row_start + width as usize * 4];
        for px in row.chunks_exact(4) {
            checksum = checksum.wrapping_add(u64::from(px[0]));
            checksum = checksum.wrapping_add(u64::from(px[1]));
            checksum = checksum.wrapping_add(u64::from(px[2]));
            checksum = checksum.wrapping_add(u64::from(px[3]));
            if px[0] != 0 || px[1] != 0 || px[2] != 0 {
                nonzero_pixels = nonzero_pixels.saturating_add(1);
            }
        }
    }

    readback.unmap();
    Ok((checksum, nonzero_pixels))
}

fn render_triangle_window(width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = IpcTriangleWindowApp::new(width, height);
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct IpcTriangleWindowApp {
    width: u32,
    height: u32,
    window: Option<Window>,
    instance: Option<Instance>,
    surface: Option<dawn_rs::Surface>,
    device: Option<dawn_rs::Device>,
    queue: Option<dawn_rs::Queue>,
    pipeline: Option<dawn_rs::RenderPipeline>,
    config: Option<dawn_rs::SurfaceConfiguration>,
    #[cfg(target_os = "macos")]
    metal_layer: Option<Layer>,
    last_redraw: Instant,
}

impl IpcTriangleWindowApp {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            window: None,
            instance: None,
            surface: None,
            device: None,
            queue: None,
            pipeline: None,
            config: None,
            #[cfg(target_os = "macos")]
            metal_layer: None,
            last_redraw: Instant::now(),
        }
    }

    fn init(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Dawn Triangle IPC Server")
                    .with_inner_size(winit::dpi::PhysicalSize::new(self.width, self.height)),
            )
            .expect("create ipc window");
        let window_handle = window.window_handle().expect("window handle");
        #[cfg(all(unix, not(target_os = "macos")))]
        let display_handle = window.display_handle().expect("display handle");

        let instance = Instance::new(None);
        let mut surface_desc = dawn_rs::SurfaceDescriptor::new();

        #[cfg(target_os = "macos")]
        {
            let layer = match window_handle.as_raw() {
                RawWindowHandle::AppKit(handle) => unsafe { Layer::from_ns_view(handle.ns_view) },
                _ => panic!("expected AppKit window handle"),
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
                _ => panic!("expected Win32 window handle"),
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
                _ => panic!("unsupported Linux handle pair"),
            }
        }

        let surface = instance.create_surface(&surface_desc);
        let adapter = request_adapter_sync(&instance).expect("request adapter");
        let device = adapter.create_device(None);
        let queue = device.get_queue();

        let mut capabilities = dawn_rs::SurfaceCapabilities::new();
        let status = surface.get_capabilities(adapter, &mut capabilities);
        assert_eq!(status, dawn_rs::Status::Success);
        let format = capabilities
            .formats
            .as_ref()
            .and_then(|formats| {
                formats
                    .iter()
                    .copied()
                    .find(|f| *f == TextureFormat::Bgra8UnormSrgb)
                    .or_else(|| formats.first().copied())
            })
            .unwrap_or(TextureFormat::Bgra8Unorm);
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

        let mut config = dawn_rs::SurfaceConfiguration::new();
        config.device = Some(device.clone());
        config.format = Some(format);
        config.usage = Some(TextureUsage::RENDER_ATTACHMENT);
        config.width = Some(self.width);
        config.height = Some(self.height);
        config.present_mode = Some(present_mode);
        config.alpha_mode = Some(alpha_mode);
        surface.configure(&config);

        let shader = create_shader(&device);
        let pipeline = create_pipeline(&device, shader, format);

        self.window = Some(window);
        self.instance = Some(instance);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.pipeline = Some(pipeline);
        self.config = Some(config);
    }

    fn redraw(&mut self) {
        let (surface, device, queue, pipeline, config) = match (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.queue.as_ref(),
            self.pipeline.as_ref(),
            self.config.as_ref(),
        ) {
            (Some(s), Some(d), Some(q), Some(p), Some(c)) => (s, d, q, p, c),
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
            _ => return,
        }
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
            r: Some(0.08),
            g: Some(0.08),
            b: Some(0.10),
            a: Some(1.0),
        });
        let mut pass_desc = RenderPassDescriptor::new();
        pass_desc.color_attachments = Some(vec![color_attachment]);

        let encoder = device.create_command_encoder(None);
        let pass = encoder.begin_render_pass(&pass_desc);
        pass.set_pipeline(pipeline.clone());
        pass.draw(3, 1, 0, 0);
        pass.end();
        queue.submit(&[encoder.finish(None)]);
        let _ = surface.present();
    }
}

impl ApplicationHandler for IpcTriangleWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.init(event_loop);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if window.id() != id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.redraw(),
            WindowEvent::Resized(size) => {
                if let (Some(surface), Some(config)) = (self.surface.as_ref(), self.config.as_mut())
                {
                    config.width = Some(size.width);
                    config.height = Some(size.height);
                    surface.configure(config);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            if self.last_redraw.elapsed().as_millis() >= 16 {
                window.request_redraw();
                self.last_redraw = Instant::now();
            }
        }
    }
}

fn create_render_target(device: &Device, width: u32, height: u32) -> dawn_rs::Texture {
    let mut extent = Extent3D::new();
    extent.width = Some(width);
    extent.height = Some(height);
    extent.depth_or_array_layers = Some(1);

    let mut desc = TextureDescriptor::new();
    desc.dimension = Some(TextureDimension::D2);
    desc.format = Some(TextureFormat::Rgba8Unorm);
    desc.mip_level_count = Some(1);
    desc.sample_count = Some(1);
    desc.size = Some(extent);
    desc.usage = Some(TextureUsage::RENDER_ATTACHMENT | TextureUsage::COPY_SRC);
    device.create_texture(&desc)
}

fn create_shader(device: &Device) -> dawn_rs::ShaderModule {
    let mut desc = ShaderModuleDescriptor::new();
    desc = desc.with_extension(
        ShaderSourceWGSL {
            code: Some(SHADER.to_string()),
        }
        .into(),
    );
    device.create_shader_module(&desc)
}

fn create_pipeline(
    device: &Device,
    shader: dawn_rs::ShaderModule,
    format: TextureFormat,
) -> dawn_rs::RenderPipeline {
    let mut vertex = VertexState::new();
    vertex.module = Some(shader.clone());
    vertex.entry_point = Some("vs_main".to_string());

    let mut fragment = FragmentState::new();
    fragment.module = Some(shader);
    fragment.entry_point = Some("fs_main".to_string());

    let mut color_target = dawn_rs::ColorTargetState::new();
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

fn to_ipc_name(name: &str) -> Result<interprocess::local_socket::Name<'static>, Box<dyn Error>> {
    if GenericNamespaced::is_supported() {
        Ok(name.to_string().to_ns_name::<GenericNamespaced>()?)
    } else {
        let path = format!("/tmp/{name}.sock");
        Ok(path.to_fs_name::<GenericFilePath>()?)
    }
}

fn align_up(value: u32, align: u32) -> u32 {
    value.div_ceil(align) * align
}

fn write_u32<W: Write>(writer: &mut W, value: u32) -> Result<(), Box<dyn Error>> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

fn read_u32<R: Read>(reader: &mut R) -> Result<u32, Box<dyn Error>> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn build_chunked_packet(id: u64, chunk: &[u8]) -> Result<WirePacket, String> {
    build_packet(
        WireCommand::ChunkedCommand,
        vec![
            WireValue::U64(id),
            WireValue::U64(chunk.len() as u64),
            WireValue::Bytes(chunk.to_vec()),
            WireValue::U64(chunk.len() as u64),
        ],
    )
}

fn decode_chunked_packet(packet: WirePacket) -> Result<Vec<u8>, String> {
    if packet.command != WireCommand::ChunkedCommand {
        return Err("unexpected control packet for wire transport".to_string());
    }
    if packet.fields.len() != 4 {
        return Err("invalid chunked packet field count".to_string());
    }
    match &packet.fields[2].value {
        WireValue::Bytes(bytes) => Ok(bytes.clone()),
        _ => Err("invalid chunked packet bytes field".to_string()),
    }
}

fn write_packet<W: Write>(writer: &mut W, packet: &WirePacket) -> std::io::Result<()> {
    let encoded = postcard::to_stdvec(&packet_to_ipc(packet)?)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    let len = u32::try_from(encoded.len()).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "wire packet too large to encode",
        )
    })?;
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(&encoded)
}

fn read_packet<R: Read>(reader: &mut R) -> std::io::Result<WirePacket> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > 64 * 1024 * 1024 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "wire packet exceeds max size",
        ));
    }
    let mut data = vec![0u8; len];
    reader.read_exact(&mut data)?;
    let ipc: IpcPacket = postcard::from_bytes(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    ipc_to_packet(ipc)
}

fn parse_wire_command(name: &str) -> Option<WireCommand> {
    match name {
        "ChunkedCommand" => Some(WireCommand::ChunkedCommand),
        _ => None,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct IpcPacket {
    command: String,
    values: Vec<IpcWireValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum IpcWireValue {
    U64(u64),
    Bytes(Vec<u8>),
}

fn packet_to_ipc(packet: &WirePacket) -> std::io::Result<IpcPacket> {
    let mut values = Vec::with_capacity(packet.fields.len());
    for field in &packet.fields {
        let value = match &field.value {
            WireValue::U64(v) => IpcWireValue::U64(*v),
            WireValue::Bytes(v) => IpcWireValue::Bytes(v.clone()),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "unsupported wire value in postcard transport",
                ))
            }
        };
        values.push(value);
    }
    Ok(IpcPacket {
        command: format!("{:?}", packet.command),
        values,
    })
}

fn ipc_to_packet(ipc: IpcPacket) -> std::io::Result<WirePacket> {
    let command = parse_wire_command(&ipc.command)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "unknown wire command"))?;
    let values = ipc
        .values
        .into_iter()
        .map(|v| match v {
            IpcWireValue::U64(v) => WireValue::U64(v),
            IpcWireValue::Bytes(v) => WireValue::Bytes(v),
        })
        .collect();
    build_packet(command, values)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
