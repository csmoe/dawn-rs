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

#[cfg(not(feature = "wire"))]
pub fn create_instance(desc: &wgpu::InstanceDescriptor) -> wgpu::Instance {
    use wgpu::custom::InstanceInterface;
    let custom = types::DawnInstance::new(desc);
    wgpu::Instance::from_custom(custom)
}

#[cfg(feature = "wire")]
pub fn create_instance(_desc: &wgpu::InstanceDescriptor) -> wgpu::Instance {
    let socket_name = std::env::var("DAWN_WGPU_WIRE_SOCKET")
        .unwrap_or_else(|_| "dawn-wgpu-wire-default".to_string());
    let connect_attempts = std::env::var("DAWN_WGPU_WIRE_CONNECT_ATTEMPTS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(5);
    let connect_delay_ms = std::env::var("DAWN_WGPU_WIRE_CONNECT_DELAY_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(10);
    let opts = wire_backend::WireInitOptions {
        reserve_surface: true,
        connect_attempts,
        connect_delay: std::time::Duration::from_millis(connect_delay_ms),
    };
    create_wire_instance_with_options(&socket_name, opts).unwrap_or_else(|err| {
        panic!("wire feature enabled: failed to create wire instance (socket={socket_name}): {err}")
    })
}

#[cfg(feature = "wire")]
fn create_wire_instance_with_options(
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
