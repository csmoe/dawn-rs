#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;
use std::ffi::CStr;

fn string_view_to_string(view: ffi::WGPUStringView) -> String {
    if view.data.is_null() || view.length == 0 {
        return String::new();
    }
    let data = view.data.cast::<u8>();
    let slice = unsafe { std::slice::from_raw_parts(data, view.length) };
    String::from_utf8_lossy(slice).into_owned()
}

pub struct AHardwareBufferProperties {
    pub y_cb_cr_info: Option<YCbCrVkDescriptor>,
}

impl Default for AHardwareBufferProperties {
    fn default() -> Self {
        Self {
        y_cb_cr_info: None,
        }
    }
}

impl AHardwareBufferProperties {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn y_cb_cr_info(mut self, value: YCbCrVkDescriptor) -> Self {
        self.y_cb_cr_info = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUAHardwareBufferProperties {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAHardwareBufferProperties) -> Self {
        Self {
            y_cb_cr_info: Some(YCbCrVkDescriptor::from_ffi(value.yCbCrInfo)),
                }
    }
}

pub struct AdapterInfo {
    pub extensions: Vec<AdapterInfoExtension>,
    pub vendor: Option<String>,
    pub architecture: Option<String>,
    pub device: Option<String>,
    pub description: Option<String>,
    pub backend_type: Option<BackendType>,
    pub adapter_type: Option<AdapterType>,
    pub vendor_id: Option<u32>,
    pub device_id: Option<u32>,
    pub subgroup_min_size: Option<u32>,
    pub subgroup_max_size: Option<u32>,
}

impl Default for AdapterInfo {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        vendor: None,
        architecture: None,
        device: None,
        description: None,
        backend_type: None,
        adapter_type: None,
        vendor_id: None,
        device_id: None,
        subgroup_min_size: None,
        subgroup_max_size: None,
        }
    }
}

impl AdapterInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn vendor(mut self, value: String) -> Self {
        self.vendor = Some(value);
        self
    }

    pub fn architecture(mut self, value: String) -> Self {
        self.architecture = Some(value);
        self
    }

    pub fn device(mut self, value: String) -> Self {
        self.device = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn backend_type(mut self, value: BackendType) -> Self {
        self.backend_type = Some(value);
        self
    }

    pub fn adapter_type(mut self, value: AdapterType) -> Self {
        self.adapter_type = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: u32) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn device_id(mut self, value: u32) -> Self {
        self.device_id = Some(value);
        self
    }

    pub fn subgroup_min_size(mut self, value: u32) -> Self {
        self.subgroup_min_size = Some(value);
        self
    }

    pub fn subgroup_max_size(mut self, value: u32) -> Self {
        self.subgroup_max_size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: AdapterInfoExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUAdapterInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUAdapterInfo = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAdapterInfo) -> Self {
        Self {
        extensions: Vec::new(),
            vendor: Some(string_view_to_string(value.vendor)),
            architecture: Some(string_view_to_string(value.architecture)),
            device: Some(string_view_to_string(value.device)),
            description: Some(string_view_to_string(value.description)),
            backend_type: Some(value.backendType.into()),
            adapter_type: Some(value.adapterType.into()),
            vendor_id: Some(value.vendorID),
            device_id: Some(value.deviceID),
            subgroup_min_size: Some(value.subgroupMinSize),
            subgroup_max_size: Some(value.subgroupMaxSize),
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUAdapterInfo) {
        unsafe { ffi::wgpuAdapterInfoFreeMembers(value) };
    }
}

pub struct AdapterPropertiesD3D {
    pub shader_model: Option<u32>,
}

impl Default for AdapterPropertiesD3D {
    fn default() -> Self {
        Self {
        shader_model: None,
        }
    }
}

impl AdapterPropertiesD3D {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn shader_model(mut self, value: u32) -> Self {
        self.shader_model = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUAdapterPropertiesD3D {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesD3D) -> Self {
        Self {
            shader_model: Some(value.shaderModel),
                }
    }
}

pub struct AdapterPropertiesWGPU {
    pub backend_type: Option<BackendType>,
}

impl Default for AdapterPropertiesWGPU {
    fn default() -> Self {
        Self {
        backend_type: None,
        }
    }
}

impl AdapterPropertiesWGPU {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn backend_type(mut self, value: BackendType) -> Self {
        self.backend_type = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUAdapterPropertiesWGPU {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesWGPU) -> Self {
        Self {
            backend_type: Some(value.backendType.into()),
                }
    }
}

pub struct AdapterPropertiesExplicitComputeSubgroupSizeConfigs {
    pub min_explicit_compute_subgroup_size: Option<u32>,
    pub max_explicit_compute_subgroup_size: Option<u32>,
}

impl Default for AdapterPropertiesExplicitComputeSubgroupSizeConfigs {
    fn default() -> Self {
        Self {
        min_explicit_compute_subgroup_size: None,
        max_explicit_compute_subgroup_size: None,
        }
    }
}

impl AdapterPropertiesExplicitComputeSubgroupSizeConfigs {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn min_explicit_compute_subgroup_size(mut self, value: u32) -> Self {
        self.min_explicit_compute_subgroup_size = Some(value);
        self
    }

    pub fn max_explicit_compute_subgroup_size(mut self, value: u32) -> Self {
        self.max_explicit_compute_subgroup_size = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUAdapterPropertiesExplicitComputeSubgroupSizeConfigs {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesExplicitComputeSubgroupSizeConfigs) -> Self {
        Self {
            min_explicit_compute_subgroup_size: Some(value.minExplicitComputeSubgroupSize),
            max_explicit_compute_subgroup_size: Some(value.maxExplicitComputeSubgroupSize),
                }
    }
}

pub struct AdapterPropertiesMemoryHeaps {
    pub heap_count: Option<usize>,
    pub heap_info: Option<Vec<MemoryHeapInfo>>,
}

impl Default for AdapterPropertiesMemoryHeaps {
    fn default() -> Self {
        Self {
        heap_count: None,
        heap_info: None,
        }
    }
}

impl AdapterPropertiesMemoryHeaps {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn heap_count(mut self, value: usize) -> Self {
        self.heap_count = Some(value);
        self
    }

    pub fn heap_info(mut self, value: Vec<MemoryHeapInfo>) -> Self {
        self.heap_info = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUAdapterPropertiesMemoryHeaps {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesMemoryHeaps) -> Self {
        Self {
            heap_count: Some(value.heapCount),
            heap_info: if value.heapInfo.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.heapInfo, value.heapCount as usize) }
                            .iter()
                            .map(|raw| MemoryHeapInfo::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUAdapterPropertiesMemoryHeaps) {
        unsafe { ffi::wgpuAdapterPropertiesMemoryHeapsFreeMembers(value) };
    }
}

pub struct AdapterPropertiesSubgroupMatrixConfigs {
    pub config_count: Option<usize>,
    pub configs: Option<Vec<SubgroupMatrixConfig>>,
}

impl Default for AdapterPropertiesSubgroupMatrixConfigs {
    fn default() -> Self {
        Self {
        config_count: None,
        configs: None,
        }
    }
}

impl AdapterPropertiesSubgroupMatrixConfigs {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn config_count(mut self, value: usize) -> Self {
        self.config_count = Some(value);
        self
    }

    pub fn configs(mut self, value: Vec<SubgroupMatrixConfig>) -> Self {
        self.configs = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs) -> Self {
        Self {
            config_count: Some(value.configCount),
            configs: if value.configs.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.configs, value.configCount as usize) }
                            .iter()
                            .map(|raw| SubgroupMatrixConfig::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs) {
        unsafe { ffi::wgpuAdapterPropertiesSubgroupMatrixConfigsFreeMembers(value) };
    }
}

pub struct AdapterPropertiesVk {
    pub driver_version: Option<u32>,
}

impl Default for AdapterPropertiesVk {
    fn default() -> Self {
        Self {
        driver_version: None,
        }
    }
}

impl AdapterPropertiesVk {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn driver_version(mut self, value: u32) -> Self {
        self.driver_version = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUAdapterPropertiesVk {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesVk) -> Self {
        Self {
            driver_version: Some(value.driverVersion),
                }
    }
}

pub struct BindGroupDescriptor {
    pub extensions: Vec<BindGroupDescriptorExtension>,
    pub label: Option<String>,
    pub layout: Option<BindGroupLayout>,
    pub entry_count: Option<usize>,
    pub entries: Option<Vec<BindGroupEntry>>,
}

impl Default for BindGroupDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        layout: None,
        entry_count: Some(0),
        entries: None,
        }
    }
}

impl BindGroupDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn layout(mut self, value: BindGroupLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn entry_count(mut self, value: usize) -> Self {
        self.entry_count = Some(value);
        self
    }

    pub fn entries(mut self, value: Vec<BindGroupEntry>) -> Self {
        self.entries = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: BindGroupDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUBindGroupDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBindGroupDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            layout: Some(unsafe { BindGroupLayout::from_raw(value.layout) }),
            entry_count: Some(value.entryCount),
            entries: if value.entries.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.entries, value.entryCount as usize) }
                            .iter()
                            .map(|raw| BindGroupEntry::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct BindGroupEntry {
    pub extensions: Vec<BindGroupEntryExtension>,
    pub binding: Option<u32>,
    pub buffer: Option<Buffer>,
    pub offset: Option<u64>,
    pub size: Option<u64>,
    pub sampler: Option<Sampler>,
    pub texture_view: Option<TextureView>,
}

impl Default for BindGroupEntry {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        binding: None,
        buffer: None,
        offset: Some(0),
        size: None,
        sampler: None,
        texture_view: None,
        }
    }
}

impl BindGroupEntry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn binding(mut self, value: u32) -> Self {
        self.binding = Some(value);
        self
    }

    pub fn buffer(mut self, value: Buffer) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn sampler(mut self, value: Sampler) -> Self {
        self.sampler = Some(value);
        self
    }

    pub fn texture_view(mut self, value: TextureView) -> Self {
        self.texture_view = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: BindGroupEntryExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUBindGroupEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupEntry = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBindGroupEntry) -> Self {
        Self {
        extensions: Vec::new(),
            binding: Some(value.binding),
            buffer: if value.buffer.is_null() {
                    None
                } else {
                    Some(unsafe { Buffer::from_raw(value.buffer) })
                },
            offset: Some(value.offset),
            size: Some(value.size),
            sampler: if value.sampler.is_null() {
                    None
                } else {
                    Some(unsafe { Sampler::from_raw(value.sampler) })
                },
            texture_view: if value.textureView.is_null() {
                    None
                } else {
                    Some(unsafe { TextureView::from_raw(value.textureView) })
                },
                }
    }
}

pub struct BindGroupLayoutDescriptor {
    pub extensions: Vec<BindGroupLayoutDescriptorExtension>,
    pub label: Option<String>,
    pub entry_count: Option<usize>,
    pub entries: Option<Vec<BindGroupLayoutEntry>>,
}

impl Default for BindGroupLayoutDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        entry_count: Some(0),
        entries: None,
        }
    }
}

impl BindGroupLayoutDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn entry_count(mut self, value: usize) -> Self {
        self.entry_count = Some(value);
        self
    }

    pub fn entries(mut self, value: Vec<BindGroupLayoutEntry>) -> Self {
        self.entries = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: BindGroupLayoutDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUBindGroupLayoutDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupLayoutDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBindGroupLayoutDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            entry_count: Some(value.entryCount),
            entries: if value.entries.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.entries, value.entryCount as usize) }
                            .iter()
                            .map(|raw| BindGroupLayoutEntry::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct BindGroupLayoutEntry {
    pub extensions: Vec<BindGroupLayoutEntryExtension>,
    pub binding: Option<u32>,
    pub visibility: Option<ShaderStage>,
    pub binding_array_size: Option<u32>,
    pub buffer: Option<BufferBindingLayout>,
    pub sampler: Option<SamplerBindingLayout>,
    pub texture: Option<TextureBindingLayout>,
    pub storage_texture: Option<StorageTextureBindingLayout>,
}

impl Default for BindGroupLayoutEntry {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        binding: None,
        visibility: None,
        binding_array_size: Some(0),
        buffer: None,
        sampler: None,
        texture: None,
        storage_texture: None,
        }
    }
}

impl BindGroupLayoutEntry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn binding(mut self, value: u32) -> Self {
        self.binding = Some(value);
        self
    }

    pub fn visibility(mut self, value: ShaderStage) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn binding_array_size(mut self, value: u32) -> Self {
        self.binding_array_size = Some(value);
        self
    }

    pub fn buffer(mut self, value: BufferBindingLayout) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn sampler(mut self, value: SamplerBindingLayout) -> Self {
        self.sampler = Some(value);
        self
    }

    pub fn texture(mut self, value: TextureBindingLayout) -> Self {
        self.texture = Some(value);
        self
    }

    pub fn storage_texture(mut self, value: StorageTextureBindingLayout) -> Self {
        self.storage_texture = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: BindGroupLayoutEntryExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUBindGroupLayoutEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupLayoutEntry = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBindGroupLayoutEntry) -> Self {
        Self {
        extensions: Vec::new(),
            binding: Some(value.binding),
            visibility: Some(value.visibility.into()),
            binding_array_size: Some(value.bindingArraySize),
            buffer: Some(BufferBindingLayout::from_ffi(value.buffer)),
            sampler: Some(SamplerBindingLayout::from_ffi(value.sampler)),
            texture: Some(TextureBindingLayout::from_ffi(value.texture)),
            storage_texture: Some(StorageTextureBindingLayout::from_ffi(value.storageTexture)),
                }
    }
}

pub struct BindingResource {
    pub extensions: Vec<BindingResourceExtension>,
    pub buffer: Option<Buffer>,
    pub offset: Option<u64>,
    pub size: Option<u64>,
    pub sampler: Option<Sampler>,
    pub texture_view: Option<TextureView>,
}

impl Default for BindingResource {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        buffer: None,
        offset: Some(0),
        size: None,
        sampler: None,
        texture_view: None,
        }
    }
}

impl BindingResource {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn buffer(mut self, value: Buffer) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn sampler(mut self, value: Sampler) -> Self {
        self.sampler = Some(value);
        self
    }

    pub fn texture_view(mut self, value: TextureView) -> Self {
        self.texture_view = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: BindingResourceExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUBindingResource, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindingResource = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBindingResource) -> Self {
        Self {
        extensions: Vec::new(),
            buffer: if value.buffer.is_null() {
                    None
                } else {
                    Some(unsafe { Buffer::from_raw(value.buffer) })
                },
            offset: Some(value.offset),
            size: Some(value.size),
            sampler: if value.sampler.is_null() {
                    None
                } else {
                    Some(unsafe { Sampler::from_raw(value.sampler) })
                },
            texture_view: if value.textureView.is_null() {
                    None
                } else {
                    Some(unsafe { TextureView::from_raw(value.textureView) })
                },
                }
    }
}

pub struct BlendComponent {
    pub operation: Option<BlendOperation>,
    pub src_factor: Option<BlendFactor>,
    pub dst_factor: Option<BlendFactor>,
}

impl Default for BlendComponent {
    fn default() -> Self {
        Self {
        operation: Some(BlendOperation::Add),
        src_factor: Some(BlendFactor::One),
        dst_factor: Some(BlendFactor::Zero),
        }
    }
}

impl BlendComponent {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn operation(mut self, value: BlendOperation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn src_factor(mut self, value: BlendFactor) -> Self {
        self.src_factor = Some(value);
        self
    }

    pub fn dst_factor(mut self, value: BlendFactor) -> Self {
        self.dst_factor = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUBlendComponent {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBlendComponent) -> Self {
        Self {
            operation: Some(value.operation.into()),
            src_factor: Some(value.srcFactor.into()),
            dst_factor: Some(value.dstFactor.into()),
                }
    }
}

pub struct BlendState {
    pub color: Option<BlendComponent>,
    pub alpha: Option<BlendComponent>,
}

impl Default for BlendState {
    fn default() -> Self {
        Self {
        color: None,
        alpha: None,
        }
    }
}

impl BlendState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn color(mut self, value: BlendComponent) -> Self {
        self.color = Some(value);
        self
    }

    pub fn alpha(mut self, value: BlendComponent) -> Self {
        self.alpha = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUBlendState {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBlendState) -> Self {
        Self {
            color: Some(BlendComponent::from_ffi(value.color)),
            alpha: Some(BlendComponent::from_ffi(value.alpha)),
                }
    }
}

pub struct BufferBindingLayout {
    pub extensions: Vec<BufferBindingLayoutExtension>,
    pub r#type: Option<BufferBindingType>,
    pub has_dynamic_offset: Option<bool>,
    pub min_binding_size: Option<u64>,
}

impl Default for BufferBindingLayout {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        r#type: Some(BufferBindingType::Uniform),
        has_dynamic_offset: None,
        min_binding_size: Some(0),
        }
    }
}

impl BufferBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn r#type(mut self, value: BufferBindingType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn has_dynamic_offset(mut self, value: bool) -> Self {
        self.has_dynamic_offset = Some(value);
        self
    }

    pub fn min_binding_size(mut self, value: u64) -> Self {
        self.min_binding_size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: BufferBindingLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUBufferBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBufferBindingLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBufferBindingLayout) -> Self {
        Self {
        extensions: Vec::new(),
            r#type: Some(value.type_.into()),
            has_dynamic_offset: Some(value.hasDynamicOffset != 0),
            min_binding_size: Some(value.minBindingSize),
                }
    }
}

pub struct BufferDescriptor {
    pub extensions: Vec<BufferDescriptorExtension>,
    pub label: Option<String>,
    pub usage: Option<BufferUsage>,
    pub size: Option<u64>,
    pub mapped_at_creation: Option<bool>,
}

impl Default for BufferDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        usage: None,
        size: None,
        mapped_at_creation: None,
        }
    }
}

impl BufferDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn usage(mut self, value: BufferUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn mapped_at_creation(mut self, value: bool) -> Self {
        self.mapped_at_creation = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: BufferDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUBufferDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBufferDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBufferDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            usage: Some(value.usage.into()),
            size: Some(value.size),
            mapped_at_creation: Some(value.mappedAtCreation != 0),
                }
    }
}

pub struct BufferHostMappedPointer {
    pub pointer: Option<*mut std::ffi::c_void>,
    pub dispose_callback: Option<Callback>,
    pub userdata: Option<*mut std::ffi::c_void>,
}

impl Default for BufferHostMappedPointer {
    fn default() -> Self {
        Self {
        pointer: None,
        dispose_callback: None,
        userdata: None,
        }
    }
}

impl BufferHostMappedPointer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pointer(mut self, value: *mut std::ffi::c_void) -> Self {
        self.pointer = Some(value);
        self
    }

    pub fn dispose_callback(mut self, value: Callback) -> Self {
        self.dispose_callback = Some(value);
        self
    }

    pub fn userdata(mut self, value: *mut std::ffi::c_void) -> Self {
        self.userdata = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUBufferHostMappedPointer {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUBufferHostMappedPointer) -> Self {
        Self {
            pointer: Some(value.pointer),
            dispose_callback: Some(value.disposeCallback),
            userdata: Some(value.userdata),
                }
    }
}

pub struct Color {
    pub r: Option<f64>,
    pub g: Option<f64>,
    pub b: Option<f64>,
    pub a: Option<f64>,
}

impl Default for Color {
    fn default() -> Self {
        Self {
        r: None,
        g: None,
        b: None,
        a: None,
        }
    }
}

impl Color {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn r(mut self, value: f64) -> Self {
        self.r = Some(value);
        self
    }

    pub fn g(mut self, value: f64) -> Self {
        self.g = Some(value);
        self
    }

    pub fn b(mut self, value: f64) -> Self {
        self.b = Some(value);
        self
    }

    pub fn a(mut self, value: f64) -> Self {
        self.a = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUColor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUColor) -> Self {
        Self {
            r: Some(value.r),
            g: Some(value.g),
            b: Some(value.b),
            a: Some(value.a),
                }
    }
}

pub struct ColorTargetState {
    pub extensions: Vec<ColorTargetStateExtension>,
    pub format: Option<TextureFormat>,
    pub blend: Option<BlendState>,
    pub write_mask: Option<ColorWriteMask>,
}

impl Default for ColorTargetState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        format: None,
        blend: None,
        write_mask: Some(ColorWriteMask::ALL),
        }
    }
}

impl ColorTargetState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn blend(mut self, value: BlendState) -> Self {
        self.blend = Some(value);
        self
    }

    pub fn write_mask(mut self, value: ColorWriteMask) -> Self {
        self.write_mask = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ColorTargetStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUColorTargetState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUColorTargetState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUColorTargetState) -> Self {
        Self {
        extensions: Vec::new(),
            format: Some(value.format.into()),
            blend: if value.blend.is_null() {
                    None
                } else {
                    Some(BlendState::from_ffi(unsafe { *value.blend }))
                },
            write_mask: Some(value.writeMask.into()),
                }
    }
}

pub struct ColorTargetStateExpandResolveTextureDawn {
    pub enabled: Option<bool>,
}

impl Default for ColorTargetStateExpandResolveTextureDawn {
    fn default() -> Self {
        Self {
        enabled: None,
        }
    }
}

impl ColorTargetStateExpandResolveTextureDawn {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUColorTargetStateExpandResolveTextureDawn {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUColorTargetStateExpandResolveTextureDawn) -> Self {
        Self {
            enabled: Some(value.enabled != 0),
                }
    }
}

pub struct CommandBufferDescriptor {
    pub extensions: Vec<CommandBufferDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for CommandBufferDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl CommandBufferDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: CommandBufferDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUCommandBufferDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCommandBufferDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUCommandBufferDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct CommandEncoderDescriptor {
    pub extensions: Vec<CommandEncoderDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for CommandEncoderDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl CommandEncoderDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: CommandEncoderDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUCommandEncoderDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCommandEncoderDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUCommandEncoderDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct CompilationInfo {
    pub extensions: Vec<CompilationInfoExtension>,
    pub message_count: Option<usize>,
    pub messages: Option<Vec<CompilationMessage>>,
}

impl Default for CompilationInfo {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        message_count: None,
        messages: None,
        }
    }
}

impl CompilationInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message_count(mut self, value: usize) -> Self {
        self.message_count = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<CompilationMessage>) -> Self {
        self.messages = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: CompilationInfoExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUCompilationInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCompilationInfo = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUCompilationInfo) -> Self {
        Self {
        extensions: Vec::new(),
            message_count: Some(value.messageCount),
            messages: if value.messages.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.messages, value.messageCount as usize) }
                            .iter()
                            .map(|raw| CompilationMessage::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct CompilationMessage {
    pub extensions: Vec<CompilationMessageExtension>,
    pub message: Option<String>,
    pub r#type: Option<CompilationMessageType>,
    pub line_num: Option<u64>,
    pub line_pos: Option<u64>,
    pub offset: Option<u64>,
    pub length: Option<u64>,
}

impl Default for CompilationMessage {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        message: None,
        r#type: None,
        line_num: None,
        line_pos: None,
        offset: None,
        length: None,
        }
    }
}

impl CompilationMessage {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn r#type(mut self, value: CompilationMessageType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn line_num(mut self, value: u64) -> Self {
        self.line_num = Some(value);
        self
    }

    pub fn line_pos(mut self, value: u64) -> Self {
        self.line_pos = Some(value);
        self
    }

    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn length(mut self, value: u64) -> Self {
        self.length = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: CompilationMessageExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUCompilationMessage, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCompilationMessage = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUCompilationMessage) -> Self {
        Self {
        extensions: Vec::new(),
            message: Some(string_view_to_string(value.message)),
            r#type: Some(value.type_.into()),
            line_num: Some(value.lineNum),
            line_pos: Some(value.linePos),
            offset: Some(value.offset),
            length: Some(value.length),
                }
    }
}

pub struct ComputePassDescriptor {
    pub extensions: Vec<ComputePassDescriptorExtension>,
    pub label: Option<String>,
    pub timestamp_writes: Option<PassTimestampWrites>,
}

impl Default for ComputePassDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        timestamp_writes: None,
        }
    }
}

impl ComputePassDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn timestamp_writes(mut self, value: PassTimestampWrites) -> Self {
        self.timestamp_writes = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ComputePassDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUComputePassDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUComputePassDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUComputePassDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            timestamp_writes: if value.timestampWrites.is_null() {
                    None
                } else {
                    Some(PassTimestampWrites::from_ffi(unsafe { *value.timestampWrites }))
                },
                }
    }
}

pub struct ComputePipelineDescriptor {
    pub extensions: Vec<ComputePipelineDescriptorExtension>,
    pub label: Option<String>,
    pub layout: Option<PipelineLayout>,
    pub compute: Option<ComputeState>,
}

impl Default for ComputePipelineDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        layout: None,
        compute: None,
        }
    }
}

impl ComputePipelineDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn layout(mut self, value: PipelineLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn compute(mut self, value: ComputeState) -> Self {
        self.compute = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ComputePipelineDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUComputePipelineDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUComputePipelineDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUComputePipelineDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            layout: if value.layout.is_null() {
                    None
                } else {
                    Some(unsafe { PipelineLayout::from_raw(value.layout) })
                },
            compute: Some(ComputeState::from_ffi(value.compute)),
                }
    }
}

pub struct ComputeState {
    pub extensions: Vec<ComputeStateExtension>,
    pub module: Option<ShaderModule>,
    pub entry_point: Option<String>,
    pub constant_count: Option<usize>,
    pub constants: Option<Vec<ConstantEntry>>,
}

impl Default for ComputeState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        module: None,
        entry_point: None,
        constant_count: Some(0),
        constants: None,
        }
    }
}

impl ComputeState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn module(mut self, value: ShaderModule) -> Self {
        self.module = Some(value);
        self
    }

    pub fn entry_point(mut self, value: String) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn constant_count(mut self, value: usize) -> Self {
        self.constant_count = Some(value);
        self
    }

    pub fn constants(mut self, value: Vec<ConstantEntry>) -> Self {
        self.constants = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ComputeStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUComputeState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUComputeState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUComputeState) -> Self {
        Self {
        extensions: Vec::new(),
            module: Some(unsafe { ShaderModule::from_raw(value.module) }),
            entry_point: if value.entryPoint.data.is_null() || value.entryPoint.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.entryPoint))
                },
            constant_count: Some(value.constantCount),
            constants: if value.constants.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.constants, value.constantCount as usize) }
                            .iter()
                            .map(|raw| ConstantEntry::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct ConstantEntry {
    pub extensions: Vec<ConstantEntryExtension>,
    pub key: Option<String>,
    pub value: Option<f64>,
}

impl Default for ConstantEntry {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        key: None,
        value: None,
        }
    }
}

impl ConstantEntry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn key(mut self, value: String) -> Self {
        self.key = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ConstantEntryExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUConstantEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUConstantEntry = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUConstantEntry) -> Self {
        Self {
        extensions: Vec::new(),
            key: Some(string_view_to_string(value.key)),
            value: Some(value.value),
                }
    }
}

pub struct CopyTextureForBrowserOptions {
    pub extensions: Vec<CopyTextureForBrowserOptionsExtension>,
    pub flip_y: Option<bool>,
    pub needs_color_space_conversion: Option<bool>,
    pub src_alpha_mode: Option<AlphaMode>,
    pub src_transfer_function_parameters: Option<Vec<f32>>,
    pub conversion_matrix: Option<Vec<f32>>,
    pub dst_transfer_function_parameters: Option<Vec<f32>>,
    pub dst_alpha_mode: Option<AlphaMode>,
    pub internal_usage: Option<bool>,
}

impl Default for CopyTextureForBrowserOptions {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        flip_y: None,
        needs_color_space_conversion: None,
        src_alpha_mode: Some(AlphaMode::Unpremultiplied),
        src_transfer_function_parameters: None,
        conversion_matrix: None,
        dst_transfer_function_parameters: None,
        dst_alpha_mode: Some(AlphaMode::Unpremultiplied),
        internal_usage: None,
        }
    }
}

impl CopyTextureForBrowserOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn flip_y(mut self, value: bool) -> Self {
        self.flip_y = Some(value);
        self
    }

    pub fn needs_color_space_conversion(mut self, value: bool) -> Self {
        self.needs_color_space_conversion = Some(value);
        self
    }

    pub fn src_alpha_mode(mut self, value: AlphaMode) -> Self {
        self.src_alpha_mode = Some(value);
        self
    }

    pub fn src_transfer_function_parameters(mut self, value: Vec<f32>) -> Self {
        self.src_transfer_function_parameters = Some(value);
        self
    }

    pub fn conversion_matrix(mut self, value: Vec<f32>) -> Self {
        self.conversion_matrix = Some(value);
        self
    }

    pub fn dst_transfer_function_parameters(mut self, value: Vec<f32>) -> Self {
        self.dst_transfer_function_parameters = Some(value);
        self
    }

    pub fn dst_alpha_mode(mut self, value: AlphaMode) -> Self {
        self.dst_alpha_mode = Some(value);
        self
    }

    pub fn internal_usage(mut self, value: bool) -> Self {
        self.internal_usage = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: CopyTextureForBrowserOptionsExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUCopyTextureForBrowserOptions, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCopyTextureForBrowserOptions = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUCopyTextureForBrowserOptions) -> Self {
        Self {
        extensions: Vec::new(),
            flip_y: Some(value.flipY != 0),
            needs_color_space_conversion: Some(value.needsColorSpaceConversion != 0),
            src_alpha_mode: Some(value.srcAlphaMode.into()),
            src_transfer_function_parameters: if value.srcTransferFunctionParameters.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.srcTransferFunctionParameters, 7usize) }.to_vec())
                },
            conversion_matrix: if value.conversionMatrix.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.conversionMatrix, 9usize) }.to_vec())
                },
            dst_transfer_function_parameters: if value.dstTransferFunctionParameters.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.dstTransferFunctionParameters, 7usize) }.to_vec())
                },
            dst_alpha_mode: Some(value.dstAlphaMode.into()),
            internal_usage: Some(value.internalUsage != 0),
                }
    }
}

pub struct DawnWGSLBlocklist {
    pub blocklisted_feature_count: Option<usize>,
    pub blocklisted_features: Option<Vec<String>>,
}

impl Default for DawnWGSLBlocklist {
    fn default() -> Self {
        Self {
        blocklisted_feature_count: Some(0),
        blocklisted_features: None,
        }
    }
}

impl DawnWGSLBlocklist {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn blocklisted_feature_count(mut self, value: usize) -> Self {
        self.blocklisted_feature_count = Some(value);
        self
    }

    pub fn blocklisted_features(mut self, value: Vec<String>) -> Self {
        self.blocklisted_features = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnWGSLBlocklist {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnWGSLBlocklist) -> Self {
        Self {
            blocklisted_feature_count: Some(value.blocklistedFeatureCount),
            blocklisted_features: if value.blocklistedFeatures.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.blocklistedFeatures, value.blocklistedFeatureCount as usize) }
                            .iter()
                            .map(|raw| {
                                if raw.is_null() {
                                    String::new()
                                } else {
                                    unsafe { CStr::from_ptr(*raw) }.to_string_lossy().into_owned()
                                }
                            })
                            .collect(),
                    )
                },
                }
    }
}

pub struct DawnAdapterPropertiesPowerPreference {
    pub power_preference: Option<PowerPreference>,
}

impl Default for DawnAdapterPropertiesPowerPreference {
    fn default() -> Self {
        Self {
        power_preference: None,
        }
    }
}

impl DawnAdapterPropertiesPowerPreference {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn power_preference(mut self, value: PowerPreference) -> Self {
        self.power_preference = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnAdapterPropertiesPowerPreference {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnAdapterPropertiesPowerPreference) -> Self {
        Self {
            power_preference: Some(value.powerPreference.into()),
                }
    }
}

pub struct DawnBufferDescriptorErrorInfoFromWireClient {
    pub out_of_memory: Option<bool>,
}

impl Default for DawnBufferDescriptorErrorInfoFromWireClient {
    fn default() -> Self {
        Self {
        out_of_memory: None,
        }
    }
}

impl DawnBufferDescriptorErrorInfoFromWireClient {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn out_of_memory(mut self, value: bool) -> Self {
        self.out_of_memory = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnBufferDescriptorErrorInfoFromWireClient {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnBufferDescriptorErrorInfoFromWireClient) -> Self {
        Self {
            out_of_memory: Some(value.outOfMemory != 0),
                }
    }
}

pub struct DawnCacheDeviceDescriptor {
    pub isolation_key: Option<String>,
    pub load_data_function: Option<DawnLoadCacheDataFunction>,
    pub store_data_function: Option<DawnStoreCacheDataFunction>,
    pub function_userdata: Option<*mut std::ffi::c_void>,
}

impl Default for DawnCacheDeviceDescriptor {
    fn default() -> Self {
        Self {
        isolation_key: None,
        load_data_function: None,
        store_data_function: None,
        function_userdata: None,
        }
    }
}

impl DawnCacheDeviceDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn isolation_key(mut self, value: String) -> Self {
        self.isolation_key = Some(value);
        self
    }

    pub fn load_data_function(mut self, value: DawnLoadCacheDataFunction) -> Self {
        self.load_data_function = Some(value);
        self
    }

    pub fn store_data_function(mut self, value: DawnStoreCacheDataFunction) -> Self {
        self.store_data_function = Some(value);
        self
    }

    pub fn function_userdata(mut self, value: *mut std::ffi::c_void) -> Self {
        self.function_userdata = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnCacheDeviceDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnCacheDeviceDescriptor) -> Self {
        Self {
            isolation_key: Some(string_view_to_string(value.isolationKey)),
            load_data_function: Some(value.loadDataFunction),
            store_data_function: Some(value.storeDataFunction),
            function_userdata: Some(value.functionUserdata),
                }
    }
}

pub struct DawnCompilationMessageUtf16 {
    pub line_pos: Option<u64>,
    pub offset: Option<u64>,
    pub length: Option<u64>,
}

impl Default for DawnCompilationMessageUtf16 {
    fn default() -> Self {
        Self {
        line_pos: None,
        offset: None,
        length: None,
        }
    }
}

impl DawnCompilationMessageUtf16 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn line_pos(mut self, value: u64) -> Self {
        self.line_pos = Some(value);
        self
    }

    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn length(mut self, value: u64) -> Self {
        self.length = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnCompilationMessageUtf16 {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnCompilationMessageUtf16) -> Self {
        Self {
            line_pos: Some(value.linePos),
            offset: Some(value.offset),
            length: Some(value.length),
                }
    }
}

pub struct DawnConsumeAdapterDescriptor {
    pub consume_adapter: Option<bool>,
}

impl Default for DawnConsumeAdapterDescriptor {
    fn default() -> Self {
        Self {
        consume_adapter: None,
        }
    }
}

impl DawnConsumeAdapterDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn consume_adapter(mut self, value: bool) -> Self {
        self.consume_adapter = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnConsumeAdapterDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnConsumeAdapterDescriptor) -> Self {
        Self {
            consume_adapter: Some(value.consumeAdapter != 0),
                }
    }
}

pub struct DawnDeviceAllocatorControl {
    pub allocator_heap_block_size: Option<usize>,
}

impl Default for DawnDeviceAllocatorControl {
    fn default() -> Self {
        Self {
        allocator_heap_block_size: Some(0),
        }
    }
}

impl DawnDeviceAllocatorControl {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn allocator_heap_block_size(mut self, value: usize) -> Self {
        self.allocator_heap_block_size = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnDeviceAllocatorControl {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnDeviceAllocatorControl) -> Self {
        Self {
            allocator_heap_block_size: Some(value.allocatorHeapBlockSize),
                }
    }
}

pub struct DawnDrmFormatCapabilities {
    pub properties_count: Option<usize>,
    pub properties: Option<Vec<DawnDrmFormatProperties>>,
}

impl Default for DawnDrmFormatCapabilities {
    fn default() -> Self {
        Self {
        properties_count: None,
        properties: None,
        }
    }
}

impl DawnDrmFormatCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn properties_count(mut self, value: usize) -> Self {
        self.properties_count = Some(value);
        self
    }

    pub fn properties(mut self, value: Vec<DawnDrmFormatProperties>) -> Self {
        self.properties = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnDrmFormatCapabilities {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnDrmFormatCapabilities) -> Self {
        Self {
            properties_count: Some(value.propertiesCount),
            properties: if value.properties.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.properties, value.propertiesCount as usize) }
                            .iter()
                            .map(|raw| DawnDrmFormatProperties::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUDawnDrmFormatCapabilities) {
        unsafe { ffi::wgpuDawnDrmFormatCapabilitiesFreeMembers(value) };
    }
}

pub struct DawnDrmFormatProperties {
    pub modifier: Option<u64>,
    pub modifier_plane_count: Option<u32>,
}

impl Default for DawnDrmFormatProperties {
    fn default() -> Self {
        Self {
        modifier: None,
        modifier_plane_count: None,
        }
    }
}

impl DawnDrmFormatProperties {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn modifier(mut self, value: u64) -> Self {
        self.modifier = Some(value);
        self
    }

    pub fn modifier_plane_count(mut self, value: u32) -> Self {
        self.modifier_plane_count = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnDrmFormatProperties {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnDrmFormatProperties) -> Self {
        Self {
            modifier: Some(value.modifier),
            modifier_plane_count: Some(value.modifierPlaneCount),
                }
    }
}

pub struct DawnEncoderInternalUsageDescriptor {
    pub use_internal_usages: Option<bool>,
}

impl Default for DawnEncoderInternalUsageDescriptor {
    fn default() -> Self {
        Self {
        use_internal_usages: None,
        }
    }
}

impl DawnEncoderInternalUsageDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn use_internal_usages(mut self, value: bool) -> Self {
        self.use_internal_usages = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnEncoderInternalUsageDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnEncoderInternalUsageDescriptor) -> Self {
        Self {
            use_internal_usages: Some(value.useInternalUsages != 0),
                }
    }
}

pub struct DawnFakeBufferOOMForTesting {
    pub fake_oom_at_wire_client_map: Option<bool>,
    pub fake_oom_at_native_map: Option<bool>,
    pub fake_oom_at_device: Option<bool>,
}

impl Default for DawnFakeBufferOOMForTesting {
    fn default() -> Self {
        Self {
        fake_oom_at_wire_client_map: None,
        fake_oom_at_native_map: None,
        fake_oom_at_device: None,
        }
    }
}

impl DawnFakeBufferOOMForTesting {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn fake_oom_at_wire_client_map(mut self, value: bool) -> Self {
        self.fake_oom_at_wire_client_map = Some(value);
        self
    }

    pub fn fake_oom_at_native_map(mut self, value: bool) -> Self {
        self.fake_oom_at_native_map = Some(value);
        self
    }

    pub fn fake_oom_at_device(mut self, value: bool) -> Self {
        self.fake_oom_at_device = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnFakeBufferOOMForTesting {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnFakeBufferOOMForTesting) -> Self {
        Self {
            fake_oom_at_wire_client_map: Some(value.fakeOOMAtWireClientMap != 0),
            fake_oom_at_native_map: Some(value.fakeOOMAtNativeMap != 0),
            fake_oom_at_device: Some(value.fakeOOMAtDevice != 0),
                }
    }
}

pub struct DawnFakeDeviceInitializeErrorForTesting {

}

impl Default for DawnFakeDeviceInitializeErrorForTesting {
    fn default() -> Self {
        Self {

        }
    }
}

impl DawnFakeDeviceInitializeErrorForTesting {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_ffi(&self) -> ffi::WGPUDawnFakeDeviceInitializeErrorForTesting {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnFakeDeviceInitializeErrorForTesting) -> Self {
        let _ = value;
                Self::default()
    }
}

pub struct DawnFormatCapabilities {
    pub extensions: Vec<DawnFormatCapabilitiesExtension>,
}

impl Default for DawnFormatCapabilities {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        }
    }
}

impl DawnFormatCapabilities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_extension(mut self, extension: DawnFormatCapabilitiesExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUDawnFormatCapabilities, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUDawnFormatCapabilities = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnFormatCapabilities) -> Self {
        Self {
        extensions: Vec::new(),
                }
    }
}

pub struct DawnHostMappedPointerLimits {
    pub host_mapped_pointer_alignment: Option<u32>,
}

impl Default for DawnHostMappedPointerLimits {
    fn default() -> Self {
        Self {
        host_mapped_pointer_alignment: None,
        }
    }
}

impl DawnHostMappedPointerLimits {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn host_mapped_pointer_alignment(mut self, value: u32) -> Self {
        self.host_mapped_pointer_alignment = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnHostMappedPointerLimits {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnHostMappedPointerLimits) -> Self {
        Self {
            host_mapped_pointer_alignment: Some(value.hostMappedPointerAlignment),
                }
    }
}

pub struct DawnInjectedInvalidSType {
    pub invalid_s_type: Option<SType>,
}

impl Default for DawnInjectedInvalidSType {
    fn default() -> Self {
        Self {
        invalid_s_type: None,
        }
    }
}

impl DawnInjectedInvalidSType {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn invalid_s_type(mut self, value: SType) -> Self {
        self.invalid_s_type = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnInjectedInvalidSType {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnInjectedInvalidSType) -> Self {
        Self {
            invalid_s_type: Some(value.invalidSType.into()),
                }
    }
}

pub struct DawnRenderPassColorAttachmentRenderToSingleSampled {
    pub implicit_sample_count: Option<u32>,
}

impl Default for DawnRenderPassColorAttachmentRenderToSingleSampled {
    fn default() -> Self {
        Self {
        implicit_sample_count: Some(1),
        }
    }
}

impl DawnRenderPassColorAttachmentRenderToSingleSampled {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn implicit_sample_count(mut self, value: u32) -> Self {
        self.implicit_sample_count = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnRenderPassColorAttachmentRenderToSingleSampled {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnRenderPassColorAttachmentRenderToSingleSampled) -> Self {
        Self {
            implicit_sample_count: Some(value.implicitSampleCount),
                }
    }
}

pub struct DawnShaderModuleSPIRVOptionsDescriptor {
    pub allow_non_uniform_derivatives: Option<bool>,
}

impl Default for DawnShaderModuleSPIRVOptionsDescriptor {
    fn default() -> Self {
        Self {
        allow_non_uniform_derivatives: None,
        }
    }
}

impl DawnShaderModuleSPIRVOptionsDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn allow_non_uniform_derivatives(mut self, value: bool) -> Self {
        self.allow_non_uniform_derivatives = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnShaderModuleSPIRVOptionsDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnShaderModuleSPIRVOptionsDescriptor) -> Self {
        Self {
            allow_non_uniform_derivatives: Some(value.allowNonUniformDerivatives != 0),
                }
    }
}

pub struct DawnTexelCopyBufferRowAlignmentLimits {
    pub min_texel_copy_buffer_row_alignment: Option<u32>,
}

impl Default for DawnTexelCopyBufferRowAlignmentLimits {
    fn default() -> Self {
        Self {
        min_texel_copy_buffer_row_alignment: None,
        }
    }
}

impl DawnTexelCopyBufferRowAlignmentLimits {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn min_texel_copy_buffer_row_alignment(mut self, value: u32) -> Self {
        self.min_texel_copy_buffer_row_alignment = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnTexelCopyBufferRowAlignmentLimits {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnTexelCopyBufferRowAlignmentLimits) -> Self {
        Self {
            min_texel_copy_buffer_row_alignment: Some(value.minTexelCopyBufferRowAlignment),
                }
    }
}

pub struct DawnTextureInternalUsageDescriptor {
    pub internal_usage: Option<TextureUsage>,
}

impl Default for DawnTextureInternalUsageDescriptor {
    fn default() -> Self {
        Self {
        internal_usage: None,
        }
    }
}

impl DawnTextureInternalUsageDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn internal_usage(mut self, value: TextureUsage) -> Self {
        self.internal_usage = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnTextureInternalUsageDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnTextureInternalUsageDescriptor) -> Self {
        Self {
            internal_usage: Some(value.internalUsage.into()),
                }
    }
}

pub struct DawnTogglesDescriptor {
    pub enabled_toggle_count: Option<usize>,
    pub enabled_toggles: Option<Vec<String>>,
    pub disabled_toggle_count: Option<usize>,
    pub disabled_toggles: Option<Vec<String>>,
}

impl Default for DawnTogglesDescriptor {
    fn default() -> Self {
        Self {
        enabled_toggle_count: Some(0),
        enabled_toggles: None,
        disabled_toggle_count: Some(0),
        disabled_toggles: None,
        }
    }
}

impl DawnTogglesDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn enabled_toggle_count(mut self, value: usize) -> Self {
        self.enabled_toggle_count = Some(value);
        self
    }

    pub fn enabled_toggles(mut self, value: Vec<String>) -> Self {
        self.enabled_toggles = Some(value);
        self
    }

    pub fn disabled_toggle_count(mut self, value: usize) -> Self {
        self.disabled_toggle_count = Some(value);
        self
    }

    pub fn disabled_toggles(mut self, value: Vec<String>) -> Self {
        self.disabled_toggles = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnTogglesDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnTogglesDescriptor) -> Self {
        Self {
            enabled_toggle_count: Some(value.enabledToggleCount),
            enabled_toggles: if value.enabledToggles.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.enabledToggles, value.enabledToggleCount as usize) }
                            .iter()
                            .map(|raw| {
                                if raw.is_null() {
                                    String::new()
                                } else {
                                    unsafe { CStr::from_ptr(*raw) }.to_string_lossy().into_owned()
                                }
                            })
                            .collect(),
                    )
                },
            disabled_toggle_count: Some(value.disabledToggleCount),
            disabled_toggles: if value.disabledToggles.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.disabledToggles, value.disabledToggleCount as usize) }
                            .iter()
                            .map(|raw| {
                                if raw.is_null() {
                                    String::new()
                                } else {
                                    unsafe { CStr::from_ptr(*raw) }.to_string_lossy().into_owned()
                                }
                            })
                            .collect(),
                    )
                },
                }
    }
}

pub struct DawnWireWGSLControl {
    pub enable_experimental: Option<bool>,
    pub enable_unsafe: Option<bool>,
    pub enable_testing: Option<bool>,
}

impl Default for DawnWireWGSLControl {
    fn default() -> Self {
        Self {
        enable_experimental: None,
        enable_unsafe: None,
        enable_testing: None,
        }
    }
}

impl DawnWireWGSLControl {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn enable_experimental(mut self, value: bool) -> Self {
        self.enable_experimental = Some(value);
        self
    }

    pub fn enable_unsafe(mut self, value: bool) -> Self {
        self.enable_unsafe = Some(value);
        self
    }

    pub fn enable_testing(mut self, value: bool) -> Self {
        self.enable_testing = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUDawnWireWGSLControl {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDawnWireWGSLControl) -> Self {
        Self {
            enable_experimental: Some(value.enableExperimental != 0),
            enable_unsafe: Some(value.enableUnsafe != 0),
            enable_testing: Some(value.enableTesting != 0),
                }
    }
}

pub struct DepthStencilState {
    pub extensions: Vec<DepthStencilStateExtension>,
    pub format: Option<TextureFormat>,
    pub depth_write_enabled: Option<OptionalBool>,
    pub depth_compare: Option<CompareFunction>,
    pub stencil_front: Option<StencilFaceState>,
    pub stencil_back: Option<StencilFaceState>,
    pub stencil_read_mask: Option<u32>,
    pub stencil_write_mask: Option<u32>,
    pub depth_bias: Option<i32>,
    pub depth_bias_slope_scale: Option<f32>,
    pub depth_bias_clamp: Option<f32>,
}

impl Default for DepthStencilState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        format: None,
        depth_write_enabled: None,
        depth_compare: None,
        stencil_front: None,
        stencil_back: None,
        stencil_read_mask: Some(4294967295),
        stencil_write_mask: Some(4294967295),
        depth_bias: Some(0),
        depth_bias_slope_scale: None,
        depth_bias_clamp: None,
        }
    }
}

impl DepthStencilState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn depth_write_enabled(mut self, value: OptionalBool) -> Self {
        self.depth_write_enabled = Some(value);
        self
    }

    pub fn depth_compare(mut self, value: CompareFunction) -> Self {
        self.depth_compare = Some(value);
        self
    }

    pub fn stencil_front(mut self, value: StencilFaceState) -> Self {
        self.stencil_front = Some(value);
        self
    }

    pub fn stencil_back(mut self, value: StencilFaceState) -> Self {
        self.stencil_back = Some(value);
        self
    }

    pub fn stencil_read_mask(mut self, value: u32) -> Self {
        self.stencil_read_mask = Some(value);
        self
    }

    pub fn stencil_write_mask(mut self, value: u32) -> Self {
        self.stencil_write_mask = Some(value);
        self
    }

    pub fn depth_bias(mut self, value: i32) -> Self {
        self.depth_bias = Some(value);
        self
    }

    pub fn depth_bias_slope_scale(mut self, value: f32) -> Self {
        self.depth_bias_slope_scale = Some(value);
        self
    }

    pub fn depth_bias_clamp(mut self, value: f32) -> Self {
        self.depth_bias_clamp = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: DepthStencilStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUDepthStencilState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUDepthStencilState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDepthStencilState) -> Self {
        Self {
        extensions: Vec::new(),
            format: Some(value.format.into()),
            depth_write_enabled: Some(value.depthWriteEnabled.into()),
            depth_compare: Some(value.depthCompare.into()),
            stencil_front: Some(StencilFaceState::from_ffi(value.stencilFront)),
            stencil_back: Some(StencilFaceState::from_ffi(value.stencilBack)),
            stencil_read_mask: Some(value.stencilReadMask),
            stencil_write_mask: Some(value.stencilWriteMask),
            depth_bias: Some(value.depthBias),
            depth_bias_slope_scale: Some(value.depthBiasSlopeScale),
            depth_bias_clamp: Some(value.depthBiasClamp),
                }
    }
}

pub struct DeviceDescriptor {
    pub extensions: Vec<DeviceDescriptorExtension>,
    pub label: Option<String>,
    pub required_feature_count: Option<usize>,
    pub required_features: Option<Vec<FeatureName>>,
    pub required_limits: Option<Limits>,
    pub default_queue: Option<QueueDescriptor>,
    pub device_lost_callback_info: Option<DeviceLostCallbackInfo>,
    pub uncaptured_error_callback_info: Option<UncapturedErrorCallbackInfo>,
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        required_feature_count: Some(0),
        required_features: None,
        required_limits: None,
        default_queue: None,
        device_lost_callback_info: None,
        uncaptured_error_callback_info: None,
        }
    }
}

impl DeviceDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn required_feature_count(mut self, value: usize) -> Self {
        self.required_feature_count = Some(value);
        self
    }

    pub fn required_features(mut self, value: Vec<FeatureName>) -> Self {
        self.required_features = Some(value);
        self
    }

    pub fn required_limits(mut self, value: Limits) -> Self {
        self.required_limits = Some(value);
        self
    }

    pub fn default_queue(mut self, value: QueueDescriptor) -> Self {
        self.default_queue = Some(value);
        self
    }

    pub fn device_lost_callback_info(mut self, value: DeviceLostCallbackInfo) -> Self {
        self.device_lost_callback_info = Some(value);
        self
    }

    pub fn uncaptured_error_callback_info(mut self, value: UncapturedErrorCallbackInfo) -> Self {
        self.uncaptured_error_callback_info = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: DeviceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUDeviceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUDeviceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUDeviceDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            required_feature_count: Some(value.requiredFeatureCount),
            required_features: if value.requiredFeatures.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.requiredFeatures, value.requiredFeatureCount as usize) }
                            .iter()
                            .map(|raw| FeatureName::from(*raw))
                            .collect(),
                    )
                },
            required_limits: if value.requiredLimits.is_null() {
                    None
                } else {
                    Some(Limits::from_ffi(unsafe { *value.requiredLimits }))
                },
            default_queue: Some(QueueDescriptor::from_ffi(value.defaultQueue)),
            device_lost_callback_info: None,
            uncaptured_error_callback_info: None,
                }
    }
}

pub struct Extent2D {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Default for Extent2D {
    fn default() -> Self {
        Self {
        width: None,
        height: None,
        }
    }
}

impl Extent2D {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUExtent2D {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUExtent2D) -> Self {
        Self {
            width: Some(value.width),
            height: Some(value.height),
                }
    }
}

pub struct Extent3D {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub depth_or_array_layers: Option<u32>,
}

impl Default for Extent3D {
    fn default() -> Self {
        Self {
        width: None,
        height: Some(1),
        depth_or_array_layers: Some(1),
        }
    }
}

impl Extent3D {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(value);
        self
    }

    pub fn depth_or_array_layers(mut self, value: u32) -> Self {
        self.depth_or_array_layers = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUExtent3D {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUExtent3D) -> Self {
        Self {
            width: Some(value.width),
            height: Some(value.height),
            depth_or_array_layers: Some(value.depthOrArrayLayers),
                }
    }
}

pub struct ExternalTextureBindingEntry {
    pub external_texture: Option<ExternalTexture>,
}

impl Default for ExternalTextureBindingEntry {
    fn default() -> Self {
        Self {
        external_texture: None,
        }
    }
}

impl ExternalTextureBindingEntry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn external_texture(mut self, value: ExternalTexture) -> Self {
        self.external_texture = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUExternalTextureBindingEntry {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUExternalTextureBindingEntry) -> Self {
        Self {
            external_texture: Some(unsafe { ExternalTexture::from_raw(value.externalTexture) }),
                }
    }
}

pub struct ExternalTextureBindingLayout {

}

impl Default for ExternalTextureBindingLayout {
    fn default() -> Self {
        Self {

        }
    }
}

impl ExternalTextureBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_ffi(&self) -> ffi::WGPUExternalTextureBindingLayout {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUExternalTextureBindingLayout) -> Self {
        let _ = value;
                Self::default()
    }
}

pub struct ExternalTextureDescriptor {
    pub extensions: Vec<ExternalTextureDescriptorExtension>,
    pub label: Option<String>,
    pub plane_0: Option<TextureView>,
    pub plane_1: Option<TextureView>,
    pub crop_origin: Option<Origin2D>,
    pub crop_size: Option<Extent2D>,
    pub apparent_size: Option<Extent2D>,
    pub do_yuv_to_rgb_conversion_only: Option<bool>,
    pub yuv_to_rgb_conversion_matrix: Option<Vec<f32>>,
    pub src_transfer_function_parameters: Option<Vec<f32>>,
    pub dst_transfer_function_parameters: Option<Vec<f32>>,
    pub gamut_conversion_matrix: Option<Vec<f32>>,
    pub mirrored: Option<bool>,
    pub rotation: Option<ExternalTextureRotation>,
}

impl Default for ExternalTextureDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        plane_0: None,
        plane_1: None,
        crop_origin: None,
        crop_size: None,
        apparent_size: None,
        do_yuv_to_rgb_conversion_only: None,
        yuv_to_rgb_conversion_matrix: None,
        src_transfer_function_parameters: None,
        dst_transfer_function_parameters: None,
        gamut_conversion_matrix: None,
        mirrored: None,
        rotation: Some(ExternalTextureRotation::Rotate0Degrees),
        }
    }
}

impl ExternalTextureDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn plane_0(mut self, value: TextureView) -> Self {
        self.plane_0 = Some(value);
        self
    }

    pub fn plane_1(mut self, value: TextureView) -> Self {
        self.plane_1 = Some(value);
        self
    }

    pub fn crop_origin(mut self, value: Origin2D) -> Self {
        self.crop_origin = Some(value);
        self
    }

    pub fn crop_size(mut self, value: Extent2D) -> Self {
        self.crop_size = Some(value);
        self
    }

    pub fn apparent_size(mut self, value: Extent2D) -> Self {
        self.apparent_size = Some(value);
        self
    }

    pub fn do_yuv_to_rgb_conversion_only(mut self, value: bool) -> Self {
        self.do_yuv_to_rgb_conversion_only = Some(value);
        self
    }

    pub fn yuv_to_rgb_conversion_matrix(mut self, value: Vec<f32>) -> Self {
        self.yuv_to_rgb_conversion_matrix = Some(value);
        self
    }

    pub fn src_transfer_function_parameters(mut self, value: Vec<f32>) -> Self {
        self.src_transfer_function_parameters = Some(value);
        self
    }

    pub fn dst_transfer_function_parameters(mut self, value: Vec<f32>) -> Self {
        self.dst_transfer_function_parameters = Some(value);
        self
    }

    pub fn gamut_conversion_matrix(mut self, value: Vec<f32>) -> Self {
        self.gamut_conversion_matrix = Some(value);
        self
    }

    pub fn mirrored(mut self, value: bool) -> Self {
        self.mirrored = Some(value);
        self
    }

    pub fn rotation(mut self, value: ExternalTextureRotation) -> Self {
        self.rotation = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ExternalTextureDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUExternalTextureDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUExternalTextureDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUExternalTextureDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            plane_0: Some(unsafe { TextureView::from_raw(value.plane0) }),
            plane_1: if value.plane1.is_null() {
                    None
                } else {
                    Some(unsafe { TextureView::from_raw(value.plane1) })
                },
            crop_origin: Some(Origin2D::from_ffi(value.cropOrigin)),
            crop_size: Some(Extent2D::from_ffi(value.cropSize)),
            apparent_size: Some(Extent2D::from_ffi(value.apparentSize)),
            do_yuv_to_rgb_conversion_only: Some(value.doYuvToRgbConversionOnly != 0),
            yuv_to_rgb_conversion_matrix: if value.yuvToRgbConversionMatrix.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.yuvToRgbConversionMatrix, 12usize) }.to_vec())
                },
            src_transfer_function_parameters: if value.srcTransferFunctionParameters.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.srcTransferFunctionParameters, 7usize) }.to_vec())
                },
            dst_transfer_function_parameters: if value.dstTransferFunctionParameters.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.dstTransferFunctionParameters, 7usize) }.to_vec())
                },
            gamut_conversion_matrix: if value.gamutConversionMatrix.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.gamutConversionMatrix, 9usize) }.to_vec())
                },
            mirrored: Some(value.mirrored != 0),
            rotation: Some(value.rotation.into()),
                }
    }
}

pub struct FragmentState {
    pub extensions: Vec<FragmentStateExtension>,
    pub module: Option<ShaderModule>,
    pub entry_point: Option<String>,
    pub constant_count: Option<usize>,
    pub constants: Option<Vec<ConstantEntry>>,
    pub target_count: Option<usize>,
    pub targets: Option<Vec<ColorTargetState>>,
}

impl Default for FragmentState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        module: None,
        entry_point: None,
        constant_count: Some(0),
        constants: None,
        target_count: None,
        targets: None,
        }
    }
}

impl FragmentState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn module(mut self, value: ShaderModule) -> Self {
        self.module = Some(value);
        self
    }

    pub fn entry_point(mut self, value: String) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn constant_count(mut self, value: usize) -> Self {
        self.constant_count = Some(value);
        self
    }

    pub fn constants(mut self, value: Vec<ConstantEntry>) -> Self {
        self.constants = Some(value);
        self
    }

    pub fn target_count(mut self, value: usize) -> Self {
        self.target_count = Some(value);
        self
    }

    pub fn targets(mut self, value: Vec<ColorTargetState>) -> Self {
        self.targets = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: FragmentStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUFragmentState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUFragmentState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUFragmentState) -> Self {
        Self {
        extensions: Vec::new(),
            module: Some(unsafe { ShaderModule::from_raw(value.module) }),
            entry_point: if value.entryPoint.data.is_null() || value.entryPoint.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.entryPoint))
                },
            constant_count: Some(value.constantCount),
            constants: if value.constants.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.constants, value.constantCount as usize) }
                            .iter()
                            .map(|raw| ConstantEntry::from_ffi(*raw))
                            .collect(),
                    )
                },
            target_count: Some(value.targetCount),
            targets: if value.targets.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.targets, value.targetCount as usize) }
                            .iter()
                            .map(|raw| ColorTargetState::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct Future {
    pub id: Option<u64>,
}

impl Default for Future {
    fn default() -> Self {
        Self {
        id: None,
        }
    }
}

impl Future {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn id(mut self, value: u64) -> Self {
        self.id = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUFuture {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUFuture) -> Self {
        Self {
            id: Some(value.id),
                }
    }
}

pub struct FutureWaitInfo {
    pub future: Option<Future>,
    pub completed: Option<bool>,
}

impl Default for FutureWaitInfo {
    fn default() -> Self {
        Self {
        future: None,
        completed: None,
        }
    }
}

impl FutureWaitInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn future(mut self, value: Future) -> Self {
        self.future = Some(value);
        self
    }

    pub fn completed(mut self, value: bool) -> Self {
        self.completed = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUFutureWaitInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUFutureWaitInfo) -> Self {
        Self {
            future: Some(Future::from_ffi(value.future)),
            completed: Some(value.completed != 0),
                }
    }
}

pub struct ImageCopyExternalTexture {
    pub extensions: Vec<ImageCopyExternalTextureExtension>,
    pub external_texture: Option<ExternalTexture>,
    pub origin: Option<Origin3D>,
    pub natural_size: Option<Extent2D>,
}

impl Default for ImageCopyExternalTexture {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        external_texture: None,
        origin: None,
        natural_size: None,
        }
    }
}

impl ImageCopyExternalTexture {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn external_texture(mut self, value: ExternalTexture) -> Self {
        self.external_texture = Some(value);
        self
    }

    pub fn origin(mut self, value: Origin3D) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn natural_size(mut self, value: Extent2D) -> Self {
        self.natural_size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ImageCopyExternalTextureExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUImageCopyExternalTexture, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUImageCopyExternalTexture = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUImageCopyExternalTexture) -> Self {
        Self {
        extensions: Vec::new(),
            external_texture: Some(unsafe { ExternalTexture::from_raw(value.externalTexture) }),
            origin: Some(Origin3D::from_ffi(value.origin)),
            natural_size: Some(Extent2D::from_ffi(value.naturalSize)),
                }
    }
}

pub struct InstanceDescriptor {
    pub extensions: Vec<InstanceDescriptorExtension>,
    pub required_feature_count: Option<usize>,
    pub required_features: Option<Vec<InstanceFeatureName>>,
    pub required_limits: Option<InstanceLimits>,
}

impl Default for InstanceDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        required_feature_count: Some(0),
        required_features: None,
        required_limits: None,
        }
    }
}

impl InstanceDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn required_feature_count(mut self, value: usize) -> Self {
        self.required_feature_count = Some(value);
        self
    }

    pub fn required_features(mut self, value: Vec<InstanceFeatureName>) -> Self {
        self.required_features = Some(value);
        self
    }

    pub fn required_limits(mut self, value: InstanceLimits) -> Self {
        self.required_limits = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: InstanceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUInstanceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUInstanceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUInstanceDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            required_feature_count: Some(value.requiredFeatureCount),
            required_features: if value.requiredFeatures.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.requiredFeatures, value.requiredFeatureCount as usize) }
                            .iter()
                            .map(|raw| InstanceFeatureName::from(*raw))
                            .collect(),
                    )
                },
            required_limits: if value.requiredLimits.is_null() {
                    None
                } else {
                    Some(InstanceLimits::from_ffi(unsafe { *value.requiredLimits }))
                },
                }
    }
}

pub struct InstanceLimits {
    pub extensions: Vec<InstanceLimitsExtension>,
    pub timed_wait_any_max_count: Option<usize>,
}

impl Default for InstanceLimits {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        timed_wait_any_max_count: Some(0),
        }
    }
}

impl InstanceLimits {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn timed_wait_any_max_count(mut self, value: usize) -> Self {
        self.timed_wait_any_max_count = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: InstanceLimitsExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUInstanceLimits, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUInstanceLimits = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUInstanceLimits) -> Self {
        Self {
        extensions: Vec::new(),
            timed_wait_any_max_count: Some(value.timedWaitAnyMaxCount),
                }
    }
}

pub struct Limits {
    pub extensions: Vec<LimitsExtension>,
    pub max_texture_dimension_1d: Option<u32>,
    pub max_texture_dimension_2d: Option<u32>,
    pub max_texture_dimension_3d: Option<u32>,
    pub max_texture_array_layers: Option<u32>,
    pub max_bind_groups: Option<u32>,
    pub max_bind_groups_plus_vertex_buffers: Option<u32>,
    pub max_bindings_per_bind_group: Option<u32>,
    pub max_dynamic_uniform_buffers_per_pipeline_layout: Option<u32>,
    pub max_dynamic_storage_buffers_per_pipeline_layout: Option<u32>,
    pub max_sampled_textures_per_shader_stage: Option<u32>,
    pub max_samplers_per_shader_stage: Option<u32>,
    pub max_storage_buffers_per_shader_stage: Option<u32>,
    pub max_storage_textures_per_shader_stage: Option<u32>,
    pub max_uniform_buffers_per_shader_stage: Option<u32>,
    pub max_uniform_buffer_binding_size: Option<u64>,
    pub max_storage_buffer_binding_size: Option<u64>,
    pub min_uniform_buffer_offset_alignment: Option<u32>,
    pub min_storage_buffer_offset_alignment: Option<u32>,
    pub max_vertex_buffers: Option<u32>,
    pub max_buffer_size: Option<u64>,
    pub max_vertex_attributes: Option<u32>,
    pub max_vertex_buffer_array_stride: Option<u32>,
    pub max_inter_stage_shader_variables: Option<u32>,
    pub max_color_attachments: Option<u32>,
    pub max_color_attachment_bytes_per_sample: Option<u32>,
    pub max_compute_workgroup_storage_size: Option<u32>,
    pub max_compute_invocations_per_workgroup: Option<u32>,
    pub max_compute_workgroup_size_x: Option<u32>,
    pub max_compute_workgroup_size_y: Option<u32>,
    pub max_compute_workgroup_size_z: Option<u32>,
    pub max_compute_workgroups_per_dimension: Option<u32>,
    pub max_immediate_size: Option<u32>,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        max_texture_dimension_1d: None,
        max_texture_dimension_2d: None,
        max_texture_dimension_3d: None,
        max_texture_array_layers: None,
        max_bind_groups: None,
        max_bind_groups_plus_vertex_buffers: None,
        max_bindings_per_bind_group: None,
        max_dynamic_uniform_buffers_per_pipeline_layout: None,
        max_dynamic_storage_buffers_per_pipeline_layout: None,
        max_sampled_textures_per_shader_stage: None,
        max_samplers_per_shader_stage: None,
        max_storage_buffers_per_shader_stage: None,
        max_storage_textures_per_shader_stage: None,
        max_uniform_buffers_per_shader_stage: None,
        max_uniform_buffer_binding_size: None,
        max_storage_buffer_binding_size: None,
        min_uniform_buffer_offset_alignment: None,
        min_storage_buffer_offset_alignment: None,
        max_vertex_buffers: None,
        max_buffer_size: None,
        max_vertex_attributes: None,
        max_vertex_buffer_array_stride: None,
        max_inter_stage_shader_variables: None,
        max_color_attachments: None,
        max_color_attachment_bytes_per_sample: None,
        max_compute_workgroup_storage_size: None,
        max_compute_invocations_per_workgroup: None,
        max_compute_workgroup_size_x: None,
        max_compute_workgroup_size_y: None,
        max_compute_workgroup_size_z: None,
        max_compute_workgroups_per_dimension: None,
        max_immediate_size: None,
        }
    }
}

impl Limits {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn max_texture_dimension_1d(mut self, value: u32) -> Self {
        self.max_texture_dimension_1d = Some(value);
        self
    }

    pub fn max_texture_dimension_2d(mut self, value: u32) -> Self {
        self.max_texture_dimension_2d = Some(value);
        self
    }

    pub fn max_texture_dimension_3d(mut self, value: u32) -> Self {
        self.max_texture_dimension_3d = Some(value);
        self
    }

    pub fn max_texture_array_layers(mut self, value: u32) -> Self {
        self.max_texture_array_layers = Some(value);
        self
    }

    pub fn max_bind_groups(mut self, value: u32) -> Self {
        self.max_bind_groups = Some(value);
        self
    }

    pub fn max_bind_groups_plus_vertex_buffers(mut self, value: u32) -> Self {
        self.max_bind_groups_plus_vertex_buffers = Some(value);
        self
    }

    pub fn max_bindings_per_bind_group(mut self, value: u32) -> Self {
        self.max_bindings_per_bind_group = Some(value);
        self
    }

    pub fn max_dynamic_uniform_buffers_per_pipeline_layout(mut self, value: u32) -> Self {
        self.max_dynamic_uniform_buffers_per_pipeline_layout = Some(value);
        self
    }

    pub fn max_dynamic_storage_buffers_per_pipeline_layout(mut self, value: u32) -> Self {
        self.max_dynamic_storage_buffers_per_pipeline_layout = Some(value);
        self
    }

    pub fn max_sampled_textures_per_shader_stage(mut self, value: u32) -> Self {
        self.max_sampled_textures_per_shader_stage = Some(value);
        self
    }

    pub fn max_samplers_per_shader_stage(mut self, value: u32) -> Self {
        self.max_samplers_per_shader_stage = Some(value);
        self
    }

    pub fn max_storage_buffers_per_shader_stage(mut self, value: u32) -> Self {
        self.max_storage_buffers_per_shader_stage = Some(value);
        self
    }

    pub fn max_storage_textures_per_shader_stage(mut self, value: u32) -> Self {
        self.max_storage_textures_per_shader_stage = Some(value);
        self
    }

    pub fn max_uniform_buffers_per_shader_stage(mut self, value: u32) -> Self {
        self.max_uniform_buffers_per_shader_stage = Some(value);
        self
    }

    pub fn max_uniform_buffer_binding_size(mut self, value: u64) -> Self {
        self.max_uniform_buffer_binding_size = Some(value);
        self
    }

    pub fn max_storage_buffer_binding_size(mut self, value: u64) -> Self {
        self.max_storage_buffer_binding_size = Some(value);
        self
    }

    pub fn min_uniform_buffer_offset_alignment(mut self, value: u32) -> Self {
        self.min_uniform_buffer_offset_alignment = Some(value);
        self
    }

    pub fn min_storage_buffer_offset_alignment(mut self, value: u32) -> Self {
        self.min_storage_buffer_offset_alignment = Some(value);
        self
    }

    pub fn max_vertex_buffers(mut self, value: u32) -> Self {
        self.max_vertex_buffers = Some(value);
        self
    }

    pub fn max_buffer_size(mut self, value: u64) -> Self {
        self.max_buffer_size = Some(value);
        self
    }

    pub fn max_vertex_attributes(mut self, value: u32) -> Self {
        self.max_vertex_attributes = Some(value);
        self
    }

    pub fn max_vertex_buffer_array_stride(mut self, value: u32) -> Self {
        self.max_vertex_buffer_array_stride = Some(value);
        self
    }

    pub fn max_inter_stage_shader_variables(mut self, value: u32) -> Self {
        self.max_inter_stage_shader_variables = Some(value);
        self
    }

    pub fn max_color_attachments(mut self, value: u32) -> Self {
        self.max_color_attachments = Some(value);
        self
    }

    pub fn max_color_attachment_bytes_per_sample(mut self, value: u32) -> Self {
        self.max_color_attachment_bytes_per_sample = Some(value);
        self
    }

    pub fn max_compute_workgroup_storage_size(mut self, value: u32) -> Self {
        self.max_compute_workgroup_storage_size = Some(value);
        self
    }

    pub fn max_compute_invocations_per_workgroup(mut self, value: u32) -> Self {
        self.max_compute_invocations_per_workgroup = Some(value);
        self
    }

    pub fn max_compute_workgroup_size_x(mut self, value: u32) -> Self {
        self.max_compute_workgroup_size_x = Some(value);
        self
    }

    pub fn max_compute_workgroup_size_y(mut self, value: u32) -> Self {
        self.max_compute_workgroup_size_y = Some(value);
        self
    }

    pub fn max_compute_workgroup_size_z(mut self, value: u32) -> Self {
        self.max_compute_workgroup_size_z = Some(value);
        self
    }

    pub fn max_compute_workgroups_per_dimension(mut self, value: u32) -> Self {
        self.max_compute_workgroups_per_dimension = Some(value);
        self
    }

    pub fn max_immediate_size(mut self, value: u32) -> Self {
        self.max_immediate_size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: LimitsExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPULimits, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPULimits = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPULimits) -> Self {
        Self {
        extensions: Vec::new(),
            max_texture_dimension_1d: Some(value.maxTextureDimension1D),
            max_texture_dimension_2d: Some(value.maxTextureDimension2D),
            max_texture_dimension_3d: Some(value.maxTextureDimension3D),
            max_texture_array_layers: Some(value.maxTextureArrayLayers),
            max_bind_groups: Some(value.maxBindGroups),
            max_bind_groups_plus_vertex_buffers: Some(value.maxBindGroupsPlusVertexBuffers),
            max_bindings_per_bind_group: Some(value.maxBindingsPerBindGroup),
            max_dynamic_uniform_buffers_per_pipeline_layout: Some(value.maxDynamicUniformBuffersPerPipelineLayout),
            max_dynamic_storage_buffers_per_pipeline_layout: Some(value.maxDynamicStorageBuffersPerPipelineLayout),
            max_sampled_textures_per_shader_stage: Some(value.maxSampledTexturesPerShaderStage),
            max_samplers_per_shader_stage: Some(value.maxSamplersPerShaderStage),
            max_storage_buffers_per_shader_stage: Some(value.maxStorageBuffersPerShaderStage),
            max_storage_textures_per_shader_stage: Some(value.maxStorageTexturesPerShaderStage),
            max_uniform_buffers_per_shader_stage: Some(value.maxUniformBuffersPerShaderStage),
            max_uniform_buffer_binding_size: Some(value.maxUniformBufferBindingSize),
            max_storage_buffer_binding_size: Some(value.maxStorageBufferBindingSize),
            min_uniform_buffer_offset_alignment: Some(value.minUniformBufferOffsetAlignment),
            min_storage_buffer_offset_alignment: Some(value.minStorageBufferOffsetAlignment),
            max_vertex_buffers: Some(value.maxVertexBuffers),
            max_buffer_size: Some(value.maxBufferSize),
            max_vertex_attributes: Some(value.maxVertexAttributes),
            max_vertex_buffer_array_stride: Some(value.maxVertexBufferArrayStride),
            max_inter_stage_shader_variables: Some(value.maxInterStageShaderVariables),
            max_color_attachments: Some(value.maxColorAttachments),
            max_color_attachment_bytes_per_sample: Some(value.maxColorAttachmentBytesPerSample),
            max_compute_workgroup_storage_size: Some(value.maxComputeWorkgroupStorageSize),
            max_compute_invocations_per_workgroup: Some(value.maxComputeInvocationsPerWorkgroup),
            max_compute_workgroup_size_x: Some(value.maxComputeWorkgroupSizeX),
            max_compute_workgroup_size_y: Some(value.maxComputeWorkgroupSizeY),
            max_compute_workgroup_size_z: Some(value.maxComputeWorkgroupSizeZ),
            max_compute_workgroups_per_dimension: Some(value.maxComputeWorkgroupsPerDimension),
            max_immediate_size: Some(value.maxImmediateSize),
                }
    }
}

pub struct MemoryHeapInfo {
    pub properties: Option<HeapProperty>,
    pub size: Option<u64>,
}

impl Default for MemoryHeapInfo {
    fn default() -> Self {
        Self {
        properties: None,
        size: None,
        }
    }
}

impl MemoryHeapInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn properties(mut self, value: HeapProperty) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUMemoryHeapInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUMemoryHeapInfo) -> Self {
        Self {
            properties: Some(value.properties.into()),
            size: Some(value.size),
                }
    }
}

pub struct MultisampleState {
    pub extensions: Vec<MultisampleStateExtension>,
    pub count: Option<u32>,
    pub mask: Option<u32>,
    pub alpha_to_coverage_enabled: Option<bool>,
}

impl Default for MultisampleState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        count: Some(1),
        mask: Some(4294967295),
        alpha_to_coverage_enabled: None,
        }
    }
}

impl MultisampleState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn count(mut self, value: u32) -> Self {
        self.count = Some(value);
        self
    }

    pub fn mask(mut self, value: u32) -> Self {
        self.mask = Some(value);
        self
    }

    pub fn alpha_to_coverage_enabled(mut self, value: bool) -> Self {
        self.alpha_to_coverage_enabled = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: MultisampleStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUMultisampleState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUMultisampleState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUMultisampleState) -> Self {
        Self {
        extensions: Vec::new(),
            count: Some(value.count),
            mask: Some(value.mask),
            alpha_to_coverage_enabled: Some(value.alphaToCoverageEnabled != 0),
                }
    }
}

pub struct Origin2D {
    pub x: Option<u32>,
    pub y: Option<u32>,
}

impl Default for Origin2D {
    fn default() -> Self {
        Self {
        x: Some(0),
        y: Some(0),
        }
    }
}

impl Origin2D {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn x(mut self, value: u32) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: u32) -> Self {
        self.y = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUOrigin2D {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUOrigin2D) -> Self {
        Self {
            x: Some(value.x),
            y: Some(value.y),
                }
    }
}

pub struct Origin3D {
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub z: Option<u32>,
}

impl Default for Origin3D {
    fn default() -> Self {
        Self {
        x: Some(0),
        y: Some(0),
        z: Some(0),
        }
    }
}

impl Origin3D {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn x(mut self, value: u32) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: u32) -> Self {
        self.y = Some(value);
        self
    }

    pub fn z(mut self, value: u32) -> Self {
        self.z = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUOrigin3D {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUOrigin3D) -> Self {
        Self {
            x: Some(value.x),
            y: Some(value.y),
            z: Some(value.z),
                }
    }
}

pub struct PassTimestampWrites {
    pub extensions: Vec<PassTimestampWritesExtension>,
    pub query_set: Option<QuerySet>,
    pub beginning_of_pass_write_index: Option<u32>,
    pub end_of_pass_write_index: Option<u32>,
}

impl Default for PassTimestampWrites {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        query_set: None,
        beginning_of_pass_write_index: None,
        end_of_pass_write_index: None,
        }
    }
}

impl PassTimestampWrites {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn query_set(mut self, value: QuerySet) -> Self {
        self.query_set = Some(value);
        self
    }

    pub fn beginning_of_pass_write_index(mut self, value: u32) -> Self {
        self.beginning_of_pass_write_index = Some(value);
        self
    }

    pub fn end_of_pass_write_index(mut self, value: u32) -> Self {
        self.end_of_pass_write_index = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: PassTimestampWritesExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUPassTimestampWrites, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPassTimestampWrites = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUPassTimestampWrites) -> Self {
        Self {
        extensions: Vec::new(),
            query_set: Some(unsafe { QuerySet::from_raw(value.querySet) }),
            beginning_of_pass_write_index: Some(value.beginningOfPassWriteIndex),
            end_of_pass_write_index: Some(value.endOfPassWriteIndex),
                }
    }
}

pub struct PipelineLayoutDescriptor {
    pub extensions: Vec<PipelineLayoutDescriptorExtension>,
    pub label: Option<String>,
    pub bind_group_layout_count: Option<usize>,
    pub bind_group_layouts: Option<Vec<BindGroupLayout>>,
    pub immediate_size: Option<u32>,
}

impl Default for PipelineLayoutDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        bind_group_layout_count: None,
        bind_group_layouts: None,
        immediate_size: Some(0),
        }
    }
}

impl PipelineLayoutDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn bind_group_layout_count(mut self, value: usize) -> Self {
        self.bind_group_layout_count = Some(value);
        self
    }

    pub fn bind_group_layouts(mut self, value: Vec<BindGroupLayout>) -> Self {
        self.bind_group_layouts = Some(value);
        self
    }

    pub fn immediate_size(mut self, value: u32) -> Self {
        self.immediate_size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: PipelineLayoutDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUPipelineLayoutDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPipelineLayoutDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUPipelineLayoutDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            bind_group_layout_count: Some(value.bindGroupLayoutCount),
            bind_group_layouts: if value.bindGroupLayouts.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.bindGroupLayouts, value.bindGroupLayoutCount as usize) }
                            .iter()
                            .map(|raw| unsafe { BindGroupLayout::from_raw(*raw) })
                            .collect(),
                    )
                },
            immediate_size: Some(value.immediateSize),
                }
    }
}

pub struct PipelineLayoutPixelLocalStorage {
    pub total_pixel_local_storage_size: Option<u64>,
    pub storage_attachment_count: Option<usize>,
    pub storage_attachments: Option<Vec<PipelineLayoutStorageAttachment>>,
}

impl Default for PipelineLayoutPixelLocalStorage {
    fn default() -> Self {
        Self {
        total_pixel_local_storage_size: None,
        storage_attachment_count: Some(0),
        storage_attachments: None,
        }
    }
}

impl PipelineLayoutPixelLocalStorage {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn total_pixel_local_storage_size(mut self, value: u64) -> Self {
        self.total_pixel_local_storage_size = Some(value);
        self
    }

    pub fn storage_attachment_count(mut self, value: usize) -> Self {
        self.storage_attachment_count = Some(value);
        self
    }

    pub fn storage_attachments(mut self, value: Vec<PipelineLayoutStorageAttachment>) -> Self {
        self.storage_attachments = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUPipelineLayoutPixelLocalStorage {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUPipelineLayoutPixelLocalStorage) -> Self {
        Self {
            total_pixel_local_storage_size: Some(value.totalPixelLocalStorageSize),
            storage_attachment_count: Some(value.storageAttachmentCount),
            storage_attachments: if value.storageAttachments.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.storageAttachments, value.storageAttachmentCount as usize) }
                            .iter()
                            .map(|raw| PipelineLayoutStorageAttachment::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct PipelineLayoutResourceTable {
    pub uses_resource_table: Option<bool>,
}

impl Default for PipelineLayoutResourceTable {
    fn default() -> Self {
        Self {
        uses_resource_table: None,
        }
    }
}

impl PipelineLayoutResourceTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn uses_resource_table(mut self, value: bool) -> Self {
        self.uses_resource_table = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUPipelineLayoutResourceTable {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUPipelineLayoutResourceTable) -> Self {
        Self {
            uses_resource_table: Some(value.usesResourceTable != 0),
                }
    }
}

pub struct PipelineLayoutStorageAttachment {
    pub extensions: Vec<PipelineLayoutStorageAttachmentExtension>,
    pub offset: Option<u64>,
    pub format: Option<TextureFormat>,
}

impl Default for PipelineLayoutStorageAttachment {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        offset: Some(0),
        format: None,
        }
    }
}

impl PipelineLayoutStorageAttachment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: PipelineLayoutStorageAttachmentExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUPipelineLayoutStorageAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPipelineLayoutStorageAttachment = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUPipelineLayoutStorageAttachment) -> Self {
        Self {
        extensions: Vec::new(),
            offset: Some(value.offset),
            format: Some(value.format.into()),
                }
    }
}

pub struct PrimitiveState {
    pub extensions: Vec<PrimitiveStateExtension>,
    pub topology: Option<PrimitiveTopology>,
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: Option<FrontFace>,
    pub cull_mode: Option<CullMode>,
    pub unclipped_depth: Option<bool>,
}

impl Default for PrimitiveState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        topology: Some(PrimitiveTopology::TriangleList),
        strip_index_format: None,
        front_face: Some(FrontFace::Ccw),
        cull_mode: Some(CullMode::None),
        unclipped_depth: None,
        }
    }
}

impl PrimitiveState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn topology(mut self, value: PrimitiveTopology) -> Self {
        self.topology = Some(value);
        self
    }

    pub fn strip_index_format(mut self, value: IndexFormat) -> Self {
        self.strip_index_format = Some(value);
        self
    }

    pub fn front_face(mut self, value: FrontFace) -> Self {
        self.front_face = Some(value);
        self
    }

    pub fn cull_mode(mut self, value: CullMode) -> Self {
        self.cull_mode = Some(value);
        self
    }

    pub fn unclipped_depth(mut self, value: bool) -> Self {
        self.unclipped_depth = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: PrimitiveStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUPrimitiveState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPrimitiveState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUPrimitiveState) -> Self {
        Self {
        extensions: Vec::new(),
            topology: Some(value.topology.into()),
            strip_index_format: Some(value.stripIndexFormat.into()),
            front_face: Some(value.frontFace.into()),
            cull_mode: Some(value.cullMode.into()),
            unclipped_depth: Some(value.unclippedDepth != 0),
                }
    }
}

pub struct QuerySetDescriptor {
    pub extensions: Vec<QuerySetDescriptorExtension>,
    pub label: Option<String>,
    pub r#type: Option<QueryType>,
    pub count: Option<u32>,
}

impl Default for QuerySetDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        r#type: None,
        count: None,
        }
    }
}

impl QuerySetDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn count(mut self, value: u32) -> Self {
        self.count = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: QuerySetDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUQuerySetDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUQuerySetDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUQuerySetDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            r#type: Some(value.type_.into()),
            count: Some(value.count),
                }
    }
}

pub struct QueueDescriptor {
    pub extensions: Vec<QueueDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for QueueDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl QueueDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: QueueDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUQueueDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUQueueDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUQueueDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct RenderBundleDescriptor {
    pub extensions: Vec<RenderBundleDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for RenderBundleDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl RenderBundleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RenderBundleDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURenderBundleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderBundleDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderBundleDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct RenderBundleEncoderDescriptor {
    pub extensions: Vec<RenderBundleEncoderDescriptorExtension>,
    pub label: Option<String>,
    pub color_format_count: Option<usize>,
    pub color_formats: Option<Vec<TextureFormat>>,
    pub depth_stencil_format: Option<TextureFormat>,
    pub sample_count: Option<u32>,
    pub depth_read_only: Option<bool>,
    pub stencil_read_only: Option<bool>,
}

impl Default for RenderBundleEncoderDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        color_format_count: None,
        color_formats: None,
        depth_stencil_format: None,
        sample_count: Some(1),
        depth_read_only: None,
        stencil_read_only: None,
        }
    }
}

impl RenderBundleEncoderDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn color_format_count(mut self, value: usize) -> Self {
        self.color_format_count = Some(value);
        self
    }

    pub fn color_formats(mut self, value: Vec<TextureFormat>) -> Self {
        self.color_formats = Some(value);
        self
    }

    pub fn depth_stencil_format(mut self, value: TextureFormat) -> Self {
        self.depth_stencil_format = Some(value);
        self
    }

    pub fn sample_count(mut self, value: u32) -> Self {
        self.sample_count = Some(value);
        self
    }

    pub fn depth_read_only(mut self, value: bool) -> Self {
        self.depth_read_only = Some(value);
        self
    }

    pub fn stencil_read_only(mut self, value: bool) -> Self {
        self.stencil_read_only = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RenderBundleEncoderDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURenderBundleEncoderDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderBundleEncoderDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderBundleEncoderDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            color_format_count: Some(value.colorFormatCount),
            color_formats: if value.colorFormats.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.colorFormats, value.colorFormatCount as usize) }
                            .iter()
                            .map(|raw| TextureFormat::from(*raw))
                            .collect(),
                    )
                },
            depth_stencil_format: Some(value.depthStencilFormat.into()),
            sample_count: Some(value.sampleCount),
            depth_read_only: Some(value.depthReadOnly != 0),
            stencil_read_only: Some(value.stencilReadOnly != 0),
                }
    }
}

pub struct RenderPassColorAttachment {
    pub extensions: Vec<RenderPassColorAttachmentExtension>,
    pub view: Option<TextureView>,
    pub depth_slice: Option<u32>,
    pub resolve_target: Option<TextureView>,
    pub load_op: Option<LoadOp>,
    pub store_op: Option<StoreOp>,
    pub clear_value: Option<Color>,
}

impl Default for RenderPassColorAttachment {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        view: None,
        depth_slice: None,
        resolve_target: None,
        load_op: None,
        store_op: None,
        clear_value: None,
        }
    }
}

impl RenderPassColorAttachment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn view(mut self, value: TextureView) -> Self {
        self.view = Some(value);
        self
    }

    pub fn depth_slice(mut self, value: u32) -> Self {
        self.depth_slice = Some(value);
        self
    }

    pub fn resolve_target(mut self, value: TextureView) -> Self {
        self.resolve_target = Some(value);
        self
    }

    pub fn load_op(mut self, value: LoadOp) -> Self {
        self.load_op = Some(value);
        self
    }

    pub fn store_op(mut self, value: StoreOp) -> Self {
        self.store_op = Some(value);
        self
    }

    pub fn clear_value(mut self, value: Color) -> Self {
        self.clear_value = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RenderPassColorAttachmentExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURenderPassColorAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassColorAttachment = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassColorAttachment) -> Self {
        Self {
        extensions: Vec::new(),
            view: if value.view.is_null() {
                    None
                } else {
                    Some(unsafe { TextureView::from_raw(value.view) })
                },
            depth_slice: Some(value.depthSlice),
            resolve_target: if value.resolveTarget.is_null() {
                    None
                } else {
                    Some(unsafe { TextureView::from_raw(value.resolveTarget) })
                },
            load_op: Some(value.loadOp.into()),
            store_op: Some(value.storeOp.into()),
            clear_value: Some(Color::from_ffi(value.clearValue)),
                }
    }
}

pub struct RenderPassDepthStencilAttachment {
    pub extensions: Vec<RenderPassDepthStencilAttachmentExtension>,
    pub view: Option<TextureView>,
    pub depth_load_op: Option<LoadOp>,
    pub depth_store_op: Option<StoreOp>,
    pub depth_clear_value: Option<f32>,
    pub depth_read_only: Option<bool>,
    pub stencil_load_op: Option<LoadOp>,
    pub stencil_store_op: Option<StoreOp>,
    pub stencil_clear_value: Option<u32>,
    pub stencil_read_only: Option<bool>,
}

impl Default for RenderPassDepthStencilAttachment {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        view: None,
        depth_load_op: None,
        depth_store_op: None,
        depth_clear_value: None,
        depth_read_only: None,
        stencil_load_op: None,
        stencil_store_op: None,
        stencil_clear_value: Some(0),
        stencil_read_only: None,
        }
    }
}

impl RenderPassDepthStencilAttachment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn view(mut self, value: TextureView) -> Self {
        self.view = Some(value);
        self
    }

    pub fn depth_load_op(mut self, value: LoadOp) -> Self {
        self.depth_load_op = Some(value);
        self
    }

    pub fn depth_store_op(mut self, value: StoreOp) -> Self {
        self.depth_store_op = Some(value);
        self
    }

    pub fn depth_clear_value(mut self, value: f32) -> Self {
        self.depth_clear_value = Some(value);
        self
    }

    pub fn depth_read_only(mut self, value: bool) -> Self {
        self.depth_read_only = Some(value);
        self
    }

    pub fn stencil_load_op(mut self, value: LoadOp) -> Self {
        self.stencil_load_op = Some(value);
        self
    }

    pub fn stencil_store_op(mut self, value: StoreOp) -> Self {
        self.stencil_store_op = Some(value);
        self
    }

    pub fn stencil_clear_value(mut self, value: u32) -> Self {
        self.stencil_clear_value = Some(value);
        self
    }

    pub fn stencil_read_only(mut self, value: bool) -> Self {
        self.stencil_read_only = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RenderPassDepthStencilAttachmentExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURenderPassDepthStencilAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassDepthStencilAttachment = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassDepthStencilAttachment) -> Self {
        Self {
        extensions: Vec::new(),
            view: Some(unsafe { TextureView::from_raw(value.view) }),
            depth_load_op: Some(value.depthLoadOp.into()),
            depth_store_op: Some(value.depthStoreOp.into()),
            depth_clear_value: Some(value.depthClearValue),
            depth_read_only: Some(value.depthReadOnly != 0),
            stencil_load_op: Some(value.stencilLoadOp.into()),
            stencil_store_op: Some(value.stencilStoreOp.into()),
            stencil_clear_value: Some(value.stencilClearValue),
            stencil_read_only: Some(value.stencilReadOnly != 0),
                }
    }
}

pub struct RenderPassDescriptor {
    pub extensions: Vec<RenderPassDescriptorExtension>,
    pub label: Option<String>,
    pub color_attachment_count: Option<usize>,
    pub color_attachments: Option<Vec<RenderPassColorAttachment>>,
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment>,
    pub occlusion_query_set: Option<QuerySet>,
    pub timestamp_writes: Option<PassTimestampWrites>,
}

impl Default for RenderPassDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        color_attachment_count: None,
        color_attachments: None,
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
        }
    }
}

impl RenderPassDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn color_attachment_count(mut self, value: usize) -> Self {
        self.color_attachment_count = Some(value);
        self
    }

    pub fn color_attachments(mut self, value: Vec<RenderPassColorAttachment>) -> Self {
        self.color_attachments = Some(value);
        self
    }

    pub fn depth_stencil_attachment(mut self, value: RenderPassDepthStencilAttachment) -> Self {
        self.depth_stencil_attachment = Some(value);
        self
    }

    pub fn occlusion_query_set(mut self, value: QuerySet) -> Self {
        self.occlusion_query_set = Some(value);
        self
    }

    pub fn timestamp_writes(mut self, value: PassTimestampWrites) -> Self {
        self.timestamp_writes = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RenderPassDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURenderPassDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            color_attachment_count: Some(value.colorAttachmentCount),
            color_attachments: if value.colorAttachments.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.colorAttachments, value.colorAttachmentCount as usize) }
                            .iter()
                            .map(|raw| RenderPassColorAttachment::from_ffi(*raw))
                            .collect(),
                    )
                },
            depth_stencil_attachment: if value.depthStencilAttachment.is_null() {
                    None
                } else {
                    Some(RenderPassDepthStencilAttachment::from_ffi(unsafe { *value.depthStencilAttachment }))
                },
            occlusion_query_set: if value.occlusionQuerySet.is_null() {
                    None
                } else {
                    Some(unsafe { QuerySet::from_raw(value.occlusionQuerySet) })
                },
            timestamp_writes: if value.timestampWrites.is_null() {
                    None
                } else {
                    Some(PassTimestampWrites::from_ffi(unsafe { *value.timestampWrites }))
                },
                }
    }
}

pub struct RenderPassDescriptorExpandResolveRect {
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Default for RenderPassDescriptorExpandResolveRect {
    fn default() -> Self {
        Self {
        x: None,
        y: None,
        width: None,
        height: None,
        }
    }
}

impl RenderPassDescriptorExpandResolveRect {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn x(mut self, value: u32) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: u32) -> Self {
        self.y = Some(value);
        self
    }

    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPURenderPassDescriptorExpandResolveRect {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassDescriptorExpandResolveRect) -> Self {
        Self {
            x: Some(value.x),
            y: Some(value.y),
            width: Some(value.width),
            height: Some(value.height),
                }
    }
}

pub struct RenderPassDescriptorResolveRect {
    pub color_offset_x: Option<u32>,
    pub color_offset_y: Option<u32>,
    pub resolve_offset_x: Option<u32>,
    pub resolve_offset_y: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Default for RenderPassDescriptorResolveRect {
    fn default() -> Self {
        Self {
        color_offset_x: None,
        color_offset_y: None,
        resolve_offset_x: None,
        resolve_offset_y: None,
        width: None,
        height: None,
        }
    }
}

impl RenderPassDescriptorResolveRect {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn color_offset_x(mut self, value: u32) -> Self {
        self.color_offset_x = Some(value);
        self
    }

    pub fn color_offset_y(mut self, value: u32) -> Self {
        self.color_offset_y = Some(value);
        self
    }

    pub fn resolve_offset_x(mut self, value: u32) -> Self {
        self.resolve_offset_x = Some(value);
        self
    }

    pub fn resolve_offset_y(mut self, value: u32) -> Self {
        self.resolve_offset_y = Some(value);
        self
    }

    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPURenderPassDescriptorResolveRect {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassDescriptorResolveRect) -> Self {
        Self {
            color_offset_x: Some(value.colorOffsetX),
            color_offset_y: Some(value.colorOffsetY),
            resolve_offset_x: Some(value.resolveOffsetX),
            resolve_offset_y: Some(value.resolveOffsetY),
            width: Some(value.width),
            height: Some(value.height),
                }
    }
}

pub struct RenderPassMaxDrawCount {
    pub max_draw_count: Option<u64>,
}

impl Default for RenderPassMaxDrawCount {
    fn default() -> Self {
        Self {
        max_draw_count: Some(50000000),
        }
    }
}

impl RenderPassMaxDrawCount {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn max_draw_count(mut self, value: u64) -> Self {
        self.max_draw_count = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPURenderPassMaxDrawCount {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassMaxDrawCount) -> Self {
        Self {
            max_draw_count: Some(value.maxDrawCount),
                }
    }
}

pub struct RenderPassPixelLocalStorage {
    pub total_pixel_local_storage_size: Option<u64>,
    pub storage_attachment_count: Option<usize>,
    pub storage_attachments: Option<Vec<RenderPassStorageAttachment>>,
}

impl Default for RenderPassPixelLocalStorage {
    fn default() -> Self {
        Self {
        total_pixel_local_storage_size: None,
        storage_attachment_count: Some(0),
        storage_attachments: None,
        }
    }
}

impl RenderPassPixelLocalStorage {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn total_pixel_local_storage_size(mut self, value: u64) -> Self {
        self.total_pixel_local_storage_size = Some(value);
        self
    }

    pub fn storage_attachment_count(mut self, value: usize) -> Self {
        self.storage_attachment_count = Some(value);
        self
    }

    pub fn storage_attachments(mut self, value: Vec<RenderPassStorageAttachment>) -> Self {
        self.storage_attachments = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPURenderPassPixelLocalStorage {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassPixelLocalStorage) -> Self {
        Self {
            total_pixel_local_storage_size: Some(value.totalPixelLocalStorageSize),
            storage_attachment_count: Some(value.storageAttachmentCount),
            storage_attachments: if value.storageAttachments.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.storageAttachments, value.storageAttachmentCount as usize) }
                            .iter()
                            .map(|raw| RenderPassStorageAttachment::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct RenderPassStorageAttachment {
    pub extensions: Vec<RenderPassStorageAttachmentExtension>,
    pub offset: Option<u64>,
    pub storage: Option<TextureView>,
    pub load_op: Option<LoadOp>,
    pub store_op: Option<StoreOp>,
    pub clear_value: Option<Color>,
}

impl Default for RenderPassStorageAttachment {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        offset: Some(0),
        storage: None,
        load_op: None,
        store_op: None,
        clear_value: None,
        }
    }
}

impl RenderPassStorageAttachment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn storage(mut self, value: TextureView) -> Self {
        self.storage = Some(value);
        self
    }

    pub fn load_op(mut self, value: LoadOp) -> Self {
        self.load_op = Some(value);
        self
    }

    pub fn store_op(mut self, value: StoreOp) -> Self {
        self.store_op = Some(value);
        self
    }

    pub fn clear_value(mut self, value: Color) -> Self {
        self.clear_value = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RenderPassStorageAttachmentExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURenderPassStorageAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassStorageAttachment = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPassStorageAttachment) -> Self {
        Self {
        extensions: Vec::new(),
            offset: Some(value.offset),
            storage: Some(unsafe { TextureView::from_raw(value.storage) }),
            load_op: Some(value.loadOp.into()),
            store_op: Some(value.storeOp.into()),
            clear_value: Some(Color::from_ffi(value.clearValue)),
                }
    }
}

pub struct RenderPipelineDescriptor {
    pub extensions: Vec<RenderPipelineDescriptorExtension>,
    pub label: Option<String>,
    pub layout: Option<PipelineLayout>,
    pub vertex: Option<VertexState>,
    pub primitive: Option<PrimitiveState>,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: Option<MultisampleState>,
    pub fragment: Option<FragmentState>,
}

impl Default for RenderPipelineDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        layout: None,
        vertex: None,
        primitive: None,
        depth_stencil: None,
        multisample: None,
        fragment: None,
        }
    }
}

impl RenderPipelineDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn layout(mut self, value: PipelineLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn vertex(mut self, value: VertexState) -> Self {
        self.vertex = Some(value);
        self
    }

    pub fn primitive(mut self, value: PrimitiveState) -> Self {
        self.primitive = Some(value);
        self
    }

    pub fn depth_stencil(mut self, value: DepthStencilState) -> Self {
        self.depth_stencil = Some(value);
        self
    }

    pub fn multisample(mut self, value: MultisampleState) -> Self {
        self.multisample = Some(value);
        self
    }

    pub fn fragment(mut self, value: FragmentState) -> Self {
        self.fragment = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RenderPipelineDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURenderPipelineDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPipelineDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURenderPipelineDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            layout: if value.layout.is_null() {
                    None
                } else {
                    Some(unsafe { PipelineLayout::from_raw(value.layout) })
                },
            vertex: Some(VertexState::from_ffi(value.vertex)),
            primitive: Some(PrimitiveState::from_ffi(value.primitive)),
            depth_stencil: if value.depthStencil.is_null() {
                    None
                } else {
                    Some(DepthStencilState::from_ffi(unsafe { *value.depthStencil }))
                },
            multisample: Some(MultisampleState::from_ffi(value.multisample)),
            fragment: if value.fragment.is_null() {
                    None
                } else {
                    Some(FragmentState::from_ffi(unsafe { *value.fragment }))
                },
                }
    }
}

pub struct RequestAdapterWebGPUBackendOptions {

}

impl Default for RequestAdapterWebGPUBackendOptions {
    fn default() -> Self {
        Self {

        }
    }
}

impl RequestAdapterWebGPUBackendOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_ffi(&self) -> ffi::WGPURequestAdapterWebGPUBackendOptions {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPURequestAdapterWebGPUBackendOptions) -> Self {
        let _ = value;
                Self::default()
    }
}

pub struct RequestAdapterWebXROptions {
    pub xr_compatible: Option<bool>,
}

impl Default for RequestAdapterWebXROptions {
    fn default() -> Self {
        Self {
        xr_compatible: None,
        }
    }
}

impl RequestAdapterWebXROptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn xr_compatible(mut self, value: bool) -> Self {
        self.xr_compatible = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPURequestAdapterWebXROptions {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPURequestAdapterWebXROptions) -> Self {
        Self {
            xr_compatible: Some(value.xrCompatible != 0),
                }
    }
}

pub struct RequestAdapterOptions {
    pub extensions: Vec<RequestAdapterOptionsExtension>,
    pub feature_level: Option<FeatureLevel>,
    pub power_preference: Option<PowerPreference>,
    pub force_fallback_adapter: Option<bool>,
    pub backend_type: Option<BackendType>,
    pub compatible_surface: Option<Surface>,
}

impl Default for RequestAdapterOptions {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        feature_level: Some(FeatureLevel::Core),
        power_preference: None,
        force_fallback_adapter: None,
        backend_type: None,
        compatible_surface: None,
        }
    }
}

impl RequestAdapterOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn feature_level(mut self, value: FeatureLevel) -> Self {
        self.feature_level = Some(value);
        self
    }

    pub fn power_preference(mut self, value: PowerPreference) -> Self {
        self.power_preference = Some(value);
        self
    }

    pub fn force_fallback_adapter(mut self, value: bool) -> Self {
        self.force_fallback_adapter = Some(value);
        self
    }

    pub fn backend_type(mut self, value: BackendType) -> Self {
        self.backend_type = Some(value);
        self
    }

    pub fn compatible_surface(mut self, value: Surface) -> Self {
        self.compatible_surface = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: RequestAdapterOptionsExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPURequestAdapterOptions, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURequestAdapterOptions = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPURequestAdapterOptions) -> Self {
        Self {
        extensions: Vec::new(),
            feature_level: Some(value.featureLevel.into()),
            power_preference: Some(value.powerPreference.into()),
            force_fallback_adapter: Some(value.forceFallbackAdapter != 0),
            backend_type: Some(value.backendType.into()),
            compatible_surface: if value.compatibleSurface.is_null() {
                    None
                } else {
                    Some(unsafe { Surface::from_raw(value.compatibleSurface) })
                },
                }
    }
}

pub struct ResourceTableDescriptor {
    pub extensions: Vec<ResourceTableDescriptorExtension>,
    pub label: Option<String>,
    pub size: Option<u32>,
}

impl Default for ResourceTableDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        size: None,
        }
    }
}

impl ResourceTableDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn size(mut self, value: u32) -> Self {
        self.size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ResourceTableDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUResourceTableDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUResourceTableDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUResourceTableDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            size: Some(value.size),
                }
    }
}

pub struct ResourceTableLimits {
    pub max_resource_table_size: Option<u32>,
}

impl Default for ResourceTableLimits {
    fn default() -> Self {
        Self {
        max_resource_table_size: None,
        }
    }
}

impl ResourceTableLimits {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn max_resource_table_size(mut self, value: u32) -> Self {
        self.max_resource_table_size = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUResourceTableLimits {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUResourceTableLimits) -> Self {
        Self {
            max_resource_table_size: Some(value.maxResourceTableSize),
                }
    }
}

pub struct SamplerBindingLayout {
    pub extensions: Vec<SamplerBindingLayoutExtension>,
    pub r#type: Option<SamplerBindingType>,
}

impl Default for SamplerBindingLayout {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        r#type: Some(SamplerBindingType::Filtering),
        }
    }
}

impl SamplerBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn r#type(mut self, value: SamplerBindingType) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SamplerBindingLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSamplerBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSamplerBindingLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSamplerBindingLayout) -> Self {
        Self {
        extensions: Vec::new(),
            r#type: Some(value.type_.into()),
                }
    }
}

pub struct SamplerDescriptor {
    pub extensions: Vec<SamplerDescriptorExtension>,
    pub label: Option<String>,
    pub address_mode_u: Option<AddressMode>,
    pub address_mode_v: Option<AddressMode>,
    pub address_mode_w: Option<AddressMode>,
    pub mag_filter: Option<FilterMode>,
    pub min_filter: Option<FilterMode>,
    pub mipmap_filter: Option<MipmapFilterMode>,
    pub lod_min_clamp: Option<f32>,
    pub lod_max_clamp: Option<f32>,
    pub compare: Option<CompareFunction>,
    pub max_anisotropy: Option<u16>,
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        address_mode_u: Some(AddressMode::ClampToEdge),
        address_mode_v: Some(AddressMode::ClampToEdge),
        address_mode_w: Some(AddressMode::ClampToEdge),
        mag_filter: Some(FilterMode::Nearest),
        min_filter: Some(FilterMode::Nearest),
        mipmap_filter: Some(MipmapFilterMode::Nearest),
        lod_min_clamp: None,
        lod_max_clamp: None,
        compare: None,
        max_anisotropy: Some(1),
        }
    }
}

impl SamplerDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn address_mode_u(mut self, value: AddressMode) -> Self {
        self.address_mode_u = Some(value);
        self
    }

    pub fn address_mode_v(mut self, value: AddressMode) -> Self {
        self.address_mode_v = Some(value);
        self
    }

    pub fn address_mode_w(mut self, value: AddressMode) -> Self {
        self.address_mode_w = Some(value);
        self
    }

    pub fn mag_filter(mut self, value: FilterMode) -> Self {
        self.mag_filter = Some(value);
        self
    }

    pub fn min_filter(mut self, value: FilterMode) -> Self {
        self.min_filter = Some(value);
        self
    }

    pub fn mipmap_filter(mut self, value: MipmapFilterMode) -> Self {
        self.mipmap_filter = Some(value);
        self
    }

    pub fn lod_min_clamp(mut self, value: f32) -> Self {
        self.lod_min_clamp = Some(value);
        self
    }

    pub fn lod_max_clamp(mut self, value: f32) -> Self {
        self.lod_max_clamp = Some(value);
        self
    }

    pub fn compare(mut self, value: CompareFunction) -> Self {
        self.compare = Some(value);
        self
    }

    pub fn max_anisotropy(mut self, value: u16) -> Self {
        self.max_anisotropy = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SamplerDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSamplerDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSamplerDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSamplerDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            address_mode_u: Some(value.addressModeU.into()),
            address_mode_v: Some(value.addressModeV.into()),
            address_mode_w: Some(value.addressModeW.into()),
            mag_filter: Some(value.magFilter.into()),
            min_filter: Some(value.minFilter.into()),
            mipmap_filter: Some(value.mipmapFilter.into()),
            lod_min_clamp: Some(value.lodMinClamp),
            lod_max_clamp: Some(value.lodMaxClamp),
            compare: Some(value.compare.into()),
            max_anisotropy: Some(value.maxAnisotropy),
                }
    }
}

pub struct ShaderModuleCompilationOptions {
    pub strict_math: Option<bool>,
}

impl Default for ShaderModuleCompilationOptions {
    fn default() -> Self {
        Self {
        strict_math: None,
        }
    }
}

impl ShaderModuleCompilationOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn strict_math(mut self, value: bool) -> Self {
        self.strict_math = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUShaderModuleCompilationOptions {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUShaderModuleCompilationOptions) -> Self {
        Self {
            strict_math: Some(value.strictMath != 0),
                }
    }
}

pub struct ShaderModuleDescriptor {
    pub extensions: Vec<ShaderModuleDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for ShaderModuleDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl ShaderModuleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: ShaderModuleDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUShaderModuleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUShaderModuleDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUShaderModuleDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct ShaderSourceSPIRV {
    pub code_size: Option<u32>,
    pub code: Option<Vec<u32>>,
}

impl Default for ShaderSourceSPIRV {
    fn default() -> Self {
        Self {
        code_size: None,
        code: None,
        }
    }
}

impl ShaderSourceSPIRV {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn code_size(mut self, value: u32) -> Self {
        self.code_size = Some(value);
        self
    }

    pub fn code(mut self, value: Vec<u32>) -> Self {
        self.code = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUShaderSourceSPIRV {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUShaderSourceSPIRV) -> Self {
        Self {
            code_size: Some(value.codeSize),
            code: if value.code.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.code, value.codeSize as usize) }.to_vec())
                },
                }
    }
}

pub struct ShaderSourceWGSL {
    pub code: Option<String>,
}

impl Default for ShaderSourceWGSL {
    fn default() -> Self {
        Self {
        code: None,
        }
    }
}

impl ShaderSourceWGSL {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUShaderSourceWGSL {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUShaderSourceWGSL) -> Self {
        Self {
            code: Some(string_view_to_string(value.code)),
                }
    }
}

pub struct SharedBufferMemoryBeginAccessDescriptor {
    pub extensions: Vec<SharedBufferMemoryBeginAccessDescriptorExtension>,
    pub initialized: Option<bool>,
    pub fence_count: Option<usize>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}

impl Default for SharedBufferMemoryBeginAccessDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        initialized: None,
        fence_count: Some(0),
        fences: None,
        signaled_values: None,
        }
    }
}

impl SharedBufferMemoryBeginAccessDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn initialized(mut self, value: bool) -> Self {
        self.initialized = Some(value);
        self
    }

    pub fn fence_count(mut self, value: usize) -> Self {
        self.fence_count = Some(value);
        self
    }

    pub fn fences(mut self, value: Vec<SharedFence>) -> Self {
        self.fences = Some(value);
        self
    }

    pub fn signaled_values(mut self, value: Vec<u64>) -> Self {
        self.signaled_values = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedBufferMemoryBeginAccessDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedBufferMemoryBeginAccessDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryBeginAccessDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedBufferMemoryBeginAccessDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            initialized: Some(value.initialized != 0),
            fence_count: Some(value.fenceCount),
            fences: if value.fences.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.fences, value.fenceCount as usize) }
                            .iter()
                            .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                            .collect(),
                    )
                },
            signaled_values: if value.signaledValues.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.signaledValues, value.fenceCount as usize) }.to_vec())
                },
                }
    }
}

pub struct SharedBufferMemoryDescriptor {
    pub extensions: Vec<SharedBufferMemoryDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for SharedBufferMemoryDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl SharedBufferMemoryDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedBufferMemoryDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedBufferMemoryDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedBufferMemoryDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct SharedBufferMemoryEndAccessState {
    pub extensions: Vec<SharedBufferMemoryEndAccessStateExtension>,
    pub initialized: Option<bool>,
    pub fence_count: Option<usize>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}

impl Default for SharedBufferMemoryEndAccessState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        initialized: None,
        fence_count: Some(0),
        fences: None,
        signaled_values: None,
        }
    }
}

impl SharedBufferMemoryEndAccessState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn initialized(mut self, value: bool) -> Self {
        self.initialized = Some(value);
        self
    }

    pub fn fence_count(mut self, value: usize) -> Self {
        self.fence_count = Some(value);
        self
    }

    pub fn fences(mut self, value: Vec<SharedFence>) -> Self {
        self.fences = Some(value);
        self
    }

    pub fn signaled_values(mut self, value: Vec<u64>) -> Self {
        self.signaled_values = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedBufferMemoryEndAccessStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedBufferMemoryEndAccessState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryEndAccessState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedBufferMemoryEndAccessState) -> Self {
        Self {
        extensions: Vec::new(),
            initialized: Some(value.initialized != 0),
            fence_count: Some(value.fenceCount),
            fences: if value.fences.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.fences, value.fenceCount as usize) }
                            .iter()
                            .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                            .collect(),
                    )
                },
            signaled_values: if value.signaledValues.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.signaledValues, value.fenceCount as usize) }.to_vec())
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUSharedBufferMemoryEndAccessState) {
        unsafe { ffi::wgpuSharedBufferMemoryEndAccessStateFreeMembers(value) };
    }
}

pub struct SharedBufferMemoryProperties {
    pub extensions: Vec<SharedBufferMemoryPropertiesExtension>,
    pub usage: Option<BufferUsage>,
    pub size: Option<u64>,
}

impl Default for SharedBufferMemoryProperties {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        usage: None,
        size: None,
        }
    }
}

impl SharedBufferMemoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn usage(mut self, value: BufferUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedBufferMemoryPropertiesExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedBufferMemoryProperties, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryProperties = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedBufferMemoryProperties) -> Self {
        Self {
        extensions: Vec::new(),
            usage: Some(value.usage.into()),
            size: Some(value.size),
                }
    }
}

pub struct SharedFenceDXGISharedHandleDescriptor {
    pub handle: Option<*mut std::ffi::c_void>,
}

impl Default for SharedFenceDXGISharedHandleDescriptor {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceDXGISharedHandleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: *mut std::ffi::c_void) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceDXGISharedHandleDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceDXGISharedHandleDescriptor) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedFenceDXGISharedHandleExportInfo {
    pub handle: Option<*mut std::ffi::c_void>,
}

impl Default for SharedFenceDXGISharedHandleExportInfo {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceDXGISharedHandleExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: *mut std::ffi::c_void) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceDXGISharedHandleExportInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceDXGISharedHandleExportInfo) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedFenceEGLSyncDescriptor {
    pub sync: Option<*mut std::ffi::c_void>,
}

impl Default for SharedFenceEGLSyncDescriptor {
    fn default() -> Self {
        Self {
        sync: None,
        }
    }
}

impl SharedFenceEGLSyncDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sync(mut self, value: *mut std::ffi::c_void) -> Self {
        self.sync = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceEGLSyncDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceEGLSyncDescriptor) -> Self {
        Self {
            sync: Some(value.sync),
                }
    }
}

pub struct SharedFenceEGLSyncExportInfo {
    pub sync: Option<*mut std::ffi::c_void>,
}

impl Default for SharedFenceEGLSyncExportInfo {
    fn default() -> Self {
        Self {
        sync: None,
        }
    }
}

impl SharedFenceEGLSyncExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sync(mut self, value: *mut std::ffi::c_void) -> Self {
        self.sync = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceEGLSyncExportInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceEGLSyncExportInfo) -> Self {
        Self {
            sync: Some(value.sync),
                }
    }
}

pub struct SharedFenceMTLSharedEventDescriptor {
    pub shared_event: Option<*mut std::ffi::c_void>,
}

impl Default for SharedFenceMTLSharedEventDescriptor {
    fn default() -> Self {
        Self {
        shared_event: None,
        }
    }
}

impl SharedFenceMTLSharedEventDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn shared_event(mut self, value: *mut std::ffi::c_void) -> Self {
        self.shared_event = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceMTLSharedEventDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceMTLSharedEventDescriptor) -> Self {
        Self {
            shared_event: Some(value.sharedEvent),
                }
    }
}

pub struct SharedFenceMTLSharedEventExportInfo {
    pub shared_event: Option<*mut std::ffi::c_void>,
}

impl Default for SharedFenceMTLSharedEventExportInfo {
    fn default() -> Self {
        Self {
        shared_event: None,
        }
    }
}

impl SharedFenceMTLSharedEventExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn shared_event(mut self, value: *mut std::ffi::c_void) -> Self {
        self.shared_event = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceMTLSharedEventExportInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceMTLSharedEventExportInfo) -> Self {
        Self {
            shared_event: Some(value.sharedEvent),
                }
    }
}

pub struct SharedFenceDescriptor {
    pub extensions: Vec<SharedFenceDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for SharedFenceDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl SharedFenceDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedFenceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedFenceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedFenceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct SharedFenceExportInfo {
    pub extensions: Vec<SharedFenceExportInfoExtension>,
    pub r#type: Option<SharedFenceType>,
}

impl Default for SharedFenceExportInfo {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        r#type: None,
        }
    }
}

impl SharedFenceExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn r#type(mut self, value: SharedFenceType) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedFenceExportInfoExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedFenceExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedFenceExportInfo = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceExportInfo) -> Self {
        Self {
        extensions: Vec::new(),
            r#type: Some(value.type_.into()),
                }
    }
}

pub struct SharedFenceSyncFDDescriptor {
    pub handle: Option<i32>,
}

impl Default for SharedFenceSyncFDDescriptor {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceSyncFDDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: i32) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceSyncFDDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceSyncFDDescriptor) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedFenceSyncFDExportInfo {
    pub handle: Option<i32>,
}

impl Default for SharedFenceSyncFDExportInfo {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceSyncFDExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: i32) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceSyncFDExportInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceSyncFDExportInfo) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedFenceVkSemaphoreOpaqueFDDescriptor {
    pub handle: Option<i32>,
}

impl Default for SharedFenceVkSemaphoreOpaqueFDDescriptor {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceVkSemaphoreOpaqueFDDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: i32) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceVkSemaphoreOpaqueFDDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceVkSemaphoreOpaqueFDDescriptor) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedFenceVkSemaphoreOpaqueFDExportInfo {
    pub handle: Option<i32>,
}

impl Default for SharedFenceVkSemaphoreOpaqueFDExportInfo {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceVkSemaphoreOpaqueFDExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: i32) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceVkSemaphoreOpaqueFDExportInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceVkSemaphoreOpaqueFDExportInfo) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedFenceVkSemaphoreZirconHandleDescriptor {
    pub handle: Option<u32>,
}

impl Default for SharedFenceVkSemaphoreZirconHandleDescriptor {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceVkSemaphoreZirconHandleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: u32) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceVkSemaphoreZirconHandleDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceVkSemaphoreZirconHandleDescriptor) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedFenceVkSemaphoreZirconHandleExportInfo {
    pub handle: Option<u32>,
}

impl Default for SharedFenceVkSemaphoreZirconHandleExportInfo {
    fn default() -> Self {
        Self {
        handle: None,
        }
    }
}

impl SharedFenceVkSemaphoreZirconHandleExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: u32) -> Self {
        self.handle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedFenceVkSemaphoreZirconHandleExportInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceVkSemaphoreZirconHandleExportInfo) -> Self {
        Self {
            handle: Some(value.handle),
                }
    }
}

pub struct SharedTextureMemoryD3DSwapchainBeginState {
    pub is_swapchain: Option<bool>,
}

impl Default for SharedTextureMemoryD3DSwapchainBeginState {
    fn default() -> Self {
        Self {
        is_swapchain: None,
        }
    }
}

impl SharedTextureMemoryD3DSwapchainBeginState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_swapchain(mut self, value: bool) -> Self {
        self.is_swapchain = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryD3DSwapchainBeginState {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryD3DSwapchainBeginState) -> Self {
        Self {
            is_swapchain: Some(value.isSwapchain != 0),
                }
    }
}

pub struct SharedTextureMemoryD3D11BeginState {
    pub requires_end_access_fence: Option<bool>,
}

impl Default for SharedTextureMemoryD3D11BeginState {
    fn default() -> Self {
        Self {
        requires_end_access_fence: None,
        }
    }
}

impl SharedTextureMemoryD3D11BeginState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn requires_end_access_fence(mut self, value: bool) -> Self {
        self.requires_end_access_fence = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryD3D11BeginState {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryD3D11BeginState) -> Self {
        Self {
            requires_end_access_fence: Some(value.requiresEndAccessFence != 0),
                }
    }
}

pub struct SharedTextureMemoryDXGISharedHandleDescriptor {
    pub handle: Option<*mut std::ffi::c_void>,
    pub use_keyed_mutex: Option<bool>,
}

impl Default for SharedTextureMemoryDXGISharedHandleDescriptor {
    fn default() -> Self {
        Self {
        handle: None,
        use_keyed_mutex: None,
        }
    }
}

impl SharedTextureMemoryDXGISharedHandleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: *mut std::ffi::c_void) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn use_keyed_mutex(mut self, value: bool) -> Self {
        self.use_keyed_mutex = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryDXGISharedHandleDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryDXGISharedHandleDescriptor) -> Self {
        Self {
            handle: Some(value.handle),
            use_keyed_mutex: Some(value.useKeyedMutex != 0),
                }
    }
}

pub struct SharedTextureMemoryEGLImageDescriptor {
    pub image: Option<*mut std::ffi::c_void>,
}

impl Default for SharedTextureMemoryEGLImageDescriptor {
    fn default() -> Self {
        Self {
        image: None,
        }
    }
}

impl SharedTextureMemoryEGLImageDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn image(mut self, value: *mut std::ffi::c_void) -> Self {
        self.image = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryEGLImageDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryEGLImageDescriptor) -> Self {
        Self {
            image: Some(value.image),
                }
    }
}

pub struct SharedTextureMemoryIOSurfaceDescriptor {
    pub io_surface: Option<*mut std::ffi::c_void>,
    pub allow_storage_binding: Option<bool>,
}

impl Default for SharedTextureMemoryIOSurfaceDescriptor {
    fn default() -> Self {
        Self {
        io_surface: None,
        allow_storage_binding: None,
        }
    }
}

impl SharedTextureMemoryIOSurfaceDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn io_surface(mut self, value: *mut std::ffi::c_void) -> Self {
        self.io_surface = Some(value);
        self
    }

    pub fn allow_storage_binding(mut self, value: bool) -> Self {
        self.allow_storage_binding = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryIOSurfaceDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryIOSurfaceDescriptor) -> Self {
        Self {
            io_surface: Some(value.ioSurface),
            allow_storage_binding: Some(value.allowStorageBinding != 0),
                }
    }
}

pub struct SharedTextureMemoryAHardwareBufferDescriptor {
    pub handle: Option<*mut std::ffi::c_void>,
    pub use_external_format: Option<bool>,
}

impl Default for SharedTextureMemoryAHardwareBufferDescriptor {
    fn default() -> Self {
        Self {
        handle: None,
        use_external_format: None,
        }
    }
}

impl SharedTextureMemoryAHardwareBufferDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn handle(mut self, value: *mut std::ffi::c_void) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn use_external_format(mut self, value: bool) -> Self {
        self.use_external_format = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryAHardwareBufferDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryAHardwareBufferDescriptor) -> Self {
        Self {
            handle: Some(value.handle),
            use_external_format: Some(value.useExternalFormat != 0),
                }
    }
}

pub struct SharedTextureMemoryAHardwareBufferProperties {
    pub y_cb_cr_info: Option<YCbCrVkDescriptor>,
}

impl Default for SharedTextureMemoryAHardwareBufferProperties {
    fn default() -> Self {
        Self {
        y_cb_cr_info: None,
        }
    }
}

impl SharedTextureMemoryAHardwareBufferProperties {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn y_cb_cr_info(mut self, value: YCbCrVkDescriptor) -> Self {
        self.y_cb_cr_info = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryAHardwareBufferProperties {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryAHardwareBufferProperties) -> Self {
        Self {
            y_cb_cr_info: Some(YCbCrVkDescriptor::from_ffi(value.yCbCrInfo)),
                }
    }
}

pub struct SharedTextureMemoryBeginAccessDescriptor {
    pub extensions: Vec<SharedTextureMemoryBeginAccessDescriptorExtension>,
    pub concurrent_read: Option<bool>,
    pub initialized: Option<bool>,
    pub fence_count: Option<usize>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}

impl Default for SharedTextureMemoryBeginAccessDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        concurrent_read: None,
        initialized: None,
        fence_count: None,
        fences: None,
        signaled_values: None,
        }
    }
}

impl SharedTextureMemoryBeginAccessDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn concurrent_read(mut self, value: bool) -> Self {
        self.concurrent_read = Some(value);
        self
    }

    pub fn initialized(mut self, value: bool) -> Self {
        self.initialized = Some(value);
        self
    }

    pub fn fence_count(mut self, value: usize) -> Self {
        self.fence_count = Some(value);
        self
    }

    pub fn fences(mut self, value: Vec<SharedFence>) -> Self {
        self.fences = Some(value);
        self
    }

    pub fn signaled_values(mut self, value: Vec<u64>) -> Self {
        self.signaled_values = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedTextureMemoryBeginAccessDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedTextureMemoryBeginAccessDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryBeginAccessDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryBeginAccessDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            concurrent_read: Some(value.concurrentRead != 0),
            initialized: Some(value.initialized != 0),
            fence_count: Some(value.fenceCount),
            fences: if value.fences.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.fences, value.fenceCount as usize) }
                            .iter()
                            .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                            .collect(),
                    )
                },
            signaled_values: if value.signaledValues.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.signaledValues, value.fenceCount as usize) }.to_vec())
                },
                }
    }
}

pub struct SharedTextureMemoryDescriptor {
    pub extensions: Vec<SharedTextureMemoryDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for SharedTextureMemoryDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl SharedTextureMemoryDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedTextureMemoryDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedTextureMemoryDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct SharedTextureMemoryDmaBufDescriptor {
    pub size: Option<Extent3D>,
    pub drm_format: Option<u32>,
    pub drm_modifier: Option<u64>,
    pub plane_count: Option<usize>,
    pub planes: Option<Vec<SharedTextureMemoryDmaBufPlane>>,
}

impl Default for SharedTextureMemoryDmaBufDescriptor {
    fn default() -> Self {
        Self {
        size: None,
        drm_format: None,
        drm_modifier: None,
        plane_count: None,
        planes: None,
        }
    }
}

impl SharedTextureMemoryDmaBufDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn size(mut self, value: Extent3D) -> Self {
        self.size = Some(value);
        self
    }

    pub fn drm_format(mut self, value: u32) -> Self {
        self.drm_format = Some(value);
        self
    }

    pub fn drm_modifier(mut self, value: u64) -> Self {
        self.drm_modifier = Some(value);
        self
    }

    pub fn plane_count(mut self, value: usize) -> Self {
        self.plane_count = Some(value);
        self
    }

    pub fn planes(mut self, value: Vec<SharedTextureMemoryDmaBufPlane>) -> Self {
        self.planes = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryDmaBufDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryDmaBufDescriptor) -> Self {
        Self {
            size: Some(Extent3D::from_ffi(value.size)),
            drm_format: Some(value.drmFormat),
            drm_modifier: Some(value.drmModifier),
            plane_count: Some(value.planeCount),
            planes: if value.planes.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.planes, value.planeCount as usize) }
                            .iter()
                            .map(|raw| SharedTextureMemoryDmaBufPlane::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct SharedTextureMemoryDmaBufPlane {
    pub fd: Option<i32>,
    pub offset: Option<u64>,
    pub stride: Option<u32>,
}

impl Default for SharedTextureMemoryDmaBufPlane {
    fn default() -> Self {
        Self {
        fd: None,
        offset: None,
        stride: None,
        }
    }
}

impl SharedTextureMemoryDmaBufPlane {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn fd(mut self, value: i32) -> Self {
        self.fd = Some(value);
        self
    }

    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn stride(mut self, value: u32) -> Self {
        self.stride = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryDmaBufPlane {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryDmaBufPlane) -> Self {
        Self {
            fd: Some(value.fd),
            offset: Some(value.offset),
            stride: Some(value.stride),
                }
    }
}

pub struct SharedTextureMemoryEndAccessState {
    pub extensions: Vec<SharedTextureMemoryEndAccessStateExtension>,
    pub initialized: Option<bool>,
    pub fence_count: Option<usize>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}

impl Default for SharedTextureMemoryEndAccessState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        initialized: None,
        fence_count: None,
        fences: None,
        signaled_values: None,
        }
    }
}

impl SharedTextureMemoryEndAccessState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn initialized(mut self, value: bool) -> Self {
        self.initialized = Some(value);
        self
    }

    pub fn fence_count(mut self, value: usize) -> Self {
        self.fence_count = Some(value);
        self
    }

    pub fn fences(mut self, value: Vec<SharedFence>) -> Self {
        self.fences = Some(value);
        self
    }

    pub fn signaled_values(mut self, value: Vec<u64>) -> Self {
        self.signaled_values = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedTextureMemoryEndAccessStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedTextureMemoryEndAccessState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryEndAccessState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryEndAccessState) -> Self {
        Self {
        extensions: Vec::new(),
            initialized: Some(value.initialized != 0),
            fence_count: Some(value.fenceCount),
            fences: if value.fences.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.fences, value.fenceCount as usize) }
                            .iter()
                            .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                            .collect(),
                    )
                },
            signaled_values: if value.signaledValues.is_null() {
                    None
                } else {
                    Some(unsafe { std::slice::from_raw_parts(value.signaledValues, value.fenceCount as usize) }.to_vec())
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUSharedTextureMemoryEndAccessState) {
        unsafe { ffi::wgpuSharedTextureMemoryEndAccessStateFreeMembers(value) };
    }
}

pub struct SharedTextureMemoryMetalEndAccessState {
    pub commands_scheduled_future: Option<Future>,
}

impl Default for SharedTextureMemoryMetalEndAccessState {
    fn default() -> Self {
        Self {
        commands_scheduled_future: None,
        }
    }
}

impl SharedTextureMemoryMetalEndAccessState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn commands_scheduled_future(mut self, value: Future) -> Self {
        self.commands_scheduled_future = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryMetalEndAccessState {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryMetalEndAccessState) -> Self {
        Self {
            commands_scheduled_future: Some(Future::from_ffi(value.commandsScheduledFuture)),
                }
    }
}

pub struct SharedTextureMemoryOpaqueFDDescriptor {
    pub vk_image_create_info: Option<*const std::ffi::c_void>,
    pub memory_fd: Option<i32>,
    pub memory_type_index: Option<u32>,
    pub allocation_size: Option<u64>,
    pub dedicated_allocation: Option<bool>,
}

impl Default for SharedTextureMemoryOpaqueFDDescriptor {
    fn default() -> Self {
        Self {
        vk_image_create_info: None,
        memory_fd: None,
        memory_type_index: None,
        allocation_size: None,
        dedicated_allocation: None,
        }
    }
}

impl SharedTextureMemoryOpaqueFDDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn vk_image_create_info(mut self, value: *const std::ffi::c_void) -> Self {
        self.vk_image_create_info = Some(value);
        self
    }

    pub fn memory_fd(mut self, value: i32) -> Self {
        self.memory_fd = Some(value);
        self
    }

    pub fn memory_type_index(mut self, value: u32) -> Self {
        self.memory_type_index = Some(value);
        self
    }

    pub fn allocation_size(mut self, value: u64) -> Self {
        self.allocation_size = Some(value);
        self
    }

    pub fn dedicated_allocation(mut self, value: bool) -> Self {
        self.dedicated_allocation = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryOpaqueFDDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryOpaqueFDDescriptor) -> Self {
        Self {
            vk_image_create_info: Some(value.vkImageCreateInfo),
            memory_fd: Some(value.memoryFD),
            memory_type_index: Some(value.memoryTypeIndex),
            allocation_size: Some(value.allocationSize),
            dedicated_allocation: Some(value.dedicatedAllocation != 0),
                }
    }
}

pub struct SharedTextureMemoryProperties {
    pub extensions: Vec<SharedTextureMemoryPropertiesExtension>,
    pub usage: Option<TextureUsage>,
    pub size: Option<Extent3D>,
    pub format: Option<TextureFormat>,
}

impl Default for SharedTextureMemoryProperties {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        usage: None,
        size: None,
        format: None,
        }
    }
}

impl SharedTextureMemoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn usage(mut self, value: TextureUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn size(mut self, value: Extent3D) -> Self {
        self.size = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SharedTextureMemoryPropertiesExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSharedTextureMemoryProperties, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryProperties = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryProperties) -> Self {
        Self {
        extensions: Vec::new(),
            usage: Some(value.usage.into()),
            size: Some(Extent3D::from_ffi(value.size)),
            format: Some(value.format.into()),
                }
    }
}

pub struct SharedTextureMemoryVkDedicatedAllocationDescriptor {
    pub dedicated_allocation: Option<bool>,
}

impl Default for SharedTextureMemoryVkDedicatedAllocationDescriptor {
    fn default() -> Self {
        Self {
        dedicated_allocation: None,
        }
    }
}

impl SharedTextureMemoryVkDedicatedAllocationDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn dedicated_allocation(mut self, value: bool) -> Self {
        self.dedicated_allocation = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryVkDedicatedAllocationDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryVkDedicatedAllocationDescriptor) -> Self {
        Self {
            dedicated_allocation: Some(value.dedicatedAllocation != 0),
                }
    }
}

pub struct SharedTextureMemoryVkImageLayoutBeginState {
    pub old_layout: Option<i32>,
    pub new_layout: Option<i32>,
}

impl Default for SharedTextureMemoryVkImageLayoutBeginState {
    fn default() -> Self {
        Self {
        old_layout: None,
        new_layout: None,
        }
    }
}

impl SharedTextureMemoryVkImageLayoutBeginState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn old_layout(mut self, value: i32) -> Self {
        self.old_layout = Some(value);
        self
    }

    pub fn new_layout(mut self, value: i32) -> Self {
        self.new_layout = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryVkImageLayoutBeginState {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryVkImageLayoutBeginState) -> Self {
        Self {
            old_layout: Some(value.oldLayout),
            new_layout: Some(value.newLayout),
                }
    }
}

pub struct SharedTextureMemoryVkImageLayoutEndState {
    pub old_layout: Option<i32>,
    pub new_layout: Option<i32>,
}

impl Default for SharedTextureMemoryVkImageLayoutEndState {
    fn default() -> Self {
        Self {
        old_layout: None,
        new_layout: None,
        }
    }
}

impl SharedTextureMemoryVkImageLayoutEndState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn old_layout(mut self, value: i32) -> Self {
        self.old_layout = Some(value);
        self
    }

    pub fn new_layout(mut self, value: i32) -> Self {
        self.new_layout = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryVkImageLayoutEndState {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryVkImageLayoutEndState) -> Self {
        Self {
            old_layout: Some(value.oldLayout),
            new_layout: Some(value.newLayout),
                }
    }
}

pub struct SharedTextureMemoryZirconHandleDescriptor {
    pub memory_fd: Option<u32>,
    pub allocation_size: Option<u64>,
}

impl Default for SharedTextureMemoryZirconHandleDescriptor {
    fn default() -> Self {
        Self {
        memory_fd: None,
        allocation_size: None,
        }
    }
}

impl SharedTextureMemoryZirconHandleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn memory_fd(mut self, value: u32) -> Self {
        self.memory_fd = Some(value);
        self
    }

    pub fn allocation_size(mut self, value: u64) -> Self {
        self.allocation_size = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSharedTextureMemoryZirconHandleDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryZirconHandleDescriptor) -> Self {
        Self {
            memory_fd: Some(value.memoryFD),
            allocation_size: Some(value.allocationSize),
                }
    }
}

pub struct StaticSamplerBindingLayout {
    pub sampler: Option<Sampler>,
    pub sampled_texture_binding: Option<u32>,
}

impl Default for StaticSamplerBindingLayout {
    fn default() -> Self {
        Self {
        sampler: None,
        sampled_texture_binding: None,
        }
    }
}

impl StaticSamplerBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sampler(mut self, value: Sampler) -> Self {
        self.sampler = Some(value);
        self
    }

    pub fn sampled_texture_binding(mut self, value: u32) -> Self {
        self.sampled_texture_binding = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUStaticSamplerBindingLayout {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUStaticSamplerBindingLayout) -> Self {
        Self {
            sampler: Some(unsafe { Sampler::from_raw(value.sampler) }),
            sampled_texture_binding: Some(value.sampledTextureBinding),
                }
    }
}

pub struct StencilFaceState {
    pub compare: Option<CompareFunction>,
    pub fail_op: Option<StencilOperation>,
    pub depth_fail_op: Option<StencilOperation>,
    pub pass_op: Option<StencilOperation>,
}

impl Default for StencilFaceState {
    fn default() -> Self {
        Self {
        compare: Some(CompareFunction::Always),
        fail_op: Some(StencilOperation::Keep),
        depth_fail_op: Some(StencilOperation::Keep),
        pass_op: Some(StencilOperation::Keep),
        }
    }
}

impl StencilFaceState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn compare(mut self, value: CompareFunction) -> Self {
        self.compare = Some(value);
        self
    }

    pub fn fail_op(mut self, value: StencilOperation) -> Self {
        self.fail_op = Some(value);
        self
    }

    pub fn depth_fail_op(mut self, value: StencilOperation) -> Self {
        self.depth_fail_op = Some(value);
        self
    }

    pub fn pass_op(mut self, value: StencilOperation) -> Self {
        self.pass_op = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUStencilFaceState {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUStencilFaceState) -> Self {
        Self {
            compare: Some(value.compare.into()),
            fail_op: Some(value.failOp.into()),
            depth_fail_op: Some(value.depthFailOp.into()),
            pass_op: Some(value.passOp.into()),
                }
    }
}

pub struct StorageTextureBindingLayout {
    pub extensions: Vec<StorageTextureBindingLayoutExtension>,
    pub access: Option<StorageTextureAccess>,
    pub format: Option<TextureFormat>,
    pub view_dimension: Option<TextureViewDimension>,
}

impl Default for StorageTextureBindingLayout {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        access: Some(StorageTextureAccess::WriteOnly),
        format: None,
        view_dimension: Some(TextureViewDimension::D2),
        }
    }
}

impl StorageTextureBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn access(mut self, value: StorageTextureAccess) -> Self {
        self.access = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn view_dimension(mut self, value: TextureViewDimension) -> Self {
        self.view_dimension = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: StorageTextureBindingLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUStorageTextureBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUStorageTextureBindingLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUStorageTextureBindingLayout) -> Self {
        Self {
        extensions: Vec::new(),
            access: Some(value.access.into()),
            format: Some(value.format.into()),
            view_dimension: Some(value.viewDimension.into()),
                }
    }
}

pub struct StringView {
    pub data: Option<*const std::os::raw::c_char>,
    pub length: Option<usize>,
}

impl Default for StringView {
    fn default() -> Self {
        Self {
        data: None,
        length: None,
        }
    }
}

impl StringView {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn data(mut self, value: *const std::os::raw::c_char) -> Self {
        self.data = Some(value);
        self
    }

    pub fn length(mut self, value: usize) -> Self {
        self.length = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUStringView {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUStringView) -> Self {
        Self {
            data: Some(value.data),
            length: Some(value.length),
                }
    }
}

pub struct SubgroupMatrixConfig {
    pub component_type: Option<SubgroupMatrixComponentType>,
    pub result_component_type: Option<SubgroupMatrixComponentType>,
    pub m: Option<u32>,
    pub n: Option<u32>,
    pub k: Option<u32>,
}

impl Default for SubgroupMatrixConfig {
    fn default() -> Self {
        Self {
        component_type: None,
        result_component_type: None,
        m: None,
        n: None,
        k: None,
        }
    }
}

impl SubgroupMatrixConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn component_type(mut self, value: SubgroupMatrixComponentType) -> Self {
        self.component_type = Some(value);
        self
    }

    pub fn result_component_type(mut self, value: SubgroupMatrixComponentType) -> Self {
        self.result_component_type = Some(value);
        self
    }

    pub fn m(mut self, value: u32) -> Self {
        self.m = Some(value);
        self
    }

    pub fn n(mut self, value: u32) -> Self {
        self.n = Some(value);
        self
    }

    pub fn k(mut self, value: u32) -> Self {
        self.k = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSubgroupMatrixConfig {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSubgroupMatrixConfig) -> Self {
        Self {
            component_type: Some(value.componentType.into()),
            result_component_type: Some(value.resultComponentType.into()),
            m: Some(value.M),
            n: Some(value.N),
            k: Some(value.K),
                }
    }
}

pub struct SupportedWGSLLanguageFeatures {
    pub feature_count: Option<usize>,
    pub features: Option<Vec<WGSLLanguageFeatureName>>,
}

impl Default for SupportedWGSLLanguageFeatures {
    fn default() -> Self {
        Self {
        feature_count: None,
        features: None,
        }
    }
}

impl SupportedWGSLLanguageFeatures {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn feature_count(mut self, value: usize) -> Self {
        self.feature_count = Some(value);
        self
    }

    pub fn features(mut self, value: Vec<WGSLLanguageFeatureName>) -> Self {
        self.features = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSupportedWGSLLanguageFeatures {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSupportedWGSLLanguageFeatures) -> Self {
        Self {
            feature_count: Some(value.featureCount),
            features: if value.features.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.features, value.featureCount as usize) }
                            .iter()
                            .map(|raw| WGSLLanguageFeatureName::from(*raw))
                            .collect(),
                    )
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUSupportedWGSLLanguageFeatures) {
        unsafe { ffi::wgpuSupportedWGSLLanguageFeaturesFreeMembers(value) };
    }
}

pub struct SupportedFeatures {
    pub feature_count: Option<usize>,
    pub features: Option<Vec<FeatureName>>,
}

impl Default for SupportedFeatures {
    fn default() -> Self {
        Self {
        feature_count: None,
        features: None,
        }
    }
}

impl SupportedFeatures {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn feature_count(mut self, value: usize) -> Self {
        self.feature_count = Some(value);
        self
    }

    pub fn features(mut self, value: Vec<FeatureName>) -> Self {
        self.features = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSupportedFeatures {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSupportedFeatures) -> Self {
        Self {
            feature_count: Some(value.featureCount),
            features: if value.features.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.features, value.featureCount as usize) }
                            .iter()
                            .map(|raw| FeatureName::from(*raw))
                            .collect(),
                    )
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUSupportedFeatures) {
        unsafe { ffi::wgpuSupportedFeaturesFreeMembers(value) };
    }
}

pub struct SupportedInstanceFeatures {
    pub feature_count: Option<usize>,
    pub features: Option<Vec<InstanceFeatureName>>,
}

impl Default for SupportedInstanceFeatures {
    fn default() -> Self {
        Self {
        feature_count: None,
        features: None,
        }
    }
}

impl SupportedInstanceFeatures {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn feature_count(mut self, value: usize) -> Self {
        self.feature_count = Some(value);
        self
    }

    pub fn features(mut self, value: Vec<InstanceFeatureName>) -> Self {
        self.features = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSupportedInstanceFeatures {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSupportedInstanceFeatures) -> Self {
        Self {
            feature_count: Some(value.featureCount),
            features: if value.features.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.features, value.featureCount as usize) }
                            .iter()
                            .map(|raw| InstanceFeatureName::from(*raw))
                            .collect(),
                    )
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUSupportedInstanceFeatures) {
        unsafe { ffi::wgpuSupportedInstanceFeaturesFreeMembers(value) };
    }
}

pub struct SurfaceCapabilities {
    pub extensions: Vec<SurfaceCapabilitiesExtension>,
    pub usages: Option<TextureUsage>,
    pub format_count: Option<usize>,
    pub formats: Option<Vec<TextureFormat>>,
    pub present_mode_count: Option<usize>,
    pub present_modes: Option<Vec<PresentMode>>,
    pub alpha_mode_count: Option<usize>,
    pub alpha_modes: Option<Vec<CompositeAlphaMode>>,
}

impl Default for SurfaceCapabilities {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        usages: None,
        format_count: None,
        formats: None,
        present_mode_count: None,
        present_modes: None,
        alpha_mode_count: None,
        alpha_modes: None,
        }
    }
}

impl SurfaceCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn usages(mut self, value: TextureUsage) -> Self {
        self.usages = Some(value);
        self
    }

    pub fn format_count(mut self, value: usize) -> Self {
        self.format_count = Some(value);
        self
    }

    pub fn formats(mut self, value: Vec<TextureFormat>) -> Self {
        self.formats = Some(value);
        self
    }

    pub fn present_mode_count(mut self, value: usize) -> Self {
        self.present_mode_count = Some(value);
        self
    }

    pub fn present_modes(mut self, value: Vec<PresentMode>) -> Self {
        self.present_modes = Some(value);
        self
    }

    pub fn alpha_mode_count(mut self, value: usize) -> Self {
        self.alpha_mode_count = Some(value);
        self
    }

    pub fn alpha_modes(mut self, value: Vec<CompositeAlphaMode>) -> Self {
        self.alpha_modes = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SurfaceCapabilitiesExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSurfaceCapabilities, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceCapabilities = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceCapabilities) -> Self {
        Self {
        extensions: Vec::new(),
            usages: Some(value.usages.into()),
            format_count: Some(value.formatCount),
            formats: if value.formats.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.formats, value.formatCount as usize) }
                            .iter()
                            .map(|raw| TextureFormat::from(*raw))
                            .collect(),
                    )
                },
            present_mode_count: Some(value.presentModeCount),
            present_modes: if value.presentModes.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.presentModes, value.presentModeCount as usize) }
                            .iter()
                            .map(|raw| PresentMode::from(*raw))
                            .collect(),
                    )
                },
            alpha_mode_count: Some(value.alphaModeCount),
            alpha_modes: if value.alphaModes.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.alphaModes, value.alphaModeCount as usize) }
                            .iter()
                            .map(|raw| CompositeAlphaMode::from(*raw))
                            .collect(),
                    )
                },
                }
    }

    pub(crate) fn free_members(value: ffi::WGPUSurfaceCapabilities) {
        unsafe { ffi::wgpuSurfaceCapabilitiesFreeMembers(value) };
    }
}

pub struct SurfaceColorManagement {
    pub color_space: Option<PredefinedColorSpace>,
    pub tone_mapping_mode: Option<ToneMappingMode>,
}

impl Default for SurfaceColorManagement {
    fn default() -> Self {
        Self {
        color_space: None,
        tone_mapping_mode: None,
        }
    }
}

impl SurfaceColorManagement {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn color_space(mut self, value: PredefinedColorSpace) -> Self {
        self.color_space = Some(value);
        self
    }

    pub fn tone_mapping_mode(mut self, value: ToneMappingMode) -> Self {
        self.tone_mapping_mode = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceColorManagement {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceColorManagement) -> Self {
        Self {
            color_space: Some(value.colorSpace.into()),
            tone_mapping_mode: Some(value.toneMappingMode.into()),
                }
    }
}

pub struct SurfaceConfiguration {
    pub extensions: Vec<SurfaceConfigurationExtension>,
    pub device: Option<Device>,
    pub format: Option<TextureFormat>,
    pub usage: Option<TextureUsage>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub view_format_count: Option<usize>,
    pub view_formats: Option<Vec<TextureFormat>>,
    pub alpha_mode: Option<CompositeAlphaMode>,
    pub present_mode: Option<PresentMode>,
}

impl Default for SurfaceConfiguration {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        device: None,
        format: None,
        usage: Some(TextureUsage::RENDER_ATTACHMENT),
        width: None,
        height: None,
        view_format_count: Some(0),
        view_formats: None,
        alpha_mode: Some(CompositeAlphaMode::Auto),
        present_mode: Some(PresentMode::Fifo),
        }
    }
}

impl SurfaceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn device(mut self, value: Device) -> Self {
        self.device = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn usage(mut self, value: TextureUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(value);
        self
    }

    pub fn view_format_count(mut self, value: usize) -> Self {
        self.view_format_count = Some(value);
        self
    }

    pub fn view_formats(mut self, value: Vec<TextureFormat>) -> Self {
        self.view_formats = Some(value);
        self
    }

    pub fn alpha_mode(mut self, value: CompositeAlphaMode) -> Self {
        self.alpha_mode = Some(value);
        self
    }

    pub fn present_mode(mut self, value: PresentMode) -> Self {
        self.present_mode = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SurfaceConfigurationExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSurfaceConfiguration, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceConfiguration = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceConfiguration) -> Self {
        Self {
        extensions: Vec::new(),
            device: Some(unsafe { Device::from_raw(value.device) }),
            format: Some(value.format.into()),
            usage: Some(value.usage.into()),
            width: Some(value.width),
            height: Some(value.height),
            view_format_count: Some(value.viewFormatCount),
            view_formats: if value.viewFormats.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.viewFormats, value.viewFormatCount as usize) }
                            .iter()
                            .map(|raw| TextureFormat::from(*raw))
                            .collect(),
                    )
                },
            alpha_mode: Some(value.alphaMode.into()),
            present_mode: Some(value.presentMode.into()),
                }
    }
}

pub struct SurfaceDescriptor {
    pub extensions: Vec<SurfaceDescriptorExtension>,
    pub label: Option<String>,
}

impl Default for SurfaceDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        }
    }
}

impl SurfaceDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SurfaceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSurfaceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
                }
    }
}

pub struct SurfaceDescriptorFromWindowsUWPSwapChainPanel {
    pub swap_chain_panel: Option<*mut std::ffi::c_void>,
}

impl Default for SurfaceDescriptorFromWindowsUWPSwapChainPanel {
    fn default() -> Self {
        Self {
        swap_chain_panel: None,
        }
    }
}

impl SurfaceDescriptorFromWindowsUWPSwapChainPanel {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn swap_chain_panel(mut self, value: *mut std::ffi::c_void) -> Self {
        self.swap_chain_panel = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceDescriptorFromWindowsUWPSwapChainPanel {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceDescriptorFromWindowsUWPSwapChainPanel) -> Self {
        Self {
            swap_chain_panel: Some(value.swapChainPanel),
                }
    }
}

pub struct SurfaceDescriptorFromWindowsWinUISwapChainPanel {
    pub swap_chain_panel: Option<*mut std::ffi::c_void>,
}

impl Default for SurfaceDescriptorFromWindowsWinUISwapChainPanel {
    fn default() -> Self {
        Self {
        swap_chain_panel: None,
        }
    }
}

impl SurfaceDescriptorFromWindowsWinUISwapChainPanel {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn swap_chain_panel(mut self, value: *mut std::ffi::c_void) -> Self {
        self.swap_chain_panel = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceDescriptorFromWindowsWinUISwapChainPanel {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceDescriptorFromWindowsWinUISwapChainPanel) -> Self {
        Self {
            swap_chain_panel: Some(value.swapChainPanel),
                }
    }
}

pub struct SurfaceDescriptorFromWindowsCoreWindow {
    pub core_window: Option<*mut std::ffi::c_void>,
}

impl Default for SurfaceDescriptorFromWindowsCoreWindow {
    fn default() -> Self {
        Self {
        core_window: None,
        }
    }
}

impl SurfaceDescriptorFromWindowsCoreWindow {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn core_window(mut self, value: *mut std::ffi::c_void) -> Self {
        self.core_window = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceDescriptorFromWindowsCoreWindow {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceDescriptorFromWindowsCoreWindow) -> Self {
        Self {
            core_window: Some(value.coreWindow),
                }
    }
}

pub struct SurfaceSourceXCBWindow {
    pub connection: Option<*mut std::ffi::c_void>,
    pub window: Option<u32>,
}

impl Default for SurfaceSourceXCBWindow {
    fn default() -> Self {
        Self {
        connection: None,
        window: None,
        }
    }
}

impl SurfaceSourceXCBWindow {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn connection(mut self, value: *mut std::ffi::c_void) -> Self {
        self.connection = Some(value);
        self
    }

    pub fn window(mut self, value: u32) -> Self {
        self.window = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceSourceXCBWindow {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceXCBWindow) -> Self {
        Self {
            connection: Some(value.connection),
            window: Some(value.window),
                }
    }
}

pub struct SurfaceSourceAndroidNativeWindow {
    pub window: Option<*mut std::ffi::c_void>,
}

impl Default for SurfaceSourceAndroidNativeWindow {
    fn default() -> Self {
        Self {
        window: None,
        }
    }
}

impl SurfaceSourceAndroidNativeWindow {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn window(mut self, value: *mut std::ffi::c_void) -> Self {
        self.window = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceSourceAndroidNativeWindow {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceAndroidNativeWindow) -> Self {
        Self {
            window: Some(value.window),
                }
    }
}

pub struct SurfaceSourceMetalLayer {
    pub layer: Option<*mut std::ffi::c_void>,
}

impl Default for SurfaceSourceMetalLayer {
    fn default() -> Self {
        Self {
        layer: None,
        }
    }
}

impl SurfaceSourceMetalLayer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn layer(mut self, value: *mut std::ffi::c_void) -> Self {
        self.layer = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceSourceMetalLayer {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceMetalLayer) -> Self {
        Self {
            layer: Some(value.layer),
                }
    }
}

pub struct SurfaceSourceWaylandSurface {
    pub display: Option<*mut std::ffi::c_void>,
    pub surface: Option<*mut std::ffi::c_void>,
}

impl Default for SurfaceSourceWaylandSurface {
    fn default() -> Self {
        Self {
        display: None,
        surface: None,
        }
    }
}

impl SurfaceSourceWaylandSurface {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn display(mut self, value: *mut std::ffi::c_void) -> Self {
        self.display = Some(value);
        self
    }

    pub fn surface(mut self, value: *mut std::ffi::c_void) -> Self {
        self.surface = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceSourceWaylandSurface {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceWaylandSurface) -> Self {
        Self {
            display: Some(value.display),
            surface: Some(value.surface),
                }
    }
}

pub struct SurfaceSourceWindowsHWND {
    pub hinstance: Option<*mut std::ffi::c_void>,
    pub hwnd: Option<*mut std::ffi::c_void>,
}

impl Default for SurfaceSourceWindowsHWND {
    fn default() -> Self {
        Self {
        hinstance: None,
        hwnd: None,
        }
    }
}

impl SurfaceSourceWindowsHWND {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn hinstance(mut self, value: *mut std::ffi::c_void) -> Self {
        self.hinstance = Some(value);
        self
    }

    pub fn hwnd(mut self, value: *mut std::ffi::c_void) -> Self {
        self.hwnd = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceSourceWindowsHWND {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceWindowsHWND) -> Self {
        Self {
            hinstance: Some(value.hinstance),
            hwnd: Some(value.hwnd),
                }
    }
}

pub struct SurfaceSourceXlibWindow {
    pub display: Option<*mut std::ffi::c_void>,
    pub window: Option<u64>,
}

impl Default for SurfaceSourceXlibWindow {
    fn default() -> Self {
        Self {
        display: None,
        window: None,
        }
    }
}

impl SurfaceSourceXlibWindow {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn display(mut self, value: *mut std::ffi::c_void) -> Self {
        self.display = Some(value);
        self
    }

    pub fn window(mut self, value: u64) -> Self {
        self.window = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUSurfaceSourceXlibWindow {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceXlibWindow) -> Self {
        Self {
            display: Some(value.display),
            window: Some(value.window),
                }
    }
}

pub struct SurfaceTexture {
    pub extensions: Vec<SurfaceTextureExtension>,
    pub texture: Option<Texture>,
    pub status: Option<SurfaceGetCurrentTextureStatus>,
}

impl Default for SurfaceTexture {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        texture: None,
        status: None,
        }
    }
}

impl SurfaceTexture {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn texture(mut self, value: Texture) -> Self {
        self.texture = Some(value);
        self
    }

    pub fn status(mut self, value: SurfaceGetCurrentTextureStatus) -> Self {
        self.status = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: SurfaceTextureExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUSurfaceTexture, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceTexture = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceTexture) -> Self {
        Self {
        extensions: Vec::new(),
            texture: Some(unsafe { Texture::from_raw(value.texture) }),
            status: Some(value.status.into()),
                }
    }
}

pub struct TexelBufferBindingEntry {
    pub texel_buffer_view: Option<TexelBufferView>,
}

impl Default for TexelBufferBindingEntry {
    fn default() -> Self {
        Self {
        texel_buffer_view: None,
        }
    }
}

impl TexelBufferBindingEntry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn texel_buffer_view(mut self, value: TexelBufferView) -> Self {
        self.texel_buffer_view = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUTexelBufferBindingEntry {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTexelBufferBindingEntry) -> Self {
        Self {
            texel_buffer_view: Some(unsafe { TexelBufferView::from_raw(value.texelBufferView) }),
                }
    }
}

pub struct TexelBufferBindingLayout {
    pub access: Option<TexelBufferAccess>,
    pub format: Option<TextureFormat>,
}

impl Default for TexelBufferBindingLayout {
    fn default() -> Self {
        Self {
        access: Some(TexelBufferAccess::ReadWrite),
        format: None,
        }
    }
}

impl TexelBufferBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn access(mut self, value: TexelBufferAccess) -> Self {
        self.access = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUTexelBufferBindingLayout {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTexelBufferBindingLayout) -> Self {
        Self {
            access: Some(value.access.into()),
            format: Some(value.format.into()),
                }
    }
}

pub struct TexelBufferViewDescriptor {
    pub extensions: Vec<TexelBufferViewDescriptorExtension>,
    pub label: Option<String>,
    pub format: Option<TextureFormat>,
    pub offset: Option<u64>,
    pub size: Option<u64>,
}

impl Default for TexelBufferViewDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        format: None,
        offset: Some(0),
        size: None,
        }
    }
}

impl TexelBufferViewDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: TexelBufferViewDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUTexelBufferViewDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTexelBufferViewDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTexelBufferViewDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            format: Some(value.format.into()),
            offset: Some(value.offset),
            size: Some(value.size),
                }
    }
}

pub struct TexelCopyBufferInfo {
    pub layout: Option<TexelCopyBufferLayout>,
    pub buffer: Option<Buffer>,
}

impl Default for TexelCopyBufferInfo {
    fn default() -> Self {
        Self {
        layout: None,
        buffer: None,
        }
    }
}

impl TexelCopyBufferInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn layout(mut self, value: TexelCopyBufferLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn buffer(mut self, value: Buffer) -> Self {
        self.buffer = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUTexelCopyBufferInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTexelCopyBufferInfo) -> Self {
        Self {
            layout: Some(TexelCopyBufferLayout::from_ffi(value.layout)),
            buffer: Some(unsafe { Buffer::from_raw(value.buffer) }),
                }
    }
}

pub struct TexelCopyBufferLayout {
    pub offset: Option<u64>,
    pub bytes_per_row: Option<u32>,
    pub rows_per_image: Option<u32>,
}

impl Default for TexelCopyBufferLayout {
    fn default() -> Self {
        Self {
        offset: Some(0),
        bytes_per_row: None,
        rows_per_image: None,
        }
    }
}

impl TexelCopyBufferLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn bytes_per_row(mut self, value: u32) -> Self {
        self.bytes_per_row = Some(value);
        self
    }

    pub fn rows_per_image(mut self, value: u32) -> Self {
        self.rows_per_image = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUTexelCopyBufferLayout {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTexelCopyBufferLayout) -> Self {
        Self {
            offset: Some(value.offset),
            bytes_per_row: Some(value.bytesPerRow),
            rows_per_image: Some(value.rowsPerImage),
                }
    }
}

pub struct TexelCopyTextureInfo {
    pub texture: Option<Texture>,
    pub mip_level: Option<u32>,
    pub origin: Option<Origin3D>,
    pub aspect: Option<TextureAspect>,
}

impl Default for TexelCopyTextureInfo {
    fn default() -> Self {
        Self {
        texture: None,
        mip_level: Some(0),
        origin: None,
        aspect: Some(TextureAspect::All),
        }
    }
}

impl TexelCopyTextureInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn texture(mut self, value: Texture) -> Self {
        self.texture = Some(value);
        self
    }

    pub fn mip_level(mut self, value: u32) -> Self {
        self.mip_level = Some(value);
        self
    }

    pub fn origin(mut self, value: Origin3D) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn aspect(mut self, value: TextureAspect) -> Self {
        self.aspect = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUTexelCopyTextureInfo {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTexelCopyTextureInfo) -> Self {
        Self {
            texture: Some(unsafe { Texture::from_raw(value.texture) }),
            mip_level: Some(value.mipLevel),
            origin: Some(Origin3D::from_ffi(value.origin)),
            aspect: Some(value.aspect.into()),
                }
    }
}

pub struct TextureBindingLayout {
    pub extensions: Vec<TextureBindingLayoutExtension>,
    pub sample_type: Option<TextureSampleType>,
    pub view_dimension: Option<TextureViewDimension>,
    pub multisampled: Option<bool>,
}

impl Default for TextureBindingLayout {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        sample_type: Some(TextureSampleType::Float),
        view_dimension: Some(TextureViewDimension::D2),
        multisampled: None,
        }
    }
}

impl TextureBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn sample_type(mut self, value: TextureSampleType) -> Self {
        self.sample_type = Some(value);
        self
    }

    pub fn view_dimension(mut self, value: TextureViewDimension) -> Self {
        self.view_dimension = Some(value);
        self
    }

    pub fn multisampled(mut self, value: bool) -> Self {
        self.multisampled = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: TextureBindingLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUTextureBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTextureBindingLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTextureBindingLayout) -> Self {
        Self {
        extensions: Vec::new(),
            sample_type: Some(value.sampleType.into()),
            view_dimension: Some(value.viewDimension.into()),
            multisampled: Some(value.multisampled != 0),
                }
    }
}

pub struct TextureComponentSwizzle {
    pub r: Option<ComponentSwizzle>,
    pub g: Option<ComponentSwizzle>,
    pub b: Option<ComponentSwizzle>,
    pub a: Option<ComponentSwizzle>,
}

impl Default for TextureComponentSwizzle {
    fn default() -> Self {
        Self {
        r: Some(ComponentSwizzle::R),
        g: Some(ComponentSwizzle::G),
        b: Some(ComponentSwizzle::B),
        a: Some(ComponentSwizzle::A),
        }
    }
}

impl TextureComponentSwizzle {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn r(mut self, value: ComponentSwizzle) -> Self {
        self.r = Some(value);
        self
    }

    pub fn g(mut self, value: ComponentSwizzle) -> Self {
        self.g = Some(value);
        self
    }

    pub fn b(mut self, value: ComponentSwizzle) -> Self {
        self.b = Some(value);
        self
    }

    pub fn a(mut self, value: ComponentSwizzle) -> Self {
        self.a = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUTextureComponentSwizzle {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTextureComponentSwizzle) -> Self {
        Self {
            r: Some(value.r.into()),
            g: Some(value.g.into()),
            b: Some(value.b.into()),
            a: Some(value.a.into()),
                }
    }
}

pub struct TextureComponentSwizzleDescriptor {
    pub swizzle: Option<TextureComponentSwizzle>,
}

impl Default for TextureComponentSwizzleDescriptor {
    fn default() -> Self {
        Self {
        swizzle: None,
        }
    }
}

impl TextureComponentSwizzleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn swizzle(mut self, value: TextureComponentSwizzle) -> Self {
        self.swizzle = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUTextureComponentSwizzleDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTextureComponentSwizzleDescriptor) -> Self {
        Self {
            swizzle: Some(TextureComponentSwizzle::from_ffi(value.swizzle)),
                }
    }
}

pub struct TextureDescriptor {
    pub extensions: Vec<TextureDescriptorExtension>,
    pub label: Option<String>,
    pub usage: Option<TextureUsage>,
    pub dimension: Option<TextureDimension>,
    pub size: Option<Extent3D>,
    pub format: Option<TextureFormat>,
    pub mip_level_count: Option<u32>,
    pub sample_count: Option<u32>,
    pub view_format_count: Option<usize>,
    pub view_formats: Option<Vec<TextureFormat>>,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        usage: None,
        dimension: Some(TextureDimension::D2),
        size: None,
        format: None,
        mip_level_count: Some(1),
        sample_count: Some(1),
        view_format_count: Some(0),
        view_formats: None,
        }
    }
}

impl TextureDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn usage(mut self, value: TextureUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn dimension(mut self, value: TextureDimension) -> Self {
        self.dimension = Some(value);
        self
    }

    pub fn size(mut self, value: Extent3D) -> Self {
        self.size = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn mip_level_count(mut self, value: u32) -> Self {
        self.mip_level_count = Some(value);
        self
    }

    pub fn sample_count(mut self, value: u32) -> Self {
        self.sample_count = Some(value);
        self
    }

    pub fn view_format_count(mut self, value: usize) -> Self {
        self.view_format_count = Some(value);
        self
    }

    pub fn view_formats(mut self, value: Vec<TextureFormat>) -> Self {
        self.view_formats = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: TextureDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUTextureDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTextureDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTextureDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            usage: Some(value.usage.into()),
            dimension: Some(value.dimension.into()),
            size: Some(Extent3D::from_ffi(value.size)),
            format: Some(value.format.into()),
            mip_level_count: Some(value.mipLevelCount),
            sample_count: Some(value.sampleCount),
            view_format_count: Some(value.viewFormatCount),
            view_formats: if value.viewFormats.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.viewFormats, value.viewFormatCount as usize) }
                            .iter()
                            .map(|raw| TextureFormat::from(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct TextureViewDescriptor {
    pub extensions: Vec<TextureViewDescriptorExtension>,
    pub label: Option<String>,
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub base_mip_level: Option<u32>,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: Option<u32>,
    pub array_layer_count: Option<u32>,
    pub aspect: Option<TextureAspect>,
    pub usage: Option<TextureUsage>,
}

impl Default for TextureViewDescriptor {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        label: None,
        format: None,
        dimension: None,
        base_mip_level: Some(0),
        mip_level_count: None,
        base_array_layer: Some(0),
        array_layer_count: None,
        aspect: Some(TextureAspect::All),
        usage: None,
        }
    }
}

impl TextureViewDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn format(mut self, value: TextureFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn dimension(mut self, value: TextureViewDimension) -> Self {
        self.dimension = Some(value);
        self
    }

    pub fn base_mip_level(mut self, value: u32) -> Self {
        self.base_mip_level = Some(value);
        self
    }

    pub fn mip_level_count(mut self, value: u32) -> Self {
        self.mip_level_count = Some(value);
        self
    }

    pub fn base_array_layer(mut self, value: u32) -> Self {
        self.base_array_layer = Some(value);
        self
    }

    pub fn array_layer_count(mut self, value: u32) -> Self {
        self.array_layer_count = Some(value);
        self
    }

    pub fn aspect(mut self, value: TextureAspect) -> Self {
        self.aspect = Some(value);
        self
    }

    pub fn usage(mut self, value: TextureUsage) -> Self {
        self.usage = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: TextureViewDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUTextureViewDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTextureViewDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUTextureViewDescriptor) -> Self {
        Self {
        extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.label))
                },
            format: Some(value.format.into()),
            dimension: Some(value.dimension.into()),
            base_mip_level: Some(value.baseMipLevel),
            mip_level_count: Some(value.mipLevelCount),
            base_array_layer: Some(value.baseArrayLayer),
            array_layer_count: Some(value.arrayLayerCount),
            aspect: Some(value.aspect.into()),
            usage: Some(value.usage.into()),
                }
    }
}

pub struct VertexAttribute {
    pub extensions: Vec<VertexAttributeExtension>,
    pub format: Option<VertexFormat>,
    pub offset: Option<u64>,
    pub shader_location: Option<u32>,
}

impl Default for VertexAttribute {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        format: None,
        offset: None,
        shader_location: None,
        }
    }
}

impl VertexAttribute {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn format(mut self, value: VertexFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn offset(mut self, value: u64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn shader_location(mut self, value: u32) -> Self {
        self.shader_location = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: VertexAttributeExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUVertexAttribute, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUVertexAttribute = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUVertexAttribute) -> Self {
        Self {
        extensions: Vec::new(),
            format: Some(value.format.into()),
            offset: Some(value.offset),
            shader_location: Some(value.shaderLocation),
                }
    }
}

pub struct VertexBufferLayout {
    pub extensions: Vec<VertexBufferLayoutExtension>,
    pub step_mode: Option<VertexStepMode>,
    pub array_stride: Option<u64>,
    pub attribute_count: Option<usize>,
    pub attributes: Option<Vec<VertexAttribute>>,
}

impl Default for VertexBufferLayout {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        step_mode: None,
        array_stride: None,
        attribute_count: None,
        attributes: None,
        }
    }
}

impl VertexBufferLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn step_mode(mut self, value: VertexStepMode) -> Self {
        self.step_mode = Some(value);
        self
    }

    pub fn array_stride(mut self, value: u64) -> Self {
        self.array_stride = Some(value);
        self
    }

    pub fn attribute_count(mut self, value: usize) -> Self {
        self.attribute_count = Some(value);
        self
    }

    pub fn attributes(mut self, value: Vec<VertexAttribute>) -> Self {
        self.attributes = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: VertexBufferLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUVertexBufferLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUVertexBufferLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUVertexBufferLayout) -> Self {
        Self {
        extensions: Vec::new(),
            step_mode: Some(value.stepMode.into()),
            array_stride: Some(value.arrayStride),
            attribute_count: Some(value.attributeCount),
            attributes: if value.attributes.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.attributes, value.attributeCount as usize) }
                            .iter()
                            .map(|raw| VertexAttribute::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct VertexState {
    pub extensions: Vec<VertexStateExtension>,
    pub module: Option<ShaderModule>,
    pub entry_point: Option<String>,
    pub constant_count: Option<usize>,
    pub constants: Option<Vec<ConstantEntry>>,
    pub buffer_count: Option<usize>,
    pub buffers: Option<Vec<VertexBufferLayout>>,
}

impl Default for VertexState {
    fn default() -> Self {
        Self {
        extensions: Vec::new(),
        module: None,
        entry_point: None,
        constant_count: Some(0),
        constants: None,
        buffer_count: Some(0),
        buffers: None,
        }
    }
}

impl VertexState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn module(mut self, value: ShaderModule) -> Self {
        self.module = Some(value);
        self
    }

    pub fn entry_point(mut self, value: String) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn constant_count(mut self, value: usize) -> Self {
        self.constant_count = Some(value);
        self
    }

    pub fn constants(mut self, value: Vec<ConstantEntry>) -> Self {
        self.constants = Some(value);
        self
    }

    pub fn buffer_count(mut self, value: usize) -> Self {
        self.buffer_count = Some(value);
        self
    }

    pub fn buffers(mut self, value: Vec<VertexBufferLayout>) -> Self {
        self.buffers = Some(value);
        self
    }
    pub fn with_extension(mut self, extension: VertexStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }

    pub fn to_ffi(&self) -> (ffi::WGPUVertexState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUVertexState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }

    pub(crate) fn from_ffi(value: ffi::WGPUVertexState) -> Self {
        Self {
        extensions: Vec::new(),
            module: Some(unsafe { ShaderModule::from_raw(value.module) }),
            entry_point: if value.entryPoint.data.is_null() || value.entryPoint.length == 0 {
                    None
                } else {
                    Some(string_view_to_string(value.entryPoint))
                },
            constant_count: Some(value.constantCount),
            constants: if value.constants.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.constants, value.constantCount as usize) }
                            .iter()
                            .map(|raw| ConstantEntry::from_ffi(*raw))
                            .collect(),
                    )
                },
            buffer_count: Some(value.bufferCount),
            buffers: if value.buffers.is_null() {
                    None
                } else {
                    Some(
                        unsafe { std::slice::from_raw_parts(value.buffers, value.bufferCount as usize) }
                            .iter()
                            .map(|raw| VertexBufferLayout::from_ffi(*raw))
                            .collect(),
                    )
                },
                }
    }
}

pub struct YCbCrVkDescriptor {
    pub vk_format: Option<u32>,
    pub vk_y_cb_cr_model: Option<u32>,
    pub vk_y_cb_cr_range: Option<u32>,
    pub vk_component_swizzle_red: Option<u32>,
    pub vk_component_swizzle_green: Option<u32>,
    pub vk_component_swizzle_blue: Option<u32>,
    pub vk_component_swizzle_alpha: Option<u32>,
    pub vk_x_chroma_offset: Option<u32>,
    pub vk_y_chroma_offset: Option<u32>,
    pub vk_chroma_filter: Option<FilterMode>,
    pub force_explicit_reconstruction: Option<bool>,
    pub external_format: Option<u64>,
}

impl Default for YCbCrVkDescriptor {
    fn default() -> Self {
        Self {
        vk_format: Some(0),
        vk_y_cb_cr_model: Some(0),
        vk_y_cb_cr_range: Some(0),
        vk_component_swizzle_red: Some(0),
        vk_component_swizzle_green: Some(0),
        vk_component_swizzle_blue: Some(0),
        vk_component_swizzle_alpha: Some(0),
        vk_x_chroma_offset: Some(0),
        vk_y_chroma_offset: Some(0),
        vk_chroma_filter: Some(FilterMode::Nearest),
        force_explicit_reconstruction: None,
        external_format: Some(0),
        }
    }
}

impl YCbCrVkDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn vk_format(mut self, value: u32) -> Self {
        self.vk_format = Some(value);
        self
    }

    pub fn vk_y_cb_cr_model(mut self, value: u32) -> Self {
        self.vk_y_cb_cr_model = Some(value);
        self
    }

    pub fn vk_y_cb_cr_range(mut self, value: u32) -> Self {
        self.vk_y_cb_cr_range = Some(value);
        self
    }

    pub fn vk_component_swizzle_red(mut self, value: u32) -> Self {
        self.vk_component_swizzle_red = Some(value);
        self
    }

    pub fn vk_component_swizzle_green(mut self, value: u32) -> Self {
        self.vk_component_swizzle_green = Some(value);
        self
    }

    pub fn vk_component_swizzle_blue(mut self, value: u32) -> Self {
        self.vk_component_swizzle_blue = Some(value);
        self
    }

    pub fn vk_component_swizzle_alpha(mut self, value: u32) -> Self {
        self.vk_component_swizzle_alpha = Some(value);
        self
    }

    pub fn vk_x_chroma_offset(mut self, value: u32) -> Self {
        self.vk_x_chroma_offset = Some(value);
        self
    }

    pub fn vk_y_chroma_offset(mut self, value: u32) -> Self {
        self.vk_y_chroma_offset = Some(value);
        self
    }

    pub fn vk_chroma_filter(mut self, value: FilterMode) -> Self {
        self.vk_chroma_filter = Some(value);
        self
    }

    pub fn force_explicit_reconstruction(mut self, value: bool) -> Self {
        self.force_explicit_reconstruction = Some(value);
        self
    }

    pub fn external_format(mut self, value: u64) -> Self {
        self.external_format = Some(value);
        self
    }
    pub fn to_ffi(&self) -> ffi::WGPUYCbCrVkDescriptor {
        let _ = self;
        unsafe { std::mem::zeroed() }
    }

    pub(crate) fn from_ffi(value: ffi::WGPUYCbCrVkDescriptor) -> Self {
        Self {
            vk_format: Some(value.vkFormat),
            vk_y_cb_cr_model: Some(value.vkYCbCrModel),
            vk_y_cb_cr_range: Some(value.vkYCbCrRange),
            vk_component_swizzle_red: Some(value.vkComponentSwizzleRed),
            vk_component_swizzle_green: Some(value.vkComponentSwizzleGreen),
            vk_component_swizzle_blue: Some(value.vkComponentSwizzleBlue),
            vk_component_swizzle_alpha: Some(value.vkComponentSwizzleAlpha),
            vk_x_chroma_offset: Some(value.vkXChromaOffset),
            vk_y_chroma_offset: Some(value.vkYChromaOffset),
            vk_chroma_filter: Some(value.vkChromaFilter.into()),
            force_explicit_reconstruction: Some(value.forceExplicitReconstruction != 0),
            external_format: Some(value.externalFormat),
                }
    }
}

