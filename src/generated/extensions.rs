#![allow(dead_code, unused_imports)]

use crate::ffi;
use super::*;

pub struct ChainedStructStorage {
    entries: Vec<Box<ffi::WGPUChainedStruct>>,
}

impl ChainedStructStorage {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn push(
        &mut self,
        s_type: ffi::WGPUSType,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let mut node = Box::new(ffi::WGPUChainedStruct { next, sType: s_type });
        let ptr = std::ptr::from_mut(node.as_mut());
        self.entries.push(node);
        ptr
    }
}

#[allow(dead_code)]
pub enum AdapterInfoExtension {
    AdapterPropertiesD3D(AdapterPropertiesD3D),
    AdapterPropertiesWGPU(AdapterPropertiesWGPU),
    AdapterPropertiesExplicitComputeSubgroupSizeConfigs(AdapterPropertiesExplicitComputeSubgroupSizeConfigs),
    AdapterPropertiesMemoryHeaps(AdapterPropertiesMemoryHeaps),
    AdapterPropertiesSubgroupMatrixConfigs(AdapterPropertiesSubgroupMatrixConfigs),
    AdapterPropertiesVk(AdapterPropertiesVk),
    DawnAdapterPropertiesPowerPreference(DawnAdapterPropertiesPowerPreference),
}

impl AdapterInfoExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            AdapterInfoExtension::AdapterPropertiesD3D(_) => storage.push(SType::AdapterPropertiesD3D as ffi::WGPUSType, next),
            AdapterInfoExtension::AdapterPropertiesWGPU(_) => storage.push(SType::AdapterPropertiesWGPU as ffi::WGPUSType, next),
            AdapterInfoExtension::AdapterPropertiesExplicitComputeSubgroupSizeConfigs(_) => storage.push(SType::AdapterPropertiesExplicitComputeSubgroupSizeConfigs as ffi::WGPUSType, next),
            AdapterInfoExtension::AdapterPropertiesMemoryHeaps(_) => storage.push(SType::AdapterPropertiesMemoryHeaps as ffi::WGPUSType, next),
            AdapterInfoExtension::AdapterPropertiesSubgroupMatrixConfigs(_) => storage.push(SType::AdapterPropertiesSubgroupMatrixConfigs as ffi::WGPUSType, next),
            AdapterInfoExtension::AdapterPropertiesVk(_) => storage.push(SType::AdapterPropertiesVk as ffi::WGPUSType, next),
            AdapterInfoExtension::DawnAdapterPropertiesPowerPreference(_) => storage.push(SType::DawnAdapterPropertiesPowerPreference as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum BindGroupDescriptorExtension {

}

impl BindGroupDescriptorExtension {
    pub fn push_chain(
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

impl BindGroupEntryExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            BindGroupEntryExtension::ExternalTextureBindingEntry(_) => storage.push(SType::ExternalTextureBindingEntry as ffi::WGPUSType, next),
            BindGroupEntryExtension::TexelBufferBindingEntry(_) => storage.push(SType::TexelBufferBindingEntry as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum BindGroupLayoutDescriptorExtension {

}

impl BindGroupLayoutDescriptorExtension {
    pub fn push_chain(
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

impl BindGroupLayoutEntryExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            BindGroupLayoutEntryExtension::ExternalTextureBindingLayout(_) => storage.push(SType::ExternalTextureBindingLayout as ffi::WGPUSType, next),
            BindGroupLayoutEntryExtension::StaticSamplerBindingLayout(_) => storage.push(SType::StaticSamplerBindingLayout as ffi::WGPUSType, next),
            BindGroupLayoutEntryExtension::TexelBufferBindingLayout(_) => storage.push(SType::TexelBufferBindingLayout as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum BindingResourceExtension {

}

impl BindingResourceExtension {
    pub fn push_chain(
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
pub enum BufferBindingLayoutExtension {

}

impl BufferBindingLayoutExtension {
    pub fn push_chain(
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
    DawnBufferDescriptorErrorInfoFromWireClient(DawnBufferDescriptorErrorInfoFromWireClient),
    DawnFakeBufferOOMForTesting(DawnFakeBufferOOMForTesting),
}

impl BufferDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            BufferDescriptorExtension::BufferHostMappedPointer(_) => storage.push(SType::BufferHostMappedPointer as ffi::WGPUSType, next),
            BufferDescriptorExtension::DawnBufferDescriptorErrorInfoFromWireClient(_) => storage.push(SType::DawnBufferDescriptorErrorInfoFromWireClient as ffi::WGPUSType, next),
            BufferDescriptorExtension::DawnFakeBufferOOMForTesting(_) => storage.push(SType::DawnFakeBufferOOMForTesting as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum ColorTargetStateExtension {
    ColorTargetStateExpandResolveTextureDawn(ColorTargetStateExpandResolveTextureDawn),
}

impl ColorTargetStateExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            ColorTargetStateExtension::ColorTargetStateExpandResolveTextureDawn(_) => storage.push(SType::ColorTargetStateExpandResolveTextureDawn as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum CommandBufferDescriptorExtension {

}

impl CommandBufferDescriptorExtension {
    pub fn push_chain(
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

impl CommandEncoderDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            CommandEncoderDescriptorExtension::DawnEncoderInternalUsageDescriptor(_) => storage.push(SType::DawnEncoderInternalUsageDescriptor as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum CompilationInfoExtension {

}

impl CompilationInfoExtension {
    pub fn push_chain(
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

impl CompilationMessageExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            CompilationMessageExtension::DawnCompilationMessageUtf16(_) => storage.push(SType::DawnCompilationMessageUtf16 as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum ComputePassDescriptorExtension {

}

impl ComputePassDescriptorExtension {
    pub fn push_chain(
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
pub enum ComputePipelineDescriptorExtension {

}

impl ComputePipelineDescriptorExtension {
    pub fn push_chain(
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
pub enum ComputeStateExtension {

}

impl ComputeStateExtension {
    pub fn push_chain(
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
pub enum ConstantEntryExtension {

}

impl ConstantEntryExtension {
    pub fn push_chain(
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
pub enum CopyTextureForBrowserOptionsExtension {

}

impl CopyTextureForBrowserOptionsExtension {
    pub fn push_chain(
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

impl DawnFormatCapabilitiesExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            DawnFormatCapabilitiesExtension::DawnDrmFormatCapabilities(_) => storage.push(SType::DawnDrmFormatCapabilities as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum DepthStencilStateExtension {

}

impl DepthStencilStateExtension {
    pub fn push_chain(
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

impl DeviceDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            DeviceDescriptorExtension::DawnCacheDeviceDescriptor(_) => storage.push(SType::DawnCacheDeviceDescriptor as ffi::WGPUSType, next),
            DeviceDescriptorExtension::DawnConsumeAdapterDescriptor(_) => storage.push(SType::DawnConsumeAdapterDescriptor as ffi::WGPUSType, next),
            DeviceDescriptorExtension::DawnDeviceAllocatorControl(_) => storage.push(SType::DawnDeviceAllocatorControl as ffi::WGPUSType, next),
            DeviceDescriptorExtension::DawnFakeDeviceInitializeErrorForTesting(_) => storage.push(SType::DawnFakeDeviceInitializeErrorForTesting as ffi::WGPUSType, next),
            DeviceDescriptorExtension::DawnTogglesDescriptor(_) => storage.push(SType::DawnTogglesDescriptor as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum ExternalTextureDescriptorExtension {

}

impl ExternalTextureDescriptorExtension {
    pub fn push_chain(
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
pub enum FragmentStateExtension {

}

impl FragmentStateExtension {
    pub fn push_chain(
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
pub enum ImageCopyExternalTextureExtension {

}

impl ImageCopyExternalTextureExtension {
    pub fn push_chain(
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

impl InstanceDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            InstanceDescriptorExtension::DawnWGSLBlocklist(_) => storage.push(SType::DawnWGSLBlocklist as ffi::WGPUSType, next),
            InstanceDescriptorExtension::DawnTogglesDescriptor(_) => storage.push(SType::DawnTogglesDescriptor as ffi::WGPUSType, next),
            InstanceDescriptorExtension::DawnWireWGSLControl(_) => storage.push(SType::DawnWireWGSLControl as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum InstanceLimitsExtension {

}

impl InstanceLimitsExtension {
    pub fn push_chain(
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

impl LimitsExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            LimitsExtension::DawnHostMappedPointerLimits(_) => storage.push(SType::DawnHostMappedPointerLimits as ffi::WGPUSType, next),
            LimitsExtension::DawnTexelCopyBufferRowAlignmentLimits(_) => storage.push(SType::DawnTexelCopyBufferRowAlignmentLimits as ffi::WGPUSType, next),
            LimitsExtension::ResourceTableLimits(_) => storage.push(SType::ResourceTableLimits as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum MultisampleStateExtension {

}

impl MultisampleStateExtension {
    pub fn push_chain(
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
pub enum PassTimestampWritesExtension {

}

impl PassTimestampWritesExtension {
    pub fn push_chain(
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

impl PipelineLayoutDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            PipelineLayoutDescriptorExtension::PipelineLayoutPixelLocalStorage(_) => storage.push(SType::PipelineLayoutPixelLocalStorage as ffi::WGPUSType, next),
            PipelineLayoutDescriptorExtension::PipelineLayoutResourceTable(_) => storage.push(SType::PipelineLayoutResourceTable as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum PipelineLayoutStorageAttachmentExtension {

}

impl PipelineLayoutStorageAttachmentExtension {
    pub fn push_chain(
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
pub enum PrimitiveStateExtension {

}

impl PrimitiveStateExtension {
    pub fn push_chain(
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
pub enum QuerySetDescriptorExtension {

}

impl QuerySetDescriptorExtension {
    pub fn push_chain(
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
pub enum QueueDescriptorExtension {

}

impl QueueDescriptorExtension {
    pub fn push_chain(
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
pub enum RenderBundleDescriptorExtension {

}

impl RenderBundleDescriptorExtension {
    pub fn push_chain(
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
pub enum RenderBundleEncoderDescriptorExtension {

}

impl RenderBundleEncoderDescriptorExtension {
    pub fn push_chain(
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
    DawnRenderPassColorAttachmentRenderToSingleSampled(DawnRenderPassColorAttachmentRenderToSingleSampled),
}

impl RenderPassColorAttachmentExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            RenderPassColorAttachmentExtension::DawnRenderPassColorAttachmentRenderToSingleSampled(_) => storage.push(SType::DawnRenderPassColorAttachmentRenderToSingleSampled as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum RenderPassDepthStencilAttachmentExtension {

}

impl RenderPassDepthStencilAttachmentExtension {
    pub fn push_chain(
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
    RenderPassDescriptorExpandResolveRect(RenderPassDescriptorExpandResolveRect),
    RenderPassDescriptorResolveRect(RenderPassDescriptorResolveRect),
    RenderPassMaxDrawCount(RenderPassMaxDrawCount),
    RenderPassPixelLocalStorage(RenderPassPixelLocalStorage),
}

impl RenderPassDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            RenderPassDescriptorExtension::RenderPassDescriptorExpandResolveRect(_) => storage.push(SType::RenderPassDescriptorExpandResolveRect as ffi::WGPUSType, next),
            RenderPassDescriptorExtension::RenderPassDescriptorResolveRect(_) => storage.push(SType::RenderPassDescriptorResolveRect as ffi::WGPUSType, next),
            RenderPassDescriptorExtension::RenderPassMaxDrawCount(_) => storage.push(SType::RenderPassMaxDrawCount as ffi::WGPUSType, next),
            RenderPassDescriptorExtension::RenderPassPixelLocalStorage(_) => storage.push(SType::RenderPassPixelLocalStorage as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum RenderPassStorageAttachmentExtension {

}

impl RenderPassStorageAttachmentExtension {
    pub fn push_chain(
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
pub enum RenderPipelineDescriptorExtension {

}

impl RenderPipelineDescriptorExtension {
    pub fn push_chain(
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

impl RequestAdapterOptionsExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            RequestAdapterOptionsExtension::DawnTogglesDescriptor(_) => storage.push(SType::DawnTogglesDescriptor as ffi::WGPUSType, next),
            RequestAdapterOptionsExtension::RequestAdapterWebGPUBackendOptions(_) => storage.push(SType::RequestAdapterWebGPUBackendOptions as ffi::WGPUSType, next),
            RequestAdapterOptionsExtension::RequestAdapterWebXROptions(_) => storage.push(SType::RequestAdapterWebXROptions as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum ResourceTableDescriptorExtension {

}

impl ResourceTableDescriptorExtension {
    pub fn push_chain(
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
pub enum SamplerBindingLayoutExtension {

}

impl SamplerBindingLayoutExtension {
    pub fn push_chain(
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

impl SamplerDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SamplerDescriptorExtension::YCbCrVkDescriptor(_) => storage.push(SType::YCbCrVkDescriptor as ffi::WGPUSType, next),
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

impl ShaderModuleDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            ShaderModuleDescriptorExtension::DawnShaderModuleSPIRVOptionsDescriptor(_) => storage.push(SType::DawnShaderModuleSPIRVOptionsDescriptor as ffi::WGPUSType, next),
            ShaderModuleDescriptorExtension::ShaderModuleCompilationOptions(_) => storage.push(SType::ShaderModuleCompilationOptions as ffi::WGPUSType, next),
            ShaderModuleDescriptorExtension::ShaderSourceSPIRV(_) => storage.push(SType::ShaderSourceSPIRV as ffi::WGPUSType, next),
            ShaderModuleDescriptorExtension::ShaderSourceWGSL(_) => storage.push(SType::ShaderSourceWGSL as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum SharedBufferMemoryBeginAccessDescriptorExtension {

}

impl SharedBufferMemoryBeginAccessDescriptorExtension {
    pub fn push_chain(
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
pub enum SharedBufferMemoryDescriptorExtension {

}

impl SharedBufferMemoryDescriptorExtension {
    pub fn push_chain(
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
pub enum SharedBufferMemoryEndAccessStateExtension {

}

impl SharedBufferMemoryEndAccessStateExtension {
    pub fn push_chain(
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
pub enum SharedBufferMemoryPropertiesExtension {

}

impl SharedBufferMemoryPropertiesExtension {
    pub fn push_chain(
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
    SharedFenceVkSemaphoreZirconHandleDescriptor(SharedFenceVkSemaphoreZirconHandleDescriptor),
}

impl SharedFenceDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedFenceDescriptorExtension::SharedFenceDXGISharedHandleDescriptor(_) => storage.push(SType::SharedFenceDXGISharedHandleDescriptor as ffi::WGPUSType, next),
            SharedFenceDescriptorExtension::SharedFenceEGLSyncDescriptor(_) => storage.push(SType::SharedFenceEGLSyncDescriptor as ffi::WGPUSType, next),
            SharedFenceDescriptorExtension::SharedFenceMTLSharedEventDescriptor(_) => storage.push(SType::SharedFenceMTLSharedEventDescriptor as ffi::WGPUSType, next),
            SharedFenceDescriptorExtension::SharedFenceSyncFDDescriptor(_) => storage.push(SType::SharedFenceSyncFDDescriptor as ffi::WGPUSType, next),
            SharedFenceDescriptorExtension::SharedFenceVkSemaphoreOpaqueFDDescriptor(_) => storage.push(SType::SharedFenceVkSemaphoreOpaqueFDDescriptor as ffi::WGPUSType, next),
            SharedFenceDescriptorExtension::SharedFenceVkSemaphoreZirconHandleDescriptor(_) => storage.push(SType::SharedFenceVkSemaphoreZirconHandleDescriptor as ffi::WGPUSType, next),
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
    SharedFenceVkSemaphoreZirconHandleExportInfo(SharedFenceVkSemaphoreZirconHandleExportInfo),
}

impl SharedFenceExportInfoExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedFenceExportInfoExtension::SharedFenceDXGISharedHandleExportInfo(_) => storage.push(SType::SharedFenceDXGISharedHandleExportInfo as ffi::WGPUSType, next),
            SharedFenceExportInfoExtension::SharedFenceEGLSyncExportInfo(_) => storage.push(SType::SharedFenceEGLSyncExportInfo as ffi::WGPUSType, next),
            SharedFenceExportInfoExtension::SharedFenceMTLSharedEventExportInfo(_) => storage.push(SType::SharedFenceMTLSharedEventExportInfo as ffi::WGPUSType, next),
            SharedFenceExportInfoExtension::SharedFenceSyncFDExportInfo(_) => storage.push(SType::SharedFenceSyncFDExportInfo as ffi::WGPUSType, next),
            SharedFenceExportInfoExtension::SharedFenceVkSemaphoreOpaqueFDExportInfo(_) => storage.push(SType::SharedFenceVkSemaphoreOpaqueFDExportInfo as ffi::WGPUSType, next),
            SharedFenceExportInfoExtension::SharedFenceVkSemaphoreZirconHandleExportInfo(_) => storage.push(SType::SharedFenceVkSemaphoreZirconHandleExportInfo as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum SharedTextureMemoryBeginAccessDescriptorExtension {
    SharedTextureMemoryD3DSwapchainBeginState(SharedTextureMemoryD3DSwapchainBeginState),
    SharedTextureMemoryD3D11BeginState(SharedTextureMemoryD3D11BeginState),
    SharedTextureMemoryVkImageLayoutBeginState(SharedTextureMemoryVkImageLayoutBeginState),
}

impl SharedTextureMemoryBeginAccessDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryD3DSwapchainBeginState(_) => storage.push(SType::SharedTextureMemoryD3DSwapchainBeginState as ffi::WGPUSType, next),
            SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryD3D11BeginState(_) => storage.push(SType::SharedTextureMemoryD3D11BeginState as ffi::WGPUSType, next),
            SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryVkImageLayoutBeginState(_) => storage.push(SType::SharedTextureMemoryVkImageLayoutBeginState as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum SharedTextureMemoryDescriptorExtension {
    SharedTextureMemoryDXGISharedHandleDescriptor(SharedTextureMemoryDXGISharedHandleDescriptor),
    SharedTextureMemoryEGLImageDescriptor(SharedTextureMemoryEGLImageDescriptor),
    SharedTextureMemoryIOSurfaceDescriptor(SharedTextureMemoryIOSurfaceDescriptor),
    SharedTextureMemoryAHardwareBufferDescriptor(SharedTextureMemoryAHardwareBufferDescriptor),
    SharedTextureMemoryDmaBufDescriptor(SharedTextureMemoryDmaBufDescriptor),
    SharedTextureMemoryOpaqueFDDescriptor(SharedTextureMemoryOpaqueFDDescriptor),
    SharedTextureMemoryVkDedicatedAllocationDescriptor(SharedTextureMemoryVkDedicatedAllocationDescriptor),
    SharedTextureMemoryZirconHandleDescriptor(SharedTextureMemoryZirconHandleDescriptor),
}

impl SharedTextureMemoryDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDXGISharedHandleDescriptor(_) => storage.push(SType::SharedTextureMemoryDXGISharedHandleDescriptor as ffi::WGPUSType, next),
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryEGLImageDescriptor(_) => storage.push(SType::SharedTextureMemoryEGLImageDescriptor as ffi::WGPUSType, next),
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryIOSurfaceDescriptor(_) => storage.push(SType::SharedTextureMemoryIOSurfaceDescriptor as ffi::WGPUSType, next),
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryAHardwareBufferDescriptor(_) => storage.push(SType::SharedTextureMemoryAHardwareBufferDescriptor as ffi::WGPUSType, next),
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDmaBufDescriptor(_) => storage.push(SType::SharedTextureMemoryDmaBufDescriptor as ffi::WGPUSType, next),
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryOpaqueFDDescriptor(_) => storage.push(SType::SharedTextureMemoryOpaqueFDDescriptor as ffi::WGPUSType, next),
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryVkDedicatedAllocationDescriptor(_) => storage.push(SType::SharedTextureMemoryVkDedicatedAllocationDescriptor as ffi::WGPUSType, next),
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryZirconHandleDescriptor(_) => storage.push(SType::SharedTextureMemoryZirconHandleDescriptor as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum SharedTextureMemoryEndAccessStateExtension {
    SharedTextureMemoryMetalEndAccessState(SharedTextureMemoryMetalEndAccessState),
    SharedTextureMemoryVkImageLayoutEndState(SharedTextureMemoryVkImageLayoutEndState),
}

impl SharedTextureMemoryEndAccessStateExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryEndAccessStateExtension::SharedTextureMemoryMetalEndAccessState(_) => storage.push(SType::SharedTextureMemoryMetalEndAccessState as ffi::WGPUSType, next),
            SharedTextureMemoryEndAccessStateExtension::SharedTextureMemoryVkImageLayoutEndState(_) => storage.push(SType::SharedTextureMemoryVkImageLayoutEndState as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum SharedTextureMemoryPropertiesExtension {
    SharedTextureMemoryAHardwareBufferProperties(SharedTextureMemoryAHardwareBufferProperties),
}

impl SharedTextureMemoryPropertiesExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SharedTextureMemoryPropertiesExtension::SharedTextureMemoryAHardwareBufferProperties(_) => storage.push(SType::SharedTextureMemoryAHardwareBufferProperties as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum StorageTextureBindingLayoutExtension {

}

impl StorageTextureBindingLayoutExtension {
    pub fn push_chain(
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
pub enum SurfaceCapabilitiesExtension {

}

impl SurfaceCapabilitiesExtension {
    pub fn push_chain(
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
pub enum SurfaceConfigurationExtension {

}

impl SurfaceConfigurationExtension {
    pub fn push_chain(
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
    SurfaceDescriptorFromWindowsUWPSwapChainPanel(SurfaceDescriptorFromWindowsUWPSwapChainPanel),
    SurfaceDescriptorFromWindowsWinUISwapChainPanel(SurfaceDescriptorFromWindowsWinUISwapChainPanel),
    SurfaceDescriptorFromWindowsCoreWindow(SurfaceDescriptorFromWindowsCoreWindow),
    SurfaceSourceXCBWindow(SurfaceSourceXCBWindow),
    SurfaceSourceAndroidNativeWindow(SurfaceSourceAndroidNativeWindow),
    SurfaceSourceMetalLayer(SurfaceSourceMetalLayer),
    SurfaceSourceWaylandSurface(SurfaceSourceWaylandSurface),
    SurfaceSourceWindowsHWND(SurfaceSourceWindowsHWND),
    SurfaceSourceXlibWindow(SurfaceSourceXlibWindow),
}

impl SurfaceDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            SurfaceDescriptorExtension::SurfaceColorManagement(_) => storage.push(SType::SurfaceColorManagement as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsUWPSwapChainPanel(_) => storage.push(SType::SurfaceDescriptorFromWindowsUWPSwapChainPanel as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsWinUISwapChainPanel(_) => storage.push(SType::SurfaceDescriptorFromWindowsWinUISwapChainPanel as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsCoreWindow(_) => storage.push(SType::SurfaceDescriptorFromWindowsCoreWindow as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceSourceXCBWindow(_) => storage.push(SType::SurfaceSourceXCBWindow as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceSourceAndroidNativeWindow(_) => storage.push(SType::SurfaceSourceAndroidNativeWindow as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceSourceMetalLayer(_) => storage.push(SType::SurfaceSourceMetalLayer as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceSourceWaylandSurface(_) => storage.push(SType::SurfaceSourceWaylandSurface as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceSourceWindowsHWND(_) => storage.push(SType::SurfaceSourceWindowsHWND as ffi::WGPUSType, next),
            SurfaceDescriptorExtension::SurfaceSourceXlibWindow(_) => storage.push(SType::SurfaceSourceXlibWindow as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum SurfaceTextureExtension {

}

impl SurfaceTextureExtension {
    pub fn push_chain(
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
pub enum TexelBufferViewDescriptorExtension {

}

impl TexelBufferViewDescriptorExtension {
    pub fn push_chain(
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
pub enum TextureBindingLayoutExtension {

}

impl TextureBindingLayoutExtension {
    pub fn push_chain(
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

impl TextureDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            TextureDescriptorExtension::DawnTextureInternalUsageDescriptor(_) => storage.push(SType::DawnTextureInternalUsageDescriptor as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum TextureViewDescriptorExtension {
    TextureComponentSwizzleDescriptor(TextureComponentSwizzleDescriptor),
    YCbCrVkDescriptor(YCbCrVkDescriptor),
}

impl TextureViewDescriptorExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        match self {
            TextureViewDescriptorExtension::TextureComponentSwizzleDescriptor(_) => storage.push(SType::TextureComponentSwizzleDescriptor as ffi::WGPUSType, next),
            TextureViewDescriptorExtension::YCbCrVkDescriptor(_) => storage.push(SType::YCbCrVkDescriptor as ffi::WGPUSType, next),
        }
    }
}


#[allow(dead_code)]
pub enum VertexAttributeExtension {

}

impl VertexAttributeExtension {
    pub fn push_chain(
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
pub enum VertexBufferLayoutExtension {

}

impl VertexBufferLayoutExtension {
    pub fn push_chain(
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
pub enum VertexStateExtension {

}

impl VertexStateExtension {
    pub fn push_chain(
        &self,
        storage: &mut ChainedStructStorage,
        next: *mut ffi::WGPUChainedStruct,
    ) -> *mut ffi::WGPUChainedStruct {
        let _ = self;
        let _ = storage;
        next
    }
}


