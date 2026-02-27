use dawn_rs::wire_ipc::{self, IpcMessage, OutboundPacket, read_message, write_message};
use dawn_rs::wire_shim::{WireHelperClient, WireHelperServer, WireInstanceHandle};
use dawn_rs::{
    Color, Instance, LoadOp, RenderPassColorAttachment, RenderPassDescriptor,
    RequestAdapterOptions, RequestAdapterStatus, SurfaceGetCurrentTextureStatus, SurfaceTexture,
    TextureUsage,
};
use interprocess::TryClone;
use interprocess::local_socket::traits::Stream as _;
#[cfg(target_os = "macos")]
use raw_window_metal::Layer;
use std::env;
use std::error::Error;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
#[cfg(all(unix, not(target_os = "macos")))]
use winit::raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowAttributes, WindowId};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

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
        return run_gpu_process(&socket_name, parent_pid);
    }
    run_browser_process()
}

fn run_browser_process() -> Result<(), Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let socket_name = format!("dawn-wgpu-rainbow-wire-{stamp}");
    let reconnect_delay = Duration::from_millis(400);
    loop {
        match run_browser_session(&socket_name) {
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

fn run_browser_session(socket_name: &str) -> Result<(), Box<dyn Error>> {
    let mut child = spawn_gpu_process(socket_name)?;
    let stream = wire_ipc::connect_with_retry(socket_name, 300, Duration::from_millis(10))?;
    let mut reader_stream = stream.try_clone()?;
    let mut writer_stream = stream.try_clone()?;
    let _ = writer_stream.set_send_timeout(Some(Duration::from_millis(200)));

    let (to_writer_tx, to_writer_rx) = mpsc::channel::<OutboundPacket>();
    let client = Arc::new(Mutex::new(WireHelperClient::new(
        0,
        {
            let to_writer_tx = to_writer_tx.clone();
            move |bytes: &[u8]| {
                let _ = to_writer_tx.send(OutboundPacket::Wire(bytes.to_vec()));
            }
        },
        |msg: &str| eprintln!("wire client error: {msg}"),
    )?));

    let reserved_instance = {
        let mut guard = client.lock().map_err(|_| "wire client lock poisoned")?;
        guard.reserve_instance()
    };
    if reserved_instance.instance.is_null() {
        return Err("reserve_instance returned null".into());
    }
    let reserved_surface = {
        let mut guard = client.lock().map_err(|_| "wire client lock poisoned")?;
        guard.reserve_surface(reserved_instance.instance)
    };
    if reserved_surface.surface.is_null() {
        return Err("reserve_surface returned null".into());
    }

    write_message(
        &mut writer_stream,
        &IpcMessage::ReserveInstance {
            id: reserved_instance.handle.id,
            generation: reserved_instance.handle.generation,
        },
    )?;
    write_message(
        &mut writer_stream,
        &IpcMessage::ReserveSurface {
            id: reserved_surface.handle.id,
            generation: reserved_surface.handle.generation,
            instance_id: reserved_surface.instance_handle.id,
            instance_generation: reserved_surface.instance_handle.generation,
        },
    )?;
    writer_stream.flush()?;
    match read_message(&mut reader_stream)? {
        IpcMessage::ReserveAck { ok } if ok => {}
        _ => return Err("server rejected wire object injection".into()),
    }
    let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));

    let stop = Arc::new(AtomicBool::new(false));
    let client_for_reader = client.clone();
    let (writer_thread, reader_thread) = wire_ipc::start_wire_threads(
        stop.clone(),
        reader_stream,
        writer_stream,
        to_writer_rx,
        32,
        Duration::from_millis(4),
        move |frame: &[u8]| {
            let mut guard = client_for_reader
                .lock()
                .map_err(|_| "wire client lock poisoned".to_string())?;
            if !guard.handle_commands(frame) {
                return Err("wire client failed to handle commands".to_string());
            }
            Ok(())
        },
        || {},
        |_| {},
    );

    let instance = unsafe { WireHelperClient::reserved_instance_to_instance(reserved_instance) };
    let surface = unsafe { WireHelperClient::reserved_surface_to_surface(reserved_surface) };

    let client_for_flush = client.clone();
    let mut flush_wire = move || {
        if let Ok(mut guard) = client_for_flush.lock() {
            let _ = guard.flush();
        }
    };

    let mut adapter_opts = RequestAdapterOptions::new();
    adapter_opts.compatible_surface = Some(surface.clone());
    let adapter = request_adapter_sync(&instance, Some(adapter_opts), &mut flush_wire)?;
    let device = request_device_sync(&instance, &adapter, &mut flush_wire)?;
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

    let start = Instant::now();
    while child.try_wait()?.is_none() {
        let phase = start.elapsed().as_secs_f32();
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
        let view = texture.create_view(None);

        let mut color_attachment = RenderPassColorAttachment::new();
        color_attachment.view = Some(view);
        color_attachment.load_op = Some(LoadOp::Clear);
        color_attachment.store_op = Some(dawn_rs::StoreOp::Store);
        let pulse = (phase * 2.0).sin() * 0.5 + 0.5;
        color_attachment.clear_value = Some(Color {
            r: Some(0.08 + 0.50 * pulse as f64),
            g: Some(0.04 + 0.35 * (1.0 - pulse as f64)),
            b: Some(0.12 + 0.45 * pulse as f64),
            a: Some(1.0),
        });
        let mut pass_desc = RenderPassDescriptor::new();
        pass_desc.color_attachments = Some(vec![color_attachment]);

        let encoder = device.create_command_encoder(None);
        let pass = encoder.begin_render_pass(&pass_desc);
        pass.end();
        queue.submit(&[encoder.finish(None)]);
        let _ = surface.present();

        flush_wire();
        instance.process_events();
        thread::sleep(Duration::from_millis(16));
    }

    stop.store(true, Ordering::Relaxed);
    let _ = to_writer_tx.send(OutboundPacket::Shutdown);
    let _ = writer_thread.join();
    let _ = reader_thread.join();
    if let Ok(mut guard) = client.lock() {
        let _ = guard.flush();
        guard.disconnect();
    }
    if child.try_wait()?.is_none() {
        let _ = child.kill();
        let _ = child.wait();
    }
    Ok(())
}

fn run_gpu_process(socket_name: &str, parent_pid: Option<u32>) -> Result<(), Box<dyn Error>> {
    let stream = wire_ipc::bind_and_accept(socket_name)?;
    let mut reader_stream = stream.try_clone()?;
    let writer_stream = stream.try_clone()?;

    let instance_handle = match read_message(&mut reader_stream)
        .map_err(|e| format!("gpu read ReserveInstance failed: {e}"))?
    {
        IpcMessage::ReserveInstance { id, generation } => WireInstanceHandle { id, generation },
        _ => return Err("expected ReserveInstance".into()),
    };
    let (surface_handle, surface_instance_handle) = match read_message(&mut reader_stream)
        .map_err(|e| format!("gpu read ReserveSurface failed: {e}"))?
    {
        IpcMessage::ReserveSurface {
            id,
            generation,
            instance_id,
            instance_generation,
        } => (
            WireInstanceHandle { id, generation },
            WireInstanceHandle {
                id: instance_id,
                generation: instance_generation,
            },
        ),
        _ => return Err("expected ReserveSurface".into()),
    };

    let event_loop = EventLoop::new()?;
    let mut app = GpuProcessApp::new(
        reader_stream,
        writer_stream,
        instance_handle,
        surface_handle,
        surface_instance_handle,
        parent_pid,
    );
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct PumpGuard {
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::Sender<OutboundPacket>>,
    writer_thread: Option<thread::JoinHandle<Result<(), String>>>,
    reader_thread: Option<thread::JoinHandle<Result<(), String>>>,
}

impl PumpGuard {
    fn new(
        stop: Arc<AtomicBool>,
        tx: mpsc::Sender<OutboundPacket>,
        writer_thread: thread::JoinHandle<Result<(), String>>,
        reader_thread: thread::JoinHandle<Result<(), String>>,
    ) -> Self {
        Self {
            stop,
            tx: Some(tx),
            writer_thread: Some(writer_thread),
            reader_thread: Some(reader_thread),
        }
    }
}

impl Drop for PumpGuard {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(OutboundPacket::Shutdown);
        }
        if let Some(handle) = self.writer_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
    }
}

struct GpuProcessApp {
    reader_stream: Option<interprocess::local_socket::Stream>,
    writer_stream: Option<interprocess::local_socket::Stream>,
    instance_handle: WireInstanceHandle,
    surface_handle: WireInstanceHandle,
    surface_instance_handle: WireInstanceHandle,
    window: Option<Window>,
    server: Option<Arc<Mutex<WireHelperServer>>>,
    _native_instance: Option<Instance>,
    _native_surface: Option<dawn_rs::Surface>,
    pump: Option<PumpGuard>,
    disconnect_rx: Option<mpsc::Receiver<()>>,
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
    ) -> Self {
        Self {
            reader_stream: Some(reader_stream),
            writer_stream: Some(writer_stream),
            instance_handle,
            surface_handle,
            surface_instance_handle,
            window: None,
            server: None,
            _native_instance: None,
            _native_surface: None,
            pump: None,
            disconnect_rx: None,
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
        let _ = writer_stream.set_send_timeout(Some(Duration::from_millis(200)));

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
        write_message(&mut writer_stream, &IpcMessage::ReserveAck { ok })?;
        writer_stream.flush()?;
        if !ok {
            return Err("failed to inject instance/surface into wire server".into());
        }

        let stop = Arc::new(AtomicBool::new(false));
        let server_for_reader = server.clone();
        let server_for_after = server.clone();
        let (writer_raw, reader_raw) = wire_ipc::start_wire_threads(
            stop.clone(),
            reader_stream,
            writer_stream,
            to_writer_rx,
            32,
            Duration::from_millis(4),
            move |frame: &[u8]| {
                let mut guard = server_for_reader
                    .lock()
                    .map_err(|_| "wire server lock poisoned".to_string())?;
                if !guard.handle_commands(frame) {
                    return Err("wire server failed to handle commands".to_string());
                }
                Ok(())
            },
            move || {
                if let Ok(mut guard) = server_for_after.lock() {
                    let _ = guard.flush();
                }
            },
            |_| {},
        );
        let (disconnect_tx, disconnect_rx) = mpsc::channel::<()>();
        let writer_thread = thread::spawn(move || match writer_raw.join() {
            Ok(res) => res,
            Err(_) => Err("wire writer thread panicked".to_string()),
        });
        let reader_thread = thread::spawn(move || {
            let res = match reader_raw.join() {
                Ok(res) => res,
                Err(_) => Err("wire reader thread panicked".to_string()),
            };
            let _ = disconnect_tx.send(());
            res
        });

        self.window = Some(window);
        self.server = Some(server);
        self._native_instance = Some(native_instance);
        self._native_surface = Some(native_surface);
        self.disconnect_rx = Some(disconnect_rx);
        self.pump = Some(PumpGuard::new(
            stop,
            to_writer_tx,
            writer_thread,
            reader_thread,
        ));
        Ok(())
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
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(unix)]
        if let Some(parent_pid) = self.parent_pid {
            let current_ppid = unsafe { libc::getppid() } as u32;
            if current_ppid != parent_pid {
                event_loop.exit();
                return;
            }
        }
        if let Some(rx) = self.disconnect_rx.as_ref()
            && rx.try_recv().is_ok()
        {
            event_loop.exit();
            return;
        }
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
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

fn request_device_sync(
    instance: &Instance,
    adapter: &dawn_rs::Adapter,
    mut flush_wire: impl FnMut(),
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

fn spawn_gpu_process(socket_name: &str) -> Result<Child, Box<dyn Error>> {
    let current = env::current_exe()?;
    let mut cmd = Command::new(current);
    cmd.arg("--gpu-process")
        .arg(socket_name)
        .arg(std::process::id().to_string());
    #[cfg(unix)]
    cmd.arg0("rainbow_cat_gpu_child");
    Ok(cmd.spawn()?)
}
