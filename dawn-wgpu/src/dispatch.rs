use crate::types::*;
use dawn_rs::*;
use wgpu::custom::*;

pub(crate) fn dispatch_adapter(adapter: Adapter) -> DispatchAdapter {
    DispatchAdapter::custom(DawnAdapter {
        inner: SendSync::new(adapter),
    })
}

pub(crate) fn dispatch_device(device: Device) -> DispatchDevice {
    DispatchDevice::custom(DawnDevice { inner: device })
}

pub(crate) fn dispatch_queue(queue: Queue) -> DispatchQueue {
    DispatchQueue::custom(DawnQueue { inner: queue })
}

pub(crate) fn dispatch_surface(surface: DawnSurface) -> DispatchSurface {
    DispatchSurface::custom(surface)
}

pub(crate) fn dispatch_shader_module(module: ShaderModule) -> DispatchShaderModule {
    DispatchShaderModule::custom(DawnShaderModule { inner: module })
}

pub(crate) fn dispatch_bind_group_layout(layout: BindGroupLayout) -> DispatchBindGroupLayout {
    DispatchBindGroupLayout::custom(DawnBindGroupLayout { inner: layout })
}

pub(crate) fn dispatch_bind_group(group: BindGroup) -> DispatchBindGroup {
    DispatchBindGroup::custom(DawnBindGroup { inner: group })
}

pub(crate) fn dispatch_texture_view(view: TextureView) -> DispatchTextureView {
    DispatchTextureView::custom(DawnTextureView { inner: view })
}

pub(crate) fn dispatch_sampler(sampler: Sampler) -> DispatchSampler {
    DispatchSampler::custom(DawnSampler { inner: sampler })
}

pub(crate) fn dispatch_buffer(buffer: Buffer) -> DispatchBuffer {
    DispatchBuffer::custom(DawnBuffer { inner: buffer })
}

pub(crate) fn dispatch_texture(texture: Texture) -> DispatchTexture {
    DispatchTexture::custom(DawnTexture { inner: texture })
}

pub(crate) fn dispatch_external_texture(texture: ExternalTexture) -> DispatchExternalTexture {
    DispatchExternalTexture::custom(DawnExternalTexture { inner: texture })
}

pub(crate) fn dispatch_query_set(query_set: QuerySet) -> DispatchQuerySet {
    DispatchQuerySet::custom(DawnQuerySet { inner: query_set })
}

pub(crate) fn dispatch_pipeline_layout(layout: PipelineLayout) -> DispatchPipelineLayout {
    DispatchPipelineLayout::custom(DawnPipelineLayout { inner: layout })
}

pub(crate) fn dispatch_render_pipeline(pipeline: RenderPipeline) -> DispatchRenderPipeline {
    DispatchRenderPipeline::custom(DawnRenderPipeline { inner: pipeline })
}

pub(crate) fn dispatch_compute_pipeline(pipeline: ComputePipeline) -> DispatchComputePipeline {
    DispatchComputePipeline::custom(DawnComputePipeline { inner: pipeline })
}

pub(crate) fn dispatch_command_encoder(encoder: CommandEncoder) -> DispatchCommandEncoder {
    DispatchCommandEncoder::custom(DawnCommandEncoder {
        inner: SendSync::new(encoder),
    })
}

pub(crate) fn dispatch_command_buffer(buffer: CommandBuffer) -> DispatchCommandBuffer {
    DispatchCommandBuffer::custom(DawnCommandBuffer { inner: buffer })
}

pub(crate) fn dispatch_compute_pass(pass: ComputePassEncoder) -> DispatchComputePass {
    DispatchComputePass::custom(DawnComputePass {
        inner: SendSync::new(pass),
        ended: false,
    })
}

pub(crate) fn dispatch_render_pass(pass: RenderPassEncoder) -> DispatchRenderPass {
    DispatchRenderPass::custom(DawnRenderPass {
        inner: SendSync::new(pass),
        ended: false,
    })
}

pub(crate) fn dispatch_render_bundle_encoder(
    encoder: RenderBundleEncoder,
) -> DispatchRenderBundleEncoder {
    DispatchRenderBundleEncoder::custom(DawnRenderBundleEncoder {
        inner: SendSync::new(encoder),
    })
}

pub(crate) fn dispatch_render_bundle(bundle: RenderBundle) -> DispatchRenderBundle {
    DispatchRenderBundle::custom(DawnRenderBundle { inner: bundle })
}

pub(crate) fn dispatch_surface_output_detail(surface: Surface) -> DispatchSurfaceOutputDetail {
    DispatchSurfaceOutputDetail::custom(DawnSurfaceOutputDetail {
        surface: SendSync::new(surface),
    })
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

pub(crate) fn expect_adapter(adapter: &DispatchAdapter) -> Adapter {
    adapter
        .as_custom::<DawnAdapter>()
        .expect("wgpu-compat: adapter not dawn")
        .inner
        .get()
}

pub(crate) fn expect_device(device: &DispatchDevice) -> Device {
    device
        .as_custom::<DawnDevice>()
        .expect("wgpu-compat: device not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_queue(queue: &DispatchQueue) -> Queue {
    queue
        .as_custom::<DawnQueue>()
        .expect("wgpu-compat: queue not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_surface(surface: &DispatchSurface) -> DawnSurface {
    surface
        .as_custom::<DawnSurface>()
        .expect("wgpu-compat: surface not dawn")
        .clone()
}

pub(crate) fn expect_shader_module(module: &DispatchShaderModule) -> ShaderModule {
    module
        .as_custom::<DawnShaderModule>()
        .expect("wgpu-compat: shader module not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_bind_group_layout(layout: &DispatchBindGroupLayout) -> BindGroupLayout {
    layout
        .as_custom::<DawnBindGroupLayout>()
        .expect("wgpu-compat: bind group layout not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_bind_group(group: &DispatchBindGroup) -> BindGroup {
    group
        .as_custom::<DawnBindGroup>()
        .expect("wgpu-compat: bind group not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_texture_view(view: &DispatchTextureView) -> TextureView {
    view.as_custom::<DawnTextureView>()
        .expect("wgpu-compat: texture view not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_sampler(sampler: &DispatchSampler) -> Sampler {
    sampler
        .as_custom::<DawnSampler>()
        .expect("wgpu-compat: sampler not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_buffer(buffer: &DispatchBuffer) -> Buffer {
    buffer
        .as_custom::<DawnBuffer>()
        .expect("wgpu-compat: buffer not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_texture(texture: &DispatchTexture) -> Texture {
    texture
        .as_custom::<DawnTexture>()
        .expect("wgpu-compat: texture not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_external_texture(texture: &DispatchExternalTexture) -> ExternalTexture {
    texture
        .as_custom::<DawnExternalTexture>()
        .expect("wgpu-compat: external texture not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_query_set(query_set: &DispatchQuerySet) -> QuerySet {
    query_set
        .as_custom::<DawnQuerySet>()
        .expect("wgpu-compat: query set not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_pipeline_layout(layout: &DispatchPipelineLayout) -> PipelineLayout {
    layout
        .as_custom::<DawnPipelineLayout>()
        .expect("wgpu-compat: pipeline layout not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_render_pipeline(pipeline: &DispatchRenderPipeline) -> RenderPipeline {
    pipeline
        .as_custom::<DawnRenderPipeline>()
        .expect("wgpu-compat: render pipeline not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_compute_pipeline(pipeline: &DispatchComputePipeline) -> ComputePipeline {
    pipeline
        .as_custom::<DawnComputePipeline>()
        .expect("wgpu-compat: compute pipeline not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_command_encoder(encoder: &DispatchCommandEncoder) -> CommandEncoder {
    encoder
        .as_custom::<DawnCommandEncoder>()
        .expect("wgpu-compat: command encoder not dawn")
        .inner
        .get()
}

pub(crate) fn expect_command_buffer(buffer: &DispatchCommandBuffer) -> CommandBuffer {
    buffer
        .as_custom::<DawnCommandBuffer>()
        .expect("wgpu-compat: command buffer not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_render_bundle(bundle: &DispatchRenderBundle) -> RenderBundle {
    bundle
        .as_custom::<DawnRenderBundle>()
        .expect("wgpu-compat: render bundle not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_surface_output_detail(detail: &DispatchSurfaceOutputDetail) -> Surface {
    detail
        .as_custom::<DawnSurfaceOutputDetail>()
        .expect("wgpu-compat: surface output detail not dawn")
        .surface
        .get()
}

pub(crate) fn expect_device_from_api(device: &wgpu::Device) -> Device {
    device
        .as_custom::<DawnDevice>()
        .expect("wgpu-compat: device not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_surface_from_api(surface: &wgpu::Surface) -> DawnSurface {
    surface
        .as_custom::<DawnSurface>()
        .expect("wgpu-compat: surface not dawn")
        .clone()
}

pub(crate) fn expect_buffer_from_api(buffer: &wgpu::Buffer) -> Buffer {
    buffer
        .as_custom::<DawnBuffer>()
        .expect("wgpu-compat: buffer not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_texture_from_api(texture: &wgpu::Texture) -> Texture {
    texture
        .as_custom::<DawnTexture>()
        .expect("wgpu-compat: texture not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_texture_view_from_api(view: &wgpu::TextureView) -> TextureView {
    view.as_custom::<DawnTextureView>()
        .expect("wgpu-compat: texture view not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_sampler_from_api(sampler: &wgpu::Sampler) -> Sampler {
    sampler
        .as_custom::<DawnSampler>()
        .expect("wgpu-compat: sampler not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_bind_group_layout_from_api(layout: &wgpu::BindGroupLayout) -> BindGroupLayout {
    layout
        .as_custom::<DawnBindGroupLayout>()
        .expect("wgpu-compat: bind group layout not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_pipeline_layout_from_api(layout: &wgpu::PipelineLayout) -> PipelineLayout {
    layout
        .as_custom::<DawnPipelineLayout>()
        .expect("wgpu-compat: pipeline layout not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_shader_module_from_api(module: &wgpu::ShaderModule) -> ShaderModule {
    module
        .as_custom::<DawnShaderModule>()
        .expect("wgpu-compat: shader module not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_bind_group_from_api(group: &wgpu::BindGroup) -> BindGroup {
    group
        .as_custom::<DawnBindGroup>()
        .expect("wgpu-compat: bind group not dawn")
        .inner
        .clone()
}

pub(crate) fn expect_query_set_from_api(query_set: &wgpu::QuerySet) -> QuerySet {
    query_set
        .as_custom::<DawnQuerySet>()
        .expect("wgpu-compat: query set not dawn")
        .inner
        .clone()
}
