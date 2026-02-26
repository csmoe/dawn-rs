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

#[derive(Debug, Clone)]
pub struct Compat<T>(T);

impl<T> Compat<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Compat<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct CompatTexture<'a> {
    texture: Texture,
    desc: &'a wgpu::TextureDescriptor<'a>,
}

impl<'a> CompatTexture<'a> {
    pub fn new(texture: Texture, desc: &'a wgpu::TextureDescriptor<'a>) -> Self {
        Self { texture, desc }
    }
}

impl From<Compat<Instance>> for wgpu::Instance {
    fn from(value: Compat<Instance>) -> Self {
        wgpu::Instance::from_custom(DawnInstance {
            inner: SendSync::new(value.0),
        })
    }
}

impl TryFrom<&wgpu::Instance> for Compat<Instance> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Instance) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnInstance>()
            .map(|i| Compat(i.inner.get()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl From<Compat<Adapter>> for wgpu::Adapter {
    fn from(value: Compat<Adapter>) -> Self {
        wgpu::Adapter::from_custom(DawnAdapter {
            inner: SendSync::new(value.0),
        })
    }
}

impl TryFrom<&wgpu::Adapter> for Compat<Adapter> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Adapter) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnAdapter>()
            .map(|a| Compat(a.inner.get()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl From<Compat<Device>> for wgpu::Device {
    fn from(value: Compat<Device>) -> Self {
        wgpu::Device::from_custom(DawnDevice { inner: value.0 })
    }
}

impl TryFrom<&wgpu::Device> for Compat<Device> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Device) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnDevice>()
            .map(|d| Compat(d.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl From<Compat<Queue>> for wgpu::Queue {
    fn from(value: Compat<Queue>) -> Self {
        wgpu::Queue::from_custom(DawnQueue { inner: value.0 })
    }
}

impl TryFrom<&wgpu::Queue> for Compat<Queue> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Queue) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnQueue>()
            .map(|q| Compat(q.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl From<CompatTexture<'_>> for wgpu::Texture {
    fn from(value: CompatTexture<'_>) -> Self {
        wgpu::Texture::from_custom(
            DawnTexture {
                inner: value.texture,
            },
            value.desc,
        )
    }
}

impl TryFrom<&wgpu::Texture> for Compat<Texture> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Texture) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnTexture>()
            .map(|t| Compat(t.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::Surface<'_>> for Compat<Surface> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Surface<'_>) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnSurface>()
            .map(|s| Compat(s.inner.get()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::Buffer> for Compat<Buffer> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Buffer) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnBuffer>()
            .map(|b| Compat(b.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::TextureView> for Compat<TextureView> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::TextureView) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnTextureView>()
            .map(|v| Compat(v.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::Sampler> for Compat<Sampler> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::Sampler) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnSampler>()
            .map(|s| Compat(s.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::BindGroupLayout> for Compat<BindGroupLayout> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::BindGroupLayout) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnBindGroupLayout>()
            .map(|l| Compat(l.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::BindGroup> for Compat<BindGroup> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::BindGroup) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnBindGroup>()
            .map(|g| Compat(g.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::PipelineLayout> for Compat<PipelineLayout> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::PipelineLayout) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnPipelineLayout>()
            .map(|l| Compat(l.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::RenderPipeline> for Compat<RenderPipeline> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::RenderPipeline) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnRenderPipeline>()
            .map(|p| Compat(p.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::ComputePipeline> for Compat<ComputePipeline> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::ComputePipeline) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnComputePipeline>()
            .map(|p| Compat(p.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::ShaderModule> for Compat<ShaderModule> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::ShaderModule) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnShaderModule>()
            .map(|m| Compat(m.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::CommandEncoder> for Compat<CommandEncoder> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::CommandEncoder) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnCommandEncoder>()
            .map(|e| Compat(e.inner.get()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::CommandBuffer> for Compat<CommandBuffer> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::CommandBuffer) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnCommandBuffer>()
            .map(|b| Compat(b.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::RenderBundle> for Compat<RenderBundle> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::RenderBundle) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnRenderBundle>()
            .map(|b| Compat(b.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}

impl TryFrom<&wgpu::QuerySet> for Compat<QuerySet> {
    type Error = WgpuCompatError;

    fn try_from(value: &wgpu::QuerySet) -> Result<Self, Self::Error> {
        value
            .as_custom::<DawnQuerySet>()
            .map(|q| Compat(q.inner.clone()))
            .ok_or(WgpuCompatError::NotDawnBackend)
    }
}
