use crate::Compat;
use dawn_rs::wire::{
    Client as DawnWireClient, ClientOptions, ReservedTexture as DawnReservedTexture, WireError,
    WireHandle, WireTextureReservation,
};
use dawn_rs::{Instance, Surface};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WireBackendError {
    #[error(transparent)]
    Wire(#[from] WireError),
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

#[derive(Debug, Clone)]
pub struct ReservedWireTexture {
    pub texture: dawn_rs::Texture,
    pub texture_handle: WireHandle,
    pub device_handle: WireHandle,
    pub width: u32,
    pub height: u32,
    pub format: dawn_rs::TextureFormat,
    pub usage: dawn_rs::TextureUsage,
}

impl ReservedWireTexture {
    pub fn reservation(&self) -> WireTextureReservation {
        WireTextureReservation::new(
            self.texture_handle,
            self.device_handle,
            self.width,
            self.height,
            self.format as u32,
            self.usage.bits(),
        )
    }
}

pub struct IpcWireBackend {
    client: DawnWireClient,
}

impl IpcWireBackend {
    pub fn connect_name_with_options(
        name: &str,
        opts: WireInitOptions,
    ) -> Result<Self, WireBackendError> {
        let client = DawnWireClient::connect(
            name,
            ClientOptions {
                reserve_surface: opts.reserve_surface,
                connect_attempts: opts.connect_attempts,
                connect_delay: opts.connect_delay,
                max_allocation_size: 0,
            },
        )?;
        Ok(Self { client })
    }

    pub fn connect_name(name: &str) -> Result<Self, WireBackendError> {
        Self::connect_name_with_options(name, WireInitOptions::default())
    }

    pub fn dawn_instance(&self) -> Instance {
        self.client.instance()
    }

    pub fn dawn_surface(&self) -> Option<Surface> {
        self.client.surface()
    }

    pub fn wgpu_instance(&self) -> wgpu::Instance {
        Compat::from(self.dawn_instance()).into()
    }

    pub fn pump(&self) {
        self.client.pump();
    }

    pub fn reserve_texture(
        &self,
        device: &dawn_rs::Device,
        width: u32,
        height: u32,
        format: dawn_rs::TextureFormat,
        usage: dawn_rs::TextureUsage,
    ) -> Result<ReservedWireTexture, WireBackendError> {
        let reserved: DawnReservedTexture = self
            .client
            .reserve_texture(device, width, height, format, usage)?;
        Ok(ReservedWireTexture {
            texture: reserved.texture,
            texture_handle: reserved.reservation.texture_handle(),
            device_handle: reserved.reservation.device_handle(),
            width: reserved.width,
            height: reserved.height,
            format,
            usage,
        })
    }

    pub fn into_instance_and_handle(self) -> (Instance, Arc<WireBackendHandle>) {
        let instance = self.dawn_instance();
        (instance, Arc::new(WireBackendHandle::new(self)))
    }
}

impl From<&IpcWireBackend> for wgpu::Instance {
    fn from(value: &IpcWireBackend) -> Self {
        value.wgpu_instance()
    }
}

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
