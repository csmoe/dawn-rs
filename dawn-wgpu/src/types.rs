use dawn_rs::*;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct SendSync<T: Send>(Arc<Mutex<T>>);

impl<T: Send> SendSync<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }

    pub(crate) fn get(&self) -> T
    where
        T: Clone,
    {
        self.0.lock().expect("send sync wrapper poisoned").clone()
    }

    pub(crate) fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.0.lock().expect("send sync wrapper poisoned");
        f(&*guard)
    }

    pub(crate) fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.0.lock().expect("send sync wrapper poisoned");
        f(&mut *guard)
    }
}

impl<T: fmt::Debug + Send> fmt::Debug for SendSync<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.0.lock().map_err(|_| fmt::Error)?;
        f.debug_tuple("SendSync").field(&*guard).finish()
    }
}

pub(crate) struct DawnInstance {
    pub(crate) inner: SendSync<Instance>,
    #[cfg(feature = "wire")]
    pub(crate) wire_handle: Option<Arc<crate::wire_backend::WireBackendHandle>>,
}

impl DawnInstance {
    pub(crate) fn from_instance(instance: Instance) -> Self {
        Self {
            inner: SendSync::new(instance),
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
pub(crate) struct DawnAdapter {
    pub(crate) inner: SendSync<Adapter>,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnDevice {
    pub(crate) inner: Device,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnQueue {
    pub(crate) inner: Queue,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnShaderModule {
    pub(crate) inner: ShaderModule,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnBindGroupLayout {
    pub(crate) inner: BindGroupLayout,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnBindGroup {
    pub(crate) inner: BindGroup,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnTextureView {
    pub(crate) inner: TextureView,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnSampler {
    pub(crate) inner: Sampler,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnBuffer {
    pub(crate) inner: Buffer,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnTexture {
    pub(crate) inner: Texture,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnExternalTexture {
    pub(crate) inner: ExternalTexture,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnQuerySet {
    pub(crate) inner: QuerySet,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnPipelineLayout {
    pub(crate) inner: PipelineLayout,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnRenderPipeline {
    pub(crate) inner: RenderPipeline,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnComputePipeline {
    pub(crate) inner: ComputePipeline,
}

#[derive(Debug)]
pub(crate) struct DawnCommandEncoder {
    pub(crate) inner: SendSync<CommandEncoder>,
}

#[derive(Debug)]
pub(crate) struct DawnComputePass {
    pub(crate) inner: SendSync<ComputePassEncoder>,
    pub(crate) ended: bool,
}

#[derive(Debug)]
pub(crate) struct DawnRenderPass {
    pub(crate) inner: SendSync<RenderPassEncoder>,
    pub(crate) ended: bool,
}

#[derive(Debug)]
pub(crate) struct DawnCommandBuffer {
    pub(crate) inner: CommandBuffer,
}

#[derive(Debug)]
pub(crate) struct DawnRenderBundleEncoder {
    pub(crate) inner: SendSync<RenderBundleEncoder>,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnRenderBundle {
    pub(crate) inner: RenderBundle,
}

#[cfg(target_os = "macos")]
#[derive(Debug)]
pub(crate) struct MetalLayerHandle {
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
pub(crate) struct DawnSurface {
    pub(crate) inner: SendSync<Surface>,
    #[cfg(target_os = "macos")]
    pub(crate) metal_layer: Option<Arc<MetalLayerHandle>>,
}

#[derive(Debug, Clone)]
pub(crate) struct DawnSurfaceOutputDetail {
    pub(crate) surface: SendSync<Surface>,
}

#[derive(Debug)]
pub(crate) struct DawnQueueWriteBuffer {
    pub(crate) inner: Vec<u8>,
}

#[derive(Debug)]
pub(crate) struct DawnBufferMappedRange {
    pub(crate) data: *mut u8,
    pub(crate) size: usize,
}

unsafe impl Send for DawnBufferMappedRange {}
unsafe impl Sync for DawnBufferMappedRange {}

#[derive(Debug)]
pub(crate) struct DawnPipelineCache;

#[derive(Debug)]
pub(crate) struct DawnBlas;

#[derive(Debug)]
pub(crate) struct DawnTlas;
