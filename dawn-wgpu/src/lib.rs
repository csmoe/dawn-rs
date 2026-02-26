#![allow(unexpected_cfgs, dead_code, unreachable_patterns)]

mod backend;
mod compat;
mod dispatch;
mod error;
mod future;
mod mapping;
mod types;

pub use compat::{
    WgpuCompatError, from_wgpu_adapter, from_wgpu_bind_group, from_wgpu_bind_group_layout,
    from_wgpu_buffer, from_wgpu_command_buffer, from_wgpu_command_encoder,
    from_wgpu_compute_pipeline, from_wgpu_device, from_wgpu_instance, from_wgpu_pipeline_layout,
    from_wgpu_query_set, from_wgpu_queue, from_wgpu_render_bundle, from_wgpu_render_pipeline,
    from_wgpu_sampler, from_wgpu_shader_module, from_wgpu_surface, from_wgpu_texture,
    from_wgpu_texture_view, to_wgpu_adapter, to_wgpu_device, to_wgpu_instance, to_wgpu_queue,
    to_wgpu_texture,
};
