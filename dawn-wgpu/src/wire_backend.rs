use crate::Compat;
use dawn_rs::Instance;
use dawn_rs::wire_ipc;
use dawn_rs::wire_shim::WireHelperClient;
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

pub struct IpcWireBackend {
    client: Arc<Mutex<WireHelperClient>>,
    instance: Instance,
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::SyncSender<wire_ipc::OutboundPacket>>,
    writer_thread: Option<JoinHandle<Result<(), WireBackendError>>>,
    reader_thread: Option<JoinHandle<Result<(), WireBackendError>>>,
    flush_thread: Option<JoinHandle<()>>,
}

impl IpcWireBackend {
    pub fn connect_name(name: &str) -> Result<Self, WireBackendError> {
        let stream = wire_ipc::connect_with_retry(name, 1, Duration::from_millis(0))?;
        Self::from_stream(stream)
    }

    pub fn connect_name_with_retry(
        name: &str,
        attempts: usize,
        delay: Duration,
    ) -> Result<Self, WireBackendError> {
        let stream = wire_ipc::connect_with_retry(name, attempts, delay)?;
        Self::from_stream(stream)
    }

    pub fn from_stream(stream: Stream) -> Result<Self, WireBackendError> {
        let mut reader_stream = stream.try_clone()?;
        let mut writer_stream = stream.try_clone()?;
        let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));
        let _ = writer_stream.set_send_timeout(Some(Duration::from_millis(200)));

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

        let reserved = {
            let mut guard = client
                .lock()
                .map_err(|_| WireBackendError::LockPoisoned("wire client"))?;
            guard.reserve_instance()
        };
        if reserved.instance.is_null() {
            return Err(WireBackendError::Protocol(
                "wire reserve_instance returned null instance",
            ));
        }

        wire_ipc::write_message(
            &mut writer_stream,
            &wire_ipc::IpcMessage::ReserveInstance {
                id: reserved.handle.id,
                generation: reserved.handle.generation,
            },
        )?;
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
            writer_raw
                .join()
                .map_err(|_| WireBackendError::Protocol("wire writer thread panicked"))?
                .map_err(WireBackendError::Wire)
        });
        let reader_thread = thread::spawn(move || -> Result<(), WireBackendError> {
            reader_raw
                .join()
                .map_err(|_| WireBackendError::Protocol("wire reader thread panicked"))?
                .map_err(WireBackendError::Wire)
        });

        let instance = unsafe { WireHelperClient::reserved_instance_to_instance(reserved) };
        let client_for_flush = client.clone();
        let instance_for_flush = instance.clone();
        let flush_stop = stop.clone();
        let flush_thread = thread::spawn(move || {
            while !flush_stop.load(Ordering::Relaxed) {
                if let Ok(mut guard) = client_for_flush.lock() {
                    let _ = guard.flush();
                }
                instance_for_flush.process_events();
                thread::sleep(Duration::from_millis(1));
            }
        });

        Ok(Self {
            client,
            instance,
            stop,
            tx: Some(to_writer_tx),
            writer_thread: Some(writer_thread),
            reader_thread: Some(reader_thread),
            flush_thread: Some(flush_thread),
        })
    }

    pub fn dawn_instance(&self) -> Instance {
        self.instance.clone()
    }

    pub fn wgpu_instance(&self) -> wgpu::Instance {
        Compat::from(self.instance.clone()).into()
    }
}

impl From<&IpcWireBackend> for wgpu::Instance {
    fn from(value: &IpcWireBackend) -> Self {
        value.wgpu_instance()
    }
}

impl Drop for IpcWireBackend {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(wire_ipc::OutboundPacket::Shutdown);
        }
        if let Some(handle) = self.flush_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.writer_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
        if let Ok(mut guard) = self.client.lock() {
            let _ = guard.flush();
            guard.disconnect();
        }
    }
}
