use crate::types::*;
use dawn_rs::*;
use std::fmt;

#[derive(Debug)]
pub enum WgpuCompatError {
    NotCustomBackend,
    NotDawnBackend,
}

impl fmt::Display for WgpuCompatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WgpuCompatError::NotCustomBackend => write!(f, "wgpu object is not a custom backend"),
            WgpuCompatError::NotDawnBackend => write!(f, "wgpu custom backend is not dawn-rs"),
        }
    }
}

impl std::error::Error for WgpuCompatError {}

pub fn to_wgpu_instance(instance: Instance) -> wgpu::Instance {
    wgpu::Instance::from_custom(DawnInstance {
        inner: SendSync::new(instance),
    })
}

pub fn from_wgpu_instance(instance: &wgpu::Instance) -> Result<Instance, WgpuCompatError> {
    instance
        .as_custom::<DawnInstance>()
        .map(|i| i.inner.get())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn to_wgpu_adapter(adapter: Adapter) -> wgpu::Adapter {
    wgpu::Adapter::from_custom(DawnAdapter {
        inner: SendSync::new(adapter),
    })
}

pub fn from_wgpu_adapter(adapter: &wgpu::Adapter) -> Result<Adapter, WgpuCompatError> {
    adapter
        .as_custom::<DawnAdapter>()
        .map(|a| a.inner.get())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn to_wgpu_device(device: Device) -> wgpu::Device {
    wgpu::Device::from_custom(DawnDevice { inner: device })
}

pub fn from_wgpu_device(device: &wgpu::Device) -> Result<Device, WgpuCompatError> {
    device
        .as_custom::<DawnDevice>()
        .map(|d| d.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn to_wgpu_queue(queue: Queue) -> wgpu::Queue {
    wgpu::Queue::from_custom(DawnQueue { inner: queue })
}

pub fn from_wgpu_queue(queue: &wgpu::Queue) -> Result<Queue, WgpuCompatError> {
    queue
        .as_custom::<DawnQueue>()
        .map(|q| q.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_surface(surface: &wgpu::Surface) -> Result<Surface, WgpuCompatError> {
    surface
        .as_custom::<DawnSurface>()
        .map(|s| s.inner.get())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_buffer(buffer: &wgpu::Buffer) -> Result<Buffer, WgpuCompatError> {
    buffer
        .as_custom::<DawnBuffer>()
        .map(|b| b.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn to_wgpu_texture(texture: Texture, desc: &wgpu::TextureDescriptor<'_>) -> wgpu::Texture {
    wgpu::Texture::from_custom(DawnTexture { inner: texture }, desc)
}

pub fn from_wgpu_texture(texture: &wgpu::Texture) -> Result<Texture, WgpuCompatError> {
    texture
        .as_custom::<DawnTexture>()
        .map(|t| t.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_texture_view(view: &wgpu::TextureView) -> Result<TextureView, WgpuCompatError> {
    view.as_custom::<DawnTextureView>()
        .map(|v| v.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_sampler(sampler: &wgpu::Sampler) -> Result<Sampler, WgpuCompatError> {
    sampler
        .as_custom::<DawnSampler>()
        .map(|s| s.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_bind_group_layout(
    layout: &wgpu::BindGroupLayout,
) -> Result<BindGroupLayout, WgpuCompatError> {
    layout
        .as_custom::<DawnBindGroupLayout>()
        .map(|l| l.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_bind_group(group: &wgpu::BindGroup) -> Result<BindGroup, WgpuCompatError> {
    group
        .as_custom::<DawnBindGroup>()
        .map(|g| g.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_pipeline_layout(
    layout: &wgpu::PipelineLayout,
) -> Result<PipelineLayout, WgpuCompatError> {
    layout
        .as_custom::<DawnPipelineLayout>()
        .map(|l| l.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_render_pipeline(
    pipeline: &wgpu::RenderPipeline,
) -> Result<RenderPipeline, WgpuCompatError> {
    pipeline
        .as_custom::<DawnRenderPipeline>()
        .map(|p| p.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_compute_pipeline(
    pipeline: &wgpu::ComputePipeline,
) -> Result<ComputePipeline, WgpuCompatError> {
    pipeline
        .as_custom::<DawnComputePipeline>()
        .map(|p| p.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_shader_module(
    module: &wgpu::ShaderModule,
) -> Result<ShaderModule, WgpuCompatError> {
    module
        .as_custom::<DawnShaderModule>()
        .map(|m| m.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_command_encoder(
    encoder: &wgpu::CommandEncoder,
) -> Result<CommandEncoder, WgpuCompatError> {
    encoder
        .as_custom::<DawnCommandEncoder>()
        .map(|e| e.inner.get())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_command_buffer(
    buffer: &wgpu::CommandBuffer,
) -> Result<CommandBuffer, WgpuCompatError> {
    buffer
        .as_custom::<DawnCommandBuffer>()
        .map(|b| b.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_render_bundle(
    bundle: &wgpu::RenderBundle,
) -> Result<RenderBundle, WgpuCompatError> {
    bundle
        .as_custom::<DawnRenderBundle>()
        .map(|b| b.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn from_wgpu_query_set(query_set: &wgpu::QuerySet) -> Result<QuerySet, WgpuCompatError> {
    query_set
        .as_custom::<DawnQuerySet>()
        .map(|q| q.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}
