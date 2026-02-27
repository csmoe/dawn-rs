use crate::Compat;
use dawn_rs::Instance;
use dawn_rs::wire_shim::WireHelperClient;
use interprocess::TryClone;
use interprocess::local_socket::Stream;
use interprocess::local_socket::{GenericFilePath, GenericNamespaced, prelude::*};
use std::fmt;
use std::io::{Read, Write};
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

#[derive(Debug)]
enum OutboundPacket {
    Wire(Vec<u8>),
    Shutdown,
}

#[derive(Debug)]
enum IpcMessage {
    ReserveInstance { id: u32, generation: u32 },
    ReserveAck { ok: bool },
    WireBytes(Vec<u8>),
    HandleCommands,
    Shutdown,
}

pub struct IpcWireBackend {
    client: Arc<Mutex<WireHelperClient>>,
    instance: Instance,
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::Sender<OutboundPacket>>,
    writer_thread: Option<JoinHandle<Result<(), WireBackendError>>>,
    reader_thread: Option<JoinHandle<Result<(), WireBackendError>>>,
    flush_thread: Option<JoinHandle<()>>,
}

impl IpcWireBackend {
    pub fn connect_name(name: &str) -> Result<Self, WireBackendError> {
        let ipc_name = if GenericNamespaced::is_supported() {
            name.to_string().to_ns_name::<GenericNamespaced>()?
        } else {
            format!("/tmp/{name}.sock").to_fs_name::<GenericFilePath>()?
        };
        let stream = Stream::connect(ipc_name)?;
        Self::from_stream(stream)
    }

    pub fn connect_name_with_retry(
        name: &str,
        attempts: usize,
        delay: Duration,
    ) -> Result<Self, WireBackendError> {
        let ipc_name = if GenericNamespaced::is_supported() {
            name.to_string().to_ns_name::<GenericNamespaced>()?
        } else {
            format!("/tmp/{name}.sock").to_fs_name::<GenericFilePath>()?
        };
        let mut last_err: Option<std::io::Error> = None;
        for _ in 0..attempts.max(1) {
            match Stream::connect(ipc_name.clone()) {
                Ok(stream) => return Self::from_stream(stream),
                Err(e) => {
                    last_err = Some(e);
                    thread::sleep(delay);
                }
            }
        }
        Err(WireBackendError::Io(last_err.unwrap_or_else(|| {
            std::io::Error::other("failed to connect wire backend")
        })))
    }

    pub fn from_stream(stream: Stream) -> Result<Self, WireBackendError> {
        let mut reader_stream = stream.try_clone()?;
        let mut writer_stream = stream.try_clone()?;
        let _ = reader_stream.set_recv_timeout(Some(Duration::from_millis(200)));
        let _ = writer_stream.set_send_timeout(Some(Duration::from_millis(200)));

        let (to_writer_tx, to_writer_rx) = mpsc::channel::<OutboundPacket>();
        let client = Arc::new(Mutex::new(
            WireHelperClient::new(
                0,
                {
                    let to_writer_tx = to_writer_tx.clone();
                    move |bytes: &[u8]| {
                        let _ = to_writer_tx.send(OutboundPacket::Wire(bytes.to_vec()));
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

        write_message(
            &mut writer_stream,
            &IpcMessage::ReserveInstance {
                id: reserved.handle.id,
                generation: reserved.handle.generation,
            },
        )?;
        writer_stream.flush()?;
        match read_message(&mut reader_stream)? {
            IpcMessage::ReserveAck { ok } if ok => {}
            IpcMessage::ReserveAck { ok: false } => {
                return Err(WireBackendError::Protocol(
                    "server rejected wire handle injection",
                ));
            }
            _ => return Err(WireBackendError::Protocol("unexpected reserve ack message")),
        }

        let stop = Arc::new(AtomicBool::new(false));
        let client_for_reader = client.clone();
        let writer_stop = stop.clone();
        let writer_thread = thread::spawn(move || -> Result<(), WireBackendError> {
            loop {
                match to_writer_rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(OutboundPacket::Wire(packet)) => {
                        if writer_stop.load(Ordering::Relaxed) {
                            break;
                        }
                        write_message(&mut writer_stream, &IpcMessage::WireBytes(packet))?;
                        write_message(&mut writer_stream, &IpcMessage::HandleCommands)?;
                        writer_stream.flush()?;
                    }
                    Ok(OutboundPacket::Shutdown) => {
                        let _ = write_message(&mut writer_stream, &IpcMessage::Shutdown);
                        let _ = writer_stream.flush();
                        break;
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        if writer_stop.load(Ordering::Relaxed) {
                            break;
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
            Ok(())
        });

        let reader_stop = stop.clone();
        let reader_thread = thread::spawn(move || -> Result<(), WireBackendError> {
            let mut pending = Vec::<Vec<u8>>::new();
            while !reader_stop.load(Ordering::Relaxed) {
                let message = match read_message(&mut reader_stream) {
                    Ok(message) => message,
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(e)
                        if e.kind() == std::io::ErrorKind::TimedOut
                            || e.kind() == std::io::ErrorKind::WouldBlock =>
                    {
                        continue;
                    }
                    Err(e) => return Err(WireBackendError::Io(e)),
                };
                match message {
                    IpcMessage::WireBytes(bytes) => pending.push(bytes),
                    IpcMessage::HandleCommands => {
                        if pending.is_empty() {
                            continue;
                        }
                        let mut guard = client_for_reader
                            .lock()
                            .map_err(|_| WireBackendError::LockPoisoned("wire client"))?;
                        for frame in pending.drain(..) {
                            if !guard.handle_commands(&frame) {
                                return Err(WireBackendError::Protocol(
                                    "wire client HandleCommands failed",
                                ));
                            }
                        }
                    }
                    IpcMessage::Shutdown => break,
                    _ => {}
                }
            }
            Ok(())
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
            let _ = tx.send(OutboundPacket::Shutdown);
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

fn write_message<W: Write>(writer: &mut W, message: &IpcMessage) -> std::io::Result<()> {
    match message {
        IpcMessage::ReserveInstance { id, generation } => {
            writer.write_all(&[1])?;
            writer.write_all(&id.to_le_bytes())?;
            writer.write_all(&generation.to_le_bytes())?;
        }
        IpcMessage::ReserveAck { ok } => {
            writer.write_all(&[2])?;
            writer.write_all(&[*ok as u8])?;
        }
        IpcMessage::WireBytes(bytes) => {
            writer.write_all(&[3])?;
            write_len_prefixed_bytes(writer, bytes)?;
        }
        IpcMessage::HandleCommands => writer.write_all(&[4])?,
        IpcMessage::Shutdown => writer.write_all(&[5])?,
    }
    Ok(())
}

fn read_message<R: Read>(reader: &mut R) -> std::io::Result<IpcMessage> {
    let mut tag = [0u8; 1];
    reader.read_exact(&mut tag)?;
    match tag[0] {
        1 => {
            let id = read_u32(reader)?;
            let generation = read_u32(reader)?;
            Ok(IpcMessage::ReserveInstance { id, generation })
        }
        2 => {
            let ok = read_u8(reader)? != 0;
            Ok(IpcMessage::ReserveAck { ok })
        }
        3 => Ok(IpcMessage::WireBytes(read_len_prefixed_bytes(reader)?)),
        4 => Ok(IpcMessage::HandleCommands),
        5 => Ok(IpcMessage::Shutdown),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unknown ipc message tag",
        )),
    }
}

fn write_len_prefixed_bytes<W: Write>(writer: &mut W, bytes: &[u8]) -> std::io::Result<()> {
    let len = u32::try_from(bytes.len())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "payload too large"))?;
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(bytes)
}

fn read_len_prefixed_bytes<R: Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    let len = read_u32(reader)? as usize;
    if len > 64 * 1024 * 1024 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "wire packet exceeds max size",
        ));
    }
    let mut data = vec![0u8; len];
    reader.read_exact(&mut data)?;
    Ok(data)
}

fn read_u8<R: Read>(reader: &mut R) -> std::io::Result<u8> {
    let mut b = [0u8; 1];
    reader.read_exact(&mut b)?;
    Ok(b[0])
}

fn read_u32<R: Read>(reader: &mut R) -> std::io::Result<u32> {
    let mut b = [0u8; 4];
    reader.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}
