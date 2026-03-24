use crate::types::*;
use dawn_rs::*;

#[derive(Debug, Clone)]
pub struct DawnIntoWgpu<T>(T);

impl<T> DawnIntoWgpu<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for DawnIntoWgpu<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub struct DawnTextureIntoWgpu<'a> {
    texture: Texture,
    desc: &'a wgpu::TextureDescriptor<'a>,
}

impl<'a> DawnTextureIntoWgpu<'a> {
    pub fn new(texture: Texture, desc: &'a wgpu::TextureDescriptor<'a>) -> Self {
        Self { texture, desc }
    }
}

impl From<DawnIntoWgpu<Device>> for wgpu::Device {
    fn from(value: DawnIntoWgpu<Device>) -> Self {
        wgpu::Device::from_custom(DawnDevice::new(value.0))
    }
}

impl From<DawnIntoWgpu<Queue>> for wgpu::Queue {
    fn from(value: DawnIntoWgpu<Queue>) -> Self {
        wgpu::Queue::from_custom(DawnQueue { inner: value.0 })
    }
}

impl From<DawnTextureIntoWgpu<'_>> for wgpu::Texture {
    fn from(value: DawnTextureIntoWgpu<'_>) -> Self {
        wgpu::Texture::from_custom(
            DawnTexture {
                inner: value.texture,
            },
            value.desc,
        )
    }
}
