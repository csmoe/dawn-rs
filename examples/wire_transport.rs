use interprocess::local_socket::Stream;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub enum OutboundPacket {
    Wire(Vec<u8>),
    Shutdown,
}

pub struct PumpGuard {
    stop: Arc<AtomicBool>,
    tx: Option<mpsc::Sender<OutboundPacket>>,
    writer_thread: Option<JoinHandle<Result<(), String>>>,
    reader_thread: Option<JoinHandle<Result<(), String>>>,
}

impl PumpGuard {
    pub fn new(
        stop: Arc<AtomicBool>,
        tx: mpsc::Sender<OutboundPacket>,
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

    pub fn shutdown_and_join(mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(OutboundPacket::Shutdown);
        }
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
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(OutboundPacket::Shutdown);
        }
        if let Some(writer_thread) = self.writer_thread.take() {
            let _ = writer_thread.join();
        }
        if let Some(reader_thread) = self.reader_thread.take() {
            let _ = reader_thread.join();
        }
    }
}

pub fn start_transport_threads(
    stop: Arc<AtomicBool>,
    mut reader_stream: Stream,
    mut writer_stream: Stream,
    to_writer_rx: mpsc::Receiver<OutboundPacket>,
    mut handle_packet: impl FnMut(&[u8]) -> Result<(), String> + Send + 'static,
    mut after_handle_commands: impl FnMut() + Send + 'static,
) -> (
    JoinHandle<Result<(), String>>,
    JoinHandle<Result<(), String>>,
) {
    let writer_stop = stop.clone();
    let writer_thread = thread::spawn(move || -> Result<(), String> {
        loop {
            match to_writer_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(OutboundPacket::Wire(packet)) => {
                    if writer_stop.load(Ordering::Relaxed) {
                        break;
                    }
                    write_message(&mut writer_stream, &IpcMessage::WireBytes(packet))
                        .map_err(|e| e.to_string())?;
                    write_message(&mut writer_stream, &IpcMessage::HandleCommands)
                        .map_err(|e| e.to_string())?;
                    writer_stream.flush().map_err(|e| e.to_string())?;
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

    let reader_stop = stop;
    let reader_thread = thread::spawn(move || -> Result<(), String> {
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
                Err(e) => return Err(e.to_string()),
            };
            match message {
                // Opaque Dawn wire bytes from peer Flush.
                IpcMessage::WireBytes(bytes) => pending.push(bytes),
                IpcMessage::HandleCommands => {
                    if pending.is_empty() {
                        continue;
                    }
                    for frame in pending.drain(..) {
                        handle_packet(&frame)?;
                    }
                    after_handle_commands();
                }
                IpcMessage::Shutdown => break,
                _ => continue,
            }
        }
        Ok(())
    });

    (writer_thread, reader_thread)
}

pub fn write_message<W: Write>(writer: &mut W, message: &IpcMessage) -> std::io::Result<()> {
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
        IpcMessage::HandleCommands => {
            writer.write_all(&[4])?;
        }
        IpcMessage::Shutdown => {
            writer.write_all(&[5])?;
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

#[derive(Debug)]
pub enum IpcMessage {
    ReserveInstance { id: u32, generation: u32 },
    ReserveAck { ok: bool },
    WireBytes(Vec<u8>),
    HandleCommands,
    Shutdown,
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
