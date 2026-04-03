#![allow(unexpected_cfgs, dead_code, unreachable_patterns)]

mod backend;
mod compat;
mod dispatch;
mod error;
mod future;
mod mapping;
mod types;

pub use compat::{DawnIntoWgpu, DawnTextureIntoWgpu};
use std::sync::Arc;
pub use types::*;

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
    let custom =
        types::DawnInstance::from_factory(wgpu::Backends::all(), move || instance, Some(handle));
    Ok(wgpu::Instance::from_custom(custom))
}

pub async fn request_adapter_with_fallback(
    instance: &wgpu::Instance,
    options: &wgpu::RequestAdapterOptions<'_, '_>,
    backends: wgpu::Backends,
) -> Result<wgpu::Adapter, wgpu::RequestAdapterError> {
    let instance = instance
        .as_custom::<types::DawnInstance>()
        .ok_or(wgpu::RequestAdapterError::NotFound {
            active_backends: wgpu::Backends::empty(),
            requested_backends: backends,
            supported_backends: wgpu::Backends::empty(),
            no_fallback_backends: wgpu::Backends::empty(),
            no_adapter_backends: wgpu::Backends::empty(),
            incompatible_surface_backends: wgpu::Backends::empty(),
        })?
        .clone();

    let worker = Arc::clone(&instance.inner);
    let power_preference = mapping::map_power_preference(options.power_preference);
    let force_fallback_adapter = options.force_fallback_adapter;
    let compatible_surface = options
        .compatible_surface
        .map(|surface| dispatch::expect_surface_from_api(surface).inner.clone());

    instance.with_instance(move |state| {
        for dawn_options in backend::adapter_attempts(
            backends,
            power_preference,
            compatible_surface,
            force_fallback_adapter,
        ) {
            if let Ok(adapter) = backend::request_adapter_sync(&state.instance, &dawn_options) {
                if dawn_options.force_fallback_adapter == Some(true)
                    && !backend::is_swiftshader_adapter(&adapter)
                {
                    continue;
                }

                return Ok(wgpu::Adapter::from_custom(
                    types::DawnAdapter::from_adapter(Arc::clone(&worker), adapter),
                ));
            }
        }

        Err(wgpu::RequestAdapterError::NotFound {
            active_backends: state.backends,
            requested_backends: backends,
            supported_backends: wgpu::Backends::PRIMARY | wgpu::Backends::GL,
            no_fallback_backends: wgpu::Backends::empty(),
            no_adapter_backends: wgpu::Backends::empty(),
            incompatible_surface_backends: wgpu::Backends::empty(),
        })
    })
}
