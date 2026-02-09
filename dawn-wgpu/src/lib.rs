#![allow(unexpected_cfgs, dead_code, unreachable_patterns)]

use dawn_rs::*;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use wgpu::custom::*;

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
struct DawnInstance {
    inner: Instance,
}

#[derive(Debug, Clone)]
struct DawnAdapter {
    inner: Adapter,
}

#[derive(Debug, Clone)]
struct DawnDevice {
    inner: Device,
}

#[derive(Debug, Clone)]
struct DawnQueue {
    inner: Queue,
}

#[derive(Debug, Clone)]
struct DawnShaderModule {
    inner: ShaderModule,
}

#[derive(Debug, Clone)]
struct DawnBindGroupLayout {
    inner: BindGroupLayout,
}

#[derive(Debug, Clone)]
struct DawnBindGroup {
    inner: BindGroup,
}

#[derive(Debug, Clone)]
struct DawnTextureView {
    inner: TextureView,
}

#[derive(Debug, Clone)]
struct DawnSampler {
    inner: Sampler,
}

#[derive(Debug, Clone)]
struct DawnBuffer {
    inner: Buffer,
}

#[derive(Debug, Clone)]
struct DawnTexture {
    inner: Texture,
}

#[derive(Debug, Clone)]
struct DawnExternalTexture {
    inner: ExternalTexture,
}

#[derive(Debug, Clone)]
struct DawnQuerySet {
    inner: QuerySet,
}

#[derive(Debug, Clone)]
struct DawnPipelineLayout {
    inner: PipelineLayout,
}

#[derive(Debug, Clone)]
struct DawnRenderPipeline {
    inner: RenderPipeline,
}

#[derive(Debug, Clone)]
struct DawnComputePipeline {
    inner: ComputePipeline,
}

#[derive(Debug)]
struct DawnCommandEncoder {
    inner: CommandEncoder,
}

#[derive(Debug)]
struct DawnComputePass {
    inner: ComputePassEncoder,
    ended: bool,
}

#[derive(Debug)]
struct DawnRenderPass {
    inner: RenderPassEncoder,
    ended: bool,
}

#[derive(Debug)]
struct DawnCommandBuffer {
    inner: CommandBuffer,
}

#[derive(Debug)]
struct DawnRenderBundleEncoder {
    inner: RenderBundleEncoder,
}

#[derive(Debug, Clone)]
struct DawnRenderBundle {
    inner: RenderBundle,
}

#[cfg(target_os = "macos")]
#[derive(Debug)]
struct MetalLayerHandle {
    ptr: *mut std::ffi::c_void,
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
struct DawnSurface {
    inner: Surface,
    #[cfg(target_os = "macos")]
    metal_layer: Option<Arc<MetalLayerHandle>>,
}

#[derive(Debug, Clone)]
struct DawnSurfaceOutputDetail {
    surface: Surface,
}

#[derive(Debug)]
struct DawnQueueWriteBuffer {
    inner: Vec<u8>,
}

#[derive(Debug)]
struct DawnBufferMappedRange {
    data: *mut u8,
    size: usize,
}

#[derive(Debug)]
struct DawnPipelineCache;

#[derive(Debug)]
struct DawnBlas;

#[derive(Debug)]
struct DawnTlas;

#[derive(Debug)]
struct DawnError(String);

impl fmt::Display for DawnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DawnError {}

macro_rules! unsafe_send_sync {
    ($($ty:ty),+ $(,)?) => {
        $(unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {})+
    };
}

unsafe_send_sync!(
    DawnInstance,
    DawnAdapter,
    DawnDevice,
    DawnQueue,
    DawnShaderModule,
    DawnBindGroupLayout,
    DawnBindGroup,
    DawnTextureView,
    DawnSampler,
    DawnBuffer,
    DawnTexture,
    DawnExternalTexture,
    DawnQuerySet,
    DawnPipelineLayout,
    DawnRenderPipeline,
    DawnComputePipeline,
    DawnCommandEncoder,
    DawnComputePass,
    DawnRenderPass,
    DawnCommandBuffer,
    DawnRenderBundleEncoder,
    DawnRenderBundle,
    DawnSurface,
    DawnSurfaceOutputDetail,
    DawnQueueWriteBuffer,
    DawnBufferMappedRange,
    DawnPipelineCache,
    DawnBlas,
    DawnTlas,
    DawnError,
);

struct CallbackFuture<T> {
    shared: Arc<Mutex<CallbackShared<T>>>,
}

struct CallbackShared<T> {
    result: Option<T>,
    waker: Option<Waker>,
}

impl<T> CallbackFuture<T> {
    fn new() -> (Self, Arc<Mutex<CallbackShared<T>>>) {
        let shared = Arc::new(Mutex::new(CallbackShared {
            result: None,
            waker: None,
        }));
        (
            Self {
                shared: shared.clone(),
            },
            shared,
        )
    }
}

impl<T> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared = self.shared.lock().expect("wgpu-compat future lock");
        if let Some(result) = shared.result.take() {
            return Poll::Ready(result);
        }
        shared.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

fn complete_shared<T>(shared: &Arc<Mutex<CallbackShared<T>>>, value: T) {
    let mut shared = shared.lock().expect("wgpu-compat future lock");
    shared.result = Some(value);
    if let Some(waker) = shared.waker.take() {
        waker.wake();
    }
}

fn label_to_string(label: wgpu::Label<'_>) -> Option<String> {
    label.map(|s| s.to_string())
}

fn map_backend_type_to_wgpu(value: BackendType) -> wgpu::Backend {
    match value {
        BackendType::Vulkan => wgpu::Backend::Vulkan,
        BackendType::Metal => wgpu::Backend::Metal,
        BackendType::D3D12 => wgpu::Backend::Dx12,
        BackendType::D3D11 => wgpu::Backend::Noop,
        BackendType::OpenGL | BackendType::OpenGLes => wgpu::Backend::Gl,
        BackendType::WebGPU => wgpu::Backend::BrowserWebGpu,
        _ => wgpu::Backend::Noop,
    }
}

fn map_power_preference(value: wgpu::PowerPreference) -> PowerPreference {
    match value {
        wgpu::PowerPreference::None => PowerPreference::Undefined,
        wgpu::PowerPreference::LowPower => PowerPreference::LowPower,
        wgpu::PowerPreference::HighPerformance => PowerPreference::HighPerformance,
        _ => PowerPreference::Undefined,
    }
}

fn map_texture_format(value: wgpu::TextureFormat) -> TextureFormat {
    use wgpu::TextureFormat as W;
    match value {
        W::R8Unorm => TextureFormat::R8Unorm,
        W::R8Snorm => TextureFormat::R8Snorm,
        W::R8Uint => TextureFormat::R8Uint,
        W::R8Sint => TextureFormat::R8Sint,
        W::R16Unorm => TextureFormat::R16Unorm,
        W::R16Snorm => TextureFormat::R16Snorm,
        W::R16Uint => TextureFormat::R16Uint,
        W::R16Sint => TextureFormat::R16Sint,
        W::R16Float => TextureFormat::R16Float,
        W::Rg8Unorm => TextureFormat::Rg8Unorm,
        W::Rg8Snorm => TextureFormat::Rg8Snorm,
        W::Rg8Uint => TextureFormat::Rg8Uint,
        W::Rg8Sint => TextureFormat::Rg8Sint,
        W::R32Float => TextureFormat::R32Float,
        W::R32Uint => TextureFormat::R32Uint,
        W::R32Sint => TextureFormat::R32Sint,
        W::Rg16Unorm => TextureFormat::Rg16Unorm,
        W::Rg16Snorm => TextureFormat::Rg16Snorm,
        W::Rg16Uint => TextureFormat::Rg16Uint,
        W::Rg16Sint => TextureFormat::Rg16Sint,
        W::Rg16Float => TextureFormat::Rg16Float,
        W::Rgba8Unorm => TextureFormat::Rgba8Unorm,
        W::Rgba8UnormSrgb => TextureFormat::Rgba8UnormSrgb,
        W::Rgba8Snorm => TextureFormat::Rgba8Snorm,
        W::Rgba8Uint => TextureFormat::Rgba8Uint,
        W::Rgba8Sint => TextureFormat::Rgba8Sint,
        W::Bgra8Unorm => TextureFormat::Bgra8Unorm,
        W::Bgra8UnormSrgb => TextureFormat::Bgra8UnormSrgb,
        W::Rgb10a2Unorm => TextureFormat::Rgb10A2Unorm,
        W::Rgb10a2Uint => TextureFormat::Rgb10A2Uint,
        W::Rg11b10Ufloat => TextureFormat::Rg11B10Ufloat,
        W::Rgb9e5Ufloat => TextureFormat::Rgb9E5Ufloat,
        W::Rg32Float => TextureFormat::Rg32Float,
        W::Rg32Uint => TextureFormat::Rg32Uint,
        W::Rg32Sint => TextureFormat::Rg32Sint,
        W::Rgba16Unorm => TextureFormat::Rgba16Unorm,
        W::Rgba16Snorm => TextureFormat::Rgba16Snorm,
        W::Rgba16Uint => TextureFormat::Rgba16Uint,
        W::Rgba16Sint => TextureFormat::Rgba16Sint,
        W::Rgba16Float => TextureFormat::Rgba16Float,
        W::Rgba32Float => TextureFormat::Rgba32Float,
        W::Rgba32Uint => TextureFormat::Rgba32Uint,
        W::Rgba32Sint => TextureFormat::Rgba32Sint,
        W::Stencil8 => TextureFormat::Stencil8,
        W::Depth16Unorm => TextureFormat::Depth16Unorm,
        W::Depth24Plus => TextureFormat::Depth24Plus,
        W::Depth24PlusStencil8 => TextureFormat::Depth24PlusStencil8,
        W::Depth32Float => TextureFormat::Depth32Float,
        W::Depth32FloatStencil8 => TextureFormat::Depth32FloatStencil8,
        W::Bc1RgbaUnorm => TextureFormat::Bc1RgbaUnorm,
        W::Bc1RgbaUnormSrgb => TextureFormat::Bc1RgbaUnormSrgb,
        W::Bc2RgbaUnorm => TextureFormat::Bc2RgbaUnorm,
        W::Bc2RgbaUnormSrgb => TextureFormat::Bc2RgbaUnormSrgb,
        W::Bc3RgbaUnorm => TextureFormat::Bc3RgbaUnorm,
        W::Bc3RgbaUnormSrgb => TextureFormat::Bc3RgbaUnormSrgb,
        W::Bc4RUnorm => TextureFormat::Bc4RUnorm,
        W::Bc4RSnorm => TextureFormat::Bc4RSnorm,
        W::Bc5RgUnorm => TextureFormat::Bc5RgUnorm,
        W::Bc5RgSnorm => TextureFormat::Bc5RgSnorm,
        W::Bc6hRgbUfloat => TextureFormat::Bc6HRgbUfloat,
        W::Bc6hRgbFloat => TextureFormat::Bc6HRgbFloat,
        W::Bc7RgbaUnorm => TextureFormat::Bc7RgbaUnorm,
        W::Bc7RgbaUnormSrgb => TextureFormat::Bc7RgbaUnormSrgb,
        W::Etc2Rgb8Unorm => TextureFormat::Etc2Rgb8Unorm,
        W::Etc2Rgb8UnormSrgb => TextureFormat::Etc2Rgb8UnormSrgb,
        W::Etc2Rgb8A1Unorm => TextureFormat::Etc2Rgb8A1Unorm,
        W::Etc2Rgb8A1UnormSrgb => TextureFormat::Etc2Rgb8A1UnormSrgb,
        W::Etc2Rgba8Unorm => TextureFormat::Etc2Rgba8Unorm,
        W::Etc2Rgba8UnormSrgb => TextureFormat::Etc2Rgba8UnormSrgb,
        W::EacR11Unorm => TextureFormat::EacR11Unorm,
        W::EacR11Snorm => TextureFormat::EacR11Snorm,
        W::EacRg11Unorm => TextureFormat::EacRg11Unorm,
        W::EacRg11Snorm => TextureFormat::EacRg11Snorm,
        W::Astc { block, channel } => match (block, channel) {
            (wgpu::AstcBlock::B4x4, wgpu::AstcChannel::Unorm) => TextureFormat::Astc4X4Unorm,
            (wgpu::AstcBlock::B4x4, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc4X4UnormSrgb
            }
            (wgpu::AstcBlock::B5x4, wgpu::AstcChannel::Unorm) => TextureFormat::Astc5X4Unorm,
            (wgpu::AstcBlock::B5x4, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc5X4UnormSrgb
            }
            (wgpu::AstcBlock::B5x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc5X5Unorm,
            (wgpu::AstcBlock::B5x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc5X5UnormSrgb
            }
            (wgpu::AstcBlock::B6x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc6X5Unorm,
            (wgpu::AstcBlock::B6x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc6X5UnormSrgb
            }
            (wgpu::AstcBlock::B6x6, wgpu::AstcChannel::Unorm) => TextureFormat::Astc6X6Unorm,
            (wgpu::AstcBlock::B6x6, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc6X6UnormSrgb
            }
            (wgpu::AstcBlock::B8x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc8X5Unorm,
            (wgpu::AstcBlock::B8x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc8X5UnormSrgb
            }
            (wgpu::AstcBlock::B8x6, wgpu::AstcChannel::Unorm) => TextureFormat::Astc8X6Unorm,
            (wgpu::AstcBlock::B8x6, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc8X6UnormSrgb
            }
            (wgpu::AstcBlock::B8x8, wgpu::AstcChannel::Unorm) => TextureFormat::Astc8X8Unorm,
            (wgpu::AstcBlock::B8x8, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc8X8UnormSrgb
            }
            (wgpu::AstcBlock::B10x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X5Unorm,
            (wgpu::AstcBlock::B10x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X5UnormSrgb
            }
            (wgpu::AstcBlock::B10x6, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X6Unorm,
            (wgpu::AstcBlock::B10x6, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X6UnormSrgb
            }
            (wgpu::AstcBlock::B10x8, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X8Unorm,
            (wgpu::AstcBlock::B10x8, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X8UnormSrgb
            }
            (wgpu::AstcBlock::B10x10, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X10Unorm,
            (wgpu::AstcBlock::B10x10, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X10UnormSrgb
            }
            (wgpu::AstcBlock::B12x10, wgpu::AstcChannel::Unorm) => TextureFormat::Astc12X10Unorm,
            (wgpu::AstcBlock::B12x10, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc12X10UnormSrgb
            }
            (wgpu::AstcBlock::B12x12, wgpu::AstcChannel::Unorm) => TextureFormat::Astc12X12Unorm,
            (wgpu::AstcBlock::B12x12, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc12X12UnormSrgb
            }
            _ => panic!("wgpu-compat: unsupported ASTC format"),
        },
        _ => panic!("wgpu-compat: unsupported texture format"),
    }
}

fn map_texture_format_to_wgpu(value: TextureFormat) -> wgpu::TextureFormat {
    use wgpu::TextureFormat as W;
    match value {
        TextureFormat::R8Unorm => W::R8Unorm,
        TextureFormat::R8Snorm => W::R8Snorm,
        TextureFormat::R8Uint => W::R8Uint,
        TextureFormat::R8Sint => W::R8Sint,
        TextureFormat::R16Unorm => W::R16Unorm,
        TextureFormat::R16Snorm => W::R16Snorm,
        TextureFormat::R16Uint => W::R16Uint,
        TextureFormat::R16Sint => W::R16Sint,
        TextureFormat::R16Float => W::R16Float,
        TextureFormat::Rg8Unorm => W::Rg8Unorm,
        TextureFormat::Rg8Snorm => W::Rg8Snorm,
        TextureFormat::Rg8Uint => W::Rg8Uint,
        TextureFormat::Rg8Sint => W::Rg8Sint,
        TextureFormat::R32Float => W::R32Float,
        TextureFormat::R32Uint => W::R32Uint,
        TextureFormat::R32Sint => W::R32Sint,
        TextureFormat::Rg16Unorm => W::Rg16Unorm,
        TextureFormat::Rg16Snorm => W::Rg16Snorm,
        TextureFormat::Rg16Uint => W::Rg16Uint,
        TextureFormat::Rg16Sint => W::Rg16Sint,
        TextureFormat::Rg16Float => W::Rg16Float,
        TextureFormat::Rgba8Unorm => W::Rgba8Unorm,
        TextureFormat::Rgba8UnormSrgb => W::Rgba8UnormSrgb,
        TextureFormat::Rgba8Snorm => W::Rgba8Snorm,
        TextureFormat::Rgba8Uint => W::Rgba8Uint,
        TextureFormat::Rgba8Sint => W::Rgba8Sint,
        TextureFormat::Bgra8Unorm => W::Bgra8Unorm,
        TextureFormat::Bgra8UnormSrgb => W::Bgra8UnormSrgb,
        TextureFormat::Rgb10A2Uint => W::Rgb10a2Uint,
        TextureFormat::Rgb10A2Unorm => W::Rgb10a2Unorm,
        TextureFormat::Rg11B10Ufloat => W::Rg11b10Ufloat,
        TextureFormat::Rgb9E5Ufloat => W::Rgb9e5Ufloat,
        TextureFormat::Rg32Float => W::Rg32Float,
        TextureFormat::Rg32Uint => W::Rg32Uint,
        TextureFormat::Rg32Sint => W::Rg32Sint,
        TextureFormat::Rgba16Unorm => W::Rgba16Unorm,
        TextureFormat::Rgba16Snorm => W::Rgba16Snorm,
        TextureFormat::Rgba16Uint => W::Rgba16Uint,
        TextureFormat::Rgba16Sint => W::Rgba16Sint,
        TextureFormat::Rgba16Float => W::Rgba16Float,
        TextureFormat::Rgba32Float => W::Rgba32Float,
        TextureFormat::Rgba32Uint => W::Rgba32Uint,
        TextureFormat::Rgba32Sint => W::Rgba32Sint,
        TextureFormat::Stencil8 => W::Stencil8,
        TextureFormat::Depth16Unorm => W::Depth16Unorm,
        TextureFormat::Depth24Plus => W::Depth24Plus,
        TextureFormat::Depth24PlusStencil8 => W::Depth24PlusStencil8,
        TextureFormat::Depth32Float => W::Depth32Float,
        TextureFormat::Depth32FloatStencil8 => W::Depth32FloatStencil8,
        TextureFormat::Bc1RgbaUnorm => W::Bc1RgbaUnorm,
        TextureFormat::Bc1RgbaUnormSrgb => W::Bc1RgbaUnormSrgb,
        TextureFormat::Bc2RgbaUnorm => W::Bc2RgbaUnorm,
        TextureFormat::Bc2RgbaUnormSrgb => W::Bc2RgbaUnormSrgb,
        TextureFormat::Bc3RgbaUnorm => W::Bc3RgbaUnorm,
        TextureFormat::Bc3RgbaUnormSrgb => W::Bc3RgbaUnormSrgb,
        TextureFormat::Bc4RUnorm => W::Bc4RUnorm,
        TextureFormat::Bc4RSnorm => W::Bc4RSnorm,
        TextureFormat::Bc5RgUnorm => W::Bc5RgUnorm,
        TextureFormat::Bc5RgSnorm => W::Bc5RgSnorm,
        TextureFormat::Bc6HRgbUfloat => W::Bc6hRgbUfloat,
        TextureFormat::Bc6HRgbFloat => W::Bc6hRgbFloat,
        TextureFormat::Bc7RgbaUnorm => W::Bc7RgbaUnorm,
        TextureFormat::Bc7RgbaUnormSrgb => W::Bc7RgbaUnormSrgb,
        TextureFormat::Etc2Rgb8Unorm => W::Etc2Rgb8Unorm,
        TextureFormat::Etc2Rgb8UnormSrgb => W::Etc2Rgb8UnormSrgb,
        TextureFormat::Etc2Rgb8A1Unorm => W::Etc2Rgb8A1Unorm,
        TextureFormat::Etc2Rgb8A1UnormSrgb => W::Etc2Rgb8A1UnormSrgb,
        TextureFormat::Etc2Rgba8Unorm => W::Etc2Rgba8Unorm,
        TextureFormat::Etc2Rgba8UnormSrgb => W::Etc2Rgba8UnormSrgb,
        TextureFormat::EacR11Unorm => W::EacR11Unorm,
        TextureFormat::EacR11Snorm => W::EacR11Snorm,
        TextureFormat::EacRg11Unorm => W::EacRg11Unorm,
        TextureFormat::EacRg11Snorm => W::EacRg11Snorm,
        TextureFormat::Astc4X4Unorm => W::Astc {
            block: wgpu::AstcBlock::B4x4,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc4X4UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B4x4,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc5X4Unorm => W::Astc {
            block: wgpu::AstcBlock::B5x4,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc5X4UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B5x4,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc5X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B5x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc5X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B5x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc6X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B6x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc6X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B6x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc6X6Unorm => W::Astc {
            block: wgpu::AstcBlock::B6x6,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc6X6UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B6x6,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc8X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B8x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc8X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B8x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc8X6Unorm => W::Astc {
            block: wgpu::AstcBlock::B8x6,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc8X6UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B8x6,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc8X8Unorm => W::Astc {
            block: wgpu::AstcBlock::B8x8,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc8X8UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B8x8,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X6Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x6,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X6UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x6,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X8Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x8,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X8UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x8,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X10Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x10,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X10UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x10,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc12X10Unorm => W::Astc {
            block: wgpu::AstcBlock::B12x10,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc12X10UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B12x10,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc12X12Unorm => W::Astc {
            block: wgpu::AstcBlock::B12x12,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc12X12UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B12x12,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        _ => panic!("wgpu-compat: unsupported texture format conversion"),
    }
}

fn map_texture_dimension(value: wgpu::TextureDimension) -> TextureDimension {
    match value {
        wgpu::TextureDimension::D1 => TextureDimension::D1,
        wgpu::TextureDimension::D2 => TextureDimension::D2,
        wgpu::TextureDimension::D3 => TextureDimension::D3,
        _ => TextureDimension::D2,
    }
}

fn map_texture_view_dimension(value: wgpu::TextureViewDimension) -> TextureViewDimension {
    match value {
        wgpu::TextureViewDimension::D1 => TextureViewDimension::D1,
        wgpu::TextureViewDimension::D2 => TextureViewDimension::D2,
        wgpu::TextureViewDimension::D2Array => TextureViewDimension::D2Array,
        wgpu::TextureViewDimension::Cube => TextureViewDimension::Cube,
        wgpu::TextureViewDimension::CubeArray => TextureViewDimension::CubeArray,
        wgpu::TextureViewDimension::D3 => TextureViewDimension::D3,
        _ => TextureViewDimension::D2,
    }
}

fn map_texture_aspect(value: wgpu::TextureAspect) -> TextureAspect {
    match value {
        wgpu::TextureAspect::All => TextureAspect::All,
        wgpu::TextureAspect::StencilOnly => TextureAspect::StencilOnly,
        wgpu::TextureAspect::DepthOnly => TextureAspect::DepthOnly,
        wgpu::TextureAspect::Plane0 => TextureAspect::Plane0Only,
        wgpu::TextureAspect::Plane1 => TextureAspect::Plane1Only,
        wgpu::TextureAspect::Plane2 => TextureAspect::Plane2Only,
        _ => TextureAspect::All,
    }
}

fn map_filter_mode(value: wgpu::FilterMode) -> FilterMode {
    match value {
        wgpu::FilterMode::Nearest => FilterMode::Nearest,
        wgpu::FilterMode::Linear => FilterMode::Linear,
        _ => FilterMode::Nearest,
    }
}

fn map_mipmap_filter_mode(value: wgpu::MipmapFilterMode) -> MipmapFilterMode {
    match value {
        wgpu::MipmapFilterMode::Nearest => MipmapFilterMode::Nearest,
        wgpu::MipmapFilterMode::Linear => MipmapFilterMode::Linear,
        _ => MipmapFilterMode::Nearest,
    }
}

fn map_address_mode(value: wgpu::AddressMode) -> AddressMode {
    match value {
        wgpu::AddressMode::ClampToEdge => AddressMode::ClampToEdge,
        wgpu::AddressMode::Repeat => AddressMode::Repeat,
        wgpu::AddressMode::MirrorRepeat => AddressMode::MirrorRepeat,
        _ => AddressMode::ClampToEdge,
    }
}

fn map_compare_function(value: wgpu::CompareFunction) -> CompareFunction {
    match value {
        wgpu::CompareFunction::Never => CompareFunction::Never,
        wgpu::CompareFunction::Less => CompareFunction::Less,
        wgpu::CompareFunction::Equal => CompareFunction::Equal,
        wgpu::CompareFunction::LessEqual => CompareFunction::LessEqual,
        wgpu::CompareFunction::Greater => CompareFunction::Greater,
        wgpu::CompareFunction::NotEqual => CompareFunction::NotEqual,
        wgpu::CompareFunction::GreaterEqual => CompareFunction::GreaterEqual,
        wgpu::CompareFunction::Always => CompareFunction::Always,
        _ => CompareFunction::Always,
    }
}

fn map_index_format(value: wgpu::IndexFormat) -> IndexFormat {
    match value {
        wgpu::IndexFormat::Uint16 => IndexFormat::Uint16,
        wgpu::IndexFormat::Uint32 => IndexFormat::Uint32,
        _ => IndexFormat::Uint32,
    }
}

fn map_primitive_topology(value: wgpu::PrimitiveTopology) -> PrimitiveTopology {
    match value {
        wgpu::PrimitiveTopology::PointList => PrimitiveTopology::PointList,
        wgpu::PrimitiveTopology::LineList => PrimitiveTopology::LineList,
        wgpu::PrimitiveTopology::LineStrip => PrimitiveTopology::LineStrip,
        wgpu::PrimitiveTopology::TriangleList => PrimitiveTopology::TriangleList,
        wgpu::PrimitiveTopology::TriangleStrip => PrimitiveTopology::TriangleStrip,
        _ => PrimitiveTopology::TriangleList,
    }
}

fn map_front_face(value: wgpu::FrontFace) -> FrontFace {
    match value {
        wgpu::FrontFace::Ccw => FrontFace::Ccw,
        wgpu::FrontFace::Cw => FrontFace::Cw,
        _ => FrontFace::Ccw,
    }
}

fn map_cull_mode(value: Option<wgpu::Face>) -> CullMode {
    match value {
        Some(wgpu::Face::Front) => CullMode::Front,
        Some(wgpu::Face::Back) => CullMode::Back,
        None => CullMode::None,
    }
}

fn map_stencil_operation(value: wgpu::StencilOperation) -> StencilOperation {
    match value {
        wgpu::StencilOperation::Keep => StencilOperation::Keep,
        wgpu::StencilOperation::Zero => StencilOperation::Zero,
        wgpu::StencilOperation::Replace => StencilOperation::Replace,
        wgpu::StencilOperation::Invert => StencilOperation::Invert,
        wgpu::StencilOperation::IncrementClamp => StencilOperation::IncrementClamp,
        wgpu::StencilOperation::DecrementClamp => StencilOperation::DecrementClamp,
        wgpu::StencilOperation::IncrementWrap => StencilOperation::IncrementWrap,
        wgpu::StencilOperation::DecrementWrap => StencilOperation::DecrementWrap,
        _ => StencilOperation::Keep,
    }
}

fn map_blend_operation(value: wgpu::BlendOperation) -> BlendOperation {
    match value {
        wgpu::BlendOperation::Add => BlendOperation::Add,
        wgpu::BlendOperation::Subtract => BlendOperation::Subtract,
        wgpu::BlendOperation::ReverseSubtract => BlendOperation::ReverseSubtract,
        wgpu::BlendOperation::Min => BlendOperation::Min,
        wgpu::BlendOperation::Max => BlendOperation::Max,
        _ => BlendOperation::Add,
    }
}

fn map_blend_factor(value: wgpu::BlendFactor) -> BlendFactor {
    match value {
        wgpu::BlendFactor::Zero => BlendFactor::Zero,
        wgpu::BlendFactor::One => BlendFactor::One,
        wgpu::BlendFactor::Src => BlendFactor::Src,
        wgpu::BlendFactor::OneMinusSrc => BlendFactor::OneMinusSrc,
        wgpu::BlendFactor::SrcAlpha => BlendFactor::SrcAlpha,
        wgpu::BlendFactor::OneMinusSrcAlpha => BlendFactor::OneMinusSrcAlpha,
        wgpu::BlendFactor::Dst => BlendFactor::Dst,
        wgpu::BlendFactor::OneMinusDst => BlendFactor::OneMinusDst,
        wgpu::BlendFactor::DstAlpha => BlendFactor::DstAlpha,
        wgpu::BlendFactor::OneMinusDstAlpha => BlendFactor::OneMinusDstAlpha,
        wgpu::BlendFactor::SrcAlphaSaturated => BlendFactor::SrcAlphaSaturated,
        wgpu::BlendFactor::Constant => BlendFactor::Constant,
        wgpu::BlendFactor::OneMinusConstant => BlendFactor::OneMinusConstant,
        _ => BlendFactor::One,
    }
}

fn map_color_write_mask(value: wgpu::ColorWrites) -> ColorWriteMask {
    ColorWriteMask::from_bits_truncate(value.bits() as u64)
}

fn map_buffer_usages(value: wgpu::BufferUsages) -> BufferUsage {
    BufferUsage::from_bits_truncate(value.bits() as u64)
}

fn map_texture_usages(value: wgpu::TextureUsages) -> TextureUsage {
    TextureUsage::from_bits_truncate(value.bits() as u64)
}

fn map_texture_usage_to_wgpu(value: TextureUsage) -> wgpu::TextureUsages {
    let mut out = wgpu::TextureUsages::empty();
    if value.contains(TextureUsage::COPY_SRC) {
        out |= wgpu::TextureUsages::COPY_SRC;
    }
    if value.contains(TextureUsage::COPY_DST) {
        out |= wgpu::TextureUsages::COPY_DST;
    }
    if value.contains(TextureUsage::TEXTURE_BINDING) {
        out |= wgpu::TextureUsages::TEXTURE_BINDING;
    }
    if value.contains(TextureUsage::STORAGE_BINDING) {
        out |= wgpu::TextureUsages::STORAGE_BINDING;
    }
    if value.contains(TextureUsage::RENDER_ATTACHMENT) {
        out |= wgpu::TextureUsages::RENDER_ATTACHMENT;
    }
    if value.contains(TextureUsage::TRANSIENT_ATTACHMENT) {
        out |= wgpu::TextureUsages::TRANSIENT;
    }
    if value.contains(TextureUsage::STORAGE_ATTACHMENT) {
        out |= wgpu::TextureUsages::STORAGE_BINDING;
    }
    out
}

fn map_shader_stages(value: wgpu::ShaderStages) -> ShaderStage {
    ShaderStage::from_bits_truncate(value.bits() as u64)
}

fn map_sampler_binding_type(value: wgpu::SamplerBindingType) -> SamplerBindingType {
    match value {
        wgpu::SamplerBindingType::Filtering => SamplerBindingType::Filtering,
        wgpu::SamplerBindingType::NonFiltering => SamplerBindingType::NonFiltering,
        wgpu::SamplerBindingType::Comparison => SamplerBindingType::Comparison,
        _ => SamplerBindingType::Filtering,
    }
}

fn map_texture_sample_type(value: wgpu::TextureSampleType) -> TextureSampleType {
    match value {
        wgpu::TextureSampleType::Float { filterable } => {
            if filterable {
                TextureSampleType::Float
            } else {
                TextureSampleType::UnfilterableFloat
            }
        }
        wgpu::TextureSampleType::Depth => TextureSampleType::Depth,
        wgpu::TextureSampleType::Sint => TextureSampleType::Sint,
        wgpu::TextureSampleType::Uint => TextureSampleType::Uint,
        _ => TextureSampleType::Float,
    }
}

fn map_storage_texture_access(value: wgpu::StorageTextureAccess) -> StorageTextureAccess {
    match value {
        wgpu::StorageTextureAccess::ReadOnly => StorageTextureAccess::ReadOnly,
        wgpu::StorageTextureAccess::WriteOnly => StorageTextureAccess::WriteOnly,
        wgpu::StorageTextureAccess::ReadWrite => StorageTextureAccess::ReadWrite,
        _ => StorageTextureAccess::WriteOnly,
    }
}

fn map_vertex_format(value: wgpu::VertexFormat) -> VertexFormat {
    match value {
        wgpu::VertexFormat::Uint8x2 => VertexFormat::Uint8X2,
        wgpu::VertexFormat::Uint8x4 => VertexFormat::Uint8X4,
        wgpu::VertexFormat::Sint8x2 => VertexFormat::Sint8X2,
        wgpu::VertexFormat::Sint8x4 => VertexFormat::Sint8X4,
        wgpu::VertexFormat::Unorm8x2 => VertexFormat::Unorm8X2,
        wgpu::VertexFormat::Unorm8x4 => VertexFormat::Unorm8X4,
        wgpu::VertexFormat::Snorm8x2 => VertexFormat::Snorm8X2,
        wgpu::VertexFormat::Snorm8x4 => VertexFormat::Snorm8X4,
        wgpu::VertexFormat::Uint16x2 => VertexFormat::Uint16X2,
        wgpu::VertexFormat::Uint16x4 => VertexFormat::Uint16X4,
        wgpu::VertexFormat::Sint16x2 => VertexFormat::Sint16X2,
        wgpu::VertexFormat::Sint16x4 => VertexFormat::Sint16X4,
        wgpu::VertexFormat::Unorm16x2 => VertexFormat::Unorm16X2,
        wgpu::VertexFormat::Unorm16x4 => VertexFormat::Unorm16X4,
        wgpu::VertexFormat::Snorm16x2 => VertexFormat::Snorm16X2,
        wgpu::VertexFormat::Snorm16x4 => VertexFormat::Snorm16X4,
        wgpu::VertexFormat::Float16x2 => VertexFormat::Float16X2,
        wgpu::VertexFormat::Float16x4 => VertexFormat::Float16X4,
        wgpu::VertexFormat::Float32 => VertexFormat::Float32,
        wgpu::VertexFormat::Float32x2 => VertexFormat::Float32X2,
        wgpu::VertexFormat::Float32x3 => VertexFormat::Float32X3,
        wgpu::VertexFormat::Float32x4 => VertexFormat::Float32X4,
        wgpu::VertexFormat::Uint32 => VertexFormat::Uint32,
        wgpu::VertexFormat::Uint32x2 => VertexFormat::Uint32X2,
        wgpu::VertexFormat::Uint32x3 => VertexFormat::Uint32X3,
        wgpu::VertexFormat::Uint32x4 => VertexFormat::Uint32X4,
        wgpu::VertexFormat::Sint32 => VertexFormat::Sint32,
        wgpu::VertexFormat::Sint32x2 => VertexFormat::Sint32X2,
        wgpu::VertexFormat::Sint32x3 => VertexFormat::Sint32X3,
        wgpu::VertexFormat::Sint32x4 => VertexFormat::Sint32X4,
        wgpu::VertexFormat::Unorm10_10_10_2 => VertexFormat::Unorm1010102,
        wgpu::VertexFormat::Unorm8x4Bgra => VertexFormat::Unorm8X4Bgra,
        _ => VertexFormat::Float32X3,
    }
}

fn map_vertex_step_mode(value: wgpu::VertexStepMode) -> VertexStepMode {
    match value {
        wgpu::VertexStepMode::Vertex => VertexStepMode::Vertex,
        wgpu::VertexStepMode::Instance => VertexStepMode::Instance,
        _ => VertexStepMode::Vertex,
    }
}

fn map_load_op<T>(value: wgpu::LoadOp<T>) -> LoadOp {
    match value {
        wgpu::LoadOp::Load => LoadOp::Load,
        wgpu::LoadOp::Clear(_) => LoadOp::Clear,
        wgpu::LoadOp::DontCare(_) => LoadOp::Load,
    }
}

fn map_store_op(value: wgpu::StoreOp) -> StoreOp {
    match value {
        wgpu::StoreOp::Store => StoreOp::Store,
        wgpu::StoreOp::Discard => StoreOp::Discard,
        _ => StoreOp::Store,
    }
}

fn map_color(value: wgpu::Color) -> Color {
    Color {
        r: Some(value.r),
        g: Some(value.g),
        b: Some(value.b),
        a: Some(value.a),
    }
}

fn map_origin_3d(value: wgpu::Origin3d) -> Origin3D {
    Origin3D {
        x: Some(value.x),
        y: Some(value.y),
        z: Some(value.z),
    }
}

fn map_extent_3d(value: wgpu::Extent3d) -> Extent3D {
    Extent3D {
        width: Some(value.width),
        height: Some(value.height),
        depth_or_array_layers: Some(value.depth_or_array_layers),
    }
}

fn map_texel_copy_buffer_layout(value: wgpu::TexelCopyBufferLayout) -> TexelCopyBufferLayout {
    TexelCopyBufferLayout {
        offset: Some(value.offset),
        bytes_per_row: value.bytes_per_row,
        rows_per_image: value.rows_per_image,
    }
}

fn map_texel_copy_buffer_info(value: wgpu::TexelCopyBufferInfo<'_>) -> TexelCopyBufferInfo {
    TexelCopyBufferInfo {
        layout: Some(map_texel_copy_buffer_layout(value.layout)),
        buffer: Some(expect_buffer_from_api(value.buffer)),
    }
}

fn map_texel_copy_texture_info(value: wgpu::TexelCopyTextureInfo<'_>) -> TexelCopyTextureInfo {
    TexelCopyTextureInfo {
        texture: Some(expect_texture_from_api(value.texture)),
        mip_level: Some(value.mip_level),
        origin: Some(map_origin_3d(value.origin)),
        aspect: Some(map_texture_aspect(value.aspect)),
    }
}

fn map_shader_module_descriptor(desc: wgpu::ShaderModuleDescriptor<'_>) -> ShaderModuleDescriptor {
    let mut out = ShaderModuleDescriptor::new();
    out.label = label_to_string(desc.label);
    match desc.source {
        wgpu::ShaderSource::Wgsl(source) => {
            let ext = ShaderSourceWGSL {
                code: Some(source.to_string()),
            };
            out = out.with_extension(ShaderModuleDescriptorExtension::from(ext));
        }
        _ => panic!("wgpu-compat: unsupported shader source"),
    }
    out
}

fn map_buffer_descriptor(desc: &wgpu::BufferDescriptor<'_>) -> BufferDescriptor {
    let mut out = BufferDescriptor::new();
    out.label = label_to_string(desc.label);
    out.size = Some(desc.size);
    out.usage = Some(map_buffer_usages(desc.usage));
    out.mapped_at_creation = Some(desc.mapped_at_creation);
    out
}

fn map_texture_descriptor(desc: &wgpu::TextureDescriptor<'_>) -> TextureDescriptor {
    let mut out = TextureDescriptor::new();
    out.label = label_to_string(desc.label);
    out.size = Some(map_extent_3d(desc.size));
    out.mip_level_count = Some(desc.mip_level_count);
    out.sample_count = Some(desc.sample_count);
    out.dimension = Some(map_texture_dimension(desc.dimension));
    out.format = Some(map_texture_format(desc.format));
    out.usage = Some(map_texture_usages(desc.usage));
    out.view_formats = Some(
        desc.view_formats
            .iter()
            .copied()
            .map(map_texture_format)
            .collect(),
    );
    out
}

fn map_texture_view_descriptor(desc: &wgpu::TextureViewDescriptor<'_>) -> TextureViewDescriptor {
    let mut out = TextureViewDescriptor::new();
    out.label = label_to_string(desc.label);
    out.format = desc.format.map(map_texture_format);
    out.dimension = desc.dimension.map(map_texture_view_dimension);
    out.aspect = Some(map_texture_aspect(desc.aspect));
    out.base_mip_level = Some(desc.base_mip_level);
    if let Some(count) = desc.mip_level_count {
        out.mip_level_count = Some(count);
    }
    out.base_array_layer = Some(desc.base_array_layer);
    if let Some(count) = desc.array_layer_count {
        out.array_layer_count = Some(count);
    }
    out.usage = desc.usage.map(map_texture_usages);
    out
}

fn map_sampler_descriptor(desc: &wgpu::SamplerDescriptor<'_>) -> SamplerDescriptor {
    let mut out = SamplerDescriptor::new();
    out.label = label_to_string(desc.label);
    out.address_mode_u = Some(map_address_mode(desc.address_mode_u));
    out.address_mode_v = Some(map_address_mode(desc.address_mode_v));
    out.address_mode_w = Some(map_address_mode(desc.address_mode_w));
    out.mag_filter = Some(map_filter_mode(desc.mag_filter));
    out.min_filter = Some(map_filter_mode(desc.min_filter));
    out.mipmap_filter = Some(map_mipmap_filter_mode(desc.mipmap_filter));
    out.lod_min_clamp = Some(desc.lod_min_clamp);
    out.lod_max_clamp = Some(desc.lod_max_clamp);
    out.compare = desc.compare.map(map_compare_function);
    out.max_anisotropy = Some(desc.anisotropy_clamp);
    out
}

fn map_bind_group_layout_entry(entry: &wgpu::BindGroupLayoutEntry) -> BindGroupLayoutEntry {
    let mut out = BindGroupLayoutEntry::new();
    out.binding = Some(entry.binding);
    out.visibility = Some(map_shader_stages(entry.visibility));
    out.binding_array_size = entry.count.map(|v| v.get()).or(Some(0));

    match entry.ty {
        wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset,
            min_binding_size,
        } => {
            let mut layout = BufferBindingLayout::new();
            layout.r#type = Some(match ty {
                wgpu::BufferBindingType::Uniform => BufferBindingType::Uniform,
                wgpu::BufferBindingType::Storage { read_only } => {
                    if read_only {
                        BufferBindingType::ReadOnlyStorage
                    } else {
                        BufferBindingType::Storage
                    }
                }
                _ => BufferBindingType::Uniform,
            });
            layout.has_dynamic_offset = Some(has_dynamic_offset);
            layout.min_binding_size = min_binding_size.map(|v| v.get());
            out.buffer = Some(layout);
        }
        wgpu::BindingType::Sampler(ty) => {
            let mut layout = SamplerBindingLayout::new();
            layout.r#type = Some(map_sampler_binding_type(ty));
            out.sampler = Some(layout);
        }
        wgpu::BindingType::Texture {
            sample_type,
            view_dimension,
            multisampled,
        } => {
            let mut layout = TextureBindingLayout::new();
            layout.sample_type = Some(map_texture_sample_type(sample_type));
            layout.view_dimension = Some(map_texture_view_dimension(view_dimension));
            layout.multisampled = Some(multisampled);
            out.texture = Some(layout);
        }
        wgpu::BindingType::StorageTexture {
            access,
            format,
            view_dimension,
        } => {
            let mut layout = StorageTextureBindingLayout::new();
            layout.access = Some(map_storage_texture_access(access));
            layout.format = Some(map_texture_format(format));
            layout.view_dimension = Some(map_texture_view_dimension(view_dimension));
            out.storage_texture = Some(layout);
        }
        _ => panic!("wgpu-compat: unsupported binding type"),
    }

    out
}

fn map_bind_group_layout_descriptor(
    desc: &wgpu::BindGroupLayoutDescriptor<'_>,
) -> BindGroupLayoutDescriptor {
    let mut out = BindGroupLayoutDescriptor::new();
    out.label = label_to_string(desc.label);
    out.entries = Some(
        desc.entries
            .iter()
            .map(map_bind_group_layout_entry)
            .collect(),
    );
    out
}

fn map_bind_group_entry(entry: &wgpu::BindGroupEntry<'_>) -> BindGroupEntry {
    let mut out = BindGroupEntry::default();
    out.binding = Some(entry.binding);
    match &entry.resource {
        wgpu::BindingResource::Buffer(buffer) => {
            out.buffer = Some(expect_buffer_from_api(buffer.buffer));
            out.offset = Some(buffer.offset);
            out.size = Some(buffer.size.map(|v| v.get()).unwrap_or(WHOLE_SIZE));
        }
        wgpu::BindingResource::Sampler(sampler) => {
            out.sampler = Some(expect_sampler_from_api(sampler));
        }
        wgpu::BindingResource::TextureView(view) => {
            out.texture_view = Some(expect_texture_view_from_api(view));
        }
        _ => panic!("wgpu-compat: binding resource arrays not supported"),
    }
    out
}

fn map_bind_group_descriptor(desc: &wgpu::BindGroupDescriptor<'_>) -> BindGroupDescriptor {
    let mut out = BindGroupDescriptor::new();
    out.label = label_to_string(desc.label);
    out.layout = Some(expect_bind_group_layout_from_api(desc.layout));
    out.entries = Some(desc.entries.iter().map(map_bind_group_entry).collect());
    out
}

fn map_pipeline_layout_descriptor(
    desc: &wgpu::PipelineLayoutDescriptor<'_>,
) -> PipelineLayoutDescriptor {
    let mut out = PipelineLayoutDescriptor::new();
    out.label = label_to_string(desc.label);
    out.bind_group_layouts = Some(
        desc.bind_group_layouts
            .iter()
            .copied()
            .map(expect_bind_group_layout_from_api)
            .collect(),
    );
    out.immediate_size = Some(0);
    out
}

fn map_vertex_attribute(attr: &wgpu::VertexAttribute) -> VertexAttribute {
    let mut out = VertexAttribute::new();
    out.format = Some(map_vertex_format(attr.format));
    out.offset = Some(attr.offset);
    out.shader_location = Some(attr.shader_location);
    out
}

fn map_vertex_buffer_layout(layout: &wgpu::VertexBufferLayout<'_>) -> VertexBufferLayout {
    let mut out = VertexBufferLayout::new();
    out.array_stride = Some(layout.array_stride);
    out.step_mode = Some(map_vertex_step_mode(layout.step_mode));
    out.attributes = Some(layout.attributes.iter().map(map_vertex_attribute).collect());
    out
}

fn map_vertex_state(state: &wgpu::VertexState<'_>) -> VertexState {
    let mut out = VertexState::new();
    out.module = Some(expect_shader_module_from_api(state.module));
    out.entry_point = state.entry_point.map(str::to_string);
    out.constants = Some(
        state
            .compilation_options
            .constants
            .iter()
            .map(|(k, v)| {
                let mut entry = ConstantEntry::new();
                entry.key = Some(k.to_string());
                entry.value = Some(*v);
                entry
            })
            .collect(),
    );
    out.buffers = Some(state.buffers.iter().map(map_vertex_buffer_layout).collect());
    out
}

fn map_blend_component(comp: &wgpu::BlendComponent) -> BlendComponent {
    BlendComponent {
        operation: Some(map_blend_operation(comp.operation)),
        src_factor: Some(map_blend_factor(comp.src_factor)),
        dst_factor: Some(map_blend_factor(comp.dst_factor)),
    }
}

fn map_blend_state(state: &wgpu::BlendState) -> BlendState {
    BlendState {
        color: Some(map_blend_component(&state.color)),
        alpha: Some(map_blend_component(&state.alpha)),
    }
}

fn map_color_target_state(state: &wgpu::ColorTargetState) -> ColorTargetState {
    let mut out = ColorTargetState::new();
    out.format = Some(map_texture_format(state.format));
    out.blend = state.blend.as_ref().map(map_blend_state);
    out.write_mask = Some(map_color_write_mask(state.write_mask));
    out
}

fn map_fragment_state(state: &wgpu::FragmentState<'_>) -> FragmentState {
    let mut out = FragmentState::new();
    out.module = Some(expect_shader_module_from_api(state.module));
    out.entry_point = state.entry_point.map(str::to_string);
    out.constants = Some(
        state
            .compilation_options
            .constants
            .iter()
            .map(|(k, v)| {
                let mut entry = ConstantEntry::new();
                entry.key = Some(k.to_string());
                entry.value = Some(*v);
                entry
            })
            .collect(),
    );
    out.targets = Some(
        state
            .targets
            .iter()
            .map(|t| t.as_ref().map(map_color_target_state).unwrap_or_default())
            .collect(),
    );
    out
}

fn map_stencil_face_state(state: &wgpu::StencilFaceState) -> StencilFaceState {
    StencilFaceState {
        compare: Some(map_compare_function(state.compare)),
        fail_op: Some(map_stencil_operation(state.fail_op)),
        depth_fail_op: Some(map_stencil_operation(state.depth_fail_op)),
        pass_op: Some(map_stencil_operation(state.pass_op)),
    }
}

fn map_depth_stencil_state(state: &wgpu::DepthStencilState) -> DepthStencilState {
    let mut out = DepthStencilState::new();
    out.format = Some(map_texture_format(state.format));
    out.depth_write_enabled = Some(if state.depth_write_enabled {
        OptionalBool::True
    } else {
        OptionalBool::False
    });
    out.depth_compare = Some(map_compare_function(state.depth_compare));
    out.stencil_front = Some(map_stencil_face_state(&state.stencil.front));
    out.stencil_back = Some(map_stencil_face_state(&state.stencil.back));
    out.stencil_read_mask = Some(state.stencil.read_mask);
    out.stencil_write_mask = Some(state.stencil.write_mask);
    out.depth_bias = Some(state.bias.constant);
    out.depth_bias_slope_scale = Some(state.bias.slope_scale);
    out.depth_bias_clamp = Some(state.bias.clamp);
    out
}

fn map_multisample_state(state: wgpu::MultisampleState) -> MultisampleState {
    let mut out = MultisampleState::new();
    out.count = Some(state.count);
    out.mask = Some(state.mask.try_into().unwrap_or(u32::MAX));
    out.alpha_to_coverage_enabled = Some(state.alpha_to_coverage_enabled);
    out
}

fn map_primitive_state(state: wgpu::PrimitiveState) -> PrimitiveState {
    let mut out = PrimitiveState::new();
    out.topology = Some(map_primitive_topology(state.topology));
    out.strip_index_format = state.strip_index_format.map(map_index_format);
    out.front_face = Some(map_front_face(state.front_face));
    out.cull_mode = Some(map_cull_mode(state.cull_mode));
    out.unclipped_depth = Some(state.unclipped_depth);
    out
}

fn map_render_pipeline_descriptor(
    desc: &wgpu::RenderPipelineDescriptor<'_>,
) -> RenderPipelineDescriptor {
    let mut out = RenderPipelineDescriptor::new();
    out.label = label_to_string(desc.label);
    out.layout = desc.layout.map(expect_pipeline_layout_from_api);
    out.vertex = Some(map_vertex_state(&desc.vertex));
    out.primitive = Some(map_primitive_state(desc.primitive));
    out.depth_stencil = desc.depth_stencil.as_ref().map(map_depth_stencil_state);
    out.multisample = Some(map_multisample_state(desc.multisample));
    out.fragment = desc.fragment.as_ref().map(map_fragment_state);
    out
}

fn map_compute_pipeline_descriptor(
    desc: &wgpu::ComputePipelineDescriptor<'_>,
) -> ComputePipelineDescriptor {
    let mut compute = ComputeState::new();
    compute.module = Some(expect_shader_module_from_api(desc.module));
    compute.entry_point = desc.entry_point.map(str::to_string);
    compute.constants = Some(
        desc.compilation_options
            .constants
            .iter()
            .map(|(k, v)| {
                let mut entry = ConstantEntry::new();
                entry.key = Some(k.to_string());
                entry.value = Some(*v);
                entry
            })
            .collect(),
    );

    let mut out = ComputePipelineDescriptor::new();
    out.label = label_to_string(desc.label);
    out.layout = desc.layout.map(expect_pipeline_layout_from_api);
    out.compute = Some(compute);
    out
}

fn map_command_encoder_descriptor(
    desc: &wgpu::CommandEncoderDescriptor<'_>,
) -> CommandEncoderDescriptor {
    let mut out = CommandEncoderDescriptor::new();
    out.label = label_to_string(desc.label);
    out
}

fn map_render_bundle_encoder_descriptor(
    desc: &wgpu::RenderBundleEncoderDescriptor<'_>,
) -> RenderBundleEncoderDescriptor {
    let (depth_stencil_format, depth_read_only, stencil_read_only) = match desc.depth_stencil {
        Some(depth_stencil) => (
            Some(map_texture_format(depth_stencil.format)),
            Some(depth_stencil.depth_read_only),
            Some(depth_stencil.stencil_read_only),
        ),
        None => (None, None, None),
    };
    let mut out = RenderBundleEncoderDescriptor::new();
    out.label = label_to_string(desc.label);
    out.color_formats = Some(
        desc.color_formats
            .iter()
            .map(|v| {
                v.map(map_texture_format)
                    .unwrap_or(TextureFormat::Undefined)
            })
            .collect(),
    );
    out.depth_stencil_format = depth_stencil_format;
    out.sample_count = Some(desc.sample_count);
    out.depth_read_only = depth_read_only;
    out.stencil_read_only = stencil_read_only;
    out
}

fn map_render_pass_color_attachment(
    attachment: &wgpu::RenderPassColorAttachment<'_>,
) -> RenderPassColorAttachment {
    let (load_op, clear_value) = match attachment.ops.load {
        wgpu::LoadOp::Load => (LoadOp::Load, None),
        wgpu::LoadOp::Clear(value) => (LoadOp::Clear, Some(map_color(value))),
        wgpu::LoadOp::DontCare(_) => (LoadOp::Load, None),
    };
    let mut out = RenderPassColorAttachment::new();
    out.view = Some(expect_texture_view_from_api(attachment.view));
    if let Some(depth_slice) = attachment.depth_slice {
        out.depth_slice = Some(depth_slice);
    }
    out.resolve_target = attachment.resolve_target.map(expect_texture_view_from_api);
    out.load_op = Some(load_op);
    out.store_op = Some(map_store_op(attachment.ops.store));
    out.clear_value = clear_value;
    out
}

fn map_render_pass_depth_stencil_attachment(
    attachment: &wgpu::RenderPassDepthStencilAttachment<'_>,
) -> RenderPassDepthStencilAttachment {
    let depth_read_only = attachment.depth_ops.is_none();
    let stencil_read_only = attachment.stencil_ops.is_none();
    let (depth_load_op, depth_store_op, depth_clear_value) = match attachment.depth_ops.as_ref() {
        Some(ops) => match ops.load {
            wgpu::LoadOp::Load => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
            wgpu::LoadOp::Clear(value) => (
                Some(LoadOp::Clear),
                Some(map_store_op(ops.store)),
                Some(value),
            ),
            wgpu::LoadOp::DontCare(_) => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
        },
        None => (None, None, None),
    };
    let (stencil_load_op, stencil_store_op, stencil_clear_value) = match attachment
        .stencil_ops
        .as_ref()
    {
        Some(ops) => match ops.load {
            wgpu::LoadOp::Load => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
            wgpu::LoadOp::Clear(value) => (
                Some(LoadOp::Clear),
                Some(map_store_op(ops.store)),
                Some(value),
            ),
            wgpu::LoadOp::DontCare(_) => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
        },
        None => (None, None, None),
    };
    let mut out = RenderPassDepthStencilAttachment::new();
    out.view = Some(expect_texture_view_from_api(attachment.view));
    out.depth_load_op = depth_load_op;
    out.depth_store_op = depth_store_op;
    out.depth_clear_value = depth_clear_value;
    out.depth_read_only = Some(depth_read_only);
    out.stencil_load_op = stencil_load_op;
    out.stencil_store_op = stencil_store_op;
    out.stencil_clear_value = stencil_clear_value;
    out.stencil_read_only = Some(stencil_read_only);
    out
}

fn map_render_pass_descriptor(desc: &wgpu::RenderPassDescriptor<'_>) -> RenderPassDescriptor {
    let mut out = RenderPassDescriptor::new();
    out.label = label_to_string(desc.label);
    out.color_attachments = Some(
        desc.color_attachments
            .iter()
            .map(|att| {
                att.as_ref()
                    .map(map_render_pass_color_attachment)
                    .unwrap_or_default()
            })
            .collect(),
    );
    out.depth_stencil_attachment = desc
        .depth_stencil_attachment
        .as_ref()
        .map(map_render_pass_depth_stencil_attachment);
    out.occlusion_query_set = desc.occlusion_query_set.map(expect_query_set_from_api);
    out.timestamp_writes = None;
    out
}

fn map_compute_pass_descriptor(desc: &wgpu::ComputePassDescriptor<'_>) -> ComputePassDescriptor {
    let mut out = ComputePassDescriptor::new();
    out.label = label_to_string(desc.label);
    out.timestamp_writes = None;
    out
}

fn map_surface_configuration(config: &wgpu::SurfaceConfiguration) -> SurfaceConfiguration {
    let mut out = SurfaceConfiguration::new();
    out.format = Some(map_texture_format(config.format));
    out.usage = Some(map_texture_usages(config.usage));
    out.width = Some(config.width);
    out.height = Some(config.height);
    out.present_mode = Some(match config.present_mode {
        wgpu::PresentMode::Fifo => PresentMode::Fifo,
        wgpu::PresentMode::Mailbox => PresentMode::Mailbox,
        wgpu::PresentMode::Immediate => PresentMode::Immediate,
        wgpu::PresentMode::AutoVsync => PresentMode::Fifo,
        wgpu::PresentMode::AutoNoVsync => PresentMode::Immediate,
        _ => PresentMode::Fifo,
    });
    out.alpha_mode = Some(match config.alpha_mode {
        wgpu::CompositeAlphaMode::Auto => CompositeAlphaMode::Auto,
        wgpu::CompositeAlphaMode::Opaque => CompositeAlphaMode::Opaque,
        wgpu::CompositeAlphaMode::PreMultiplied => CompositeAlphaMode::Premultiplied,
        wgpu::CompositeAlphaMode::PostMultiplied => CompositeAlphaMode::Premultiplied,
        wgpu::CompositeAlphaMode::Inherit => CompositeAlphaMode::Inherit,
        _ => CompositeAlphaMode::Auto,
    });
    out.view_formats = Some(
        config
            .view_formats
            .iter()
            .copied()
            .map(map_texture_format)
            .collect(),
    );
    out
}

fn map_surface_capabilities(caps: SurfaceCapabilities) -> wgpu::SurfaceCapabilities {
    wgpu::SurfaceCapabilities {
        formats: caps
            .formats
            .clone()
            .unwrap_or_default()
            .iter()
            .copied()
            .map(map_texture_format_to_wgpu)
            .collect(),
        present_modes: caps
            .present_modes
            .clone()
            .unwrap_or_default()
            .iter()
            .copied()
            .map(|m| match m {
                PresentMode::Fifo => wgpu::PresentMode::Fifo,
                PresentMode::Mailbox => wgpu::PresentMode::Mailbox,
                PresentMode::Immediate => wgpu::PresentMode::Immediate,
                PresentMode::FifoRelaxed => wgpu::PresentMode::AutoVsync,
                _ => wgpu::PresentMode::Fifo,
            })
            .collect(),
        alpha_modes: caps
            .alpha_modes
            .clone()
            .unwrap_or_default()
            .iter()
            .copied()
            .map(|m| match m {
                CompositeAlphaMode::Auto => wgpu::CompositeAlphaMode::Auto,
                CompositeAlphaMode::Opaque => wgpu::CompositeAlphaMode::Opaque,
                CompositeAlphaMode::Premultiplied => wgpu::CompositeAlphaMode::PreMultiplied,
                CompositeAlphaMode::Unpremultiplied => wgpu::CompositeAlphaMode::PostMultiplied,
                CompositeAlphaMode::Inherit => wgpu::CompositeAlphaMode::Inherit,
                _ => wgpu::CompositeAlphaMode::Auto,
            })
            .collect(),
        usages: map_texture_usage_to_wgpu(caps.usages.unwrap_or(TextureUsage::RENDER_ATTACHMENT)),
    }
}

fn map_compilation_info(info: &CompilationInfo) -> wgpu::CompilationInfo {
    wgpu::CompilationInfo {
        messages: info
            .messages
            .as_ref()
            .map(|messages| {
                messages
                    .iter()
                    .map(|message| wgpu::CompilationMessage {
                        message: message.message.clone().unwrap_or_default(),
                        message_type: match message.r#type.unwrap_or(CompilationMessageType::Info) {
                            CompilationMessageType::Error => wgpu::CompilationMessageType::Error,
                            CompilationMessageType::Warning => {
                                wgpu::CompilationMessageType::Warning
                            }
                            CompilationMessageType::Info => wgpu::CompilationMessageType::Info,
                        },
                        location: Some(wgpu::SourceLocation {
                            line_number: message.line_num.unwrap_or(0) as u32,
                            line_position: message.line_pos.unwrap_or(0) as u32,
                            offset: message.offset.unwrap_or(0) as u32,
                            length: message.length.unwrap_or(0) as u32,
                        }),
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn map_uncaptured_error(error_type: ErrorType, message: String) -> wgpu::Error {
    match error_type {
        ErrorType::OutOfMemory => wgpu::Error::OutOfMemory {
            source: Box::new(DawnError(message)),
        },
        ErrorType::Validation => wgpu::Error::Validation {
            source: Box::new(DawnError(message.clone())),
            description: message,
        },
        ErrorType::Internal | ErrorType::Unknown => wgpu::Error::Internal {
            source: Box::new(DawnError(message.clone())),
            description: message,
        },
        ErrorType::NoError => wgpu::Error::Internal {
            source: Box::new(DawnError("no error".to_string())),
            description: "no error".to_string(),
        },
    }
}

fn map_limits_to_dawn(limits: &wgpu::Limits) -> Limits {
    let mut out = Limits::new();
    out.max_texture_dimension_1d = Some(limits.max_texture_dimension_1d);
    out.max_texture_dimension_2d = Some(limits.max_texture_dimension_2d);
    out.max_texture_dimension_3d = Some(limits.max_texture_dimension_3d);
    out.max_texture_array_layers = Some(limits.max_texture_array_layers);
    out.max_bind_groups = Some(limits.max_bind_groups);
    out.max_bindings_per_bind_group = Some(limits.max_bindings_per_bind_group);
    out.max_dynamic_uniform_buffers_per_pipeline_layout =
        Some(limits.max_dynamic_uniform_buffers_per_pipeline_layout);
    out.max_dynamic_storage_buffers_per_pipeline_layout =
        Some(limits.max_dynamic_storage_buffers_per_pipeline_layout);
    out.max_sampled_textures_per_shader_stage = Some(limits.max_sampled_textures_per_shader_stage);
    out.max_samplers_per_shader_stage = Some(limits.max_samplers_per_shader_stage);
    out.max_storage_buffers_per_shader_stage = Some(limits.max_storage_buffers_per_shader_stage);
    out.max_storage_textures_per_shader_stage = Some(limits.max_storage_textures_per_shader_stage);
    out.max_uniform_buffers_per_shader_stage = Some(limits.max_uniform_buffers_per_shader_stage);
    out.max_uniform_buffer_binding_size = Some(limits.max_uniform_buffer_binding_size as u64);
    out.max_storage_buffer_binding_size = Some(limits.max_storage_buffer_binding_size as u64);
    out.min_uniform_buffer_offset_alignment = Some(limits.min_uniform_buffer_offset_alignment);
    out.min_storage_buffer_offset_alignment = Some(limits.min_storage_buffer_offset_alignment);
    out.max_vertex_buffers = Some(limits.max_vertex_buffers);
    out.max_buffer_size = Some(limits.max_buffer_size);
    out.max_vertex_attributes = Some(limits.max_vertex_attributes);
    out.max_vertex_buffer_array_stride = Some(limits.max_vertex_buffer_array_stride);
    out.max_inter_stage_shader_variables = Some(limits.max_inter_stage_shader_components);
    out.max_color_attachments = Some(limits.max_color_attachments);
    out.max_color_attachment_bytes_per_sample = Some(limits.max_color_attachment_bytes_per_sample);
    out.max_compute_workgroup_storage_size = Some(limits.max_compute_workgroup_storage_size);
    out.max_compute_invocations_per_workgroup = Some(limits.max_compute_invocations_per_workgroup);
    out.max_compute_workgroup_size_x = Some(limits.max_compute_workgroup_size_x);
    out.max_compute_workgroup_size_y = Some(limits.max_compute_workgroup_size_y);
    out.max_compute_workgroup_size_z = Some(limits.max_compute_workgroup_size_z);
    out.max_compute_workgroups_per_dimension = Some(limits.max_compute_workgroups_per_dimension);
    out.max_bind_groups_plus_vertex_buffers =
        Some(limits.max_bind_groups + limits.max_vertex_buffers);
    out
}

fn map_limits_to_wgpu(limits: &Limits) -> wgpu::Limits {
    let mut out = wgpu::Limits::default();
    if let Some(value) = limits.max_texture_dimension_1d {
        out.max_texture_dimension_1d = value;
    }
    if let Some(value) = limits.max_texture_dimension_2d {
        out.max_texture_dimension_2d = value;
    }
    if let Some(value) = limits.max_texture_dimension_3d {
        out.max_texture_dimension_3d = value;
    }
    if let Some(value) = limits.max_texture_array_layers {
        out.max_texture_array_layers = value;
    }
    if let Some(value) = limits.max_bind_groups {
        out.max_bind_groups = value;
    }
    if let Some(value) = limits.max_bindings_per_bind_group {
        out.max_bindings_per_bind_group = value;
    }
    if let Some(value) = limits.max_dynamic_uniform_buffers_per_pipeline_layout {
        out.max_dynamic_uniform_buffers_per_pipeline_layout = value;
    }
    if let Some(value) = limits.max_dynamic_storage_buffers_per_pipeline_layout {
        out.max_dynamic_storage_buffers_per_pipeline_layout = value;
    }
    if let Some(value) = limits.max_sampled_textures_per_shader_stage {
        out.max_sampled_textures_per_shader_stage = value;
    }
    if let Some(value) = limits.max_samplers_per_shader_stage {
        out.max_samplers_per_shader_stage = value;
    }
    if let Some(value) = limits.max_storage_buffers_per_shader_stage {
        out.max_storage_buffers_per_shader_stage = value;
    }
    if let Some(value) = limits.max_storage_textures_per_shader_stage {
        out.max_storage_textures_per_shader_stage = value;
    }
    if let Some(value) = limits.max_uniform_buffers_per_shader_stage {
        out.max_uniform_buffers_per_shader_stage = value;
    }
    if let Some(value) = limits.max_uniform_buffer_binding_size {
        out.max_uniform_buffer_binding_size = value as u32;
    }
    if let Some(value) = limits.max_storage_buffer_binding_size {
        out.max_storage_buffer_binding_size = value as u32;
    }
    if let Some(value) = limits.min_uniform_buffer_offset_alignment {
        out.min_uniform_buffer_offset_alignment = value;
    }
    if let Some(value) = limits.min_storage_buffer_offset_alignment {
        out.min_storage_buffer_offset_alignment = value;
    }
    if let Some(value) = limits.max_vertex_buffers {
        out.max_vertex_buffers = value;
    }
    if let Some(value) = limits.max_buffer_size {
        out.max_buffer_size = value;
    }
    if let Some(value) = limits.max_vertex_attributes {
        out.max_vertex_attributes = value;
    }
    if let Some(value) = limits.max_vertex_buffer_array_stride {
        out.max_vertex_buffer_array_stride = value;
    }
    if let Some(value) = limits.max_inter_stage_shader_variables {
        out.max_inter_stage_shader_components = value;
    }
    if let Some(value) = limits.max_color_attachments {
        out.max_color_attachments = value;
    }
    if let Some(value) = limits.max_color_attachment_bytes_per_sample {
        out.max_color_attachment_bytes_per_sample = value;
    }
    if let Some(value) = limits.max_compute_workgroup_storage_size {
        out.max_compute_workgroup_storage_size = value;
    }
    if let Some(value) = limits.max_compute_invocations_per_workgroup {
        out.max_compute_invocations_per_workgroup = value;
    }
    if let Some(value) = limits.max_compute_workgroup_size_x {
        out.max_compute_workgroup_size_x = value;
    }
    if let Some(value) = limits.max_compute_workgroup_size_y {
        out.max_compute_workgroup_size_y = value;
    }
    if let Some(value) = limits.max_compute_workgroup_size_z {
        out.max_compute_workgroup_size_z = value;
    }
    if let Some(value) = limits.max_compute_workgroups_per_dimension {
        out.max_compute_workgroups_per_dimension = value;
    }
    out
}

fn map_features_to_dawn(features: wgpu::Features) -> Vec<FeatureName> {
    let mut out = Vec::new();
    if features.contains(wgpu::Features::DEPTH_CLIP_CONTROL) {
        out.push(FeatureName::DepthClipControl);
    }
    if features.contains(wgpu::Features::DEPTH32FLOAT_STENCIL8) {
        out.push(FeatureName::Depth32FloatStencil8);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
        out.push(FeatureName::TextureCompressionBc);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC_SLICED_3D) {
        out.push(FeatureName::TextureCompressionBcSliced3D);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2) {
        out.push(FeatureName::TextureCompressionEtc2);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC) {
        out.push(FeatureName::TextureCompressionAstc);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_HDR) {
        out.push(FeatureName::TextureCompressionAstc);
    }
    if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        out.push(FeatureName::TimestampQuery);
    }
    if features.contains(wgpu::Features::INDIRECT_FIRST_INSTANCE) {
        out.push(FeatureName::IndirectFirstInstance);
    }
    if features.contains(wgpu::Features::SHADER_F16) {
        out.push(FeatureName::ShaderF16);
    }
    if features.contains(wgpu::Features::RG11B10UFLOAT_RENDERABLE) {
        out.push(FeatureName::Rg11B10UfloatRenderable);
    }
    if features.contains(wgpu::Features::BGRA8UNORM_STORAGE) {
        out.push(FeatureName::Bgra8UnormStorage);
    }
    if features.contains(wgpu::Features::FLOAT32_FILTERABLE) {
        out.push(FeatureName::Float32Filterable);
    }
    if features.contains(wgpu::Features::CLIP_DISTANCES) {
        out.push(FeatureName::ClipDistances);
    }
    if features.contains(wgpu::Features::DUAL_SOURCE_BLENDING) {
        out.push(FeatureName::DualSourceBlending);
    }
    if features.contains(wgpu::Features::SUBGROUP) {
        out.push(FeatureName::Subgroups);
    }
    if features.contains(wgpu::Features::TEXTURE_FORMAT_16BIT_NORM) {
        out.push(FeatureName::Unorm16TextureFormats);
    }
    if features.contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT) {
        out.push(FeatureName::MultiDrawIndirect);
    }
    out
}

fn map_features_to_wgpu(features: &SupportedFeatures) -> wgpu::Features {
    let mut out = wgpu::Features::empty();
    if let Some(list) = &features.features {
        for feature in list {
            match feature {
                FeatureName::DepthClipControl => out |= wgpu::Features::DEPTH_CLIP_CONTROL,
                FeatureName::Depth32FloatStencil8 => out |= wgpu::Features::DEPTH32FLOAT_STENCIL8,
                FeatureName::TextureCompressionBc => out |= wgpu::Features::TEXTURE_COMPRESSION_BC,
                FeatureName::TextureCompressionBcSliced3D => {
                    out |= wgpu::Features::TEXTURE_COMPRESSION_BC_SLICED_3D
                }
                FeatureName::TextureCompressionEtc2 => {
                    out |= wgpu::Features::TEXTURE_COMPRESSION_ETC2
                }
                FeatureName::TextureCompressionAstc => {
                    out |= wgpu::Features::TEXTURE_COMPRESSION_ASTC
                }
                FeatureName::TimestampQuery => out |= wgpu::Features::TIMESTAMP_QUERY,
                FeatureName::IndirectFirstInstance => {
                    out |= wgpu::Features::INDIRECT_FIRST_INSTANCE
                }
                FeatureName::ShaderF16 => out |= wgpu::Features::SHADER_F16,
                FeatureName::Rg11B10UfloatRenderable => {
                    out |= wgpu::Features::RG11B10UFLOAT_RENDERABLE
                }
                FeatureName::Bgra8UnormStorage => out |= wgpu::Features::BGRA8UNORM_STORAGE,
                FeatureName::Float32Filterable => out |= wgpu::Features::FLOAT32_FILTERABLE,
                FeatureName::Float32Blendable => {}
                FeatureName::ClipDistances => out |= wgpu::Features::CLIP_DISTANCES,
                FeatureName::DualSourceBlending => out |= wgpu::Features::DUAL_SOURCE_BLENDING,
                FeatureName::Subgroups => out |= wgpu::Features::SUBGROUP,
                FeatureName::Unorm16TextureFormats => {
                    out |= wgpu::Features::TEXTURE_FORMAT_16BIT_NORM
                }
                FeatureName::MultiDrawIndirect => out |= wgpu::Features::MULTI_DRAW_INDIRECT_COUNT,
                _ => {}
            }
        }
    }
    out
}

fn bytes_to_u32(data: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity((data.len() + 3) / 4);
    for chunk in data.chunks(4) {
        let mut buf = [0u8; 4];
        buf[..chunk.len()].copy_from_slice(chunk);
        out.push(u32::from_le_bytes(buf));
    }
    out
}

fn dispatch_adapter(adapter: Adapter) -> DispatchAdapter {
    DispatchAdapter::custom(DawnAdapter { inner: adapter })
}

fn dispatch_device(device: Device) -> DispatchDevice {
    DispatchDevice::custom(DawnDevice { inner: device })
}

fn dispatch_queue(queue: Queue) -> DispatchQueue {
    DispatchQueue::custom(DawnQueue { inner: queue })
}

fn dispatch_surface(surface: DawnSurface) -> DispatchSurface {
    DispatchSurface::custom(surface)
}

fn dispatch_shader_module(module: ShaderModule) -> DispatchShaderModule {
    DispatchShaderModule::custom(DawnShaderModule { inner: module })
}

fn dispatch_bind_group_layout(layout: BindGroupLayout) -> DispatchBindGroupLayout {
    DispatchBindGroupLayout::custom(DawnBindGroupLayout { inner: layout })
}

fn dispatch_bind_group(group: BindGroup) -> DispatchBindGroup {
    DispatchBindGroup::custom(DawnBindGroup { inner: group })
}

fn dispatch_texture_view(view: TextureView) -> DispatchTextureView {
    DispatchTextureView::custom(DawnTextureView { inner: view })
}

fn dispatch_sampler(sampler: Sampler) -> DispatchSampler {
    DispatchSampler::custom(DawnSampler { inner: sampler })
}

fn dispatch_buffer(buffer: Buffer) -> DispatchBuffer {
    DispatchBuffer::custom(DawnBuffer { inner: buffer })
}

fn dispatch_texture(texture: Texture) -> DispatchTexture {
    DispatchTexture::custom(DawnTexture { inner: texture })
}

fn dispatch_external_texture(texture: ExternalTexture) -> DispatchExternalTexture {
    DispatchExternalTexture::custom(DawnExternalTexture { inner: texture })
}

fn dispatch_query_set(query_set: QuerySet) -> DispatchQuerySet {
    DispatchQuerySet::custom(DawnQuerySet { inner: query_set })
}

fn dispatch_pipeline_layout(layout: PipelineLayout) -> DispatchPipelineLayout {
    DispatchPipelineLayout::custom(DawnPipelineLayout { inner: layout })
}

fn dispatch_render_pipeline(pipeline: RenderPipeline) -> DispatchRenderPipeline {
    DispatchRenderPipeline::custom(DawnRenderPipeline { inner: pipeline })
}

fn dispatch_compute_pipeline(pipeline: ComputePipeline) -> DispatchComputePipeline {
    DispatchComputePipeline::custom(DawnComputePipeline { inner: pipeline })
}

fn dispatch_command_encoder(encoder: CommandEncoder) -> DispatchCommandEncoder {
    DispatchCommandEncoder::custom(DawnCommandEncoder { inner: encoder })
}

fn dispatch_command_buffer(buffer: CommandBuffer) -> DispatchCommandBuffer {
    DispatchCommandBuffer::custom(DawnCommandBuffer { inner: buffer })
}

fn dispatch_compute_pass(pass: ComputePassEncoder) -> DispatchComputePass {
    DispatchComputePass::custom(DawnComputePass {
        inner: pass,
        ended: false,
    })
}

fn dispatch_render_pass(pass: RenderPassEncoder) -> DispatchRenderPass {
    DispatchRenderPass::custom(DawnRenderPass {
        inner: pass,
        ended: false,
    })
}

fn dispatch_render_bundle_encoder(encoder: RenderBundleEncoder) -> DispatchRenderBundleEncoder {
    DispatchRenderBundleEncoder::custom(DawnRenderBundleEncoder { inner: encoder })
}

fn dispatch_render_bundle(bundle: RenderBundle) -> DispatchRenderBundle {
    DispatchRenderBundle::custom(DawnRenderBundle { inner: bundle })
}

fn dispatch_surface_output_detail(surface: Surface) -> DispatchSurfaceOutputDetail {
    DispatchSurfaceOutputDetail::custom(DawnSurfaceOutputDetail { surface })
}

fn dispatch_queue_write_buffer(data: Vec<u8>) -> DispatchQueueWriteBuffer {
    DispatchQueueWriteBuffer::custom(DawnQueueWriteBuffer { inner: data })
}

fn dispatch_buffer_mapped_range(ptr: *mut u8, size: usize) -> DispatchBufferMappedRange {
    DispatchBufferMappedRange::custom(DawnBufferMappedRange { data: ptr, size })
}

fn dispatch_pipeline_cache() -> DispatchPipelineCache {
    DispatchPipelineCache::custom(DawnPipelineCache)
}

fn dispatch_blas() -> DispatchBlas {
    DispatchBlas::custom(DawnBlas)
}

fn dispatch_tlas() -> DispatchTlas {
    DispatchTlas::custom(DawnTlas)
}

fn expect_adapter(adapter: &DispatchAdapter) -> Adapter {
    adapter
        .as_custom::<DawnAdapter>()
        .expect("wgpu-compat: adapter not dawn")
        .inner
        .clone()
}

fn expect_device(device: &DispatchDevice) -> Device {
    device
        .as_custom::<DawnDevice>()
        .expect("wgpu-compat: device not dawn")
        .inner
        .clone()
}

fn expect_queue(queue: &DispatchQueue) -> Queue {
    queue
        .as_custom::<DawnQueue>()
        .expect("wgpu-compat: queue not dawn")
        .inner
        .clone()
}

fn expect_surface(surface: &DispatchSurface) -> DawnSurface {
    surface
        .as_custom::<DawnSurface>()
        .expect("wgpu-compat: surface not dawn")
        .clone()
}

fn expect_shader_module(module: &DispatchShaderModule) -> ShaderModule {
    module
        .as_custom::<DawnShaderModule>()
        .expect("wgpu-compat: shader module not dawn")
        .inner
        .clone()
}

fn expect_bind_group_layout(layout: &DispatchBindGroupLayout) -> BindGroupLayout {
    layout
        .as_custom::<DawnBindGroupLayout>()
        .expect("wgpu-compat: bind group layout not dawn")
        .inner
        .clone()
}

fn expect_bind_group(group: &DispatchBindGroup) -> BindGroup {
    group
        .as_custom::<DawnBindGroup>()
        .expect("wgpu-compat: bind group not dawn")
        .inner
        .clone()
}

fn expect_texture_view(view: &DispatchTextureView) -> TextureView {
    view.as_custom::<DawnTextureView>()
        .expect("wgpu-compat: texture view not dawn")
        .inner
        .clone()
}

fn expect_sampler(sampler: &DispatchSampler) -> Sampler {
    sampler
        .as_custom::<DawnSampler>()
        .expect("wgpu-compat: sampler not dawn")
        .inner
        .clone()
}

fn expect_buffer(buffer: &DispatchBuffer) -> Buffer {
    buffer
        .as_custom::<DawnBuffer>()
        .expect("wgpu-compat: buffer not dawn")
        .inner
        .clone()
}

fn expect_texture(texture: &DispatchTexture) -> Texture {
    texture
        .as_custom::<DawnTexture>()
        .expect("wgpu-compat: texture not dawn")
        .inner
        .clone()
}

fn expect_external_texture(texture: &DispatchExternalTexture) -> ExternalTexture {
    texture
        .as_custom::<DawnExternalTexture>()
        .expect("wgpu-compat: external texture not dawn")
        .inner
        .clone()
}

fn expect_query_set(query_set: &DispatchQuerySet) -> QuerySet {
    query_set
        .as_custom::<DawnQuerySet>()
        .expect("wgpu-compat: query set not dawn")
        .inner
        .clone()
}

fn expect_pipeline_layout(layout: &DispatchPipelineLayout) -> PipelineLayout {
    layout
        .as_custom::<DawnPipelineLayout>()
        .expect("wgpu-compat: pipeline layout not dawn")
        .inner
        .clone()
}

fn expect_render_pipeline(pipeline: &DispatchRenderPipeline) -> RenderPipeline {
    pipeline
        .as_custom::<DawnRenderPipeline>()
        .expect("wgpu-compat: render pipeline not dawn")
        .inner
        .clone()
}

fn expect_compute_pipeline(pipeline: &DispatchComputePipeline) -> ComputePipeline {
    pipeline
        .as_custom::<DawnComputePipeline>()
        .expect("wgpu-compat: compute pipeline not dawn")
        .inner
        .clone()
}

fn expect_command_encoder(encoder: &DispatchCommandEncoder) -> CommandEncoder {
    encoder
        .as_custom::<DawnCommandEncoder>()
        .expect("wgpu-compat: command encoder not dawn")
        .inner
        .clone()
}

fn expect_command_buffer(buffer: &DispatchCommandBuffer) -> CommandBuffer {
    buffer
        .as_custom::<DawnCommandBuffer>()
        .expect("wgpu-compat: command buffer not dawn")
        .inner
        .clone()
}

fn expect_render_bundle(bundle: &DispatchRenderBundle) -> RenderBundle {
    bundle
        .as_custom::<DawnRenderBundle>()
        .expect("wgpu-compat: render bundle not dawn")
        .inner
        .clone()
}

fn expect_surface_output_detail(detail: &DispatchSurfaceOutputDetail) -> Surface {
    detail
        .as_custom::<DawnSurfaceOutputDetail>()
        .expect("wgpu-compat: surface output detail not dawn")
        .surface
        .clone()
}

fn expect_device_from_api(device: &wgpu::Device) -> Device {
    device
        .as_custom::<DawnDevice>()
        .expect("wgpu-compat: device not dawn")
        .inner
        .clone()
}

fn expect_surface_from_api(surface: &wgpu::Surface) -> DawnSurface {
    surface
        .as_custom::<DawnSurface>()
        .expect("wgpu-compat: surface not dawn")
        .clone()
}

fn expect_buffer_from_api(buffer: &wgpu::Buffer) -> Buffer {
    buffer
        .as_custom::<DawnBuffer>()
        .expect("wgpu-compat: buffer not dawn")
        .inner
        .clone()
}

fn expect_texture_from_api(texture: &wgpu::Texture) -> Texture {
    texture
        .as_custom::<DawnTexture>()
        .expect("wgpu-compat: texture not dawn")
        .inner
        .clone()
}

fn expect_texture_view_from_api(view: &wgpu::TextureView) -> TextureView {
    view.as_custom::<DawnTextureView>()
        .expect("wgpu-compat: texture view not dawn")
        .inner
        .clone()
}

fn expect_sampler_from_api(sampler: &wgpu::Sampler) -> Sampler {
    sampler
        .as_custom::<DawnSampler>()
        .expect("wgpu-compat: sampler not dawn")
        .inner
        .clone()
}

fn expect_bind_group_layout_from_api(layout: &wgpu::BindGroupLayout) -> BindGroupLayout {
    layout
        .as_custom::<DawnBindGroupLayout>()
        .expect("wgpu-compat: bind group layout not dawn")
        .inner
        .clone()
}

fn expect_pipeline_layout_from_api(layout: &wgpu::PipelineLayout) -> PipelineLayout {
    layout
        .as_custom::<DawnPipelineLayout>()
        .expect("wgpu-compat: pipeline layout not dawn")
        .inner
        .clone()
}

fn expect_shader_module_from_api(module: &wgpu::ShaderModule) -> ShaderModule {
    module
        .as_custom::<DawnShaderModule>()
        .expect("wgpu-compat: shader module not dawn")
        .inner
        .clone()
}

fn expect_bind_group_from_api(group: &wgpu::BindGroup) -> BindGroup {
    group
        .as_custom::<DawnBindGroup>()
        .expect("wgpu-compat: bind group not dawn")
        .inner
        .clone()
}

fn expect_query_set_from_api(query_set: &wgpu::QuerySet) -> QuerySet {
    query_set
        .as_custom::<DawnQuerySet>()
        .expect("wgpu-compat: query set not dawn")
        .inner
        .clone()
}

impl InstanceInterface for DawnInstance {
    fn new(_desc: &wgpu::InstanceDescriptor) -> Self {
        let mut desc = InstanceDescriptor::new();
        desc.required_features = Some(vec![InstanceFeatureName::TimedWaitAny]);
        let instance = Instance::new(Some(&desc));
        Self { inner: instance }
    }

    unsafe fn create_surface(
        &self,
        target: wgpu::SurfaceTargetUnsafe,
    ) -> Result<DispatchSurface, wgpu::CreateSurfaceError> {
        match target {
            #[cfg(target_os = "macos")]
            wgpu::SurfaceTargetUnsafe::CoreAnimationLayer(layer) => {
                let mut desc = SurfaceDescriptor::new();
                let source = SurfaceSourceMetalLayer { layer: Some(layer) };
                desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                let surface = self.inner.create_surface(&desc);
                let dawn_surface = DawnSurface {
                    inner: surface,
                    metal_layer: None,
                };
                Ok(dispatch_surface(dawn_surface))
            }
            #[cfg(target_os = "macos")]
            wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_window_handle, ..
            } => {
                use wgpu::rwh::RawWindowHandle;
                match raw_window_handle {
                    RawWindowHandle::AppKit(handle) => {
                        let layer =
                            unsafe { raw_window_metal::Layer::from_ns_view(handle.ns_view) };
                        let layer_ptr = layer.into_raw();
                        let mut desc = SurfaceDescriptor::new();
                        let source = SurfaceSourceMetalLayer {
                            layer: Some(layer_ptr.as_ptr().cast()),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.create_surface(&desc);
                        let handle = MetalLayerHandle {
                            ptr: layer_ptr.as_ptr().cast(),
                        };
                        let dawn_surface = DawnSurface {
                            inner: surface,
                            metal_layer: Some(Arc::new(handle)),
                        };
                        Ok(dispatch_surface(dawn_surface))
                    }
                    _ => panic!("wgpu-compat: unsupported raw window handle on macOS"),
                }
            }
            #[cfg(target_os = "windows")]
            wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_window_handle, ..
            } => {
                use wgpu::rwh::RawWindowHandle;
                match raw_window_handle {
                    RawWindowHandle::Win32(handle) => {
                        let mut desc = SurfaceDescriptor::new();
                        let source = SurfaceSourceWindowsHWND {
                            hinstance: Some(handle.hinstance.cast()),
                            hwnd: Some(handle.hwnd.cast()),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.create_surface(&desc);
                        let dawn_surface = DawnSurface { inner: surface };
                        Ok(dispatch_surface(dawn_surface))
                    }
                    _ => panic!("wgpu-compat: unsupported raw window handle on Windows"),
                }
            }
            #[cfg(all(unix, not(target_vendor = "apple")))]
            wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle,
                raw_window_handle,
            } => {
                use wgpu::rwh::{RawDisplayHandle, RawWindowHandle};
                match (raw_display_handle, raw_window_handle) {
                    (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
                        let mut desc = SurfaceDescriptor::new();
                        let source = SurfaceSourceWaylandSurface {
                            display: Some(display.display.cast()),
                            surface: Some(window.surface.cast()),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.create_surface(&desc);
                        let dawn_surface = DawnSurface { inner: surface };
                        Ok(dispatch_surface(dawn_surface))
                    }
                    (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
                        let mut desc = SurfaceDescriptor::new();
                        let source = SurfaceSourceXlibWindow {
                            display: Some(display.display.cast()),
                            window: Some(window.window as u64),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.create_surface(&desc);
                        let dawn_surface = DawnSurface { inner: surface };
                        Ok(dispatch_surface(dawn_surface))
                    }
                    (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                        let mut desc = SurfaceDescriptor::new();
                        let source = SurfaceSourceXCBWindow {
                            connection: Some(display.connection.cast()),
                            window: Some(window.window),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.create_surface(&desc);
                        let dawn_surface = DawnSurface { inner: surface };
                        Ok(dispatch_surface(dawn_surface))
                    }
                    _ => panic!("wgpu-compat: unsupported raw window handle on unix"),
                }
            }
            _ => panic!("wgpu-compat: unsupported surface target"),
        }
    }

    fn request_adapter(
        &self,
        options: &wgpu::RequestAdapterOptions<'_, '_>,
    ) -> Pin<Box<dyn wgpu::custom::RequestAdapterFuture>> {
        let (future, shared) = CallbackFuture::new();
        let mut dawn_options = RequestAdapterOptions::new();
        dawn_options.power_preference = Some(map_power_preference(options.power_preference));
        dawn_options.force_fallback_adapter = Some(options.force_fallback_adapter);
        if let Some(surface) = options.compatible_surface {
            dawn_options.compatible_surface = Some(expect_surface_from_api(surface).inner);
        }
        let future_handle =
            self.inner
                .request_adapter(Some(&dawn_options), move |status, adapter, _message| {
                    if status == RequestAdapterStatus::Success {
                        let adapter = adapter.expect("wgpu-compat: missing adapter");
                        complete_shared(&shared, Ok(dispatch_adapter(adapter)));
                    } else {
                        complete_shared(
                            &shared,
                            Err(wgpu::RequestAdapterError::NotFound {
                                active_backends: wgpu::Backends::empty(),
                                requested_backends: wgpu::Backends::empty(),
                                supported_backends: wgpu::Backends::empty(),
                                no_fallback_backends: wgpu::Backends::empty(),
                                no_adapter_backends: wgpu::Backends::empty(),
                                incompatible_surface_backends: wgpu::Backends::empty(),
                            }),
                        );
                    }
                });
        let _ = self.inner.wait_any(
            Some(&mut [FutureWaitInfo {
                future: Some(future_handle),
                completed: None,
            }]),
            0,
        );
        Box::pin(future)
    }

    fn poll_all_devices(&self, _force_wait: bool) -> bool {
        self.inner.process_events();
        true
    }

    fn wgsl_language_features(&self) -> wgpu::WgslLanguageFeatures {
        let mut features = SupportedWGSLLanguageFeatures::new();
        self.inner.get_wgsl_language_features(&mut features);
        let mut out = wgpu::WgslLanguageFeatures::empty();
        if let Some(list) = features.features.as_ref() {
            for feature in list {
                if *feature == WGSLLanguageFeatureName::ReadonlyAndReadwriteStorageTextures {
                    out |= wgpu::WgslLanguageFeatures::ReadOnlyAndReadWriteStorageTextures;
                }
            }
        }
        out
    }

    fn enumerate_adapters(
        &self,
        _backends: wgpu::Backends,
    ) -> Pin<Box<dyn wgpu::custom::EnumerateAdapterFuture>> {
        let (future, shared) = CallbackFuture::new();
        complete_shared(&shared, Vec::new());
        Box::pin(future)
    }
}

impl AdapterInterface for DawnAdapter {
    fn request_device(
        &self,
        desc: &wgpu::DeviceDescriptor<'_>,
    ) -> Pin<Box<dyn wgpu::custom::RequestDeviceFuture>> {
        let (future, shared) = CallbackFuture::new();
        let mut dawn_desc = DeviceDescriptor::new();
        dawn_desc.label = label_to_string(desc.label);
        if !desc.required_features.is_empty() {
            dawn_desc.required_features = Some(map_features_to_dawn(desc.required_features));
        }
        if desc.required_limits != wgpu::Limits::default() {
            dawn_desc.required_limits = Some(map_limits_to_dawn(&desc.required_limits));
        }
        let error_info = dawn_rs::UncapturedErrorCallbackInfo::new();
        error_info
            .callback
            .replace(Some(Box::new(|_devices, ty, message| {
                eprintln!("Uncaptured error {:?}: {}", ty, message);
            })));
        dawn_desc.uncaptured_error_callback_info = Some(error_info);
        let instance = self.inner.get_instance();
        let future_handle =
            self.inner
                .request_device(Some(&dawn_desc), move |status, device, _message| {
                    if status == RequestDeviceStatus::Success {
                        let device = device.expect("wgpu-compat: missing device");
                        let queue = device.get_queue();
                        complete_shared(
                            &shared,
                            Ok((dispatch_device(device), dispatch_queue(queue))),
                        );
                    } else {
                        panic!("wgpu-compat: request_device failed");
                    }
                });
        let _ = instance.wait_any(
            Some(&mut [FutureWaitInfo {
                future: Some(future_handle),
                completed: None,
            }]),
            0,
        );
        Box::pin(future)
    }

    fn is_surface_supported(&self, surface: &DispatchSurface) -> bool {
        surface.as_custom::<DawnSurface>().is_some()
    }

    fn features(&self) -> wgpu::Features {
        let mut features = SupportedFeatures::new();
        self.inner.get_features(&mut features);
        map_features_to_wgpu(&features)
    }

    fn limits(&self) -> wgpu::Limits {
        let mut limits = Limits::new();
        let _ = self.inner.get_limits(&mut limits);
        map_limits_to_wgpu(&limits)
    }

    fn downlevel_capabilities(&self) -> wgpu::DownlevelCapabilities {
        wgpu::DownlevelCapabilities::default()
    }

    fn get_info(&self) -> wgpu::AdapterInfo {
        let mut info = AdapterInfo::new();
        let _ = self.inner.get_info(&mut info);
        wgpu::AdapterInfo {
            name: info.description.clone().unwrap_or_default(),
            vendor: info.vendor_id.unwrap_or(0),
            device: info.device_id.unwrap_or(0),
            device_type: match info.adapter_type.unwrap_or(AdapterType::Unknown) {
                AdapterType::DiscreteGpu => wgpu::DeviceType::DiscreteGpu,
                AdapterType::IntegratedGpu => wgpu::DeviceType::IntegratedGpu,
                AdapterType::Cpu => wgpu::DeviceType::Cpu,
                AdapterType::Unknown => wgpu::DeviceType::Other,
            },
            backend: map_backend_type_to_wgpu(info.backend_type.unwrap_or(BackendType::Undefined)),
            driver: info.architecture.clone().unwrap_or_default(),
            driver_info: info.device.clone().unwrap_or_default(),
            device_pci_bus_id: String::new(),
            subgroup_min_size: wgpu::MINIMUM_SUBGROUP_MIN_SIZE,
            subgroup_max_size: wgpu::MAXIMUM_SUBGROUP_MAX_SIZE,
            transient_saves_memory: false,
        }
    }

    fn get_texture_format_features(
        &self,
        _format: wgpu::TextureFormat,
    ) -> wgpu::TextureFormatFeatures {
        wgpu::TextureFormatFeatures {
            allowed_usages: wgpu::TextureUsages::empty(),
            flags: wgpu::TextureFormatFeatureFlags::empty(),
        }
    }

    fn get_presentation_timestamp(&self) -> wgpu::PresentationTimestamp {
        wgpu::PresentationTimestamp::INVALID_TIMESTAMP
    }
}

impl DeviceInterface for DawnDevice {
    fn features(&self) -> wgpu::Features {
        let adapter = self.inner.get_adapter();
        DawnAdapter { inner: adapter }.features()
    }

    fn limits(&self) -> wgpu::Limits {
        let adapter = self.inner.get_adapter();
        DawnAdapter { inner: adapter }.limits()
    }

    fn create_shader_module(
        &self,
        desc: wgpu::ShaderModuleDescriptor<'_>,
        _shader_bound_checks: wgpu::ShaderRuntimeChecks,
    ) -> DispatchShaderModule {
        let dawn_desc = map_shader_module_descriptor(desc);
        let module = self.inner.create_shader_module(&dawn_desc);
        dispatch_shader_module(module)
    }

    unsafe fn create_shader_module_passthrough(
        &self,
        _desc: &wgpu::ShaderModuleDescriptorPassthrough<'_>,
    ) -> DispatchShaderModule {
        panic!("wgpu-compat: create_shader_module_passthrough not supported");
    }

    fn create_bind_group_layout(
        &self,
        desc: &wgpu::BindGroupLayoutDescriptor<'_>,
    ) -> DispatchBindGroupLayout {
        let dawn_desc = map_bind_group_layout_descriptor(desc);
        let layout = self.inner.create_bind_group_layout(&dawn_desc);
        dispatch_bind_group_layout(layout)
    }

    fn create_bind_group(&self, desc: &wgpu::BindGroupDescriptor<'_>) -> DispatchBindGroup {
        let dawn_desc = map_bind_group_descriptor(desc);
        let group = self.inner.create_bind_group(&dawn_desc);
        dispatch_bind_group(group)
    }

    fn create_pipeline_layout(
        &self,
        desc: &wgpu::PipelineLayoutDescriptor<'_>,
    ) -> DispatchPipelineLayout {
        let dawn_desc = map_pipeline_layout_descriptor(desc);
        let layout = self.inner.create_pipeline_layout(&dawn_desc);
        dispatch_pipeline_layout(layout)
    }

    fn create_render_pipeline(
        &self,
        desc: &wgpu::RenderPipelineDescriptor<'_>,
    ) -> DispatchRenderPipeline {
        let dawn_desc = map_render_pipeline_descriptor(desc);
        let pipeline = self.inner.create_render_pipeline(&dawn_desc);
        dispatch_render_pipeline(pipeline)
    }

    fn create_mesh_pipeline(
        &self,
        _desc: &wgpu::MeshPipelineDescriptor<'_>,
    ) -> DispatchRenderPipeline {
        panic!("wgpu-compat: mesh pipelines not supported");
    }

    fn create_compute_pipeline(
        &self,
        desc: &wgpu::ComputePipelineDescriptor<'_>,
    ) -> DispatchComputePipeline {
        let dawn_desc = map_compute_pipeline_descriptor(desc);
        let pipeline = self.inner.create_compute_pipeline(&dawn_desc);
        dispatch_compute_pipeline(pipeline)
    }

    unsafe fn create_pipeline_cache(
        &self,
        _desc: &wgpu::PipelineCacheDescriptor<'_>,
    ) -> DispatchPipelineCache {
        dispatch_pipeline_cache()
    }

    fn create_buffer(&self, desc: &wgpu::BufferDescriptor<'_>) -> DispatchBuffer {
        let dawn_desc = map_buffer_descriptor(desc);
        let buffer = self
            .inner
            .create_buffer(&dawn_desc)
            .expect("wgpu-compat: create_buffer returned null");
        dispatch_buffer(buffer)
    }

    fn create_texture(&self, desc: &wgpu::TextureDescriptor<'_>) -> DispatchTexture {
        let dawn_desc = map_texture_descriptor(desc);
        let texture = self.inner.create_texture(&dawn_desc);
        dispatch_texture(texture)
    }

    fn create_external_texture(
        &self,
        desc: &wgpu::ExternalTextureDescriptor<'_>,
        _planes: &[&wgpu::TextureView],
    ) -> DispatchExternalTexture {
        let mut dawn_desc = ExternalTextureDescriptor::new();
        dawn_desc.label = label_to_string(desc.label);
        let texture = self.inner.create_external_texture(&dawn_desc);
        dispatch_external_texture(texture)
    }

    fn create_blas(
        &self,
        _desc: &wgpu::CreateBlasDescriptor<'_>,
        _sizes: wgpu::BlasGeometrySizeDescriptors,
    ) -> (Option<u64>, DispatchBlas) {
        (None, dispatch_blas())
    }

    fn create_tlas(&self, _desc: &wgpu::CreateTlasDescriptor<'_>) -> DispatchTlas {
        dispatch_tlas()
    }

    fn create_sampler(&self, desc: &wgpu::SamplerDescriptor<'_>) -> DispatchSampler {
        let dawn_desc = map_sampler_descriptor(desc);
        let sampler = self.inner.create_sampler(Some(&dawn_desc));
        dispatch_sampler(sampler)
    }

    fn create_query_set(&self, desc: &wgpu::QuerySetDescriptor<'_>) -> DispatchQuerySet {
        let ty = match desc.ty {
            wgpu::QueryType::Occlusion => QueryType::Occlusion,
            wgpu::QueryType::Timestamp => QueryType::Timestamp,
            _ => panic!("wgpu-compat: query type not supported"),
        };
        let mut dawn_desc = QuerySetDescriptor::new();
        dawn_desc.label = label_to_string(desc.label);
        dawn_desc.r#type = Some(ty);
        dawn_desc.count = Some(desc.count);
        let set = self.inner.create_query_set(&dawn_desc);
        dispatch_query_set(set)
    }

    fn create_command_encoder(
        &self,
        desc: &wgpu::CommandEncoderDescriptor<'_>,
    ) -> DispatchCommandEncoder {
        let dawn_desc = map_command_encoder_descriptor(desc);
        let encoder = self.inner.create_command_encoder(Some(&dawn_desc));
        dispatch_command_encoder(encoder)
    }

    fn create_render_bundle_encoder(
        &self,
        desc: &wgpu::RenderBundleEncoderDescriptor<'_>,
    ) -> DispatchRenderBundleEncoder {
        let dawn_desc = map_render_bundle_encoder_descriptor(desc);
        let encoder = self.inner.create_render_bundle_encoder(&dawn_desc);
        dispatch_render_bundle_encoder(encoder)
    }

    fn set_device_lost_callback(&self, _device_lost_callback: wgpu::custom::BoxDeviceLostCallback) {
    }

    fn on_uncaptured_error(&self, _handler: Arc<dyn wgpu::UncapturedErrorHandler>) {}

    fn push_error_scope(&self, filter: wgpu::ErrorFilter) -> u32 {
        let filter = match filter {
            wgpu::ErrorFilter::Validation => ErrorFilter::Validation,
            wgpu::ErrorFilter::OutOfMemory => ErrorFilter::OutOfMemory,
            wgpu::ErrorFilter::Internal => ErrorFilter::Internal,
        };
        self.inner.push_error_scope(filter);
        0
    }

    fn pop_error_scope(&self, _index: u32) -> Pin<Box<dyn wgpu::custom::PopErrorScopeFuture>> {
        let (future, shared) = CallbackFuture::new();
        let _ = self.inner.pop_error_scope(move |status, ty, message| {
            if status == PopErrorScopeStatus::Success {
                if ty == ErrorType::NoError {
                    complete_shared(&shared, None);
                } else {
                    complete_shared(&shared, Some(map_uncaptured_error(ty, message)));
                }
            } else {
                complete_shared(
                    &shared,
                    Some(wgpu::Error::Internal {
                        source: Box::new(DawnError("pop_error_scope failed".to_string())),
                        description: "pop_error_scope failed".to_string(),
                    }),
                );
            }
        });
        Box::pin(future)
    }

    unsafe fn start_graphics_debugger_capture(&self) {
        let _ = &self.inner;
    }

    unsafe fn stop_graphics_debugger_capture(&self) {
        let _ = &self.inner;
    }

    fn poll(&self, _poll_type: wgt::PollType<u64>) -> Result<wgpu::PollStatus, wgpu::PollError> {
        self.inner.tick();
        Ok(wgpu::PollStatus::QueueEmpty)
    }

    fn get_internal_counters(&self) -> wgpu::InternalCounters {
        wgpu::InternalCounters::default()
    }

    fn generate_allocator_report(&self) -> Option<wgpu::AllocatorReport> {
        None
    }

    fn destroy(&self) {
        self.inner.destroy();
    }
}

impl QueueInterface for DawnQueue {
    fn write_buffer(&self, buffer: &DispatchBuffer, offset: wgpu::BufferAddress, data: &[u8]) {
        let buffer = expect_buffer(buffer);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data.len()) };
        self.inner.write_buffer(buffer, offset, data_slice);
    }

    fn create_staging_buffer(&self, size: wgpu::BufferSize) -> Option<DispatchQueueWriteBuffer> {
        Some(dispatch_queue_write_buffer(vec![0; size.get() as usize]))
    }

    fn validate_write_buffer(
        &self,
        _buffer: &DispatchBuffer,
        _offset: wgpu::BufferAddress,
        _size: wgpu::BufferSize,
    ) -> Option<()> {
        Some(())
    }

    fn write_staging_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: wgpu::BufferAddress,
        staging_buffer: &DispatchQueueWriteBuffer,
    ) {
        let buffer = expect_buffer(buffer);
        let staging = staging_buffer
            .as_custom::<DawnQueueWriteBuffer>()
            .expect("wgpu-compat: queue write buffer not dawn");
        let data_ptr = staging.inner.as_ptr().cast::<std::ffi::c_void>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, staging.inner.len()) };
        self.inner.write_buffer(buffer, offset, data_slice);
    }

    fn write_texture(
        &self,
        texture: wgpu::TexelCopyTextureInfo<'_>,
        data: &[u8],
        data_layout: wgpu::TexelCopyBufferLayout,
        size: wgpu::Extent3d,
    ) {
        let destination = map_texel_copy_texture_info(texture);
        let data_layout = map_texel_copy_buffer_layout(data_layout);
        let write_size = map_extent_3d(size);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data.len()) };
        self.inner
            .write_texture(&destination, data_slice, &data_layout, &write_size);
    }

    #[cfg(web)]
    #[allow(unexpected_cfgs)]
    fn copy_external_image_to_texture(
        &self,
        _source: &wgpu::CopyExternalImageSourceInfo,
        _dest: wgpu::CopyExternalImageDestInfo<&wgpu::Texture>,
        _size: wgpu::Extent3d,
    ) {
        unimplemented!();
    }

    fn submit(&self, command_buffers: &mut dyn Iterator<Item = DispatchCommandBuffer>) -> u64 {
        let buffers = command_buffers
            .map(|buffer| expect_command_buffer(&buffer))
            .collect::<Vec<_>>();
        self.inner.submit(&buffers);
        0
    }

    fn get_timestamp_period(&self) -> f32 {
        1.0
    }

    fn on_submitted_work_done(&self, callback: wgpu::custom::BoxSubmittedWorkDoneCallback) {
        let mut callback = Some(callback);
        let _ = self.inner.on_submitted_work_done(move |status, _message| {
            let _ = status;
            if let Some(cb) = callback.take() {
                cb();
            }
        });
    }

    fn compact_blas(&self, _blas: &DispatchBlas) -> (Option<u64>, DispatchBlas) {
        (None, dispatch_blas())
    }
}

impl ShaderModuleInterface for DawnShaderModule {
    fn get_compilation_info(&self) -> Pin<Box<dyn wgpu::custom::ShaderCompilationInfoFuture>> {
        let (future, shared) = CallbackFuture::new();
        let _ = self.inner.get_compilation_info(move |status, info| {
            if status == CompilationInfoRequestStatus::Success {
                complete_shared(&shared, map_compilation_info(info));
            } else {
                complete_shared(&shared, wgpu::CompilationInfo { messages: vec![] });
            }
        });
        Box::pin(future)
    }
}

impl BindGroupLayoutInterface for DawnBindGroupLayout {}
impl BindGroupInterface for DawnBindGroup {}
impl TextureViewInterface for DawnTextureView {}
impl SamplerInterface for DawnSampler {}

impl BufferInterface for DawnBuffer {
    fn map_async(
        &self,
        mode: wgpu::MapMode,
        range: std::ops::Range<wgpu::BufferAddress>,
        callback: wgpu::custom::BufferMapCallback,
    ) {
        let mode = match mode {
            wgpu::MapMode::Read => MapMode::READ,
            wgpu::MapMode::Write => MapMode::WRITE,
        };
        let mut callback = Some(callback);
        let _ = self.inner.map_async(
            mode,
            range.start as usize,
            (range.end - range.start) as usize,
            move |status, message| {
                let result = match status {
                    MapAsyncStatus::Success => Ok(()),
                    _ => {
                        let _ = message;
                        Err(wgpu::BufferAsyncError)
                    }
                };
                if let Some(cb) = callback.take() {
                    cb(result);
                }
            },
        );
    }

    fn get_mapped_range(
        &self,
        sub_range: std::ops::Range<wgpu::BufferAddress>,
    ) -> DispatchBufferMappedRange {
        let ptr = self.inner.get_mapped_range(
            sub_range.start as usize,
            (sub_range.end - sub_range.start) as usize,
        );
        dispatch_buffer_mapped_range(ptr.cast(), (sub_range.end - sub_range.start) as usize)
    }

    fn unmap(&self) {
        self.inner.unmap();
    }

    fn destroy(&self) {
        self.inner.destroy();
    }
}

impl TextureInterface for DawnTexture {
    fn create_view(&self, desc: &wgpu::TextureViewDescriptor<'_>) -> DispatchTextureView {
        let desc = map_texture_view_descriptor(desc);
        let view = self.inner.create_view(Some(&desc));
        dispatch_texture_view(view)
    }

    fn destroy(&self) {
        self.inner.destroy();
    }
}

impl ExternalTextureInterface for DawnExternalTexture {
    fn destroy(&self) {
        self.inner.destroy();
    }
}

impl BlasInterface for DawnBlas {
    fn prepare_compact_async(&self, _callback: wgpu::custom::BlasCompactCallback) {
        panic!("wgpu-compat: blas not supported");
    }

    fn ready_for_compaction(&self) -> bool {
        false
    }
}

impl TlasInterface for DawnTlas {}
impl QuerySetInterface for DawnQuerySet {}
impl PipelineLayoutInterface for DawnPipelineLayout {}

impl RenderPipelineInterface for DawnRenderPipeline {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout {
        let layout = self.inner.get_bind_group_layout(index);
        dispatch_bind_group_layout(layout)
    }
}

impl ComputePipelineInterface for DawnComputePipeline {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout {
        let layout = self.inner.get_bind_group_layout(index);
        dispatch_bind_group_layout(layout)
    }
}

impl PipelineCacheInterface for DawnPipelineCache {
    fn get_data(&self) -> Option<Vec<u8>> {
        None
    }
}

impl CommandEncoderInterface for DawnCommandEncoder {
    fn copy_buffer_to_buffer(
        &self,
        source: &DispatchBuffer,
        source_offset: wgpu::BufferAddress,
        destination: &DispatchBuffer,
        destination_offset: wgpu::BufferAddress,
        copy_size: Option<wgpu::BufferAddress>,
    ) {
        let source = expect_buffer(source);
        let destination = expect_buffer(destination);
        self.inner.copy_buffer_to_buffer(
            source,
            source_offset,
            destination,
            destination_offset,
            copy_size.unwrap_or(WHOLE_SIZE),
        );
    }

    fn copy_buffer_to_texture(
        &self,
        source: wgpu::TexelCopyBufferInfo<'_>,
        destination: wgpu::TexelCopyTextureInfo<'_>,
        copy_size: wgpu::Extent3d,
    ) {
        let source = map_texel_copy_buffer_info(source);
        let dest = map_texel_copy_texture_info(destination);
        let size = map_extent_3d(copy_size);
        self.inner.copy_buffer_to_texture(&source, &dest, &size);
    }

    fn copy_texture_to_buffer(
        &self,
        source: wgpu::TexelCopyTextureInfo<'_>,
        destination: wgpu::TexelCopyBufferInfo<'_>,
        copy_size: wgpu::Extent3d,
    ) {
        let source = map_texel_copy_texture_info(source);
        let dest = map_texel_copy_buffer_info(destination);
        let size = map_extent_3d(copy_size);
        self.inner.copy_texture_to_buffer(&source, &dest, &size);
    }

    fn copy_texture_to_texture(
        &self,
        source: wgpu::TexelCopyTextureInfo<'_>,
        destination: wgpu::TexelCopyTextureInfo<'_>,
        copy_size: wgpu::Extent3d,
    ) {
        let source = map_texel_copy_texture_info(source);
        let dest = map_texel_copy_texture_info(destination);
        let size = map_extent_3d(copy_size);
        self.inner.copy_texture_to_texture(&source, &dest, &size);
    }

    fn begin_compute_pass(&self, desc: &wgpu::ComputePassDescriptor<'_>) -> DispatchComputePass {
        let dawn_desc = map_compute_pass_descriptor(desc);
        let pass = self.inner.begin_compute_pass(Some(&dawn_desc));
        dispatch_compute_pass(pass)
    }

    fn begin_render_pass(&self, desc: &wgpu::RenderPassDescriptor<'_>) -> DispatchRenderPass {
        let dawn_desc = map_render_pass_descriptor(desc);
        let pass = self.inner.begin_render_pass(&dawn_desc);
        dispatch_render_pass(pass)
    }

    fn finish(&mut self) -> DispatchCommandBuffer {
        let buffer = self.inner.finish(None);
        dispatch_command_buffer(buffer)
    }

    fn clear_texture(
        &self,
        texture: &DispatchTexture,
        subresource_range: &wgpu::ImageSubresourceRange,
    ) {
        let _ = texture;
        let _ = subresource_range;
    }

    fn clear_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: wgpu::BufferAddress,
        size: Option<wgpu::BufferAddress>,
    ) {
        let buffer = expect_buffer(buffer);
        self.inner
            .clear_buffer(buffer, offset, size.unwrap_or(WHOLE_SIZE));
    }

    fn insert_debug_marker(&self, label: &str) {
        self.inner.insert_debug_marker(label.to_string());
    }

    fn push_debug_group(&self, label: &str) {
        self.inner.push_debug_group(label.to_string());
    }

    fn pop_debug_group(&self) {
        self.inner.pop_debug_group();
    }

    fn write_timestamp(&self, query_set: &DispatchQuerySet, query_index: u32) {
        let set = expect_query_set(query_set);
        self.inner.write_timestamp(set, query_index);
    }

    fn resolve_query_set(
        &self,
        query_set: &DispatchQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &DispatchBuffer,
        destination_offset: wgpu::BufferAddress,
    ) {
        let set = expect_query_set(query_set);
        let buffer = expect_buffer(destination);
        self.inner
            .resolve_query_set(set, first_query, query_count, buffer, destination_offset);
    }

    fn mark_acceleration_structures_built<'a>(
        &self,
        _blas: &mut dyn Iterator<Item = &'a wgpu::Blas>,
        _tlas: &mut dyn Iterator<Item = &'a wgpu::Tlas>,
    ) {
        panic!("wgpu-compat: blas/tlas not supported");
    }

    fn build_acceleration_structures<'a>(
        &self,
        _blas: &mut dyn Iterator<Item = &'a wgpu::BlasBuildEntry<'a>>,
        _tlas: &mut dyn Iterator<Item = &'a wgpu::Tlas>,
    ) {
        panic!("wgpu-compat: blas/tlas not supported");
    }

    fn transition_resources<'a>(
        &mut self,
        _buffer_transitions: &mut dyn Iterator<Item = wgpu::BufferTransition<&'a DispatchBuffer>>,
        _texture_transitions: &mut dyn Iterator<
            Item = wgpu::TextureTransition<&'a DispatchTexture>,
        >,
    ) {
    }
}

impl ComputePassInterface for DawnComputePass {
    fn set_pipeline(&mut self, pipeline: &DispatchComputePipeline) {
        let pipeline = expect_compute_pipeline(pipeline);
        self.inner.set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[wgpu::DynamicOffset],
    ) {
        let group = bind_group.map(expect_bind_group);
        self.inner.set_bind_group(index, group, offsets);
    }

    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        let data = bytes_to_u32(data);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_len = data.len() * std::mem::size_of::<u32>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
        self.inner.set_immediates(offset, data_slice);
    }

    fn insert_debug_marker(&mut self, label: &str) {
        self.inner.insert_debug_marker(label.to_string());
    }

    fn push_debug_group(&mut self, group_label: &str) {
        self.inner.push_debug_group(group_label.to_string());
    }

    fn pop_debug_group(&mut self) {
        self.inner.pop_debug_group();
    }

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        let set = expect_query_set(query_set);
        self.inner.write_timestamp(set, query_index);
    }

    fn begin_pipeline_statistics_query(
        &mut self,
        _query_set: &DispatchQuerySet,
        _query_index: u32,
    ) {
        panic!("wgpu-compat: pipeline statistics not supported");
    }

    fn end_pipeline_statistics_query(&mut self) {
        panic!("wgpu-compat: pipeline statistics not supported");
    }

    fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
        self.inner.dispatch_workgroups(x, y, z);
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner
            .dispatch_workgroups_indirect(buffer, indirect_offset);
    }

    fn end(&mut self) {
        if !self.ended {
            self.inner.end();
            self.ended = true;
        }
    }
}

impl Drop for DawnComputePass {
    fn drop(&mut self) {
        if !self.ended {
            self.inner.end();
            self.ended = true;
        }
    }
}

impl RenderPassInterface for DawnRenderPass {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline) {
        let pipeline = expect_render_pipeline(pipeline);
        self.inner.set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[wgpu::DynamicOffset],
    ) {
        let group = bind_group.map(expect_bind_group);
        self.inner.set_bind_group(index, group, offsets);
    }

    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: wgpu::IndexFormat,
        offset: wgpu::BufferAddress,
        size: Option<wgpu::BufferSize>,
    ) {
        let buffer = expect_buffer(buffer);
        let size = size.map(|v| v.get()).unwrap_or(WHOLE_SIZE);
        self.inner
            .set_index_buffer(buffer, map_index_format(index_format), offset, size);
    }

    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: wgpu::BufferAddress,
        size: Option<wgpu::BufferSize>,
    ) {
        let buffer = expect_buffer(buffer);
        let size = size.map(|v| v.get()).unwrap_or(WHOLE_SIZE);
        self.inner
            .set_vertex_buffer(slot, Some(buffer), offset, size);
    }

    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        let data = bytes_to_u32(data);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_len = data.len() * std::mem::size_of::<u32>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
        self.inner.set_immediates(offset, data_slice);
    }

    fn set_blend_constant(&mut self, color: wgpu::Color) {
        let color = map_color(color);
        self.inner.set_blend_constant(&color);
    }

    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.inner.set_scissor_rect(x, y, width, height);
    }

    fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        self.inner
            .set_viewport(x, y, width, height, min_depth, max_depth);
    }

    fn set_stencil_reference(&mut self, reference: u32) {
        self.inner.set_stencil_reference(reference);
    }

    fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        self.inner.draw(
            vertices.end - vertices.start,
            instances.end - instances.start,
            vertices.start,
            instances.start,
        );
    }

    fn draw_indexed(
        &mut self,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        self.inner.draw_indexed(
            indices.end - indices.start,
            instances.end - instances.start,
            indices.start,
            base_vertex,
            instances.start,
        );
    }

    fn draw_mesh_tasks(&mut self, _group_count_x: u32, _group_count_y: u32, _group_count_z: u32) {
        panic!("wgpu-compat: mesh tasks not supported");
    }

    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner.draw_indirect(buffer, indirect_offset);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner.draw_indexed_indirect(buffer, indirect_offset);
    }

    fn draw_mesh_tasks_indirect(
        &mut self,
        _indirect_buffer: &DispatchBuffer,
        _indirect_offset: wgpu::BufferAddress,
    ) {
        panic!("wgpu-compat: mesh tasks not supported");
    }

    fn multi_draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
        count: u32,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner
            .multi_draw_indirect(buffer, indirect_offset, count, None, 0);
    }

    fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
        count: u32,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner
            .multi_draw_indexed_indirect(buffer, indirect_offset, count, None, 0);
    }

    fn multi_draw_indirect_count(
        &mut self,
        _indirect_buffer: &DispatchBuffer,
        _indirect_offset: wgpu::BufferAddress,
        _count_buffer: &DispatchBuffer,
        _count_buffer_offset: wgpu::BufferAddress,
        _max_count: u32,
    ) {
        panic!("wgpu-compat: multi_draw_indirect_count not supported");
    }

    fn multi_draw_mesh_tasks_indirect(
        &mut self,
        _indirect_buffer: &DispatchBuffer,
        _indirect_offset: wgpu::BufferAddress,
        _count: u32,
    ) {
        panic!("wgpu-compat: mesh tasks not supported");
    }

    fn multi_draw_indexed_indirect_count(
        &mut self,
        _indirect_buffer: &DispatchBuffer,
        _indirect_offset: wgpu::BufferAddress,
        _count_buffer: &DispatchBuffer,
        _count_buffer_offset: wgpu::BufferAddress,
        _max_count: u32,
    ) {
        panic!("wgpu-compat: multi_draw_indexed_indirect_count not supported");
    }

    fn multi_draw_mesh_tasks_indirect_count(
        &mut self,
        _indirect_buffer: &DispatchBuffer,
        _indirect_offset: wgpu::BufferAddress,
        _count_buffer: &DispatchBuffer,
        _count_buffer_offset: wgpu::BufferAddress,
        _max_count: u32,
    ) {
        panic!("wgpu-compat: mesh tasks not supported");
    }

    fn insert_debug_marker(&mut self, label: &str) {
        self.inner.insert_debug_marker(label.to_string());
    }

    fn push_debug_group(&mut self, group_label: &str) {
        self.inner.push_debug_group(group_label.to_string());
    }

    fn pop_debug_group(&mut self) {
        self.inner.pop_debug_group();
    }

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        let set = expect_query_set(query_set);
        self.inner.write_timestamp(set, query_index);
    }

    fn begin_occlusion_query(&mut self, query_index: u32) {
        self.inner.begin_occlusion_query(query_index);
    }

    fn end_occlusion_query(&mut self) {
        self.inner.end_occlusion_query();
    }

    fn begin_pipeline_statistics_query(
        &mut self,
        _query_set: &DispatchQuerySet,
        _query_index: u32,
    ) {
        panic!("wgpu-compat: pipeline statistics not supported");
    }

    fn end_pipeline_statistics_query(&mut self) {
        panic!("wgpu-compat: pipeline statistics not supported");
    }

    fn execute_bundles(&mut self, render_bundles: &mut dyn Iterator<Item = &DispatchRenderBundle>) {
        let bundles = render_bundles.map(expect_render_bundle).collect::<Vec<_>>();
        self.inner.execute_bundles(&bundles);
    }

    fn end(&mut self) {
        if !self.ended {
            self.inner.end();
            self.ended = true;
        }
    }
}

impl Drop for DawnRenderPass {
    fn drop(&mut self) {
        if !self.ended {
            self.inner.end();
            self.ended = true;
        }
    }
}

impl RenderBundleEncoderInterface for DawnRenderBundleEncoder {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline) {
        let pipeline = expect_render_pipeline(pipeline);
        self.inner.set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[wgpu::DynamicOffset],
    ) {
        let group = bind_group.map(expect_bind_group);
        self.inner.set_bind_group(index, group, offsets);
    }

    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: wgpu::IndexFormat,
        offset: wgpu::BufferAddress,
        size: Option<wgpu::BufferSize>,
    ) {
        let buffer = expect_buffer(buffer);
        let size = size.map(|v| v.get()).unwrap_or(WHOLE_SIZE);
        self.inner
            .set_index_buffer(buffer, map_index_format(index_format), offset, size);
    }

    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: wgpu::BufferAddress,
        size: Option<wgpu::BufferSize>,
    ) {
        let buffer = expect_buffer(buffer);
        let size = size.map(|v| v.get()).unwrap_or(WHOLE_SIZE);
        self.inner
            .set_vertex_buffer(slot, Some(buffer), offset, size);
    }

    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        let data = bytes_to_u32(data);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_len = data.len() * std::mem::size_of::<u32>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
        self.inner.set_immediates(offset, data_slice);
    }

    fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        self.inner.draw(
            vertices.end - vertices.start,
            instances.end - instances.start,
            vertices.start,
            instances.start,
        );
    }

    fn draw_indexed(
        &mut self,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        self.inner.draw_indexed(
            indices.end - indices.start,
            instances.end - instances.start,
            indices.start,
            base_vertex,
            instances.start,
        );
    }

    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner.draw_indirect(buffer, indirect_offset);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner.draw_indexed_indirect(buffer, indirect_offset);
    }

    fn finish(self, desc: &wgpu::RenderBundleDescriptor<'_>) -> DispatchRenderBundle {
        let mut dawn_desc = RenderBundleDescriptor::new();
        dawn_desc.label = label_to_string(desc.label);
        let bundle = self.inner.finish(Some(&dawn_desc));
        dispatch_render_bundle(bundle)
    }
}

impl CommandBufferInterface for DawnCommandBuffer {}
impl RenderBundleInterface for DawnRenderBundle {}

impl SurfaceInterface for DawnSurface {
    fn get_capabilities(&self, adapter: &DispatchAdapter) -> wgpu::SurfaceCapabilities {
        let adapter = expect_adapter(adapter);
        let mut caps = SurfaceCapabilities::new();
        let _ = self.inner.get_capabilities(adapter, &mut caps);
        map_surface_capabilities(caps)
    }

    fn configure(&self, device: &DispatchDevice, config: &wgpu::SurfaceConfiguration) {
        let mut config = map_surface_configuration(config);
        config.device = Some(expect_device(device));
        self.inner.configure(&config);
    }

    fn get_current_texture(
        &self,
    ) -> (
        Option<DispatchTexture>,
        wgpu::SurfaceStatus,
        DispatchSurfaceOutputDetail,
    ) {
        let mut surface_texture = SurfaceTexture::new();
        self.inner.get_current_texture(&mut surface_texture);
        let status = match surface_texture
            .status
            .unwrap_or(SurfaceGetCurrentTextureStatus::Error)
        {
            SurfaceGetCurrentTextureStatus::SuccessOptimal => wgpu::SurfaceStatus::Good,
            SurfaceGetCurrentTextureStatus::SuccessSuboptimal => wgpu::SurfaceStatus::Suboptimal,
            SurfaceGetCurrentTextureStatus::Timeout => wgpu::SurfaceStatus::Timeout,
            SurfaceGetCurrentTextureStatus::Outdated => wgpu::SurfaceStatus::Outdated,
            SurfaceGetCurrentTextureStatus::Lost => wgpu::SurfaceStatus::Lost,
            SurfaceGetCurrentTextureStatus::Error => wgpu::SurfaceStatus::Unknown,
        };
        (
            surface_texture.texture.map(dispatch_texture),
            status,
            dispatch_surface_output_detail(self.inner.clone()),
        )
    }
}

impl SurfaceOutputDetailInterface for DawnSurfaceOutputDetail {
    fn present(&self) {
        let _ = self.surface.present();
    }

    fn texture_discard(&self) {
        // Dawn does not expose an explicit surface texture discard API.
    }
}

impl QueueWriteBufferInterface for DawnQueueWriteBuffer {
    fn slice(&self) -> &[u8] {
        &self.inner
    }

    fn slice_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl BufferMappedRangeInterface for DawnBufferMappedRange {
    fn slice(&self) -> &[u8] {
        if self.data.is_null() || self.size == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.data, self.size) }
    }

    fn slice_mut(&mut self) -> &mut [u8] {
        if self.data.is_null() || self.size == 0 {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.data, self.size) }
    }

    #[cfg(web)]
    #[allow(unexpected_cfgs)]
    fn as_uint8array(&self) -> &js_sys::Uint8Array {
        unimplemented!();
    }
}

pub fn to_wgpu_instance(instance: Instance) -> wgpu::Instance {
    wgpu::Instance::from_custom(DawnInstance { inner: instance })
}

pub fn from_wgpu_instance(instance: &wgpu::Instance) -> Result<Instance, WgpuCompatError> {
    instance
        .as_custom::<DawnInstance>()
        .map(|i| i.inner.clone())
        .ok_or(WgpuCompatError::NotDawnBackend)
}

pub fn to_wgpu_adapter(adapter: Adapter) -> wgpu::Adapter {
    wgpu::Adapter::from_custom(DawnAdapter { inner: adapter })
}

pub fn from_wgpu_adapter(adapter: &wgpu::Adapter) -> Result<Adapter, WgpuCompatError> {
    adapter
        .as_custom::<DawnAdapter>()
        .map(|a| a.inner.clone())
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
        .map(|s| s.inner.clone())
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
        .map(|e| e.inner.clone())
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
