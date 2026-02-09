#![allow(dead_code, unused_imports)]
use crate::ffi;
use super::*;
use std::any::Any;
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
        Self { y_cb_cr_info: None }
    }
}
impl AHardwareBufferProperties {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUAHardwareBufferProperties, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUAHardwareBufferProperties = unsafe { std::mem::zeroed() };
        if let Some(value) = &self.y_cb_cr_info {
            let (raw_value, storage_value) = value.to_ffi();
            raw.yCbCrInfo = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUAHardwareBufferProperties) -> Self {
        Self {
            y_cb_cr_info: Some(YCbCrVkDescriptor::from_ffi(value.yCbCrInfo)),
        }
    }
}
pub struct AdapterInfo {
    pub(crate) extensions: Vec<AdapterInfoExtension>,
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
    pub fn push_extension(&mut self, ext: AdapterInfoExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUAdapterInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUAdapterInfo = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.vendor {
            raw.vendor = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.vendor = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = &self.architecture {
            raw.architecture = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.architecture = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = &self.device {
            raw.device = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.device = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = &self.description {
            raw.description = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.description = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.backend_type {
            raw.backendType = value.into();
        } else {
            raw.backendType = 0 as ffi::WGPUBackendType;
        }
        if let Some(value) = self.adapter_type {
            raw.adapterType = value.into();
        } else {
            raw.adapterType = 0 as ffi::WGPUAdapterType;
        }
        if let Some(value) = self.vendor_id {
            raw.vendorID = value;
        }
        if let Some(value) = self.device_id {
            raw.deviceID = value;
        }
        if let Some(value) = self.subgroup_min_size {
            raw.subgroupMinSize = value;
        }
        if let Some(value) = self.subgroup_max_size {
            raw.subgroupMaxSize = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: AdapterInfoExtension) -> Self {
        self.extensions.push(extension);
        self
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
        Self { shader_model: None }
    }
}
impl AdapterPropertiesD3D {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUAdapterPropertiesD3D, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUAdapterPropertiesD3D = unsafe { std::mem::zeroed() };
        if let Some(value) = self.shader_model {
            raw.shaderModel = value;
        }
        (raw, storage)
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
        Self { backend_type: None }
    }
}
impl AdapterPropertiesWGPU {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUAdapterPropertiesWGPU, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUAdapterPropertiesWGPU = unsafe { std::mem::zeroed() };
        if let Some(value) = self.backend_type {
            raw.backendType = value.into();
        } else {
            raw.backendType = 0 as ffi::WGPUBackendType;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (
        ffi::WGPUAdapterPropertiesExplicitComputeSubgroupSizeConfigs,
        ChainedStructStorage,
    ) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUAdapterPropertiesExplicitComputeSubgroupSizeConfigs = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.min_explicit_compute_subgroup_size {
            raw.minExplicitComputeSubgroupSize = value;
        }
        if let Some(value) = self.max_explicit_compute_subgroup_size {
            raw.maxExplicitComputeSubgroupSize = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUAdapterPropertiesExplicitComputeSubgroupSizeConfigs,
    ) -> Self {
        Self {
            min_explicit_compute_subgroup_size: Some(
                value.minExplicitComputeSubgroupSize,
            ),
            max_explicit_compute_subgroup_size: Some(
                value.maxExplicitComputeSubgroupSize,
            ),
        }
    }
}
pub struct AdapterPropertiesMemoryHeaps {
    pub heap_info: Option<Vec<MemoryHeapInfo>>,
}
impl Default for AdapterPropertiesMemoryHeaps {
    fn default() -> Self {
        Self { heap_info: None }
    }
}
impl AdapterPropertiesMemoryHeaps {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUAdapterPropertiesMemoryHeaps, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUAdapterPropertiesMemoryHeaps = unsafe {
            std::mem::zeroed()
        };
        raw.heapCount = self.heap_info.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.heap_info {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUMemoryHeapInfo> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.heapInfo = ptr;
            raw.heapCount = len_value;
        } else {
            raw.heapInfo = std::ptr::null();
            raw.heapCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesMemoryHeaps) -> Self {
        Self {
            heap_info: if value.heapInfo.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.heapInfo,
                            value.heapCount as usize,
                        )
                    }
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
    pub configs: Option<Vec<SubgroupMatrixConfig>>,
}
impl Default for AdapterPropertiesSubgroupMatrixConfigs {
    fn default() -> Self {
        Self { configs: None }
    }
}
impl AdapterPropertiesSubgroupMatrixConfigs {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs = unsafe {
            std::mem::zeroed()
        };
        raw.configCount = self.configs.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.configs {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUSubgroupMatrixConfig> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.configs = ptr;
            raw.configCount = len_value;
        } else {
            raw.configs = std::ptr::null();
            raw.configCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs,
    ) -> Self {
        Self {
            configs: if value.configs.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.configs,
                            value.configCount as usize,
                        )
                    }
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
        Self { driver_version: None }
    }
}
impl AdapterPropertiesVk {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUAdapterPropertiesVk, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUAdapterPropertiesVk = unsafe { std::mem::zeroed() };
        if let Some(value) = self.driver_version {
            raw.driverVersion = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUAdapterPropertiesVk) -> Self {
        Self {
            driver_version: Some(value.driverVersion),
        }
    }
}
pub struct BindGroupDescriptor {
    pub(crate) extensions: Vec<BindGroupDescriptorExtension>,
    pub label: Option<String>,
    pub layout: Option<BindGroupLayout>,
    pub entries: Option<Vec<BindGroupEntry>>,
}
impl Default for BindGroupDescriptor {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            label: None,
            layout: None,
            entries: None,
        }
    }
}
impl BindGroupDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: BindGroupDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUBindGroupDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.layout = self
            .layout
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        raw.entryCount = self.entries.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.entries {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUBindGroupEntry> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.entries = ptr;
            raw.entryCount = len_value;
        } else {
            raw.entries = std::ptr::null();
            raw.entryCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: BindGroupDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
            entries: if value.entries.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.entries,
                            value.entryCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| BindGroupEntry::from_ffi(*raw))
                        .collect(),
                )
            },
        }
    }
}
pub struct BindGroupEntry {
    pub(crate) extensions: Vec<BindGroupEntryExtension>,
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
    pub fn push_extension(&mut self, ext: BindGroupEntryExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUBindGroupEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupEntry = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.binding {
            raw.binding = value;
        }
        raw.buffer = self
            .buffer
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.size {
            raw.size = value;
        }
        raw.sampler = self
            .sampler
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        raw.textureView = self
            .texture_view
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: BindGroupEntryExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<BindGroupLayoutDescriptorExtension>,
    pub label: Option<String>,
    pub entries: Option<Vec<BindGroupLayoutEntry>>,
}
impl Default for BindGroupLayoutDescriptor {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            label: None,
            entries: None,
        }
    }
}
impl BindGroupLayoutDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: BindGroupLayoutDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUBindGroupLayoutDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupLayoutDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.entryCount = self.entries.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.entries {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUBindGroupLayoutEntry> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.entries = ptr;
            raw.entryCount = len_value;
        } else {
            raw.entries = std::ptr::null();
            raw.entryCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: BindGroupLayoutDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUBindGroupLayoutDescriptor) -> Self {
        Self {
            extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                None
            } else {
                Some(string_view_to_string(value.label))
            },
            entries: if value.entries.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.entries,
                            value.entryCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| BindGroupLayoutEntry::from_ffi(*raw))
                        .collect(),
                )
            },
        }
    }
}
pub struct BindGroupLayoutEntry {
    pub(crate) extensions: Vec<BindGroupLayoutEntryExtension>,
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
    pub fn push_extension(&mut self, ext: BindGroupLayoutEntryExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUBindGroupLayoutEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindGroupLayoutEntry = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.binding {
            raw.binding = value;
        }
        if let Some(value) = self.visibility {
            raw.visibility = value.into();
        } else {
            raw.visibility = 0 as ffi::WGPUShaderStage;
        }
        if let Some(value) = self.binding_array_size {
            raw.bindingArraySize = value;
        }
        if let Some(value) = &self.buffer {
            let (raw_value, storage_value) = value.to_ffi();
            raw.buffer = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.sampler {
            let (raw_value, storage_value) = value.to_ffi();
            raw.sampler = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.texture {
            let (raw_value, storage_value) = value.to_ffi();
            raw.texture = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.storage_texture {
            let (raw_value, storage_value) = value.to_ffi();
            raw.storageTexture = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: BindGroupLayoutEntryExtension) -> Self {
        self.extensions.push(extension);
        self
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
            storage_texture: Some(
                StorageTextureBindingLayout::from_ffi(value.storageTexture),
            ),
        }
    }
}
pub struct BindingResource {
    pub(crate) extensions: Vec<BindingResourceExtension>,
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
    pub fn push_extension(&mut self, ext: BindingResourceExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUBindingResource, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBindingResource = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.buffer = self
            .buffer
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.size {
            raw.size = value;
        }
        raw.sampler = self
            .sampler
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        raw.textureView = self
            .texture_view
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: BindingResourceExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUBlendComponent, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUBlendComponent = unsafe { std::mem::zeroed() };
        if let Some(value) = self.operation {
            raw.operation = value.into();
        } else {
            raw.operation = 0 as ffi::WGPUBlendOperation;
        }
        if let Some(value) = self.src_factor {
            raw.srcFactor = value.into();
        } else {
            raw.srcFactor = 0 as ffi::WGPUBlendFactor;
        }
        if let Some(value) = self.dst_factor {
            raw.dstFactor = value.into();
        } else {
            raw.dstFactor = 0 as ffi::WGPUBlendFactor;
        }
        (raw, storage)
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
        Self { color: None, alpha: None }
    }
}
impl BlendState {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUBlendState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUBlendState = unsafe { std::mem::zeroed() };
        if let Some(value) = &self.color {
            let (raw_value, storage_value) = value.to_ffi();
            raw.color = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.alpha {
            let (raw_value, storage_value) = value.to_ffi();
            raw.alpha = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUBlendState) -> Self {
        Self {
            color: Some(BlendComponent::from_ffi(value.color)),
            alpha: Some(BlendComponent::from_ffi(value.alpha)),
        }
    }
}
pub struct BufferBindingLayout {
    pub(crate) extensions: Vec<BufferBindingLayoutExtension>,
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
    pub fn push_extension(&mut self, ext: BufferBindingLayoutExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUBufferBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBufferBindingLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.r#type {
            raw.type_ = value.into();
        } else {
            raw.type_ = 0 as ffi::WGPUBufferBindingType;
        }
        raw.hasDynamicOffset = if self.has_dynamic_offset.unwrap_or(false) {
            1
        } else {
            0
        };
        if let Some(value) = self.min_binding_size {
            raw.minBindingSize = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: BufferBindingLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<BufferDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: BufferDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUBufferDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUBufferDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.usage {
            raw.usage = value.into();
        } else {
            raw.usage = 0 as ffi::WGPUBufferUsage;
        }
        if let Some(value) = self.size {
            raw.size = value;
        }
        raw.mappedAtCreation = if self.mapped_at_creation.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: BufferDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUBufferHostMappedPointer, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUBufferHostMappedPointer = unsafe { std::mem::zeroed() };
        if let Some(value) = self.pointer {
            raw.pointer = value;
        }
        if let Some(value) = self.dispose_callback {
            raw.disposeCallback = value;
        }
        if let Some(value) = self.userdata {
            raw.userdata = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUColor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUColor = unsafe { std::mem::zeroed() };
        if let Some(value) = self.r {
            raw.r = value;
        }
        if let Some(value) = self.g {
            raw.g = value;
        }
        if let Some(value) = self.b {
            raw.b = value;
        }
        if let Some(value) = self.a {
            raw.a = value;
        }
        (raw, storage)
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
    pub(crate) extensions: Vec<ColorTargetStateExtension>,
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
    pub fn push_extension(&mut self, ext: ColorTargetStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUColorTargetState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUColorTargetState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = &self.blend {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.blend = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.blend = std::ptr::null();
        }
        if let Some(value) = self.write_mask {
            raw.writeMask = value.into();
        } else {
            raw.writeMask = 0 as ffi::WGPUColorWriteMask;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: ColorTargetStateExtension) -> Self {
        self.extensions.push(extension);
        self
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
        Self { enabled: None }
    }
}
impl ColorTargetStateExpandResolveTextureDawn {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUColorTargetStateExpandResolveTextureDawn, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUColorTargetStateExpandResolveTextureDawn = unsafe {
            std::mem::zeroed()
        };
        raw.enabled = if self.enabled.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUColorTargetStateExpandResolveTextureDawn,
    ) -> Self {
        Self {
            enabled: Some(value.enabled != 0),
        }
    }
}
pub struct CommandBufferDescriptor {
    pub(crate) extensions: Vec<CommandBufferDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: CommandBufferDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUCommandBufferDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCommandBufferDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: CommandBufferDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<CommandEncoderDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: CommandEncoderDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUCommandEncoderDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCommandEncoderDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: CommandEncoderDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<CompilationInfoExtension>,
    pub messages: Option<Vec<CompilationMessage>>,
}
impl Default for CompilationInfo {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            messages: None,
        }
    }
}
impl CompilationInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: CompilationInfoExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUCompilationInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCompilationInfo = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.messageCount = self.messages.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.messages {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUCompilationMessage> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.messages = ptr;
            raw.messageCount = len_value;
        } else {
            raw.messages = std::ptr::null();
            raw.messageCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: CompilationInfoExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUCompilationInfo) -> Self {
        Self {
            extensions: Vec::new(),
            messages: if value.messages.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.messages,
                            value.messageCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| CompilationMessage::from_ffi(*raw))
                        .collect(),
                )
            },
        }
    }
}
pub struct CompilationMessage {
    pub(crate) extensions: Vec<CompilationMessageExtension>,
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
    pub fn push_extension(&mut self, ext: CompilationMessageExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUCompilationMessage, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCompilationMessage = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.message {
            raw.message = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.message = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.r#type {
            raw.type_ = value.into();
        } else {
            raw.type_ = 0 as ffi::WGPUCompilationMessageType;
        }
        if let Some(value) = self.line_num {
            raw.lineNum = value;
        }
        if let Some(value) = self.line_pos {
            raw.linePos = value;
        }
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.length {
            raw.length = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: CompilationMessageExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<ComputePassDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: ComputePassDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUComputePassDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUComputePassDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = &self.timestamp_writes {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.timestampWrites = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.timestampWrites = std::ptr::null();
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: ComputePassDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<ComputePipelineDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: ComputePipelineDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUComputePipelineDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUComputePipelineDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.layout = self
            .layout
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.compute {
            let (raw_value, storage_value) = value.to_ffi();
            raw.compute = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: ComputePipelineDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<ComputeStateExtension>,
    pub module: Option<ShaderModule>,
    pub entry_point: Option<String>,
    pub constants: Option<Vec<ConstantEntry>>,
}
impl Default for ComputeState {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            module: None,
            entry_point: None,
            constants: None,
        }
    }
}
impl ComputeState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: ComputeStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUComputeState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUComputeState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.module = self
            .module
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.entry_point {
            raw.entryPoint = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.entryPoint = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.constantCount = self.constants.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.constants {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUConstantEntry> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.constants = ptr;
            raw.constantCount = len_value;
        } else {
            raw.constants = std::ptr::null();
            raw.constantCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: ComputeStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUComputeState) -> Self {
        Self {
            extensions: Vec::new(),
            module: Some(unsafe { ShaderModule::from_raw(value.module) }),
            entry_point: if value.entryPoint.data.is_null()
                || value.entryPoint.length == 0
            {
                None
            } else {
                Some(string_view_to_string(value.entryPoint))
            },
            constants: if value.constants.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.constants,
                            value.constantCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| ConstantEntry::from_ffi(*raw))
                        .collect(),
                )
            },
        }
    }
}
pub struct ConstantEntry {
    pub(crate) extensions: Vec<ConstantEntryExtension>,
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
    pub fn push_extension(&mut self, ext: ConstantEntryExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUConstantEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUConstantEntry = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.key {
            raw.key = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.key = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.value {
            raw.value = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: ConstantEntryExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<CopyTextureForBrowserOptionsExtension>,
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
    pub fn push_extension(&mut self, ext: CopyTextureForBrowserOptionsExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUCopyTextureForBrowserOptions, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUCopyTextureForBrowserOptions = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        raw.flipY = if self.flip_y.unwrap_or(false) { 1 } else { 0 };
        raw.needsColorSpaceConversion = if self
            .needs_color_space_conversion
            .unwrap_or(false)
        {
            1
        } else {
            0
        };
        if let Some(value) = self.src_alpha_mode {
            raw.srcAlphaMode = value.into();
        } else {
            raw.srcAlphaMode = 0 as ffi::WGPUAlphaMode;
        }
        if let Some(values) = &self.src_transfer_function_parameters {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.srcTransferFunctionParameters = ptr;
        } else {
            raw.srcTransferFunctionParameters = std::ptr::null();
            let _ = 0;
        }
        if let Some(values) = &self.conversion_matrix {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.conversionMatrix = ptr;
        } else {
            raw.conversionMatrix = std::ptr::null();
            let _ = 0;
        }
        if let Some(values) = &self.dst_transfer_function_parameters {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.dstTransferFunctionParameters = ptr;
        } else {
            raw.dstTransferFunctionParameters = std::ptr::null();
            let _ = 0;
        }
        if let Some(value) = self.dst_alpha_mode {
            raw.dstAlphaMode = value.into();
        } else {
            raw.dstAlphaMode = 0 as ffi::WGPUAlphaMode;
        }
        raw.internalUsage = if self.internal_usage.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: CopyTextureForBrowserOptionsExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUCopyTextureForBrowserOptions) -> Self {
        Self {
            extensions: Vec::new(),
            flip_y: Some(value.flipY != 0),
            needs_color_space_conversion: Some(value.needsColorSpaceConversion != 0),
            src_alpha_mode: Some(value.srcAlphaMode.into()),
            src_transfer_function_parameters: if value
                .srcTransferFunctionParameters
                .is_null()
            {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.srcTransferFunctionParameters,
                            7usize,
                        )
                    }
                        .to_vec(),
                )
            },
            conversion_matrix: if value.conversionMatrix.is_null() {
                None
            } else {
                Some(
                    unsafe { std::slice::from_raw_parts(value.conversionMatrix, 9usize) }
                        .to_vec(),
                )
            },
            dst_transfer_function_parameters: if value
                .dstTransferFunctionParameters
                .is_null()
            {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.dstTransferFunctionParameters,
                            7usize,
                        )
                    }
                        .to_vec(),
                )
            },
            dst_alpha_mode: Some(value.dstAlphaMode.into()),
            internal_usage: Some(value.internalUsage != 0),
        }
    }
}
pub struct DawnWGSLBlocklist {
    pub blocklisted_features: Option<Vec<String>>,
}
impl Default for DawnWGSLBlocklist {
    fn default() -> Self {
        Self { blocklisted_features: None }
    }
}
impl DawnWGSLBlocklist {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUDawnWGSLBlocklist, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnWGSLBlocklist = unsafe { std::mem::zeroed() };
        raw.blocklistedFeatureCount = self
            .blocklisted_features
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.blocklisted_features {
            let len_value = values.len();
            let mut c_strings: Vec<std::ffi::CString> = Vec::with_capacity(values.len());
            let mut ptrs: Vec<*const std::os::raw::c_char> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let c_string = std::ffi::CString::new(item.as_str())
                    .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
                ptrs.push(c_string.as_ptr());
                c_strings.push(c_string);
            }
            let ptr = storage.push_vec(ptrs);
            storage.push_any(c_strings);
            raw.blocklistedFeatures = ptr;
            raw.blocklistedFeatureCount = len_value;
        } else {
            raw.blocklistedFeatures = std::ptr::null();
            raw.blocklistedFeatureCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDawnWGSLBlocklist) -> Self {
        Self {
            blocklisted_features: if value.blocklistedFeatures.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.blocklistedFeatures,
                            value.blocklistedFeatureCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| {
                            if raw.is_null() {
                                String::new()
                            } else {
                                unsafe { CStr::from_ptr(*raw) }
                                    .to_string_lossy()
                                    .into_owned()
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
        Self { power_preference: None }
    }
}
impl DawnAdapterPropertiesPowerPreference {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnAdapterPropertiesPowerPreference, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnAdapterPropertiesPowerPreference = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.power_preference {
            raw.powerPreference = value.into();
        } else {
            raw.powerPreference = 0 as ffi::WGPUPowerPreference;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUDawnAdapterPropertiesPowerPreference,
    ) -> Self {
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
        Self { out_of_memory: None }
    }
}
impl DawnBufferDescriptorErrorInfoFromWireClient {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnBufferDescriptorErrorInfoFromWireClient, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnBufferDescriptorErrorInfoFromWireClient = unsafe {
            std::mem::zeroed()
        };
        raw.outOfMemory = if self.out_of_memory.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUDawnBufferDescriptorErrorInfoFromWireClient,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnCacheDeviceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnCacheDeviceDescriptor = unsafe { std::mem::zeroed() };
        if let Some(value) = &self.isolation_key {
            raw.isolationKey = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.isolationKey = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.load_data_function {
            raw.loadDataFunction = value;
        }
        if let Some(value) = self.store_data_function {
            raw.storeDataFunction = value;
        }
        if let Some(value) = self.function_userdata {
            raw.functionUserdata = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnCompilationMessageUtf16, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnCompilationMessageUtf16 = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.line_pos {
            raw.linePos = value;
        }
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.length {
            raw.length = value;
        }
        (raw, storage)
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
        Self { consume_adapter: None }
    }
}
impl DawnConsumeAdapterDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnConsumeAdapterDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnConsumeAdapterDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.consumeAdapter = if self.consume_adapter.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnDeviceAllocatorControl, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnDeviceAllocatorControl = unsafe { std::mem::zeroed() };
        if let Some(value) = self.allocator_heap_block_size {
            raw.allocatorHeapBlockSize = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDawnDeviceAllocatorControl) -> Self {
        Self {
            allocator_heap_block_size: Some(value.allocatorHeapBlockSize),
        }
    }
}
pub struct DawnDrmFormatCapabilities {
    pub properties: Option<Vec<DawnDrmFormatProperties>>,
}
impl Default for DawnDrmFormatCapabilities {
    fn default() -> Self {
        Self { properties: None }
    }
}
impl DawnDrmFormatCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnDrmFormatCapabilities, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnDrmFormatCapabilities = unsafe { std::mem::zeroed() };
        raw.propertiesCount = self.properties.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.properties {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUDawnDrmFormatProperties> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.properties = ptr;
            raw.propertiesCount = len_value;
        } else {
            raw.properties = std::ptr::null();
            raw.propertiesCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDawnDrmFormatCapabilities) -> Self {
        Self {
            properties: if value.properties.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.properties,
                            value.propertiesCount as usize,
                        )
                    }
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnDrmFormatProperties, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnDrmFormatProperties = unsafe { std::mem::zeroed() };
        if let Some(value) = self.modifier {
            raw.modifier = value;
        }
        if let Some(value) = self.modifier_plane_count {
            raw.modifierPlaneCount = value;
        }
        (raw, storage)
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
        Self { use_internal_usages: None }
    }
}
impl DawnEncoderInternalUsageDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnEncoderInternalUsageDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnEncoderInternalUsageDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.useInternalUsages = if self.use_internal_usages.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnFakeBufferOOMForTesting, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnFakeBufferOOMForTesting = unsafe {
            std::mem::zeroed()
        };
        raw.fakeOOMAtWireClientMap = if self.fake_oom_at_wire_client_map.unwrap_or(false)
        {
            1
        } else {
            0
        };
        raw.fakeOOMAtNativeMap = if self.fake_oom_at_native_map.unwrap_or(false) {
            1
        } else {
            0
        };
        raw.fakeOOMAtDevice = if self.fake_oom_at_device.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDawnFakeBufferOOMForTesting) -> Self {
        Self {
            fake_oom_at_wire_client_map: Some(value.fakeOOMAtWireClientMap != 0),
            fake_oom_at_native_map: Some(value.fakeOOMAtNativeMap != 0),
            fake_oom_at_device: Some(value.fakeOOMAtDevice != 0),
        }
    }
}
pub struct DawnFakeDeviceInitializeErrorForTesting {}
impl Default for DawnFakeDeviceInitializeErrorForTesting {
    fn default() -> Self {
        Self {}
    }
}
impl DawnFakeDeviceInitializeErrorForTesting {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnFakeDeviceInitializeErrorForTesting, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnFakeDeviceInitializeErrorForTesting = unsafe {
            std::mem::zeroed()
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUDawnFakeDeviceInitializeErrorForTesting,
    ) -> Self {
        let _ = value;
        Self::default()
    }
}
pub struct DawnFormatCapabilities {
    pub(crate) extensions: Vec<DawnFormatCapabilitiesExtension>,
}
impl Default for DawnFormatCapabilities {
    fn default() -> Self {
        Self { extensions: Vec::new() }
    }
}
impl DawnFormatCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: DawnFormatCapabilitiesExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnFormatCapabilities, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUDawnFormatCapabilities = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: DawnFormatCapabilitiesExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDawnFormatCapabilities) -> Self {
        Self { extensions: Vec::new() }
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnHostMappedPointerLimits, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnHostMappedPointerLimits = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.host_mapped_pointer_alignment {
            raw.hostMappedPointerAlignment = value;
        }
        (raw, storage)
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
        Self { invalid_s_type: None }
    }
}
impl DawnInjectedInvalidSType {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnInjectedInvalidSType, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnInjectedInvalidSType = unsafe { std::mem::zeroed() };
        if let Some(value) = self.invalid_s_type {
            raw.invalidSType = value.into();
        } else {
            raw.invalidSType = 0 as ffi::WGPUSType;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (
        ffi::WGPUDawnRenderPassColorAttachmentRenderToSingleSampled,
        ChainedStructStorage,
    ) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnRenderPassColorAttachmentRenderToSingleSampled = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.implicit_sample_count {
            raw.implicitSampleCount = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUDawnRenderPassColorAttachmentRenderToSingleSampled,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnShaderModuleSPIRVOptionsDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnShaderModuleSPIRVOptionsDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.allowNonUniformDerivatives = if self
            .allow_non_uniform_derivatives
            .unwrap_or(false)
        {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUDawnShaderModuleSPIRVOptionsDescriptor,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnTexelCopyBufferRowAlignmentLimits, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnTexelCopyBufferRowAlignmentLimits = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.min_texel_copy_buffer_row_alignment {
            raw.minTexelCopyBufferRowAlignment = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUDawnTexelCopyBufferRowAlignmentLimits,
    ) -> Self {
        Self {
            min_texel_copy_buffer_row_alignment: Some(
                value.minTexelCopyBufferRowAlignment,
            ),
        }
    }
}
pub struct DawnTextureInternalUsageDescriptor {
    pub internal_usage: Option<TextureUsage>,
}
impl Default for DawnTextureInternalUsageDescriptor {
    fn default() -> Self {
        Self { internal_usage: None }
    }
}
impl DawnTextureInternalUsageDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnTextureInternalUsageDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnTextureInternalUsageDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.internal_usage {
            raw.internalUsage = value.into();
        } else {
            raw.internalUsage = 0 as ffi::WGPUTextureUsage;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDawnTextureInternalUsageDescriptor) -> Self {
        Self {
            internal_usage: Some(value.internalUsage.into()),
        }
    }
}
pub struct DawnTogglesDescriptor {
    pub enabled_toggles: Option<Vec<String>>,
    pub disabled_toggles: Option<Vec<String>>,
}
impl Default for DawnTogglesDescriptor {
    fn default() -> Self {
        Self {
            enabled_toggles: None,
            disabled_toggles: None,
        }
    }
}
impl DawnTogglesDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUDawnTogglesDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnTogglesDescriptor = unsafe { std::mem::zeroed() };
        raw.enabledToggleCount = self
            .enabled_toggles
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.enabled_toggles {
            let len_value = values.len();
            let mut c_strings: Vec<std::ffi::CString> = Vec::with_capacity(values.len());
            let mut ptrs: Vec<*const std::os::raw::c_char> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let c_string = std::ffi::CString::new(item.as_str())
                    .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
                ptrs.push(c_string.as_ptr());
                c_strings.push(c_string);
            }
            let ptr = storage.push_vec(ptrs);
            storage.push_any(c_strings);
            raw.enabledToggles = ptr;
            raw.enabledToggleCount = len_value;
        } else {
            raw.enabledToggles = std::ptr::null();
            raw.enabledToggleCount = 0;
        }
        raw.disabledToggleCount = self
            .disabled_toggles
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.disabled_toggles {
            let len_value = values.len();
            let mut c_strings: Vec<std::ffi::CString> = Vec::with_capacity(values.len());
            let mut ptrs: Vec<*const std::os::raw::c_char> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let c_string = std::ffi::CString::new(item.as_str())
                    .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
                ptrs.push(c_string.as_ptr());
                c_strings.push(c_string);
            }
            let ptr = storage.push_vec(ptrs);
            storage.push_any(c_strings);
            raw.disabledToggles = ptr;
            raw.disabledToggleCount = len_value;
        } else {
            raw.disabledToggles = std::ptr::null();
            raw.disabledToggleCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDawnTogglesDescriptor) -> Self {
        Self {
            enabled_toggles: if value.enabledToggles.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.enabledToggles,
                            value.enabledToggleCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| {
                            if raw.is_null() {
                                String::new()
                            } else {
                                unsafe { CStr::from_ptr(*raw) }
                                    .to_string_lossy()
                                    .into_owned()
                            }
                        })
                        .collect(),
                )
            },
            disabled_toggles: if value.disabledToggles.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.disabledToggles,
                            value.disabledToggleCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| {
                            if raw.is_null() {
                                String::new()
                            } else {
                                unsafe { CStr::from_ptr(*raw) }
                                    .to_string_lossy()
                                    .into_owned()
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUDawnWireWGSLControl, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUDawnWireWGSLControl = unsafe { std::mem::zeroed() };
        raw.enableExperimental = if self.enable_experimental.unwrap_or(false) {
            1
        } else {
            0
        };
        raw.enableUnsafe = if self.enable_unsafe.unwrap_or(false) { 1 } else { 0 };
        raw.enableTesting = if self.enable_testing.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
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
    pub(crate) extensions: Vec<DepthStencilStateExtension>,
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
    pub fn push_extension(&mut self, ext: DepthStencilStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUDepthStencilState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUDepthStencilState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = self.depth_write_enabled {
            raw.depthWriteEnabled = value.into();
        } else {
            raw.depthWriteEnabled = 0 as ffi::WGPUOptionalBool;
        }
        if let Some(value) = self.depth_compare {
            raw.depthCompare = value.into();
        } else {
            raw.depthCompare = 0 as ffi::WGPUCompareFunction;
        }
        if let Some(value) = &self.stencil_front {
            let (raw_value, storage_value) = value.to_ffi();
            raw.stencilFront = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.stencil_back {
            let (raw_value, storage_value) = value.to_ffi();
            raw.stencilBack = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = self.stencil_read_mask {
            raw.stencilReadMask = value;
        }
        if let Some(value) = self.stencil_write_mask {
            raw.stencilWriteMask = value;
        }
        if let Some(value) = self.depth_bias {
            raw.depthBias = value;
        }
        if let Some(value) = self.depth_bias_slope_scale {
            raw.depthBiasSlopeScale = value;
        }
        if let Some(value) = self.depth_bias_clamp {
            raw.depthBiasClamp = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: DepthStencilStateExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<DeviceDescriptorExtension>,
    pub label: Option<String>,
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
    pub fn push_extension(&mut self, ext: DeviceDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUDeviceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUDeviceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.requiredFeatureCount = self
            .required_features
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.required_features {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUFeatureName> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.requiredFeatures = ptr;
            raw.requiredFeatureCount = len_value;
        } else {
            raw.requiredFeatures = std::ptr::null();
            raw.requiredFeatureCount = 0;
        }
        if let Some(value) = &self.required_limits {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.requiredLimits = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.requiredLimits = std::ptr::null();
        }
        if let Some(value) = &self.default_queue {
            let (raw_value, storage_value) = value.to_ffi();
            raw.defaultQueue = raw_value;
            storage.push_storage(storage_value);
        }
        let _ = &self.device_lost_callback_info;
        let _ = &self.uncaptured_error_callback_info;
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: DeviceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUDeviceDescriptor) -> Self {
        Self {
            extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                None
            } else {
                Some(string_view_to_string(value.label))
            },
            required_features: if value.requiredFeatures.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.requiredFeatures,
                            value.requiredFeatureCount as usize,
                        )
                    }
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
        Self { width: None, height: None }
    }
}
impl Extent2D {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUExtent2D, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUExtent2D = unsafe { std::mem::zeroed() };
        if let Some(value) = self.width {
            raw.width = value;
        }
        if let Some(value) = self.height {
            raw.height = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUExtent3D, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUExtent3D = unsafe { std::mem::zeroed() };
        if let Some(value) = self.width {
            raw.width = value;
        }
        if let Some(value) = self.height {
            raw.height = value;
        }
        if let Some(value) = self.depth_or_array_layers {
            raw.depthOrArrayLayers = value;
        }
        (raw, storage)
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
        Self { external_texture: None }
    }
}
impl ExternalTextureBindingEntry {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUExternalTextureBindingEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUExternalTextureBindingEntry = unsafe {
            std::mem::zeroed()
        };
        raw.externalTexture = self
            .external_texture
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUExternalTextureBindingEntry) -> Self {
        Self {
            external_texture: Some(unsafe {
                ExternalTexture::from_raw(value.externalTexture)
            }),
        }
    }
}
pub struct ExternalTextureBindingLayout {}
impl Default for ExternalTextureBindingLayout {
    fn default() -> Self {
        Self {}
    }
}
impl ExternalTextureBindingLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUExternalTextureBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUExternalTextureBindingLayout = unsafe {
            std::mem::zeroed()
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUExternalTextureBindingLayout) -> Self {
        let _ = value;
        Self::default()
    }
}
pub struct ExternalTextureDescriptor {
    pub(crate) extensions: Vec<ExternalTextureDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: ExternalTextureDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUExternalTextureDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUExternalTextureDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.plane0 = self
            .plane_0
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        raw.plane1 = self
            .plane_1
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.crop_origin {
            let (raw_value, storage_value) = value.to_ffi();
            raw.cropOrigin = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.crop_size {
            let (raw_value, storage_value) = value.to_ffi();
            raw.cropSize = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.apparent_size {
            let (raw_value, storage_value) = value.to_ffi();
            raw.apparentSize = raw_value;
            storage.push_storage(storage_value);
        }
        raw.doYuvToRgbConversionOnly = if self
            .do_yuv_to_rgb_conversion_only
            .unwrap_or(false)
        {
            1
        } else {
            0
        };
        if let Some(values) = &self.yuv_to_rgb_conversion_matrix {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.yuvToRgbConversionMatrix = ptr;
        } else {
            raw.yuvToRgbConversionMatrix = std::ptr::null();
            let _ = 0;
        }
        if let Some(values) = &self.src_transfer_function_parameters {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.srcTransferFunctionParameters = ptr;
        } else {
            raw.srcTransferFunctionParameters = std::ptr::null();
            let _ = 0;
        }
        if let Some(values) = &self.dst_transfer_function_parameters {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.dstTransferFunctionParameters = ptr;
        } else {
            raw.dstTransferFunctionParameters = std::ptr::null();
            let _ = 0;
        }
        if let Some(values) = &self.gamut_conversion_matrix {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.gamutConversionMatrix = ptr;
        } else {
            raw.gamutConversionMatrix = std::ptr::null();
            let _ = 0;
        }
        raw.mirrored = if self.mirrored.unwrap_or(false) { 1 } else { 0 };
        if let Some(value) = self.rotation {
            raw.rotation = value.into();
        } else {
            raw.rotation = 0 as ffi::WGPUExternalTextureRotation;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: ExternalTextureDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.yuvToRgbConversionMatrix,
                            12usize,
                        )
                    }
                        .to_vec(),
                )
            },
            src_transfer_function_parameters: if value
                .srcTransferFunctionParameters
                .is_null()
            {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.srcTransferFunctionParameters,
                            7usize,
                        )
                    }
                        .to_vec(),
                )
            },
            dst_transfer_function_parameters: if value
                .dstTransferFunctionParameters
                .is_null()
            {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.dstTransferFunctionParameters,
                            7usize,
                        )
                    }
                        .to_vec(),
                )
            },
            gamut_conversion_matrix: if value.gamutConversionMatrix.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(value.gamutConversionMatrix, 9usize)
                    }
                        .to_vec(),
                )
            },
            mirrored: Some(value.mirrored != 0),
            rotation: Some(value.rotation.into()),
        }
    }
}
pub struct FragmentState {
    pub(crate) extensions: Vec<FragmentStateExtension>,
    pub module: Option<ShaderModule>,
    pub entry_point: Option<String>,
    pub constants: Option<Vec<ConstantEntry>>,
    pub targets: Option<Vec<ColorTargetState>>,
}
impl Default for FragmentState {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            module: None,
            entry_point: None,
            constants: None,
            targets: None,
        }
    }
}
impl FragmentState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: FragmentStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUFragmentState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUFragmentState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.module = self
            .module
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.entry_point {
            raw.entryPoint = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.entryPoint = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.constantCount = self.constants.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.constants {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUConstantEntry> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.constants = ptr;
            raw.constantCount = len_value;
        } else {
            raw.constants = std::ptr::null();
            raw.constantCount = 0;
        }
        raw.targetCount = self.targets.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.targets {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUColorTargetState> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.targets = ptr;
            raw.targetCount = len_value;
        } else {
            raw.targets = std::ptr::null();
            raw.targetCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: FragmentStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUFragmentState) -> Self {
        Self {
            extensions: Vec::new(),
            module: Some(unsafe { ShaderModule::from_raw(value.module) }),
            entry_point: if value.entryPoint.data.is_null()
                || value.entryPoint.length == 0
            {
                None
            } else {
                Some(string_view_to_string(value.entryPoint))
            },
            constants: if value.constants.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.constants,
                            value.constantCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| ConstantEntry::from_ffi(*raw))
                        .collect(),
                )
            },
            targets: if value.targets.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.targets,
                            value.targetCount as usize,
                        )
                    }
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
        Self { id: None }
    }
}
impl Future {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUFuture, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUFuture = unsafe { std::mem::zeroed() };
        if let Some(value) = self.id {
            raw.id = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUFuture) -> Self {
        Self { id: Some(value.id) }
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUFutureWaitInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUFutureWaitInfo = unsafe { std::mem::zeroed() };
        if let Some(value) = &self.future {
            let (raw_value, storage_value) = value.to_ffi();
            raw.future = raw_value;
            storage.push_storage(storage_value);
        }
        raw.completed = if self.completed.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUFutureWaitInfo) -> Self {
        Self {
            future: Some(Future::from_ffi(value.future)),
            completed: Some(value.completed != 0),
        }
    }
}
pub struct ImageCopyExternalTexture {
    pub(crate) extensions: Vec<ImageCopyExternalTextureExtension>,
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
    pub fn push_extension(&mut self, ext: ImageCopyExternalTextureExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUImageCopyExternalTexture, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUImageCopyExternalTexture = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.externalTexture = self
            .external_texture
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.origin {
            let (raw_value, storage_value) = value.to_ffi();
            raw.origin = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.natural_size {
            let (raw_value, storage_value) = value.to_ffi();
            raw.naturalSize = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: ImageCopyExternalTextureExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUImageCopyExternalTexture) -> Self {
        Self {
            extensions: Vec::new(),
            external_texture: Some(unsafe {
                ExternalTexture::from_raw(value.externalTexture)
            }),
            origin: Some(Origin3D::from_ffi(value.origin)),
            natural_size: Some(Extent2D::from_ffi(value.naturalSize)),
        }
    }
}
pub struct InstanceDescriptor {
    pub(crate) extensions: Vec<InstanceDescriptorExtension>,
    pub required_features: Option<Vec<InstanceFeatureName>>,
    pub required_limits: Option<InstanceLimits>,
}
impl Default for InstanceDescriptor {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            required_features: None,
            required_limits: None,
        }
    }
}
impl InstanceDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: InstanceDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUInstanceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUInstanceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.requiredFeatureCount = self
            .required_features
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.required_features {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUInstanceFeatureName> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.requiredFeatures = ptr;
            raw.requiredFeatureCount = len_value;
        } else {
            raw.requiredFeatures = std::ptr::null();
            raw.requiredFeatureCount = 0;
        }
        if let Some(value) = &self.required_limits {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.requiredLimits = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.requiredLimits = std::ptr::null();
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: InstanceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUInstanceDescriptor) -> Self {
        Self {
            extensions: Vec::new(),
            required_features: if value.requiredFeatures.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.requiredFeatures,
                            value.requiredFeatureCount as usize,
                        )
                    }
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
    pub(crate) extensions: Vec<InstanceLimitsExtension>,
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
    pub fn push_extension(&mut self, ext: InstanceLimitsExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUInstanceLimits, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUInstanceLimits = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.timed_wait_any_max_count {
            raw.timedWaitAnyMaxCount = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: InstanceLimitsExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUInstanceLimits) -> Self {
        Self {
            extensions: Vec::new(),
            timed_wait_any_max_count: Some(value.timedWaitAnyMaxCount),
        }
    }
}
pub struct Limits {
    pub(crate) extensions: Vec<LimitsExtension>,
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
    pub fn push_extension(&mut self, ext: LimitsExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPULimits, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPULimits = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.max_texture_dimension_1d {
            raw.maxTextureDimension1D = value;
        }
        if let Some(value) = self.max_texture_dimension_2d {
            raw.maxTextureDimension2D = value;
        }
        if let Some(value) = self.max_texture_dimension_3d {
            raw.maxTextureDimension3D = value;
        }
        if let Some(value) = self.max_texture_array_layers {
            raw.maxTextureArrayLayers = value;
        }
        if let Some(value) = self.max_bind_groups {
            raw.maxBindGroups = value;
        }
        if let Some(value) = self.max_bind_groups_plus_vertex_buffers {
            raw.maxBindGroupsPlusVertexBuffers = value;
        }
        if let Some(value) = self.max_bindings_per_bind_group {
            raw.maxBindingsPerBindGroup = value;
        }
        if let Some(value) = self.max_dynamic_uniform_buffers_per_pipeline_layout {
            raw.maxDynamicUniformBuffersPerPipelineLayout = value;
        }
        if let Some(value) = self.max_dynamic_storage_buffers_per_pipeline_layout {
            raw.maxDynamicStorageBuffersPerPipelineLayout = value;
        }
        if let Some(value) = self.max_sampled_textures_per_shader_stage {
            raw.maxSampledTexturesPerShaderStage = value;
        }
        if let Some(value) = self.max_samplers_per_shader_stage {
            raw.maxSamplersPerShaderStage = value;
        }
        if let Some(value) = self.max_storage_buffers_per_shader_stage {
            raw.maxStorageBuffersPerShaderStage = value;
        }
        if let Some(value) = self.max_storage_textures_per_shader_stage {
            raw.maxStorageTexturesPerShaderStage = value;
        }
        if let Some(value) = self.max_uniform_buffers_per_shader_stage {
            raw.maxUniformBuffersPerShaderStage = value;
        }
        if let Some(value) = self.max_uniform_buffer_binding_size {
            raw.maxUniformBufferBindingSize = value;
        }
        if let Some(value) = self.max_storage_buffer_binding_size {
            raw.maxStorageBufferBindingSize = value;
        }
        if let Some(value) = self.min_uniform_buffer_offset_alignment {
            raw.minUniformBufferOffsetAlignment = value;
        }
        if let Some(value) = self.min_storage_buffer_offset_alignment {
            raw.minStorageBufferOffsetAlignment = value;
        }
        if let Some(value) = self.max_vertex_buffers {
            raw.maxVertexBuffers = value;
        }
        if let Some(value) = self.max_buffer_size {
            raw.maxBufferSize = value;
        }
        if let Some(value) = self.max_vertex_attributes {
            raw.maxVertexAttributes = value;
        }
        if let Some(value) = self.max_vertex_buffer_array_stride {
            raw.maxVertexBufferArrayStride = value;
        }
        if let Some(value) = self.max_inter_stage_shader_variables {
            raw.maxInterStageShaderVariables = value;
        }
        if let Some(value) = self.max_color_attachments {
            raw.maxColorAttachments = value;
        }
        if let Some(value) = self.max_color_attachment_bytes_per_sample {
            raw.maxColorAttachmentBytesPerSample = value;
        }
        if let Some(value) = self.max_compute_workgroup_storage_size {
            raw.maxComputeWorkgroupStorageSize = value;
        }
        if let Some(value) = self.max_compute_invocations_per_workgroup {
            raw.maxComputeInvocationsPerWorkgroup = value;
        }
        if let Some(value) = self.max_compute_workgroup_size_x {
            raw.maxComputeWorkgroupSizeX = value;
        }
        if let Some(value) = self.max_compute_workgroup_size_y {
            raw.maxComputeWorkgroupSizeY = value;
        }
        if let Some(value) = self.max_compute_workgroup_size_z {
            raw.maxComputeWorkgroupSizeZ = value;
        }
        if let Some(value) = self.max_compute_workgroups_per_dimension {
            raw.maxComputeWorkgroupsPerDimension = value;
        }
        if let Some(value) = self.max_immediate_size {
            raw.maxImmediateSize = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: LimitsExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPULimits) -> Self {
        Self {
            extensions: Vec::new(),
            max_texture_dimension_1d: Some(value.maxTextureDimension1D),
            max_texture_dimension_2d: Some(value.maxTextureDimension2D),
            max_texture_dimension_3d: Some(value.maxTextureDimension3D),
            max_texture_array_layers: Some(value.maxTextureArrayLayers),
            max_bind_groups: Some(value.maxBindGroups),
            max_bind_groups_plus_vertex_buffers: Some(
                value.maxBindGroupsPlusVertexBuffers,
            ),
            max_bindings_per_bind_group: Some(value.maxBindingsPerBindGroup),
            max_dynamic_uniform_buffers_per_pipeline_layout: Some(
                value.maxDynamicUniformBuffersPerPipelineLayout,
            ),
            max_dynamic_storage_buffers_per_pipeline_layout: Some(
                value.maxDynamicStorageBuffersPerPipelineLayout,
            ),
            max_sampled_textures_per_shader_stage: Some(
                value.maxSampledTexturesPerShaderStage,
            ),
            max_samplers_per_shader_stage: Some(value.maxSamplersPerShaderStage),
            max_storage_buffers_per_shader_stage: Some(
                value.maxStorageBuffersPerShaderStage,
            ),
            max_storage_textures_per_shader_stage: Some(
                value.maxStorageTexturesPerShaderStage,
            ),
            max_uniform_buffers_per_shader_stage: Some(
                value.maxUniformBuffersPerShaderStage,
            ),
            max_uniform_buffer_binding_size: Some(value.maxUniformBufferBindingSize),
            max_storage_buffer_binding_size: Some(value.maxStorageBufferBindingSize),
            min_uniform_buffer_offset_alignment: Some(
                value.minUniformBufferOffsetAlignment,
            ),
            min_storage_buffer_offset_alignment: Some(
                value.minStorageBufferOffsetAlignment,
            ),
            max_vertex_buffers: Some(value.maxVertexBuffers),
            max_buffer_size: Some(value.maxBufferSize),
            max_vertex_attributes: Some(value.maxVertexAttributes),
            max_vertex_buffer_array_stride: Some(value.maxVertexBufferArrayStride),
            max_inter_stage_shader_variables: Some(value.maxInterStageShaderVariables),
            max_color_attachments: Some(value.maxColorAttachments),
            max_color_attachment_bytes_per_sample: Some(
                value.maxColorAttachmentBytesPerSample,
            ),
            max_compute_workgroup_storage_size: Some(
                value.maxComputeWorkgroupStorageSize,
            ),
            max_compute_invocations_per_workgroup: Some(
                value.maxComputeInvocationsPerWorkgroup,
            ),
            max_compute_workgroup_size_x: Some(value.maxComputeWorkgroupSizeX),
            max_compute_workgroup_size_y: Some(value.maxComputeWorkgroupSizeY),
            max_compute_workgroup_size_z: Some(value.maxComputeWorkgroupSizeZ),
            max_compute_workgroups_per_dimension: Some(
                value.maxComputeWorkgroupsPerDimension,
            ),
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUMemoryHeapInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUMemoryHeapInfo = unsafe { std::mem::zeroed() };
        if let Some(value) = self.properties {
            raw.properties = value.into();
        } else {
            raw.properties = 0 as ffi::WGPUHeapProperty;
        }
        if let Some(value) = self.size {
            raw.size = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUMemoryHeapInfo) -> Self {
        Self {
            properties: Some(value.properties.into()),
            size: Some(value.size),
        }
    }
}
pub struct MultisampleState {
    pub(crate) extensions: Vec<MultisampleStateExtension>,
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
    pub fn push_extension(&mut self, ext: MultisampleStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUMultisampleState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUMultisampleState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.count {
            raw.count = value;
        }
        if let Some(value) = self.mask {
            raw.mask = value;
        }
        raw.alphaToCoverageEnabled = if self.alpha_to_coverage_enabled.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: MultisampleStateExtension) -> Self {
        self.extensions.push(extension);
        self
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
        Self { x: Some(0), y: Some(0) }
    }
}
impl Origin2D {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUOrigin2D, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUOrigin2D = unsafe { std::mem::zeroed() };
        if let Some(value) = self.x {
            raw.x = value;
        }
        if let Some(value) = self.y {
            raw.y = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUOrigin3D, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUOrigin3D = unsafe { std::mem::zeroed() };
        if let Some(value) = self.x {
            raw.x = value;
        }
        if let Some(value) = self.y {
            raw.y = value;
        }
        if let Some(value) = self.z {
            raw.z = value;
        }
        (raw, storage)
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
    pub(crate) extensions: Vec<PassTimestampWritesExtension>,
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
    pub fn push_extension(&mut self, ext: PassTimestampWritesExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUPassTimestampWrites, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPassTimestampWrites = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.querySet = self
            .query_set
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.beginning_of_pass_write_index {
            raw.beginningOfPassWriteIndex = value;
        }
        if let Some(value) = self.end_of_pass_write_index {
            raw.endOfPassWriteIndex = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: PassTimestampWritesExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<PipelineLayoutDescriptorExtension>,
    pub label: Option<String>,
    pub bind_group_layouts: Option<Vec<BindGroupLayout>>,
    pub immediate_size: Option<u32>,
}
impl Default for PipelineLayoutDescriptor {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            label: None,
            bind_group_layouts: None,
            immediate_size: Some(0),
        }
    }
}
impl PipelineLayoutDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: PipelineLayoutDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUPipelineLayoutDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPipelineLayoutDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.bindGroupLayoutCount = self
            .bind_group_layouts
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.bind_group_layouts {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUBindGroupLayout> = values
                .iter()
                .map(|v| v.as_raw())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.bindGroupLayouts = ptr;
            raw.bindGroupLayoutCount = len_value;
        } else {
            raw.bindGroupLayouts = std::ptr::null();
            raw.bindGroupLayoutCount = 0;
        }
        if let Some(value) = self.immediate_size {
            raw.immediateSize = value;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: PipelineLayoutDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUPipelineLayoutDescriptor) -> Self {
        Self {
            extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                None
            } else {
                Some(string_view_to_string(value.label))
            },
            bind_group_layouts: if value.bindGroupLayouts.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.bindGroupLayouts,
                            value.bindGroupLayoutCount as usize,
                        )
                    }
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
    pub storage_attachments: Option<Vec<PipelineLayoutStorageAttachment>>,
}
impl Default for PipelineLayoutPixelLocalStorage {
    fn default() -> Self {
        Self {
            total_pixel_local_storage_size: None,
            storage_attachments: None,
        }
    }
}
impl PipelineLayoutPixelLocalStorage {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUPipelineLayoutPixelLocalStorage, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUPipelineLayoutPixelLocalStorage = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.total_pixel_local_storage_size {
            raw.totalPixelLocalStorageSize = value;
        }
        raw.storageAttachmentCount = self
            .storage_attachments
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.storage_attachments {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUPipelineLayoutStorageAttachment> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.storageAttachments = ptr;
            raw.storageAttachmentCount = len_value;
        } else {
            raw.storageAttachments = std::ptr::null();
            raw.storageAttachmentCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUPipelineLayoutPixelLocalStorage) -> Self {
        Self {
            total_pixel_local_storage_size: Some(value.totalPixelLocalStorageSize),
            storage_attachments: if value.storageAttachments.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.storageAttachments,
                            value.storageAttachmentCount as usize,
                        )
                    }
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
        Self { uses_resource_table: None }
    }
}
impl PipelineLayoutResourceTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUPipelineLayoutResourceTable, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUPipelineLayoutResourceTable = unsafe {
            std::mem::zeroed()
        };
        raw.usesResourceTable = if self.uses_resource_table.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUPipelineLayoutResourceTable) -> Self {
        Self {
            uses_resource_table: Some(value.usesResourceTable != 0),
        }
    }
}
pub struct PipelineLayoutStorageAttachment {
    pub(crate) extensions: Vec<PipelineLayoutStorageAttachmentExtension>,
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
    pub fn push_extension(&mut self, ext: PipelineLayoutStorageAttachmentExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUPipelineLayoutStorageAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPipelineLayoutStorageAttachment = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: PipelineLayoutStorageAttachmentExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<PrimitiveStateExtension>,
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
    pub fn push_extension(&mut self, ext: PrimitiveStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUPrimitiveState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUPrimitiveState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.topology {
            raw.topology = value.into();
        } else {
            raw.topology = 0 as ffi::WGPUPrimitiveTopology;
        }
        if let Some(value) = self.strip_index_format {
            raw.stripIndexFormat = value.into();
        } else {
            raw.stripIndexFormat = 0 as ffi::WGPUIndexFormat;
        }
        if let Some(value) = self.front_face {
            raw.frontFace = value.into();
        } else {
            raw.frontFace = 0 as ffi::WGPUFrontFace;
        }
        if let Some(value) = self.cull_mode {
            raw.cullMode = value.into();
        } else {
            raw.cullMode = 0 as ffi::WGPUCullMode;
        }
        raw.unclippedDepth = if self.unclipped_depth.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: PrimitiveStateExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<QuerySetDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: QuerySetDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUQuerySetDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUQuerySetDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.r#type {
            raw.type_ = value.into();
        } else {
            raw.type_ = 0 as ffi::WGPUQueryType;
        }
        if let Some(value) = self.count {
            raw.count = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: QuerySetDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<QueueDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: QueueDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUQueueDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUQueueDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: QueueDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<RenderBundleDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: RenderBundleDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderBundleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderBundleDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: RenderBundleDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<RenderBundleEncoderDescriptorExtension>,
    pub label: Option<String>,
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
    pub fn push_extension(&mut self, ext: RenderBundleEncoderDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderBundleEncoderDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderBundleEncoderDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.colorFormatCount = self.color_formats.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.color_formats {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUTextureFormat> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.colorFormats = ptr;
            raw.colorFormatCount = len_value;
        } else {
            raw.colorFormats = std::ptr::null();
            raw.colorFormatCount = 0;
        }
        if let Some(value) = self.depth_stencil_format {
            raw.depthStencilFormat = value.into();
        } else {
            raw.depthStencilFormat = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = self.sample_count {
            raw.sampleCount = value;
        }
        raw.depthReadOnly = if self.depth_read_only.unwrap_or(false) { 1 } else { 0 };
        raw.stencilReadOnly = if self.stencil_read_only.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: RenderBundleEncoderDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPURenderBundleEncoderDescriptor) -> Self {
        Self {
            extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                None
            } else {
                Some(string_view_to_string(value.label))
            },
            color_formats: if value.colorFormats.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.colorFormats,
                            value.colorFormatCount as usize,
                        )
                    }
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
    pub(crate) extensions: Vec<RenderPassColorAttachmentExtension>,
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
    pub fn push_extension(&mut self, ext: RenderPassColorAttachmentExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassColorAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassColorAttachment = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.view = self
            .view
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.depth_slice {
            raw.depthSlice = value;
        }
        raw.resolveTarget = self
            .resolve_target
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.load_op {
            raw.loadOp = value.into();
        } else {
            raw.loadOp = 0 as ffi::WGPULoadOp;
        }
        if let Some(value) = self.store_op {
            raw.storeOp = value.into();
        } else {
            raw.storeOp = 0 as ffi::WGPUStoreOp;
        }
        if let Some(value) = &self.clear_value {
            let (raw_value, storage_value) = value.to_ffi();
            raw.clearValue = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: RenderPassColorAttachmentExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<RenderPassDepthStencilAttachmentExtension>,
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
    pub fn push_extension(&mut self, ext: RenderPassDepthStencilAttachmentExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassDepthStencilAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassDepthStencilAttachment = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        raw.view = self
            .view
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.depth_load_op {
            raw.depthLoadOp = value.into();
        } else {
            raw.depthLoadOp = 0 as ffi::WGPULoadOp;
        }
        if let Some(value) = self.depth_store_op {
            raw.depthStoreOp = value.into();
        } else {
            raw.depthStoreOp = 0 as ffi::WGPUStoreOp;
        }
        if let Some(value) = self.depth_clear_value {
            raw.depthClearValue = value;
        }
        raw.depthReadOnly = if self.depth_read_only.unwrap_or(false) { 1 } else { 0 };
        if let Some(value) = self.stencil_load_op {
            raw.stencilLoadOp = value.into();
        } else {
            raw.stencilLoadOp = 0 as ffi::WGPULoadOp;
        }
        if let Some(value) = self.stencil_store_op {
            raw.stencilStoreOp = value.into();
        } else {
            raw.stencilStoreOp = 0 as ffi::WGPUStoreOp;
        }
        if let Some(value) = self.stencil_clear_value {
            raw.stencilClearValue = value;
        }
        raw.stencilReadOnly = if self.stencil_read_only.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: RenderPassDepthStencilAttachmentExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<RenderPassDescriptorExtension>,
    pub label: Option<String>,
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
    pub fn push_extension(&mut self, ext: RenderPassDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.colorAttachmentCount = self
            .color_attachments
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.color_attachments {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPURenderPassColorAttachment> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.colorAttachments = ptr;
            raw.colorAttachmentCount = len_value;
        } else {
            raw.colorAttachments = std::ptr::null();
            raw.colorAttachmentCount = 0;
        }
        if let Some(value) = &self.depth_stencil_attachment {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.depthStencilAttachment = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.depthStencilAttachment = std::ptr::null();
        }
        raw.occlusionQuerySet = self
            .occlusion_query_set
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.timestamp_writes {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.timestampWrites = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.timestampWrites = std::ptr::null();
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: RenderPassDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPURenderPassDescriptor) -> Self {
        Self {
            extensions: Vec::new(),
            label: if value.label.data.is_null() || value.label.length == 0 {
                None
            } else {
                Some(string_view_to_string(value.label))
            },
            color_attachments: if value.colorAttachments.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.colorAttachments,
                            value.colorAttachmentCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| RenderPassColorAttachment::from_ffi(*raw))
                        .collect(),
                )
            },
            depth_stencil_attachment: if value.depthStencilAttachment.is_null() {
                None
            } else {
                Some(
                    RenderPassDepthStencilAttachment::from_ffi(unsafe {
                        *value.depthStencilAttachment
                    }),
                )
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassDescriptorExpandResolveRect, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPURenderPassDescriptorExpandResolveRect = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.x {
            raw.x = value;
        }
        if let Some(value) = self.y {
            raw.y = value;
        }
        if let Some(value) = self.width {
            raw.width = value;
        }
        if let Some(value) = self.height {
            raw.height = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPURenderPassDescriptorExpandResolveRect,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassDescriptorResolveRect, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPURenderPassDescriptorResolveRect = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.color_offset_x {
            raw.colorOffsetX = value;
        }
        if let Some(value) = self.color_offset_y {
            raw.colorOffsetY = value;
        }
        if let Some(value) = self.resolve_offset_x {
            raw.resolveOffsetX = value;
        }
        if let Some(value) = self.resolve_offset_y {
            raw.resolveOffsetY = value;
        }
        if let Some(value) = self.width {
            raw.width = value;
        }
        if let Some(value) = self.height {
            raw.height = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassMaxDrawCount, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPURenderPassMaxDrawCount = unsafe { std::mem::zeroed() };
        if let Some(value) = self.max_draw_count {
            raw.maxDrawCount = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPURenderPassMaxDrawCount) -> Self {
        Self {
            max_draw_count: Some(value.maxDrawCount),
        }
    }
}
pub struct RenderPassPixelLocalStorage {
    pub total_pixel_local_storage_size: Option<u64>,
    pub storage_attachments: Option<Vec<RenderPassStorageAttachment>>,
}
impl Default for RenderPassPixelLocalStorage {
    fn default() -> Self {
        Self {
            total_pixel_local_storage_size: None,
            storage_attachments: None,
        }
    }
}
impl RenderPassPixelLocalStorage {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassPixelLocalStorage, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPURenderPassPixelLocalStorage = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.total_pixel_local_storage_size {
            raw.totalPixelLocalStorageSize = value;
        }
        raw.storageAttachmentCount = self
            .storage_attachments
            .as_ref()
            .map(|v| v.len())
            .unwrap_or(0);
        if let Some(values) = &self.storage_attachments {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPURenderPassStorageAttachment> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.storageAttachments = ptr;
            raw.storageAttachmentCount = len_value;
        } else {
            raw.storageAttachments = std::ptr::null();
            raw.storageAttachmentCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPURenderPassPixelLocalStorage) -> Self {
        Self {
            total_pixel_local_storage_size: Some(value.totalPixelLocalStorageSize),
            storage_attachments: if value.storageAttachments.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.storageAttachments,
                            value.storageAttachmentCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| RenderPassStorageAttachment::from_ffi(*raw))
                        .collect(),
                )
            },
        }
    }
}
pub struct RenderPassStorageAttachment {
    pub(crate) extensions: Vec<RenderPassStorageAttachmentExtension>,
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
    pub fn push_extension(&mut self, ext: RenderPassStorageAttachmentExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPassStorageAttachment, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPassStorageAttachment = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        raw.storage = self
            .storage
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.load_op {
            raw.loadOp = value.into();
        } else {
            raw.loadOp = 0 as ffi::WGPULoadOp;
        }
        if let Some(value) = self.store_op {
            raw.storeOp = value.into();
        } else {
            raw.storeOp = 0 as ffi::WGPUStoreOp;
        }
        if let Some(value) = &self.clear_value {
            let (raw_value, storage_value) = value.to_ffi();
            raw.clearValue = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: RenderPassStorageAttachmentExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<RenderPipelineDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: RenderPipelineDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURenderPipelineDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURenderPipelineDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.layout = self
            .layout
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.vertex {
            let (raw_value, storage_value) = value.to_ffi();
            raw.vertex = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.primitive {
            let (raw_value, storage_value) = value.to_ffi();
            raw.primitive = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.depth_stencil {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.depthStencil = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.depthStencil = std::ptr::null();
        }
        if let Some(value) = &self.multisample {
            let (raw_value, storage_value) = value.to_ffi();
            raw.multisample = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = &self.fragment {
            let (raw_value, storage_value) = value.to_ffi();
            let ptr = storage.push_value(raw_value);
            raw.fragment = ptr;
            storage.push_storage(storage_value);
        } else {
            raw.fragment = std::ptr::null();
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: RenderPipelineDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
pub struct RequestAdapterWebGPUBackendOptions {}
impl Default for RequestAdapterWebGPUBackendOptions {
    fn default() -> Self {
        Self {}
    }
}
impl RequestAdapterWebGPUBackendOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURequestAdapterWebGPUBackendOptions, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPURequestAdapterWebGPUBackendOptions = unsafe {
            std::mem::zeroed()
        };
        (raw, storage)
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
        Self { xr_compatible: None }
    }
}
impl RequestAdapterWebXROptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURequestAdapterWebXROptions, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPURequestAdapterWebXROptions = unsafe { std::mem::zeroed() };
        raw.xrCompatible = if self.xr_compatible.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPURequestAdapterWebXROptions) -> Self {
        Self {
            xr_compatible: Some(value.xrCompatible != 0),
        }
    }
}
pub struct RequestAdapterOptions {
    pub(crate) extensions: Vec<RequestAdapterOptionsExtension>,
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
    pub fn push_extension(&mut self, ext: RequestAdapterOptionsExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPURequestAdapterOptions, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPURequestAdapterOptions = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.feature_level {
            raw.featureLevel = value.into();
        } else {
            raw.featureLevel = 0 as ffi::WGPUFeatureLevel;
        }
        if let Some(value) = self.power_preference {
            raw.powerPreference = value.into();
        } else {
            raw.powerPreference = 0 as ffi::WGPUPowerPreference;
        }
        raw.forceFallbackAdapter = if self.force_fallback_adapter.unwrap_or(false) {
            1
        } else {
            0
        };
        if let Some(value) = self.backend_type {
            raw.backendType = value.into();
        } else {
            raw.backendType = 0 as ffi::WGPUBackendType;
        }
        raw.compatibleSurface = self
            .compatible_surface
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: RequestAdapterOptionsExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<ResourceTableDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: ResourceTableDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUResourceTableDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUResourceTableDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.size {
            raw.size = value;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: ResourceTableDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUResourceTableLimits, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUResourceTableLimits = unsafe { std::mem::zeroed() };
        if let Some(value) = self.max_resource_table_size {
            raw.maxResourceTableSize = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUResourceTableLimits) -> Self {
        Self {
            max_resource_table_size: Some(value.maxResourceTableSize),
        }
    }
}
pub struct SamplerBindingLayout {
    pub(crate) extensions: Vec<SamplerBindingLayoutExtension>,
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
    pub fn push_extension(&mut self, ext: SamplerBindingLayoutExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSamplerBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSamplerBindingLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.r#type {
            raw.type_ = value.into();
        } else {
            raw.type_ = 0 as ffi::WGPUSamplerBindingType;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SamplerBindingLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSamplerBindingLayout) -> Self {
        Self {
            extensions: Vec::new(),
            r#type: Some(value.type_.into()),
        }
    }
}
pub struct SamplerDescriptor {
    pub(crate) extensions: Vec<SamplerDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: SamplerDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUSamplerDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSamplerDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.address_mode_u {
            raw.addressModeU = value.into();
        } else {
            raw.addressModeU = 0 as ffi::WGPUAddressMode;
        }
        if let Some(value) = self.address_mode_v {
            raw.addressModeV = value.into();
        } else {
            raw.addressModeV = 0 as ffi::WGPUAddressMode;
        }
        if let Some(value) = self.address_mode_w {
            raw.addressModeW = value.into();
        } else {
            raw.addressModeW = 0 as ffi::WGPUAddressMode;
        }
        if let Some(value) = self.mag_filter {
            raw.magFilter = value.into();
        } else {
            raw.magFilter = 0 as ffi::WGPUFilterMode;
        }
        if let Some(value) = self.min_filter {
            raw.minFilter = value.into();
        } else {
            raw.minFilter = 0 as ffi::WGPUFilterMode;
        }
        if let Some(value) = self.mipmap_filter {
            raw.mipmapFilter = value.into();
        } else {
            raw.mipmapFilter = 0 as ffi::WGPUMipmapFilterMode;
        }
        if let Some(value) = self.lod_min_clamp {
            raw.lodMinClamp = value;
        }
        if let Some(value) = self.lod_max_clamp {
            raw.lodMaxClamp = value;
        }
        if let Some(value) = self.compare {
            raw.compare = value.into();
        } else {
            raw.compare = 0 as ffi::WGPUCompareFunction;
        }
        if let Some(value) = self.max_anisotropy {
            raw.maxAnisotropy = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SamplerDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
        Self { strict_math: None }
    }
}
impl ShaderModuleCompilationOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUShaderModuleCompilationOptions, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUShaderModuleCompilationOptions = unsafe {
            std::mem::zeroed()
        };
        raw.strictMath = if self.strict_math.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUShaderModuleCompilationOptions) -> Self {
        Self {
            strict_math: Some(value.strictMath != 0),
        }
    }
}
pub struct ShaderModuleDescriptor {
    pub(crate) extensions: Vec<ShaderModuleDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: ShaderModuleDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUShaderModuleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUShaderModuleDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: ShaderModuleDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub code: Option<Vec<u32>>,
}
impl Default for ShaderSourceSPIRV {
    fn default() -> Self {
        Self { code: None }
    }
}
impl ShaderSourceSPIRV {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUShaderSourceSPIRV, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUShaderSourceSPIRV = unsafe { std::mem::zeroed() };
        raw.codeSize = (self.code.as_ref().map(|v| v.len()).unwrap_or(0)) as u32;
        if let Some(values) = &self.code {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.code = ptr;
            raw.codeSize = len_value as u32;
        } else {
            raw.code = std::ptr::null();
            raw.codeSize = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUShaderSourceSPIRV) -> Self {
        Self {
            code: if value.code.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(value.code, value.codeSize as usize)
                    }
                        .to_vec(),
                )
            },
        }
    }
}
pub struct ShaderSourceWGSL {
    pub code: Option<String>,
}
impl Default for ShaderSourceWGSL {
    fn default() -> Self {
        Self { code: None }
    }
}
impl ShaderSourceWGSL {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUShaderSourceWGSL, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUShaderSourceWGSL = unsafe { std::mem::zeroed() };
        if let Some(value) = &self.code {
            raw.code = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.code = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUShaderSourceWGSL) -> Self {
        Self {
            code: Some(string_view_to_string(value.code)),
        }
    }
}
pub struct SharedBufferMemoryBeginAccessDescriptor {
    pub(crate) extensions: Vec<SharedBufferMemoryBeginAccessDescriptorExtension>,
    pub initialized: Option<bool>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}
impl Default for SharedBufferMemoryBeginAccessDescriptor {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            initialized: None,
            fences: None,
            signaled_values: None,
        }
    }
}
impl SharedBufferMemoryBeginAccessDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(
        &mut self,
        ext: SharedBufferMemoryBeginAccessDescriptorExtension,
    ) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedBufferMemoryBeginAccessDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryBeginAccessDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        raw.initialized = if self.initialized.unwrap_or(false) { 1 } else { 0 };
        raw.fenceCount = self.signaled_values.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.fences {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUSharedFence> = values
                .iter()
                .map(|v| v.as_raw())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.fences = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.fences = std::ptr::null();
            raw.fenceCount = 0;
        }
        if let Some(values) = &self.signaled_values {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.signaledValues = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.signaledValues = std::ptr::null();
            raw.fenceCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedBufferMemoryBeginAccessDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedBufferMemoryBeginAccessDescriptor,
    ) -> Self {
        Self {
            extensions: Vec::new(),
            initialized: Some(value.initialized != 0),
            fences: if value.fences.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.fences,
                            value.fenceCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                        .collect(),
                )
            },
            signaled_values: if value.signaledValues.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.signaledValues,
                            value.fenceCount as usize,
                        )
                    }
                        .to_vec(),
                )
            },
        }
    }
}
pub struct SharedBufferMemoryDescriptor {
    pub(crate) extensions: Vec<SharedBufferMemoryDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: SharedBufferMemoryDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedBufferMemoryDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedBufferMemoryDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<SharedBufferMemoryEndAccessStateExtension>,
    pub initialized: Option<bool>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}
impl Default for SharedBufferMemoryEndAccessState {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            initialized: None,
            fences: None,
            signaled_values: None,
        }
    }
}
impl SharedBufferMemoryEndAccessState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: SharedBufferMemoryEndAccessStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedBufferMemoryEndAccessState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryEndAccessState = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        raw.initialized = if self.initialized.unwrap_or(false) { 1 } else { 0 };
        raw.fenceCount = self.signaled_values.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.fences {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUSharedFence> = values
                .iter()
                .map(|v| v.as_raw())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.fences = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.fences = std::ptr::null();
            raw.fenceCount = 0;
        }
        if let Some(values) = &self.signaled_values {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.signaledValues = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.signaledValues = std::ptr::null();
            raw.fenceCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedBufferMemoryEndAccessStateExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedBufferMemoryEndAccessState) -> Self {
        Self {
            extensions: Vec::new(),
            initialized: Some(value.initialized != 0),
            fences: if value.fences.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.fences,
                            value.fenceCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                        .collect(),
                )
            },
            signaled_values: if value.signaledValues.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.signaledValues,
                            value.fenceCount as usize,
                        )
                    }
                        .to_vec(),
                )
            },
        }
    }
    pub(crate) fn free_members(value: ffi::WGPUSharedBufferMemoryEndAccessState) {
        unsafe { ffi::wgpuSharedBufferMemoryEndAccessStateFreeMembers(value) };
    }
}
pub struct SharedBufferMemoryProperties {
    pub(crate) extensions: Vec<SharedBufferMemoryPropertiesExtension>,
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
    pub fn push_extension(&mut self, ext: SharedBufferMemoryPropertiesExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedBufferMemoryProperties, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedBufferMemoryProperties = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = self.usage {
            raw.usage = value.into();
        } else {
            raw.usage = 0 as ffi::WGPUBufferUsage;
        }
        if let Some(value) = self.size {
            raw.size = value;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedBufferMemoryPropertiesExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
        Self { handle: None }
    }
}
impl SharedFenceDXGISharedHandleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceDXGISharedHandleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceDXGISharedHandleDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedFenceDXGISharedHandleDescriptor,
    ) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedFenceDXGISharedHandleExportInfo {
    pub handle: Option<*mut std::ffi::c_void>,
}
impl Default for SharedFenceDXGISharedHandleExportInfo {
    fn default() -> Self {
        Self { handle: None }
    }
}
impl SharedFenceDXGISharedHandleExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceDXGISharedHandleExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceDXGISharedHandleExportInfo = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedFenceDXGISharedHandleExportInfo,
    ) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedFenceEGLSyncDescriptor {
    pub sync: Option<*mut std::ffi::c_void>,
}
impl Default for SharedFenceEGLSyncDescriptor {
    fn default() -> Self {
        Self { sync: None }
    }
}
impl SharedFenceEGLSyncDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceEGLSyncDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceEGLSyncDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.sync {
            raw.sync = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceEGLSyncDescriptor) -> Self {
        Self { sync: Some(value.sync) }
    }
}
pub struct SharedFenceEGLSyncExportInfo {
    pub sync: Option<*mut std::ffi::c_void>,
}
impl Default for SharedFenceEGLSyncExportInfo {
    fn default() -> Self {
        Self { sync: None }
    }
}
impl SharedFenceEGLSyncExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceEGLSyncExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceEGLSyncExportInfo = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.sync {
            raw.sync = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceEGLSyncExportInfo) -> Self {
        Self { sync: Some(value.sync) }
    }
}
pub struct SharedFenceMTLSharedEventDescriptor {
    pub shared_event: Option<*mut std::ffi::c_void>,
}
impl Default for SharedFenceMTLSharedEventDescriptor {
    fn default() -> Self {
        Self { shared_event: None }
    }
}
impl SharedFenceMTLSharedEventDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceMTLSharedEventDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceMTLSharedEventDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.shared_event {
            raw.sharedEvent = value;
        }
        (raw, storage)
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
        Self { shared_event: None }
    }
}
impl SharedFenceMTLSharedEventExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceMTLSharedEventExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceMTLSharedEventExportInfo = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.shared_event {
            raw.sharedEvent = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceMTLSharedEventExportInfo) -> Self {
        Self {
            shared_event: Some(value.sharedEvent),
        }
    }
}
pub struct SharedFenceDescriptor {
    pub(crate) extensions: Vec<SharedFenceDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: SharedFenceDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedFenceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SharedFenceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<SharedFenceExportInfoExtension>,
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
    pub fn push_extension(&mut self, ext: SharedFenceExportInfoExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedFenceExportInfo = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.r#type {
            raw.type_ = value.into();
        } else {
            raw.type_ = 0 as ffi::WGPUSharedFenceType;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SharedFenceExportInfoExtension) -> Self {
        self.extensions.push(extension);
        self
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
        Self { handle: None }
    }
}
impl SharedFenceSyncFDDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceSyncFDDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceSyncFDDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceSyncFDDescriptor) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedFenceSyncFDExportInfo {
    pub handle: Option<i32>,
}
impl Default for SharedFenceSyncFDExportInfo {
    fn default() -> Self {
        Self { handle: None }
    }
}
impl SharedFenceSyncFDExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceSyncFDExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceSyncFDExportInfo = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedFenceSyncFDExportInfo) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedFenceVkSemaphoreOpaqueFDDescriptor {
    pub handle: Option<i32>,
}
impl Default for SharedFenceVkSemaphoreOpaqueFDDescriptor {
    fn default() -> Self {
        Self { handle: None }
    }
}
impl SharedFenceVkSemaphoreOpaqueFDDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceVkSemaphoreOpaqueFDDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceVkSemaphoreOpaqueFDDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedFenceVkSemaphoreOpaqueFDDescriptor,
    ) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedFenceVkSemaphoreOpaqueFDExportInfo {
    pub handle: Option<i32>,
}
impl Default for SharedFenceVkSemaphoreOpaqueFDExportInfo {
    fn default() -> Self {
        Self { handle: None }
    }
}
impl SharedFenceVkSemaphoreOpaqueFDExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceVkSemaphoreOpaqueFDExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceVkSemaphoreOpaqueFDExportInfo = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedFenceVkSemaphoreOpaqueFDExportInfo,
    ) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedFenceVkSemaphoreZirconHandleDescriptor {
    pub handle: Option<u32>,
}
impl Default for SharedFenceVkSemaphoreZirconHandleDescriptor {
    fn default() -> Self {
        Self { handle: None }
    }
}
impl SharedFenceVkSemaphoreZirconHandleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceVkSemaphoreZirconHandleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceVkSemaphoreZirconHandleDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedFenceVkSemaphoreZirconHandleDescriptor,
    ) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedFenceVkSemaphoreZirconHandleExportInfo {
    pub handle: Option<u32>,
}
impl Default for SharedFenceVkSemaphoreZirconHandleExportInfo {
    fn default() -> Self {
        Self { handle: None }
    }
}
impl SharedFenceVkSemaphoreZirconHandleExportInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedFenceVkSemaphoreZirconHandleExportInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedFenceVkSemaphoreZirconHandleExportInfo = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedFenceVkSemaphoreZirconHandleExportInfo,
    ) -> Self {
        Self { handle: Some(value.handle) }
    }
}
pub struct SharedTextureMemoryD3DSwapchainBeginState {
    pub is_swapchain: Option<bool>,
}
impl Default for SharedTextureMemoryD3DSwapchainBeginState {
    fn default() -> Self {
        Self { is_swapchain: None }
    }
}
impl SharedTextureMemoryD3DSwapchainBeginState {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryD3DSwapchainBeginState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryD3DSwapchainBeginState = unsafe {
            std::mem::zeroed()
        };
        raw.isSwapchain = if self.is_swapchain.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryD3DSwapchainBeginState,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryD3D11BeginState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryD3D11BeginState = unsafe {
            std::mem::zeroed()
        };
        raw.requiresEndAccessFence = if self.requires_end_access_fence.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryDXGISharedHandleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryDXGISharedHandleDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        raw.useKeyedMutex = if self.use_keyed_mutex.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryDXGISharedHandleDescriptor,
    ) -> Self {
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
        Self { image: None }
    }
}
impl SharedTextureMemoryEGLImageDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryEGLImageDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryEGLImageDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.image {
            raw.image = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryEGLImageDescriptor,
    ) -> Self {
        Self { image: Some(value.image) }
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryIOSurfaceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryIOSurfaceDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.io_surface {
            raw.ioSurface = value;
        }
        raw.allowStorageBinding = if self.allow_storage_binding.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryIOSurfaceDescriptor,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryAHardwareBufferDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryAHardwareBufferDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.handle {
            raw.handle = value;
        }
        raw.useExternalFormat = if self.use_external_format.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryAHardwareBufferDescriptor,
    ) -> Self {
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
        Self { y_cb_cr_info: None }
    }
}
impl SharedTextureMemoryAHardwareBufferProperties {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryAHardwareBufferProperties, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryAHardwareBufferProperties = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = &self.y_cb_cr_info {
            let (raw_value, storage_value) = value.to_ffi();
            raw.yCbCrInfo = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryAHardwareBufferProperties,
    ) -> Self {
        Self {
            y_cb_cr_info: Some(YCbCrVkDescriptor::from_ffi(value.yCbCrInfo)),
        }
    }
}
pub struct SharedTextureMemoryBeginAccessDescriptor {
    pub(crate) extensions: Vec<SharedTextureMemoryBeginAccessDescriptorExtension>,
    pub concurrent_read: Option<bool>,
    pub initialized: Option<bool>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}
impl Default for SharedTextureMemoryBeginAccessDescriptor {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            concurrent_read: None,
            initialized: None,
            fences: None,
            signaled_values: None,
        }
    }
}
impl SharedTextureMemoryBeginAccessDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(
        &mut self,
        ext: SharedTextureMemoryBeginAccessDescriptorExtension,
    ) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryBeginAccessDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryBeginAccessDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        raw.concurrentRead = if self.concurrent_read.unwrap_or(false) { 1 } else { 0 };
        raw.initialized = if self.initialized.unwrap_or(false) { 1 } else { 0 };
        raw.fenceCount = self.signaled_values.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.fences {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUSharedFence> = values
                .iter()
                .map(|v| v.as_raw())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.fences = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.fences = std::ptr::null();
            raw.fenceCount = 0;
        }
        if let Some(values) = &self.signaled_values {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.signaledValues = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.signaledValues = std::ptr::null();
            raw.fenceCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedTextureMemoryBeginAccessDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryBeginAccessDescriptor,
    ) -> Self {
        Self {
            extensions: Vec::new(),
            concurrent_read: Some(value.concurrentRead != 0),
            initialized: Some(value.initialized != 0),
            fences: if value.fences.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.fences,
                            value.fenceCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                        .collect(),
                )
            },
            signaled_values: if value.signaledValues.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.signaledValues,
                            value.fenceCount as usize,
                        )
                    }
                        .to_vec(),
                )
            },
        }
    }
}
pub struct SharedTextureMemoryDescriptor {
    pub(crate) extensions: Vec<SharedTextureMemoryDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: SharedTextureMemoryDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedTextureMemoryDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
    pub planes: Option<Vec<SharedTextureMemoryDmaBufPlane>>,
}
impl Default for SharedTextureMemoryDmaBufDescriptor {
    fn default() -> Self {
        Self {
            size: None,
            drm_format: None,
            drm_modifier: None,
            planes: None,
        }
    }
}
impl SharedTextureMemoryDmaBufDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryDmaBufDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryDmaBufDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = &self.size {
            let (raw_value, storage_value) = value.to_ffi();
            raw.size = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = self.drm_format {
            raw.drmFormat = value;
        }
        if let Some(value) = self.drm_modifier {
            raw.drmModifier = value;
        }
        raw.planeCount = self.planes.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.planes {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUSharedTextureMemoryDmaBufPlane> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.planes = ptr;
            raw.planeCount = len_value;
        } else {
            raw.planes = std::ptr::null();
            raw.planeCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryDmaBufDescriptor) -> Self {
        Self {
            size: Some(Extent3D::from_ffi(value.size)),
            drm_format: Some(value.drmFormat),
            drm_modifier: Some(value.drmModifier),
            planes: if value.planes.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.planes,
                            value.planeCount as usize,
                        )
                    }
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryDmaBufPlane, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryDmaBufPlane = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.fd {
            raw.fd = value;
        }
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.stride {
            raw.stride = value;
        }
        (raw, storage)
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
    pub(crate) extensions: Vec<SharedTextureMemoryEndAccessStateExtension>,
    pub initialized: Option<bool>,
    pub fences: Option<Vec<SharedFence>>,
    pub signaled_values: Option<Vec<u64>>,
}
impl Default for SharedTextureMemoryEndAccessState {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            initialized: None,
            fences: None,
            signaled_values: None,
        }
    }
}
impl SharedTextureMemoryEndAccessState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: SharedTextureMemoryEndAccessStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryEndAccessState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryEndAccessState = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        raw.initialized = if self.initialized.unwrap_or(false) { 1 } else { 0 };
        raw.fenceCount = self.signaled_values.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.fences {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUSharedFence> = values
                .iter()
                .map(|v| v.as_raw())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.fences = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.fences = std::ptr::null();
            raw.fenceCount = 0;
        }
        if let Some(values) = &self.signaled_values {
            let len_value = values.len();
            let raw_vec = values.to_vec();
            let ptr = storage.push_vec(raw_vec);
            raw.signaledValues = ptr;
            raw.fenceCount = len_value;
        } else {
            raw.signaledValues = std::ptr::null();
            raw.fenceCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedTextureMemoryEndAccessStateExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSharedTextureMemoryEndAccessState) -> Self {
        Self {
            extensions: Vec::new(),
            initialized: Some(value.initialized != 0),
            fences: if value.fences.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.fences,
                            value.fenceCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| unsafe { SharedFence::from_raw(*raw) })
                        .collect(),
                )
            },
            signaled_values: if value.signaledValues.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.signaledValues,
                            value.fenceCount as usize,
                        )
                    }
                        .to_vec(),
                )
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryMetalEndAccessState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryMetalEndAccessState = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = &self.commands_scheduled_future {
            let (raw_value, storage_value) = value.to_ffi();
            raw.commandsScheduledFuture = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryMetalEndAccessState,
    ) -> Self {
        Self {
            commands_scheduled_future: Some(
                Future::from_ffi(value.commandsScheduledFuture),
            ),
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryOpaqueFDDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryOpaqueFDDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.vk_image_create_info {
            raw.vkImageCreateInfo = value;
        }
        if let Some(value) = self.memory_fd {
            raw.memoryFD = value;
        }
        if let Some(value) = self.memory_type_index {
            raw.memoryTypeIndex = value;
        }
        if let Some(value) = self.allocation_size {
            raw.allocationSize = value;
        }
        raw.dedicatedAllocation = if self.dedicated_allocation.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryOpaqueFDDescriptor,
    ) -> Self {
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
    pub(crate) extensions: Vec<SharedTextureMemoryPropertiesExtension>,
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
    pub fn push_extension(&mut self, ext: SharedTextureMemoryPropertiesExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryProperties, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSharedTextureMemoryProperties = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = self.usage {
            raw.usage = value.into();
        } else {
            raw.usage = 0 as ffi::WGPUTextureUsage;
        }
        if let Some(value) = &self.size {
            let (raw_value, storage_value) = value.to_ffi();
            raw.size = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: SharedTextureMemoryPropertiesExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
        Self { dedicated_allocation: None }
    }
}
impl SharedTextureMemoryVkDedicatedAllocationDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (
        ffi::WGPUSharedTextureMemoryVkDedicatedAllocationDescriptor,
        ChainedStructStorage,
    ) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryVkDedicatedAllocationDescriptor = unsafe {
            std::mem::zeroed()
        };
        raw.dedicatedAllocation = if self.dedicated_allocation.unwrap_or(false) {
            1
        } else {
            0
        };
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryVkDedicatedAllocationDescriptor,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryVkImageLayoutBeginState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryVkImageLayoutBeginState = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.old_layout {
            raw.oldLayout = value;
        }
        if let Some(value) = self.new_layout {
            raw.newLayout = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryVkImageLayoutBeginState,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryVkImageLayoutEndState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryVkImageLayoutEndState = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.old_layout {
            raw.oldLayout = value;
        }
        if let Some(value) = self.new_layout {
            raw.newLayout = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryVkImageLayoutEndState,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSharedTextureMemoryZirconHandleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSharedTextureMemoryZirconHandleDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.memory_fd {
            raw.memoryFD = value;
        }
        if let Some(value) = self.allocation_size {
            raw.allocationSize = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSharedTextureMemoryZirconHandleDescriptor,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUStaticSamplerBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUStaticSamplerBindingLayout = unsafe { std::mem::zeroed() };
        raw.sampler = self
            .sampler
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.sampled_texture_binding {
            raw.sampledTextureBinding = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUStencilFaceState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUStencilFaceState = unsafe { std::mem::zeroed() };
        if let Some(value) = self.compare {
            raw.compare = value.into();
        } else {
            raw.compare = 0 as ffi::WGPUCompareFunction;
        }
        if let Some(value) = self.fail_op {
            raw.failOp = value.into();
        } else {
            raw.failOp = 0 as ffi::WGPUStencilOperation;
        }
        if let Some(value) = self.depth_fail_op {
            raw.depthFailOp = value.into();
        } else {
            raw.depthFailOp = 0 as ffi::WGPUStencilOperation;
        }
        if let Some(value) = self.pass_op {
            raw.passOp = value.into();
        } else {
            raw.passOp = 0 as ffi::WGPUStencilOperation;
        }
        (raw, storage)
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
    pub(crate) extensions: Vec<StorageTextureBindingLayoutExtension>,
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
    pub fn push_extension(&mut self, ext: StorageTextureBindingLayoutExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUStorageTextureBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUStorageTextureBindingLayout = unsafe {
            std::mem::zeroed()
        };
        raw.nextInChain = next;
        if let Some(value) = self.access {
            raw.access = value.into();
        } else {
            raw.access = 0 as ffi::WGPUStorageTextureAccess;
        }
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = self.view_dimension {
            raw.viewDimension = value.into();
        } else {
            raw.viewDimension = 0 as ffi::WGPUTextureViewDimension;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: StorageTextureBindingLayoutExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
        Self { data: None, length: None }
    }
}
impl StringView {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUStringView, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUStringView = unsafe { std::mem::zeroed() };
        if let Some(value) = self.data {
            raw.data = value;
        }
        if let Some(value) = self.length {
            raw.length = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSubgroupMatrixConfig, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSubgroupMatrixConfig = unsafe { std::mem::zeroed() };
        if let Some(value) = self.component_type {
            raw.componentType = value.into();
        } else {
            raw.componentType = 0 as ffi::WGPUSubgroupMatrixComponentType;
        }
        if let Some(value) = self.result_component_type {
            raw.resultComponentType = value.into();
        } else {
            raw.resultComponentType = 0 as ffi::WGPUSubgroupMatrixComponentType;
        }
        if let Some(value) = self.m {
            raw.M = value;
        }
        if let Some(value) = self.n {
            raw.N = value;
        }
        if let Some(value) = self.k {
            raw.K = value;
        }
        (raw, storage)
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
    pub features: Option<Vec<WGSLLanguageFeatureName>>,
}
impl Default for SupportedWGSLLanguageFeatures {
    fn default() -> Self {
        Self { features: None }
    }
}
impl SupportedWGSLLanguageFeatures {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSupportedWGSLLanguageFeatures, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSupportedWGSLLanguageFeatures = unsafe {
            std::mem::zeroed()
        };
        raw.featureCount = self.features.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.features {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUWGSLLanguageFeatureName> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.features = ptr;
            raw.featureCount = len_value;
        } else {
            raw.features = std::ptr::null();
            raw.featureCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSupportedWGSLLanguageFeatures) -> Self {
        Self {
            features: if value.features.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.features,
                            value.featureCount as usize,
                        )
                    }
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
    pub features: Option<Vec<FeatureName>>,
}
impl Default for SupportedFeatures {
    fn default() -> Self {
        Self { features: None }
    }
}
impl SupportedFeatures {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUSupportedFeatures, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSupportedFeatures = unsafe { std::mem::zeroed() };
        raw.featureCount = self.features.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.features {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUFeatureName> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.features = ptr;
            raw.featureCount = len_value;
        } else {
            raw.features = std::ptr::null();
            raw.featureCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSupportedFeatures) -> Self {
        Self {
            features: if value.features.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.features,
                            value.featureCount as usize,
                        )
                    }
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
    pub features: Option<Vec<InstanceFeatureName>>,
}
impl Default for SupportedInstanceFeatures {
    fn default() -> Self {
        Self { features: None }
    }
}
impl SupportedInstanceFeatures {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSupportedInstanceFeatures, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSupportedInstanceFeatures = unsafe { std::mem::zeroed() };
        raw.featureCount = self.features.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.features {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUInstanceFeatureName> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.features = ptr;
            raw.featureCount = len_value;
        } else {
            raw.features = std::ptr::null();
            raw.featureCount = 0;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSupportedInstanceFeatures) -> Self {
        Self {
            features: if value.features.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.features,
                            value.featureCount as usize,
                        )
                    }
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
    pub(crate) extensions: Vec<SurfaceCapabilitiesExtension>,
    pub usages: Option<TextureUsage>,
    pub formats: Option<Vec<TextureFormat>>,
    pub present_modes: Option<Vec<PresentMode>>,
    pub alpha_modes: Option<Vec<CompositeAlphaMode>>,
}
impl Default for SurfaceCapabilities {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            usages: None,
            formats: None,
            present_modes: None,
            alpha_modes: None,
        }
    }
}
impl SurfaceCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: SurfaceCapabilitiesExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUSurfaceCapabilities, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceCapabilities = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.usages {
            raw.usages = value.into();
        } else {
            raw.usages = 0 as ffi::WGPUTextureUsage;
        }
        raw.formatCount = self.formats.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.formats {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUTextureFormat> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.formats = ptr;
            raw.formatCount = len_value;
        } else {
            raw.formats = std::ptr::null();
            raw.formatCount = 0;
        }
        raw.presentModeCount = self.present_modes.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.present_modes {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUPresentMode> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.presentModes = ptr;
            raw.presentModeCount = len_value;
        } else {
            raw.presentModes = std::ptr::null();
            raw.presentModeCount = 0;
        }
        raw.alphaModeCount = self.alpha_modes.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.alpha_modes {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUCompositeAlphaMode> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.alphaModes = ptr;
            raw.alphaModeCount = len_value;
        } else {
            raw.alphaModes = std::ptr::null();
            raw.alphaModeCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SurfaceCapabilitiesExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceCapabilities) -> Self {
        Self {
            extensions: Vec::new(),
            usages: Some(value.usages.into()),
            formats: if value.formats.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.formats,
                            value.formatCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| TextureFormat::from(*raw))
                        .collect(),
                )
            },
            present_modes: if value.presentModes.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.presentModes,
                            value.presentModeCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| PresentMode::from(*raw))
                        .collect(),
                )
            },
            alpha_modes: if value.alphaModes.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.alphaModes,
                            value.alphaModeCount as usize,
                        )
                    }
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceColorManagement, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceColorManagement = unsafe { std::mem::zeroed() };
        if let Some(value) = self.color_space {
            raw.colorSpace = value.into();
        } else {
            raw.colorSpace = 0 as ffi::WGPUPredefinedColorSpace;
        }
        if let Some(value) = self.tone_mapping_mode {
            raw.toneMappingMode = value.into();
        } else {
            raw.toneMappingMode = 0 as ffi::WGPUToneMappingMode;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceColorManagement) -> Self {
        Self {
            color_space: Some(value.colorSpace.into()),
            tone_mapping_mode: Some(value.toneMappingMode.into()),
        }
    }
}
pub struct SurfaceConfiguration {
    pub(crate) extensions: Vec<SurfaceConfigurationExtension>,
    pub device: Option<Device>,
    pub format: Option<TextureFormat>,
    pub usage: Option<TextureUsage>,
    pub width: Option<u32>,
    pub height: Option<u32>,
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
    pub fn push_extension(&mut self, ext: SurfaceConfigurationExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceConfiguration, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceConfiguration = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.device = self
            .device
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = self.usage {
            raw.usage = value.into();
        } else {
            raw.usage = 0 as ffi::WGPUTextureUsage;
        }
        if let Some(value) = self.width {
            raw.width = value;
        }
        if let Some(value) = self.height {
            raw.height = value;
        }
        raw.viewFormatCount = self.view_formats.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.view_formats {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUTextureFormat> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.viewFormats = ptr;
            raw.viewFormatCount = len_value;
        } else {
            raw.viewFormats = std::ptr::null();
            raw.viewFormatCount = 0;
        }
        if let Some(value) = self.alpha_mode {
            raw.alphaMode = value.into();
        } else {
            raw.alphaMode = 0 as ffi::WGPUCompositeAlphaMode;
        }
        if let Some(value) = self.present_mode {
            raw.presentMode = value.into();
        } else {
            raw.presentMode = 0 as ffi::WGPUPresentMode;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SurfaceConfigurationExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceConfiguration) -> Self {
        Self {
            extensions: Vec::new(),
            device: Some(unsafe { Device::from_raw(value.device) }),
            format: Some(value.format.into()),
            usage: Some(value.usage.into()),
            width: Some(value.width),
            height: Some(value.height),
            view_formats: if value.viewFormats.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.viewFormats,
                            value.viewFormatCount as usize,
                        )
                    }
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
    pub(crate) extensions: Vec<SurfaceDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: SurfaceDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUSurfaceDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SurfaceDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
        Self { swap_chain_panel: None }
    }
}
impl SurfaceDescriptorFromWindowsUWPSwapChainPanel {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceDescriptorFromWindowsUWPSwapChainPanel, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceDescriptorFromWindowsUWPSwapChainPanel = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.swap_chain_panel {
            raw.swapChainPanel = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSurfaceDescriptorFromWindowsUWPSwapChainPanel,
    ) -> Self {
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
        Self { swap_chain_panel: None }
    }
}
impl SurfaceDescriptorFromWindowsWinUISwapChainPanel {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (
        ffi::WGPUSurfaceDescriptorFromWindowsWinUISwapChainPanel,
        ChainedStructStorage,
    ) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceDescriptorFromWindowsWinUISwapChainPanel = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.swap_chain_panel {
            raw.swapChainPanel = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSurfaceDescriptorFromWindowsWinUISwapChainPanel,
    ) -> Self {
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
        Self { core_window: None }
    }
}
impl SurfaceDescriptorFromWindowsCoreWindow {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceDescriptorFromWindowsCoreWindow, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceDescriptorFromWindowsCoreWindow = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.core_window {
            raw.coreWindow = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(
        value: ffi::WGPUSurfaceDescriptorFromWindowsCoreWindow,
    ) -> Self {
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceSourceXCBWindow, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceSourceXCBWindow = unsafe { std::mem::zeroed() };
        if let Some(value) = self.connection {
            raw.connection = value;
        }
        if let Some(value) = self.window {
            raw.window = value;
        }
        (raw, storage)
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
        Self { window: None }
    }
}
impl SurfaceSourceAndroidNativeWindow {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceSourceAndroidNativeWindow, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceSourceAndroidNativeWindow = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.window {
            raw.window = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceAndroidNativeWindow) -> Self {
        Self { window: Some(value.window) }
    }
}
pub struct SurfaceSourceMetalLayer {
    pub layer: Option<*mut std::ffi::c_void>,
}
impl Default for SurfaceSourceMetalLayer {
    fn default() -> Self {
        Self { layer: None }
    }
}
impl SurfaceSourceMetalLayer {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceSourceMetalLayer, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceSourceMetalLayer = unsafe { std::mem::zeroed() };
        if let Some(value) = self.layer {
            raw.layer = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceMetalLayer) -> Self {
        Self { layer: Some(value.layer) }
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceSourceWaylandSurface, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceSourceWaylandSurface = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = self.display {
            raw.display = value;
        }
        if let Some(value) = self.surface {
            raw.surface = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceSourceWindowsHWND, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceSourceWindowsHWND = unsafe { std::mem::zeroed() };
        if let Some(value) = self.hinstance {
            raw.hinstance = value;
        }
        if let Some(value) = self.hwnd {
            raw.hwnd = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUSurfaceSourceXlibWindow, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUSurfaceSourceXlibWindow = unsafe { std::mem::zeroed() };
        if let Some(value) = self.display {
            raw.display = value;
        }
        if let Some(value) = self.window {
            raw.window = value;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUSurfaceSourceXlibWindow) -> Self {
        Self {
            display: Some(value.display),
            window: Some(value.window),
        }
    }
}
pub struct SurfaceTexture {
    pub(crate) extensions: Vec<SurfaceTextureExtension>,
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
    pub fn push_extension(&mut self, ext: SurfaceTextureExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUSurfaceTexture, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUSurfaceTexture = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.texture = self
            .texture
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.status {
            raw.status = value.into();
        } else {
            raw.status = 0 as ffi::WGPUSurfaceGetCurrentTextureStatus;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: SurfaceTextureExtension) -> Self {
        self.extensions.push(extension);
        self
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
        Self { texel_buffer_view: None }
    }
}
impl TexelBufferBindingEntry {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTexelBufferBindingEntry, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUTexelBufferBindingEntry = unsafe { std::mem::zeroed() };
        raw.texelBufferView = self
            .texel_buffer_view
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUTexelBufferBindingEntry) -> Self {
        Self {
            texel_buffer_view: Some(unsafe {
                TexelBufferView::from_raw(value.texelBufferView)
            }),
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTexelBufferBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUTexelBufferBindingLayout = unsafe { std::mem::zeroed() };
        if let Some(value) = self.access {
            raw.access = value.into();
        } else {
            raw.access = 0 as ffi::WGPUTexelBufferAccess;
        }
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUTexelBufferBindingLayout) -> Self {
        Self {
            access: Some(value.access.into()),
            format: Some(value.format.into()),
        }
    }
}
pub struct TexelBufferViewDescriptor {
    pub(crate) extensions: Vec<TexelBufferViewDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: TexelBufferViewDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTexelBufferViewDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTexelBufferViewDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.size {
            raw.size = value;
        }
        (raw, storage)
    }
    pub fn with_extension(
        mut self,
        extension: TexelBufferViewDescriptorExtension,
    ) -> Self {
        self.extensions.push(extension);
        self
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
        Self { layout: None, buffer: None }
    }
}
impl TexelCopyBufferInfo {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUTexelCopyBufferInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUTexelCopyBufferInfo = unsafe { std::mem::zeroed() };
        if let Some(value) = &self.layout {
            let (raw_value, storage_value) = value.to_ffi();
            raw.layout = raw_value;
            storage.push_storage(storage_value);
        }
        raw.buffer = self
            .buffer
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTexelCopyBufferLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUTexelCopyBufferLayout = unsafe { std::mem::zeroed() };
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.bytes_per_row {
            raw.bytesPerRow = value;
        }
        if let Some(value) = self.rows_per_image {
            raw.rowsPerImage = value;
        }
        (raw, storage)
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTexelCopyTextureInfo, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUTexelCopyTextureInfo = unsafe { std::mem::zeroed() };
        raw.texture = self
            .texture
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = self.mip_level {
            raw.mipLevel = value;
        }
        if let Some(value) = &self.origin {
            let (raw_value, storage_value) = value.to_ffi();
            raw.origin = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = self.aspect {
            raw.aspect = value.into();
        } else {
            raw.aspect = 0 as ffi::WGPUTextureAspect;
        }
        (raw, storage)
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
    pub(crate) extensions: Vec<TextureBindingLayoutExtension>,
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
    pub fn push_extension(&mut self, ext: TextureBindingLayoutExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTextureBindingLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTextureBindingLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.sample_type {
            raw.sampleType = value.into();
        } else {
            raw.sampleType = 0 as ffi::WGPUTextureSampleType;
        }
        if let Some(value) = self.view_dimension {
            raw.viewDimension = value.into();
        } else {
            raw.viewDimension = 0 as ffi::WGPUTextureViewDimension;
        }
        raw.multisampled = if self.multisampled.unwrap_or(false) { 1 } else { 0 };
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: TextureBindingLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTextureComponentSwizzle, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUTextureComponentSwizzle = unsafe { std::mem::zeroed() };
        if let Some(value) = self.r {
            raw.r = value.into();
        } else {
            raw.r = 0 as ffi::WGPUComponentSwizzle;
        }
        if let Some(value) = self.g {
            raw.g = value.into();
        } else {
            raw.g = 0 as ffi::WGPUComponentSwizzle;
        }
        if let Some(value) = self.b {
            raw.b = value.into();
        } else {
            raw.b = 0 as ffi::WGPUComponentSwizzle;
        }
        if let Some(value) = self.a {
            raw.a = value.into();
        } else {
            raw.a = 0 as ffi::WGPUComponentSwizzle;
        }
        (raw, storage)
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
        Self { swizzle: None }
    }
}
impl TextureComponentSwizzleDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTextureComponentSwizzleDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUTextureComponentSwizzleDescriptor = unsafe {
            std::mem::zeroed()
        };
        if let Some(value) = &self.swizzle {
            let (raw_value, storage_value) = value.to_ffi();
            raw.swizzle = raw_value;
            storage.push_storage(storage_value);
        }
        (raw, storage)
    }
    pub(crate) fn from_ffi(value: ffi::WGPUTextureComponentSwizzleDescriptor) -> Self {
        Self {
            swizzle: Some(TextureComponentSwizzle::from_ffi(value.swizzle)),
        }
    }
}
pub struct TextureDescriptor {
    pub(crate) extensions: Vec<TextureDescriptorExtension>,
    pub label: Option<String>,
    pub usage: Option<TextureUsage>,
    pub dimension: Option<TextureDimension>,
    pub size: Option<Extent3D>,
    pub format: Option<TextureFormat>,
    pub mip_level_count: Option<u32>,
    pub sample_count: Option<u32>,
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
            view_formats: None,
        }
    }
}
impl TextureDescriptor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: TextureDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUTextureDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTextureDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.usage {
            raw.usage = value.into();
        } else {
            raw.usage = 0 as ffi::WGPUTextureUsage;
        }
        if let Some(value) = self.dimension {
            raw.dimension = value.into();
        } else {
            raw.dimension = 0 as ffi::WGPUTextureDimension;
        }
        if let Some(value) = &self.size {
            let (raw_value, storage_value) = value.to_ffi();
            raw.size = raw_value;
            storage.push_storage(storage_value);
        }
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = self.mip_level_count {
            raw.mipLevelCount = value;
        }
        if let Some(value) = self.sample_count {
            raw.sampleCount = value;
        }
        raw.viewFormatCount = self.view_formats.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.view_formats {
            let len_value = values.len();
            let raw_vec: Vec<ffi::WGPUTextureFormat> = values
                .iter()
                .map(|v| (*v).into())
                .collect();
            let ptr = storage.push_vec(raw_vec);
            raw.viewFormats = ptr;
            raw.viewFormatCount = len_value;
        } else {
            raw.viewFormats = std::ptr::null();
            raw.viewFormatCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: TextureDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
            view_formats: if value.viewFormats.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.viewFormats,
                            value.viewFormatCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| TextureFormat::from(*raw))
                        .collect(),
                )
            },
        }
    }
}
pub struct TextureViewDescriptor {
    pub(crate) extensions: Vec<TextureViewDescriptorExtension>,
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
    pub fn push_extension(&mut self, ext: TextureViewDescriptorExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(
        &self,
    ) -> (ffi::WGPUTextureViewDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUTextureViewDescriptor = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = &self.label {
            raw.label = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.label = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUTextureFormat;
        }
        if let Some(value) = self.dimension {
            raw.dimension = value.into();
        } else {
            raw.dimension = 0 as ffi::WGPUTextureViewDimension;
        }
        if let Some(value) = self.base_mip_level {
            raw.baseMipLevel = value;
        }
        if let Some(value) = self.mip_level_count {
            raw.mipLevelCount = value;
        }
        if let Some(value) = self.base_array_layer {
            raw.baseArrayLayer = value;
        }
        if let Some(value) = self.array_layer_count {
            raw.arrayLayerCount = value;
        }
        if let Some(value) = self.aspect {
            raw.aspect = value.into();
        } else {
            raw.aspect = 0 as ffi::WGPUTextureAspect;
        }
        if let Some(value) = self.usage {
            raw.usage = value.into();
        } else {
            raw.usage = 0 as ffi::WGPUTextureUsage;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: TextureViewDescriptorExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<VertexAttributeExtension>,
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
    pub fn push_extension(&mut self, ext: VertexAttributeExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUVertexAttribute, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUVertexAttribute = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.format {
            raw.format = value.into();
        } else {
            raw.format = 0 as ffi::WGPUVertexFormat;
        }
        if let Some(value) = self.offset {
            raw.offset = value;
        }
        if let Some(value) = self.shader_location {
            raw.shaderLocation = value;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: VertexAttributeExtension) -> Self {
        self.extensions.push(extension);
        self
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
    pub(crate) extensions: Vec<VertexBufferLayoutExtension>,
    pub step_mode: Option<VertexStepMode>,
    pub array_stride: Option<u64>,
    pub attributes: Option<Vec<VertexAttribute>>,
}
impl Default for VertexBufferLayout {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            step_mode: None,
            array_stride: None,
            attributes: None,
        }
    }
}
impl VertexBufferLayout {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: VertexBufferLayoutExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUVertexBufferLayout, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUVertexBufferLayout = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        if let Some(value) = self.step_mode {
            raw.stepMode = value.into();
        } else {
            raw.stepMode = 0 as ffi::WGPUVertexStepMode;
        }
        if let Some(value) = self.array_stride {
            raw.arrayStride = value;
        }
        raw.attributeCount = self.attributes.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.attributes {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUVertexAttribute> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.attributes = ptr;
            raw.attributeCount = len_value;
        } else {
            raw.attributes = std::ptr::null();
            raw.attributeCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: VertexBufferLayoutExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUVertexBufferLayout) -> Self {
        Self {
            extensions: Vec::new(),
            step_mode: Some(value.stepMode.into()),
            array_stride: Some(value.arrayStride),
            attributes: if value.attributes.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.attributes,
                            value.attributeCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| VertexAttribute::from_ffi(*raw))
                        .collect(),
                )
            },
        }
    }
}
pub struct VertexState {
    pub(crate) extensions: Vec<VertexStateExtension>,
    pub module: Option<ShaderModule>,
    pub entry_point: Option<String>,
    pub constants: Option<Vec<ConstantEntry>>,
    pub buffers: Option<Vec<VertexBufferLayout>>,
}
impl Default for VertexState {
    fn default() -> Self {
        Self {
            extensions: Vec::new(),
            module: None,
            entry_point: None,
            constants: None,
            buffers: None,
        }
    }
}
impl VertexState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_extension(&mut self, ext: VertexStateExtension) {
        self.extensions.push(ext);
    }
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUVertexState, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter().rev() {
            next = ext.push_chain(&mut storage, next);
        }
        let mut raw: ffi::WGPUVertexState = unsafe { std::mem::zeroed() };
        raw.nextInChain = next;
        raw.module = self
            .module
            .as_ref()
            .map(|v| v.as_raw())
            .unwrap_or(std::ptr::null_mut());
        if let Some(value) = &self.entry_point {
            raw.entryPoint = ffi::WGPUStringView {
                data: value.as_ptr().cast(),
                length: value.len(),
            };
        } else {
            raw.entryPoint = ffi::WGPUStringView {
                data: std::ptr::null(),
                length: 0,
            };
        }
        raw.constantCount = self.constants.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.constants {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUConstantEntry> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.constants = ptr;
            raw.constantCount = len_value;
        } else {
            raw.constants = std::ptr::null();
            raw.constantCount = 0;
        }
        raw.bufferCount = self.buffers.as_ref().map(|v| v.len()).unwrap_or(0);
        if let Some(values) = &self.buffers {
            let len_value = values.len();
            let mut raw_vec: Vec<ffi::WGPUVertexBufferLayout> = Vec::with_capacity(
                values.len(),
            );
            for item in values.iter() {
                let (raw_item, storage_item) = item.to_ffi();
                raw_vec.push(raw_item);
                storage.push_storage(storage_item);
            }
            let ptr = storage.push_vec(raw_vec);
            raw.buffers = ptr;
            raw.bufferCount = len_value;
        } else {
            raw.buffers = std::ptr::null();
            raw.bufferCount = 0;
        }
        (raw, storage)
    }
    pub fn with_extension(mut self, extension: VertexStateExtension) -> Self {
        self.extensions.push(extension);
        self
    }
    pub(crate) fn from_ffi(value: ffi::WGPUVertexState) -> Self {
        Self {
            extensions: Vec::new(),
            module: Some(unsafe { ShaderModule::from_raw(value.module) }),
            entry_point: if value.entryPoint.data.is_null()
                || value.entryPoint.length == 0
            {
                None
            } else {
                Some(string_view_to_string(value.entryPoint))
            },
            constants: if value.constants.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.constants,
                            value.constantCount as usize,
                        )
                    }
                        .iter()
                        .map(|raw| ConstantEntry::from_ffi(*raw))
                        .collect(),
                )
            },
            buffers: if value.buffers.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        std::slice::from_raw_parts(
                            value.buffers,
                            value.bufferCount as usize,
                        )
                    }
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
    pub(crate) fn to_ffi(&self) -> (ffi::WGPUYCbCrVkDescriptor, ChainedStructStorage) {
        let mut storage = ChainedStructStorage::new();
        let mut raw: ffi::WGPUYCbCrVkDescriptor = unsafe { std::mem::zeroed() };
        if let Some(value) = self.vk_format {
            raw.vkFormat = value;
        }
        if let Some(value) = self.vk_y_cb_cr_model {
            raw.vkYCbCrModel = value;
        }
        if let Some(value) = self.vk_y_cb_cr_range {
            raw.vkYCbCrRange = value;
        }
        if let Some(value) = self.vk_component_swizzle_red {
            raw.vkComponentSwizzleRed = value;
        }
        if let Some(value) = self.vk_component_swizzle_green {
            raw.vkComponentSwizzleGreen = value;
        }
        if let Some(value) = self.vk_component_swizzle_blue {
            raw.vkComponentSwizzleBlue = value;
        }
        if let Some(value) = self.vk_component_swizzle_alpha {
            raw.vkComponentSwizzleAlpha = value;
        }
        if let Some(value) = self.vk_x_chroma_offset {
            raw.vkXChromaOffset = value;
        }
        if let Some(value) = self.vk_y_chroma_offset {
            raw.vkYChromaOffset = value;
        }
        if let Some(value) = self.vk_chroma_filter {
            raw.vkChromaFilter = value.into();
        } else {
            raw.vkChromaFilter = 0 as ffi::WGPUFilterMode;
        }
        raw.forceExplicitReconstruction = if self
            .force_explicit_reconstruction
            .unwrap_or(false)
        {
            1
        } else {
            0
        };
        if let Some(value) = self.external_format {
            raw.externalFormat = value;
        }
        (raw, storage)
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
