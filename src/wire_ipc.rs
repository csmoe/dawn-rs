use interprocess::local_socket::{
    GenericFilePath, GenericNamespaced, ListenerOptions, Name, Stream, prelude::*,
};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum OutboundPacket {
    Wire(Vec<u8>),
    Shutdown,
}

#[derive(Debug)]
pub enum IpcMessage {
    ReserveInstance {
        id: u32,
        generation: u32,
    },
    ReserveSurface {
        id: u32,
        generation: u32,
        instance_id: u32,
        instance_generation: u32,
    },
    ReserveAck {
        ok: bool,
    },
    ReserveWireTexture {
        id: u32,
        generation: u32,
        device_id: u32,
        device_generation: u32,
        width: u32,
        height: u32,
        format: u32,
        usage: u64,
    },
    ReserveWireTextureAck {
        ok: bool,
    },
    SetDxgiSharedTexture {
        handle: u64,
        use_keyed_mutex: bool,
    },
    SetDmabufSharedTexture {
        fd: i32,
        drm_format: u32,
        drm_modifier: u64,
        stride: u32,
        offset: u64,
    },
    /// Raw Dawn wire command stream bytes.
    ///
    /// This variant intentionally bypasses `postcard` and keeps a dedicated
    /// length-prefixed binary fast path to avoid extra serialization/copies on
    /// the hottest IPC path.
    WireBytes(Vec<u8>),
    HandleCommands,
    AnimationPhase {
        phase: f32,
    },
    FrameRgba {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
    FrameShm {
        width: u32,
        height: u32,
        len: u32,
        seq: u64,
    },
    FrameRequest {
        seq: u64,
        width: u32,
        height: u32,
        phase: f32,
    },
    FrameReady {
        seq: u64,
        width: u32,
        height: u32,
        len: u32,
    },
    Shutdown,
}

const TAG_RESERVE_INSTANCE: u8 = 1;
const TAG_RESERVE_ACK: u8 = 2;
/// Dedicated tag for raw Dawn wire command bytes.
///
/// Unlike control-plane messages, this payload is encoded/decoded with a
/// manual length-prefixed binary path for performance reasons.
const TAG_WIRE_BYTES: u8 = 3;
const TAG_HANDLE_COMMANDS: u8 = 4;
const TAG_SHUTDOWN: u8 = 5;
const TAG_ANIMATION_PHASE: u8 = 6;
const TAG_FRAME_RGBA: u8 = 7;
const TAG_FRAME_SHM: u8 = 8;
const TAG_FRAME_REQUEST: u8 = 9;
const TAG_FRAME_READY: u8 = 10;
const TAG_RESERVE_SURFACE: u8 = 11;
const TAG_RESERVE_WIRE_TEXTURE: u8 = 12;
const TAG_RESERVE_WIRE_TEXTURE_ACK: u8 = 13;
const TAG_SET_DXGI_SHARED_TEXTURE: u8 = 14;
const TAG_SET_DMABUF_SHARED_TEXTURE: u8 = 15;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ControlMessage {
    ReserveInstance {
        id: u32,
        generation: u32,
    },
    ReserveSurface {
        id: u32,
        generation: u32,
        instance_id: u32,
        instance_generation: u32,
    },
    ReserveAck {
        ok: bool,
    },
    ReserveWireTexture {
        id: u32,
        generation: u32,
        device_id: u32,
        device_generation: u32,
        width: u32,
        height: u32,
        format: u32,
        usage: u64,
    },
    ReserveWireTextureAck {
        ok: bool,
    },
    SetDxgiSharedTexture {
        handle: u64,
        use_keyed_mutex: bool,
    },
    SetDmabufSharedTexture {
        fd: i32,
        drm_format: u32,
        drm_modifier: u64,
        stride: u32,
        offset: u64,
    },
    HandleCommands,
    AnimationPhase {
        phase: f32,
    },
    FrameRgba {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
    FrameShm {
        width: u32,
        height: u32,
        len: u32,
        seq: u64,
    },
    FrameRequest {
        seq: u64,
        width: u32,
        height: u32,
        phase: f32,
    },
    FrameReady {
        seq: u64,
        width: u32,
        height: u32,
        len: u32,
    },
    Shutdown,
}

impl ControlMessage {
    fn tag(&self) -> u8 {
        match self {
            Self::ReserveInstance { .. } => TAG_RESERVE_INSTANCE,
            Self::ReserveSurface { .. } => TAG_RESERVE_SURFACE,
            Self::ReserveAck { .. } => TAG_RESERVE_ACK,
            Self::ReserveWireTexture { .. } => TAG_RESERVE_WIRE_TEXTURE,
            Self::ReserveWireTextureAck { .. } => TAG_RESERVE_WIRE_TEXTURE_ACK,
            Self::SetDxgiSharedTexture { .. } => TAG_SET_DXGI_SHARED_TEXTURE,
            Self::SetDmabufSharedTexture { .. } => TAG_SET_DMABUF_SHARED_TEXTURE,
            Self::HandleCommands => TAG_HANDLE_COMMANDS,
            Self::AnimationPhase { .. } => TAG_ANIMATION_PHASE,
            Self::FrameRgba { .. } => TAG_FRAME_RGBA,
            Self::FrameShm { .. } => TAG_FRAME_SHM,
            Self::FrameRequest { .. } => TAG_FRAME_REQUEST,
            Self::FrameReady { .. } => TAG_FRAME_READY,
            Self::Shutdown => TAG_SHUTDOWN,
        }
    }
}

impl TryFrom<&IpcMessage> for ControlMessage {
    type Error = &'static str;

    fn try_from(value: &IpcMessage) -> Result<Self, Self::Error> {
        Ok(match value {
            IpcMessage::ReserveInstance { id, generation } => Self::ReserveInstance {
                id: *id,
                generation: *generation,
            },
            IpcMessage::ReserveSurface {
                id,
                generation,
                instance_id,
                instance_generation,
            } => Self::ReserveSurface {
                id: *id,
                generation: *generation,
                instance_id: *instance_id,
                instance_generation: *instance_generation,
            },
            IpcMessage::ReserveAck { ok } => Self::ReserveAck { ok: *ok },
            IpcMessage::ReserveWireTexture {
                id,
                generation,
                device_id,
                device_generation,
                width,
                height,
                format,
                usage,
            } => Self::ReserveWireTexture {
                id: *id,
                generation: *generation,
                device_id: *device_id,
                device_generation: *device_generation,
                width: *width,
                height: *height,
                format: *format,
                usage: *usage,
            },
            IpcMessage::ReserveWireTextureAck { ok } => Self::ReserveWireTextureAck { ok: *ok },
            IpcMessage::SetDxgiSharedTexture {
                handle,
                use_keyed_mutex,
            } => Self::SetDxgiSharedTexture {
                handle: *handle,
                use_keyed_mutex: *use_keyed_mutex,
            },
            IpcMessage::SetDmabufSharedTexture {
                fd,
                drm_format,
                drm_modifier,
                stride,
                offset,
            } => Self::SetDmabufSharedTexture {
                fd: *fd,
                drm_format: *drm_format,
                drm_modifier: *drm_modifier,
                stride: *stride,
                offset: *offset,
            },
            IpcMessage::HandleCommands => Self::HandleCommands,
            IpcMessage::AnimationPhase { phase } => Self::AnimationPhase { phase: *phase },
            IpcMessage::FrameRgba {
                width,
                height,
                data,
            } => Self::FrameRgba {
                width: *width,
                height: *height,
                data: data.clone(),
            },
            IpcMessage::FrameShm {
                width,
                height,
                len,
                seq,
            } => Self::FrameShm {
                width: *width,
                height: *height,
                len: *len,
                seq: *seq,
            },
            IpcMessage::FrameRequest {
                seq,
                width,
                height,
                phase,
            } => Self::FrameRequest {
                seq: *seq,
                width: *width,
                height: *height,
                phase: *phase,
            },
            IpcMessage::FrameReady {
                seq,
                width,
                height,
                len,
            } => Self::FrameReady {
                seq: *seq,
                width: *width,
                height: *height,
                len: *len,
            },
            IpcMessage::Shutdown => Self::Shutdown,
            IpcMessage::WireBytes(_) => return Err("wire bytes are not postcard control messages"),
        })
    }
}

impl From<ControlMessage> for IpcMessage {
    fn from(value: ControlMessage) -> Self {
        match value {
            ControlMessage::ReserveInstance { id, generation } => {
                IpcMessage::ReserveInstance { id, generation }
            }
            ControlMessage::ReserveSurface {
                id,
                generation,
                instance_id,
                instance_generation,
            } => IpcMessage::ReserveSurface {
                id,
                generation,
                instance_id,
                instance_generation,
            },
            ControlMessage::ReserveAck { ok } => IpcMessage::ReserveAck { ok },
            ControlMessage::ReserveWireTexture {
                id,
                generation,
                device_id,
                device_generation,
                width,
                height,
                format,
                usage,
            } => IpcMessage::ReserveWireTexture {
                id,
                generation,
                device_id,
                device_generation,
                width,
                height,
                format,
                usage,
            },
            ControlMessage::ReserveWireTextureAck { ok } => {
                IpcMessage::ReserveWireTextureAck { ok }
            }
            ControlMessage::SetDxgiSharedTexture {
                handle,
                use_keyed_mutex,
            } => IpcMessage::SetDxgiSharedTexture {
                handle,
                use_keyed_mutex,
            },
            ControlMessage::SetDmabufSharedTexture {
                fd,
                drm_format,
                drm_modifier,
                stride,
                offset,
            } => IpcMessage::SetDmabufSharedTexture {
                fd,
                drm_format,
                drm_modifier,
                stride,
                offset,
            },
            ControlMessage::HandleCommands => IpcMessage::HandleCommands,
            ControlMessage::AnimationPhase { phase } => IpcMessage::AnimationPhase { phase },
            ControlMessage::FrameRgba {
                width,
                height,
                data,
            } => IpcMessage::FrameRgba {
                width,
                height,
                data,
            },
            ControlMessage::FrameShm {
                width,
                height,
                len,
                seq,
            } => IpcMessage::FrameShm {
                width,
                height,
                len,
                seq,
            },
            ControlMessage::FrameRequest {
                seq,
                width,
                height,
                phase,
            } => IpcMessage::FrameRequest {
                seq,
                width,
                height,
                phase,
            },
            ControlMessage::FrameReady {
                seq,
                width,
                height,
                len,
            } => IpcMessage::FrameReady {
                seq,
                width,
                height,
                len,
            },
            ControlMessage::Shutdown => IpcMessage::Shutdown,
        }
    }
}

pub fn start_wire_threads(
    stop: Arc<AtomicBool>,
    mut reader_stream: Stream,
    mut writer_stream: Stream,
    outbound_rx: mpsc::Receiver<OutboundPacket>,
    max_batch_packets: usize,
    flush_interval: Duration,
    max_pending_packets: usize,
    max_pending_bytes: usize,
    mut handle_frame: impl FnMut(&[u8]) -> Result<(), String> + Send + 'static,
    mut after_handle_commands: impl FnMut() + Send + 'static,
    mut on_animation_phase: impl FnMut(f32) + Send + 'static,
    mut recycle_packet: impl FnMut(Vec<u8>) + Send + 'static,
) -> (
    JoinHandle<Result<(), String>>,
    JoinHandle<Result<(), String>>,
) {
    let writer_stop = stop.clone();
    let writer_thread = thread::spawn(move || -> Result<(), String> {
        let mut batch: Vec<Vec<u8>> = Vec::new();

        loop {
            let packet = match outbound_rx.recv() {
                Ok(packet) => packet,
                Err(_) => {
                    flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
                    break;
                }
            };
            match packet {
                OutboundPacket::Wire(packet) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
                        break;
                    }
                    batch.push(packet);
                    if batch.len() >= max_batch_packets.max(1) {
                        flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
                    }
                }
                OutboundPacket::Shutdown => {
                    flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
                    let _ = write_message(&mut writer_stream, &IpcMessage::Shutdown);
                    let _ = writer_stream.flush();
                    break;
                }
            }

            if batch.is_empty() {
                continue;
            }

            let deadline = Instant::now() + flush_interval;
            while batch.len() < max_batch_packets.max(1) {
                let timeout = deadline.saturating_duration_since(Instant::now());
                if timeout.is_zero() {
                    break;
                }
                match outbound_rx.recv_timeout(timeout) {
                    Ok(OutboundPacket::Wire(packet)) => {
                        if writer_stop.load(Ordering::Relaxed) {
                            flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
                            return Ok(());
                        }
                        batch.push(packet);
                    }
                    Ok(OutboundPacket::Shutdown) => {
                        flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
                        let _ = write_message(&mut writer_stream, &IpcMessage::Shutdown);
                        let _ = writer_stream.flush();
                        return Ok(());
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
                        return Ok(());
                    }
                }
            }
            flush_batch(&mut writer_stream, &mut batch, &mut recycle_packet)?;
        }
        Ok(())
    });

    let reader_stop = stop;
    let reader_thread = thread::spawn(move || -> Result<(), String> {
        let mut pending = Vec::<u8>::new();
        let mut pending_packets = 0usize;
        let mut consume_pending =
            |pending: &mut Vec<u8>, pending_packets: &mut usize| -> Result<(), String> {
                if pending.is_empty() {
                    return Ok(());
                }
                handle_frame(pending)?;
                after_handle_commands();
                pending.clear();
                *pending_packets = 0;
                Ok(())
            };

        while !reader_stop.load(Ordering::Relaxed) {
            let message = match read_message(&mut reader_stream) {
                Ok(message) => message,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    consume_pending(&mut pending, &mut pending_packets)?;
                    break;
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    continue;
                }
                Err(e) => return Err(e.to_string()),
            };

            match message {
                IpcMessage::WireBytes(bytes) => {
                    if max_pending_packets > 0 && pending_packets >= max_pending_packets {
                        return Err(format!(
                            "wire pending packet limit exceeded: {}",
                            max_pending_packets
                        ));
                    }
                    if max_pending_bytes > 0
                        && pending.len().saturating_add(bytes.len()) > max_pending_bytes
                    {
                        return Err(format!(
                            "wire pending byte limit exceeded: {}",
                            max_pending_bytes
                        ));
                    }
                    pending.extend_from_slice(&bytes);
                    pending_packets += 1;
                }
                IpcMessage::HandleCommands => consume_pending(&mut pending, &mut pending_packets)?,
                IpcMessage::AnimationPhase { phase } => on_animation_phase(phase),
                IpcMessage::Shutdown => {
                    consume_pending(&mut pending, &mut pending_packets)?;
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    });

    (writer_thread, reader_thread)
}

pub fn to_ipc_name(name: &str) -> std::io::Result<Name<'static>> {
    if GenericNamespaced::is_supported() {
        name.to_string()
            .to_ns_name::<GenericNamespaced>()
            .map_err(std::io::Error::other)
    } else {
        format!("/tmp/{name}.sock")
            .to_fs_name::<GenericFilePath>()
            .map_err(std::io::Error::other)
    }
}

pub fn connect_with_retry(name: &str, attempts: usize, delay: Duration) -> std::io::Result<Stream> {
    let ipc_name = to_ipc_name(name)?;
    let mut last_err: Option<std::io::Error> = None;
    for _ in 0..attempts.max(1) {
        match Stream::connect(ipc_name.clone()) {
            Ok(stream) => return Ok(stream),
            Err(e) => {
                last_err = Some(e);
                std::thread::sleep(delay);
            }
        }
    }
    Err(last_err.unwrap_or_else(|| std::io::Error::other("failed to connect ipc socket")))
}

pub fn bind_and_accept(name: &str) -> std::io::Result<Stream> {
    let listener = ListenerOptions::new()
        .name(to_ipc_name(name)?)
        .reclaim_name(true)
        .create_sync()?;
    listener.accept()
}

fn flush_batch<W: Write, F: FnMut(Vec<u8>)>(
    writer: &mut W,
    batch: &mut Vec<Vec<u8>>,
    recycle_packet: &mut F,
) -> Result<(), String> {
    if batch.is_empty() {
        return Ok(());
    }
    for packet in batch.drain(..) {
        write_wire_bytes(writer, &packet).map_err(|e| e.to_string())?;
        recycle_packet(packet);
    }
    write_message(writer, &IpcMessage::HandleCommands).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}

fn write_wire_bytes<W: Write>(writer: &mut W, payload: &[u8]) -> std::io::Result<()> {
    writer.write_all(&[TAG_WIRE_BYTES])?;
    let len = u32::try_from(payload.len()).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "wire packet too large")
    })?;
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(payload)?;
    Ok(())
}

pub fn write_message<W: Write>(writer: &mut W, message: &IpcMessage) -> std::io::Result<()> {
    match message {
        IpcMessage::WireBytes(bytes) => {
            // Wire command payload uses a special raw path and does not go
            // through postcard; keep this branch allocation-light.
            writer.write_all(&[TAG_WIRE_BYTES])?;
            write_len_prefixed_bytes(writer, bytes)?;
        }
        _ => {
            let control = ControlMessage::try_from(message)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
            let payload = postcard::to_allocvec(&control).map_err(std::io::Error::other)?;
            writer.write_all(&[control.tag()])?;
            write_len_prefixed_bytes(writer, &payload)?;
        }
    }
    Ok(())
}

pub fn read_message<R: Read>(reader: &mut R) -> std::io::Result<IpcMessage> {
    let mut tag = [0u8; 1];
    reader.read_exact(&mut tag)?;
    if tag[0] == TAG_WIRE_BYTES {
        // Symmetric fast path for raw Dawn wire bytes; control messages are
        // decoded below via postcard.
        return Ok(IpcMessage::WireBytes(read_len_prefixed_bytes(reader)?));
    }

    let payload = read_len_prefixed_bytes(reader)?;
    let control: ControlMessage = postcard::from_bytes(&payload).map_err(std::io::Error::other)?;
    if control.tag() != tag[0] {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "ipc control message tag mismatch",
        ));
    }
    let message: IpcMessage = control.into();
    if let IpcMessage::FrameRgba {
        width,
        height,
        data,
    } = &message
    {
        let expected = *width as usize * *height as usize * 4;
        if data.len() != expected {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "frame rgba size mismatch",
            ));
        }
    }
    Ok(message)
}

fn write_len_prefixed_bytes<W: Write>(writer: &mut W, bytes: &[u8]) -> std::io::Result<()> {
    let len = u32::try_from(bytes.len())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "payload too large"))?;
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(bytes)
}

fn read_len_prefixed_bytes<R: Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes)?;
    let len = u32::from_le_bytes(len_bytes) as usize;
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
