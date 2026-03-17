use dawn_rs::*;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

pub struct DawnInstance {
    pub(crate) inner: Instance,
    #[cfg(feature = "wire")]
    pub(crate) wire_handle: Option<Arc<crate::wire_backend::WireBackendHandle>>,
}

impl DawnInstance {
    pub(crate) fn from_instance(instance: Instance) -> Self {
        Self {
            inner: instance,
            #[cfg(feature = "wire")]
            wire_handle: None,
        }
    }
}

impl Clone for DawnInstance {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            #[cfg(feature = "wire")]
            wire_handle: self.wire_handle.clone(),
        }
    }
}

impl fmt::Debug for DawnInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("DawnInstance");
        dbg.field("inner", &self.inner);
        #[cfg(feature = "wire")]
        dbg.field("wire_handle", &self.wire_handle.is_some());
        dbg.finish()
    }
}

#[derive(Debug, Clone)]
pub struct DawnAdapter {
    pub(crate) inner: Adapter,
}

#[derive(Clone)]
pub struct DawnDevice {
    pub(crate) inner: Device,
    pub(crate) device_lost_callback:
        Arc<std::sync::Mutex<Option<wgpu::custom::BoxDeviceLostCallback>>>,
    pub(crate) uncaptured_error_handler:
        Arc<std::sync::Mutex<Option<Arc<dyn wgpu::UncapturedErrorHandler>>>>,
}

impl DawnDevice {
    pub(crate) fn new(device: Device) -> Self {
        Self {
            inner: device,
            device_lost_callback: Arc::new(std::sync::Mutex::new(None)),
            uncaptured_error_handler: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub(crate) fn with_callback_state(
        device: Device,
        device_lost_callback: Arc<std::sync::Mutex<Option<wgpu::custom::BoxDeviceLostCallback>>>,
        uncaptured_error_handler: Arc<std::sync::Mutex<Option<Arc<dyn wgpu::UncapturedErrorHandler>>>>,
    ) -> Self {
        Self {
            inner: device,
            device_lost_callback,
            uncaptured_error_handler,
        }
    }
}

impl fmt::Debug for DawnDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DawnDevice")
            .field("inner", &self.inner)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct DawnQueue {
    pub(crate) inner: Queue,
}

#[derive(Debug, Clone)]
pub struct DawnShaderModule {
    pub(crate) inner: ShaderModule,
}

#[derive(Debug, Clone)]
pub struct DawnBindGroupLayout {
    pub(crate) inner: BindGroupLayout,
}

#[derive(Debug, Clone)]
pub struct DawnBindGroup {
    pub(crate) inner: BindGroup,
}

#[derive(Debug, Clone)]
pub struct DawnTextureView {
    pub(crate) inner: TextureView,
}

#[derive(Debug, Clone)]
pub struct DawnSampler {
    pub(crate) inner: Sampler,
}

#[derive(Debug, Clone)]
pub struct DawnBuffer {
    pub(crate) inner: Buffer,
}

#[derive(Debug, Clone)]
pub struct DawnTexture {
    pub(crate) inner: Texture,
}

#[derive(Debug, Clone)]
pub struct DawnExternalTexture {
    pub(crate) inner: ExternalTexture,
}

#[derive(Debug, Clone)]
pub struct DawnQuerySet {
    pub(crate) inner: QuerySet,
}

#[derive(Debug, Clone)]
pub struct DawnPipelineLayout {
    pub(crate) inner: PipelineLayout,
}

#[derive(Debug, Clone)]
pub struct DawnRenderPipeline {
    pub(crate) inner: RenderPipeline,
}

#[derive(Debug, Clone)]
pub struct DawnComputePipeline {
    pub(crate) inner: ComputePipeline,
}

#[derive(Debug)]
pub struct DawnCommandEncoder {
    pub(crate) inner: CommandEncoder,
}

#[derive(Debug)]
pub struct DawnComputePass {
    pub(crate) inner: ComputePassEncoder,
    pub(crate) ended: bool,
}

#[derive(Debug)]
pub struct DawnRenderPass {
    pub(crate) inner: RenderPassEncoder,
    pub(crate) ended: bool,
}

#[derive(Debug)]
pub struct DawnCommandBuffer {
    pub(crate) inner: CommandBuffer,
}

#[derive(Debug)]
pub struct DawnRenderBundleEncoder {
    pub(crate) inner: RenderBundleEncoder,
}

#[derive(Debug, Clone)]
pub struct DawnRenderBundle {
    pub(crate) inner: RenderBundle,
}

#[cfg(target_os = "macos")]
#[derive(Debug)]
pub struct MetalLayerHandle {
    pub(crate) ptr: *mut std::ffi::c_void,
}

#[cfg(target_os = "macos")]
unsafe impl Send for MetalLayerHandle {}
#[cfg(target_os = "macos")]
unsafe impl Sync for MetalLayerHandle {}

#[cfg(target_os = "macos")]
impl Drop for MetalLayerHandle {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe {
            let ptr = self.ptr.cast::<objc2_quartz_core::CAMetalLayer>();
            let _ = objc2::rc::Retained::from_raw(ptr);
        }
    }
}

#[derive(Debug, Clone)]
pub struct DawnSurface {
    pub(crate) inner: Surface,
    #[cfg(target_os = "macos")]
    pub(crate) metal_layer: Option<Arc<MetalLayerHandle>>,
}

#[derive(Debug, Clone)]
pub struct DawnSurfaceOutputDetail {
    pub(crate) surface: Surface,
}

#[derive(Debug)]
pub struct DawnQueueWriteBuffer {
    pub(crate) inner: Vec<u8>,
}

#[derive(Debug)]
pub struct DawnBufferMappedRange {
    pub(crate) data: *mut u8,
    pub(crate) size: usize,
}

unsafe impl Send for DawnBufferMappedRange {}
unsafe impl Sync for DawnBufferMappedRange {}

#[derive(Debug)]
pub struct DawnPipelineCache;

#[derive(Debug)]
pub struct DawnBlas;

#[derive(Debug)]
pub struct DawnTlas;

macro_rules! impl_deref_to_inner {
    ($name:ident, $target:ty) => {
        impl Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

impl_deref_to_inner!(DawnInstance, Instance);
impl_deref_to_inner!(DawnAdapter, Adapter);
impl_deref_to_inner!(DawnDevice, Device);
impl_deref_to_inner!(DawnQueue, Queue);
impl_deref_to_inner!(DawnShaderModule, ShaderModule);
impl_deref_to_inner!(DawnBindGroupLayout, BindGroupLayout);
impl_deref_to_inner!(DawnBindGroup, BindGroup);
impl_deref_to_inner!(DawnTextureView, TextureView);
impl_deref_to_inner!(DawnSampler, Sampler);
impl_deref_to_inner!(DawnBuffer, Buffer);
impl_deref_to_inner!(DawnTexture, Texture);
impl_deref_to_inner!(DawnExternalTexture, ExternalTexture);
impl_deref_to_inner!(DawnQuerySet, QuerySet);
impl_deref_to_inner!(DawnPipelineLayout, PipelineLayout);
impl_deref_to_inner!(DawnRenderPipeline, RenderPipeline);
impl_deref_to_inner!(DawnComputePipeline, ComputePipeline);
impl_deref_to_inner!(DawnCommandEncoder, CommandEncoder);
impl_deref_to_inner!(DawnComputePass, ComputePassEncoder);
impl_deref_to_inner!(DawnRenderPass, RenderPassEncoder);
impl_deref_to_inner!(DawnCommandBuffer, CommandBuffer);
impl_deref_to_inner!(DawnRenderBundleEncoder, RenderBundleEncoder);
impl_deref_to_inner!(DawnRenderBundle, RenderBundle);
impl_deref_to_inner!(DawnSurface, Surface);
