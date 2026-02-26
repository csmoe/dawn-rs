#![allow(dead_code)]
use super::wire_types::{wire_command_name, WireCommand, WirePacket};
#[derive(Debug)]
pub enum WireServerError {
    Handler(String),
    UnexpectedCommand(&'static str),
}
pub trait WireServerDoer {
    fn do_chunked_command(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("chunked command"))
    }
    fn do_adapter_request_device(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("adapter request device"))
    }
    fn do_bind_group_layout_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("bind group layout set label"))
    }
    fn do_bind_group_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("bind group set label"))
    }
    fn do_buffer_create_texel_view(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("buffer create texel view"))
    }
    fn do_buffer_map_async(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("buffer map async"))
    }
    fn do_buffer_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("buffer set label"))
    }
    fn do_buffer_update_mapped_data(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("buffer update mapped data"))
    }
    fn do_command_buffer_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command buffer set label"))
    }
    fn do_command_encoder_begin_compute_pass(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder begin compute pass"))
    }
    fn do_command_encoder_begin_render_pass(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder begin render pass"))
    }
    fn do_command_encoder_clear_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder clear buffer"))
    }
    fn do_command_encoder_copy_buffer_to_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder copy buffer to buffer"))
    }
    fn do_command_encoder_copy_buffer_to_texture(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder copy buffer to texture"))
    }
    fn do_command_encoder_copy_texture_to_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder copy texture to buffer"))
    }
    fn do_command_encoder_copy_texture_to_texture(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("command encoder copy texture to texture"),
        )
    }
    fn do_command_encoder_finish(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder finish"))
    }
    fn do_command_encoder_inject_validation_error(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("command encoder inject validation error"),
        )
    }
    fn do_command_encoder_insert_debug_marker(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder insert debug marker"))
    }
    fn do_command_encoder_pop_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder pop debug group"))
    }
    fn do_command_encoder_push_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder push debug group"))
    }
    fn do_command_encoder_resolve_query_set(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder resolve query set"))
    }
    fn do_command_encoder_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder set label"))
    }
    fn do_command_encoder_write_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder write buffer"))
    }
    fn do_command_encoder_write_timestamp(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("command encoder write timestamp"))
    }
    fn do_compute_pass_encoder_dispatch_workgroups(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "compute pass encoder dispatch workgroups",
            ),
        )
    }
    fn do_compute_pass_encoder_dispatch_workgroups_indirect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "compute pass encoder dispatch workgroups indirect",
            ),
        )
    }
    fn do_compute_pass_encoder_end(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder end"))
    }
    fn do_compute_pass_encoder_insert_debug_marker(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "compute pass encoder insert debug marker",
            ),
        )
    }
    fn do_compute_pass_encoder_pop_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder pop debug group"))
    }
    fn do_compute_pass_encoder_push_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder push debug group"))
    }
    fn do_compute_pass_encoder_set_bind_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder set bind group"))
    }
    fn do_compute_pass_encoder_set_immediates(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder set immediates"))
    }
    fn do_compute_pass_encoder_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder set label"))
    }
    fn do_compute_pass_encoder_set_pipeline(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder set pipeline"))
    }
    fn do_compute_pass_encoder_set_resource_table(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("compute pass encoder set resource table"),
        )
    }
    fn do_compute_pass_encoder_write_timestamp(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pass encoder write timestamp"))
    }
    fn do_compute_pipeline_get_bind_group_layout(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pipeline get bind group layout"))
    }
    fn do_compute_pipeline_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("compute pipeline set label"))
    }
    fn do_device_create_bind_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create bind group"))
    }
    fn do_device_create_bind_group_layout(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create bind group layout"))
    }
    fn do_device_create_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create buffer"))
    }
    fn do_device_create_command_encoder(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create command encoder"))
    }
    fn do_device_create_compute_pipeline(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create compute pipeline"))
    }
    fn do_device_create_compute_pipeline_async(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create compute pipeline async"))
    }
    fn do_device_create_error_external_texture(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create error external texture"))
    }
    fn do_device_create_error_shader_module(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create error shader module"))
    }
    fn do_device_create_external_texture(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create external texture"))
    }
    fn do_device_create_pipeline_layout(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create pipeline layout"))
    }
    fn do_device_create_query_set(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create query set"))
    }
    fn do_device_create_render_bundle_encoder(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create render bundle encoder"))
    }
    fn do_device_create_render_pipeline(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create render pipeline"))
    }
    fn do_device_create_render_pipeline_async(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create render pipeline async"))
    }
    fn do_device_create_sampler(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create sampler"))
    }
    fn do_device_create_shader_module(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device create shader module"))
    }
    fn do_device_force_loss(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device force loss"))
    }
    fn do_device_get_a_hardware_buffer_properties(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("device get a hardware buffer properties"),
        )
    }
    fn do_device_import_shared_buffer_memory(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device import shared buffer memory"))
    }
    fn do_device_import_shared_fence(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device import shared fence"))
    }
    fn do_device_import_shared_texture_memory(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device import shared texture memory"))
    }
    fn do_device_pop_error_scope(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device pop error scope"))
    }
    fn do_device_push_error_scope(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device push error scope"))
    }
    fn do_device_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device set label"))
    }
    fn do_device_tick(&mut self, _packet: &WirePacket) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device tick"))
    }
    fn do_device_validate_texture_descriptor(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("device validate texture descriptor"))
    }
    fn do_external_texture_destroy(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("external texture destroy"))
    }
    fn do_external_texture_expire(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("external texture expire"))
    }
    fn do_external_texture_refresh(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("external texture refresh"))
    }
    fn do_external_texture_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("external texture set label"))
    }
    fn do_instance_request_adapter(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("instance request adapter"))
    }
    fn do_pipeline_layout_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("pipeline layout set label"))
    }
    fn do_query_set_destroy(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("query set destroy"))
    }
    fn do_query_set_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("query set set label"))
    }
    fn do_queue_copy_external_texture_for_browser(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("queue copy external texture for browser"),
        )
    }
    fn do_queue_copy_texture_for_browser(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("queue copy texture for browser"))
    }
    fn do_queue_on_submitted_work_done(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("queue on submitted work done"))
    }
    fn do_queue_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("queue set label"))
    }
    fn do_queue_write_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("queue write buffer"))
    }
    fn do_queue_write_buffer_xl(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("queue write buffer xl"))
    }
    fn do_queue_write_texture(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("queue write texture"))
    }
    fn do_queue_write_texture_xl(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("queue write texture xl"))
    }
    fn do_render_bundle_encoder_draw(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder draw"))
    }
    fn do_render_bundle_encoder_draw_indexed(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder draw indexed"))
    }
    fn do_render_bundle_encoder_draw_indexed_indirect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render bundle encoder draw indexed indirect",
            ),
        )
    }
    fn do_render_bundle_encoder_draw_indirect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder draw indirect"))
    }
    fn do_render_bundle_encoder_finish(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder finish"))
    }
    fn do_render_bundle_encoder_insert_debug_marker(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render bundle encoder insert debug marker",
            ),
        )
    }
    fn do_render_bundle_encoder_pop_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder pop debug group"))
    }
    fn do_render_bundle_encoder_push_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder push debug group"))
    }
    fn do_render_bundle_encoder_set_bind_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder set bind group"))
    }
    fn do_render_bundle_encoder_set_immediates(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder set immediates"))
    }
    fn do_render_bundle_encoder_set_index_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder set index buffer"))
    }
    fn do_render_bundle_encoder_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder set label"))
    }
    fn do_render_bundle_encoder_set_pipeline(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle encoder set pipeline"))
    }
    fn do_render_bundle_encoder_set_resource_table(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render bundle encoder set resource table",
            ),
        )
    }
    fn do_render_bundle_encoder_set_vertex_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("render bundle encoder set vertex buffer"),
        )
    }
    fn do_render_bundle_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render bundle set label"))
    }
    fn do_render_pass_encoder_begin_occlusion_query(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render pass encoder begin occlusion query",
            ),
        )
    }
    fn do_render_pass_encoder_draw(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder draw"))
    }
    fn do_render_pass_encoder_draw_indexed(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder draw indexed"))
    }
    fn do_render_pass_encoder_draw_indexed_indirect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render pass encoder draw indexed indirect",
            ),
        )
    }
    fn do_render_pass_encoder_draw_indirect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder draw indirect"))
    }
    fn do_render_pass_encoder_end(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder end"))
    }
    fn do_render_pass_encoder_end_occlusion_query(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("render pass encoder end occlusion query"),
        )
    }
    fn do_render_pass_encoder_execute_bundles(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder execute bundles"))
    }
    fn do_render_pass_encoder_insert_debug_marker(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("render pass encoder insert debug marker"),
        )
    }
    fn do_render_pass_encoder_multi_draw_indexed_indirect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render pass encoder multi draw indexed indirect",
            ),
        )
    }
    fn do_render_pass_encoder_multi_draw_indirect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand("render pass encoder multi draw indirect"),
        )
    }
    fn do_render_pass_encoder_pixel_local_storage_barrier(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render pass encoder pixel local storage barrier",
            ),
        )
    }
    fn do_render_pass_encoder_pop_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder pop debug group"))
    }
    fn do_render_pass_encoder_push_debug_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder push debug group"))
    }
    fn do_render_pass_encoder_set_bind_group(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set bind group"))
    }
    fn do_render_pass_encoder_set_blend_constant(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set blend constant"))
    }
    fn do_render_pass_encoder_set_immediates(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set immediates"))
    }
    fn do_render_pass_encoder_set_index_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set index buffer"))
    }
    fn do_render_pass_encoder_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set label"))
    }
    fn do_render_pass_encoder_set_pipeline(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set pipeline"))
    }
    fn do_render_pass_encoder_set_resource_table(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set resource table"))
    }
    fn do_render_pass_encoder_set_scissor_rect(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set scissor rect"))
    }
    fn do_render_pass_encoder_set_stencil_reference(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(
            WireServerError::UnexpectedCommand(
                "render pass encoder set stencil reference",
            ),
        )
    }
    fn do_render_pass_encoder_set_vertex_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set vertex buffer"))
    }
    fn do_render_pass_encoder_set_viewport(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder set viewport"))
    }
    fn do_render_pass_encoder_write_timestamp(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pass encoder write timestamp"))
    }
    fn do_render_pipeline_get_bind_group_layout(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pipeline get bind group layout"))
    }
    fn do_render_pipeline_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("render pipeline set label"))
    }
    fn do_sampler_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("sampler set label"))
    }
    fn do_shader_module_get_compilation_info(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shader module get compilation info"))
    }
    fn do_shader_module_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shader module set label"))
    }
    fn do_shared_buffer_memory_begin_access(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared buffer memory begin access"))
    }
    fn do_shared_buffer_memory_create_buffer(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared buffer memory create buffer"))
    }
    fn do_shared_buffer_memory_end_access(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared buffer memory end access"))
    }
    fn do_shared_buffer_memory_get_properties(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared buffer memory get properties"))
    }
    fn do_shared_buffer_memory_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared buffer memory set label"))
    }
    fn do_shared_fence_export_info(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared fence export info"))
    }
    fn do_shared_texture_memory_begin_access(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared texture memory begin access"))
    }
    fn do_shared_texture_memory_create_texture(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared texture memory create texture"))
    }
    fn do_shared_texture_memory_end_access(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared texture memory end access"))
    }
    fn do_shared_texture_memory_get_properties(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared texture memory get properties"))
    }
    fn do_shared_texture_memory_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("shared texture memory set label"))
    }
    fn do_surface_get_current_texture(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("surface get current texture"))
    }
    fn do_surface_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("surface set label"))
    }
    fn do_texel_buffer_view_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texel buffer view set label"))
    }
    fn do_texture_create_error_view(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture create error view"))
    }
    fn do_texture_create_view(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture create view"))
    }
    fn do_texture_destroy(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture destroy"))
    }
    fn do_texture_pin(&mut self, _packet: &WirePacket) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture pin"))
    }
    fn do_texture_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture set label"))
    }
    fn do_texture_set_ownership_for_memory_dump(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture set ownership for memory dump"))
    }
    fn do_texture_unpin(&mut self, _packet: &WirePacket) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture unpin"))
    }
    fn do_texture_view_set_label(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("texture view set label"))
    }
    fn do_unregister_object(
        &mut self,
        _packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        Err(WireServerError::UnexpectedCommand("unregister object"))
    }
}
pub struct GeneratedServerDoer {
    responses: Vec<WirePacket>,
}
impl GeneratedServerDoer {
    pub fn new() -> Self {
        Self { responses: Vec::new() }
    }
    pub fn pop_response(&mut self) -> Option<WirePacket> {
        if self.responses.is_empty() { None } else { Some(self.responses.remove(0)) }
    }
}
impl Default for GeneratedServerDoer {
    fn default() -> Self {
        Self::new()
    }
}
impl WireServerDoer for GeneratedServerDoer {
    fn do_instance_request_adapter(
        &mut self,
        packet: &WirePacket,
    ) -> Result<(), WireServerError> {
        use crate::generated::{
            AdapterInfo, FutureWaitInfo, Instance, InstanceDescriptor,
            InstanceFeatureName, RequestAdapterStatus, Status, WaitStatus,
        };
        use super::wire_types::{build_packet, WireCommand, WireValue};
        use std::sync::mpsc;
        let event_manager = packet
            .fields
            .iter()
            .find(|f| f.name == "event manager handle")
            .and_then(|f| match f.value {
                WireValue::U64(v) => Some(v),
                _ => None,
            })
            .unwrap_or(0);
        let future_id = packet
            .fields
            .iter()
            .find(|f| f.name == "future")
            .and_then(|f| match f.value {
                WireValue::U64(v) => Some(v),
                _ => None,
            })
            .unwrap_or(0);
        let mut instance_desc = InstanceDescriptor::new();
        instance_desc.required_features = Some(vec![InstanceFeatureName::TimedWaitAny]);
        let instance = Instance::new(Some(&instance_desc));
        let (tx, rx) = mpsc::channel::<Result<String, String>>();
        let future = instance
            .request_adapter(
                None,
                move |status, adapter, message| {
                    if status != RequestAdapterStatus::Success {
                        let _ = tx.send(Err(format!("{status:?}: {message}")));
                        return;
                    }
                    let adapter = match adapter {
                        Some(v) => v,
                        None => {
                            let _ = tx
                                .send(
                                    Err("request adapter returned no adapter".to_string()),
                                );
                            return;
                        }
                    };
                    let mut info = AdapterInfo::new();
                    let get_info_status = adapter.get_info(&mut info);
                    if get_info_status != Status::Success {
                        let _ = tx
                            .send(
                                Err(format!("adapter.get_info failed: {get_info_status:?}")),
                            );
                        return;
                    }
                    let vendor = info.vendor.as_deref().unwrap_or("Unknown");
                    let device = info.device.as_deref().unwrap_or("Unknown");
                    let description = info.description.as_deref().unwrap_or("Unknown");
                    let _ = tx.send(Ok(format!("{vendor} {device} ({description})")));
                },
            );
        let wait = instance
            .wait_any(
                Some(
                    &mut [
                        FutureWaitInfo {
                            future: Some(future),
                            completed: None,
                        },
                    ],
                ),
                0,
            );
        if wait != WaitStatus::Success {
            return Err(
                WireServerError::Handler(format!("instance.wait_any failed: {wait:?}")),
            );
        }
        let info_string = rx
            .recv()
            .map_err(|e| WireServerError::Handler(e.to_string()))?
            .map_err(WireServerError::Handler)?;
        let response = build_packet(
                WireCommand::ReturnInstanceRequestAdapterCallback,
                vec![
                    WireValue::U64(event_manager),
                    WireValue::U64(future_id),
                    WireValue::U32(0),
                    WireValue::String("ok".to_string()),
                    WireValue::String(info_string),
                    WireValue::Null,
                    WireValue::U32(0),
                    WireValue::Bytes(Vec::new()),
                ],
            )
            .map_err(WireServerError::Handler)?;
        self.responses.push(response);
        Ok(())
    }
}
pub struct WireServer<H: WireServerDoer> {
    handler: H,
}
impl<H: WireServerDoer> WireServer<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
    pub fn handler_mut(&mut self) -> &mut H {
        &mut self.handler
    }
    pub fn handle_packet(&mut self, packet: WirePacket) -> Result<(), WireServerError> {
        match packet.command {
            WireCommand::ChunkedCommand => self.handler.do_chunked_command(&packet),
            WireCommand::AdapterRequestDevice => {
                self.handler.do_adapter_request_device(&packet)
            }
            WireCommand::BindGroupLayoutSetLabel => {
                self.handler.do_bind_group_layout_set_label(&packet)
            }
            WireCommand::BindGroupSetLabel => {
                self.handler.do_bind_group_set_label(&packet)
            }
            WireCommand::BufferCreateTexelView => {
                self.handler.do_buffer_create_texel_view(&packet)
            }
            WireCommand::BufferMapAsync => self.handler.do_buffer_map_async(&packet),
            WireCommand::BufferSetLabel => self.handler.do_buffer_set_label(&packet),
            WireCommand::BufferUpdateMappedData => {
                self.handler.do_buffer_update_mapped_data(&packet)
            }
            WireCommand::CommandBufferSetLabel => {
                self.handler.do_command_buffer_set_label(&packet)
            }
            WireCommand::CommandEncoderBeginComputePass => {
                self.handler.do_command_encoder_begin_compute_pass(&packet)
            }
            WireCommand::CommandEncoderBeginRenderPass => {
                self.handler.do_command_encoder_begin_render_pass(&packet)
            }
            WireCommand::CommandEncoderClearBuffer => {
                self.handler.do_command_encoder_clear_buffer(&packet)
            }
            WireCommand::CommandEncoderCopyBufferToBuffer => {
                self.handler.do_command_encoder_copy_buffer_to_buffer(&packet)
            }
            WireCommand::CommandEncoderCopyBufferToTexture => {
                self.handler.do_command_encoder_copy_buffer_to_texture(&packet)
            }
            WireCommand::CommandEncoderCopyTextureToBuffer => {
                self.handler.do_command_encoder_copy_texture_to_buffer(&packet)
            }
            WireCommand::CommandEncoderCopyTextureToTexture => {
                self.handler.do_command_encoder_copy_texture_to_texture(&packet)
            }
            WireCommand::CommandEncoderFinish => {
                self.handler.do_command_encoder_finish(&packet)
            }
            WireCommand::CommandEncoderInjectValidationError => {
                self.handler.do_command_encoder_inject_validation_error(&packet)
            }
            WireCommand::CommandEncoderInsertDebugMarker => {
                self.handler.do_command_encoder_insert_debug_marker(&packet)
            }
            WireCommand::CommandEncoderPopDebugGroup => {
                self.handler.do_command_encoder_pop_debug_group(&packet)
            }
            WireCommand::CommandEncoderPushDebugGroup => {
                self.handler.do_command_encoder_push_debug_group(&packet)
            }
            WireCommand::CommandEncoderResolveQuerySet => {
                self.handler.do_command_encoder_resolve_query_set(&packet)
            }
            WireCommand::CommandEncoderSetLabel => {
                self.handler.do_command_encoder_set_label(&packet)
            }
            WireCommand::CommandEncoderWriteBuffer => {
                self.handler.do_command_encoder_write_buffer(&packet)
            }
            WireCommand::CommandEncoderWriteTimestamp => {
                self.handler.do_command_encoder_write_timestamp(&packet)
            }
            WireCommand::ComputePassEncoderDispatchWorkgroups => {
                self.handler.do_compute_pass_encoder_dispatch_workgroups(&packet)
            }
            WireCommand::ComputePassEncoderDispatchWorkgroupsIndirect => {
                self.handler
                    .do_compute_pass_encoder_dispatch_workgroups_indirect(&packet)
            }
            WireCommand::ComputePassEncoderEnd => {
                self.handler.do_compute_pass_encoder_end(&packet)
            }
            WireCommand::ComputePassEncoderInsertDebugMarker => {
                self.handler.do_compute_pass_encoder_insert_debug_marker(&packet)
            }
            WireCommand::ComputePassEncoderPopDebugGroup => {
                self.handler.do_compute_pass_encoder_pop_debug_group(&packet)
            }
            WireCommand::ComputePassEncoderPushDebugGroup => {
                self.handler.do_compute_pass_encoder_push_debug_group(&packet)
            }
            WireCommand::ComputePassEncoderSetBindGroup => {
                self.handler.do_compute_pass_encoder_set_bind_group(&packet)
            }
            WireCommand::ComputePassEncoderSetImmediates => {
                self.handler.do_compute_pass_encoder_set_immediates(&packet)
            }
            WireCommand::ComputePassEncoderSetLabel => {
                self.handler.do_compute_pass_encoder_set_label(&packet)
            }
            WireCommand::ComputePassEncoderSetPipeline => {
                self.handler.do_compute_pass_encoder_set_pipeline(&packet)
            }
            WireCommand::ComputePassEncoderSetResourceTable => {
                self.handler.do_compute_pass_encoder_set_resource_table(&packet)
            }
            WireCommand::ComputePassEncoderWriteTimestamp => {
                self.handler.do_compute_pass_encoder_write_timestamp(&packet)
            }
            WireCommand::ComputePipelineGetBindGroupLayout => {
                self.handler.do_compute_pipeline_get_bind_group_layout(&packet)
            }
            WireCommand::ComputePipelineSetLabel => {
                self.handler.do_compute_pipeline_set_label(&packet)
            }
            WireCommand::DeviceCreateBindGroup => {
                self.handler.do_device_create_bind_group(&packet)
            }
            WireCommand::DeviceCreateBindGroupLayout => {
                self.handler.do_device_create_bind_group_layout(&packet)
            }
            WireCommand::DeviceCreateBuffer => {
                self.handler.do_device_create_buffer(&packet)
            }
            WireCommand::DeviceCreateCommandEncoder => {
                self.handler.do_device_create_command_encoder(&packet)
            }
            WireCommand::DeviceCreateComputePipeline => {
                self.handler.do_device_create_compute_pipeline(&packet)
            }
            WireCommand::DeviceCreateComputePipelineAsync => {
                self.handler.do_device_create_compute_pipeline_async(&packet)
            }
            WireCommand::DeviceCreateErrorExternalTexture => {
                self.handler.do_device_create_error_external_texture(&packet)
            }
            WireCommand::DeviceCreateErrorShaderModule => {
                self.handler.do_device_create_error_shader_module(&packet)
            }
            WireCommand::DeviceCreateExternalTexture => {
                self.handler.do_device_create_external_texture(&packet)
            }
            WireCommand::DeviceCreatePipelineLayout => {
                self.handler.do_device_create_pipeline_layout(&packet)
            }
            WireCommand::DeviceCreateQuerySet => {
                self.handler.do_device_create_query_set(&packet)
            }
            WireCommand::DeviceCreateRenderBundleEncoder => {
                self.handler.do_device_create_render_bundle_encoder(&packet)
            }
            WireCommand::DeviceCreateRenderPipeline => {
                self.handler.do_device_create_render_pipeline(&packet)
            }
            WireCommand::DeviceCreateRenderPipelineAsync => {
                self.handler.do_device_create_render_pipeline_async(&packet)
            }
            WireCommand::DeviceCreateSampler => {
                self.handler.do_device_create_sampler(&packet)
            }
            WireCommand::DeviceCreateShaderModule => {
                self.handler.do_device_create_shader_module(&packet)
            }
            WireCommand::DeviceForceLoss => self.handler.do_device_force_loss(&packet),
            WireCommand::DeviceGetAHardwareBufferProperties => {
                self.handler.do_device_get_a_hardware_buffer_properties(&packet)
            }
            WireCommand::DeviceImportSharedBufferMemory => {
                self.handler.do_device_import_shared_buffer_memory(&packet)
            }
            WireCommand::DeviceImportSharedFence => {
                self.handler.do_device_import_shared_fence(&packet)
            }
            WireCommand::DeviceImportSharedTextureMemory => {
                self.handler.do_device_import_shared_texture_memory(&packet)
            }
            WireCommand::DevicePopErrorScope => {
                self.handler.do_device_pop_error_scope(&packet)
            }
            WireCommand::DevicePushErrorScope => {
                self.handler.do_device_push_error_scope(&packet)
            }
            WireCommand::DeviceSetLabel => self.handler.do_device_set_label(&packet),
            WireCommand::DeviceTick => self.handler.do_device_tick(&packet),
            WireCommand::DeviceValidateTextureDescriptor => {
                self.handler.do_device_validate_texture_descriptor(&packet)
            }
            WireCommand::ExternalTextureDestroy => {
                self.handler.do_external_texture_destroy(&packet)
            }
            WireCommand::ExternalTextureExpire => {
                self.handler.do_external_texture_expire(&packet)
            }
            WireCommand::ExternalTextureRefresh => {
                self.handler.do_external_texture_refresh(&packet)
            }
            WireCommand::ExternalTextureSetLabel => {
                self.handler.do_external_texture_set_label(&packet)
            }
            WireCommand::InstanceRequestAdapter => {
                self.handler.do_instance_request_adapter(&packet)
            }
            WireCommand::PipelineLayoutSetLabel => {
                self.handler.do_pipeline_layout_set_label(&packet)
            }
            WireCommand::QuerySetDestroy => self.handler.do_query_set_destroy(&packet),
            WireCommand::QuerySetSetLabel => self.handler.do_query_set_set_label(&packet),
            WireCommand::QueueCopyExternalTextureForBrowser => {
                self.handler.do_queue_copy_external_texture_for_browser(&packet)
            }
            WireCommand::QueueCopyTextureForBrowser => {
                self.handler.do_queue_copy_texture_for_browser(&packet)
            }
            WireCommand::QueueOnSubmittedWorkDone => {
                self.handler.do_queue_on_submitted_work_done(&packet)
            }
            WireCommand::QueueSetLabel => self.handler.do_queue_set_label(&packet),
            WireCommand::QueueWriteBuffer => self.handler.do_queue_write_buffer(&packet),
            WireCommand::QueueWriteBufferXl => {
                self.handler.do_queue_write_buffer_xl(&packet)
            }
            WireCommand::QueueWriteTexture => {
                self.handler.do_queue_write_texture(&packet)
            }
            WireCommand::QueueWriteTextureXl => {
                self.handler.do_queue_write_texture_xl(&packet)
            }
            WireCommand::RenderBundleEncoderDraw => {
                self.handler.do_render_bundle_encoder_draw(&packet)
            }
            WireCommand::RenderBundleEncoderDrawIndexed => {
                self.handler.do_render_bundle_encoder_draw_indexed(&packet)
            }
            WireCommand::RenderBundleEncoderDrawIndexedIndirect => {
                self.handler.do_render_bundle_encoder_draw_indexed_indirect(&packet)
            }
            WireCommand::RenderBundleEncoderDrawIndirect => {
                self.handler.do_render_bundle_encoder_draw_indirect(&packet)
            }
            WireCommand::RenderBundleEncoderFinish => {
                self.handler.do_render_bundle_encoder_finish(&packet)
            }
            WireCommand::RenderBundleEncoderInsertDebugMarker => {
                self.handler.do_render_bundle_encoder_insert_debug_marker(&packet)
            }
            WireCommand::RenderBundleEncoderPopDebugGroup => {
                self.handler.do_render_bundle_encoder_pop_debug_group(&packet)
            }
            WireCommand::RenderBundleEncoderPushDebugGroup => {
                self.handler.do_render_bundle_encoder_push_debug_group(&packet)
            }
            WireCommand::RenderBundleEncoderSetBindGroup => {
                self.handler.do_render_bundle_encoder_set_bind_group(&packet)
            }
            WireCommand::RenderBundleEncoderSetImmediates => {
                self.handler.do_render_bundle_encoder_set_immediates(&packet)
            }
            WireCommand::RenderBundleEncoderSetIndexBuffer => {
                self.handler.do_render_bundle_encoder_set_index_buffer(&packet)
            }
            WireCommand::RenderBundleEncoderSetLabel => {
                self.handler.do_render_bundle_encoder_set_label(&packet)
            }
            WireCommand::RenderBundleEncoderSetPipeline => {
                self.handler.do_render_bundle_encoder_set_pipeline(&packet)
            }
            WireCommand::RenderBundleEncoderSetResourceTable => {
                self.handler.do_render_bundle_encoder_set_resource_table(&packet)
            }
            WireCommand::RenderBundleEncoderSetVertexBuffer => {
                self.handler.do_render_bundle_encoder_set_vertex_buffer(&packet)
            }
            WireCommand::RenderBundleSetLabel => {
                self.handler.do_render_bundle_set_label(&packet)
            }
            WireCommand::RenderPassEncoderBeginOcclusionQuery => {
                self.handler.do_render_pass_encoder_begin_occlusion_query(&packet)
            }
            WireCommand::RenderPassEncoderDraw => {
                self.handler.do_render_pass_encoder_draw(&packet)
            }
            WireCommand::RenderPassEncoderDrawIndexed => {
                self.handler.do_render_pass_encoder_draw_indexed(&packet)
            }
            WireCommand::RenderPassEncoderDrawIndexedIndirect => {
                self.handler.do_render_pass_encoder_draw_indexed_indirect(&packet)
            }
            WireCommand::RenderPassEncoderDrawIndirect => {
                self.handler.do_render_pass_encoder_draw_indirect(&packet)
            }
            WireCommand::RenderPassEncoderEnd => {
                self.handler.do_render_pass_encoder_end(&packet)
            }
            WireCommand::RenderPassEncoderEndOcclusionQuery => {
                self.handler.do_render_pass_encoder_end_occlusion_query(&packet)
            }
            WireCommand::RenderPassEncoderExecuteBundles => {
                self.handler.do_render_pass_encoder_execute_bundles(&packet)
            }
            WireCommand::RenderPassEncoderInsertDebugMarker => {
                self.handler.do_render_pass_encoder_insert_debug_marker(&packet)
            }
            WireCommand::RenderPassEncoderMultiDrawIndexedIndirect => {
                self.handler.do_render_pass_encoder_multi_draw_indexed_indirect(&packet)
            }
            WireCommand::RenderPassEncoderMultiDrawIndirect => {
                self.handler.do_render_pass_encoder_multi_draw_indirect(&packet)
            }
            WireCommand::RenderPassEncoderPixelLocalStorageBarrier => {
                self.handler.do_render_pass_encoder_pixel_local_storage_barrier(&packet)
            }
            WireCommand::RenderPassEncoderPopDebugGroup => {
                self.handler.do_render_pass_encoder_pop_debug_group(&packet)
            }
            WireCommand::RenderPassEncoderPushDebugGroup => {
                self.handler.do_render_pass_encoder_push_debug_group(&packet)
            }
            WireCommand::RenderPassEncoderSetBindGroup => {
                self.handler.do_render_pass_encoder_set_bind_group(&packet)
            }
            WireCommand::RenderPassEncoderSetBlendConstant => {
                self.handler.do_render_pass_encoder_set_blend_constant(&packet)
            }
            WireCommand::RenderPassEncoderSetImmediates => {
                self.handler.do_render_pass_encoder_set_immediates(&packet)
            }
            WireCommand::RenderPassEncoderSetIndexBuffer => {
                self.handler.do_render_pass_encoder_set_index_buffer(&packet)
            }
            WireCommand::RenderPassEncoderSetLabel => {
                self.handler.do_render_pass_encoder_set_label(&packet)
            }
            WireCommand::RenderPassEncoderSetPipeline => {
                self.handler.do_render_pass_encoder_set_pipeline(&packet)
            }
            WireCommand::RenderPassEncoderSetResourceTable => {
                self.handler.do_render_pass_encoder_set_resource_table(&packet)
            }
            WireCommand::RenderPassEncoderSetScissorRect => {
                self.handler.do_render_pass_encoder_set_scissor_rect(&packet)
            }
            WireCommand::RenderPassEncoderSetStencilReference => {
                self.handler.do_render_pass_encoder_set_stencil_reference(&packet)
            }
            WireCommand::RenderPassEncoderSetVertexBuffer => {
                self.handler.do_render_pass_encoder_set_vertex_buffer(&packet)
            }
            WireCommand::RenderPassEncoderSetViewport => {
                self.handler.do_render_pass_encoder_set_viewport(&packet)
            }
            WireCommand::RenderPassEncoderWriteTimestamp => {
                self.handler.do_render_pass_encoder_write_timestamp(&packet)
            }
            WireCommand::RenderPipelineGetBindGroupLayout => {
                self.handler.do_render_pipeline_get_bind_group_layout(&packet)
            }
            WireCommand::RenderPipelineSetLabel => {
                self.handler.do_render_pipeline_set_label(&packet)
            }
            WireCommand::SamplerSetLabel => self.handler.do_sampler_set_label(&packet),
            WireCommand::ShaderModuleGetCompilationInfo => {
                self.handler.do_shader_module_get_compilation_info(&packet)
            }
            WireCommand::ShaderModuleSetLabel => {
                self.handler.do_shader_module_set_label(&packet)
            }
            WireCommand::SharedBufferMemoryBeginAccess => {
                self.handler.do_shared_buffer_memory_begin_access(&packet)
            }
            WireCommand::SharedBufferMemoryCreateBuffer => {
                self.handler.do_shared_buffer_memory_create_buffer(&packet)
            }
            WireCommand::SharedBufferMemoryEndAccess => {
                self.handler.do_shared_buffer_memory_end_access(&packet)
            }
            WireCommand::SharedBufferMemoryGetProperties => {
                self.handler.do_shared_buffer_memory_get_properties(&packet)
            }
            WireCommand::SharedBufferMemorySetLabel => {
                self.handler.do_shared_buffer_memory_set_label(&packet)
            }
            WireCommand::SharedFenceExportInfo => {
                self.handler.do_shared_fence_export_info(&packet)
            }
            WireCommand::SharedTextureMemoryBeginAccess => {
                self.handler.do_shared_texture_memory_begin_access(&packet)
            }
            WireCommand::SharedTextureMemoryCreateTexture => {
                self.handler.do_shared_texture_memory_create_texture(&packet)
            }
            WireCommand::SharedTextureMemoryEndAccess => {
                self.handler.do_shared_texture_memory_end_access(&packet)
            }
            WireCommand::SharedTextureMemoryGetProperties => {
                self.handler.do_shared_texture_memory_get_properties(&packet)
            }
            WireCommand::SharedTextureMemorySetLabel => {
                self.handler.do_shared_texture_memory_set_label(&packet)
            }
            WireCommand::SurfaceGetCurrentTexture => {
                self.handler.do_surface_get_current_texture(&packet)
            }
            WireCommand::SurfaceSetLabel => self.handler.do_surface_set_label(&packet),
            WireCommand::TexelBufferViewSetLabel => {
                self.handler.do_texel_buffer_view_set_label(&packet)
            }
            WireCommand::TextureCreateErrorView => {
                self.handler.do_texture_create_error_view(&packet)
            }
            WireCommand::TextureCreateView => {
                self.handler.do_texture_create_view(&packet)
            }
            WireCommand::TextureDestroy => self.handler.do_texture_destroy(&packet),
            WireCommand::TexturePin => self.handler.do_texture_pin(&packet),
            WireCommand::TextureSetLabel => self.handler.do_texture_set_label(&packet),
            WireCommand::TextureSetOwnershipForMemoryDump => {
                self.handler.do_texture_set_ownership_for_memory_dump(&packet)
            }
            WireCommand::TextureUnpin => self.handler.do_texture_unpin(&packet),
            WireCommand::TextureViewSetLabel => {
                self.handler.do_texture_view_set_label(&packet)
            }
            WireCommand::UnregisterObject => self.handler.do_unregister_object(&packet),
            other => Err(WireServerError::UnexpectedCommand(wire_command_name(other))),
        }
    }
    pub fn handle_packets<I>(&mut self, packets: I) -> Result<(), WireServerError>
    where
        I: IntoIterator<Item = WirePacket>,
    {
        for packet in packets {
            self.handle_packet(packet)?;
        }
        Ok(())
    }
}
