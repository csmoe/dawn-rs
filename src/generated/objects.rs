#![allow(dead_code, unused_imports)]
use crate::ffi;
use super::*;
#[derive(Debug)]
pub struct Adapter {
    raw: ffi::WGPUAdapter,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl Adapter {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUAdapter) -> Self {
        Self {
            raw,
            _not_sync: std::marker::PhantomData,
        }
    }
    pub fn as_raw(&self) -> ffi::WGPUAdapter {
        self.raw
    }
    pub fn get_instance(&self) -> Instance {
        let result = unsafe { ffi::wgpuAdapterGetInstance(self.raw) };
        unsafe { Instance::from_raw(result) }
    }
    pub fn get_limits(&self, limits: &mut Limits) -> Status {
        let (mut limits_ffi, _limits_storage) = limits.to_ffi();
        let limits_ptr = std::ptr::addr_of_mut!(limits_ffi);
        let result = unsafe { ffi::wgpuAdapterGetLimits(self.raw, limits_ptr) };
        *limits = Limits::from_ffi(limits_ffi);
        result.into()
    }
    pub fn get_info(&self, info: &mut AdapterInfo) -> Status {
        let (mut info_ffi, _info_storage) = info.to_ffi();
        let info_ptr = std::ptr::addr_of_mut!(info_ffi);
        let result = unsafe { ffi::wgpuAdapterGetInfo(self.raw, info_ptr) };
        *info = AdapterInfo::from_ffi(info_ffi);
        result.into()
    }
    pub fn has_feature(&self, feature: FeatureName) -> bool {
        let feature_ffi: ffi::WGPUFeatureName = feature.into();
        let result = unsafe { ffi::wgpuAdapterHasFeature(self.raw, feature_ffi) };
        result != 0
    }
    pub fn get_features(&self, features: &mut SupportedFeatures) -> () {
        let (mut features_ffi, _features_storage) = features.to_ffi();
        let features_ptr = std::ptr::addr_of_mut!(features_ffi);
        unsafe { ffi::wgpuAdapterGetFeatures(self.raw, features_ptr) };
        *features = SupportedFeatures::from_ffi(features_ffi);
        ()
    }
    pub fn request_device(
        &self,
        descriptor: Option<&DeviceDescriptor>,
        callback: impl FnMut(
            RequestDeviceStatus,
            Option<Device>,
            String,
        ) + Send + 'static,
    ) -> Future {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let callback_box: RequestDeviceCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPURequestDeviceCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(request_device_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuAdapterRequestDevice(self.raw, descriptor_ptr, callback_info_ffi)
        };
        Future::from_ffi(result)
    }
    pub fn create_device(&self, descriptor: Option<&DeviceDescriptor>) -> Device {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe { ffi::wgpuAdapterCreateDevice(self.raw, descriptor_ptr) };
        unsafe { Device::from_raw(result) }
    }
    pub fn get_format_capabilities(
        &self,
        format: TextureFormat,
        capabilities: &mut DawnFormatCapabilities,
    ) -> Status {
        let format_ffi: ffi::WGPUTextureFormat = format.into();
        let (mut capabilities_ffi, _capabilities_storage) = capabilities.to_ffi();
        let capabilities_ptr = std::ptr::addr_of_mut!(capabilities_ffi);
        let result = unsafe {
            ffi::wgpuAdapterGetFormatCapabilities(self.raw, format_ffi, capabilities_ptr)
        };
        *capabilities = DawnFormatCapabilities::from_ffi(capabilities_ffi);
        result.into()
    }
}
impl Drop for Adapter {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuAdapterRelease(self.raw) };
    }
}
impl Clone for Adapter {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuAdapterAddRef(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct BindGroup {
    raw: ffi::WGPUBindGroup,
}
impl BindGroup {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUBindGroup) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUBindGroup {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuBindGroupSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for BindGroup {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuBindGroupRelease(self.raw) };
    }
}
impl Clone for BindGroup {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuBindGroupAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for BindGroup {}
unsafe impl Sync for BindGroup {}
#[derive(Debug)]
pub struct BindGroupLayout {
    raw: ffi::WGPUBindGroupLayout,
}
impl BindGroupLayout {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUBindGroupLayout) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUBindGroupLayout {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuBindGroupLayoutSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for BindGroupLayout {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuBindGroupLayoutRelease(self.raw) };
    }
}
impl Clone for BindGroupLayout {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuBindGroupLayoutAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for BindGroupLayout {}
unsafe impl Sync for BindGroupLayout {}
#[derive(Debug)]
pub struct Buffer {
    raw: ffi::WGPUBuffer,
}
impl Buffer {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUBuffer) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUBuffer {
        self.raw
    }
    pub fn map_async(
        &self,
        mode: MapMode,
        offset: usize,
        size: usize,
        callback: impl FnMut(MapAsyncStatus, String) + Send + 'static,
    ) -> Future {
        let mode_ffi: ffi::WGPUMapMode = mode.into();
        let callback_box: BufferMapCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPUBufferMapCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(buffer_map_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuBufferMapAsync(self.raw, mode_ffi, offset, size, callback_info_ffi)
        };
        Future::from_ffi(result)
    }
    pub fn get_mapped_range(&self, offset: usize, size: usize) -> *mut std::ffi::c_void {
        let result = unsafe { ffi::wgpuBufferGetMappedRange(self.raw, offset, size) };
        result
    }
    pub fn get_const_mapped_range(
        &self,
        offset: usize,
        size: usize,
    ) -> *const std::ffi::c_void {
        let result = unsafe {
            ffi::wgpuBufferGetConstMappedRange(self.raw, offset, size)
        };
        result
    }
    pub fn write_mapped_range(
        &self,
        offset: usize,
        data: &[std::ffi::c_void],
    ) -> Status {
        let data_ptr = data.as_ptr();
        let result = unsafe {
            ffi::wgpuBufferWriteMappedRange(self.raw, offset, data_ptr, data.len())
        };
        result.into()
    }
    pub fn read_mapped_range(
        &self,
        offset: usize,
        mut data: &mut [std::ffi::c_void],
    ) -> Status {
        let data_ptr = data.as_mut_ptr();
        let result = unsafe {
            ffi::wgpuBufferReadMappedRange(self.raw, offset, data_ptr, data.len())
        };
        result.into()
    }
    pub fn create_texel_view(
        &self,
        descriptor: &TexelBufferViewDescriptor,
    ) -> TexelBufferView {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe { ffi::wgpuBufferCreateTexelView(self.raw, descriptor_ptr) };
        unsafe { TexelBufferView::from_raw(result) }
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuBufferSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn get_usage(&self) -> BufferUsage {
        let result = unsafe { ffi::wgpuBufferGetUsage(self.raw) };
        result.into()
    }
    pub fn get_size(&self) -> u64 {
        let result = unsafe { ffi::wgpuBufferGetSize(self.raw) };
        result
    }
    pub fn get_map_state(&self) -> BufferMapState {
        let result = unsafe { ffi::wgpuBufferGetMapState(self.raw) };
        result.into()
    }
    pub fn unmap(&self) -> () {
        unsafe { ffi::wgpuBufferUnmap(self.raw) };
        ()
    }
    pub fn destroy(&self) -> () {
        unsafe { ffi::wgpuBufferDestroy(self.raw) };
        ()
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuBufferRelease(self.raw) };
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuBufferAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}
#[derive(Debug)]
pub struct CommandBuffer {
    raw: ffi::WGPUCommandBuffer,
}
impl CommandBuffer {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUCommandBuffer) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUCommandBuffer {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuCommandBufferSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for CommandBuffer {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuCommandBufferRelease(self.raw) };
    }
}
impl Clone for CommandBuffer {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuCommandBufferAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}
#[derive(Debug)]
pub struct CommandEncoder {
    raw: ffi::WGPUCommandEncoder,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl CommandEncoder {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUCommandEncoder) -> Self {
        Self {
            raw,
            _not_sync: std::marker::PhantomData,
        }
    }
    pub fn as_raw(&self) -> ffi::WGPUCommandEncoder {
        self.raw
    }
    pub fn finish(&self, descriptor: Option<&CommandBufferDescriptor>) -> CommandBuffer {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe { ffi::wgpuCommandEncoderFinish(self.raw, descriptor_ptr) };
        unsafe { CommandBuffer::from_raw(result) }
    }
    pub fn begin_compute_pass(
        &self,
        descriptor: Option<&ComputePassDescriptor>,
    ) -> ComputePassEncoder {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe {
            ffi::wgpuCommandEncoderBeginComputePass(self.raw, descriptor_ptr)
        };
        unsafe { ComputePassEncoder::from_raw(result) }
    }
    pub fn begin_render_pass(
        &self,
        descriptor: &RenderPassDescriptor,
    ) -> RenderPassEncoder {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuCommandEncoderBeginRenderPass(self.raw, descriptor_ptr)
        };
        unsafe { RenderPassEncoder::from_raw(result) }
    }
    pub fn copy_buffer_to_buffer(
        &self,
        source: Buffer,
        source_offset: u64,
        destination: Buffer,
        destination_offset: u64,
        size: u64,
    ) -> () {
        unsafe {
            ffi::wgpuCommandEncoderCopyBufferToBuffer(
                self.raw,
                source.as_raw(),
                source_offset,
                destination.as_raw(),
                destination_offset,
                size,
            )
        };
        ()
    }
    pub fn copy_buffer_to_texture(
        &self,
        source: &TexelCopyBufferInfo,
        destination: &TexelCopyTextureInfo,
        copy_size: &Extent3D,
    ) -> () {
        let (source_ffi, _source_storage) = source.to_ffi();
        let source_ptr = std::ptr::addr_of!(source_ffi);
        let (destination_ffi, _destination_storage) = destination.to_ffi();
        let destination_ptr = std::ptr::addr_of!(destination_ffi);
        let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
        let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
        unsafe {
            ffi::wgpuCommandEncoderCopyBufferToTexture(
                self.raw,
                source_ptr,
                destination_ptr,
                copy_size_ptr,
            )
        };
        ()
    }
    pub fn copy_texture_to_buffer(
        &self,
        source: &TexelCopyTextureInfo,
        destination: &TexelCopyBufferInfo,
        copy_size: &Extent3D,
    ) -> () {
        let (source_ffi, _source_storage) = source.to_ffi();
        let source_ptr = std::ptr::addr_of!(source_ffi);
        let (destination_ffi, _destination_storage) = destination.to_ffi();
        let destination_ptr = std::ptr::addr_of!(destination_ffi);
        let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
        let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
        unsafe {
            ffi::wgpuCommandEncoderCopyTextureToBuffer(
                self.raw,
                source_ptr,
                destination_ptr,
                copy_size_ptr,
            )
        };
        ()
    }
    pub fn copy_texture_to_texture(
        &self,
        source: &TexelCopyTextureInfo,
        destination: &TexelCopyTextureInfo,
        copy_size: &Extent3D,
    ) -> () {
        let (source_ffi, _source_storage) = source.to_ffi();
        let source_ptr = std::ptr::addr_of!(source_ffi);
        let (destination_ffi, _destination_storage) = destination.to_ffi();
        let destination_ptr = std::ptr::addr_of!(destination_ffi);
        let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
        let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
        unsafe {
            ffi::wgpuCommandEncoderCopyTextureToTexture(
                self.raw,
                source_ptr,
                destination_ptr,
                copy_size_ptr,
            )
        };
        ()
    }
    pub fn clear_buffer(&self, buffer: Buffer, offset: u64, size: u64) -> () {
        unsafe {
            ffi::wgpuCommandEncoderClearBuffer(self.raw, buffer.as_raw(), offset, size)
        };
        ()
    }
    pub fn inject_validation_error(&self, message: String) -> () {
        let message_ffi = ffi::WGPUStringView {
            data: message.as_ptr().cast(),
            length: message.len(),
        };
        unsafe { ffi::wgpuCommandEncoderInjectValidationError(self.raw, message_ffi) };
        ()
    }
    pub fn insert_debug_marker(&self, marker_label: String) -> () {
        let marker_label_ffi = ffi::WGPUStringView {
            data: marker_label.as_ptr().cast(),
            length: marker_label.len(),
        };
        unsafe { ffi::wgpuCommandEncoderInsertDebugMarker(self.raw, marker_label_ffi) };
        ()
    }
    pub fn pop_debug_group(&self) -> () {
        unsafe { ffi::wgpuCommandEncoderPopDebugGroup(self.raw) };
        ()
    }
    pub fn push_debug_group(&self, group_label: String) -> () {
        let group_label_ffi = ffi::WGPUStringView {
            data: group_label.as_ptr().cast(),
            length: group_label.len(),
        };
        unsafe { ffi::wgpuCommandEncoderPushDebugGroup(self.raw, group_label_ffi) };
        ()
    }
    pub fn resolve_query_set(
        &self,
        query_set: QuerySet,
        first_query: u32,
        query_count: u32,
        destination: Buffer,
        destination_offset: u64,
    ) -> () {
        unsafe {
            ffi::wgpuCommandEncoderResolveQuerySet(
                self.raw,
                query_set.as_raw(),
                first_query,
                query_count,
                destination.as_raw(),
                destination_offset,
            )
        };
        ()
    }
    pub fn write_buffer(&self, buffer: Buffer, buffer_offset: u64, data: &[u8]) -> () {
        let data_ptr = data.as_ptr();
        unsafe {
            ffi::wgpuCommandEncoderWriteBuffer(
                self.raw,
                buffer.as_raw(),
                buffer_offset,
                data_ptr,
                (data.len()) as u64,
            )
        };
        ()
    }
    pub fn write_timestamp(&self, query_set: QuerySet, query_index: u32) -> () {
        unsafe {
            ffi::wgpuCommandEncoderWriteTimestamp(
                self.raw,
                query_set.as_raw(),
                query_index,
            )
        };
        ()
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuCommandEncoderSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn set_resource_table(&self, table: Option<ResourceTable>) -> () {
        let table_raw = table
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        unsafe { ffi::wgpuCommandEncoderSetResourceTable(self.raw, table_raw) };
        ()
    }
}
impl Drop for CommandEncoder {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuCommandEncoderRelease(self.raw) };
    }
}
impl Clone for CommandEncoder {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuCommandEncoderAddRef(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct ComputePassEncoder {
    raw: ffi::WGPUComputePassEncoder,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl ComputePassEncoder {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUComputePassEncoder) -> Self {
        Self {
            raw,
            _not_sync: std::marker::PhantomData,
        }
    }
    pub fn as_raw(&self) -> ffi::WGPUComputePassEncoder {
        self.raw
    }
    pub fn insert_debug_marker(&self, marker_label: String) -> () {
        let marker_label_ffi = ffi::WGPUStringView {
            data: marker_label.as_ptr().cast(),
            length: marker_label.len(),
        };
        unsafe {
            ffi::wgpuComputePassEncoderInsertDebugMarker(self.raw, marker_label_ffi)
        };
        ()
    }
    pub fn pop_debug_group(&self) -> () {
        unsafe { ffi::wgpuComputePassEncoderPopDebugGroup(self.raw) };
        ()
    }
    pub fn push_debug_group(&self, group_label: String) -> () {
        let group_label_ffi = ffi::WGPUStringView {
            data: group_label.as_ptr().cast(),
            length: group_label.len(),
        };
        unsafe { ffi::wgpuComputePassEncoderPushDebugGroup(self.raw, group_label_ffi) };
        ()
    }
    pub fn set_pipeline(&self, pipeline: ComputePipeline) -> () {
        unsafe { ffi::wgpuComputePassEncoderSetPipeline(self.raw, pipeline.as_raw()) };
        ()
    }
    pub fn set_bind_group(
        &self,
        group_index: u32,
        group: Option<BindGroup>,
        dynamic_offsets: &[u32],
    ) -> () {
        let group_raw = group
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let dynamic_offsets_ptr = dynamic_offsets.as_ptr();
        unsafe {
            ffi::wgpuComputePassEncoderSetBindGroup(
                self.raw,
                group_index,
                group_raw,
                dynamic_offsets.len(),
                dynamic_offsets_ptr,
            )
        };
        ()
    }
    pub fn write_timestamp(&self, query_set: QuerySet, query_index: u32) -> () {
        unsafe {
            ffi::wgpuComputePassEncoderWriteTimestamp(
                self.raw,
                query_set.as_raw(),
                query_index,
            )
        };
        ()
    }
    pub fn dispatch_workgroups(
        &self,
        workgroup_count_x: u32,
        workgroup_count_y: u32,
        workgroup_count_z: u32,
    ) -> () {
        unsafe {
            ffi::wgpuComputePassEncoderDispatchWorkgroups(
                self.raw,
                workgroup_count_x,
                workgroup_count_y,
                workgroup_count_z,
            )
        };
        ()
    }
    pub fn dispatch_workgroups_indirect(
        &self,
        indirect_buffer: Buffer,
        indirect_offset: u64,
    ) -> () {
        unsafe {
            ffi::wgpuComputePassEncoderDispatchWorkgroupsIndirect(
                self.raw,
                indirect_buffer.as_raw(),
                indirect_offset,
            )
        };
        ()
    }
    pub fn end(&self) -> () {
        unsafe { ffi::wgpuComputePassEncoderEnd(self.raw) };
        ()
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuComputePassEncoderSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn set_immediates(&self, offset: u32, data: &[std::ffi::c_void]) -> () {
        let data_ptr = data.as_ptr();
        unsafe {
            ffi::wgpuComputePassEncoderSetImmediates(
                self.raw,
                offset,
                data_ptr,
                data.len(),
            )
        };
        ()
    }
}
impl Drop for ComputePassEncoder {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuComputePassEncoderRelease(self.raw) };
    }
}
impl Clone for ComputePassEncoder {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuComputePassEncoderAddRef(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct ComputePipeline {
    raw: ffi::WGPUComputePipeline,
}
impl ComputePipeline {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUComputePipeline) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUComputePipeline {
        self.raw
    }
    pub fn get_bind_group_layout(&self, group_index: u32) -> BindGroupLayout {
        let result = unsafe {
            ffi::wgpuComputePipelineGetBindGroupLayout(self.raw, group_index)
        };
        unsafe { BindGroupLayout::from_raw(result) }
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuComputePipelineSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for ComputePipeline {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuComputePipelineRelease(self.raw) };
    }
}
impl Clone for ComputePipeline {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuComputePipelineAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for ComputePipeline {}
unsafe impl Sync for ComputePipeline {}
#[derive(Debug)]
pub struct Device {
    raw: ffi::WGPUDevice,
}
impl Device {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUDevice) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUDevice {
        self.raw
    }
    pub fn create_bind_group(&self, descriptor: &BindGroupDescriptor) -> BindGroup {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe { ffi::wgpuDeviceCreateBindGroup(self.raw, descriptor_ptr) };
        unsafe { BindGroup::from_raw(result) }
    }
    pub fn create_bind_group_layout(
        &self,
        descriptor: &BindGroupLayoutDescriptor,
    ) -> BindGroupLayout {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateBindGroupLayout(self.raw, descriptor_ptr)
        };
        unsafe { BindGroupLayout::from_raw(result) }
    }
    pub fn create_buffer(&self, descriptor: &BufferDescriptor) -> Option<Buffer> {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe { ffi::wgpuDeviceCreateBuffer(self.raw, descriptor_ptr) };
        if result.is_null() { None } else { Some(unsafe { Buffer::from_raw(result) }) }
    }
    pub fn create_error_buffer(&self, descriptor: &BufferDescriptor) -> Buffer {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateErrorBuffer(self.raw, descriptor_ptr)
        };
        unsafe { Buffer::from_raw(result) }
    }
    pub fn create_command_encoder(
        &self,
        descriptor: Option<&CommandEncoderDescriptor>,
    ) -> CommandEncoder {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe {
            ffi::wgpuDeviceCreateCommandEncoder(self.raw, descriptor_ptr)
        };
        unsafe { CommandEncoder::from_raw(result) }
    }
    pub fn create_compute_pipeline(
        &self,
        descriptor: &ComputePipelineDescriptor,
    ) -> ComputePipeline {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateComputePipeline(self.raw, descriptor_ptr)
        };
        unsafe { ComputePipeline::from_raw(result) }
    }
    pub fn create_compute_pipeline_async(
        &self,
        descriptor: &ComputePipelineDescriptor,
        callback: impl FnMut(
            CreatePipelineAsyncStatus,
            Option<ComputePipeline>,
            String,
        ) + Send + 'static,
    ) -> Future {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let callback_box: CreateComputePipelineAsyncCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPUCreateComputePipelineAsyncCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(create_compute_pipeline_async_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuDeviceCreateComputePipelineAsync(
                self.raw,
                descriptor_ptr,
                callback_info_ffi,
            )
        };
        Future::from_ffi(result)
    }
    pub fn create_external_texture(
        &self,
        external_texture_descriptor: &ExternalTextureDescriptor,
    ) -> ExternalTexture {
        let (external_texture_descriptor_ffi, _external_texture_descriptor_storage) = external_texture_descriptor
            .to_ffi();
        let external_texture_descriptor_ptr = std::ptr::addr_of!(
            external_texture_descriptor_ffi
        );
        let result = unsafe {
            ffi::wgpuDeviceCreateExternalTexture(
                self.raw,
                external_texture_descriptor_ptr,
            )
        };
        unsafe { ExternalTexture::from_raw(result) }
    }
    pub fn create_error_external_texture(&self) -> ExternalTexture {
        let result = unsafe { ffi::wgpuDeviceCreateErrorExternalTexture(self.raw) };
        unsafe { ExternalTexture::from_raw(result) }
    }
    pub fn create_pipeline_layout(
        &self,
        descriptor: &PipelineLayoutDescriptor,
    ) -> PipelineLayout {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreatePipelineLayout(self.raw, descriptor_ptr)
        };
        unsafe { PipelineLayout::from_raw(result) }
    }
    pub fn create_query_set(&self, descriptor: &QuerySetDescriptor) -> QuerySet {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe { ffi::wgpuDeviceCreateQuerySet(self.raw, descriptor_ptr) };
        unsafe { QuerySet::from_raw(result) }
    }
    pub fn create_render_pipeline_async(
        &self,
        descriptor: &RenderPipelineDescriptor,
        callback: impl FnMut(
            CreatePipelineAsyncStatus,
            Option<RenderPipeline>,
            String,
        ) + Send + 'static,
    ) -> Future {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let callback_box: CreateRenderPipelineAsyncCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPUCreateRenderPipelineAsyncCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(create_render_pipeline_async_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuDeviceCreateRenderPipelineAsync(
                self.raw,
                descriptor_ptr,
                callback_info_ffi,
            )
        };
        Future::from_ffi(result)
    }
    pub fn create_render_bundle_encoder(
        &self,
        descriptor: &RenderBundleEncoderDescriptor,
    ) -> RenderBundleEncoder {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateRenderBundleEncoder(self.raw, descriptor_ptr)
        };
        unsafe { RenderBundleEncoder::from_raw(result) }
    }
    pub fn create_render_pipeline(
        &self,
        descriptor: &RenderPipelineDescriptor,
    ) -> RenderPipeline {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateRenderPipeline(self.raw, descriptor_ptr)
        };
        unsafe { RenderPipeline::from_raw(result) }
    }
    pub fn create_sampler(&self, descriptor: Option<&SamplerDescriptor>) -> Sampler {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe { ffi::wgpuDeviceCreateSampler(self.raw, descriptor_ptr) };
        unsafe { Sampler::from_raw(result) }
    }
    pub fn create_shader_module(
        &self,
        descriptor: &ShaderModuleDescriptor,
    ) -> ShaderModule {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateShaderModule(self.raw, descriptor_ptr)
        };
        unsafe { ShaderModule::from_raw(result) }
    }
    pub fn create_error_shader_module(
        &self,
        descriptor: &ShaderModuleDescriptor,
        error_message: String,
    ) -> ShaderModule {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let error_message_ffi = ffi::WGPUStringView {
            data: error_message.as_ptr().cast(),
            length: error_message.len(),
        };
        let result = unsafe {
            ffi::wgpuDeviceCreateErrorShaderModule(
                self.raw,
                descriptor_ptr,
                error_message_ffi,
            )
        };
        unsafe { ShaderModule::from_raw(result) }
    }
    pub fn create_texture(&self, descriptor: &TextureDescriptor) -> Texture {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe { ffi::wgpuDeviceCreateTexture(self.raw, descriptor_ptr) };
        unsafe { Texture::from_raw(result) }
    }
    pub fn create_resource_table(
        &self,
        descriptor: &ResourceTableDescriptor,
    ) -> ResourceTable {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateResourceTable(self.raw, descriptor_ptr)
        };
        unsafe { ResourceTable::from_raw(result) }
    }
    pub fn import_shared_buffer_memory(
        &self,
        descriptor: &SharedBufferMemoryDescriptor,
    ) -> SharedBufferMemory {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceImportSharedBufferMemory(self.raw, descriptor_ptr)
        };
        unsafe { SharedBufferMemory::from_raw(result) }
    }
    pub fn import_shared_texture_memory(
        &self,
        descriptor: &SharedTextureMemoryDescriptor,
    ) -> SharedTextureMemory {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceImportSharedTextureMemory(self.raw, descriptor_ptr)
        };
        unsafe { SharedTextureMemory::from_raw(result) }
    }
    pub fn import_shared_fence(
        &self,
        descriptor: &SharedFenceDescriptor,
    ) -> SharedFence {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceImportSharedFence(self.raw, descriptor_ptr)
        };
        unsafe { SharedFence::from_raw(result) }
    }
    pub fn create_error_texture(&self, descriptor: &TextureDescriptor) -> Texture {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuDeviceCreateErrorTexture(self.raw, descriptor_ptr)
        };
        unsafe { Texture::from_raw(result) }
    }
    pub fn destroy(&self) -> () {
        unsafe { ffi::wgpuDeviceDestroy(self.raw) };
        ()
    }
    pub fn get_a_hardware_buffer_properties(
        &self,
        handle: *mut std::ffi::c_void,
        properties: &mut AHardwareBufferProperties,
    ) -> Status {
        let (mut properties_ffi, _properties_storage) = properties.to_ffi();
        let properties_ptr = std::ptr::addr_of_mut!(properties_ffi);
        let result = unsafe {
            ffi::wgpuDeviceGetAHardwareBufferProperties(self.raw, handle, properties_ptr)
        };
        *properties = AHardwareBufferProperties::from_ffi(properties_ffi);
        result.into()
    }
    pub fn get_limits(&self, limits: &mut Limits) -> Status {
        let (mut limits_ffi, _limits_storage) = limits.to_ffi();
        let limits_ptr = std::ptr::addr_of_mut!(limits_ffi);
        let result = unsafe { ffi::wgpuDeviceGetLimits(self.raw, limits_ptr) };
        *limits = Limits::from_ffi(limits_ffi);
        result.into()
    }
    pub fn get_lost_future(&self) -> Future {
        let result = unsafe { ffi::wgpuDeviceGetLostFuture(self.raw) };
        Future::from_ffi(result)
    }
    pub fn has_feature(&self, feature: FeatureName) -> bool {
        let feature_ffi: ffi::WGPUFeatureName = feature.into();
        let result = unsafe { ffi::wgpuDeviceHasFeature(self.raw, feature_ffi) };
        result != 0
    }
    pub fn get_features(&self, features: &mut SupportedFeatures) -> () {
        let (mut features_ffi, _features_storage) = features.to_ffi();
        let features_ptr = std::ptr::addr_of_mut!(features_ffi);
        unsafe { ffi::wgpuDeviceGetFeatures(self.raw, features_ptr) };
        *features = SupportedFeatures::from_ffi(features_ffi);
        ()
    }
    pub fn get_adapter_info(&self, adapter_info: &mut AdapterInfo) -> Status {
        let (mut adapter_info_ffi, _adapter_info_storage) = adapter_info.to_ffi();
        let adapter_info_ptr = std::ptr::addr_of_mut!(adapter_info_ffi);
        let result = unsafe {
            ffi::wgpuDeviceGetAdapterInfo(self.raw, adapter_info_ptr)
        };
        *adapter_info = AdapterInfo::from_ffi(adapter_info_ffi);
        result.into()
    }
    pub fn get_adapter(&self) -> Adapter {
        let result = unsafe { ffi::wgpuDeviceGetAdapter(self.raw) };
        unsafe { Adapter::from_raw(result) }
    }
    pub fn get_queue(&self) -> Queue {
        let result = unsafe { ffi::wgpuDeviceGetQueue(self.raw) };
        unsafe { Queue::from_raw(result) }
    }
    pub fn inject_error(&self, r#type: ErrorType, message: String) -> () {
        let r#type_ffi: ffi::WGPUErrorType = r#type.into();
        let message_ffi = ffi::WGPUStringView {
            data: message.as_ptr().cast(),
            length: message.len(),
        };
        unsafe { ffi::wgpuDeviceInjectError(self.raw, r#type_ffi, message_ffi) };
        ()
    }
    pub fn force_loss(&self, r#type: DeviceLostReason, message: String) -> () {
        let r#type_ffi: ffi::WGPUDeviceLostReason = r#type.into();
        let message_ffi = ffi::WGPUStringView {
            data: message.as_ptr().cast(),
            length: message.len(),
        };
        unsafe { ffi::wgpuDeviceForceLoss(self.raw, r#type_ffi, message_ffi) };
        ()
    }
    pub fn tick(&self) -> () {
        unsafe { ffi::wgpuDeviceTick(self.raw) };
        ()
    }
    pub fn set_logging_callback(
        &self,
        callback: impl FnMut(LoggingType, String) + Send + 'static,
    ) -> () {
        let callback_box: LoggingCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPULoggingCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            callback: Some(logging_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        unsafe { ffi::wgpuDeviceSetLoggingCallback(self.raw, callback_info_ffi) };
        ()
    }
    pub fn push_error_scope(&self, filter: ErrorFilter) -> () {
        let filter_ffi: ffi::WGPUErrorFilter = filter.into();
        unsafe { ffi::wgpuDevicePushErrorScope(self.raw, filter_ffi) };
        ()
    }
    pub fn pop_error_scope(
        &self,
        callback: impl FnMut(PopErrorScopeStatus, ErrorType, String) + Send + 'static,
    ) -> Future {
        let callback_box: PopErrorScopeCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPUPopErrorScopeCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(pop_error_scope_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuDevicePopErrorScope(self.raw, callback_info_ffi)
        };
        Future::from_ffi(result)
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuDeviceSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn validate_texture_descriptor(&self, descriptor: &TextureDescriptor) -> () {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        unsafe { ffi::wgpuDeviceValidateTextureDescriptor(self.raw, descriptor_ptr) };
        ()
    }
}
impl Drop for Device {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuDeviceRelease(self.raw) };
    }
}
impl Clone for Device {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuDeviceAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for Device {}
unsafe impl Sync for Device {}
#[derive(Debug)]
pub struct ExternalTexture {
    raw: ffi::WGPUExternalTexture,
}
impl ExternalTexture {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUExternalTexture) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUExternalTexture {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuExternalTextureSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn destroy(&self) -> () {
        unsafe { ffi::wgpuExternalTextureDestroy(self.raw) };
        ()
    }
    pub fn expire(&self) -> () {
        unsafe { ffi::wgpuExternalTextureExpire(self.raw) };
        ()
    }
    pub fn refresh(&self) -> () {
        unsafe { ffi::wgpuExternalTextureRefresh(self.raw) };
        ()
    }
}
impl Drop for ExternalTexture {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuExternalTextureRelease(self.raw) };
    }
}
impl Clone for ExternalTexture {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuExternalTextureAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for ExternalTexture {}
unsafe impl Sync for ExternalTexture {}
#[derive(Debug)]
pub struct Instance {
    raw: ffi::WGPUInstance,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl Instance {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUInstance) -> Self {
        Self {
            raw,
            _not_sync: std::marker::PhantomData,
        }
    }
    pub fn as_raw(&self) -> ffi::WGPUInstance {
        self.raw
    }
    pub fn new(descriptor: Option<&InstanceDescriptor>) -> Self {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe { ffi::wgpuCreateInstance(descriptor_ptr) };
        unsafe { Instance::from_raw(result) }
    }
    pub fn create_surface(&self, descriptor: &SurfaceDescriptor) -> Surface {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe { ffi::wgpuInstanceCreateSurface(self.raw, descriptor_ptr) };
        unsafe { Surface::from_raw(result) }
    }
    pub fn process_events(&self) -> () {
        unsafe { ffi::wgpuInstanceProcessEvents(self.raw) };
        ()
    }
    pub fn wait_any(
        &self,
        mut futures: Option<&mut [FutureWaitInfo]>,
        timeout_ns: u64,
    ) -> WaitStatus {
        let mut futures_raw: Vec<ffi::WGPUFutureWaitInfo> = Vec::new();
        let mut futures_storage: Vec<ChainedStructStorage> = Vec::new();
        let futures_ptr = if let Some(value) = futures.as_deref() {
            for item in value {
                let (raw, storage) = item.to_ffi();
                futures_raw.push(raw);
                futures_storage.push(storage);
            }
            futures_raw.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };
        let result = unsafe {
            ffi::wgpuInstanceWaitAny(
                self.raw,
                futures.as_deref().map(|v| v.len()).unwrap_or(0),
                futures_ptr,
                timeout_ns,
            )
        };
        result.into()
    }
    pub fn request_adapter(
        &self,
        options: Option<&RequestAdapterOptions>,
        callback: impl FnMut(
            RequestAdapterStatus,
            Option<Adapter>,
            String,
        ) + Send + 'static,
    ) -> Future {
        let mut options_storage = ChainedStructStorage::new();
        let options_ptr = if let Some(value) = &options {
            let (options_ffi, storage) = value.to_ffi();
            options_storage = storage;
            std::ptr::addr_of!(options_ffi)
        } else {
            std::ptr::null()
        };
        let callback_box: RequestAdapterCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPURequestAdapterCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(request_adapter_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuInstanceRequestAdapter(self.raw, options_ptr, callback_info_ffi)
        };
        Future::from_ffi(result)
    }
    pub fn has_wgsl_language_feature(&self, feature: WGSLLanguageFeatureName) -> bool {
        let feature_ffi: ffi::WGPUWGSLLanguageFeatureName = feature.into();
        let result = unsafe {
            ffi::wgpuInstanceHasWGSLLanguageFeature(self.raw, feature_ffi)
        };
        result != 0
    }
    pub fn get_wgsl_language_features(
        &self,
        features: &mut SupportedWGSLLanguageFeatures,
    ) -> () {
        let (mut features_ffi, _features_storage) = features.to_ffi();
        let features_ptr = std::ptr::addr_of_mut!(features_ffi);
        unsafe { ffi::wgpuInstanceGetWGSLLanguageFeatures(self.raw, features_ptr) };
        *features = SupportedWGSLLanguageFeatures::from_ffi(features_ffi);
        ()
    }
}
impl Drop for Instance {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuInstanceRelease(self.raw) };
    }
}
impl Clone for Instance {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuInstanceAddRef(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct PipelineLayout {
    raw: ffi::WGPUPipelineLayout,
}
impl PipelineLayout {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUPipelineLayout) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUPipelineLayout {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuPipelineLayoutSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for PipelineLayout {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuPipelineLayoutRelease(self.raw) };
    }
}
impl Clone for PipelineLayout {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuPipelineLayoutAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for PipelineLayout {}
unsafe impl Sync for PipelineLayout {}
#[derive(Debug)]
pub struct QuerySet {
    raw: ffi::WGPUQuerySet,
}
impl QuerySet {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUQuerySet) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUQuerySet {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuQuerySetSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn get_type(&self) -> QueryType {
        let result = unsafe { ffi::wgpuQuerySetGetType(self.raw) };
        result.into()
    }
    pub fn get_count(&self) -> u32 {
        let result = unsafe { ffi::wgpuQuerySetGetCount(self.raw) };
        result
    }
    pub fn destroy(&self) -> () {
        unsafe { ffi::wgpuQuerySetDestroy(self.raw) };
        ()
    }
}
impl Drop for QuerySet {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuQuerySetRelease(self.raw) };
    }
}
impl Clone for QuerySet {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuQuerySetAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for QuerySet {}
unsafe impl Sync for QuerySet {}
#[derive(Debug)]
pub struct Queue {
    raw: ffi::WGPUQueue,
}
impl Queue {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUQueue) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUQueue {
        self.raw
    }
    pub fn submit(&self, commands: &[CommandBuffer]) -> () {
        let mut commands_raw: Vec<ffi::WGPUCommandBuffer> = commands
            .iter()
            .map(|v| v.as_raw())
            .collect();
        let commands_ptr = commands_raw.as_ptr();
        unsafe { ffi::wgpuQueueSubmit(self.raw, commands.len(), commands_ptr) };
        ()
    }
    pub fn on_submitted_work_done(
        &self,
        callback: impl FnMut(QueueWorkDoneStatus, String) + Send + 'static,
    ) -> Future {
        let callback_box: QueueWorkDoneCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPUQueueWorkDoneCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(queue_work_done_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuQueueOnSubmittedWorkDone(self.raw, callback_info_ffi)
        };
        Future::from_ffi(result)
    }
    pub fn write_buffer(
        &self,
        buffer: Buffer,
        buffer_offset: u64,
        data: &[std::ffi::c_void],
    ) -> () {
        let data_ptr = data.as_ptr();
        unsafe {
            ffi::wgpuQueueWriteBuffer(
                self.raw,
                buffer.as_raw(),
                buffer_offset,
                data_ptr,
                data.len(),
            )
        };
        ()
    }
    pub fn write_texture(
        &self,
        destination: &TexelCopyTextureInfo,
        data: &[std::ffi::c_void],
        data_layout: &TexelCopyBufferLayout,
        write_size: &Extent3D,
    ) -> () {
        let (destination_ffi, _destination_storage) = destination.to_ffi();
        let destination_ptr = std::ptr::addr_of!(destination_ffi);
        let data_ptr = data.as_ptr();
        let (data_layout_ffi, _data_layout_storage) = data_layout.to_ffi();
        let data_layout_ptr = std::ptr::addr_of!(data_layout_ffi);
        let (write_size_ffi, _write_size_storage) = write_size.to_ffi();
        let write_size_ptr = std::ptr::addr_of!(write_size_ffi);
        unsafe {
            ffi::wgpuQueueWriteTexture(
                self.raw,
                destination_ptr,
                data_ptr,
                data.len(),
                data_layout_ptr,
                write_size_ptr,
            )
        };
        ()
    }
    pub fn copy_texture_for_browser(
        &self,
        source: &TexelCopyTextureInfo,
        destination: &TexelCopyTextureInfo,
        copy_size: &Extent3D,
        options: &CopyTextureForBrowserOptions,
    ) -> () {
        let (source_ffi, _source_storage) = source.to_ffi();
        let source_ptr = std::ptr::addr_of!(source_ffi);
        let (destination_ffi, _destination_storage) = destination.to_ffi();
        let destination_ptr = std::ptr::addr_of!(destination_ffi);
        let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
        let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
        let (options_ffi, _options_storage) = options.to_ffi();
        let options_ptr = std::ptr::addr_of!(options_ffi);
        unsafe {
            ffi::wgpuQueueCopyTextureForBrowser(
                self.raw,
                source_ptr,
                destination_ptr,
                copy_size_ptr,
                options_ptr,
            )
        };
        ()
    }
    pub fn copy_external_texture_for_browser(
        &self,
        source: &ImageCopyExternalTexture,
        destination: &TexelCopyTextureInfo,
        copy_size: &Extent3D,
        options: &CopyTextureForBrowserOptions,
    ) -> () {
        let (source_ffi, _source_storage) = source.to_ffi();
        let source_ptr = std::ptr::addr_of!(source_ffi);
        let (destination_ffi, _destination_storage) = destination.to_ffi();
        let destination_ptr = std::ptr::addr_of!(destination_ffi);
        let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
        let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
        let (options_ffi, _options_storage) = options.to_ffi();
        let options_ptr = std::ptr::addr_of!(options_ffi);
        unsafe {
            ffi::wgpuQueueCopyExternalTextureForBrowser(
                self.raw,
                source_ptr,
                destination_ptr,
                copy_size_ptr,
                options_ptr,
            )
        };
        ()
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuQueueSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for Queue {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuQueueRelease(self.raw) };
    }
}
impl Clone for Queue {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuQueueAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}
#[derive(Debug)]
pub struct RenderBundle {
    raw: ffi::WGPURenderBundle,
}
impl RenderBundle {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderBundle) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPURenderBundle {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuRenderBundleSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for RenderBundle {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuRenderBundleRelease(self.raw) };
    }
}
impl Clone for RenderBundle {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuRenderBundleAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for RenderBundle {}
unsafe impl Sync for RenderBundle {}
#[derive(Debug)]
pub struct RenderBundleEncoder {
    raw: ffi::WGPURenderBundleEncoder,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl RenderBundleEncoder {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderBundleEncoder) -> Self {
        Self {
            raw,
            _not_sync: std::marker::PhantomData,
        }
    }
    pub fn as_raw(&self) -> ffi::WGPURenderBundleEncoder {
        self.raw
    }
    pub fn set_pipeline(&self, pipeline: RenderPipeline) -> () {
        unsafe { ffi::wgpuRenderBundleEncoderSetPipeline(self.raw, pipeline.as_raw()) };
        ()
    }
    pub fn set_bind_group(
        &self,
        group_index: u32,
        group: Option<BindGroup>,
        dynamic_offsets: &[u32],
    ) -> () {
        let group_raw = group
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let dynamic_offsets_ptr = dynamic_offsets.as_ptr();
        unsafe {
            ffi::wgpuRenderBundleEncoderSetBindGroup(
                self.raw,
                group_index,
                group_raw,
                dynamic_offsets.len(),
                dynamic_offsets_ptr,
            )
        };
        ()
    }
    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> () {
        unsafe {
            ffi::wgpuRenderBundleEncoderDraw(
                self.raw,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        };
        ()
    }
    pub fn draw_indexed(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    ) -> () {
        unsafe {
            ffi::wgpuRenderBundleEncoderDrawIndexed(
                self.raw,
                index_count,
                instance_count,
                first_index,
                base_vertex,
                first_instance,
            )
        };
        ()
    }
    pub fn draw_indirect(&self, indirect_buffer: Buffer, indirect_offset: u64) -> () {
        unsafe {
            ffi::wgpuRenderBundleEncoderDrawIndirect(
                self.raw,
                indirect_buffer.as_raw(),
                indirect_offset,
            )
        };
        ()
    }
    pub fn draw_indexed_indirect(
        &self,
        indirect_buffer: Buffer,
        indirect_offset: u64,
    ) -> () {
        unsafe {
            ffi::wgpuRenderBundleEncoderDrawIndexedIndirect(
                self.raw,
                indirect_buffer.as_raw(),
                indirect_offset,
            )
        };
        ()
    }
    pub fn insert_debug_marker(&self, marker_label: String) -> () {
        let marker_label_ffi = ffi::WGPUStringView {
            data: marker_label.as_ptr().cast(),
            length: marker_label.len(),
        };
        unsafe {
            ffi::wgpuRenderBundleEncoderInsertDebugMarker(self.raw, marker_label_ffi)
        };
        ()
    }
    pub fn pop_debug_group(&self) -> () {
        unsafe { ffi::wgpuRenderBundleEncoderPopDebugGroup(self.raw) };
        ()
    }
    pub fn push_debug_group(&self, group_label: String) -> () {
        let group_label_ffi = ffi::WGPUStringView {
            data: group_label.as_ptr().cast(),
            length: group_label.len(),
        };
        unsafe { ffi::wgpuRenderBundleEncoderPushDebugGroup(self.raw, group_label_ffi) };
        ()
    }
    pub fn set_vertex_buffer(
        &self,
        slot: u32,
        buffer: Option<Buffer>,
        offset: u64,
        size: u64,
    ) -> () {
        let buffer_raw = buffer
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        unsafe {
            ffi::wgpuRenderBundleEncoderSetVertexBuffer(
                self.raw,
                slot,
                buffer_raw,
                offset,
                size,
            )
        };
        ()
    }
    pub fn set_index_buffer(
        &self,
        buffer: Buffer,
        format: IndexFormat,
        offset: u64,
        size: u64,
    ) -> () {
        let format_ffi: ffi::WGPUIndexFormat = format.into();
        unsafe {
            ffi::wgpuRenderBundleEncoderSetIndexBuffer(
                self.raw,
                buffer.as_raw(),
                format_ffi,
                offset,
                size,
            )
        };
        ()
    }
    pub fn finish(&self, descriptor: Option<&RenderBundleDescriptor>) -> RenderBundle {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe {
            ffi::wgpuRenderBundleEncoderFinish(self.raw, descriptor_ptr)
        };
        unsafe { RenderBundle::from_raw(result) }
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuRenderBundleEncoderSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn set_immediates(&self, offset: u32, data: &[std::ffi::c_void]) -> () {
        let data_ptr = data.as_ptr();
        unsafe {
            ffi::wgpuRenderBundleEncoderSetImmediates(
                self.raw,
                offset,
                data_ptr,
                data.len(),
            )
        };
        ()
    }
}
impl Drop for RenderBundleEncoder {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuRenderBundleEncoderRelease(self.raw) };
    }
}
impl Clone for RenderBundleEncoder {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuRenderBundleEncoderAddRef(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct RenderPassEncoder {
    raw: ffi::WGPURenderPassEncoder,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl RenderPassEncoder {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderPassEncoder) -> Self {
        Self {
            raw,
            _not_sync: std::marker::PhantomData,
        }
    }
    pub fn as_raw(&self) -> ffi::WGPURenderPassEncoder {
        self.raw
    }
    pub fn set_pipeline(&self, pipeline: RenderPipeline) -> () {
        unsafe { ffi::wgpuRenderPassEncoderSetPipeline(self.raw, pipeline.as_raw()) };
        ()
    }
    pub fn set_bind_group(
        &self,
        group_index: u32,
        group: Option<BindGroup>,
        dynamic_offsets: &[u32],
    ) -> () {
        let group_raw = group
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let dynamic_offsets_ptr = dynamic_offsets.as_ptr();
        unsafe {
            ffi::wgpuRenderPassEncoderSetBindGroup(
                self.raw,
                group_index,
                group_raw,
                dynamic_offsets.len(),
                dynamic_offsets_ptr,
            )
        };
        ()
    }
    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> () {
        unsafe {
            ffi::wgpuRenderPassEncoderDraw(
                self.raw,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        };
        ()
    }
    pub fn draw_indexed(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    ) -> () {
        unsafe {
            ffi::wgpuRenderPassEncoderDrawIndexed(
                self.raw,
                index_count,
                instance_count,
                first_index,
                base_vertex,
                first_instance,
            )
        };
        ()
    }
    pub fn draw_indirect(&self, indirect_buffer: Buffer, indirect_offset: u64) -> () {
        unsafe {
            ffi::wgpuRenderPassEncoderDrawIndirect(
                self.raw,
                indirect_buffer.as_raw(),
                indirect_offset,
            )
        };
        ()
    }
    pub fn draw_indexed_indirect(
        &self,
        indirect_buffer: Buffer,
        indirect_offset: u64,
    ) -> () {
        unsafe {
            ffi::wgpuRenderPassEncoderDrawIndexedIndirect(
                self.raw,
                indirect_buffer.as_raw(),
                indirect_offset,
            )
        };
        ()
    }
    pub fn multi_draw_indirect(
        &self,
        indirect_buffer: Buffer,
        indirect_offset: u64,
        max_draw_count: u32,
        draw_count_buffer: Option<Buffer>,
        draw_count_buffer_offset: u64,
    ) -> () {
        let draw_count_buffer_raw = draw_count_buffer
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        unsafe {
            ffi::wgpuRenderPassEncoderMultiDrawIndirect(
                self.raw,
                indirect_buffer.as_raw(),
                indirect_offset,
                max_draw_count,
                draw_count_buffer_raw,
                draw_count_buffer_offset,
            )
        };
        ()
    }
    pub fn multi_draw_indexed_indirect(
        &self,
        indirect_buffer: Buffer,
        indirect_offset: u64,
        max_draw_count: u32,
        draw_count_buffer: Option<Buffer>,
        draw_count_buffer_offset: u64,
    ) -> () {
        let draw_count_buffer_raw = draw_count_buffer
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        unsafe {
            ffi::wgpuRenderPassEncoderMultiDrawIndexedIndirect(
                self.raw,
                indirect_buffer.as_raw(),
                indirect_offset,
                max_draw_count,
                draw_count_buffer_raw,
                draw_count_buffer_offset,
            )
        };
        ()
    }
    pub fn execute_bundles(&self, bundles: &[RenderBundle]) -> () {
        let mut bundles_raw: Vec<ffi::WGPURenderBundle> = bundles
            .iter()
            .map(|v| v.as_raw())
            .collect();
        let bundles_ptr = bundles_raw.as_ptr();
        unsafe {
            ffi::wgpuRenderPassEncoderExecuteBundles(
                self.raw,
                bundles.len(),
                bundles_ptr,
            )
        };
        ()
    }
    pub fn insert_debug_marker(&self, marker_label: String) -> () {
        let marker_label_ffi = ffi::WGPUStringView {
            data: marker_label.as_ptr().cast(),
            length: marker_label.len(),
        };
        unsafe {
            ffi::wgpuRenderPassEncoderInsertDebugMarker(self.raw, marker_label_ffi)
        };
        ()
    }
    pub fn pop_debug_group(&self) -> () {
        unsafe { ffi::wgpuRenderPassEncoderPopDebugGroup(self.raw) };
        ()
    }
    pub fn push_debug_group(&self, group_label: String) -> () {
        let group_label_ffi = ffi::WGPUStringView {
            data: group_label.as_ptr().cast(),
            length: group_label.len(),
        };
        unsafe { ffi::wgpuRenderPassEncoderPushDebugGroup(self.raw, group_label_ffi) };
        ()
    }
    pub fn set_stencil_reference(&self, reference: u32) -> () {
        unsafe { ffi::wgpuRenderPassEncoderSetStencilReference(self.raw, reference) };
        ()
    }
    pub fn set_blend_constant(&self, color: &Color) -> () {
        let (color_ffi, _color_storage) = color.to_ffi();
        let color_ptr = std::ptr::addr_of!(color_ffi);
        unsafe { ffi::wgpuRenderPassEncoderSetBlendConstant(self.raw, color_ptr) };
        ()
    }
    pub fn set_viewport(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) -> () {
        unsafe {
            ffi::wgpuRenderPassEncoderSetViewport(
                self.raw,
                x,
                y,
                width,
                height,
                min_depth,
                max_depth,
            )
        };
        ()
    }
    pub fn set_scissor_rect(&self, x: u32, y: u32, width: u32, height: u32) -> () {
        unsafe {
            ffi::wgpuRenderPassEncoderSetScissorRect(self.raw, x, y, width, height)
        };
        ()
    }
    pub fn set_vertex_buffer(
        &self,
        slot: u32,
        buffer: Option<Buffer>,
        offset: u64,
        size: u64,
    ) -> () {
        let buffer_raw = buffer
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        unsafe {
            ffi::wgpuRenderPassEncoderSetVertexBuffer(
                self.raw,
                slot,
                buffer_raw,
                offset,
                size,
            )
        };
        ()
    }
    pub fn set_index_buffer(
        &self,
        buffer: Buffer,
        format: IndexFormat,
        offset: u64,
        size: u64,
    ) -> () {
        let format_ffi: ffi::WGPUIndexFormat = format.into();
        unsafe {
            ffi::wgpuRenderPassEncoderSetIndexBuffer(
                self.raw,
                buffer.as_raw(),
                format_ffi,
                offset,
                size,
            )
        };
        ()
    }
    pub fn begin_occlusion_query(&self, query_index: u32) -> () {
        unsafe { ffi::wgpuRenderPassEncoderBeginOcclusionQuery(self.raw, query_index) };
        ()
    }
    pub fn end_occlusion_query(&self) -> () {
        unsafe { ffi::wgpuRenderPassEncoderEndOcclusionQuery(self.raw) };
        ()
    }
    pub fn write_timestamp(&self, query_set: QuerySet, query_index: u32) -> () {
        unsafe {
            ffi::wgpuRenderPassEncoderWriteTimestamp(
                self.raw,
                query_set.as_raw(),
                query_index,
            )
        };
        ()
    }
    pub fn pixel_local_storage_barrier(&self) -> () {
        unsafe { ffi::wgpuRenderPassEncoderPixelLocalStorageBarrier(self.raw) };
        ()
    }
    pub fn end(&self) -> () {
        unsafe { ffi::wgpuRenderPassEncoderEnd(self.raw) };
        ()
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuRenderPassEncoderSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn set_immediates(&self, offset: u32, data: &[std::ffi::c_void]) -> () {
        let data_ptr = data.as_ptr();
        unsafe {
            ffi::wgpuRenderPassEncoderSetImmediates(
                self.raw,
                offset,
                data_ptr,
                data.len(),
            )
        };
        ()
    }
}
impl Drop for RenderPassEncoder {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuRenderPassEncoderRelease(self.raw) };
    }
}
impl Clone for RenderPassEncoder {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuRenderPassEncoderAddRef(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct RenderPipeline {
    raw: ffi::WGPURenderPipeline,
}
impl RenderPipeline {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderPipeline) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPURenderPipeline {
        self.raw
    }
    pub fn get_bind_group_layout(&self, group_index: u32) -> BindGroupLayout {
        let result = unsafe {
            ffi::wgpuRenderPipelineGetBindGroupLayout(self.raw, group_index)
        };
        unsafe { BindGroupLayout::from_raw(result) }
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuRenderPipelineSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for RenderPipeline {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuRenderPipelineRelease(self.raw) };
    }
}
impl Clone for RenderPipeline {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuRenderPipelineAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for RenderPipeline {}
unsafe impl Sync for RenderPipeline {}
#[derive(Debug)]
pub struct ResourceTable {
    raw: ffi::WGPUResourceTable,
}
impl ResourceTable {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUResourceTable) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUResourceTable {
        self.raw
    }
    pub fn get_size(&self) -> u32 {
        let result = unsafe { ffi::wgpuResourceTableGetSize(self.raw) };
        result
    }
    pub fn destroy(&self) -> () {
        unsafe { ffi::wgpuResourceTableDestroy(self.raw) };
        ()
    }
    pub fn update(&self, slot: u32, resource: &BindingResource) -> Status {
        let (resource_ffi, _resource_storage) = resource.to_ffi();
        let resource_ptr = std::ptr::addr_of!(resource_ffi);
        let result = unsafe {
            ffi::wgpuResourceTableUpdate(self.raw, slot, resource_ptr)
        };
        result.into()
    }
    pub fn insert_binding(&self, resource: &BindingResource) -> u32 {
        let (resource_ffi, _resource_storage) = resource.to_ffi();
        let resource_ptr = std::ptr::addr_of!(resource_ffi);
        let result = unsafe {
            ffi::wgpuResourceTableInsertBinding(self.raw, resource_ptr)
        };
        result
    }
    pub fn remove_binding(&self, slot: u32) -> Status {
        let result = unsafe { ffi::wgpuResourceTableRemoveBinding(self.raw, slot) };
        result.into()
    }
}
impl Drop for ResourceTable {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuResourceTableRelease(self.raw) };
    }
}
impl Clone for ResourceTable {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuResourceTableAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for ResourceTable {}
unsafe impl Sync for ResourceTable {}
#[derive(Debug)]
pub struct Sampler {
    raw: ffi::WGPUSampler,
}
impl Sampler {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUSampler) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUSampler {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuSamplerSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for Sampler {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuSamplerRelease(self.raw) };
    }
}
impl Clone for Sampler {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuSamplerAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for Sampler {}
unsafe impl Sync for Sampler {}
#[derive(Debug)]
pub struct ShaderModule {
    raw: ffi::WGPUShaderModule,
}
impl ShaderModule {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUShaderModule) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUShaderModule {
        self.raw
    }
    pub fn get_compilation_info(
        &self,
        callback: impl FnMut(
            CompilationInfoRequestStatus,
            &CompilationInfo,
        ) + Send + 'static,
    ) -> Future {
        let callback_box: CompilationInfoCallback = Box::new(callback);
        let callback_box = Box::new(Some(callback_box));
        let callback_userdata = Box::into_raw(callback_box).cast::<std::ffi::c_void>();
        let callback_info_ffi = ffi::WGPUCompilationInfoCallbackInfo {
            nextInChain: std::ptr::null_mut(),
            mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
            callback: Some(compilation_info_callback_trampoline),
            userdata1: callback_userdata,
            userdata2: std::ptr::null_mut(),
        };
        let result = unsafe {
            ffi::wgpuShaderModuleGetCompilationInfo(self.raw, callback_info_ffi)
        };
        Future::from_ffi(result)
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuShaderModuleSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for ShaderModule {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuShaderModuleRelease(self.raw) };
    }
}
impl Clone for ShaderModule {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuShaderModuleAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for ShaderModule {}
unsafe impl Sync for ShaderModule {}
#[derive(Debug)]
pub struct SharedBufferMemory {
    raw: ffi::WGPUSharedBufferMemory,
}
impl SharedBufferMemory {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUSharedBufferMemory) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUSharedBufferMemory {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuSharedBufferMemorySetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn get_properties(
        &self,
        properties: &mut SharedBufferMemoryProperties,
    ) -> Status {
        let (mut properties_ffi, _properties_storage) = properties.to_ffi();
        let properties_ptr = std::ptr::addr_of_mut!(properties_ffi);
        let result = unsafe {
            ffi::wgpuSharedBufferMemoryGetProperties(self.raw, properties_ptr)
        };
        *properties = SharedBufferMemoryProperties::from_ffi(properties_ffi);
        result.into()
    }
    pub fn create_buffer(&self, descriptor: Option<&BufferDescriptor>) -> Buffer {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe {
            ffi::wgpuSharedBufferMemoryCreateBuffer(self.raw, descriptor_ptr)
        };
        unsafe { Buffer::from_raw(result) }
    }
    pub fn begin_access(
        &self,
        buffer: Buffer,
        descriptor: &SharedBufferMemoryBeginAccessDescriptor,
    ) -> Status {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuSharedBufferMemoryBeginAccess(
                self.raw,
                buffer.as_raw(),
                descriptor_ptr,
            )
        };
        result.into()
    }
    pub fn end_access(
        &self,
        buffer: Buffer,
        descriptor: &mut SharedBufferMemoryEndAccessState,
    ) -> Status {
        let (mut descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of_mut!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuSharedBufferMemoryEndAccess(
                self.raw,
                buffer.as_raw(),
                descriptor_ptr,
            )
        };
        *descriptor = SharedBufferMemoryEndAccessState::from_ffi(descriptor_ffi);
        result.into()
    }
    pub fn is_device_lost(&self) -> bool {
        let result = unsafe { ffi::wgpuSharedBufferMemoryIsDeviceLost(self.raw) };
        result != 0
    }
}
impl Drop for SharedBufferMemory {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuSharedBufferMemoryRelease(self.raw) };
    }
}
impl Clone for SharedBufferMemory {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuSharedBufferMemoryAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for SharedBufferMemory {}
unsafe impl Sync for SharedBufferMemory {}
#[derive(Debug)]
pub struct SharedFence {
    raw: ffi::WGPUSharedFence,
}
impl SharedFence {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUSharedFence) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUSharedFence {
        self.raw
    }
    pub fn export_info(&self, info: &mut SharedFenceExportInfo) -> () {
        let (mut info_ffi, _info_storage) = info.to_ffi();
        let info_ptr = std::ptr::addr_of_mut!(info_ffi);
        unsafe { ffi::wgpuSharedFenceExportInfo(self.raw, info_ptr) };
        *info = SharedFenceExportInfo::from_ffi(info_ffi);
        ()
    }
}
impl Drop for SharedFence {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuSharedFenceRelease(self.raw) };
    }
}
impl Clone for SharedFence {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuSharedFenceAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for SharedFence {}
unsafe impl Sync for SharedFence {}
#[derive(Debug)]
pub struct SharedTextureMemory {
    raw: ffi::WGPUSharedTextureMemory,
}
impl SharedTextureMemory {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUSharedTextureMemory) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUSharedTextureMemory {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuSharedTextureMemorySetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn get_properties(
        &self,
        properties: &mut SharedTextureMemoryProperties,
    ) -> Status {
        let (mut properties_ffi, _properties_storage) = properties.to_ffi();
        let properties_ptr = std::ptr::addr_of_mut!(properties_ffi);
        let result = unsafe {
            ffi::wgpuSharedTextureMemoryGetProperties(self.raw, properties_ptr)
        };
        *properties = SharedTextureMemoryProperties::from_ffi(properties_ffi);
        result.into()
    }
    pub fn create_texture(&self, descriptor: Option<&TextureDescriptor>) -> Texture {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe {
            ffi::wgpuSharedTextureMemoryCreateTexture(self.raw, descriptor_ptr)
        };
        unsafe { Texture::from_raw(result) }
    }
    pub fn begin_access(
        &self,
        texture: Texture,
        descriptor: &SharedTextureMemoryBeginAccessDescriptor,
    ) -> Status {
        let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuSharedTextureMemoryBeginAccess(
                self.raw,
                texture.as_raw(),
                descriptor_ptr,
            )
        };
        result.into()
    }
    pub fn end_access(
        &self,
        texture: Texture,
        descriptor: &mut SharedTextureMemoryEndAccessState,
    ) -> Status {
        let (mut descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
        let descriptor_ptr = std::ptr::addr_of_mut!(descriptor_ffi);
        let result = unsafe {
            ffi::wgpuSharedTextureMemoryEndAccess(
                self.raw,
                texture.as_raw(),
                descriptor_ptr,
            )
        };
        *descriptor = SharedTextureMemoryEndAccessState::from_ffi(descriptor_ffi);
        result.into()
    }
    pub fn is_device_lost(&self) -> bool {
        let result = unsafe { ffi::wgpuSharedTextureMemoryIsDeviceLost(self.raw) };
        result != 0
    }
}
impl Drop for SharedTextureMemory {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuSharedTextureMemoryRelease(self.raw) };
    }
}
impl Clone for SharedTextureMemory {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuSharedTextureMemoryAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for SharedTextureMemory {}
unsafe impl Sync for SharedTextureMemory {}
#[derive(Debug)]
pub struct Surface {
    raw: ffi::WGPUSurface,
    _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl Surface {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUSurface) -> Self {
        Self {
            raw,
            _not_sync: std::marker::PhantomData,
        }
    }
    pub fn as_raw(&self) -> ffi::WGPUSurface {
        self.raw
    }
    pub fn configure(&self, config: &SurfaceConfiguration) -> () {
        let (config_ffi, _config_storage) = config.to_ffi();
        let config_ptr = std::ptr::addr_of!(config_ffi);
        unsafe { ffi::wgpuSurfaceConfigure(self.raw, config_ptr) };
        ()
    }
    pub fn get_capabilities(
        &self,
        adapter: Adapter,
        capabilities: &mut SurfaceCapabilities,
    ) -> Status {
        let (mut capabilities_ffi, _capabilities_storage) = capabilities.to_ffi();
        let capabilities_ptr = std::ptr::addr_of_mut!(capabilities_ffi);
        let result = unsafe {
            ffi::wgpuSurfaceGetCapabilities(self.raw, adapter.as_raw(), capabilities_ptr)
        };
        *capabilities = SurfaceCapabilities::from_ffi(capabilities_ffi);
        result.into()
    }
    pub fn get_current_texture(&self, surface_texture: &mut SurfaceTexture) -> () {
        let (mut surface_texture_ffi, _surface_texture_storage) = surface_texture
            .to_ffi();
        let surface_texture_ptr = std::ptr::addr_of_mut!(surface_texture_ffi);
        unsafe { ffi::wgpuSurfaceGetCurrentTexture(self.raw, surface_texture_ptr) };
        *surface_texture = SurfaceTexture::from_ffi(surface_texture_ffi);
        ()
    }
    pub fn present(&self) -> Status {
        let result = unsafe { ffi::wgpuSurfacePresent(self.raw) };
        result.into()
    }
    pub fn unconfigure(&self) -> () {
        unsafe { ffi::wgpuSurfaceUnconfigure(self.raw) };
        ()
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuSurfaceSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for Surface {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuSurfaceRelease(self.raw) };
    }
}
impl Clone for Surface {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuSurfaceAddRef(self.raw) };
        Self {
            raw: self.raw,
            _not_sync: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct TexelBufferView {
    raw: ffi::WGPUTexelBufferView,
}
impl TexelBufferView {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUTexelBufferView) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUTexelBufferView {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuTexelBufferViewSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for TexelBufferView {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuTexelBufferViewRelease(self.raw) };
    }
}
impl Clone for TexelBufferView {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuTexelBufferViewAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for TexelBufferView {}
unsafe impl Sync for TexelBufferView {}
#[derive(Debug)]
pub struct Texture {
    raw: ffi::WGPUTexture,
}
impl Texture {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUTexture) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUTexture {
        self.raw
    }
    pub fn create_view(
        &self,
        descriptor: Option<&TextureViewDescriptor>,
    ) -> TextureView {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe { ffi::wgpuTextureCreateView(self.raw, descriptor_ptr) };
        unsafe { TextureView::from_raw(result) }
    }
    pub fn create_error_view(
        &self,
        descriptor: Option<&TextureViewDescriptor>,
    ) -> TextureView {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe {
            ffi::wgpuTextureCreateErrorView(self.raw, descriptor_ptr)
        };
        unsafe { TextureView::from_raw(result) }
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuTextureSetLabel(self.raw, label_ffi) };
        ()
    }
    pub fn get_width(&self) -> u32 {
        let result = unsafe { ffi::wgpuTextureGetWidth(self.raw) };
        result
    }
    pub fn get_height(&self) -> u32 {
        let result = unsafe { ffi::wgpuTextureGetHeight(self.raw) };
        result
    }
    pub fn get_depth_or_array_layers(&self) -> u32 {
        let result = unsafe { ffi::wgpuTextureGetDepthOrArrayLayers(self.raw) };
        result
    }
    pub fn get_mip_level_count(&self) -> u32 {
        let result = unsafe { ffi::wgpuTextureGetMipLevelCount(self.raw) };
        result
    }
    pub fn get_sample_count(&self) -> u32 {
        let result = unsafe { ffi::wgpuTextureGetSampleCount(self.raw) };
        result
    }
    pub fn get_dimension(&self) -> TextureDimension {
        let result = unsafe { ffi::wgpuTextureGetDimension(self.raw) };
        result.into()
    }
    pub fn get_format(&self) -> TextureFormat {
        let result = unsafe { ffi::wgpuTextureGetFormat(self.raw) };
        result.into()
    }
    pub fn get_usage(&self) -> TextureUsage {
        let result = unsafe { ffi::wgpuTextureGetUsage(self.raw) };
        result.into()
    }
    pub fn get_texture_binding_view_dimension(&self) -> TextureViewDimension {
        let result = unsafe { ffi::wgpuTextureGetTextureBindingViewDimension(self.raw) };
        result.into()
    }
    pub fn destroy(&self) -> () {
        unsafe { ffi::wgpuTextureDestroy(self.raw) };
        ()
    }
    pub fn pin(&self, usage: TextureUsage) -> () {
        let usage_ffi: ffi::WGPUTextureUsage = usage.into();
        unsafe { ffi::wgpuTexturePin(self.raw, usage_ffi) };
        ()
    }
    pub fn unpin(&self) -> () {
        unsafe { ffi::wgpuTextureUnpin(self.raw) };
        ()
    }
    pub fn set_ownership_for_memory_dump(&self, owner_guid: u64) -> () {
        unsafe { ffi::wgpuTextureSetOwnershipForMemoryDump(self.raw, owner_guid) };
        ()
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuTextureRelease(self.raw) };
    }
}
impl Clone for Texture {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuTextureAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for Texture {}
unsafe impl Sync for Texture {}
#[derive(Debug)]
pub struct TextureView {
    raw: ffi::WGPUTextureView,
}
impl TextureView {
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUTextureView) -> Self {
        Self { raw }
    }
    pub fn as_raw(&self) -> ffi::WGPUTextureView {
        self.raw
    }
    pub fn set_label(&self, label: String) -> () {
        let label_ffi = ffi::WGPUStringView {
            data: label.as_ptr().cast(),
            length: label.len(),
        };
        unsafe { ffi::wgpuTextureViewSetLabel(self.raw, label_ffi) };
        ()
    }
}
impl Drop for TextureView {
    fn drop(&mut self) {
        if self.as_raw().is_null() {
            return;
        }
        unsafe { ffi::wgpuTextureViewRelease(self.raw) };
    }
}
impl Clone for TextureView {
    fn clone(&self) -> Self {
        unsafe { ffi::wgpuTextureViewAddRef(self.raw) };
        Self { raw: self.raw }
    }
}
unsafe impl Send for TextureView {}
unsafe impl Sync for TextureView {}
