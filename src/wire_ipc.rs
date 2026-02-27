use interprocess::local_socket::{
    GenericFilePath, GenericNamespaced, ListenerOptions, Name, Stream, prelude::*,
};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub enum OutboundPacket {
    Wire(Vec<u8>),
    AnimationPhase(f32),
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

pub fn start_wire_threads(
    stop: Arc<AtomicBool>,
    mut reader_stream: Stream,
    mut writer_stream: Stream,
    outbound_rx: mpsc::Receiver<OutboundPacket>,
    max_batch_packets: usize,
    flush_interval: Duration,
    mut handle_frame: impl FnMut(&[u8]) -> Result<(), String> + Send + 'static,
    mut after_handle_commands: impl FnMut() + Send + 'static,
    mut on_animation_phase: impl FnMut(f32) + Send + 'static,
) -> (
    JoinHandle<Result<(), String>>,
    JoinHandle<Result<(), String>>,
) {
    let writer_stop = stop.clone();
    let writer_thread = thread::spawn(move || -> Result<(), String> {
        let mut batch: Vec<Vec<u8>> = Vec::new();

        loop {
            match outbound_rx.recv_timeout(flush_interval) {
                Ok(OutboundPacket::Wire(packet)) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        flush_batch(&mut writer_stream, &mut batch)?;
                        break;
                    }
                    batch.push(packet);
                    if batch.len() >= max_batch_packets.max(1) {
                        flush_batch(&mut writer_stream, &mut batch)?;
                    }
                }
                Ok(OutboundPacket::Shutdown) => {
                    flush_batch(&mut writer_stream, &mut batch)?;
                    let _ = write_message(&mut writer_stream, &IpcMessage::Shutdown);
                    let _ = writer_stream.flush();
                    break;
                }
                Ok(OutboundPacket::AnimationPhase(phase)) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        flush_batch(&mut writer_stream, &mut batch)?;
                        break;
                    }
                    flush_batch(&mut writer_stream, &mut batch)?;
                    write_message(&mut writer_stream, &IpcMessage::AnimationPhase { phase })
                        .map_err(|e| e.to_string())?;
                    writer_stream.flush().map_err(|e| e.to_string())?;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    flush_batch(&mut writer_stream, &mut batch)?;
                    if writer_stop.load(Ordering::Relaxed) {
                        break;
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    flush_batch(&mut writer_stream, &mut batch)?;
                    break;
                }
            }
        }
        Ok(())
    });

    let reader_stop = stop;
    let reader_thread = thread::spawn(move || -> Result<(), String> {
        let mut pending = Vec::<Vec<u8>>::new();
        let mut consume_pending = |pending: &mut Vec<Vec<u8>>| -> Result<(), String> {
            if pending.is_empty() {
                return Ok(());
            }
            for frame in pending.drain(..) {
                handle_frame(&frame)?;
            }
            after_handle_commands();
            Ok(())
        };

        while !reader_stop.load(Ordering::Relaxed) {
            let message = match read_message(&mut reader_stream) {
                Ok(message) => message,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    consume_pending(&mut pending)?;
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
                IpcMessage::WireBytes(bytes) => pending.push(bytes),
                IpcMessage::HandleCommands => consume_pending(&mut pending)?,
                IpcMessage::AnimationPhase { phase } => on_animation_phase(phase),
                IpcMessage::Shutdown => {
                    consume_pending(&mut pending)?;
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

fn flush_batch<W: Write>(writer: &mut W, batch: &mut Vec<Vec<u8>>) -> Result<(), String> {
    if batch.is_empty() {
        return Ok(());
    }
    for packet in batch.drain(..) {
        write_message(writer, &IpcMessage::WireBytes(packet)).map_err(|e| e.to_string())?;
    }
    write_message(writer, &IpcMessage::HandleCommands).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn write_message<W: Write>(writer: &mut W, message: &IpcMessage) -> std::io::Result<()> {
    match message {
        IpcMessage::ReserveInstance { id, generation } => {
            writer.write_all(&[1])?;
            writer.write_all(&id.to_le_bytes())?;
            writer.write_all(&generation.to_le_bytes())?;
        }
        IpcMessage::ReserveSurface {
            id,
            generation,
            instance_id,
            instance_generation,
        } => {
            writer.write_all(&[11])?;
            writer.write_all(&id.to_le_bytes())?;
            writer.write_all(&generation.to_le_bytes())?;
            writer.write_all(&instance_id.to_le_bytes())?;
            writer.write_all(&instance_generation.to_le_bytes())?;
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
        IpcMessage::AnimationPhase { phase } => {
            writer.write_all(&[6])?;
            writer.write_all(&phase.to_le_bytes())?;
        }
        IpcMessage::FrameRgba {
            width,
            height,
            data,
        } => {
            writer.write_all(&[7])?;
            writer.write_all(&width.to_le_bytes())?;
            writer.write_all(&height.to_le_bytes())?;
            write_len_prefixed_bytes(writer, data)?;
        }
        IpcMessage::FrameShm {
            width,
            height,
            len,
            seq,
        } => {
            writer.write_all(&[8])?;
            writer.write_all(&width.to_le_bytes())?;
            writer.write_all(&height.to_le_bytes())?;
            writer.write_all(&len.to_le_bytes())?;
            writer.write_all(&seq.to_le_bytes())?;
        }
        IpcMessage::FrameRequest {
            seq,
            width,
            height,
            phase,
        } => {
            writer.write_all(&[9])?;
            writer.write_all(&seq.to_le_bytes())?;
            writer.write_all(&width.to_le_bytes())?;
            writer.write_all(&height.to_le_bytes())?;
            writer.write_all(&phase.to_le_bytes())?;
        }
        IpcMessage::FrameReady {
            seq,
            width,
            height,
            len,
        } => {
            writer.write_all(&[10])?;
            writer.write_all(&seq.to_le_bytes())?;
            writer.write_all(&width.to_le_bytes())?;
            writer.write_all(&height.to_le_bytes())?;
            writer.write_all(&len.to_le_bytes())?;
        }
    }
    Ok(())
}

pub fn read_message<R: Read>(reader: &mut R) -> std::io::Result<IpcMessage> {
    let mut tag = [0u8; 1];
    reader.read_exact(&mut tag)?;
    match tag[0] {
        1 => {
            let id = read_u32(reader)?;
            let generation = read_u32(reader)?;
            Ok(IpcMessage::ReserveInstance { id, generation })
        }
        11 => {
            let id = read_u32(reader)?;
            let generation = read_u32(reader)?;
            let instance_id = read_u32(reader)?;
            let instance_generation = read_u32(reader)?;
            Ok(IpcMessage::ReserveSurface {
                id,
                generation,
                instance_id,
                instance_generation,
            })
        }
        2 => {
            let ok = read_u8(reader)? != 0;
            Ok(IpcMessage::ReserveAck { ok })
        }
        3 => Ok(IpcMessage::WireBytes(read_len_prefixed_bytes(reader)?)),
        4 => Ok(IpcMessage::HandleCommands),
        5 => Ok(IpcMessage::Shutdown),
        6 => {
            let mut b = [0u8; 4];
            reader.read_exact(&mut b)?;
            Ok(IpcMessage::AnimationPhase {
                phase: f32::from_le_bytes(b),
            })
        }
        7 => {
            let width = read_u32(reader)?;
            let height = read_u32(reader)?;
            let data = read_len_prefixed_bytes(reader)?;
            let expected = width as usize * height as usize * 4;
            if data.len() != expected {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "frame rgba size mismatch",
                ));
            }
            Ok(IpcMessage::FrameRgba {
                width,
                height,
                data,
            })
        }
        8 => {
            let width = read_u32(reader)?;
            let height = read_u32(reader)?;
            let len = read_u32(reader)?;
            let mut seq_bytes = [0u8; 8];
            reader.read_exact(&mut seq_bytes)?;
            Ok(IpcMessage::FrameShm {
                width,
                height,
                len,
                seq: u64::from_le_bytes(seq_bytes),
            })
        }
        9 => {
            let mut seq_bytes = [0u8; 8];
            reader.read_exact(&mut seq_bytes)?;
            let width = read_u32(reader)?;
            let height = read_u32(reader)?;
            let mut phase_bytes = [0u8; 4];
            reader.read_exact(&mut phase_bytes)?;
            Ok(IpcMessage::FrameRequest {
                seq: u64::from_le_bytes(seq_bytes),
                width,
                height,
                phase: f32::from_le_bytes(phase_bytes),
            })
        }
        10 => {
            let mut seq_bytes = [0u8; 8];
            reader.read_exact(&mut seq_bytes)?;
            let width = read_u32(reader)?;
            let height = read_u32(reader)?;
            let len = read_u32(reader)?;
            Ok(IpcMessage::FrameReady {
                seq: u64::from_le_bytes(seq_bytes),
                width,
                height,
                len,
            })
        }
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
