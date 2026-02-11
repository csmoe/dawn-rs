#![allow(dead_code, unused_imports)]
use crate::ffi;
use crate::generated::*;
use std::any::Any;
pub(crate) struct ChainedStructStorage {
    entries: Vec<Box<ffi::WGPUChainedStruct>>,
    buffers: Vec<Box<dyn Any>>,
    nested: Vec<ChainedStructStorage>,
}
impl ChainedStructStorage {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
            buffers: Vec::new(),
            nested: Vec::new(),
        }
    }
    pub(crate) fn push(
        &mut self,
        s_type: ffi::WGPUSType,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let mut node = Box::new(ffi::WGPUChainedStruct {
            next,
            sType: s_type,
        });
        let ptr = std::ptr::from_mut(node.as_mut());
        self.entries.push(node);
        ptr
    }
    pub(crate) fn push_value<T: 'static>(&mut self, value: T) -> *const T {
        let boxed = Box::new(value);
        let ptr = std::ptr::from_ref(boxed.as_ref());
        self.buffers.push(boxed);
        ptr
    }
    pub(crate) fn push_value_mut<T: 'static>(&mut self, value: T) -> *mut T {
        let mut boxed = Box::new(value);
        let ptr = std::ptr::from_mut(boxed.as_mut());
        self.buffers.push(boxed);
        ptr
    }
    pub(crate) fn push_vec<T: 'static>(&mut self, value: Vec<T>) -> *const T {
        let ptr = value.as_ptr();
        self.buffers.push(Box::new(value));
        ptr
    }
    pub(crate) fn push_vec_mut<T: 'static>(&mut self, value: Vec<T>) -> *mut T {
        let mut value = value;
        let ptr = value.as_mut_ptr();
        self.buffers.push(Box::new(value));
        ptr
    }
    pub(crate) fn push_any<T: 'static>(&mut self, value: T) {
        self.buffers.push(Box::new(value));
    }
    pub(crate) fn push_storage(&mut self, storage: ChainedStructStorage) {
        self.nested.push(storage);
    }
}
#[allow(dead_code)]
pub enum AdapterInfoExtension {
    AdapterPropertiesD3D(AdapterPropertiesD3D),
    AdapterPropertiesWGPU(AdapterPropertiesWGPU),
    AdapterPropertiesExplicitComputeSubgroupSizeConfigs(
        AdapterPropertiesExplicitComputeSubgroupSizeConfigs,
    ),
    AdapterPropertiesMemoryHeaps(AdapterPropertiesMemoryHeaps),
    AdapterPropertiesSubgroupMatrixConfigs(AdapterPropertiesSubgroupMatrixConfigs),
    AdapterPropertiesVk(AdapterPropertiesVk),
    DawnAdapterPropertiesPowerPreference(DawnAdapterPropertiesPowerPreference),
}
impl std::convert::From<AdapterPropertiesD3D> for AdapterInfoExtension {
    fn from(ext: AdapterPropertiesD3D) -> Self {
        AdapterInfoExtension::AdapterPropertiesD3D(ext)
    }
}
impl std::convert::From<AdapterPropertiesWGPU> for AdapterInfoExtension {
    fn from(ext: AdapterPropertiesWGPU) -> Self {
        AdapterInfoExtension::AdapterPropertiesWGPU(ext)
    }
}
impl std::convert::From<AdapterPropertiesExplicitComputeSubgroupSizeConfigs>
for AdapterInfoExtension {
    fn from(ext: AdapterPropertiesExplicitComputeSubgroupSizeConfigs) -> Self {
        AdapterInfoExtension::AdapterPropertiesExplicitComputeSubgroupSizeConfigs(ext)
    }
}
impl std::convert::From<AdapterPropertiesMemoryHeaps> for AdapterInfoExtension {
    fn from(ext: AdapterPropertiesMemoryHeaps) -> Self {
        AdapterInfoExtension::AdapterPropertiesMemoryHeaps(ext)
    }
}
impl std::convert::From<AdapterPropertiesSubgroupMatrixConfigs>
for AdapterInfoExtension {
    fn from(ext: AdapterPropertiesSubgroupMatrixConfigs) -> Self {
        AdapterInfoExtension::AdapterPropertiesSubgroupMatrixConfigs(ext)
    }
}
impl std::convert::From<AdapterPropertiesVk> for AdapterInfoExtension {
    fn from(ext: AdapterPropertiesVk) -> Self {
        AdapterInfoExtension::AdapterPropertiesVk(ext)
    }
}
impl std::convert::From<DawnAdapterPropertiesPowerPreference> for AdapterInfoExtension {
    fn from(ext: DawnAdapterPropertiesPowerPreference) -> Self {
        AdapterInfoExtension::DawnAdapterPropertiesPowerPreference(ext)
    }
}
impl AdapterInfoExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            AdapterInfoExtension::AdapterPropertiesD3D(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::AdapterPropertiesD3D as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            AdapterInfoExtension::AdapterPropertiesWGPU(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::AdapterPropertiesWGPU as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            AdapterInfoExtension::AdapterPropertiesExplicitComputeSubgroupSizeConfigs(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::AdapterPropertiesExplicitComputeSubgroupSizeConfigs
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            AdapterInfoExtension::AdapterPropertiesMemoryHeaps(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::AdapterPropertiesMemoryHeaps as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            AdapterInfoExtension::AdapterPropertiesSubgroupMatrixConfigs(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::AdapterPropertiesSubgroupMatrixConfigs
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            AdapterInfoExtension::AdapterPropertiesVk(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::AdapterPropertiesVk as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            AdapterInfoExtension::DawnAdapterPropertiesPowerPreference(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnAdapterPropertiesPowerPreference
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum BindGroupDescriptorExtension {}
impl BindGroupDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum BindGroupEntryExtension {
    ExternalTextureBindingEntry(ExternalTextureBindingEntry),
    TexelBufferBindingEntry(TexelBufferBindingEntry),
}
impl std::convert::From<ExternalTextureBindingEntry> for BindGroupEntryExtension {
    fn from(ext: ExternalTextureBindingEntry) -> Self {
        BindGroupEntryExtension::ExternalTextureBindingEntry(ext)
    }
}
impl std::convert::From<TexelBufferBindingEntry> for BindGroupEntryExtension {
    fn from(ext: TexelBufferBindingEntry) -> Self {
        BindGroupEntryExtension::TexelBufferBindingEntry(ext)
    }
}
impl BindGroupEntryExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            BindGroupEntryExtension::ExternalTextureBindingEntry(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::ExternalTextureBindingEntry as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            BindGroupEntryExtension::TexelBufferBindingEntry(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::TexelBufferBindingEntry as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum BindGroupLayoutDescriptorExtension {}
impl BindGroupLayoutDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum BindGroupLayoutEntryExtension {
    ExternalTextureBindingLayout(ExternalTextureBindingLayout),
    StaticSamplerBindingLayout(StaticSamplerBindingLayout),
    TexelBufferBindingLayout(TexelBufferBindingLayout),
}
impl std::convert::From<ExternalTextureBindingLayout> for BindGroupLayoutEntryExtension {
    fn from(ext: ExternalTextureBindingLayout) -> Self {
        BindGroupLayoutEntryExtension::ExternalTextureBindingLayout(ext)
    }
}
impl std::convert::From<StaticSamplerBindingLayout> for BindGroupLayoutEntryExtension {
    fn from(ext: StaticSamplerBindingLayout) -> Self {
        BindGroupLayoutEntryExtension::StaticSamplerBindingLayout(ext)
    }
}
impl std::convert::From<TexelBufferBindingLayout> for BindGroupLayoutEntryExtension {
    fn from(ext: TexelBufferBindingLayout) -> Self {
        BindGroupLayoutEntryExtension::TexelBufferBindingLayout(ext)
    }
}
impl BindGroupLayoutEntryExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            BindGroupLayoutEntryExtension::ExternalTextureBindingLayout(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::ExternalTextureBindingLayout as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            BindGroupLayoutEntryExtension::StaticSamplerBindingLayout(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::StaticSamplerBindingLayout as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            BindGroupLayoutEntryExtension::TexelBufferBindingLayout(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::TexelBufferBindingLayout as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum BindingResourceExtension {}
impl BindingResourceExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum BufferBindingLayoutExtension {}
impl BufferBindingLayoutExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum BufferDescriptorExtension {
    BufferHostMappedPointer(BufferHostMappedPointer),
    DawnBufferDescriptorErrorInfoFromWireClient(
        DawnBufferDescriptorErrorInfoFromWireClient,
    ),
    DawnFakeBufferOOMForTesting(DawnFakeBufferOOMForTesting),
}
impl std::convert::From<BufferHostMappedPointer> for BufferDescriptorExtension {
    fn from(ext: BufferHostMappedPointer) -> Self {
        BufferDescriptorExtension::BufferHostMappedPointer(ext)
    }
}
impl std::convert::From<DawnBufferDescriptorErrorInfoFromWireClient>
for BufferDescriptorExtension {
    fn from(ext: DawnBufferDescriptorErrorInfoFromWireClient) -> Self {
        BufferDescriptorExtension::DawnBufferDescriptorErrorInfoFromWireClient(ext)
    }
}
impl std::convert::From<DawnFakeBufferOOMForTesting> for BufferDescriptorExtension {
    fn from(ext: DawnFakeBufferOOMForTesting) -> Self {
        BufferDescriptorExtension::DawnFakeBufferOOMForTesting(ext)
    }
}
impl BufferDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            BufferDescriptorExtension::BufferHostMappedPointer(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::BufferHostMappedPointer as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            BufferDescriptorExtension::DawnBufferDescriptorErrorInfoFromWireClient(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnBufferDescriptorErrorInfoFromWireClient
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            BufferDescriptorExtension::DawnFakeBufferOOMForTesting(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnFakeBufferOOMForTesting as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum ColorTargetStateExtension {
    ColorTargetStateExpandResolveTextureDawn(ColorTargetStateExpandResolveTextureDawn),
}
impl std::convert::From<ColorTargetStateExpandResolveTextureDawn>
for ColorTargetStateExtension {
    fn from(ext: ColorTargetStateExpandResolveTextureDawn) -> Self {
        ColorTargetStateExtension::ColorTargetStateExpandResolveTextureDawn(ext)
    }
}
impl ColorTargetStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            ColorTargetStateExtension::ColorTargetStateExpandResolveTextureDawn(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::ColorTargetStateExpandResolveTextureDawn
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum CommandBufferDescriptorExtension {}
impl CommandBufferDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum CommandEncoderDescriptorExtension {
    DawnEncoderInternalUsageDescriptor(DawnEncoderInternalUsageDescriptor),
}
impl std::convert::From<DawnEncoderInternalUsageDescriptor>
for CommandEncoderDescriptorExtension {
    fn from(ext: DawnEncoderInternalUsageDescriptor) -> Self {
        CommandEncoderDescriptorExtension::DawnEncoderInternalUsageDescriptor(ext)
    }
}
impl CommandEncoderDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            CommandEncoderDescriptorExtension::DawnEncoderInternalUsageDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnEncoderInternalUsageDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum CompilationInfoExtension {}
impl CompilationInfoExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum CompilationMessageExtension {
    DawnCompilationMessageUtf16(DawnCompilationMessageUtf16),
}
impl std::convert::From<DawnCompilationMessageUtf16> for CompilationMessageExtension {
    fn from(ext: DawnCompilationMessageUtf16) -> Self {
        CompilationMessageExtension::DawnCompilationMessageUtf16(ext)
    }
}
impl CompilationMessageExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            CompilationMessageExtension::DawnCompilationMessageUtf16(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnCompilationMessageUtf16 as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum ComputePassDescriptorExtension {}
impl ComputePassDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum ComputePipelineDescriptorExtension {}
impl ComputePipelineDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum ComputeStateExtension {}
impl ComputeStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum ConstantEntryExtension {}
impl ConstantEntryExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum CopyTextureForBrowserOptionsExtension {}
impl CopyTextureForBrowserOptionsExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum DawnFormatCapabilitiesExtension {
    DawnDrmFormatCapabilities(DawnDrmFormatCapabilities),
}
impl std::convert::From<DawnDrmFormatCapabilities> for DawnFormatCapabilitiesExtension {
    fn from(ext: DawnDrmFormatCapabilities) -> Self {
        DawnFormatCapabilitiesExtension::DawnDrmFormatCapabilities(ext)
    }
}
impl DawnFormatCapabilitiesExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            DawnFormatCapabilitiesExtension::DawnDrmFormatCapabilities(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnDrmFormatCapabilities as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum DepthStencilStateExtension {}
impl DepthStencilStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum DeviceDescriptorExtension {
    DawnCacheDeviceDescriptor(DawnCacheDeviceDescriptor),
    DawnConsumeAdapterDescriptor(DawnConsumeAdapterDescriptor),
    DawnDeviceAllocatorControl(DawnDeviceAllocatorControl),
    DawnFakeDeviceInitializeErrorForTesting(DawnFakeDeviceInitializeErrorForTesting),
    DawnTogglesDescriptor(DawnTogglesDescriptor),
}
impl std::convert::From<DawnCacheDeviceDescriptor> for DeviceDescriptorExtension {
    fn from(ext: DawnCacheDeviceDescriptor) -> Self {
        DeviceDescriptorExtension::DawnCacheDeviceDescriptor(ext)
    }
}
impl std::convert::From<DawnConsumeAdapterDescriptor> for DeviceDescriptorExtension {
    fn from(ext: DawnConsumeAdapterDescriptor) -> Self {
        DeviceDescriptorExtension::DawnConsumeAdapterDescriptor(ext)
    }
}
impl std::convert::From<DawnDeviceAllocatorControl> for DeviceDescriptorExtension {
    fn from(ext: DawnDeviceAllocatorControl) -> Self {
        DeviceDescriptorExtension::DawnDeviceAllocatorControl(ext)
    }
}
impl std::convert::From<DawnFakeDeviceInitializeErrorForTesting>
for DeviceDescriptorExtension {
    fn from(ext: DawnFakeDeviceInitializeErrorForTesting) -> Self {
        DeviceDescriptorExtension::DawnFakeDeviceInitializeErrorForTesting(ext)
    }
}
impl std::convert::From<DawnTogglesDescriptor> for DeviceDescriptorExtension {
    fn from(ext: DawnTogglesDescriptor) -> Self {
        DeviceDescriptorExtension::DawnTogglesDescriptor(ext)
    }
}
impl DeviceDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            DeviceDescriptorExtension::DawnCacheDeviceDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnCacheDeviceDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            DeviceDescriptorExtension::DawnConsumeAdapterDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnConsumeAdapterDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            DeviceDescriptorExtension::DawnDeviceAllocatorControl(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnDeviceAllocatorControl as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            DeviceDescriptorExtension::DawnFakeDeviceInitializeErrorForTesting(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnFakeDeviceInitializeErrorForTesting
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            DeviceDescriptorExtension::DawnTogglesDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnTogglesDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum ExternalTextureDescriptorExtension {}
impl ExternalTextureDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum FragmentStateExtension {}
impl FragmentStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum ImageCopyExternalTextureExtension {}
impl ImageCopyExternalTextureExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum InstanceDescriptorExtension {
    DawnWGSLBlocklist(DawnWGSLBlocklist),
    DawnTogglesDescriptor(DawnTogglesDescriptor),
    DawnWireWGSLControl(DawnWireWGSLControl),
}
impl std::convert::From<DawnWGSLBlocklist> for InstanceDescriptorExtension {
    fn from(ext: DawnWGSLBlocklist) -> Self {
        InstanceDescriptorExtension::DawnWGSLBlocklist(ext)
    }
}
impl std::convert::From<DawnTogglesDescriptor> for InstanceDescriptorExtension {
    fn from(ext: DawnTogglesDescriptor) -> Self {
        InstanceDescriptorExtension::DawnTogglesDescriptor(ext)
    }
}
impl std::convert::From<DawnWireWGSLControl> for InstanceDescriptorExtension {
    fn from(ext: DawnWireWGSLControl) -> Self {
        InstanceDescriptorExtension::DawnWireWGSLControl(ext)
    }
}
impl InstanceDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            InstanceDescriptorExtension::DawnWGSLBlocklist(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnWGSLBlocklist as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            InstanceDescriptorExtension::DawnTogglesDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnTogglesDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            InstanceDescriptorExtension::DawnWireWGSLControl(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnWireWGSLControl as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum InstanceLimitsExtension {}
impl InstanceLimitsExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum LimitsExtension {
    DawnHostMappedPointerLimits(DawnHostMappedPointerLimits),
    DawnTexelCopyBufferRowAlignmentLimits(DawnTexelCopyBufferRowAlignmentLimits),
    ResourceTableLimits(ResourceTableLimits),
}
impl std::convert::From<DawnHostMappedPointerLimits> for LimitsExtension {
    fn from(ext: DawnHostMappedPointerLimits) -> Self {
        LimitsExtension::DawnHostMappedPointerLimits(ext)
    }
}
impl std::convert::From<DawnTexelCopyBufferRowAlignmentLimits> for LimitsExtension {
    fn from(ext: DawnTexelCopyBufferRowAlignmentLimits) -> Self {
        LimitsExtension::DawnTexelCopyBufferRowAlignmentLimits(ext)
    }
}
impl std::convert::From<ResourceTableLimits> for LimitsExtension {
    fn from(ext: ResourceTableLimits) -> Self {
        LimitsExtension::ResourceTableLimits(ext)
    }
}
impl LimitsExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            LimitsExtension::DawnHostMappedPointerLimits(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnHostMappedPointerLimits as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            LimitsExtension::DawnTexelCopyBufferRowAlignmentLimits(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnTexelCopyBufferRowAlignmentLimits
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            LimitsExtension::ResourceTableLimits(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::ResourceTableLimits as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum MultisampleStateExtension {}
impl MultisampleStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum PassTimestampWritesExtension {}
impl PassTimestampWritesExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum PipelineLayoutDescriptorExtension {
    PipelineLayoutPixelLocalStorage(PipelineLayoutPixelLocalStorage),
    PipelineLayoutResourceTable(PipelineLayoutResourceTable),
}
impl std::convert::From<PipelineLayoutPixelLocalStorage>
for PipelineLayoutDescriptorExtension {
    fn from(ext: PipelineLayoutPixelLocalStorage) -> Self {
        PipelineLayoutDescriptorExtension::PipelineLayoutPixelLocalStorage(ext)
    }
}
impl std::convert::From<PipelineLayoutResourceTable>
for PipelineLayoutDescriptorExtension {
    fn from(ext: PipelineLayoutResourceTable) -> Self {
        PipelineLayoutDescriptorExtension::PipelineLayoutResourceTable(ext)
    }
}
impl PipelineLayoutDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            PipelineLayoutDescriptorExtension::PipelineLayoutPixelLocalStorage(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::PipelineLayoutPixelLocalStorage
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            PipelineLayoutDescriptorExtension::PipelineLayoutResourceTable(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::PipelineLayoutResourceTable as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum PipelineLayoutStorageAttachmentExtension {}
impl PipelineLayoutStorageAttachmentExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum PrimitiveStateExtension {}
impl PrimitiveStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum QuerySetDescriptorExtension {}
impl QuerySetDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum QueueDescriptorExtension {}
impl QueueDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum RenderBundleDescriptorExtension {}
impl RenderBundleDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum RenderBundleEncoderDescriptorExtension {}
impl RenderBundleEncoderDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum RenderPassColorAttachmentExtension {
    DawnRenderPassColorAttachmentRenderToSingleSampled(
        DawnRenderPassColorAttachmentRenderToSingleSampled,
    ),
}
impl std::convert::From<DawnRenderPassColorAttachmentRenderToSingleSampled>
for RenderPassColorAttachmentExtension {
    fn from(ext: DawnRenderPassColorAttachmentRenderToSingleSampled) -> Self {
        RenderPassColorAttachmentExtension::DawnRenderPassColorAttachmentRenderToSingleSampled(
            ext,
        )
    }
}
impl RenderPassColorAttachmentExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            RenderPassColorAttachmentExtension::DawnRenderPassColorAttachmentRenderToSingleSampled(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnRenderPassColorAttachmentRenderToSingleSampled
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum RenderPassDepthStencilAttachmentExtension {}
impl RenderPassDepthStencilAttachmentExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum RenderPassDescriptorExtension {
    DawnRenderPassSampleCount(DawnRenderPassSampleCount),
    RenderPassDescriptorExpandResolveRect(RenderPassDescriptorExpandResolveRect),
    RenderPassDescriptorResolveRect(RenderPassDescriptorResolveRect),
    RenderPassMaxDrawCount(RenderPassMaxDrawCount),
    RenderPassPixelLocalStorage(RenderPassPixelLocalStorage),
}
impl std::convert::From<DawnRenderPassSampleCount> for RenderPassDescriptorExtension {
    fn from(ext: DawnRenderPassSampleCount) -> Self {
        RenderPassDescriptorExtension::DawnRenderPassSampleCount(ext)
    }
}
impl std::convert::From<RenderPassDescriptorExpandResolveRect>
for RenderPassDescriptorExtension {
    fn from(ext: RenderPassDescriptorExpandResolveRect) -> Self {
        RenderPassDescriptorExtension::RenderPassDescriptorExpandResolveRect(ext)
    }
}
impl std::convert::From<RenderPassDescriptorResolveRect>
for RenderPassDescriptorExtension {
    fn from(ext: RenderPassDescriptorResolveRect) -> Self {
        RenderPassDescriptorExtension::RenderPassDescriptorResolveRect(ext)
    }
}
impl std::convert::From<RenderPassMaxDrawCount> for RenderPassDescriptorExtension {
    fn from(ext: RenderPassMaxDrawCount) -> Self {
        RenderPassDescriptorExtension::RenderPassMaxDrawCount(ext)
    }
}
impl std::convert::From<RenderPassPixelLocalStorage> for RenderPassDescriptorExtension {
    fn from(ext: RenderPassPixelLocalStorage) -> Self {
        RenderPassDescriptorExtension::RenderPassPixelLocalStorage(ext)
    }
}
impl RenderPassDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            RenderPassDescriptorExtension::DawnRenderPassSampleCount(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnRenderPassSampleCount as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            RenderPassDescriptorExtension::RenderPassDescriptorExpandResolveRect(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::RenderPassDescriptorExpandResolveRect
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            RenderPassDescriptorExtension::RenderPassDescriptorResolveRect(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::RenderPassDescriptorResolveRect
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            RenderPassDescriptorExtension::RenderPassMaxDrawCount(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::RenderPassMaxDrawCount as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            RenderPassDescriptorExtension::RenderPassPixelLocalStorage(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::RenderPassPixelLocalStorage as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum RenderPassStorageAttachmentExtension {}
impl RenderPassStorageAttachmentExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum RenderPipelineDescriptorExtension {}
impl RenderPipelineDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum RequestAdapterOptionsExtension {
    DawnTogglesDescriptor(DawnTogglesDescriptor),
    RequestAdapterWebGPUBackendOptions(RequestAdapterWebGPUBackendOptions),
    RequestAdapterWebXROptions(RequestAdapterWebXROptions),
}
impl std::convert::From<DawnTogglesDescriptor> for RequestAdapterOptionsExtension {
    fn from(ext: DawnTogglesDescriptor) -> Self {
        RequestAdapterOptionsExtension::DawnTogglesDescriptor(ext)
    }
}
impl std::convert::From<RequestAdapterWebGPUBackendOptions>
for RequestAdapterOptionsExtension {
    fn from(ext: RequestAdapterWebGPUBackendOptions) -> Self {
        RequestAdapterOptionsExtension::RequestAdapterWebGPUBackendOptions(ext)
    }
}
impl std::convert::From<RequestAdapterWebXROptions> for RequestAdapterOptionsExtension {
    fn from(ext: RequestAdapterWebXROptions) -> Self {
        RequestAdapterOptionsExtension::RequestAdapterWebXROptions(ext)
    }
}
impl RequestAdapterOptionsExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            RequestAdapterOptionsExtension::DawnTogglesDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnTogglesDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            RequestAdapterOptionsExtension::RequestAdapterWebGPUBackendOptions(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::RequestAdapterWebGPUBackendOptions
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            RequestAdapterOptionsExtension::RequestAdapterWebXROptions(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::RequestAdapterWebXROptions as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum ResourceTableDescriptorExtension {}
impl ResourceTableDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SamplerBindingLayoutExtension {}
impl SamplerBindingLayoutExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SamplerDescriptorExtension {
    YCbCrVkDescriptor(YCbCrVkDescriptor),
}
impl std::convert::From<YCbCrVkDescriptor> for SamplerDescriptorExtension {
    fn from(ext: YCbCrVkDescriptor) -> Self {
        SamplerDescriptorExtension::YCbCrVkDescriptor(ext)
    }
}
impl SamplerDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SamplerDescriptorExtension::YCbCrVkDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::YCbCrVkDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum ShaderModuleDescriptorExtension {
    DawnShaderModuleSPIRVOptionsDescriptor(DawnShaderModuleSPIRVOptionsDescriptor),
    ShaderModuleCompilationOptions(ShaderModuleCompilationOptions),
    ShaderSourceSPIRV(ShaderSourceSPIRV),
    ShaderSourceWGSL(ShaderSourceWGSL),
}
impl std::convert::From<DawnShaderModuleSPIRVOptionsDescriptor>
for ShaderModuleDescriptorExtension {
    fn from(ext: DawnShaderModuleSPIRVOptionsDescriptor) -> Self {
        ShaderModuleDescriptorExtension::DawnShaderModuleSPIRVOptionsDescriptor(ext)
    }
}
impl std::convert::From<ShaderModuleCompilationOptions>
for ShaderModuleDescriptorExtension {
    fn from(ext: ShaderModuleCompilationOptions) -> Self {
        ShaderModuleDescriptorExtension::ShaderModuleCompilationOptions(ext)
    }
}
impl std::convert::From<ShaderSourceSPIRV> for ShaderModuleDescriptorExtension {
    fn from(ext: ShaderSourceSPIRV) -> Self {
        ShaderModuleDescriptorExtension::ShaderSourceSPIRV(ext)
    }
}
impl std::convert::From<ShaderSourceWGSL> for ShaderModuleDescriptorExtension {
    fn from(ext: ShaderSourceWGSL) -> Self {
        ShaderModuleDescriptorExtension::ShaderSourceWGSL(ext)
    }
}
impl ShaderModuleDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            ShaderModuleDescriptorExtension::DawnShaderModuleSPIRVOptionsDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnShaderModuleSPIRVOptionsDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            ShaderModuleDescriptorExtension::ShaderModuleCompilationOptions(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::ShaderModuleCompilationOptions
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            ShaderModuleDescriptorExtension::ShaderSourceSPIRV(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::ShaderSourceSPIRV as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            ShaderModuleDescriptorExtension::ShaderSourceWGSL(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::ShaderSourceWGSL as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum SharedBufferMemoryBeginAccessDescriptorExtension {}
impl SharedBufferMemoryBeginAccessDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SharedBufferMemoryDescriptorExtension {}
impl SharedBufferMemoryDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SharedBufferMemoryEndAccessStateExtension {}
impl SharedBufferMemoryEndAccessStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SharedBufferMemoryPropertiesExtension {}
impl SharedBufferMemoryPropertiesExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SharedFenceDescriptorExtension {
    SharedFenceDXGISharedHandleDescriptor(SharedFenceDXGISharedHandleDescriptor),
    SharedFenceEGLSyncDescriptor(SharedFenceEGLSyncDescriptor),
    SharedFenceMTLSharedEventDescriptor(SharedFenceMTLSharedEventDescriptor),
    SharedFenceSyncFDDescriptor(SharedFenceSyncFDDescriptor),
    SharedFenceVkSemaphoreOpaqueFDDescriptor(SharedFenceVkSemaphoreOpaqueFDDescriptor),
    SharedFenceVkSemaphoreZirconHandleDescriptor(
        SharedFenceVkSemaphoreZirconHandleDescriptor,
    ),
}
impl std::convert::From<SharedFenceDXGISharedHandleDescriptor>
for SharedFenceDescriptorExtension {
    fn from(ext: SharedFenceDXGISharedHandleDescriptor) -> Self {
        SharedFenceDescriptorExtension::SharedFenceDXGISharedHandleDescriptor(ext)
    }
}
impl std::convert::From<SharedFenceEGLSyncDescriptor>
for SharedFenceDescriptorExtension {
    fn from(ext: SharedFenceEGLSyncDescriptor) -> Self {
        SharedFenceDescriptorExtension::SharedFenceEGLSyncDescriptor(ext)
    }
}
impl std::convert::From<SharedFenceMTLSharedEventDescriptor>
for SharedFenceDescriptorExtension {
    fn from(ext: SharedFenceMTLSharedEventDescriptor) -> Self {
        SharedFenceDescriptorExtension::SharedFenceMTLSharedEventDescriptor(ext)
    }
}
impl std::convert::From<SharedFenceSyncFDDescriptor> for SharedFenceDescriptorExtension {
    fn from(ext: SharedFenceSyncFDDescriptor) -> Self {
        SharedFenceDescriptorExtension::SharedFenceSyncFDDescriptor(ext)
    }
}
impl std::convert::From<SharedFenceVkSemaphoreOpaqueFDDescriptor>
for SharedFenceDescriptorExtension {
    fn from(ext: SharedFenceVkSemaphoreOpaqueFDDescriptor) -> Self {
        SharedFenceDescriptorExtension::SharedFenceVkSemaphoreOpaqueFDDescriptor(ext)
    }
}
impl std::convert::From<SharedFenceVkSemaphoreZirconHandleDescriptor>
for SharedFenceDescriptorExtension {
    fn from(ext: SharedFenceVkSemaphoreZirconHandleDescriptor) -> Self {
        SharedFenceDescriptorExtension::SharedFenceVkSemaphoreZirconHandleDescriptor(ext)
    }
}
impl SharedFenceDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedFenceDescriptorExtension::SharedFenceDXGISharedHandleDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceDXGISharedHandleDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceDescriptorExtension::SharedFenceEGLSyncDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceEGLSyncDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceDescriptorExtension::SharedFenceMTLSharedEventDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceMTLSharedEventDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceDescriptorExtension::SharedFenceSyncFDDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceSyncFDDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceDescriptorExtension::SharedFenceVkSemaphoreOpaqueFDDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceVkSemaphoreOpaqueFDDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceDescriptorExtension::SharedFenceVkSemaphoreZirconHandleDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceVkSemaphoreZirconHandleDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum SharedFenceExportInfoExtension {
    SharedFenceDXGISharedHandleExportInfo(SharedFenceDXGISharedHandleExportInfo),
    SharedFenceEGLSyncExportInfo(SharedFenceEGLSyncExportInfo),
    SharedFenceMTLSharedEventExportInfo(SharedFenceMTLSharedEventExportInfo),
    SharedFenceSyncFDExportInfo(SharedFenceSyncFDExportInfo),
    SharedFenceVkSemaphoreOpaqueFDExportInfo(SharedFenceVkSemaphoreOpaqueFDExportInfo),
    SharedFenceVkSemaphoreZirconHandleExportInfo(
        SharedFenceVkSemaphoreZirconHandleExportInfo,
    ),
}
impl std::convert::From<SharedFenceDXGISharedHandleExportInfo>
for SharedFenceExportInfoExtension {
    fn from(ext: SharedFenceDXGISharedHandleExportInfo) -> Self {
        SharedFenceExportInfoExtension::SharedFenceDXGISharedHandleExportInfo(ext)
    }
}
impl std::convert::From<SharedFenceEGLSyncExportInfo>
for SharedFenceExportInfoExtension {
    fn from(ext: SharedFenceEGLSyncExportInfo) -> Self {
        SharedFenceExportInfoExtension::SharedFenceEGLSyncExportInfo(ext)
    }
}
impl std::convert::From<SharedFenceMTLSharedEventExportInfo>
for SharedFenceExportInfoExtension {
    fn from(ext: SharedFenceMTLSharedEventExportInfo) -> Self {
        SharedFenceExportInfoExtension::SharedFenceMTLSharedEventExportInfo(ext)
    }
}
impl std::convert::From<SharedFenceSyncFDExportInfo> for SharedFenceExportInfoExtension {
    fn from(ext: SharedFenceSyncFDExportInfo) -> Self {
        SharedFenceExportInfoExtension::SharedFenceSyncFDExportInfo(ext)
    }
}
impl std::convert::From<SharedFenceVkSemaphoreOpaqueFDExportInfo>
for SharedFenceExportInfoExtension {
    fn from(ext: SharedFenceVkSemaphoreOpaqueFDExportInfo) -> Self {
        SharedFenceExportInfoExtension::SharedFenceVkSemaphoreOpaqueFDExportInfo(ext)
    }
}
impl std::convert::From<SharedFenceVkSemaphoreZirconHandleExportInfo>
for SharedFenceExportInfoExtension {
    fn from(ext: SharedFenceVkSemaphoreZirconHandleExportInfo) -> Self {
        SharedFenceExportInfoExtension::SharedFenceVkSemaphoreZirconHandleExportInfo(ext)
    }
}
impl SharedFenceExportInfoExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedFenceExportInfoExtension::SharedFenceDXGISharedHandleExportInfo(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceDXGISharedHandleExportInfo
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceExportInfoExtension::SharedFenceEGLSyncExportInfo(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceEGLSyncExportInfo as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceExportInfoExtension::SharedFenceMTLSharedEventExportInfo(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceMTLSharedEventExportInfo
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceExportInfoExtension::SharedFenceSyncFDExportInfo(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceSyncFDExportInfo as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceExportInfoExtension::SharedFenceVkSemaphoreOpaqueFDExportInfo(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceVkSemaphoreOpaqueFDExportInfo
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedFenceExportInfoExtension::SharedFenceVkSemaphoreZirconHandleExportInfo(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedFenceVkSemaphoreZirconHandleExportInfo
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum SharedTextureMemoryBeginAccessDescriptorExtension {
    SharedTextureMemoryD3DSwapchainBeginState(SharedTextureMemoryD3DSwapchainBeginState),
    SharedTextureMemoryD3D11BeginState(SharedTextureMemoryD3D11BeginState),
    SharedTextureMemoryVkImageLayoutBeginState(
        SharedTextureMemoryVkImageLayoutBeginState,
    ),
}
impl std::convert::From<SharedTextureMemoryD3DSwapchainBeginState>
for SharedTextureMemoryBeginAccessDescriptorExtension {
    fn from(ext: SharedTextureMemoryD3DSwapchainBeginState) -> Self {
        SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryD3DSwapchainBeginState(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryD3D11BeginState>
for SharedTextureMemoryBeginAccessDescriptorExtension {
    fn from(ext: SharedTextureMemoryD3D11BeginState) -> Self {
        SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryD3D11BeginState(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryVkImageLayoutBeginState>
for SharedTextureMemoryBeginAccessDescriptorExtension {
    fn from(ext: SharedTextureMemoryVkImageLayoutBeginState) -> Self {
        SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryVkImageLayoutBeginState(
            ext,
        )
    }
}
impl SharedTextureMemoryBeginAccessDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryD3DSwapchainBeginState(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryD3DSwapchainBeginState
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryD3D11BeginState(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryD3D11BeginState
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryVkImageLayoutBeginState(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryVkImageLayoutBeginState
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum SharedTextureMemoryDescriptorExtension {
    SharedTextureMemoryDXGISharedHandleDescriptor(
        SharedTextureMemoryDXGISharedHandleDescriptor,
    ),
    SharedTextureMemoryEGLImageDescriptor(SharedTextureMemoryEGLImageDescriptor),
    SharedTextureMemoryIOSurfaceDescriptor(SharedTextureMemoryIOSurfaceDescriptor),
    SharedTextureMemoryAHardwareBufferDescriptor(
        SharedTextureMemoryAHardwareBufferDescriptor,
    ),
    SharedTextureMemoryDmaBufDescriptor(SharedTextureMemoryDmaBufDescriptor),
    SharedTextureMemoryOpaqueFDDescriptor(SharedTextureMemoryOpaqueFDDescriptor),
    SharedTextureMemoryVkDedicatedAllocationDescriptor(
        SharedTextureMemoryVkDedicatedAllocationDescriptor,
    ),
    SharedTextureMemoryZirconHandleDescriptor(SharedTextureMemoryZirconHandleDescriptor),
}
impl std::convert::From<SharedTextureMemoryDXGISharedHandleDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryDXGISharedHandleDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDXGISharedHandleDescriptor(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryEGLImageDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryEGLImageDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryEGLImageDescriptor(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryIOSurfaceDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryIOSurfaceDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryIOSurfaceDescriptor(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryAHardwareBufferDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryAHardwareBufferDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryAHardwareBufferDescriptor(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryDmaBufDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryDmaBufDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDmaBufDescriptor(ext)
    }
}
impl std::convert::From<SharedTextureMemoryOpaqueFDDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryOpaqueFDDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryOpaqueFDDescriptor(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryVkDedicatedAllocationDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryVkDedicatedAllocationDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryVkDedicatedAllocationDescriptor(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryZirconHandleDescriptor>
for SharedTextureMemoryDescriptorExtension {
    fn from(ext: SharedTextureMemoryZirconHandleDescriptor) -> Self {
        SharedTextureMemoryDescriptorExtension::SharedTextureMemoryZirconHandleDescriptor(
            ext,
        )
    }
}
impl SharedTextureMemoryDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDXGISharedHandleDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryDXGISharedHandleDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryEGLImageDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryEGLImageDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryIOSurfaceDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryIOSurfaceDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryAHardwareBufferDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryAHardwareBufferDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDmaBufDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryDmaBufDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryOpaqueFDDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryOpaqueFDDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryVkDedicatedAllocationDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryVkDedicatedAllocationDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryZirconHandleDescriptor(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryZirconHandleDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum SharedTextureMemoryEndAccessStateExtension {
    SharedTextureMemoryMetalEndAccessState(SharedTextureMemoryMetalEndAccessState),
    SharedTextureMemoryVkImageLayoutEndState(SharedTextureMemoryVkImageLayoutEndState),
}
impl std::convert::From<SharedTextureMemoryMetalEndAccessState>
for SharedTextureMemoryEndAccessStateExtension {
    fn from(ext: SharedTextureMemoryMetalEndAccessState) -> Self {
        SharedTextureMemoryEndAccessStateExtension::SharedTextureMemoryMetalEndAccessState(
            ext,
        )
    }
}
impl std::convert::From<SharedTextureMemoryVkImageLayoutEndState>
for SharedTextureMemoryEndAccessStateExtension {
    fn from(ext: SharedTextureMemoryVkImageLayoutEndState) -> Self {
        SharedTextureMemoryEndAccessStateExtension::SharedTextureMemoryVkImageLayoutEndState(
            ext,
        )
    }
}
impl SharedTextureMemoryEndAccessStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryEndAccessStateExtension::SharedTextureMemoryMetalEndAccessState(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryMetalEndAccessState
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SharedTextureMemoryEndAccessStateExtension::SharedTextureMemoryVkImageLayoutEndState(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryVkImageLayoutEndState
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum SharedTextureMemoryPropertiesExtension {
    SharedTextureMemoryAHardwareBufferProperties(
        SharedTextureMemoryAHardwareBufferProperties,
    ),
}
impl std::convert::From<SharedTextureMemoryAHardwareBufferProperties>
for SharedTextureMemoryPropertiesExtension {
    fn from(ext: SharedTextureMemoryAHardwareBufferProperties) -> Self {
        SharedTextureMemoryPropertiesExtension::SharedTextureMemoryAHardwareBufferProperties(
            ext,
        )
    }
}
impl SharedTextureMemoryPropertiesExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryPropertiesExtension::SharedTextureMemoryAHardwareBufferProperties(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SharedTextureMemoryAHardwareBufferProperties
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum StorageTextureBindingLayoutExtension {}
impl StorageTextureBindingLayoutExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SurfaceCapabilitiesExtension {}
impl SurfaceCapabilitiesExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SurfaceConfigurationExtension {}
impl SurfaceConfigurationExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum SurfaceDescriptorExtension {
    SurfaceColorManagement(SurfaceColorManagement),
    SurfaceDescriptorFromWindowsUWPSwapChainPanel(
        SurfaceDescriptorFromWindowsUWPSwapChainPanel,
    ),
    SurfaceDescriptorFromWindowsWinUISwapChainPanel(
        SurfaceDescriptorFromWindowsWinUISwapChainPanel,
    ),
    SurfaceDescriptorFromWindowsCoreWindow(SurfaceDescriptorFromWindowsCoreWindow),
    SurfaceSourceXCBWindow(SurfaceSourceXCBWindow),
    SurfaceSourceAndroidNativeWindow(SurfaceSourceAndroidNativeWindow),
    SurfaceSourceMetalLayer(SurfaceSourceMetalLayer),
    SurfaceSourceWaylandSurface(SurfaceSourceWaylandSurface),
    SurfaceSourceWindowsHWND(SurfaceSourceWindowsHWND),
    SurfaceSourceXlibWindow(SurfaceSourceXlibWindow),
}
impl std::convert::From<SurfaceColorManagement> for SurfaceDescriptorExtension {
    fn from(ext: SurfaceColorManagement) -> Self {
        SurfaceDescriptorExtension::SurfaceColorManagement(ext)
    }
}
impl std::convert::From<SurfaceDescriptorFromWindowsUWPSwapChainPanel>
for SurfaceDescriptorExtension {
    fn from(ext: SurfaceDescriptorFromWindowsUWPSwapChainPanel) -> Self {
        SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsUWPSwapChainPanel(ext)
    }
}
impl std::convert::From<SurfaceDescriptorFromWindowsWinUISwapChainPanel>
for SurfaceDescriptorExtension {
    fn from(ext: SurfaceDescriptorFromWindowsWinUISwapChainPanel) -> Self {
        SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsWinUISwapChainPanel(ext)
    }
}
impl std::convert::From<SurfaceDescriptorFromWindowsCoreWindow>
for SurfaceDescriptorExtension {
    fn from(ext: SurfaceDescriptorFromWindowsCoreWindow) -> Self {
        SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsCoreWindow(ext)
    }
}
impl std::convert::From<SurfaceSourceXCBWindow> for SurfaceDescriptorExtension {
    fn from(ext: SurfaceSourceXCBWindow) -> Self {
        SurfaceDescriptorExtension::SurfaceSourceXCBWindow(ext)
    }
}
impl std::convert::From<SurfaceSourceAndroidNativeWindow>
for SurfaceDescriptorExtension {
    fn from(ext: SurfaceSourceAndroidNativeWindow) -> Self {
        SurfaceDescriptorExtension::SurfaceSourceAndroidNativeWindow(ext)
    }
}
impl std::convert::From<SurfaceSourceMetalLayer> for SurfaceDescriptorExtension {
    fn from(ext: SurfaceSourceMetalLayer) -> Self {
        SurfaceDescriptorExtension::SurfaceSourceMetalLayer(ext)
    }
}
impl std::convert::From<SurfaceSourceWaylandSurface> for SurfaceDescriptorExtension {
    fn from(ext: SurfaceSourceWaylandSurface) -> Self {
        SurfaceDescriptorExtension::SurfaceSourceWaylandSurface(ext)
    }
}
impl std::convert::From<SurfaceSourceWindowsHWND> for SurfaceDescriptorExtension {
    fn from(ext: SurfaceSourceWindowsHWND) -> Self {
        SurfaceDescriptorExtension::SurfaceSourceWindowsHWND(ext)
    }
}
impl std::convert::From<SurfaceSourceXlibWindow> for SurfaceDescriptorExtension {
    fn from(ext: SurfaceSourceXlibWindow) -> Self {
        SurfaceDescriptorExtension::SurfaceSourceXlibWindow(ext)
    }
}
impl SurfaceDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SurfaceDescriptorExtension::SurfaceColorManagement(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceColorManagement as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsUWPSwapChainPanel(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceDescriptorFromWindowsUWPSwapChainPanel
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsWinUISwapChainPanel(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceDescriptorFromWindowsWinUISwapChainPanel
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsCoreWindow(
                value,
            ) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceDescriptorFromWindowsCoreWindow
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceSourceXCBWindow(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceSourceXCBWindow as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceSourceAndroidNativeWindow(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceSourceAndroidNativeWindow
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceSourceMetalLayer(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceSourceMetalLayer as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceSourceWaylandSurface(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceSourceWaylandSurface as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceSourceWindowsHWND(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceSourceWindowsHWND as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            SurfaceDescriptorExtension::SurfaceSourceXlibWindow(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::SurfaceSourceXlibWindow as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum SurfaceTextureExtension {}
impl SurfaceTextureExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum TexelBufferViewDescriptorExtension {}
impl TexelBufferViewDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum TextureBindingLayoutExtension {}
impl TextureBindingLayoutExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum TextureDescriptorExtension {
    DawnTextureInternalUsageDescriptor(DawnTextureInternalUsageDescriptor),
}
impl std::convert::From<DawnTextureInternalUsageDescriptor>
for TextureDescriptorExtension {
    fn from(ext: DawnTextureInternalUsageDescriptor) -> Self {
        TextureDescriptorExtension::DawnTextureInternalUsageDescriptor(ext)
    }
}
impl TextureDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            TextureDescriptorExtension::DawnTextureInternalUsageDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::DawnTextureInternalUsageDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum TextureViewDescriptorExtension {
    TextureComponentSwizzleDescriptor(TextureComponentSwizzleDescriptor),
    YCbCrVkDescriptor(YCbCrVkDescriptor),
}
impl std::convert::From<TextureComponentSwizzleDescriptor>
for TextureViewDescriptorExtension {
    fn from(ext: TextureComponentSwizzleDescriptor) -> Self {
        TextureViewDescriptorExtension::TextureComponentSwizzleDescriptor(ext)
    }
}
impl std::convert::From<YCbCrVkDescriptor> for TextureViewDescriptorExtension {
    fn from(ext: YCbCrVkDescriptor) -> Self {
        TextureViewDescriptorExtension::YCbCrVkDescriptor(ext)
    }
}
impl TextureViewDescriptorExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            TextureViewDescriptorExtension::TextureComponentSwizzleDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::TextureComponentSwizzleDescriptor
                    as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
            TextureViewDescriptorExtension::YCbCrVkDescriptor(value) => {
                let (mut raw, storage_value) = value.to_ffi();
                raw.chain.sType = SType::YCbCrVkDescriptor as ffi::WGPUSType;
                raw.chain.next = next;
                storage.push_storage(storage_value);
                let raw_ptr = storage.push_value_mut(raw);
                raw_ptr.cast::<ffi::WGPUChainedStruct>()
            }
        }
    }
}
#[allow(dead_code)]
pub enum VertexAttributeExtension {}
impl VertexAttributeExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum VertexBufferLayoutExtension {}
impl VertexBufferLayoutExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
#[allow(dead_code)]
pub enum VertexStateExtension {}
impl VertexStateExtension {
    pub(crate) fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}
