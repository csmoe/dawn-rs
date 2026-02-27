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

pub fn create_instance(desc: &wgpu::InstanceDescriptor) -> wgpu::Instance {
    use wgpu::custom::InstanceInterface;
    let custom = types::DawnInstance::new(desc);
    wgpu::Instance::from_custom(custom)
}
