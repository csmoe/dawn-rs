use crate::Compat;
use dawn_rs::{Instance, Surface};
use dawn_rs::wire_ipc;
use dawn_rs::wire_shim::{WireHelperClient, WireInstanceHandle};
use interprocess::TryClone;
use interprocess::local_socket::Stream;
use interprocess::local_socket::traits::Stream as _;
use std::fmt;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub enum WireBackendError {
    Io(std::io::Error),
    Protocol(&'static str),
    Wire(String),
    LockPoisoned(&'static str),
}

impl fmt::Display for WireBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Protocol(msg) => write!(f, "protocol error: {msg}"),
            Self::Wire(msg) => write!(f, "wire error: {msg}"),
            Self::LockPoisoned(msg) => write!(f, "lock poisoned: {msg}"),
        }
    }
}

impl std::error::Error for WireBackendError {}

impl From<std::io::Error> for WireBackendError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WireInitOptions {
    pub reserve_surface: bool,
    pub connect_attempts: usize,
    pub connect_delay: Duration,
}

impl Default for WireInitOptions {
    fn default() -> Self {
        Self {
            reserve_surface: false,
            connect_attempts: 1,
            connect_delay: Duration::from_millis(0),
        }
    }
}

pub trait WireTransport: Send + Sync + 'static {
    fn connect(name: &str, attempts: usize, delay: Duration) -> Result<Stream, WireBackendError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IpcWireTransport;

impl WireTransport for IpcWireTransport {
    fn connect(name: &str, attempts: usize, delay: Duration) -> Result<Stream, WireBackendError> {
        Ok(wire_ipc::connect_with_retry(name, attempts, delay)?)
    }
}

pub struct WireClientBackend<T: WireTransport = IpcWireTransport> {
    client: Arc<Mutex<WireHelperClient>>,
    instance: Option<Instance>,
    surface: Option<Surface>,
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::SyncSender<wire_ipc::OutboundPacket>>,
    writer_thread: Option<JoinHandle<Result<(), WireBackendError>>>,
    reader_thread: Option<JoinHandle<Result<(), WireBackendError>>>,
    _transport: std::marker::PhantomData<T>,
}

impl<T: WireTransport> WireClientBackend<T> {
    pub fn from_stream_with_options(
        stream: Stream,
        opts: WireInitOptions,
    ) -> Result<Self, WireBackendError> {
        let mut reader_stream = stream.try_clone()?;
        let mut writer_stream = stream.try_clone()?;

        let (to_writer_tx, to_writer_rx) = mpsc::sync_channel::<wire_ipc::OutboundPacket>(1024);
        let client = Arc::new(Mutex::new(
            WireHelperClient::new(
                0,
                {
                    let to_writer_tx = to_writer_tx.clone();
                    move |bytes: &[u8]| {
                        let _ = to_writer_tx.send(wire_ipc::OutboundPacket::Wire(bytes.to_vec()));
                    }
                },
                |msg: &str| eprintln!("wire backend client error: {msg}"),
            )
            .map_err(WireBackendError::Wire)?,
        ));

        let reserved_instance = {
            let mut guard = client
                .lock()
                .map_err(|_| WireBackendError::LockPoisoned("wire client"))?;
            guard.reserve_instance()
        };
        if reserved_instance.instance.is_null() {
            return Err(WireBackendError::Protocol(
                "wire reserve_instance returned null instance",
            ));
        }
        wire_ipc::write_message(
            &mut writer_stream,
            &wire_ipc::IpcMessage::ReserveInstance {
                id: reserved_instance.handle.id,
                generation: reserved_instance.handle.generation,
            },
        )?;

        let reserved_surface = if opts.reserve_surface {
            let reserved = {
                let mut guard = client
                    .lock()
                    .map_err(|_| WireBackendError::LockPoisoned("wire client"))?;
                guard.reserve_surface(reserved_instance.instance)
            };
            if reserved.surface.is_null() {
                return Err(WireBackendError::Protocol(
                    "wire reserve_surface returned null surface",
                ));
            }
            wire_ipc::write_message(
                &mut writer_stream,
                &wire_ipc::IpcMessage::ReserveSurface {
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
            wire_ipc::IpcMessage::ReserveAck { ok } if ok => {}
            wire_ipc::IpcMessage::ReserveAck { ok: false } => {
                return Err(WireBackendError::Protocol(
                    "server rejected wire handle injection",
                ));
            }
            _ => return Err(WireBackendError::Protocol("unexpected reserve ack message")),
        }
        let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));

        let stop = Arc::new(AtomicBool::new(false));
        let client_for_reader = client.clone();
        let (writer_raw, reader_raw) = wire_ipc::start_wire_threads(
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
                    return Err("wire client HandleCommands failed".to_string());
                }
                Ok(())
            },
            || {},
            |_| {},
        );

        let writer_thread = thread::spawn(move || -> Result<(), WireBackendError> {
            let res = writer_raw
                .join()
                .map_err(|_| WireBackendError::Protocol("wire writer thread panicked"))?
                .map_err(WireBackendError::Wire);
            if let Err(err) = &res {
                eprintln!("wire client writer thread ended with error: {err}");
            } else {
                eprintln!("wire client writer thread exited");
            }
            res
        });
        let reader_thread = thread::spawn(move || -> Result<(), WireBackendError> {
            let res = reader_raw
                .join()
                .map_err(|_| WireBackendError::Protocol("wire reader thread panicked"))?
                .map_err(WireBackendError::Wire);
            if let Err(err) = &res {
                eprintln!("wire client reader thread ended with error: {err}");
            } else {
                eprintln!("wire client reader thread exited");
            }
            res
        });

        let instance =
            unsafe { WireHelperClient::reserved_instance_to_instance(reserved_instance) };
        let surface = reserved_surface
            .map(|reserved| unsafe { WireHelperClient::reserved_surface_to_surface(reserved) });

        Ok(Self {
            client,
            instance: Some(instance),
            surface,
            stop,
            tx: Some(to_writer_tx),
            writer_thread: Some(writer_thread),
            reader_thread: Some(reader_thread),
            _transport: std::marker::PhantomData,
        })
    }

    pub fn connect_name_with_options(
        name: &str,
        opts: WireInitOptions,
    ) -> Result<Self, WireBackendError> {
        let stream = T::connect(name, opts.connect_attempts, opts.connect_delay)?;
        Self::from_stream_with_options(stream, opts)
    }

    pub fn connect_name(name: &str) -> Result<Self, WireBackendError> {
        Self::connect_name_with_options(name, WireInitOptions::default())
    }

    pub fn dawn_instance(&self) -> Instance {
        self.instance
            .as_ref()
            .expect("wire backend instance already dropped")
            .clone()
    }

    pub fn dawn_surface(&self) -> Option<Surface> {
        self.surface.clone()
    }

    pub fn wgpu_instance(&self) -> wgpu::Instance {
        Compat::from(self.dawn_instance()).into()
    }

    pub fn pump(&self) {
        if let Ok(mut guard) = self.client.lock() {
            let _ = guard.flush();
        }
        if let Some(instance) = self.instance.as_ref() {
            instance.process_events();
        }
    }
}

impl<T: WireTransport> From<&WireClientBackend<T>> for wgpu::Instance {
    fn from(value: &WireClientBackend<T>) -> Self {
        value.wgpu_instance()
    }
}

impl<T: WireTransport> Drop for WireClientBackend<T> {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(wire_ipc::OutboundPacket::Shutdown);
        }
        if let Some(handle) = self.writer_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
        // Ensure object releases happen before wire client disconnect.
        let _ = self.surface.take();
        let _ = self.instance.take();
        if let Ok(mut guard) = self.client.lock() {
            let _ = guard.flush();
            guard.disconnect();
        }
    }
}

pub type IpcWireBackend = WireClientBackend<IpcWireTransport>;

pub struct WireBackendHandle {
    backend: Mutex<Option<IpcWireBackend>>,
}

impl WireBackendHandle {
    fn new(backend: IpcWireBackend) -> Self {
        Self {
            backend: Mutex::new(Some(backend)),
        }
    }
}

impl fmt::Debug for WireBackendHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WireBackendHandle").finish_non_exhaustive()
    }
}

impl Drop for WireBackendHandle {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.backend.lock() {
            let _ = guard.take();
        }
    }
}

impl IpcWireBackend {
    pub fn into_instance_and_handle(self) -> (Instance, Arc<WireBackendHandle>) {
        let instance = self.dawn_instance();
        (instance, Arc::new(WireBackendHandle::new(self)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WireHandle {
    pub id: u32,
    pub generation: u32,
}

impl From<WireInstanceHandle> for WireHandle {
    fn from(value: WireInstanceHandle) -> Self {
        Self {
            id: value.id,
            generation: value.generation,
        }
    }
}
