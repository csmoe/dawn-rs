#![allow(dead_code)]
use super::wire_types::{
    build_packet, wire_command_name, WireCommand, WirePacket, WireValue,
};
#[derive(Debug)]
pub enum WireClientError {
    Encode(String),
    Transport(String),
    UnexpectedCommand(&'static str),
}
pub trait WireClientTransport {
    fn send(&mut self, packet: WirePacket) -> Result<(), String>;
}
pub trait WireReturnHandler {
    fn on_adapter_request_device_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_buffer_map_async_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_device_create_compute_pipeline_async_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_device_create_render_pipeline_async_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_device_logging_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_device_lost_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_device_pop_error_scope_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_device_uncaptured_error_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_instance_request_adapter_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_queue_work_done_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
    fn on_shader_module_get_compilation_info_callback(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireClientError> {
        Ok(())
    }
}
pub struct WireClient<T: WireClientTransport> {
    transport: T,
}
impl<T: WireClientTransport> WireClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }
    pub fn send_command(
        &mut self,
        command: WireCommand,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(command, values).map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn chunked_command(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ChunkedCommand, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn bind_group_layout_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::BindGroupLayoutSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn bind_group_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::BindGroupSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn buffer_create_texel_view(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::BufferCreateTexelView, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn buffer_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::BufferSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn buffer_update_mapped_data(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::BufferUpdateMappedData, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_buffer_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandBufferSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_begin_compute_pass(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderBeginComputePass, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_begin_render_pass(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderBeginRenderPass, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_clear_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderClearBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_copy_buffer_to_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderCopyBufferToBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_copy_buffer_to_texture(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderCopyBufferToTexture, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_copy_texture_to_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderCopyTextureToBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_copy_texture_to_texture(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::CommandEncoderCopyTextureToTexture,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_finish(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderFinish, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_inject_validation_error(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::CommandEncoderInjectValidationError,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_insert_debug_marker(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderInsertDebugMarker, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_pop_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderPopDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_push_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderPushDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_resolve_query_set(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderResolveQuerySet, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_write_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderWriteBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn command_encoder_write_timestamp(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::CommandEncoderWriteTimestamp, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_dispatch_workgroups(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::ComputePassEncoderDispatchWorkgroups,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_dispatch_workgroups_indirect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::ComputePassEncoderDispatchWorkgroupsIndirect,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_end(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePassEncoderEnd, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_insert_debug_marker(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::ComputePassEncoderInsertDebugMarker,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_pop_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePassEncoderPopDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_push_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePassEncoderPushDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_set_bind_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePassEncoderSetBindGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePassEncoderSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_set_pipeline(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePassEncoderSetPipeline, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_set_resource_table(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::ComputePassEncoderSetResourceTable,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pass_encoder_write_timestamp(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePassEncoderWriteTimestamp, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pipeline_get_bind_group_layout(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePipelineGetBindGroupLayout, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn compute_pipeline_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ComputePipelineSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_bind_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateBindGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_bind_group_layout(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateBindGroupLayout, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_command_encoder(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateCommandEncoder, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_compute_pipeline(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateComputePipeline, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_error_external_texture(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateErrorExternalTexture, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_error_shader_module(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateErrorShaderModule, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_external_texture(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateExternalTexture, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_pipeline_layout(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreatePipelineLayout, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_query_set(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateQuerySet, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_render_bundle_encoder(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateRenderBundleEncoder, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_render_pipeline(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateRenderPipeline, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_sampler(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateSampler, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_create_shader_module(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceCreateShaderModule, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_force_loss(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceForceLoss, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_get_a_hardware_buffer_properties(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::DeviceGetAHardwareBufferProperties,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_import_shared_buffer_memory(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceImportSharedBufferMemory, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_import_shared_fence(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceImportSharedFence, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_import_shared_texture_memory(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceImportSharedTextureMemory, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_push_error_scope(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DevicePushErrorScope, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_tick(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceTick, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn device_validate_texture_descriptor(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::DeviceValidateTextureDescriptor, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn external_texture_destroy(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ExternalTextureDestroy, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn external_texture_expire(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ExternalTextureExpire, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn external_texture_refresh(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ExternalTextureRefresh, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn external_texture_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ExternalTextureSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn pipeline_layout_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::PipelineLayoutSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn query_set_destroy(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::QuerySetDestroy, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn query_set_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::QuerySetSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn queue_copy_external_texture_for_browser(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::QueueCopyExternalTextureForBrowser,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn queue_copy_texture_for_browser(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::QueueCopyTextureForBrowser, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn queue_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::QueueSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn queue_write_buffer_xl(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::QueueWriteBufferXl, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn queue_write_texture_xl(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::QueueWriteTextureXl, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_draw(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderDraw, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_draw_indexed(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderDrawIndexed, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_draw_indexed_indirect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderBundleEncoderDrawIndexedIndirect,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_draw_indirect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderDrawIndirect, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_finish(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderFinish, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_insert_debug_marker(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderBundleEncoderInsertDebugMarker,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_pop_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderPopDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_push_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderPushDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_set_bind_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderSetBindGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_set_index_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderSetIndexBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_set_pipeline(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleEncoderSetPipeline, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_set_resource_table(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderBundleEncoderSetResourceTable,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_encoder_set_vertex_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderBundleEncoderSetVertexBuffer,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_bundle_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderBundleSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_begin_occlusion_query(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderBeginOcclusionQuery,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_draw(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderDraw, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_draw_indexed(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderDrawIndexed, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_draw_indexed_indirect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderDrawIndexedIndirect,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_draw_indirect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderDrawIndirect, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_end(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderEnd, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_end_occlusion_query(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderEndOcclusionQuery,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_execute_bundles(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderExecuteBundles, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_insert_debug_marker(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderInsertDebugMarker,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_multi_draw_indexed_indirect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderMultiDrawIndexedIndirect,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_multi_draw_indirect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderMultiDrawIndirect,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_pixel_local_storage_barrier(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderPixelLocalStorageBarrier,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_pop_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderPopDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_push_debug_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderPushDebugGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_bind_group(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetBindGroup, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_blend_constant(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetBlendConstant, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_index_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetIndexBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_pipeline(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetPipeline, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_resource_table(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetResourceTable, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_scissor_rect(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetScissorRect, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_stencil_reference(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(
                WireCommand::RenderPassEncoderSetStencilReference,
                values,
            )
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_vertex_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetVertexBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_set_viewport(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderSetViewport, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pass_encoder_write_timestamp(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPassEncoderWriteTimestamp, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pipeline_get_bind_group_layout(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPipelineGetBindGroupLayout, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn render_pipeline_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::RenderPipelineSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn sampler_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SamplerSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shader_module_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::ShaderModuleSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_buffer_memory_begin_access(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedBufferMemoryBeginAccess, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_buffer_memory_create_buffer(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedBufferMemoryCreateBuffer, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_buffer_memory_end_access(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedBufferMemoryEndAccess, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_buffer_memory_get_properties(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedBufferMemoryGetProperties, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_buffer_memory_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedBufferMemorySetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_fence_export_info(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedFenceExportInfo, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_texture_memory_begin_access(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedTextureMemoryBeginAccess, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_texture_memory_create_texture(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedTextureMemoryCreateTexture, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_texture_memory_end_access(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedTextureMemoryEndAccess, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_texture_memory_get_properties(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedTextureMemoryGetProperties, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn shared_texture_memory_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SharedTextureMemorySetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn surface_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::SurfaceSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texel_buffer_view_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TexelBufferViewSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_create_error_view(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TextureCreateErrorView, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_create_view(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TextureCreateView, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_destroy(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TextureDestroy, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_pin(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TexturePin, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TextureSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_set_ownership_for_memory_dump(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TextureSetOwnershipForMemoryDump, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_unpin(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TextureUnpin, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn texture_view_set_label(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::TextureViewSetLabel, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn unregister_object(
        &mut self,
        values: Vec<WireValue>,
    ) -> Result<(), WireClientError> {
        let packet = build_packet(WireCommand::UnregisterObject, values)
            .map_err(WireClientError::Encode)?;
        self.transport.send(packet).map_err(WireClientError::Transport)
    }
    pub fn handle_return_packet<H: WireReturnHandler>(
        &mut self,
        packet: WirePacket,
        handler: &mut H,
    ) -> Result<(), WireClientError> {
        match packet.command {
            WireCommand::ReturnAdapterRequestDeviceCallback => {
                handler.on_adapter_request_device_callback(&packet)
            }
            WireCommand::ReturnBufferMapAsyncCallback => {
                handler.on_buffer_map_async_callback(&packet)
            }
            WireCommand::ReturnDeviceCreateComputePipelineAsyncCallback => {
                handler.on_device_create_compute_pipeline_async_callback(&packet)
            }
            WireCommand::ReturnDeviceCreateRenderPipelineAsyncCallback => {
                handler.on_device_create_render_pipeline_async_callback(&packet)
            }
            WireCommand::ReturnDeviceLoggingCallback => {
                handler.on_device_logging_callback(&packet)
            }
            WireCommand::ReturnDeviceLostCallback => {
                handler.on_device_lost_callback(&packet)
            }
            WireCommand::ReturnDevicePopErrorScopeCallback => {
                handler.on_device_pop_error_scope_callback(&packet)
            }
            WireCommand::ReturnDeviceUncapturedErrorCallback => {
                handler.on_device_uncaptured_error_callback(&packet)
            }
            WireCommand::ReturnInstanceRequestAdapterCallback => {
                handler.on_instance_request_adapter_callback(&packet)
            }
            WireCommand::ReturnQueueWorkDoneCallback => {
                handler.on_queue_work_done_callback(&packet)
            }
            WireCommand::ReturnShaderModuleGetCompilationInfoCallback => {
                handler.on_shader_module_get_compilation_info_callback(&packet)
            }
            other => Err(WireClientError::UnexpectedCommand(wire_command_name(other))),
        }
    }
}
