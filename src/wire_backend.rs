use crate::wire_ipc::{self, IpcMessage, OutboundPacket};
use crate::wire_shim::{WireHelperServer, WireInstanceHandle};
use interprocess::TryClone;
use interprocess::local_socket::Stream;
use std::fmt;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

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

#[derive(Debug, Clone, Copy)]
pub struct WireTextureReservation {
    pub texture_handle: WireHandle,
    pub device_handle: WireHandle,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum WireServerError {
    Io(std::io::Error),
    Protocol(&'static str),
}

impl fmt::Display for WireServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Protocol(msg) => write!(f, "protocol error: {msg}"),
        }
    }
}

impl std::error::Error for WireServerError {}

impl From<std::io::Error> for WireServerError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReservedWireObjects {
    pub instance: WireInstanceHandle,
    pub surface: WireInstanceHandle,
    pub surface_instance: WireInstanceHandle,
}

pub fn accept_wire_client(
    socket_name: &str,
) -> Result<(Stream, Stream, ReservedWireObjects), WireServerError> {
    let stream = wire_ipc::bind_and_accept(socket_name)?;
    let mut reader = stream.try_clone()?;
    let writer = stream.try_clone()?;

    let instance = match wire_ipc::read_message(&mut reader)? {
        IpcMessage::ReserveInstance { id, generation } => WireInstanceHandle { id, generation },
        _ => return Err(WireServerError::Protocol("expected ReserveInstance")),
    };

    let (surface, surface_instance) = match wire_ipc::read_message(&mut reader)? {
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
        _ => return Err(WireServerError::Protocol("expected ReserveSurface")),
    };

    Ok((
        reader,
        writer,
        ReservedWireObjects {
            instance,
            surface,
            surface_instance,
        },
    ))
}

pub fn send_reserve_ack(writer: &mut Stream, ok: bool) -> Result<(), WireServerError> {
    wire_ipc::write_message(writer, &IpcMessage::ReserveAck { ok })?;
    writer.flush()?;
    Ok(())
}

pub fn send_texture_reservation(
    writer: &mut Stream,
    reservation: WireTextureReservation,
) -> Result<(), WireServerError> {
    wire_ipc::write_message(
        writer,
        &IpcMessage::ReserveWireTexture {
            id: reservation.texture_handle.id,
            generation: reservation.texture_handle.generation,
            device_id: reservation.device_handle.id,
            device_generation: reservation.device_handle.generation,
            width: reservation.width,
            height: reservation.height,
        },
    )?;
    writer.flush()?;
    Ok(())
}

pub fn recv_texture_reservation_nonblocking(
    reader: &mut Stream,
) -> Result<Option<WireTextureReservation>, WireServerError> {
    match wire_ipc::read_message(reader) {
        Ok(IpcMessage::ReserveWireTexture {
            id,
            generation,
            device_id,
            device_generation,
            width,
            height,
        }) => Ok(Some(WireTextureReservation {
            texture_handle: WireHandle { id, generation },
            device_handle: WireHandle {
                id: device_id,
                generation: device_generation,
            },
            width,
            height,
        })),
        Ok(_) => Ok(None),
        Err(err)
            if err.kind() == std::io::ErrorKind::WouldBlock
                || err.kind() == std::io::ErrorKind::TimedOut
                || err.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            Ok(None)
        }
        Err(err) => Err(WireServerError::Io(err)),
    }
}

pub fn recv_texture_reservation_ack_nonblocking(
    reader: &mut Stream,
) -> Result<Option<bool>, WireServerError> {
    match wire_ipc::read_message(reader) {
        Ok(IpcMessage::ReserveWireTextureAck { ok }) => Ok(Some(ok)),
        Ok(_) => Ok(None),
        Err(err)
            if err.kind() == std::io::ErrorKind::WouldBlock
                || err.kind() == std::io::ErrorKind::TimedOut
                || err.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            Ok(None)
        }
        Err(err) => Err(WireServerError::Io(err)),
    }
}

pub fn send_texture_reservation_ack(writer: &mut Stream, ok: bool) -> Result<(), WireServerError> {
    wire_ipc::write_message(writer, &IpcMessage::ReserveWireTextureAck { ok })?;
    writer.flush()?;
    Ok(())
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Copy)]
pub struct DxgiImportConfig {
    pub shared_handle: usize,
    pub use_keyed_mutex: bool,
}

#[cfg(target_os = "windows")]
pub fn send_dxgi_import_config(
    writer: &mut Stream,
    config: DxgiImportConfig,
) -> Result<(), WireServerError> {
    wire_ipc::write_message(
        writer,
        &IpcMessage::SetDxgiSharedTexture {
            handle: config.shared_handle as u64,
            use_keyed_mutex: config.use_keyed_mutex,
        },
    )?;
    writer.flush()?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn recv_dxgi_import_config_nonblocking(
    reader: &mut Stream,
) -> Result<Option<DxgiImportConfig>, WireServerError> {
    match wire_ipc::read_message(reader) {
        Ok(IpcMessage::SetDxgiSharedTexture {
            handle,
            use_keyed_mutex,
        }) => Ok(Some(DxgiImportConfig {
            shared_handle: handle as usize,
            use_keyed_mutex,
        })),
        Ok(_) => Ok(None),
        Err(err)
            if err.kind() == std::io::ErrorKind::WouldBlock
                || err.kind() == std::io::ErrorKind::TimedOut
                || err.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            Ok(None)
        }
        Err(err) => Err(WireServerError::Io(err)),
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy)]
pub struct DmabufImportConfig {
    pub fd: i32,
    pub drm_format: u32,
    pub drm_modifier: u64,
    pub stride: u32,
    pub offset: u64,
}

#[cfg(target_os = "linux")]
pub fn send_dmabuf_import_config(
    writer: &mut Stream,
    config: DmabufImportConfig,
) -> Result<(), WireServerError> {
    wire_ipc::write_message(
        writer,
        &IpcMessage::SetDmabufSharedTexture {
            fd: config.fd,
            drm_format: config.drm_format,
            drm_modifier: config.drm_modifier,
            stride: config.stride,
            offset: config.offset,
        },
    )?;
    writer.flush()?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn recv_dmabuf_import_config_nonblocking(
    reader: &mut Stream,
) -> Result<Option<DmabufImportConfig>, WireServerError> {
    match wire_ipc::read_message(reader) {
        Ok(IpcMessage::SetDmabufSharedTexture {
            fd,
            drm_format,
            drm_modifier,
            stride,
            offset,
        }) => Ok(Some(DmabufImportConfig {
            fd,
            drm_format,
            drm_modifier,
            stride,
            offset,
        })),
        Ok(_) => Ok(None),
        Err(err)
            if err.kind() == std::io::ErrorKind::WouldBlock
                || err.kind() == std::io::ErrorKind::TimedOut
                || err.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            Ok(None)
        }
        Err(err) => Err(WireServerError::Io(err)),
    }
}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Copy)]
pub struct InjectIosurfaceParams {
    pub io_surface: *mut std::ffi::c_void,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Copy)]
pub struct InjectDxgiParams {
    pub shared_handle: *mut std::ffi::c_void,
    pub use_keyed_mutex: bool,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy)]
pub struct InjectDmabufParams {
    pub fd: i32,
    pub drm_format: u32,
    pub drm_modifier: u64,
    pub stride: u32,
    pub offset: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum SharedTextureImportParams {
    #[cfg(target_os = "macos")]
    Iosurface(InjectIosurfaceParams),
    #[cfg(target_os = "windows")]
    Dxgi(InjectDxgiParams),
    #[cfg(target_os = "linux")]
    Dmabuf(InjectDmabufParams),
}

pub fn inject_reserved_texture(
    server: &mut WireHelperServer,
    reservation: WireTextureReservation,
    import: SharedTextureImportParams,
) -> bool {
    let texture_handle = WireInstanceHandle {
        id: reservation.texture_handle.id,
        generation: reservation.texture_handle.generation,
    };
    let device_handle = WireInstanceHandle {
        id: reservation.device_handle.id,
        generation: reservation.device_handle.generation,
    };
    match import {
        #[cfg(target_os = "macos")]
        SharedTextureImportParams::Iosurface(params) => server.inject_iosurface_texture(
            params.io_surface,
            reservation.width,
            reservation.height,
            texture_handle,
            device_handle,
        ),
        #[cfg(target_os = "windows")]
        SharedTextureImportParams::Dxgi(params) => server.inject_dxgi_texture(
            params.shared_handle,
            params.use_keyed_mutex,
            reservation.width,
            reservation.height,
            texture_handle,
            device_handle,
        ),
        #[cfg(target_os = "linux")]
        SharedTextureImportParams::Dmabuf(params) => server.inject_dmabuf_texture(
            params.fd,
            params.drm_format,
            params.drm_modifier,
            params.stride,
            params.offset,
            reservation.width,
            reservation.height,
            texture_handle,
            device_handle,
        ),
    }
}

pub struct WireServerPump {
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::Sender<OutboundPacket>>,
    writer_thread: Option<JoinHandle<Result<(), String>>>,
    reader_thread: Option<JoinHandle<Result<(), String>>>,
    disconnect_rx: mpsc::Receiver<()>,
    pending_wire_rx: Option<mpsc::Receiver<Vec<u8>>>,
    handle_wire_on_main_thread: bool,
}

impl WireServerPump {
    pub fn start(
        reader_stream: Stream,
        writer_stream: Stream,
        tx: mpsc::Sender<OutboundPacket>,
        rx: mpsc::Receiver<OutboundPacket>,
        server: Arc<Mutex<WireHelperServer>>,
        handle_wire_on_main_thread: bool,
    ) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let server_for_reader = server.clone();
        let server_for_after = server;
        let (pending_wire_tx, pending_wire_rx) = mpsc::channel::<Vec<u8>>();
        let (writer_raw, reader_raw) = wire_ipc::start_wire_threads(
            stop.clone(),
            reader_stream,
            writer_stream,
            rx,
            32,
            Duration::from_millis(4),
            move |frame: &[u8]| {
                if handle_wire_on_main_thread {
                    pending_wire_tx
                        .send(frame.to_vec())
                        .map_err(|_| "wire frame queue disconnected".to_string())?;
                    return Ok(());
                }
                let mut guard = server_for_reader
                    .lock()
                    .map_err(|_| "wire server lock poisoned".to_string())?;
                if !guard.handle_commands(frame) {
                    return Err("wire server failed to handle commands".to_string());
                }
                Ok(())
            },
            move || {
                if handle_wire_on_main_thread {
                    return;
                }
                if let Ok(mut guard) = server_for_after.lock() {
                    let _ = guard.flush();
                }
            },
            |_| {},
        );
        let (disconnect_tx, disconnect_rx) = mpsc::channel::<()>();
        let writer_thread = thread::spawn(move || {
            let res = match writer_raw.join() {
                Ok(res) => res,
                Err(_) => Err("wire writer thread panicked".to_string()),
            };
            if let Err(err) = &res {
                eprintln!("wire server writer thread ended with error: {err}");
            } else {
                eprintln!("wire server writer thread exited");
            }
            res
        });
        let reader_thread = thread::spawn(move || {
            let res = match reader_raw.join() {
                Ok(res) => res,
                Err(_) => Err("wire reader thread panicked".to_string()),
            };
            if let Err(err) = &res {
                eprintln!("wire server reader thread ended with error: {err}");
            } else {
                eprintln!("wire server reader thread exited");
            }
            let _ = disconnect_tx.send(());
            res
        });

        Self {
            stop,
            tx: Some(tx),
            writer_thread: Some(writer_thread),
            reader_thread: Some(reader_thread),
            disconnect_rx,
            pending_wire_rx: if handle_wire_on_main_thread {
                Some(pending_wire_rx)
            } else {
                None
            },
            handle_wire_on_main_thread,
        }
    }

    pub fn process_pending(
        &self,
        server: &Arc<Mutex<WireHelperServer>>,
    ) -> Result<(), WireServerError> {
        if !self.handle_wire_on_main_thread {
            return Ok(());
        }
        let Some(rx) = self.pending_wire_rx.as_ref() else {
            return Ok(());
        };
        while let Ok(frame) = rx.try_recv() {
            let mut guard = server
                .lock()
                .map_err(|_| WireServerError::Protocol("wire server lock poisoned"))?;
            let ok = guard.handle_commands(&frame);
            let _ = guard.flush();
            if !ok {
                return Err(WireServerError::Protocol(
                    "wire server failed to handle commands",
                ));
            }
        }
        Ok(())
    }

    pub fn is_disconnected(&self) -> bool {
        self.disconnect_rx.try_recv().is_ok()
    }
}

impl Drop for WireServerPump {
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
