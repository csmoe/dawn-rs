use crate::types::*;
use dawn_rs::*;
use wgpu::custom::*;

macro_rules! impl_dispatch {
    ($fn_name:ident, $dawn_type:ty, $wrapper:ident, $dispatch_type:ident) => {
        pub(crate) fn $fn_name(inner: $dawn_type) -> $dispatch_type {
            $dispatch_type::custom($wrapper { inner })
        }
    };
}

impl_dispatch!(dispatch_queue, Queue, DawnQueue, DispatchQueue);
pub(crate) fn dispatch_adapter(adapter: DawnAdapter) -> DispatchAdapter {
    DispatchAdapter::custom(adapter)
}
impl_dispatch!(
    dispatch_shader_module,
    ShaderModule,
    DawnShaderModule,
    DispatchShaderModule
);
impl_dispatch!(
    dispatch_bind_group_layout,
    BindGroupLayout,
    DawnBindGroupLayout,
    DispatchBindGroupLayout
);
impl_dispatch!(
    dispatch_bind_group,
    BindGroup,
    DawnBindGroup,
    DispatchBindGroup
);
impl_dispatch!(
    dispatch_texture_view,
    TextureView,
    DawnTextureView,
    DispatchTextureView
);
impl_dispatch!(dispatch_sampler, Sampler, DawnSampler, DispatchSampler);
impl_dispatch!(dispatch_buffer, Buffer, DawnBuffer, DispatchBuffer);
impl_dispatch!(dispatch_texture, Texture, DawnTexture, DispatchTexture);
impl_dispatch!(
    dispatch_external_texture,
    ExternalTexture,
    DawnExternalTexture,
    DispatchExternalTexture
);
impl_dispatch!(dispatch_query_set, QuerySet, DawnQuerySet, DispatchQuerySet);
impl_dispatch!(
    dispatch_pipeline_layout,
    PipelineLayout,
    DawnPipelineLayout,
    DispatchPipelineLayout
);
impl_dispatch!(
    dispatch_render_pipeline,
    RenderPipeline,
    DawnRenderPipeline,
    DispatchRenderPipeline
);
impl_dispatch!(
    dispatch_compute_pipeline,
    ComputePipeline,
    DawnComputePipeline,
    DispatchComputePipeline
);
impl_dispatch!(
    dispatch_command_encoder,
    CommandEncoder,
    DawnCommandEncoder,
    DispatchCommandEncoder
);
impl_dispatch!(
    dispatch_command_buffer,
    CommandBuffer,
    DawnCommandBuffer,
    DispatchCommandBuffer
);
impl_dispatch!(
    dispatch_render_bundle,
    RenderBundle,
    DawnRenderBundle,
    DispatchRenderBundle
);

pub(crate) fn dispatch_device(device: Device) -> DispatchDevice {
    DispatchDevice::custom(DawnDevice::new(device))
}

pub(crate) fn dispatch_device_with_callback_state(
    device: Device,
    device_lost_callback: std::sync::Arc<
        std::sync::Mutex<Option<wgpu::custom::BoxDeviceLostCallback>>,
    >,
    uncaptured_error_handler: std::sync::Arc<
        std::sync::Mutex<Option<std::sync::Arc<dyn wgpu::UncapturedErrorHandler>>>,
    >,
) -> DispatchDevice {
    DispatchDevice::custom(DawnDevice::with_callback_state(
        device,
        device_lost_callback,
        uncaptured_error_handler,
    ))
}

pub(crate) fn dispatch_surface(surface: DawnSurface) -> DispatchSurface {
    DispatchSurface::custom(surface)
}

pub(crate) fn dispatch_compute_pass(pass: ComputePassEncoder) -> DispatchComputePass {
    DispatchComputePass::custom(DawnComputePass {
        inner: pass,
        ended: false,
    })
}

pub(crate) fn dispatch_render_pass(pass: RenderPassEncoder) -> DispatchRenderPass {
    DispatchRenderPass::custom(DawnRenderPass {
        inner: pass,
        ended: false,
    })
}

pub(crate) fn dispatch_render_bundle_encoder(
    encoder: RenderBundleEncoder,
) -> DispatchRenderBundleEncoder {
    DispatchRenderBundleEncoder::custom(DawnRenderBundleEncoder { inner: encoder })
}

pub(crate) fn dispatch_surface_output_detail(surface: Surface) -> DispatchSurfaceOutputDetail {
    DispatchSurfaceOutputDetail::custom(DawnSurfaceOutputDetail { surface })
}

pub(crate) fn dispatch_queue_write_buffer(data: Vec<u8>) -> DispatchQueueWriteBuffer {
    DispatchQueueWriteBuffer::custom(DawnQueueWriteBuffer { inner: data })
}

pub(crate) fn dispatch_buffer_mapped_range(ptr: *mut u8, size: usize) -> DispatchBufferMappedRange {
    DispatchBufferMappedRange::custom(DawnBufferMappedRange { data: ptr, size })
}

pub(crate) fn dispatch_pipeline_cache() -> DispatchPipelineCache {
    DispatchPipelineCache::custom(DawnPipelineCache)
}

pub(crate) fn dispatch_blas() -> DispatchBlas {
    DispatchBlas::custom(DawnBlas)
}

pub(crate) fn dispatch_tlas() -> DispatchTlas {
    DispatchTlas::custom(DawnTlas)
}

macro_rules! impl_expect {
    ($fn_name:ident, $dispatch_type:ty, $wrapper:ident, $inner_type:ty, $msg:literal) => {
        pub(crate) fn $fn_name(v: &$dispatch_type) -> $inner_type {
            v.as_custom::<$wrapper>().expect($msg).inner.clone()
        }
    };
}

impl_expect!(
    expect_device,
    DispatchDevice,
    DawnDevice,
    Device,
    "wgpu-compat: device not dawn"
);
impl_expect!(
    expect_queue,
    DispatchQueue,
    DawnQueue,
    Queue,
    "wgpu-compat: queue not dawn"
);
impl_expect!(
    expect_shader_module,
    DispatchShaderModule,
    DawnShaderModule,
    ShaderModule,
    "wgpu-compat: shader module not dawn"
);
impl_expect!(
    expect_bind_group_layout,
    DispatchBindGroupLayout,
    DawnBindGroupLayout,
    BindGroupLayout,
    "wgpu-compat: bind group layout not dawn"
);
impl_expect!(
    expect_bind_group,
    DispatchBindGroup,
    DawnBindGroup,
    BindGroup,
    "wgpu-compat: bind group not dawn"
);
impl_expect!(
    expect_texture_view,
    DispatchTextureView,
    DawnTextureView,
    TextureView,
    "wgpu-compat: texture view not dawn"
);
impl_expect!(
    expect_sampler,
    DispatchSampler,
    DawnSampler,
    Sampler,
    "wgpu-compat: sampler not dawn"
);
impl_expect!(
    expect_buffer,
    DispatchBuffer,
    DawnBuffer,
    Buffer,
    "wgpu-compat: buffer not dawn"
);
impl_expect!(
    expect_texture,
    DispatchTexture,
    DawnTexture,
    Texture,
    "wgpu-compat: texture not dawn"
);
impl_expect!(
    expect_external_texture,
    DispatchExternalTexture,
    DawnExternalTexture,
    ExternalTexture,
    "wgpu-compat: external texture not dawn"
);
impl_expect!(
    expect_query_set,
    DispatchQuerySet,
    DawnQuerySet,
    QuerySet,
    "wgpu-compat: query set not dawn"
);
impl_expect!(
    expect_pipeline_layout,
    DispatchPipelineLayout,
    DawnPipelineLayout,
    PipelineLayout,
    "wgpu-compat: pipeline layout not dawn"
);
impl_expect!(
    expect_render_pipeline,
    DispatchRenderPipeline,
    DawnRenderPipeline,
    RenderPipeline,
    "wgpu-compat: render pipeline not dawn"
);
impl_expect!(
    expect_compute_pipeline,
    DispatchComputePipeline,
    DawnComputePipeline,
    ComputePipeline,
    "wgpu-compat: compute pipeline not dawn"
);
impl_expect!(
    expect_command_encoder,
    DispatchCommandEncoder,
    DawnCommandEncoder,
    CommandEncoder,
    "wgpu-compat: command encoder not dawn"
);
impl_expect!(
    expect_command_buffer,
    DispatchCommandBuffer,
    DawnCommandBuffer,
    CommandBuffer,
    "wgpu-compat: command buffer not dawn"
);
impl_expect!(
    expect_render_bundle,
    DispatchRenderBundle,
    DawnRenderBundle,
    RenderBundle,
    "wgpu-compat: render bundle not dawn"
);

pub(crate) fn expect_surface(surface: &DispatchSurface) -> DawnSurface {
    surface
        .as_custom::<DawnSurface>()
        .expect("wgpu-compat: surface not dawn")
        .clone()
}

pub(crate) fn expect_surface_output_detail(detail: &DispatchSurfaceOutputDetail) -> Surface {
    detail
        .as_custom::<DawnSurfaceOutputDetail>()
        .expect("wgpu-compat: surface output detail not dawn")
        .surface
        .clone()
}

macro_rules! impl_expect_from_api {
    ($fn_name:ident, $api_type:ty, $wrapper:ident, $inner_type:ty, $msg:literal) => {
        pub(crate) fn $fn_name(v: &$api_type) -> $inner_type {
            v.as_custom::<$wrapper>().expect($msg).inner.clone()
        }
    };
}

impl_expect_from_api!(
    expect_device_from_api,
    wgpu::Device,
    DawnDevice,
    Device,
    "wgpu-compat: device not dawn"
);
impl_expect_from_api!(
    expect_buffer_from_api,
    wgpu::Buffer,
    DawnBuffer,
    Buffer,
    "wgpu-compat: buffer not dawn"
);
impl_expect_from_api!(
    expect_texture_from_api,
    wgpu::Texture,
    DawnTexture,
    Texture,
    "wgpu-compat: texture not dawn"
);
impl_expect_from_api!(
    expect_texture_view_from_api,
    wgpu::TextureView,
    DawnTextureView,
    TextureView,
    "wgpu-compat: texture view not dawn"
);
impl_expect_from_api!(
    expect_sampler_from_api,
    wgpu::Sampler,
    DawnSampler,
    Sampler,
    "wgpu-compat: sampler not dawn"
);
impl_expect_from_api!(
    expect_bind_group_layout_from_api,
    wgpu::BindGroupLayout,
    DawnBindGroupLayout,
    BindGroupLayout,
    "wgpu-compat: bind group layout not dawn"
);
impl_expect_from_api!(
    expect_pipeline_layout_from_api,
    wgpu::PipelineLayout,
    DawnPipelineLayout,
    PipelineLayout,
    "wgpu-compat: pipeline layout not dawn"
);
impl_expect_from_api!(
    expect_shader_module_from_api,
    wgpu::ShaderModule,
    DawnShaderModule,
    ShaderModule,
    "wgpu-compat: shader module not dawn"
);
impl_expect_from_api!(
    expect_bind_group_from_api,
    wgpu::BindGroup,
    DawnBindGroup,
    BindGroup,
    "wgpu-compat: bind group not dawn"
);
impl_expect_from_api!(
    expect_query_set_from_api,
    wgpu::QuerySet,
    DawnQuerySet,
    QuerySet,
    "wgpu-compat: query set not dawn"
);

pub(crate) fn expect_surface_from_api(surface: &wgpu::Surface) -> DawnSurface {
    surface
        .as_custom::<DawnSurface>()
        .expect("wgpu-compat: surface not dawn")
        .clone()
}
