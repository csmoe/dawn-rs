use crate::dispatch::*;
use crate::error::DawnError;
use crate::future::*;
use crate::mapping::*;
use crate::types::*;
use dawn_rs::*;
use std::pin::Pin;
use std::sync::Arc;
use wgpu::custom::*;

impl InstanceInterface for DawnInstance {
    fn new(_desc: &wgpu::InstanceDescriptor) -> Self {
        let mut desc = InstanceDescriptor::new();
        desc.required_features = Some(vec![InstanceFeatureName::TimedWaitAny]);
        let instance = Instance::new(Some(&desc));
        Self {
            inner: SendSync::new(instance),
        }
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
                let surface = self.inner.get().create_surface(&desc);
                let dawn_surface = DawnSurface {
                    inner: SendSync::new(surface),
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
                        let surface = self.inner.get().create_surface(&desc);
                        let handle = MetalLayerHandle {
                            ptr: layer_ptr.as_ptr().cast(),
                        };
                        let dawn_surface = DawnSurface {
                            inner: SendSync::new(surface),
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
                            hinstance: handle.hinstance.map(|h| h.get() as _),
                            hwnd: Some(handle.hwnd.get() as _),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.get().create_surface(&desc);
                        let dawn_surface = DawnSurface {
                            inner: SendSync::new(surface),
                        };
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
                            display: Some(display.display.as_ptr().cast()),
                            surface: Some(window.surface.as_ptr().cast()),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.get().create_surface(&desc);
                        let dawn_surface = DawnSurface {
                            inner: SendSync::new(surface),
                        };
                        Ok(dispatch_surface(dawn_surface))
                    }
                    (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
                        let mut desc = SurfaceDescriptor::new();
                        let source = SurfaceSourceXlibWindow {
                            display: Some(display.display.unwrap().as_ptr().cast()),
                            window: Some(window.window as u64),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.get().create_surface(&desc);
                        let dawn_surface = DawnSurface {
                            inner: SendSync::new(surface),
                        };
                        Ok(dispatch_surface(dawn_surface))
                    }
                    (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                        let mut desc = SurfaceDescriptor::new();
                        let source = SurfaceSourceXCBWindow {
                            connection: Some(display.connection.unwrap().as_ptr().cast()),
                            window: Some(window.window.get()),
                        };
                        desc = desc.with_extension(SurfaceDescriptorExtension::from(source));
                        let surface = self.inner.get().create_surface(&desc);
                        let dawn_surface = DawnSurface {
                            inner: SendSync::new(surface),
                        };
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
            dawn_options.compatible_surface = Some(expect_surface_from_api(surface).inner.get());
        }
        let future_handle = self.inner.get().request_adapter(
            Some(&dawn_options),
            move |status, adapter, _message| {
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
            },
        );
        let _ = self.inner.get().wait_any(
            Some(&mut [FutureWaitInfo {
                future: Some(future_handle),
                completed: None,
            }]),
            0,
        );
        Box::pin(future)
    }

    fn poll_all_devices(&self, _force_wait: bool) -> bool {
        self.inner.get().process_events();
        true
    }

    fn wgsl_language_features(&self) -> wgpu::WgslLanguageFeatures {
        let mut features = SupportedWGSLLanguageFeatures::new();
        self.inner.get().get_wgsl_language_features(&mut features);
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
                panic!("Uncaptured error {:?}: {}", ty, message);
            })));
        dawn_desc.uncaptured_error_callback_info = Some(error_info);
        let instance = self.inner.get().get_instance();
        let future_handle =
            self.inner
                .get()
                .request_device(Some(&dawn_desc), move |status, device, message| {
                    if status == RequestDeviceStatus::Success {
                        let device = device.expect("wgpu-compat: missing device");
                        let queue = device.get_queue();
                        complete_shared(
                            &shared,
                            Ok((dispatch_device(device), dispatch_queue(queue))),
                        );
                    } else {
                        panic!("wgpu-compat: request_device failed {}", message);
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
        self.inner.get().get_features(&mut features);
        map_features_to_wgpu(&features)
    }

    fn limits(&self) -> wgpu::Limits {
        let mut limits = Limits::new();
        let _ = self.inner.get().get_limits(&mut limits);
        map_limits_to_wgpu(&limits)
    }

    fn downlevel_capabilities(&self) -> wgpu::DownlevelCapabilities {
        wgpu::DownlevelCapabilities::default()
    }

    fn get_info(&self) -> wgpu::AdapterInfo {
        let mut info = AdapterInfo::new();
        let _ = self.inner.get().get_info(&mut info);
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
        DawnAdapter {
            inner: SendSync::new(adapter),
        }
        .features()
    }

    fn limits(&self) -> wgpu::Limits {
        let adapter = self.inner.get_adapter();
        DawnAdapter {
            inner: SendSync::new(adapter),
        }
        .limits()
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
        mut data_layout: wgpu::TexelCopyBufferLayout,
        size: wgpu::Extent3d,
    ) {
        if data_layout.rows_per_image.is_none()
            && (size.height > 1 || size.depth_or_array_layers > 1)
        {
            data_layout.rows_per_image = Some(size.height.max(1));
        }
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
        self.inner.get().copy_buffer_to_buffer(
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
        self.inner
            .get()
            .copy_buffer_to_texture(&source, &dest, &size);
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
        self.inner
            .get()
            .copy_texture_to_buffer(&source, &dest, &size);
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
        self.inner
            .get()
            .copy_texture_to_texture(&source, &dest, &size);
    }

    fn begin_compute_pass(&self, desc: &wgpu::ComputePassDescriptor<'_>) -> DispatchComputePass {
        let dawn_desc = map_compute_pass_descriptor(desc);
        let pass = self.inner.get().begin_compute_pass(Some(&dawn_desc));
        dispatch_compute_pass(pass)
    }

    fn begin_render_pass(&self, desc: &wgpu::RenderPassDescriptor<'_>) -> DispatchRenderPass {
        let dawn_desc = map_render_pass_descriptor(desc);
        let pass = self.inner.get().begin_render_pass(&dawn_desc);
        dispatch_render_pass(pass)
    }

    fn finish(&mut self) -> DispatchCommandBuffer {
        let buffer = self.inner.get().finish(None);
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
            .get()
            .clear_buffer(buffer, offset, size.unwrap_or(WHOLE_SIZE));
    }

    fn insert_debug_marker(&self, label: &str) {
        self.inner.get().insert_debug_marker(label.to_string());
    }

    fn push_debug_group(&self, label: &str) {
        self.inner.get().push_debug_group(label.to_string());
    }

    fn pop_debug_group(&self) {
        self.inner.get().pop_debug_group();
    }

    fn write_timestamp(&self, query_set: &DispatchQuerySet, query_index: u32) {
        let set = expect_query_set(query_set);
        self.inner.get().write_timestamp(set, query_index);
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
        self.inner.get().resolve_query_set(
            set,
            first_query,
            query_count,
            buffer,
            destination_offset,
        );
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
        self.inner.get().set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[wgpu::DynamicOffset],
    ) {
        let group = bind_group.map(expect_bind_group);
        self.inner.get().set_bind_group(index, group, offsets);
    }

    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        let data = bytes_to_u32(data);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_len = data.len() * std::mem::size_of::<u32>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
        self.inner.get().set_immediates(offset, data_slice);
    }

    fn insert_debug_marker(&mut self, label: &str) {
        self.inner.get().insert_debug_marker(label.to_string());
    }

    fn push_debug_group(&mut self, group_label: &str) {
        self.inner.get().push_debug_group(group_label.to_string());
    }

    fn pop_debug_group(&mut self) {
        self.inner.get().pop_debug_group();
    }

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        let set = expect_query_set(query_set);
        self.inner.get().write_timestamp(set, query_index);
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
        self.inner.get().dispatch_workgroups(x, y, z);
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner
            .get()
            .dispatch_workgroups_indirect(buffer, indirect_offset);
    }

    fn end(&mut self) {
        if !self.ended {
            self.inner.get().end();
            self.ended = true;
        }
    }
}

impl Drop for DawnComputePass {
    fn drop(&mut self) {
        if !self.ended {
            self.inner.get().end();
            self.ended = true;
        }
    }
}

impl RenderPassInterface for DawnRenderPass {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline) {
        let pipeline = expect_render_pipeline(pipeline);
        self.inner.get().set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[wgpu::DynamicOffset],
    ) {
        let group = bind_group.map(expect_bind_group);
        self.inner.get().set_bind_group(index, group, offsets);
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
            .get()
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
            .get()
            .set_vertex_buffer(slot, Some(buffer), offset, size);
    }

    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        let data = bytes_to_u32(data);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_len = data.len() * std::mem::size_of::<u32>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
        self.inner.get().set_immediates(offset, data_slice);
    }

    fn set_blend_constant(&mut self, color: wgpu::Color) {
        let color = map_color(color);
        self.inner.get().set_blend_constant(&color);
    }

    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.inner.get().set_scissor_rect(x, y, width, height);
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
            .get()
            .set_viewport(x, y, width, height, min_depth, max_depth);
    }

    fn set_stencil_reference(&mut self, reference: u32) {
        self.inner.get().set_stencil_reference(reference);
    }

    fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        self.inner.get().draw(
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
        self.inner.get().draw_indexed(
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
        self.inner.get().draw_indirect(buffer, indirect_offset);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner
            .get()
            .draw_indexed_indirect(buffer, indirect_offset);
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
            .get()
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
            .get()
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
        self.inner.get().insert_debug_marker(label.to_string());
    }

    fn push_debug_group(&mut self, group_label: &str) {
        self.inner.get().push_debug_group(group_label.to_string());
    }

    fn pop_debug_group(&mut self) {
        self.inner.get().pop_debug_group();
    }

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        let set = expect_query_set(query_set);
        self.inner.get().write_timestamp(set, query_index);
    }

    fn begin_occlusion_query(&mut self, query_index: u32) {
        self.inner.get().begin_occlusion_query(query_index);
    }

    fn end_occlusion_query(&mut self) {
        self.inner.get().end_occlusion_query();
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
        self.inner.get().execute_bundles(&bundles);
    }

    fn end(&mut self) {
        if !self.ended {
            self.inner.get().end();
            self.ended = true;
        }
    }
}

impl Drop for DawnRenderPass {
    fn drop(&mut self) {
        if !self.ended {
            self.inner.get().end();
            self.ended = true;
        }
    }
}

impl RenderBundleEncoderInterface for DawnRenderBundleEncoder {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline) {
        let pipeline = expect_render_pipeline(pipeline);
        self.inner.get().set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[wgpu::DynamicOffset],
    ) {
        let group = bind_group.map(expect_bind_group);
        self.inner.get().set_bind_group(index, group, offsets);
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
            .get()
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
            .get()
            .set_vertex_buffer(slot, Some(buffer), offset, size);
    }

    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        let data = bytes_to_u32(data);
        let data_ptr = data.as_ptr().cast::<std::ffi::c_void>();
        let data_len = data.len() * std::mem::size_of::<u32>();
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
        self.inner.get().set_immediates(offset, data_slice);
    }

    fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        self.inner.get().draw(
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
        self.inner.get().draw_indexed(
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
        self.inner.get().draw_indirect(buffer, indirect_offset);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: wgpu::BufferAddress,
    ) {
        let buffer = expect_buffer(indirect_buffer);
        self.inner
            .get()
            .draw_indexed_indirect(buffer, indirect_offset);
    }

    fn finish(self, desc: &wgpu::RenderBundleDescriptor<'_>) -> DispatchRenderBundle {
        let mut dawn_desc = RenderBundleDescriptor::new();
        dawn_desc.label = label_to_string(desc.label);
        let bundle = self.inner.get().finish(Some(&dawn_desc));
        dispatch_render_bundle(bundle)
    }
}

impl CommandBufferInterface for DawnCommandBuffer {}
impl RenderBundleInterface for DawnRenderBundle {}

impl SurfaceInterface for DawnSurface {
    fn get_capabilities(&self, adapter: &DispatchAdapter) -> wgpu::SurfaceCapabilities {
        let adapter = expect_adapter(adapter);
        let mut caps = SurfaceCapabilities::new();
        let _ = self.inner.get().get_capabilities(adapter, &mut caps);
        map_surface_capabilities(caps)
    }

    fn configure(&self, device: &DispatchDevice, config: &wgpu::SurfaceConfiguration) {
        let mut config = map_surface_configuration(config);
        config.device = Some(expect_device(device));
        self.inner.get().configure(&config);
    }

    fn get_current_texture(
        &self,
    ) -> (
        Option<DispatchTexture>,
        wgpu::SurfaceStatus,
        DispatchSurfaceOutputDetail,
    ) {
        let mut surface_texture = SurfaceTexture::new();
        self.inner.get().get_current_texture(&mut surface_texture);
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
            dispatch_surface_output_detail(self.inner.get()),
        )
    }
}

impl SurfaceOutputDetailInterface for DawnSurfaceOutputDetail {
    fn present(&self) {
        let _ = self.surface.get().present();
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
