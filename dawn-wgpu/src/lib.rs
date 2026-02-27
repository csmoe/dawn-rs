#![allow(unexpected_cfgs, dead_code, unreachable_patterns)]

mod backend;
mod compat;
mod dispatch;
mod error;
mod future;
mod mapping;
mod types;

pub use compat::{Compat, CompatTexture, WgpuCompatError};

#[cfg(feature = "wire")]
pub mod wire_backend;
#[cfg(feature = "wire")]
pub mod wire_server;

pub fn create_instance(desc: &wgpu::InstanceDescriptor) -> wgpu::Instance {
    use wgpu::custom::InstanceInterface;
    let custom = types::DawnInstance::new(desc);
    wgpu::Instance::from_custom(custom)
}

#[cfg(feature = "wire")]
pub fn create_wire_instance(name: &str) -> Result<wgpu::Instance, wire_backend::WireBackendError> {
    create_wire_instance_with_options(name, wire_backend::WireInitOptions::default())
}

#[cfg(feature = "wire")]
pub fn create_wire_instance_with_options(
    name: &str,
    opts: wire_backend::WireInitOptions,
) -> Result<wgpu::Instance, wire_backend::WireBackendError> {
    let backend = wire_backend::IpcWireBackend::connect_name_with_options(name, opts)?;
    let (instance, handle) = backend.into_instance_and_handle();
    let custom = types::DawnInstance {
        inner: types::SendSync::new(instance),
        wire_handle: Some(handle),
    };
    Ok(wgpu::Instance::from_custom(custom))
}
