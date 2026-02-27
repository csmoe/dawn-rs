use dawn_rs::wire_ipc::{self, IpcMessage};
use dawn_rs::wire_ipc::OutboundPacket;
use dawn_rs::wire_shim::WireInstanceHandle;
use dawn_rs::wire_shim::WireHelperServer;
use interprocess::TryClone;
use interprocess::local_socket::Stream;
use std::fmt;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

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
