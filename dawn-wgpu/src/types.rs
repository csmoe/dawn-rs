use dawn_rs::*;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

type InstanceJob = Box<dyn FnOnce(&mut DawnInstanceState) + Send + 'static>;

#[derive(Debug)]
pub(crate) struct DawnInstanceWorker {
    sender: std::sync::mpsc::Sender<InstanceJob>,
}

#[derive(Debug)]
pub(crate) struct DawnInstanceState {
    pub(crate) instance: Instance,
}

impl DawnInstanceState {
    fn new(instance: Instance) -> Self {
        Self { instance }
    }
}

pub struct DawnInstance {
    pub(crate) inner: Arc<DawnInstanceWorker>,
    #[cfg(feature = "wire")]
    pub(crate) wire_handle: Option<Arc<crate::wire_backend::WireBackendHandle>>,
}

impl DawnInstance {
    pub(crate) fn from_factory(
        factory: impl FnOnce() -> Instance + Send + 'static,
        #[cfg(feature = "wire")] wire_handle: Option<Arc<crate::wire_backend::WireBackendHandle>>,
    ) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<InstanceJob>();
        std::thread::spawn(move || {
            let mut state = DawnInstanceState::new(factory());
            while let Ok(job) = receiver.recv() {
                job(&mut state);
            }
        });
        Self {
            inner: Arc::new(DawnInstanceWorker { sender }),
            #[cfg(feature = "wire")]
            wire_handle,
        }
    }

    pub(crate) fn with_instance<R: Send + 'static>(
        &self,
        f: impl FnOnce(&mut DawnInstanceState) -> R + Send + 'static,
    ) -> R {
        let (sender, receiver) = std::sync::mpsc::channel::<R>();
        self.inner
            .sender
            .send(Box::new(move |state| {
                let result = f(state);
                let _ = sender.send(result);
            }))
            .expect("wgpu-compat: failed to send instance task to worker");
        receiver
            .recv()
            .expect("wgpu-compat: failed to receive instance task result")
    }
}

unsafe impl Send for DawnAdapterShared {}
unsafe impl Sync for DawnAdapterShared {}

impl Clone for DawnInstance {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            #[cfg(feature = "wire")]
            wire_handle: self.wire_handle.clone(),
        }
    }
}

impl fmt::Debug for DawnInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("DawnInstance");
        dbg.field("inner", &"worker-thread");
        #[cfg(feature = "wire")]
        dbg.field("wire_handle", &self.wire_handle.is_some());
        dbg.finish()
    }
}

#[derive(Clone)]
pub struct DawnAdapter {
    pub(crate) shared: Arc<DawnAdapterShared>,
}

impl DawnAdapter {
    pub(crate) fn from_adapter(worker: Arc<DawnInstanceWorker>, adapter: Adapter) -> Self {
        let raw = Box::into_raw(Box::new(adapter));
        Self {
            shared: Arc::new(DawnAdapterShared {
                worker,
                raw,
            }),
        }
    }

    pub(crate) fn with_adapter<R: Send + 'static>(
        &self,
        f: impl FnOnce(&Adapter) -> R + Send + 'static,
    ) -> R {
        let raw = self.shared.raw as usize;
        let (sender, receiver) = std::sync::mpsc::channel::<R>();
        self.shared
            .worker
            .sender
            .send(Box::new(move |_state| {
                // SAFETY: raw points to a valid boxed Adapter for the lifetime of shared.
                let adapter = unsafe { &*(raw as *mut Adapter) };
                let result = f(adapter);
                let _ = sender.send(result);
            }))
            .expect("wgpu-compat: failed to send adapter task to worker");
        receiver
            .recv()
            .expect("wgpu-compat: failed to receive adapter task result")
    }
}

#[derive(Debug)]
pub(crate) struct DawnAdapterShared {
    worker: Arc<DawnInstanceWorker>,
    raw: *mut Adapter,
}

impl Drop for DawnAdapterShared {
    fn drop(&mut self) {
        let raw = self.raw as usize;
        let _ = self.worker.sender.send(Box::new(move |_state| {
            // SAFETY: raw originated from Box::into_raw and is dropped exactly once here.
            unsafe { drop(Box::from_raw(raw as *mut Adapter)) };
        }));
    }
}

impl fmt::Debug for DawnAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DawnAdapter")
            .field("raw", &self.shared.raw)
            .finish()
    }
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
