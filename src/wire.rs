use crate::wire_ipc::{self, IpcMessage as WireIpcMessage, OutboundPacket as WireOutboundPacket};
use crate::wire_shim::{
    NativeProcGuard as WireNativeProcGuard, WireHelperClient as WireClientShim,
    WireHelperServer as WireServerShim, WireInstanceHandle as WireObjectHandle,
};
use crate::{Adapter, Device, Instance, RequestAdapterOptions, RequestAdapterStatus, Surface};
#[cfg(feature = "wire")]
use crate::{Texture, TextureFormat, TextureUsage};
use interprocess::TryClone;
use interprocess::local_socket::Stream;
use interprocess::local_socket::traits::Stream as _;
use std::io::Write as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use thiserror::Error;

#[cfg(feature = "wire")]
pub use crate::wire_backend::{WireHandle, WireTextureReservation};

const DEFAULT_WIRE_CHANNEL_CAPACITY: usize = 1024;
const DEFAULT_WIRE_MAX_BATCH_PACKETS: usize = 32;
const DEFAULT_WIRE_FLUSH_INTERVAL: Duration = Duration::from_millis(8);
const DEFAULT_WIRE_MAX_PENDING_PACKETS: usize = 4096;
const DEFAULT_WIRE_MAX_PENDING_BYTES: usize = 16 * 1024 * 1024;
const DEFAULT_WIRE_PACKET_POOL_LIMIT: usize = 1024;

pub fn with_native_runtime<R>(f: impl FnOnce() -> R) -> Result<R, WireError> {
    let _guard = WireNativeProcGuard::acquire().map_err(WireError::Wire)?;
    Ok(f())
}

#[derive(Debug, Error)]
pub enum WireError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("protocol error: {0}")]
    Protocol(&'static str),
    #[error("wire error: {0}")]
    Wire(String),
    #[error("lock poisoned: {0}")]
    LockPoisoned(&'static str),
    #[error("timeout: {0}")]
    Timeout(&'static str),
}

#[derive(Debug, Clone, Copy)]
pub struct ClientOptions {
    pub reserve_surface: bool,
    pub connect_attempts: usize,
    pub connect_delay: Duration,
    pub max_allocation_size: usize,
    pub transport: TransportOptions,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            reserve_surface: false,
            connect_attempts: 1,
            connect_delay: Duration::from_millis(0),
            max_allocation_size: 0,
            transport: TransportOptions::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TransportOptions {
    /// Bounded queue size used between Dawn wire callbacks and IPC writer.
    pub channel_capacity: usize,
    /// Max wire packets coalesced before forcing a flush.
    pub max_batch_packets: usize,
    /// Max time window used for packet coalescing once traffic is active.
    pub flush_interval: Duration,
    /// Safety cap for buffered inbound packet count before HandleCommands.
    pub max_pending_packets: usize,
    /// Safety cap for buffered inbound bytes before HandleCommands.
    pub max_pending_bytes: usize,
    /// Max number of recycled packet buffers kept for reuse.
    pub packet_pool_limit: usize,
}

impl Default for TransportOptions {
    fn default() -> Self {
        Self {
            channel_capacity: DEFAULT_WIRE_CHANNEL_CAPACITY,
            max_batch_packets: DEFAULT_WIRE_MAX_BATCH_PACKETS,
            flush_interval: DEFAULT_WIRE_FLUSH_INTERVAL,
            max_pending_packets: DEFAULT_WIRE_MAX_PENDING_PACKETS,
            max_pending_bytes: DEFAULT_WIRE_MAX_PENDING_BYTES,
            packet_pool_limit: DEFAULT_WIRE_PACKET_POOL_LIMIT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ServerOptions {
    pub expect_surface: bool,
    pub use_spontaneous_callbacks: bool,
    pub max_allocation_size: usize,
    pub transport: TransportOptions,
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            expect_surface: false,
            use_spontaneous_callbacks: true,
            max_allocation_size: 0,
            transport: TransportOptions::default(),
        }
    }
}

/// High-level wire client runtime.
///
/// This owns:
/// - a wire client object
/// - IPC pump threads
/// - reserved/converted wire `Instance` and optional `Surface`
///
/// Typical usage:
/// 1) connect
/// 2) use `instance()` / `surface()`
/// 3) call `pump()` in your event loop
pub struct Client {
    client: Arc<Mutex<WireClientShim>>,
    instance: Option<Instance>,
    surface: Option<Surface>,
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::SyncSender<WireOutboundPacket>>,
    writer_thread: Option<JoinHandle<Result<(), WireError>>>,
    reader_thread: Option<JoinHandle<Result<(), WireError>>>,
}

#[cfg(feature = "wire")]
#[derive(Debug, Clone)]
pub struct ReservedTexture {
    pub texture: Texture,
    pub reservation: WireTextureReservation,
    pub width: u32,
    pub height: u32,
}

impl Client {
    pub fn connect(name: &str, opts: ClientOptions) -> Result<Self, WireError> {
        let stream = wire_ipc::connect_with_retry(name, opts.connect_attempts, opts.connect_delay)?;
        Self::from_stream(stream, opts)
    }

    pub fn from_stream(stream: Stream, opts: ClientOptions) -> Result<Self, WireError> {
        let mut reader_stream = stream.try_clone()?;
        let mut writer_stream = stream.try_clone()?;

        let transport = opts.transport;
        let packet_pool = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let (to_writer_tx, to_writer_rx) =
            mpsc::sync_channel::<WireOutboundPacket>(transport.channel_capacity.max(1));
        let client = Arc::new(Mutex::new(
            WireClientShim::new(
                opts.max_allocation_size,
                {
                    let to_writer_tx = to_writer_tx.clone();
                    let packet_pool = packet_pool.clone();
                    move |bytes: &[u8]| {
                        let mut buf = if let Ok(mut pool) = packet_pool.lock() {
                            pool.pop().unwrap_or_default()
                        } else {
                            Vec::new()
                        };
                        buf.clear();
                        buf.extend_from_slice(bytes);
                        let _ = to_writer_tx.send(WireOutboundPacket::Wire(buf));
                    }
                },
                |_msg: &str| {},
            )
            .map_err(WireError::Wire)?,
        ));

        let reserved_instance = {
            let mut guard = client
                .lock()
                .map_err(|_| WireError::LockPoisoned("wire client"))?;
            guard.reserve_instance()
        };
        if reserved_instance.instance.is_null() {
            return Err(WireError::Protocol(
                "wire reserve_instance returned null instance",
            ));
        }
        wire_ipc::write_message(
            &mut writer_stream,
            &WireIpcMessage::ReserveInstance {
                id: reserved_instance.handle.id,
                generation: reserved_instance.handle.generation,
            },
        )?;

        let reserved_surface = if opts.reserve_surface {
            let reserved = {
                let mut guard = client
                    .lock()
                    .map_err(|_| WireError::LockPoisoned("wire client"))?;
                guard.reserve_surface(reserved_instance.instance)
            };
            if reserved.surface.is_null() {
                return Err(WireError::Protocol(
                    "wire reserve_surface returned null surface",
                ));
            }
            wire_ipc::write_message(
                &mut writer_stream,
                &WireIpcMessage::ReserveSurface {
                    id: reserved.handle.id,
                    generation: reserved.handle.generation,
                    instance_id: reserved.instance_handle.id,
                    instance_generation: reserved.instance_handle.generation,
                },
            )?;
            Some(reserved)
        } else {
            None
        };

        writer_stream.flush()?;
        match wire_ipc::read_message(&mut reader_stream)? {
            WireIpcMessage::ReserveAck { ok } if ok => {}
            WireIpcMessage::ReserveAck { ok: false } => {
                return Err(WireError::Protocol("server rejected injected handles"));
            }
            _ => return Err(WireError::Protocol("unexpected reserve ack message")),
        }
        let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));

        let stop = Arc::new(AtomicBool::new(false));
        let client_for_reader = client.clone();
        let (writer_raw, reader_raw) = wire_ipc::start_wire_threads(
            stop.clone(),
            reader_stream,
            writer_stream,
            to_writer_rx,
            transport.max_batch_packets.max(1),
            transport.flush_interval,
            transport.max_pending_packets,
            transport.max_pending_bytes,
            move |frame: &[u8]| {
                let mut guard = client_for_reader
                    .lock()
                    .map_err(|_| "wire client lock poisoned".to_string())?;
                if !guard.handle_commands(frame) {
                    return Err("wire client HandleCommands failed".to_string());
                }
                Ok(())
            },
            || {},
            |_| {},
            {
                let packet_pool = packet_pool.clone();
                move |mut packet: Vec<u8>| {
                    packet.clear();
                    if let Ok(mut pool) = packet_pool.lock()
                        && pool.len() < transport.packet_pool_limit
                    {
                        pool.push(packet);
                    }
                }
            },
        );

        let writer_thread = thread::spawn(move || -> Result<(), WireError> {
            writer_raw
                .join()
                .map_err(|_| WireError::Protocol("wire writer thread panicked"))?
                .map_err(WireError::Wire)
        });
        let reader_thread = thread::spawn(move || -> Result<(), WireError> {
            reader_raw
                .join()
                .map_err(|_| WireError::Protocol("wire reader thread panicked"))?
                .map_err(WireError::Wire)
        });

        let instance = unsafe { WireClientShim::reserved_instance_to_instance(reserved_instance) };
        let surface = reserved_surface
            .map(|reserved| unsafe { WireClientShim::reserved_surface_to_surface(reserved) });

        Ok(Self {
            client,
            instance: Some(instance),
            surface,
            stop,
            tx: Some(to_writer_tx),
            writer_thread: Some(writer_thread),
            reader_thread: Some(reader_thread),
        })
    }

    pub fn instance(&self) -> Instance {
        self.instance
            .as_ref()
            .expect("wire client instance already dropped")
            .clone()
    }

    pub fn surface(&self) -> Option<Surface> {
        self.surface.clone()
    }

    pub fn pump(&self) {
        if let Ok(mut guard) = self.client.lock() {
            let _ = guard.flush();
        }
        if let Some(instance) = self.instance.as_ref() {
            instance.process_events();
        }
    }

    #[cfg(feature = "wire")]
    pub fn reserve_texture(
        &self,
        device: &Device,
        width: u32,
        height: u32,
        format: TextureFormat,
        usage: TextureUsage,
    ) -> Result<ReservedTexture, WireError> {
        let mut guard = self
            .client
            .lock()
            .map_err(|_| WireError::LockPoisoned("wire client"))?;
        let reserved = guard.reserve_texture_2d(
            device.as_raw().cast(),
            width,
            height,
            format as u32,
            usage.bits(),
        );
        if reserved.texture.is_null() {
            return Err(WireError::Protocol(
                "wire reserve_texture_2d returned null texture",
            ));
        }

        let texture = unsafe { WireClientShim::reserved_texture_to_texture(reserved) };
        Ok(ReservedTexture {
            texture,
            reservation: WireTextureReservation::new(
                reserved.handle.into(),
                reserved.device_handle.into(),
                width,
                height,
                format as u32,
                usage.bits(),
            ),
            width,
            height,
        })
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(WireOutboundPacket::Shutdown);
        }
        if let Some(handle) = self.writer_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
        let _ = self.surface.take();
        let _ = self.instance.take();
        if let Ok(mut guard) = self.client.lock() {
            let _ = guard.flush();
            guard.disconnect();
        }
    }
}

/// High-level wire server runtime for instance/surface injection and command pumping.
pub struct Server {
    server: Arc<Mutex<WireServerShim>>,
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::SyncSender<WireOutboundPacket>>,
    writer_thread: Option<JoinHandle<Result<(), WireError>>>,
    reader_thread: Option<JoinHandle<Result<(), WireError>>>,
}

impl Server {
    pub fn accept_and_inject(
        socket_name: &str,
        native_instance: &Instance,
        native_surface: Option<&Surface>,
        opts: ServerOptions,
    ) -> Result<Self, WireError> {
        let stream = wire_ipc::bind_and_accept(socket_name)?;
        let mut reader_stream = stream.try_clone()?;
        let mut writer_stream = stream.try_clone()?;

        let instance_handle = match wire_ipc::read_message(&mut reader_stream)? {
            WireIpcMessage::ReserveInstance { id, generation } => {
                WireObjectHandle { id, generation }
            }
            _ => return Err(WireError::Protocol("expected ReserveInstance")),
        };

        let surface_handles = if opts.expect_surface {
            match wire_ipc::read_message(&mut reader_stream)? {
                WireIpcMessage::ReserveSurface {
                    id,
                    generation,
                    instance_id,
                    instance_generation,
                } => Some((
                    WireObjectHandle { id, generation },
                    WireObjectHandle {
                        id: instance_id,
                        generation: instance_generation,
                    },
                )),
                _ => return Err(WireError::Protocol("expected ReserveSurface")),
            }
        } else {
            None
        };

        let transport = opts.transport;
        let packet_pool = Arc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let (to_writer_tx, to_writer_rx) =
            mpsc::sync_channel::<WireOutboundPacket>(transport.channel_capacity.max(1));
        let server = Arc::new(Mutex::new(
            WireServerShim::new_native(
                opts.max_allocation_size,
                opts.use_spontaneous_callbacks,
                {
                    let to_writer_tx = to_writer_tx.clone();
                    let packet_pool = packet_pool.clone();
                    move |bytes: &[u8]| {
                        let mut buf = if let Ok(mut pool) = packet_pool.lock() {
                            pool.pop().unwrap_or_default()
                        } else {
                            Vec::new()
                        };
                        buf.clear();
                        buf.extend_from_slice(bytes);
                        let _ = to_writer_tx.send(WireOutboundPacket::Wire(buf));
                    }
                },
                |_msg: &str| {},
            )
            .map_err(WireError::Wire)?,
        ));

        let injected_instance = {
            let mut guard = server
                .lock()
                .map_err(|_| WireError::LockPoisoned("wire server"))?;
            guard.inject_instance(native_instance.as_raw().cast(), instance_handle)
        };
        let injected_surface = match (surface_handles, native_surface) {
            (Some((surface_handle, surface_instance_handle)), Some(surface)) => {
                let mut guard = server
                    .lock()
                    .map_err(|_| WireError::LockPoisoned("wire server"))?;
                guard.inject_surface(
                    surface.as_raw().cast(),
                    surface_handle,
                    surface_instance_handle,
                )
            }
            (None, _) => true,
            (Some(_), None) => false,
        };
        let ok = injected_instance && injected_surface;
        wire_ipc::write_message(&mut writer_stream, &WireIpcMessage::ReserveAck { ok })?;
        writer_stream.flush()?;
        if !ok {
            return Err(WireError::Protocol(
                "failed to inject instance/surface into wire server",
            ));
        }

        let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));
        let stop = Arc::new(AtomicBool::new(false));
        let server_for_reader = server.clone();
        let server_for_after = server.clone();
        let (writer_raw, reader_raw) = wire_ipc::start_wire_threads(
            stop.clone(),
            reader_stream,
            writer_stream,
            to_writer_rx,
            transport.max_batch_packets.max(1),
            transport.flush_interval,
            transport.max_pending_packets,
            transport.max_pending_bytes,
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
            {
                let packet_pool = packet_pool.clone();
                move |mut packet: Vec<u8>| {
                    packet.clear();
                    if let Ok(mut pool) = packet_pool.lock()
                        && pool.len() < transport.packet_pool_limit
                    {
                        pool.push(packet);
                    }
                }
            },
        );

        let writer_thread = thread::spawn(move || -> Result<(), WireError> {
            writer_raw
                .join()
                .map_err(|_| WireError::Protocol("wire writer thread panicked"))?
                .map_err(WireError::Wire)
        });
        let reader_thread = thread::spawn(move || -> Result<(), WireError> {
            reader_raw
                .join()
                .map_err(|_| WireError::Protocol("wire reader thread panicked"))?
                .map_err(WireError::Wire)
        });

        Ok(Self {
            server,
            stop,
            tx: Some(to_writer_tx),
            writer_thread: Some(writer_thread),
            reader_thread: Some(reader_thread),
        })
    }

    pub fn flush(&self) {
        if let Ok(mut guard) = self.server.lock() {
            let _ = guard.flush();
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(WireOutboundPacket::Shutdown);
        }
        if let Some(handle) = self.writer_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
    }
}

/// Requests an adapter and blocks by repeatedly pumping wire work.
pub fn request_adapter_blocking(
    instance: &Instance,
    options: Option<RequestAdapterOptions>,
    timeout: Duration,
    poll_interval: Duration,
    mut pump: impl FnMut(),
) -> Result<Adapter, WireError> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<Adapter, String>>();
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

    let started = Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(adapter)) => return Ok(adapter),
            Ok(Err(msg)) => return Err(WireError::Wire(msg)),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if started.elapsed() > timeout {
                    return Err(WireError::Timeout("request_adapter_blocking"));
                }
                pump();
                instance.process_events();
                thread::sleep(poll_interval);
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err(WireError::Protocol("request_adapter callback disconnected"));
            }
        }
    }
}

/// Requests a device and blocks by repeatedly pumping wire work.
pub fn request_device_blocking(
    instance: &Instance,
    adapter: &Adapter,
    desc: Option<&crate::DeviceDescriptor>,
    timeout: Duration,
    poll_interval: Duration,
    mut pump: impl FnMut(),
) -> Result<Device, WireError> {
    let (tx, rx) = std::sync::mpsc::channel::<Result<Device, String>>();
    let _future = adapter.request_device(desc, move |status, device, message| {
        if status != crate::RequestDeviceStatus::Success {
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

    let started = Instant::now();
    loop {
        match rx.try_recv() {
            Ok(Ok(device)) => return Ok(device),
            Ok(Err(msg)) => return Err(WireError::Wire(msg)),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                if started.elapsed() > timeout {
                    return Err(WireError::Timeout("request_device_blocking"));
                }
                pump();
                instance.process_events();
                thread::sleep(poll_interval);
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err(WireError::Protocol("request_device callback disconnected"));
            }
        }
    }
}
