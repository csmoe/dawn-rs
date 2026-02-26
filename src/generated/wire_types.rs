#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireDirection {
    ClientToServer,
    ServerToClient,
    Special,
}
#[derive(Debug, Clone, PartialEq)]
pub enum WireValue {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    Bool(bool),
    F32(f32),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Null,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WireField {
    pub name: &'static str,
    pub value: WireValue,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WirePacket {
    pub command: WireCommand,
    pub fields: Vec<WireField>,
}
#[derive(Debug, Clone, Copy)]
pub struct WireCommandMeta {
    pub name: &'static str,
    pub direction: WireDirection,
    pub fields: &'static [&'static str],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WireCommand {
    ChunkedCommand,
    AdapterRequestDevice,
    BindGroupLayoutSetLabel,
    BindGroupSetLabel,
    BufferCreateTexelView,
    BufferMapAsync,
    BufferSetLabel,
    BufferUpdateMappedData,
    CommandBufferSetLabel,
    CommandEncoderBeginComputePass,
    CommandEncoderBeginRenderPass,
    CommandEncoderClearBuffer,
    CommandEncoderCopyBufferToBuffer,
    CommandEncoderCopyBufferToTexture,
    CommandEncoderCopyTextureToBuffer,
    CommandEncoderCopyTextureToTexture,
    CommandEncoderFinish,
    CommandEncoderInjectValidationError,
    CommandEncoderInsertDebugMarker,
    CommandEncoderPopDebugGroup,
    CommandEncoderPushDebugGroup,
    CommandEncoderResolveQuerySet,
    CommandEncoderSetLabel,
    CommandEncoderWriteBuffer,
    CommandEncoderWriteTimestamp,
    ComputePassEncoderDispatchWorkgroups,
    ComputePassEncoderDispatchWorkgroupsIndirect,
    ComputePassEncoderEnd,
    ComputePassEncoderInsertDebugMarker,
    ComputePassEncoderPopDebugGroup,
    ComputePassEncoderPushDebugGroup,
    ComputePassEncoderSetBindGroup,
    ComputePassEncoderSetImmediates,
    ComputePassEncoderSetLabel,
    ComputePassEncoderSetPipeline,
    ComputePassEncoderSetResourceTable,
    ComputePassEncoderWriteTimestamp,
    ComputePipelineGetBindGroupLayout,
    ComputePipelineSetLabel,
    DeviceCreateBindGroup,
    DeviceCreateBindGroupLayout,
    DeviceCreateBuffer,
    DeviceCreateCommandEncoder,
    DeviceCreateComputePipeline,
    DeviceCreateComputePipelineAsync,
    DeviceCreateErrorExternalTexture,
    DeviceCreateErrorShaderModule,
    DeviceCreateExternalTexture,
    DeviceCreatePipelineLayout,
    DeviceCreateQuerySet,
    DeviceCreateRenderBundleEncoder,
    DeviceCreateRenderPipeline,
    DeviceCreateRenderPipelineAsync,
    DeviceCreateSampler,
    DeviceCreateShaderModule,
    DeviceForceLoss,
    DeviceGetAHardwareBufferProperties,
    DeviceImportSharedBufferMemory,
    DeviceImportSharedFence,
    DeviceImportSharedTextureMemory,
    DevicePopErrorScope,
    DevicePushErrorScope,
    DeviceSetLabel,
    DeviceTick,
    DeviceValidateTextureDescriptor,
    ExternalTextureDestroy,
    ExternalTextureExpire,
    ExternalTextureRefresh,
    ExternalTextureSetLabel,
    InstanceRequestAdapter,
    PipelineLayoutSetLabel,
    QuerySetDestroy,
    QuerySetSetLabel,
    QueueCopyExternalTextureForBrowser,
    QueueCopyTextureForBrowser,
    QueueOnSubmittedWorkDone,
    QueueSetLabel,
    QueueWriteBuffer,
    QueueWriteBufferXl,
    QueueWriteTexture,
    QueueWriteTextureXl,
    RenderBundleEncoderDraw,
    RenderBundleEncoderDrawIndexed,
    RenderBundleEncoderDrawIndexedIndirect,
    RenderBundleEncoderDrawIndirect,
    RenderBundleEncoderFinish,
    RenderBundleEncoderInsertDebugMarker,
    RenderBundleEncoderPopDebugGroup,
    RenderBundleEncoderPushDebugGroup,
    RenderBundleEncoderSetBindGroup,
    RenderBundleEncoderSetImmediates,
    RenderBundleEncoderSetIndexBuffer,
    RenderBundleEncoderSetLabel,
    RenderBundleEncoderSetPipeline,
    RenderBundleEncoderSetResourceTable,
    RenderBundleEncoderSetVertexBuffer,
    RenderBundleSetLabel,
    RenderPassEncoderBeginOcclusionQuery,
    RenderPassEncoderDraw,
    RenderPassEncoderDrawIndexed,
    RenderPassEncoderDrawIndexedIndirect,
    RenderPassEncoderDrawIndirect,
    RenderPassEncoderEnd,
    RenderPassEncoderEndOcclusionQuery,
    RenderPassEncoderExecuteBundles,
    RenderPassEncoderInsertDebugMarker,
    RenderPassEncoderMultiDrawIndexedIndirect,
    RenderPassEncoderMultiDrawIndirect,
    RenderPassEncoderPixelLocalStorageBarrier,
    RenderPassEncoderPopDebugGroup,
    RenderPassEncoderPushDebugGroup,
    RenderPassEncoderSetBindGroup,
    RenderPassEncoderSetBlendConstant,
    RenderPassEncoderSetImmediates,
    RenderPassEncoderSetIndexBuffer,
    RenderPassEncoderSetLabel,
    RenderPassEncoderSetPipeline,
    RenderPassEncoderSetResourceTable,
    RenderPassEncoderSetScissorRect,
    RenderPassEncoderSetStencilReference,
    RenderPassEncoderSetVertexBuffer,
    RenderPassEncoderSetViewport,
    RenderPassEncoderWriteTimestamp,
    RenderPipelineGetBindGroupLayout,
    RenderPipelineSetLabel,
    SamplerSetLabel,
    ShaderModuleGetCompilationInfo,
    ShaderModuleSetLabel,
    SharedBufferMemoryBeginAccess,
    SharedBufferMemoryCreateBuffer,
    SharedBufferMemoryEndAccess,
    SharedBufferMemoryGetProperties,
    SharedBufferMemorySetLabel,
    SharedFenceExportInfo,
    SharedTextureMemoryBeginAccess,
    SharedTextureMemoryCreateTexture,
    SharedTextureMemoryEndAccess,
    SharedTextureMemoryGetProperties,
    SharedTextureMemorySetLabel,
    SurfaceGetCurrentTexture,
    SurfaceSetLabel,
    TexelBufferViewSetLabel,
    TextureCreateErrorView,
    TextureCreateView,
    TextureDestroy,
    TexturePin,
    TextureSetLabel,
    TextureSetOwnershipForMemoryDump,
    TextureUnpin,
    TextureViewSetLabel,
    UnregisterObject,
    ReturnAdapterRequestDeviceCallback,
    ReturnBufferMapAsyncCallback,
    ReturnDeviceCreateComputePipelineAsyncCallback,
    ReturnDeviceCreateRenderPipelineAsyncCallback,
    ReturnDeviceLoggingCallback,
    ReturnDeviceLostCallback,
    ReturnDevicePopErrorScopeCallback,
    ReturnDeviceUncapturedErrorCallback,
    ReturnInstanceRequestAdapterCallback,
    ReturnQueueWorkDoneCallback,
    ReturnShaderModuleGetCompilationInfoCallback,
}
pub const CHUNKED_COMMAND_FIELDS: &[&str] = &["id", "size", "chunk data", "chunk size"];
pub const ADAPTER_REQUEST_DEVICE_FIELDS: &[&str] = &[
    "adapter id",
    "event manager handle",
    "future",
    "device object handle",
    "device lost future",
    "descriptor",
];
pub const BIND_GROUP_LAYOUT_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const BIND_GROUP_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const BUFFER_CREATE_TEXEL_VIEW_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const BUFFER_MAP_ASYNC_FIELDS: &[&str] = &[
    "buffer id",
    "event manager handle",
    "future",
    "mode",
    "offset",
    "size",
];
pub const BUFFER_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const BUFFER_UPDATE_MAPPED_DATA_FIELDS: &[&str] = &[
    "buffer id",
    "write data update info length",
    "write data update info",
    "offset",
    "size",
];
pub const COMMAND_BUFFER_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const COMMAND_ENCODER_BEGIN_COMPUTE_PASS_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const COMMAND_ENCODER_BEGIN_RENDER_PASS_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const COMMAND_ENCODER_CLEAR_BUFFER_FIELDS: &[&str] = &[
    "self",
    "buffer",
    "offset",
    "size",
];
pub const COMMAND_ENCODER_COPY_BUFFER_TO_BUFFER_FIELDS: &[&str] = &[
    "self",
    "source",
    "source offset",
    "destination",
    "destination offset",
    "size",
];
pub const COMMAND_ENCODER_COPY_BUFFER_TO_TEXTURE_FIELDS: &[&str] = &[
    "self",
    "source",
    "destination",
    "copy size",
];
pub const COMMAND_ENCODER_COPY_TEXTURE_TO_BUFFER_FIELDS: &[&str] = &[
    "self",
    "source",
    "destination",
    "copy size",
];
pub const COMMAND_ENCODER_COPY_TEXTURE_TO_TEXTURE_FIELDS: &[&str] = &[
    "self",
    "source",
    "destination",
    "copy size",
];
pub const COMMAND_ENCODER_FINISH_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const COMMAND_ENCODER_INJECT_VALIDATION_ERROR_FIELDS: &[&str] = &["self", "message"];
pub const COMMAND_ENCODER_INSERT_DEBUG_MARKER_FIELDS: &[&str] = &[
    "self",
    "marker label",
];
pub const COMMAND_ENCODER_POP_DEBUG_GROUP_FIELDS: &[&str] = &["self"];
pub const COMMAND_ENCODER_PUSH_DEBUG_GROUP_FIELDS: &[&str] = &["self", "group label"];
pub const COMMAND_ENCODER_RESOLVE_QUERY_SET_FIELDS: &[&str] = &[
    "self",
    "query set",
    "first query",
    "query count",
    "destination",
    "destination offset",
];
pub const COMMAND_ENCODER_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const COMMAND_ENCODER_WRITE_BUFFER_FIELDS: &[&str] = &[
    "self",
    "buffer",
    "buffer offset",
    "data",
    "size",
];
pub const COMMAND_ENCODER_WRITE_TIMESTAMP_FIELDS: &[&str] = &[
    "self",
    "query set",
    "query index",
];
pub const COMPUTE_PASS_ENCODER_DISPATCH_WORKGROUPS_FIELDS: &[&str] = &[
    "self",
    "workgroupCountX",
    "workgroupCountY",
    "workgroupCountZ",
];
pub const COMPUTE_PASS_ENCODER_DISPATCH_WORKGROUPS_INDIRECT_FIELDS: &[&str] = &[
    "self",
    "indirect buffer",
    "indirect offset",
];
pub const COMPUTE_PASS_ENCODER_END_FIELDS: &[&str] = &["self"];
pub const COMPUTE_PASS_ENCODER_INSERT_DEBUG_MARKER_FIELDS: &[&str] = &[
    "self",
    "marker label",
];
pub const COMPUTE_PASS_ENCODER_POP_DEBUG_GROUP_FIELDS: &[&str] = &["self"];
pub const COMPUTE_PASS_ENCODER_PUSH_DEBUG_GROUP_FIELDS: &[&str] = &[
    "self",
    "group label",
];
pub const COMPUTE_PASS_ENCODER_SET_BIND_GROUP_FIELDS: &[&str] = &[
    "self",
    "group index",
    "group",
    "dynamic offset count",
    "dynamic offsets",
];
pub const COMPUTE_PASS_ENCODER_SET_IMMEDIATES_FIELDS: &[&str] = &[
    "compute pass encoder id",
    "offset",
    "data",
    "size",
];
pub const COMPUTE_PASS_ENCODER_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const COMPUTE_PASS_ENCODER_SET_PIPELINE_FIELDS: &[&str] = &["self", "pipeline"];
pub const COMPUTE_PASS_ENCODER_SET_RESOURCE_TABLE_FIELDS: &[&str] = &["self", "table"];
pub const COMPUTE_PASS_ENCODER_WRITE_TIMESTAMP_FIELDS: &[&str] = &[
    "self",
    "query set",
    "query index",
];
pub const COMPUTE_PIPELINE_GET_BIND_GROUP_LAYOUT_FIELDS: &[&str] = &[
    "self",
    "group index",
    "result",
];
pub const COMPUTE_PIPELINE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const DEVICE_CREATE_BIND_GROUP_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const DEVICE_CREATE_BIND_GROUP_LAYOUT_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_CREATE_BUFFER_FIELDS: &[&str] = &[
    "device id",
    "descriptor",
    "result",
    "read handle create info length",
    "read handle create info",
    "write handle create info length",
    "write handle create info",
];
pub const DEVICE_CREATE_COMMAND_ENCODER_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_CREATE_COMPUTE_PIPELINE_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_CREATE_COMPUTE_PIPELINE_ASYNC_FIELDS: &[&str] = &[
    "device id",
    "event manager handle",
    "future",
    "pipeline object handle",
    "descriptor",
];
pub const DEVICE_CREATE_ERROR_EXTERNAL_TEXTURE_FIELDS: &[&str] = &["self", "result"];
pub const DEVICE_CREATE_ERROR_SHADER_MODULE_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "error message",
    "result",
];
pub const DEVICE_CREATE_EXTERNAL_TEXTURE_FIELDS: &[&str] = &[
    "self",
    "external texture descriptor",
    "result",
];
pub const DEVICE_CREATE_PIPELINE_LAYOUT_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_CREATE_QUERY_SET_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const DEVICE_CREATE_RENDER_BUNDLE_ENCODER_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_CREATE_RENDER_PIPELINE_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_CREATE_RENDER_PIPELINE_ASYNC_FIELDS: &[&str] = &[
    "device id",
    "event manager handle",
    "future",
    "pipeline object handle",
    "descriptor",
];
pub const DEVICE_CREATE_SAMPLER_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const DEVICE_CREATE_SHADER_MODULE_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_FORCE_LOSS_FIELDS: &[&str] = &["self", "type", "message"];
pub const DEVICE_GET_A_HARDWARE_BUFFER_PROPERTIES_FIELDS: &[&str] = &[
    "self",
    "handle",
    "properties",
];
pub const DEVICE_IMPORT_SHARED_BUFFER_MEMORY_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_IMPORT_SHARED_FENCE_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const DEVICE_IMPORT_SHARED_TEXTURE_MEMORY_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const DEVICE_POP_ERROR_SCOPE_FIELDS: &[&str] = &[
    "device id",
    "event manager handle",
    "future",
];
pub const DEVICE_PUSH_ERROR_SCOPE_FIELDS: &[&str] = &["self", "filter"];
pub const DEVICE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const DEVICE_TICK_FIELDS: &[&str] = &["self"];
pub const DEVICE_VALIDATE_TEXTURE_DESCRIPTOR_FIELDS: &[&str] = &["self", "descriptor"];
pub const EXTERNAL_TEXTURE_DESTROY_FIELDS: &[&str] = &["self"];
pub const EXTERNAL_TEXTURE_EXPIRE_FIELDS: &[&str] = &["self"];
pub const EXTERNAL_TEXTURE_REFRESH_FIELDS: &[&str] = &["self"];
pub const EXTERNAL_TEXTURE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const INSTANCE_REQUEST_ADAPTER_FIELDS: &[&str] = &[
    "instance id",
    "event manager handle",
    "future",
    "adapter object handle",
    "options",
];
pub const PIPELINE_LAYOUT_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const QUERY_SET_DESTROY_FIELDS: &[&str] = &["self"];
pub const QUERY_SET_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const QUEUE_COPY_EXTERNAL_TEXTURE_FOR_BROWSER_FIELDS: &[&str] = &[
    "self",
    "source",
    "destination",
    "copy size",
    "options",
];
pub const QUEUE_COPY_TEXTURE_FOR_BROWSER_FIELDS: &[&str] = &[
    "self",
    "source",
    "destination",
    "copy size",
    "options",
];
pub const QUEUE_ON_SUBMITTED_WORK_DONE_FIELDS: &[&str] = &[
    "queue id",
    "event manager handle",
    "future",
];
pub const QUEUE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const QUEUE_WRITE_BUFFER_FIELDS: &[&str] = &[
    "queue id",
    "buffer id",
    "buffer offset",
    "data",
    "size",
];
pub const QUEUE_WRITE_BUFFER_XL_FIELDS: &[&str] = &[
    "queue id",
    "buffer id",
    "buffer offset",
    "size",
    "write handle create info length",
    "write handle create info",
    "write data update info length",
    "write data update info",
];
pub const QUEUE_WRITE_TEXTURE_FIELDS: &[&str] = &[
    "queue id",
    "destination",
    "data",
    "data size",
    "data layout",
    "writeSize",
];
pub const QUEUE_WRITE_TEXTURE_XL_FIELDS: &[&str] = &[
    "queue id",
    "destination",
    "data size",
    "data layout",
    "writeSize",
    "write handle create info length",
    "write handle create info",
    "write data update info length",
    "write data update info",
];
pub const RENDER_BUNDLE_ENCODER_DRAW_FIELDS: &[&str] = &[
    "self",
    "vertex count",
    "instance count",
    "first vertex",
    "first instance",
];
pub const RENDER_BUNDLE_ENCODER_DRAW_INDEXED_FIELDS: &[&str] = &[
    "self",
    "index count",
    "instance count",
    "first index",
    "base vertex",
    "first instance",
];
pub const RENDER_BUNDLE_ENCODER_DRAW_INDEXED_INDIRECT_FIELDS: &[&str] = &[
    "self",
    "indirect buffer",
    "indirect offset",
];
pub const RENDER_BUNDLE_ENCODER_DRAW_INDIRECT_FIELDS: &[&str] = &[
    "self",
    "indirect buffer",
    "indirect offset",
];
pub const RENDER_BUNDLE_ENCODER_FINISH_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const RENDER_BUNDLE_ENCODER_INSERT_DEBUG_MARKER_FIELDS: &[&str] = &[
    "self",
    "marker label",
];
pub const RENDER_BUNDLE_ENCODER_POP_DEBUG_GROUP_FIELDS: &[&str] = &["self"];
pub const RENDER_BUNDLE_ENCODER_PUSH_DEBUG_GROUP_FIELDS: &[&str] = &[
    "self",
    "group label",
];
pub const RENDER_BUNDLE_ENCODER_SET_BIND_GROUP_FIELDS: &[&str] = &[
    "self",
    "group index",
    "group",
    "dynamic offset count",
    "dynamic offsets",
];
pub const RENDER_BUNDLE_ENCODER_SET_IMMEDIATES_FIELDS: &[&str] = &[
    "render bundle encoder id",
    "offset",
    "data",
    "size",
];
pub const RENDER_BUNDLE_ENCODER_SET_INDEX_BUFFER_FIELDS: &[&str] = &[
    "self",
    "buffer",
    "format",
    "offset",
    "size",
];
pub const RENDER_BUNDLE_ENCODER_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const RENDER_BUNDLE_ENCODER_SET_PIPELINE_FIELDS: &[&str] = &["self", "pipeline"];
pub const RENDER_BUNDLE_ENCODER_SET_RESOURCE_TABLE_FIELDS: &[&str] = &["self", "table"];
pub const RENDER_BUNDLE_ENCODER_SET_VERTEX_BUFFER_FIELDS: &[&str] = &[
    "self",
    "slot",
    "buffer",
    "offset",
    "size",
];
pub const RENDER_BUNDLE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const RENDER_PASS_ENCODER_BEGIN_OCCLUSION_QUERY_FIELDS: &[&str] = &[
    "self",
    "query index",
];
pub const RENDER_PASS_ENCODER_DRAW_FIELDS: &[&str] = &[
    "self",
    "vertex count",
    "instance count",
    "first vertex",
    "first instance",
];
pub const RENDER_PASS_ENCODER_DRAW_INDEXED_FIELDS: &[&str] = &[
    "self",
    "index count",
    "instance count",
    "first index",
    "base vertex",
    "first instance",
];
pub const RENDER_PASS_ENCODER_DRAW_INDEXED_INDIRECT_FIELDS: &[&str] = &[
    "self",
    "indirect buffer",
    "indirect offset",
];
pub const RENDER_PASS_ENCODER_DRAW_INDIRECT_FIELDS: &[&str] = &[
    "self",
    "indirect buffer",
    "indirect offset",
];
pub const RENDER_PASS_ENCODER_END_FIELDS: &[&str] = &["self"];
pub const RENDER_PASS_ENCODER_END_OCCLUSION_QUERY_FIELDS: &[&str] = &["self"];
pub const RENDER_PASS_ENCODER_EXECUTE_BUNDLES_FIELDS: &[&str] = &[
    "self",
    "bundle count",
    "bundles",
];
pub const RENDER_PASS_ENCODER_INSERT_DEBUG_MARKER_FIELDS: &[&str] = &[
    "self",
    "marker label",
];
pub const RENDER_PASS_ENCODER_MULTI_DRAW_INDEXED_INDIRECT_FIELDS: &[&str] = &[
    "self",
    "indirect buffer",
    "indirect offset",
    "max draw count",
    "draw count buffer",
    "draw count buffer offset",
];
pub const RENDER_PASS_ENCODER_MULTI_DRAW_INDIRECT_FIELDS: &[&str] = &[
    "self",
    "indirect buffer",
    "indirect offset",
    "max draw count",
    "draw count buffer",
    "draw count buffer offset",
];
pub const RENDER_PASS_ENCODER_PIXEL_LOCAL_STORAGE_BARRIER_FIELDS: &[&str] = &["self"];
pub const RENDER_PASS_ENCODER_POP_DEBUG_GROUP_FIELDS: &[&str] = &["self"];
pub const RENDER_PASS_ENCODER_PUSH_DEBUG_GROUP_FIELDS: &[&str] = &[
    "self",
    "group label",
];
pub const RENDER_PASS_ENCODER_SET_BIND_GROUP_FIELDS: &[&str] = &[
    "self",
    "group index",
    "group",
    "dynamic offset count",
    "dynamic offsets",
];
pub const RENDER_PASS_ENCODER_SET_BLEND_CONSTANT_FIELDS: &[&str] = &["self", "color"];
pub const RENDER_PASS_ENCODER_SET_IMMEDIATES_FIELDS: &[&str] = &[
    "render pass encoder id",
    "offset",
    "data",
    "size",
];
pub const RENDER_PASS_ENCODER_SET_INDEX_BUFFER_FIELDS: &[&str] = &[
    "self",
    "buffer",
    "format",
    "offset",
    "size",
];
pub const RENDER_PASS_ENCODER_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const RENDER_PASS_ENCODER_SET_PIPELINE_FIELDS: &[&str] = &["self", "pipeline"];
pub const RENDER_PASS_ENCODER_SET_RESOURCE_TABLE_FIELDS: &[&str] = &["self", "table"];
pub const RENDER_PASS_ENCODER_SET_SCISSOR_RECT_FIELDS: &[&str] = &[
    "self",
    "x",
    "y",
    "width",
    "height",
];
pub const RENDER_PASS_ENCODER_SET_STENCIL_REFERENCE_FIELDS: &[&str] = &[
    "self",
    "reference",
];
pub const RENDER_PASS_ENCODER_SET_VERTEX_BUFFER_FIELDS: &[&str] = &[
    "self",
    "slot",
    "buffer",
    "offset",
    "size",
];
pub const RENDER_PASS_ENCODER_SET_VIEWPORT_FIELDS: &[&str] = &[
    "self",
    "x",
    "y",
    "width",
    "height",
    "min depth",
    "max depth",
];
pub const RENDER_PASS_ENCODER_WRITE_TIMESTAMP_FIELDS: &[&str] = &[
    "self",
    "query set",
    "query index",
];
pub const RENDER_PIPELINE_GET_BIND_GROUP_LAYOUT_FIELDS: &[&str] = &[
    "self",
    "group index",
    "result",
];
pub const RENDER_PIPELINE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const SAMPLER_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const SHADER_MODULE_GET_COMPILATION_INFO_FIELDS: &[&str] = &[
    "shader module id",
    "event manager handle",
    "future",
];
pub const SHADER_MODULE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const SHARED_BUFFER_MEMORY_BEGIN_ACCESS_FIELDS: &[&str] = &[
    "self",
    "buffer",
    "descriptor",
];
pub const SHARED_BUFFER_MEMORY_CREATE_BUFFER_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const SHARED_BUFFER_MEMORY_END_ACCESS_FIELDS: &[&str] = &[
    "self",
    "buffer",
    "descriptor",
];
pub const SHARED_BUFFER_MEMORY_GET_PROPERTIES_FIELDS: &[&str] = &["self", "properties"];
pub const SHARED_BUFFER_MEMORY_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const SHARED_FENCE_EXPORT_INFO_FIELDS: &[&str] = &["self", "info"];
pub const SHARED_TEXTURE_MEMORY_BEGIN_ACCESS_FIELDS: &[&str] = &[
    "self",
    "texture",
    "descriptor",
];
pub const SHARED_TEXTURE_MEMORY_CREATE_TEXTURE_FIELDS: &[&str] = &[
    "self",
    "descriptor",
    "result",
];
pub const SHARED_TEXTURE_MEMORY_END_ACCESS_FIELDS: &[&str] = &[
    "self",
    "texture",
    "descriptor",
];
pub const SHARED_TEXTURE_MEMORY_GET_PROPERTIES_FIELDS: &[&str] = &["self", "properties"];
pub const SHARED_TEXTURE_MEMORY_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const SURFACE_GET_CURRENT_TEXTURE_FIELDS: &[&str] = &[
    "surface id",
    "configured device id",
    "texture handle",
];
pub const SURFACE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const TEXEL_BUFFER_VIEW_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const TEXTURE_CREATE_ERROR_VIEW_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const TEXTURE_CREATE_VIEW_FIELDS: &[&str] = &["self", "descriptor", "result"];
pub const TEXTURE_DESTROY_FIELDS: &[&str] = &["self"];
pub const TEXTURE_PIN_FIELDS: &[&str] = &["self", "usage"];
pub const TEXTURE_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const TEXTURE_SET_OWNERSHIP_FOR_MEMORY_DUMP_FIELDS: &[&str] = &[
    "self",
    "owner guid",
];
pub const TEXTURE_UNPIN_FIELDS: &[&str] = &["self"];
pub const TEXTURE_VIEW_SET_LABEL_FIELDS: &[&str] = &["self", "label"];
pub const UNREGISTER_OBJECT_FIELDS: &[&str] = &["object type", "object id"];
pub const RETURN_ADAPTER_REQUEST_DEVICE_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "message",
    "limits",
    "features count",
    "features",
];
pub const RETURN_BUFFER_MAP_ASYNC_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "message",
    "read data update info length",
    "read data update info",
];
pub const RETURN_DEVICE_CREATE_COMPUTE_PIPELINE_ASYNC_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "message",
];
pub const RETURN_DEVICE_CREATE_RENDER_PIPELINE_ASYNC_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "message",
];
pub const RETURN_DEVICE_LOGGING_CALLBACK_FIELDS: &[&str] = &[
    "device",
    "type",
    "message",
];
pub const RETURN_DEVICE_LOST_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "reason",
    "message",
];
pub const RETURN_DEVICE_POP_ERROR_SCOPE_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "type",
    "message",
];
pub const RETURN_DEVICE_UNCAPTURED_ERROR_CALLBACK_FIELDS: &[&str] = &[
    "device",
    "type",
    "message",
];
pub const RETURN_INSTANCE_REQUEST_ADAPTER_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "message",
    "info",
    "limits",
    "features count",
    "features",
];
pub const RETURN_QUEUE_WORK_DONE_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "message",
];
pub const RETURN_SHADER_MODULE_GET_COMPILATION_INFO_CALLBACK_FIELDS: &[&str] = &[
    "event manager",
    "future",
    "status",
    "info",
];
pub fn wire_command_meta(command: WireCommand) -> WireCommandMeta {
    match command {
        WireCommand::ChunkedCommand => {
            WireCommandMeta {
                name: "chunked command",
                direction: WireDirection::Special,
                fields: CHUNKED_COMMAND_FIELDS,
            }
        }
        WireCommand::AdapterRequestDevice => {
            WireCommandMeta {
                name: "adapter request device",
                direction: WireDirection::ClientToServer,
                fields: ADAPTER_REQUEST_DEVICE_FIELDS,
            }
        }
        WireCommand::BindGroupLayoutSetLabel => {
            WireCommandMeta {
                name: "bind group layout set label",
                direction: WireDirection::ClientToServer,
                fields: BIND_GROUP_LAYOUT_SET_LABEL_FIELDS,
            }
        }
        WireCommand::BindGroupSetLabel => {
            WireCommandMeta {
                name: "bind group set label",
                direction: WireDirection::ClientToServer,
                fields: BIND_GROUP_SET_LABEL_FIELDS,
            }
        }
        WireCommand::BufferCreateTexelView => {
            WireCommandMeta {
                name: "buffer create texel view",
                direction: WireDirection::ClientToServer,
                fields: BUFFER_CREATE_TEXEL_VIEW_FIELDS,
            }
        }
        WireCommand::BufferMapAsync => {
            WireCommandMeta {
                name: "buffer map async",
                direction: WireDirection::ClientToServer,
                fields: BUFFER_MAP_ASYNC_FIELDS,
            }
        }
        WireCommand::BufferSetLabel => {
            WireCommandMeta {
                name: "buffer set label",
                direction: WireDirection::ClientToServer,
                fields: BUFFER_SET_LABEL_FIELDS,
            }
        }
        WireCommand::BufferUpdateMappedData => {
            WireCommandMeta {
                name: "buffer update mapped data",
                direction: WireDirection::ClientToServer,
                fields: BUFFER_UPDATE_MAPPED_DATA_FIELDS,
            }
        }
        WireCommand::CommandBufferSetLabel => {
            WireCommandMeta {
                name: "command buffer set label",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_BUFFER_SET_LABEL_FIELDS,
            }
        }
        WireCommand::CommandEncoderBeginComputePass => {
            WireCommandMeta {
                name: "command encoder begin compute pass",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_BEGIN_COMPUTE_PASS_FIELDS,
            }
        }
        WireCommand::CommandEncoderBeginRenderPass => {
            WireCommandMeta {
                name: "command encoder begin render pass",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_BEGIN_RENDER_PASS_FIELDS,
            }
        }
        WireCommand::CommandEncoderClearBuffer => {
            WireCommandMeta {
                name: "command encoder clear buffer",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_CLEAR_BUFFER_FIELDS,
            }
        }
        WireCommand::CommandEncoderCopyBufferToBuffer => {
            WireCommandMeta {
                name: "command encoder copy buffer to buffer",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_COPY_BUFFER_TO_BUFFER_FIELDS,
            }
        }
        WireCommand::CommandEncoderCopyBufferToTexture => {
            WireCommandMeta {
                name: "command encoder copy buffer to texture",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_COPY_BUFFER_TO_TEXTURE_FIELDS,
            }
        }
        WireCommand::CommandEncoderCopyTextureToBuffer => {
            WireCommandMeta {
                name: "command encoder copy texture to buffer",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_COPY_TEXTURE_TO_BUFFER_FIELDS,
            }
        }
        WireCommand::CommandEncoderCopyTextureToTexture => {
            WireCommandMeta {
                name: "command encoder copy texture to texture",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_COPY_TEXTURE_TO_TEXTURE_FIELDS,
            }
        }
        WireCommand::CommandEncoderFinish => {
            WireCommandMeta {
                name: "command encoder finish",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_FINISH_FIELDS,
            }
        }
        WireCommand::CommandEncoderInjectValidationError => {
            WireCommandMeta {
                name: "command encoder inject validation error",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_INJECT_VALIDATION_ERROR_FIELDS,
            }
        }
        WireCommand::CommandEncoderInsertDebugMarker => {
            WireCommandMeta {
                name: "command encoder insert debug marker",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_INSERT_DEBUG_MARKER_FIELDS,
            }
        }
        WireCommand::CommandEncoderPopDebugGroup => {
            WireCommandMeta {
                name: "command encoder pop debug group",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_POP_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::CommandEncoderPushDebugGroup => {
            WireCommandMeta {
                name: "command encoder push debug group",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_PUSH_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::CommandEncoderResolveQuerySet => {
            WireCommandMeta {
                name: "command encoder resolve query set",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_RESOLVE_QUERY_SET_FIELDS,
            }
        }
        WireCommand::CommandEncoderSetLabel => {
            WireCommandMeta {
                name: "command encoder set label",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_SET_LABEL_FIELDS,
            }
        }
        WireCommand::CommandEncoderWriteBuffer => {
            WireCommandMeta {
                name: "command encoder write buffer",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_WRITE_BUFFER_FIELDS,
            }
        }
        WireCommand::CommandEncoderWriteTimestamp => {
            WireCommandMeta {
                name: "command encoder write timestamp",
                direction: WireDirection::ClientToServer,
                fields: COMMAND_ENCODER_WRITE_TIMESTAMP_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderDispatchWorkgroups => {
            WireCommandMeta {
                name: "compute pass encoder dispatch workgroups",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_DISPATCH_WORKGROUPS_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderDispatchWorkgroupsIndirect => {
            WireCommandMeta {
                name: "compute pass encoder dispatch workgroups indirect",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_DISPATCH_WORKGROUPS_INDIRECT_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderEnd => {
            WireCommandMeta {
                name: "compute pass encoder end",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_END_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderInsertDebugMarker => {
            WireCommandMeta {
                name: "compute pass encoder insert debug marker",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_INSERT_DEBUG_MARKER_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderPopDebugGroup => {
            WireCommandMeta {
                name: "compute pass encoder pop debug group",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_POP_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderPushDebugGroup => {
            WireCommandMeta {
                name: "compute pass encoder push debug group",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_PUSH_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderSetBindGroup => {
            WireCommandMeta {
                name: "compute pass encoder set bind group",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_SET_BIND_GROUP_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderSetImmediates => {
            WireCommandMeta {
                name: "compute pass encoder set immediates",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_SET_IMMEDIATES_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderSetLabel => {
            WireCommandMeta {
                name: "compute pass encoder set label",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_SET_LABEL_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderSetPipeline => {
            WireCommandMeta {
                name: "compute pass encoder set pipeline",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_SET_PIPELINE_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderSetResourceTable => {
            WireCommandMeta {
                name: "compute pass encoder set resource table",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_SET_RESOURCE_TABLE_FIELDS,
            }
        }
        WireCommand::ComputePassEncoderWriteTimestamp => {
            WireCommandMeta {
                name: "compute pass encoder write timestamp",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PASS_ENCODER_WRITE_TIMESTAMP_FIELDS,
            }
        }
        WireCommand::ComputePipelineGetBindGroupLayout => {
            WireCommandMeta {
                name: "compute pipeline get bind group layout",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PIPELINE_GET_BIND_GROUP_LAYOUT_FIELDS,
            }
        }
        WireCommand::ComputePipelineSetLabel => {
            WireCommandMeta {
                name: "compute pipeline set label",
                direction: WireDirection::ClientToServer,
                fields: COMPUTE_PIPELINE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::DeviceCreateBindGroup => {
            WireCommandMeta {
                name: "device create bind group",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_BIND_GROUP_FIELDS,
            }
        }
        WireCommand::DeviceCreateBindGroupLayout => {
            WireCommandMeta {
                name: "device create bind group layout",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_BIND_GROUP_LAYOUT_FIELDS,
            }
        }
        WireCommand::DeviceCreateBuffer => {
            WireCommandMeta {
                name: "device create buffer",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_BUFFER_FIELDS,
            }
        }
        WireCommand::DeviceCreateCommandEncoder => {
            WireCommandMeta {
                name: "device create command encoder",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_COMMAND_ENCODER_FIELDS,
            }
        }
        WireCommand::DeviceCreateComputePipeline => {
            WireCommandMeta {
                name: "device create compute pipeline",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_COMPUTE_PIPELINE_FIELDS,
            }
        }
        WireCommand::DeviceCreateComputePipelineAsync => {
            WireCommandMeta {
                name: "device create compute pipeline async",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_COMPUTE_PIPELINE_ASYNC_FIELDS,
            }
        }
        WireCommand::DeviceCreateErrorExternalTexture => {
            WireCommandMeta {
                name: "device create error external texture",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_ERROR_EXTERNAL_TEXTURE_FIELDS,
            }
        }
        WireCommand::DeviceCreateErrorShaderModule => {
            WireCommandMeta {
                name: "device create error shader module",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_ERROR_SHADER_MODULE_FIELDS,
            }
        }
        WireCommand::DeviceCreateExternalTexture => {
            WireCommandMeta {
                name: "device create external texture",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_EXTERNAL_TEXTURE_FIELDS,
            }
        }
        WireCommand::DeviceCreatePipelineLayout => {
            WireCommandMeta {
                name: "device create pipeline layout",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_PIPELINE_LAYOUT_FIELDS,
            }
        }
        WireCommand::DeviceCreateQuerySet => {
            WireCommandMeta {
                name: "device create query set",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_QUERY_SET_FIELDS,
            }
        }
        WireCommand::DeviceCreateRenderBundleEncoder => {
            WireCommandMeta {
                name: "device create render bundle encoder",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_RENDER_BUNDLE_ENCODER_FIELDS,
            }
        }
        WireCommand::DeviceCreateRenderPipeline => {
            WireCommandMeta {
                name: "device create render pipeline",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_RENDER_PIPELINE_FIELDS,
            }
        }
        WireCommand::DeviceCreateRenderPipelineAsync => {
            WireCommandMeta {
                name: "device create render pipeline async",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_RENDER_PIPELINE_ASYNC_FIELDS,
            }
        }
        WireCommand::DeviceCreateSampler => {
            WireCommandMeta {
                name: "device create sampler",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_SAMPLER_FIELDS,
            }
        }
        WireCommand::DeviceCreateShaderModule => {
            WireCommandMeta {
                name: "device create shader module",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_CREATE_SHADER_MODULE_FIELDS,
            }
        }
        WireCommand::DeviceForceLoss => {
            WireCommandMeta {
                name: "device force loss",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_FORCE_LOSS_FIELDS,
            }
        }
        WireCommand::DeviceGetAHardwareBufferProperties => {
            WireCommandMeta {
                name: "device get a hardware buffer properties",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_GET_A_HARDWARE_BUFFER_PROPERTIES_FIELDS,
            }
        }
        WireCommand::DeviceImportSharedBufferMemory => {
            WireCommandMeta {
                name: "device import shared buffer memory",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_IMPORT_SHARED_BUFFER_MEMORY_FIELDS,
            }
        }
        WireCommand::DeviceImportSharedFence => {
            WireCommandMeta {
                name: "device import shared fence",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_IMPORT_SHARED_FENCE_FIELDS,
            }
        }
        WireCommand::DeviceImportSharedTextureMemory => {
            WireCommandMeta {
                name: "device import shared texture memory",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_IMPORT_SHARED_TEXTURE_MEMORY_FIELDS,
            }
        }
        WireCommand::DevicePopErrorScope => {
            WireCommandMeta {
                name: "device pop error scope",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_POP_ERROR_SCOPE_FIELDS,
            }
        }
        WireCommand::DevicePushErrorScope => {
            WireCommandMeta {
                name: "device push error scope",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_PUSH_ERROR_SCOPE_FIELDS,
            }
        }
        WireCommand::DeviceSetLabel => {
            WireCommandMeta {
                name: "device set label",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::DeviceTick => {
            WireCommandMeta {
                name: "device tick",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_TICK_FIELDS,
            }
        }
        WireCommand::DeviceValidateTextureDescriptor => {
            WireCommandMeta {
                name: "device validate texture descriptor",
                direction: WireDirection::ClientToServer,
                fields: DEVICE_VALIDATE_TEXTURE_DESCRIPTOR_FIELDS,
            }
        }
        WireCommand::ExternalTextureDestroy => {
            WireCommandMeta {
                name: "external texture destroy",
                direction: WireDirection::ClientToServer,
                fields: EXTERNAL_TEXTURE_DESTROY_FIELDS,
            }
        }
        WireCommand::ExternalTextureExpire => {
            WireCommandMeta {
                name: "external texture expire",
                direction: WireDirection::ClientToServer,
                fields: EXTERNAL_TEXTURE_EXPIRE_FIELDS,
            }
        }
        WireCommand::ExternalTextureRefresh => {
            WireCommandMeta {
                name: "external texture refresh",
                direction: WireDirection::ClientToServer,
                fields: EXTERNAL_TEXTURE_REFRESH_FIELDS,
            }
        }
        WireCommand::ExternalTextureSetLabel => {
            WireCommandMeta {
                name: "external texture set label",
                direction: WireDirection::ClientToServer,
                fields: EXTERNAL_TEXTURE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::InstanceRequestAdapter => {
            WireCommandMeta {
                name: "instance request adapter",
                direction: WireDirection::ClientToServer,
                fields: INSTANCE_REQUEST_ADAPTER_FIELDS,
            }
        }
        WireCommand::PipelineLayoutSetLabel => {
            WireCommandMeta {
                name: "pipeline layout set label",
                direction: WireDirection::ClientToServer,
                fields: PIPELINE_LAYOUT_SET_LABEL_FIELDS,
            }
        }
        WireCommand::QuerySetDestroy => {
            WireCommandMeta {
                name: "query set destroy",
                direction: WireDirection::ClientToServer,
                fields: QUERY_SET_DESTROY_FIELDS,
            }
        }
        WireCommand::QuerySetSetLabel => {
            WireCommandMeta {
                name: "query set set label",
                direction: WireDirection::ClientToServer,
                fields: QUERY_SET_SET_LABEL_FIELDS,
            }
        }
        WireCommand::QueueCopyExternalTextureForBrowser => {
            WireCommandMeta {
                name: "queue copy external texture for browser",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_COPY_EXTERNAL_TEXTURE_FOR_BROWSER_FIELDS,
            }
        }
        WireCommand::QueueCopyTextureForBrowser => {
            WireCommandMeta {
                name: "queue copy texture for browser",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_COPY_TEXTURE_FOR_BROWSER_FIELDS,
            }
        }
        WireCommand::QueueOnSubmittedWorkDone => {
            WireCommandMeta {
                name: "queue on submitted work done",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_ON_SUBMITTED_WORK_DONE_FIELDS,
            }
        }
        WireCommand::QueueSetLabel => {
            WireCommandMeta {
                name: "queue set label",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::QueueWriteBuffer => {
            WireCommandMeta {
                name: "queue write buffer",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_WRITE_BUFFER_FIELDS,
            }
        }
        WireCommand::QueueWriteBufferXl => {
            WireCommandMeta {
                name: "queue write buffer xl",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_WRITE_BUFFER_XL_FIELDS,
            }
        }
        WireCommand::QueueWriteTexture => {
            WireCommandMeta {
                name: "queue write texture",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_WRITE_TEXTURE_FIELDS,
            }
        }
        WireCommand::QueueWriteTextureXl => {
            WireCommandMeta {
                name: "queue write texture xl",
                direction: WireDirection::ClientToServer,
                fields: QUEUE_WRITE_TEXTURE_XL_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderDraw => {
            WireCommandMeta {
                name: "render bundle encoder draw",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_DRAW_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderDrawIndexed => {
            WireCommandMeta {
                name: "render bundle encoder draw indexed",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_DRAW_INDEXED_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderDrawIndexedIndirect => {
            WireCommandMeta {
                name: "render bundle encoder draw indexed indirect",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_DRAW_INDEXED_INDIRECT_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderDrawIndirect => {
            WireCommandMeta {
                name: "render bundle encoder draw indirect",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_DRAW_INDIRECT_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderFinish => {
            WireCommandMeta {
                name: "render bundle encoder finish",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_FINISH_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderInsertDebugMarker => {
            WireCommandMeta {
                name: "render bundle encoder insert debug marker",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_INSERT_DEBUG_MARKER_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderPopDebugGroup => {
            WireCommandMeta {
                name: "render bundle encoder pop debug group",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_POP_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderPushDebugGroup => {
            WireCommandMeta {
                name: "render bundle encoder push debug group",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_PUSH_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderSetBindGroup => {
            WireCommandMeta {
                name: "render bundle encoder set bind group",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_SET_BIND_GROUP_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderSetImmediates => {
            WireCommandMeta {
                name: "render bundle encoder set immediates",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_SET_IMMEDIATES_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderSetIndexBuffer => {
            WireCommandMeta {
                name: "render bundle encoder set index buffer",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_SET_INDEX_BUFFER_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderSetLabel => {
            WireCommandMeta {
                name: "render bundle encoder set label",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_SET_LABEL_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderSetPipeline => {
            WireCommandMeta {
                name: "render bundle encoder set pipeline",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_SET_PIPELINE_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderSetResourceTable => {
            WireCommandMeta {
                name: "render bundle encoder set resource table",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_SET_RESOURCE_TABLE_FIELDS,
            }
        }
        WireCommand::RenderBundleEncoderSetVertexBuffer => {
            WireCommandMeta {
                name: "render bundle encoder set vertex buffer",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_ENCODER_SET_VERTEX_BUFFER_FIELDS,
            }
        }
        WireCommand::RenderBundleSetLabel => {
            WireCommandMeta {
                name: "render bundle set label",
                direction: WireDirection::ClientToServer,
                fields: RENDER_BUNDLE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderBeginOcclusionQuery => {
            WireCommandMeta {
                name: "render pass encoder begin occlusion query",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_BEGIN_OCCLUSION_QUERY_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderDraw => {
            WireCommandMeta {
                name: "render pass encoder draw",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_DRAW_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderDrawIndexed => {
            WireCommandMeta {
                name: "render pass encoder draw indexed",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_DRAW_INDEXED_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderDrawIndexedIndirect => {
            WireCommandMeta {
                name: "render pass encoder draw indexed indirect",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_DRAW_INDEXED_INDIRECT_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderDrawIndirect => {
            WireCommandMeta {
                name: "render pass encoder draw indirect",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_DRAW_INDIRECT_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderEnd => {
            WireCommandMeta {
                name: "render pass encoder end",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_END_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderEndOcclusionQuery => {
            WireCommandMeta {
                name: "render pass encoder end occlusion query",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_END_OCCLUSION_QUERY_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderExecuteBundles => {
            WireCommandMeta {
                name: "render pass encoder execute bundles",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_EXECUTE_BUNDLES_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderInsertDebugMarker => {
            WireCommandMeta {
                name: "render pass encoder insert debug marker",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_INSERT_DEBUG_MARKER_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderMultiDrawIndexedIndirect => {
            WireCommandMeta {
                name: "render pass encoder multi draw indexed indirect",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_MULTI_DRAW_INDEXED_INDIRECT_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderMultiDrawIndirect => {
            WireCommandMeta {
                name: "render pass encoder multi draw indirect",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_MULTI_DRAW_INDIRECT_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderPixelLocalStorageBarrier => {
            WireCommandMeta {
                name: "render pass encoder pixel local storage barrier",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_PIXEL_LOCAL_STORAGE_BARRIER_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderPopDebugGroup => {
            WireCommandMeta {
                name: "render pass encoder pop debug group",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_POP_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderPushDebugGroup => {
            WireCommandMeta {
                name: "render pass encoder push debug group",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_PUSH_DEBUG_GROUP_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetBindGroup => {
            WireCommandMeta {
                name: "render pass encoder set bind group",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_BIND_GROUP_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetBlendConstant => {
            WireCommandMeta {
                name: "render pass encoder set blend constant",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_BLEND_CONSTANT_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetImmediates => {
            WireCommandMeta {
                name: "render pass encoder set immediates",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_IMMEDIATES_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetIndexBuffer => {
            WireCommandMeta {
                name: "render pass encoder set index buffer",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_INDEX_BUFFER_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetLabel => {
            WireCommandMeta {
                name: "render pass encoder set label",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_LABEL_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetPipeline => {
            WireCommandMeta {
                name: "render pass encoder set pipeline",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_PIPELINE_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetResourceTable => {
            WireCommandMeta {
                name: "render pass encoder set resource table",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_RESOURCE_TABLE_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetScissorRect => {
            WireCommandMeta {
                name: "render pass encoder set scissor rect",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_SCISSOR_RECT_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetStencilReference => {
            WireCommandMeta {
                name: "render pass encoder set stencil reference",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_STENCIL_REFERENCE_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetVertexBuffer => {
            WireCommandMeta {
                name: "render pass encoder set vertex buffer",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_VERTEX_BUFFER_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderSetViewport => {
            WireCommandMeta {
                name: "render pass encoder set viewport",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_SET_VIEWPORT_FIELDS,
            }
        }
        WireCommand::RenderPassEncoderWriteTimestamp => {
            WireCommandMeta {
                name: "render pass encoder write timestamp",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PASS_ENCODER_WRITE_TIMESTAMP_FIELDS,
            }
        }
        WireCommand::RenderPipelineGetBindGroupLayout => {
            WireCommandMeta {
                name: "render pipeline get bind group layout",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PIPELINE_GET_BIND_GROUP_LAYOUT_FIELDS,
            }
        }
        WireCommand::RenderPipelineSetLabel => {
            WireCommandMeta {
                name: "render pipeline set label",
                direction: WireDirection::ClientToServer,
                fields: RENDER_PIPELINE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::SamplerSetLabel => {
            WireCommandMeta {
                name: "sampler set label",
                direction: WireDirection::ClientToServer,
                fields: SAMPLER_SET_LABEL_FIELDS,
            }
        }
        WireCommand::ShaderModuleGetCompilationInfo => {
            WireCommandMeta {
                name: "shader module get compilation info",
                direction: WireDirection::ClientToServer,
                fields: SHADER_MODULE_GET_COMPILATION_INFO_FIELDS,
            }
        }
        WireCommand::ShaderModuleSetLabel => {
            WireCommandMeta {
                name: "shader module set label",
                direction: WireDirection::ClientToServer,
                fields: SHADER_MODULE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::SharedBufferMemoryBeginAccess => {
            WireCommandMeta {
                name: "shared buffer memory begin access",
                direction: WireDirection::ClientToServer,
                fields: SHARED_BUFFER_MEMORY_BEGIN_ACCESS_FIELDS,
            }
        }
        WireCommand::SharedBufferMemoryCreateBuffer => {
            WireCommandMeta {
                name: "shared buffer memory create buffer",
                direction: WireDirection::ClientToServer,
                fields: SHARED_BUFFER_MEMORY_CREATE_BUFFER_FIELDS,
            }
        }
        WireCommand::SharedBufferMemoryEndAccess => {
            WireCommandMeta {
                name: "shared buffer memory end access",
                direction: WireDirection::ClientToServer,
                fields: SHARED_BUFFER_MEMORY_END_ACCESS_FIELDS,
            }
        }
        WireCommand::SharedBufferMemoryGetProperties => {
            WireCommandMeta {
                name: "shared buffer memory get properties",
                direction: WireDirection::ClientToServer,
                fields: SHARED_BUFFER_MEMORY_GET_PROPERTIES_FIELDS,
            }
        }
        WireCommand::SharedBufferMemorySetLabel => {
            WireCommandMeta {
                name: "shared buffer memory set label",
                direction: WireDirection::ClientToServer,
                fields: SHARED_BUFFER_MEMORY_SET_LABEL_FIELDS,
            }
        }
        WireCommand::SharedFenceExportInfo => {
            WireCommandMeta {
                name: "shared fence export info",
                direction: WireDirection::ClientToServer,
                fields: SHARED_FENCE_EXPORT_INFO_FIELDS,
            }
        }
        WireCommand::SharedTextureMemoryBeginAccess => {
            WireCommandMeta {
                name: "shared texture memory begin access",
                direction: WireDirection::ClientToServer,
                fields: SHARED_TEXTURE_MEMORY_BEGIN_ACCESS_FIELDS,
            }
        }
        WireCommand::SharedTextureMemoryCreateTexture => {
            WireCommandMeta {
                name: "shared texture memory create texture",
                direction: WireDirection::ClientToServer,
                fields: SHARED_TEXTURE_MEMORY_CREATE_TEXTURE_FIELDS,
            }
        }
        WireCommand::SharedTextureMemoryEndAccess => {
            WireCommandMeta {
                name: "shared texture memory end access",
                direction: WireDirection::ClientToServer,
                fields: SHARED_TEXTURE_MEMORY_END_ACCESS_FIELDS,
            }
        }
        WireCommand::SharedTextureMemoryGetProperties => {
            WireCommandMeta {
                name: "shared texture memory get properties",
                direction: WireDirection::ClientToServer,
                fields: SHARED_TEXTURE_MEMORY_GET_PROPERTIES_FIELDS,
            }
        }
        WireCommand::SharedTextureMemorySetLabel => {
            WireCommandMeta {
                name: "shared texture memory set label",
                direction: WireDirection::ClientToServer,
                fields: SHARED_TEXTURE_MEMORY_SET_LABEL_FIELDS,
            }
        }
        WireCommand::SurfaceGetCurrentTexture => {
            WireCommandMeta {
                name: "surface get current texture",
                direction: WireDirection::ClientToServer,
                fields: SURFACE_GET_CURRENT_TEXTURE_FIELDS,
            }
        }
        WireCommand::SurfaceSetLabel => {
            WireCommandMeta {
                name: "surface set label",
                direction: WireDirection::ClientToServer,
                fields: SURFACE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::TexelBufferViewSetLabel => {
            WireCommandMeta {
                name: "texel buffer view set label",
                direction: WireDirection::ClientToServer,
                fields: TEXEL_BUFFER_VIEW_SET_LABEL_FIELDS,
            }
        }
        WireCommand::TextureCreateErrorView => {
            WireCommandMeta {
                name: "texture create error view",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_CREATE_ERROR_VIEW_FIELDS,
            }
        }
        WireCommand::TextureCreateView => {
            WireCommandMeta {
                name: "texture create view",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_CREATE_VIEW_FIELDS,
            }
        }
        WireCommand::TextureDestroy => {
            WireCommandMeta {
                name: "texture destroy",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_DESTROY_FIELDS,
            }
        }
        WireCommand::TexturePin => {
            WireCommandMeta {
                name: "texture pin",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_PIN_FIELDS,
            }
        }
        WireCommand::TextureSetLabel => {
            WireCommandMeta {
                name: "texture set label",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_SET_LABEL_FIELDS,
            }
        }
        WireCommand::TextureSetOwnershipForMemoryDump => {
            WireCommandMeta {
                name: "texture set ownership for memory dump",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_SET_OWNERSHIP_FOR_MEMORY_DUMP_FIELDS,
            }
        }
        WireCommand::TextureUnpin => {
            WireCommandMeta {
                name: "texture unpin",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_UNPIN_FIELDS,
            }
        }
        WireCommand::TextureViewSetLabel => {
            WireCommandMeta {
                name: "texture view set label",
                direction: WireDirection::ClientToServer,
                fields: TEXTURE_VIEW_SET_LABEL_FIELDS,
            }
        }
        WireCommand::UnregisterObject => {
            WireCommandMeta {
                name: "unregister object",
                direction: WireDirection::ClientToServer,
                fields: UNREGISTER_OBJECT_FIELDS,
            }
        }
        WireCommand::ReturnAdapterRequestDeviceCallback => {
            WireCommandMeta {
                name: "adapter request device callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_ADAPTER_REQUEST_DEVICE_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnBufferMapAsyncCallback => {
            WireCommandMeta {
                name: "buffer map async callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_BUFFER_MAP_ASYNC_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnDeviceCreateComputePipelineAsyncCallback => {
            WireCommandMeta {
                name: "device create compute pipeline async callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_DEVICE_CREATE_COMPUTE_PIPELINE_ASYNC_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnDeviceCreateRenderPipelineAsyncCallback => {
            WireCommandMeta {
                name: "device create render pipeline async callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_DEVICE_CREATE_RENDER_PIPELINE_ASYNC_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnDeviceLoggingCallback => {
            WireCommandMeta {
                name: "device logging callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_DEVICE_LOGGING_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnDeviceLostCallback => {
            WireCommandMeta {
                name: "device lost callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_DEVICE_LOST_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnDevicePopErrorScopeCallback => {
            WireCommandMeta {
                name: "device pop error scope callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_DEVICE_POP_ERROR_SCOPE_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnDeviceUncapturedErrorCallback => {
            WireCommandMeta {
                name: "device uncaptured error callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_DEVICE_UNCAPTURED_ERROR_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnInstanceRequestAdapterCallback => {
            WireCommandMeta {
                name: "instance request adapter callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_INSTANCE_REQUEST_ADAPTER_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnQueueWorkDoneCallback => {
            WireCommandMeta {
                name: "queue work done callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_QUEUE_WORK_DONE_CALLBACK_FIELDS,
            }
        }
        WireCommand::ReturnShaderModuleGetCompilationInfoCallback => {
            WireCommandMeta {
                name: "shader module get compilation info callback",
                direction: WireDirection::ServerToClient,
                fields: RETURN_SHADER_MODULE_GET_COMPILATION_INFO_CALLBACK_FIELDS,
            }
        }
    }
}
pub fn wire_command_name(command: WireCommand) -> &'static str {
    wire_command_meta(command).name
}
pub fn wire_command_from_name(name: &str) -> Option<WireCommand> {
    match name {
        "chunked command" => Some(WireCommand::ChunkedCommand),
        "adapter request device" => Some(WireCommand::AdapterRequestDevice),
        "bind group layout set label" => Some(WireCommand::BindGroupLayoutSetLabel),
        "bind group set label" => Some(WireCommand::BindGroupSetLabel),
        "buffer create texel view" => Some(WireCommand::BufferCreateTexelView),
        "buffer map async" => Some(WireCommand::BufferMapAsync),
        "buffer set label" => Some(WireCommand::BufferSetLabel),
        "buffer update mapped data" => Some(WireCommand::BufferUpdateMappedData),
        "command buffer set label" => Some(WireCommand::CommandBufferSetLabel),
        "command encoder begin compute pass" => {
            Some(WireCommand::CommandEncoderBeginComputePass)
        }
        "command encoder begin render pass" => {
            Some(WireCommand::CommandEncoderBeginRenderPass)
        }
        "command encoder clear buffer" => Some(WireCommand::CommandEncoderClearBuffer),
        "command encoder copy buffer to buffer" => {
            Some(WireCommand::CommandEncoderCopyBufferToBuffer)
        }
        "command encoder copy buffer to texture" => {
            Some(WireCommand::CommandEncoderCopyBufferToTexture)
        }
        "command encoder copy texture to buffer" => {
            Some(WireCommand::CommandEncoderCopyTextureToBuffer)
        }
        "command encoder copy texture to texture" => {
            Some(WireCommand::CommandEncoderCopyTextureToTexture)
        }
        "command encoder finish" => Some(WireCommand::CommandEncoderFinish),
        "command encoder inject validation error" => {
            Some(WireCommand::CommandEncoderInjectValidationError)
        }
        "command encoder insert debug marker" => {
            Some(WireCommand::CommandEncoderInsertDebugMarker)
        }
        "command encoder pop debug group" => {
            Some(WireCommand::CommandEncoderPopDebugGroup)
        }
        "command encoder push debug group" => {
            Some(WireCommand::CommandEncoderPushDebugGroup)
        }
        "command encoder resolve query set" => {
            Some(WireCommand::CommandEncoderResolveQuerySet)
        }
        "command encoder set label" => Some(WireCommand::CommandEncoderSetLabel),
        "command encoder write buffer" => Some(WireCommand::CommandEncoderWriteBuffer),
        "command encoder write timestamp" => {
            Some(WireCommand::CommandEncoderWriteTimestamp)
        }
        "compute pass encoder dispatch workgroups" => {
            Some(WireCommand::ComputePassEncoderDispatchWorkgroups)
        }
        "compute pass encoder dispatch workgroups indirect" => {
            Some(WireCommand::ComputePassEncoderDispatchWorkgroupsIndirect)
        }
        "compute pass encoder end" => Some(WireCommand::ComputePassEncoderEnd),
        "compute pass encoder insert debug marker" => {
            Some(WireCommand::ComputePassEncoderInsertDebugMarker)
        }
        "compute pass encoder pop debug group" => {
            Some(WireCommand::ComputePassEncoderPopDebugGroup)
        }
        "compute pass encoder push debug group" => {
            Some(WireCommand::ComputePassEncoderPushDebugGroup)
        }
        "compute pass encoder set bind group" => {
            Some(WireCommand::ComputePassEncoderSetBindGroup)
        }
        "compute pass encoder set immediates" => {
            Some(WireCommand::ComputePassEncoderSetImmediates)
        }
        "compute pass encoder set label" => Some(WireCommand::ComputePassEncoderSetLabel),
        "compute pass encoder set pipeline" => {
            Some(WireCommand::ComputePassEncoderSetPipeline)
        }
        "compute pass encoder set resource table" => {
            Some(WireCommand::ComputePassEncoderSetResourceTable)
        }
        "compute pass encoder write timestamp" => {
            Some(WireCommand::ComputePassEncoderWriteTimestamp)
        }
        "compute pipeline get bind group layout" => {
            Some(WireCommand::ComputePipelineGetBindGroupLayout)
        }
        "compute pipeline set label" => Some(WireCommand::ComputePipelineSetLabel),
        "device create bind group" => Some(WireCommand::DeviceCreateBindGroup),
        "device create bind group layout" => {
            Some(WireCommand::DeviceCreateBindGroupLayout)
        }
        "device create buffer" => Some(WireCommand::DeviceCreateBuffer),
        "device create command encoder" => Some(WireCommand::DeviceCreateCommandEncoder),
        "device create compute pipeline" => {
            Some(WireCommand::DeviceCreateComputePipeline)
        }
        "device create compute pipeline async" => {
            Some(WireCommand::DeviceCreateComputePipelineAsync)
        }
        "device create error external texture" => {
            Some(WireCommand::DeviceCreateErrorExternalTexture)
        }
        "device create error shader module" => {
            Some(WireCommand::DeviceCreateErrorShaderModule)
        }
        "device create external texture" => {
            Some(WireCommand::DeviceCreateExternalTexture)
        }
        "device create pipeline layout" => Some(WireCommand::DeviceCreatePipelineLayout),
        "device create query set" => Some(WireCommand::DeviceCreateQuerySet),
        "device create render bundle encoder" => {
            Some(WireCommand::DeviceCreateRenderBundleEncoder)
        }
        "device create render pipeline" => Some(WireCommand::DeviceCreateRenderPipeline),
        "device create render pipeline async" => {
            Some(WireCommand::DeviceCreateRenderPipelineAsync)
        }
        "device create sampler" => Some(WireCommand::DeviceCreateSampler),
        "device create shader module" => Some(WireCommand::DeviceCreateShaderModule),
        "device force loss" => Some(WireCommand::DeviceForceLoss),
        "device get a hardware buffer properties" => {
            Some(WireCommand::DeviceGetAHardwareBufferProperties)
        }
        "device import shared buffer memory" => {
            Some(WireCommand::DeviceImportSharedBufferMemory)
        }
        "device import shared fence" => Some(WireCommand::DeviceImportSharedFence),
        "device import shared texture memory" => {
            Some(WireCommand::DeviceImportSharedTextureMemory)
        }
        "device pop error scope" => Some(WireCommand::DevicePopErrorScope),
        "device push error scope" => Some(WireCommand::DevicePushErrorScope),
        "device set label" => Some(WireCommand::DeviceSetLabel),
        "device tick" => Some(WireCommand::DeviceTick),
        "device validate texture descriptor" => {
            Some(WireCommand::DeviceValidateTextureDescriptor)
        }
        "external texture destroy" => Some(WireCommand::ExternalTextureDestroy),
        "external texture expire" => Some(WireCommand::ExternalTextureExpire),
        "external texture refresh" => Some(WireCommand::ExternalTextureRefresh),
        "external texture set label" => Some(WireCommand::ExternalTextureSetLabel),
        "instance request adapter" => Some(WireCommand::InstanceRequestAdapter),
        "pipeline layout set label" => Some(WireCommand::PipelineLayoutSetLabel),
        "query set destroy" => Some(WireCommand::QuerySetDestroy),
        "query set set label" => Some(WireCommand::QuerySetSetLabel),
        "queue copy external texture for browser" => {
            Some(WireCommand::QueueCopyExternalTextureForBrowser)
        }
        "queue copy texture for browser" => Some(WireCommand::QueueCopyTextureForBrowser),
        "queue on submitted work done" => Some(WireCommand::QueueOnSubmittedWorkDone),
        "queue set label" => Some(WireCommand::QueueSetLabel),
        "queue write buffer" => Some(WireCommand::QueueWriteBuffer),
        "queue write buffer xl" => Some(WireCommand::QueueWriteBufferXl),
        "queue write texture" => Some(WireCommand::QueueWriteTexture),
        "queue write texture xl" => Some(WireCommand::QueueWriteTextureXl),
        "render bundle encoder draw" => Some(WireCommand::RenderBundleEncoderDraw),
        "render bundle encoder draw indexed" => {
            Some(WireCommand::RenderBundleEncoderDrawIndexed)
        }
        "render bundle encoder draw indexed indirect" => {
            Some(WireCommand::RenderBundleEncoderDrawIndexedIndirect)
        }
        "render bundle encoder draw indirect" => {
            Some(WireCommand::RenderBundleEncoderDrawIndirect)
        }
        "render bundle encoder finish" => Some(WireCommand::RenderBundleEncoderFinish),
        "render bundle encoder insert debug marker" => {
            Some(WireCommand::RenderBundleEncoderInsertDebugMarker)
        }
        "render bundle encoder pop debug group" => {
            Some(WireCommand::RenderBundleEncoderPopDebugGroup)
        }
        "render bundle encoder push debug group" => {
            Some(WireCommand::RenderBundleEncoderPushDebugGroup)
        }
        "render bundle encoder set bind group" => {
            Some(WireCommand::RenderBundleEncoderSetBindGroup)
        }
        "render bundle encoder set immediates" => {
            Some(WireCommand::RenderBundleEncoderSetImmediates)
        }
        "render bundle encoder set index buffer" => {
            Some(WireCommand::RenderBundleEncoderSetIndexBuffer)
        }
        "render bundle encoder set label" => {
            Some(WireCommand::RenderBundleEncoderSetLabel)
        }
        "render bundle encoder set pipeline" => {
            Some(WireCommand::RenderBundleEncoderSetPipeline)
        }
        "render bundle encoder set resource table" => {
            Some(WireCommand::RenderBundleEncoderSetResourceTable)
        }
        "render bundle encoder set vertex buffer" => {
            Some(WireCommand::RenderBundleEncoderSetVertexBuffer)
        }
        "render bundle set label" => Some(WireCommand::RenderBundleSetLabel),
        "render pass encoder begin occlusion query" => {
            Some(WireCommand::RenderPassEncoderBeginOcclusionQuery)
        }
        "render pass encoder draw" => Some(WireCommand::RenderPassEncoderDraw),
        "render pass encoder draw indexed" => {
            Some(WireCommand::RenderPassEncoderDrawIndexed)
        }
        "render pass encoder draw indexed indirect" => {
            Some(WireCommand::RenderPassEncoderDrawIndexedIndirect)
        }
        "render pass encoder draw indirect" => {
            Some(WireCommand::RenderPassEncoderDrawIndirect)
        }
        "render pass encoder end" => Some(WireCommand::RenderPassEncoderEnd),
        "render pass encoder end occlusion query" => {
            Some(WireCommand::RenderPassEncoderEndOcclusionQuery)
        }
        "render pass encoder execute bundles" => {
            Some(WireCommand::RenderPassEncoderExecuteBundles)
        }
        "render pass encoder insert debug marker" => {
            Some(WireCommand::RenderPassEncoderInsertDebugMarker)
        }
        "render pass encoder multi draw indexed indirect" => {
            Some(WireCommand::RenderPassEncoderMultiDrawIndexedIndirect)
        }
        "render pass encoder multi draw indirect" => {
            Some(WireCommand::RenderPassEncoderMultiDrawIndirect)
        }
        "render pass encoder pixel local storage barrier" => {
            Some(WireCommand::RenderPassEncoderPixelLocalStorageBarrier)
        }
        "render pass encoder pop debug group" => {
            Some(WireCommand::RenderPassEncoderPopDebugGroup)
        }
        "render pass encoder push debug group" => {
            Some(WireCommand::RenderPassEncoderPushDebugGroup)
        }
        "render pass encoder set bind group" => {
            Some(WireCommand::RenderPassEncoderSetBindGroup)
        }
        "render pass encoder set blend constant" => {
            Some(WireCommand::RenderPassEncoderSetBlendConstant)
        }
        "render pass encoder set immediates" => {
            Some(WireCommand::RenderPassEncoderSetImmediates)
        }
        "render pass encoder set index buffer" => {
            Some(WireCommand::RenderPassEncoderSetIndexBuffer)
        }
        "render pass encoder set label" => Some(WireCommand::RenderPassEncoderSetLabel),
        "render pass encoder set pipeline" => {
            Some(WireCommand::RenderPassEncoderSetPipeline)
        }
        "render pass encoder set resource table" => {
            Some(WireCommand::RenderPassEncoderSetResourceTable)
        }
        "render pass encoder set scissor rect" => {
            Some(WireCommand::RenderPassEncoderSetScissorRect)
        }
        "render pass encoder set stencil reference" => {
            Some(WireCommand::RenderPassEncoderSetStencilReference)
        }
        "render pass encoder set vertex buffer" => {
            Some(WireCommand::RenderPassEncoderSetVertexBuffer)
        }
        "render pass encoder set viewport" => {
            Some(WireCommand::RenderPassEncoderSetViewport)
        }
        "render pass encoder write timestamp" => {
            Some(WireCommand::RenderPassEncoderWriteTimestamp)
        }
        "render pipeline get bind group layout" => {
            Some(WireCommand::RenderPipelineGetBindGroupLayout)
        }
        "render pipeline set label" => Some(WireCommand::RenderPipelineSetLabel),
        "sampler set label" => Some(WireCommand::SamplerSetLabel),
        "shader module get compilation info" => {
            Some(WireCommand::ShaderModuleGetCompilationInfo)
        }
        "shader module set label" => Some(WireCommand::ShaderModuleSetLabel),
        "shared buffer memory begin access" => {
            Some(WireCommand::SharedBufferMemoryBeginAccess)
        }
        "shared buffer memory create buffer" => {
            Some(WireCommand::SharedBufferMemoryCreateBuffer)
        }
        "shared buffer memory end access" => {
            Some(WireCommand::SharedBufferMemoryEndAccess)
        }
        "shared buffer memory get properties" => {
            Some(WireCommand::SharedBufferMemoryGetProperties)
        }
        "shared buffer memory set label" => Some(WireCommand::SharedBufferMemorySetLabel),
        "shared fence export info" => Some(WireCommand::SharedFenceExportInfo),
        "shared texture memory begin access" => {
            Some(WireCommand::SharedTextureMemoryBeginAccess)
        }
        "shared texture memory create texture" => {
            Some(WireCommand::SharedTextureMemoryCreateTexture)
        }
        "shared texture memory end access" => {
            Some(WireCommand::SharedTextureMemoryEndAccess)
        }
        "shared texture memory get properties" => {
            Some(WireCommand::SharedTextureMemoryGetProperties)
        }
        "shared texture memory set label" => {
            Some(WireCommand::SharedTextureMemorySetLabel)
        }
        "surface get current texture" => Some(WireCommand::SurfaceGetCurrentTexture),
        "surface set label" => Some(WireCommand::SurfaceSetLabel),
        "texel buffer view set label" => Some(WireCommand::TexelBufferViewSetLabel),
        "texture create error view" => Some(WireCommand::TextureCreateErrorView),
        "texture create view" => Some(WireCommand::TextureCreateView),
        "texture destroy" => Some(WireCommand::TextureDestroy),
        "texture pin" => Some(WireCommand::TexturePin),
        "texture set label" => Some(WireCommand::TextureSetLabel),
        "texture set ownership for memory dump" => {
            Some(WireCommand::TextureSetOwnershipForMemoryDump)
        }
        "texture unpin" => Some(WireCommand::TextureUnpin),
        "texture view set label" => Some(WireCommand::TextureViewSetLabel),
        "unregister object" => Some(WireCommand::UnregisterObject),
        "adapter request device callback" => {
            Some(WireCommand::ReturnAdapterRequestDeviceCallback)
        }
        "buffer map async callback" => Some(WireCommand::ReturnBufferMapAsyncCallback),
        "device create compute pipeline async callback" => {
            Some(WireCommand::ReturnDeviceCreateComputePipelineAsyncCallback)
        }
        "device create render pipeline async callback" => {
            Some(WireCommand::ReturnDeviceCreateRenderPipelineAsyncCallback)
        }
        "device logging callback" => Some(WireCommand::ReturnDeviceLoggingCallback),
        "device lost callback" => Some(WireCommand::ReturnDeviceLostCallback),
        "device pop error scope callback" => {
            Some(WireCommand::ReturnDevicePopErrorScopeCallback)
        }
        "device uncaptured error callback" => {
            Some(WireCommand::ReturnDeviceUncapturedErrorCallback)
        }
        "instance request adapter callback" => {
            Some(WireCommand::ReturnInstanceRequestAdapterCallback)
        }
        "queue work done callback" => Some(WireCommand::ReturnQueueWorkDoneCallback),
        "shader module get compilation info callback" => {
            Some(WireCommand::ReturnShaderModuleGetCompilationInfoCallback)
        }
        _ => None,
    }
}
pub fn build_packet(
    command: WireCommand,
    values: Vec<WireValue>,
) -> Result<WirePacket, String> {
    let fields = wire_command_meta(command).fields;
    if values.len() != fields.len() {
        return Err(
            format!(
                "field length mismatch for {}: got {}, expected {}",
                wire_command_name(command),
                values.len(),
                fields.len(),
            ),
        );
    }
    let fields = fields
        .iter()
        .zip(values)
        .map(|(name, value)| WireField { name, value })
        .collect();
    Ok(WirePacket { command, fields })
}
