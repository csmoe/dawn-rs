#![allow(dead_code, unused_imports)]
use crate::ffi;
use bitflags::bitflags;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum WGSLLanguageFeatureName {
    ReadonlyAndReadwriteStorageTextures = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ReadonlyAndReadwriteStorageTextures
        as u32,
    Packed4X8IntegerDotProduct = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_Packed4x8IntegerDotProduct
        as u32,
    UnrestrictedPointerParameters = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UnrestrictedPointerParameters
        as u32,
    PointerCompositeAccess = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_PointerCompositeAccess
        as u32,
    UniformBufferStandardLayout = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UniformBufferStandardLayout
        as u32,
    SubgroupId = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SubgroupId
        as u32,
    TextureAndSamplerLet = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_TextureAndSamplerLet
        as u32,
    ChromiumTestingUnimplemented = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnimplemented
        as u32,
    ChromiumTestingUnsafeExperimental = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnsafeExperimental
        as u32,
    ChromiumTestingExperimental = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingExperimental
        as u32,
    ChromiumTestingShippedWithKillswitch = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShippedWithKillswitch
        as u32,
    ChromiumTestingShipped = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShipped
        as u32,
    SizedBindingArray = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SizedBindingArray
        as u32,
    TexelBuffers = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_TexelBuffers
        as u32,
    ChromiumPrint = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumPrint
        as u32,
    FragmentDepth = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_FragmentDepth
        as u32,
    ImmediateAddressSpace = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ImmediateAddressSpace
        as u32,
    SubgroupUniformity = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SubgroupUniformity
        as u32,
    BufferView = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_BufferView
        as u32,
    FilteringParameters = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_FilteringParameters
        as u32,
    SwizzleAssignment = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SwizzleAssignment
        as u32,
    LinearIndexing = ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_LinearIndexing
        as u32,
}
impl From<ffi::WGPUWGSLLanguageFeatureName> for WGSLLanguageFeatureName {
    fn from(value: ffi::WGPUWGSLLanguageFeatureName) -> Self {
        match value {
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ReadonlyAndReadwriteStorageTextures => {
                WGSLLanguageFeatureName::ReadonlyAndReadwriteStorageTextures
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_Packed4x8IntegerDotProduct => {
                WGSLLanguageFeatureName::Packed4X8IntegerDotProduct
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UnrestrictedPointerParameters => {
                WGSLLanguageFeatureName::UnrestrictedPointerParameters
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_PointerCompositeAccess => {
                WGSLLanguageFeatureName::PointerCompositeAccess
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UniformBufferStandardLayout => {
                WGSLLanguageFeatureName::UniformBufferStandardLayout
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SubgroupId => {
                WGSLLanguageFeatureName::SubgroupId
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_TextureAndSamplerLet => {
                WGSLLanguageFeatureName::TextureAndSamplerLet
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnimplemented => {
                WGSLLanguageFeatureName::ChromiumTestingUnimplemented
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnsafeExperimental => {
                WGSLLanguageFeatureName::ChromiumTestingUnsafeExperimental
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingExperimental => {
                WGSLLanguageFeatureName::ChromiumTestingExperimental
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShippedWithKillswitch => {
                WGSLLanguageFeatureName::ChromiumTestingShippedWithKillswitch
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShipped => {
                WGSLLanguageFeatureName::ChromiumTestingShipped
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SizedBindingArray => {
                WGSLLanguageFeatureName::SizedBindingArray
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_TexelBuffers => {
                WGSLLanguageFeatureName::TexelBuffers
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumPrint => {
                WGSLLanguageFeatureName::ChromiumPrint
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_FragmentDepth => {
                WGSLLanguageFeatureName::FragmentDepth
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ImmediateAddressSpace => {
                WGSLLanguageFeatureName::ImmediateAddressSpace
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SubgroupUniformity => {
                WGSLLanguageFeatureName::SubgroupUniformity
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_BufferView => {
                WGSLLanguageFeatureName::BufferView
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_FilteringParameters => {
                WGSLLanguageFeatureName::FilteringParameters
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SwizzleAssignment => {
                WGSLLanguageFeatureName::SwizzleAssignment
            }
            ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_LinearIndexing => {
                WGSLLanguageFeatureName::LinearIndexing
            }
            _ => WGSLLanguageFeatureName::ReadonlyAndReadwriteStorageTextures,
        }
    }
}
impl From<WGSLLanguageFeatureName> for ffi::WGPUWGSLLanguageFeatureName {
    fn from(value: WGSLLanguageFeatureName) -> Self {
        value as ffi::WGPUWGSLLanguageFeatureName
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AdapterType {
    DiscreteGpu = ffi::WGPUAdapterType_WGPUAdapterType_DiscreteGPU as u32,
    IntegratedGpu = ffi::WGPUAdapterType_WGPUAdapterType_IntegratedGPU as u32,
    Cpu = ffi::WGPUAdapterType_WGPUAdapterType_CPU as u32,
    Unknown = ffi::WGPUAdapterType_WGPUAdapterType_Unknown as u32,
}
impl From<ffi::WGPUAdapterType> for AdapterType {
    fn from(value: ffi::WGPUAdapterType) -> Self {
        match value {
            ffi::WGPUAdapterType_WGPUAdapterType_DiscreteGPU => AdapterType::DiscreteGpu,
            ffi::WGPUAdapterType_WGPUAdapterType_IntegratedGPU => AdapterType::IntegratedGpu,
            ffi::WGPUAdapterType_WGPUAdapterType_CPU => AdapterType::Cpu,
            ffi::WGPUAdapterType_WGPUAdapterType_Unknown => AdapterType::Unknown,
            _ => AdapterType::DiscreteGpu,
        }
    }
}
impl From<AdapterType> for ffi::WGPUAdapterType {
    fn from(value: AdapterType) -> Self {
        value as ffi::WGPUAdapterType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AddressMode {
    Undefined = ffi::WGPUAddressMode_WGPUAddressMode_Undefined as u32,
    ClampToEdge = ffi::WGPUAddressMode_WGPUAddressMode_ClampToEdge as u32,
    Repeat = ffi::WGPUAddressMode_WGPUAddressMode_Repeat as u32,
    MirrorRepeat = ffi::WGPUAddressMode_WGPUAddressMode_MirrorRepeat as u32,
}
impl From<ffi::WGPUAddressMode> for AddressMode {
    fn from(value: ffi::WGPUAddressMode) -> Self {
        match value {
            ffi::WGPUAddressMode_WGPUAddressMode_Undefined => AddressMode::Undefined,
            ffi::WGPUAddressMode_WGPUAddressMode_ClampToEdge => AddressMode::ClampToEdge,
            ffi::WGPUAddressMode_WGPUAddressMode_Repeat => AddressMode::Repeat,
            ffi::WGPUAddressMode_WGPUAddressMode_MirrorRepeat => AddressMode::MirrorRepeat,
            _ => AddressMode::Undefined,
        }
    }
}
impl From<AddressMode> for ffi::WGPUAddressMode {
    fn from(value: AddressMode) -> Self {
        value as ffi::WGPUAddressMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AlphaMode {
    Opaque = ffi::WGPUAlphaMode_WGPUAlphaMode_Opaque as u32,
    Premultiplied = ffi::WGPUAlphaMode_WGPUAlphaMode_Premultiplied as u32,
    Unpremultiplied = ffi::WGPUAlphaMode_WGPUAlphaMode_Unpremultiplied as u32,
}
impl From<ffi::WGPUAlphaMode> for AlphaMode {
    fn from(value: ffi::WGPUAlphaMode) -> Self {
        match value {
            ffi::WGPUAlphaMode_WGPUAlphaMode_Opaque => AlphaMode::Opaque,
            ffi::WGPUAlphaMode_WGPUAlphaMode_Premultiplied => AlphaMode::Premultiplied,
            ffi::WGPUAlphaMode_WGPUAlphaMode_Unpremultiplied => AlphaMode::Unpremultiplied,
            _ => AlphaMode::Opaque,
        }
    }
}
impl From<AlphaMode> for ffi::WGPUAlphaMode {
    fn from(value: AlphaMode) -> Self {
        value as ffi::WGPUAlphaMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BackendType {
    Undefined = ffi::WGPUBackendType_WGPUBackendType_Undefined as u32,
    Null = ffi::WGPUBackendType_WGPUBackendType_Null as u32,
    WebGPU = ffi::WGPUBackendType_WGPUBackendType_WebGPU as u32,
    D3D11 = ffi::WGPUBackendType_WGPUBackendType_D3D11 as u32,
    D3D12 = ffi::WGPUBackendType_WGPUBackendType_D3D12 as u32,
    Metal = ffi::WGPUBackendType_WGPUBackendType_Metal as u32,
    Vulkan = ffi::WGPUBackendType_WGPUBackendType_Vulkan as u32,
    OpenGL = ffi::WGPUBackendType_WGPUBackendType_OpenGL as u32,
    OpenGLes = ffi::WGPUBackendType_WGPUBackendType_OpenGLES as u32,
}
impl From<ffi::WGPUBackendType> for BackendType {
    fn from(value: ffi::WGPUBackendType) -> Self {
        match value {
            ffi::WGPUBackendType_WGPUBackendType_Undefined => BackendType::Undefined,
            ffi::WGPUBackendType_WGPUBackendType_Null => BackendType::Null,
            ffi::WGPUBackendType_WGPUBackendType_WebGPU => BackendType::WebGPU,
            ffi::WGPUBackendType_WGPUBackendType_D3D11 => BackendType::D3D11,
            ffi::WGPUBackendType_WGPUBackendType_D3D12 => BackendType::D3D12,
            ffi::WGPUBackendType_WGPUBackendType_Metal => BackendType::Metal,
            ffi::WGPUBackendType_WGPUBackendType_Vulkan => BackendType::Vulkan,
            ffi::WGPUBackendType_WGPUBackendType_OpenGL => BackendType::OpenGL,
            ffi::WGPUBackendType_WGPUBackendType_OpenGLES => BackendType::OpenGLes,
            _ => BackendType::Undefined,
        }
    }
}
impl From<BackendType> for ffi::WGPUBackendType {
    fn from(value: BackendType) -> Self {
        value as ffi::WGPUBackendType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BlendFactor {
    Undefined = ffi::WGPUBlendFactor_WGPUBlendFactor_Undefined as u32,
    Zero = ffi::WGPUBlendFactor_WGPUBlendFactor_Zero as u32,
    One = ffi::WGPUBlendFactor_WGPUBlendFactor_One as u32,
    Src = ffi::WGPUBlendFactor_WGPUBlendFactor_Src as u32,
    OneMinusSrc = ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc as u32,
    SrcAlpha = ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlpha as u32,
    OneMinusSrcAlpha = ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrcAlpha as u32,
    Dst = ffi::WGPUBlendFactor_WGPUBlendFactor_Dst as u32,
    OneMinusDst = ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDst as u32,
    DstAlpha = ffi::WGPUBlendFactor_WGPUBlendFactor_DstAlpha as u32,
    OneMinusDstAlpha = ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDstAlpha as u32,
    SrcAlphaSaturated = ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlphaSaturated as u32,
    Constant = ffi::WGPUBlendFactor_WGPUBlendFactor_Constant as u32,
    OneMinusConstant = ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusConstant as u32,
    Src1 = ffi::WGPUBlendFactor_WGPUBlendFactor_Src1 as u32,
    OneMinusSrc1 = ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1 as u32,
    Src1Alpha = ffi::WGPUBlendFactor_WGPUBlendFactor_Src1Alpha as u32,
    OneMinusSrc1Alpha = ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1Alpha as u32,
}
impl From<ffi::WGPUBlendFactor> for BlendFactor {
    fn from(value: ffi::WGPUBlendFactor) -> Self {
        match value {
            ffi::WGPUBlendFactor_WGPUBlendFactor_Undefined => BlendFactor::Undefined,
            ffi::WGPUBlendFactor_WGPUBlendFactor_Zero => BlendFactor::Zero,
            ffi::WGPUBlendFactor_WGPUBlendFactor_One => BlendFactor::One,
            ffi::WGPUBlendFactor_WGPUBlendFactor_Src => BlendFactor::Src,
            ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc => BlendFactor::OneMinusSrc,
            ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlpha => BlendFactor::SrcAlpha,
            ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrcAlpha => BlendFactor::OneMinusSrcAlpha,
            ffi::WGPUBlendFactor_WGPUBlendFactor_Dst => BlendFactor::Dst,
            ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDst => BlendFactor::OneMinusDst,
            ffi::WGPUBlendFactor_WGPUBlendFactor_DstAlpha => BlendFactor::DstAlpha,
            ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDstAlpha => BlendFactor::OneMinusDstAlpha,
            ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlphaSaturated => {
                BlendFactor::SrcAlphaSaturated
            }
            ffi::WGPUBlendFactor_WGPUBlendFactor_Constant => BlendFactor::Constant,
            ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusConstant => BlendFactor::OneMinusConstant,
            ffi::WGPUBlendFactor_WGPUBlendFactor_Src1 => BlendFactor::Src1,
            ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1 => BlendFactor::OneMinusSrc1,
            ffi::WGPUBlendFactor_WGPUBlendFactor_Src1Alpha => BlendFactor::Src1Alpha,
            ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1Alpha => {
                BlendFactor::OneMinusSrc1Alpha
            }
            _ => BlendFactor::Undefined,
        }
    }
}
impl From<BlendFactor> for ffi::WGPUBlendFactor {
    fn from(value: BlendFactor) -> Self {
        value as ffi::WGPUBlendFactor
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BlendOperation {
    Undefined = ffi::WGPUBlendOperation_WGPUBlendOperation_Undefined as u32,
    Add = ffi::WGPUBlendOperation_WGPUBlendOperation_Add as u32,
    Subtract = ffi::WGPUBlendOperation_WGPUBlendOperation_Subtract as u32,
    ReverseSubtract = ffi::WGPUBlendOperation_WGPUBlendOperation_ReverseSubtract as u32,
    Min = ffi::WGPUBlendOperation_WGPUBlendOperation_Min as u32,
    Max = ffi::WGPUBlendOperation_WGPUBlendOperation_Max as u32,
}
impl From<ffi::WGPUBlendOperation> for BlendOperation {
    fn from(value: ffi::WGPUBlendOperation) -> Self {
        match value {
            ffi::WGPUBlendOperation_WGPUBlendOperation_Undefined => BlendOperation::Undefined,
            ffi::WGPUBlendOperation_WGPUBlendOperation_Add => BlendOperation::Add,
            ffi::WGPUBlendOperation_WGPUBlendOperation_Subtract => BlendOperation::Subtract,
            ffi::WGPUBlendOperation_WGPUBlendOperation_ReverseSubtract => {
                BlendOperation::ReverseSubtract
            }
            ffi::WGPUBlendOperation_WGPUBlendOperation_Min => BlendOperation::Min,
            ffi::WGPUBlendOperation_WGPUBlendOperation_Max => BlendOperation::Max,
            _ => BlendOperation::Undefined,
        }
    }
}
impl From<BlendOperation> for ffi::WGPUBlendOperation {
    fn from(value: BlendOperation) -> Self {
        value as ffi::WGPUBlendOperation
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BufferBindingType {
    BindingNotUsed = ffi::WGPUBufferBindingType_WGPUBufferBindingType_BindingNotUsed as u32,
    Undefined = ffi::WGPUBufferBindingType_WGPUBufferBindingType_Undefined as u32,
    Uniform = ffi::WGPUBufferBindingType_WGPUBufferBindingType_Uniform as u32,
    Storage = ffi::WGPUBufferBindingType_WGPUBufferBindingType_Storage as u32,
    ReadOnlyStorage = ffi::WGPUBufferBindingType_WGPUBufferBindingType_ReadOnlyStorage as u32,
}
impl From<ffi::WGPUBufferBindingType> for BufferBindingType {
    fn from(value: ffi::WGPUBufferBindingType) -> Self {
        match value {
            ffi::WGPUBufferBindingType_WGPUBufferBindingType_BindingNotUsed => {
                BufferBindingType::BindingNotUsed
            }
            ffi::WGPUBufferBindingType_WGPUBufferBindingType_Undefined => {
                BufferBindingType::Undefined
            }
            ffi::WGPUBufferBindingType_WGPUBufferBindingType_Uniform => BufferBindingType::Uniform,
            ffi::WGPUBufferBindingType_WGPUBufferBindingType_Storage => BufferBindingType::Storage,
            ffi::WGPUBufferBindingType_WGPUBufferBindingType_ReadOnlyStorage => {
                BufferBindingType::ReadOnlyStorage
            }
            _ => BufferBindingType::BindingNotUsed,
        }
    }
}
impl From<BufferBindingType> for ffi::WGPUBufferBindingType {
    fn from(value: BufferBindingType) -> Self {
        value as ffi::WGPUBufferBindingType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BufferMapState {
    Unmapped = ffi::WGPUBufferMapState_WGPUBufferMapState_Unmapped as u32,
    Pending = ffi::WGPUBufferMapState_WGPUBufferMapState_Pending as u32,
    Mapped = ffi::WGPUBufferMapState_WGPUBufferMapState_Mapped as u32,
}
impl From<ffi::WGPUBufferMapState> for BufferMapState {
    fn from(value: ffi::WGPUBufferMapState) -> Self {
        match value {
            ffi::WGPUBufferMapState_WGPUBufferMapState_Unmapped => BufferMapState::Unmapped,
            ffi::WGPUBufferMapState_WGPUBufferMapState_Pending => BufferMapState::Pending,
            ffi::WGPUBufferMapState_WGPUBufferMapState_Mapped => BufferMapState::Mapped,
            _ => BufferMapState::Unmapped,
        }
    }
}
impl From<BufferMapState> for ffi::WGPUBufferMapState {
    fn from(value: BufferMapState) -> Self {
        value as ffi::WGPUBufferMapState
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CallbackMode {
    WaitAnyOnly = ffi::WGPUCallbackMode_WGPUCallbackMode_WaitAnyOnly as u32,
    AllowProcessEvents = ffi::WGPUCallbackMode_WGPUCallbackMode_AllowProcessEvents as u32,
    AllowSpontaneous = ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous as u32,
}
impl From<ffi::WGPUCallbackMode> for CallbackMode {
    fn from(value: ffi::WGPUCallbackMode) -> Self {
        match value {
            ffi::WGPUCallbackMode_WGPUCallbackMode_WaitAnyOnly => CallbackMode::WaitAnyOnly,
            ffi::WGPUCallbackMode_WGPUCallbackMode_AllowProcessEvents => {
                CallbackMode::AllowProcessEvents
            }
            ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous => {
                CallbackMode::AllowSpontaneous
            }
            _ => CallbackMode::WaitAnyOnly,
        }
    }
}
impl From<CallbackMode> for ffi::WGPUCallbackMode {
    fn from(value: CallbackMode) -> Self {
        value as ffi::WGPUCallbackMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CompareFunction {
    Undefined = ffi::WGPUCompareFunction_WGPUCompareFunction_Undefined as u32,
    Never = ffi::WGPUCompareFunction_WGPUCompareFunction_Never as u32,
    Less = ffi::WGPUCompareFunction_WGPUCompareFunction_Less as u32,
    Equal = ffi::WGPUCompareFunction_WGPUCompareFunction_Equal as u32,
    LessEqual = ffi::WGPUCompareFunction_WGPUCompareFunction_LessEqual as u32,
    Greater = ffi::WGPUCompareFunction_WGPUCompareFunction_Greater as u32,
    NotEqual = ffi::WGPUCompareFunction_WGPUCompareFunction_NotEqual as u32,
    GreaterEqual = ffi::WGPUCompareFunction_WGPUCompareFunction_GreaterEqual as u32,
    Always = ffi::WGPUCompareFunction_WGPUCompareFunction_Always as u32,
}
impl From<ffi::WGPUCompareFunction> for CompareFunction {
    fn from(value: ffi::WGPUCompareFunction) -> Self {
        match value {
            ffi::WGPUCompareFunction_WGPUCompareFunction_Undefined => CompareFunction::Undefined,
            ffi::WGPUCompareFunction_WGPUCompareFunction_Never => CompareFunction::Never,
            ffi::WGPUCompareFunction_WGPUCompareFunction_Less => CompareFunction::Less,
            ffi::WGPUCompareFunction_WGPUCompareFunction_Equal => CompareFunction::Equal,
            ffi::WGPUCompareFunction_WGPUCompareFunction_LessEqual => CompareFunction::LessEqual,
            ffi::WGPUCompareFunction_WGPUCompareFunction_Greater => CompareFunction::Greater,
            ffi::WGPUCompareFunction_WGPUCompareFunction_NotEqual => CompareFunction::NotEqual,
            ffi::WGPUCompareFunction_WGPUCompareFunction_GreaterEqual => {
                CompareFunction::GreaterEqual
            }
            ffi::WGPUCompareFunction_WGPUCompareFunction_Always => CompareFunction::Always,
            _ => CompareFunction::Undefined,
        }
    }
}
impl From<CompareFunction> for ffi::WGPUCompareFunction {
    fn from(value: CompareFunction) -> Self {
        value as ffi::WGPUCompareFunction
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CompilationInfoRequestStatus {
    Success = ffi::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_Success as u32,
    CallbackCancelled =
        ffi::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_CallbackCancelled
            as u32,
}
impl From<ffi::WGPUCompilationInfoRequestStatus> for CompilationInfoRequestStatus {
    fn from(value: ffi::WGPUCompilationInfoRequestStatus) -> Self {
        match value {
            ffi::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_Success => {
                CompilationInfoRequestStatus::Success
            }
            ffi::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_CallbackCancelled => {
                CompilationInfoRequestStatus::CallbackCancelled
            }
            _ => CompilationInfoRequestStatus::Success,
        }
    }
}
impl From<CompilationInfoRequestStatus> for ffi::WGPUCompilationInfoRequestStatus {
    fn from(value: CompilationInfoRequestStatus) -> Self {
        value as ffi::WGPUCompilationInfoRequestStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CompilationMessageType {
    Error = ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Error as u32,
    Warning = ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Warning as u32,
    Info = ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Info as u32,
}
impl From<ffi::WGPUCompilationMessageType> for CompilationMessageType {
    fn from(value: ffi::WGPUCompilationMessageType) -> Self {
        match value {
            ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Error => {
                CompilationMessageType::Error
            }
            ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Warning => {
                CompilationMessageType::Warning
            }
            ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Info => {
                CompilationMessageType::Info
            }
            _ => CompilationMessageType::Error,
        }
    }
}
impl From<CompilationMessageType> for ffi::WGPUCompilationMessageType {
    fn from(value: CompilationMessageType) -> Self {
        value as ffi::WGPUCompilationMessageType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ComponentSwizzle {
    Undefined = ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Undefined as u32,
    Zero = ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Zero as u32,
    One = ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_One as u32,
    R = ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_R as u32,
    G = ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_G as u32,
    B = ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_B as u32,
    A = ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_A as u32,
}
impl From<ffi::WGPUComponentSwizzle> for ComponentSwizzle {
    fn from(value: ffi::WGPUComponentSwizzle) -> Self {
        match value {
            ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Undefined => ComponentSwizzle::Undefined,
            ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Zero => ComponentSwizzle::Zero,
            ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_One => ComponentSwizzle::One,
            ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_R => ComponentSwizzle::R,
            ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_G => ComponentSwizzle::G,
            ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_B => ComponentSwizzle::B,
            ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_A => ComponentSwizzle::A,
            _ => ComponentSwizzle::Undefined,
        }
    }
}
impl From<ComponentSwizzle> for ffi::WGPUComponentSwizzle {
    fn from(value: ComponentSwizzle) -> Self {
        value as ffi::WGPUComponentSwizzle
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CompositeAlphaMode {
    Auto = ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Auto as u32,
    Opaque = ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Opaque as u32,
    Premultiplied = ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Premultiplied as u32,
    Unpremultiplied = ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Unpremultiplied as u32,
    Inherit = ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Inherit as u32,
}
impl From<ffi::WGPUCompositeAlphaMode> for CompositeAlphaMode {
    fn from(value: ffi::WGPUCompositeAlphaMode) -> Self {
        match value {
            ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Auto => CompositeAlphaMode::Auto,
            ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Opaque => CompositeAlphaMode::Opaque,
            ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Premultiplied => {
                CompositeAlphaMode::Premultiplied
            }
            ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Unpremultiplied => {
                CompositeAlphaMode::Unpremultiplied
            }
            ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Inherit => {
                CompositeAlphaMode::Inherit
            }
            _ => CompositeAlphaMode::Auto,
        }
    }
}
impl From<CompositeAlphaMode> for ffi::WGPUCompositeAlphaMode {
    fn from(value: CompositeAlphaMode) -> Self {
        value as ffi::WGPUCompositeAlphaMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CreatePipelineAsyncStatus {
    Success = ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_Success as u32,
    CallbackCancelled =
        ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_CallbackCancelled as u32,
    ValidationError =
        ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_ValidationError as u32,
    InternalError =
        ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_InternalError as u32,
}
impl From<ffi::WGPUCreatePipelineAsyncStatus> for CreatePipelineAsyncStatus {
    fn from(value: ffi::WGPUCreatePipelineAsyncStatus) -> Self {
        match value {
            ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_Success => {
                CreatePipelineAsyncStatus::Success
            }
            ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_CallbackCancelled => {
                CreatePipelineAsyncStatus::CallbackCancelled
            }
            ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_ValidationError => {
                CreatePipelineAsyncStatus::ValidationError
            }
            ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_InternalError => {
                CreatePipelineAsyncStatus::InternalError
            }
            _ => CreatePipelineAsyncStatus::Success,
        }
    }
}
impl From<CreatePipelineAsyncStatus> for ffi::WGPUCreatePipelineAsyncStatus {
    fn from(value: CreatePipelineAsyncStatus) -> Self {
        value as ffi::WGPUCreatePipelineAsyncStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CullMode {
    Undefined = ffi::WGPUCullMode_WGPUCullMode_Undefined as u32,
    None = ffi::WGPUCullMode_WGPUCullMode_None as u32,
    Front = ffi::WGPUCullMode_WGPUCullMode_Front as u32,
    Back = ffi::WGPUCullMode_WGPUCullMode_Back as u32,
}
impl From<ffi::WGPUCullMode> for CullMode {
    fn from(value: ffi::WGPUCullMode) -> Self {
        match value {
            ffi::WGPUCullMode_WGPUCullMode_Undefined => CullMode::Undefined,
            ffi::WGPUCullMode_WGPUCullMode_None => CullMode::None,
            ffi::WGPUCullMode_WGPUCullMode_Front => CullMode::Front,
            ffi::WGPUCullMode_WGPUCullMode_Back => CullMode::Back,
            _ => CullMode::Undefined,
        }
    }
}
impl From<CullMode> for ffi::WGPUCullMode {
    fn from(value: CullMode) -> Self {
        value as ffi::WGPUCullMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DeviceLostReason {
    Unknown = ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Unknown as u32,
    Destroyed = ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Destroyed as u32,
    CallbackCancelled = ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_CallbackCancelled as u32,
    FailedCreation = ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_FailedCreation as u32,
}
impl From<ffi::WGPUDeviceLostReason> for DeviceLostReason {
    fn from(value: ffi::WGPUDeviceLostReason) -> Self {
        match value {
            ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Unknown => DeviceLostReason::Unknown,
            ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Destroyed => DeviceLostReason::Destroyed,
            ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_CallbackCancelled => {
                DeviceLostReason::CallbackCancelled
            }
            ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_FailedCreation => {
                DeviceLostReason::FailedCreation
            }
            _ => DeviceLostReason::Unknown,
        }
    }
}
impl From<DeviceLostReason> for ffi::WGPUDeviceLostReason {
    fn from(value: DeviceLostReason) -> Self {
        value as ffi::WGPUDeviceLostReason
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ErrorFilter {
    Validation = ffi::WGPUErrorFilter_WGPUErrorFilter_Validation as u32,
    OutOfMemory = ffi::WGPUErrorFilter_WGPUErrorFilter_OutOfMemory as u32,
    Internal = ffi::WGPUErrorFilter_WGPUErrorFilter_Internal as u32,
}
impl From<ffi::WGPUErrorFilter> for ErrorFilter {
    fn from(value: ffi::WGPUErrorFilter) -> Self {
        match value {
            ffi::WGPUErrorFilter_WGPUErrorFilter_Validation => ErrorFilter::Validation,
            ffi::WGPUErrorFilter_WGPUErrorFilter_OutOfMemory => ErrorFilter::OutOfMemory,
            ffi::WGPUErrorFilter_WGPUErrorFilter_Internal => ErrorFilter::Internal,
            _ => ErrorFilter::Validation,
        }
    }
}
impl From<ErrorFilter> for ffi::WGPUErrorFilter {
    fn from(value: ErrorFilter) -> Self {
        value as ffi::WGPUErrorFilter
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ErrorType {
    NoError = ffi::WGPUErrorType_WGPUErrorType_NoError as u32,
    Validation = ffi::WGPUErrorType_WGPUErrorType_Validation as u32,
    OutOfMemory = ffi::WGPUErrorType_WGPUErrorType_OutOfMemory as u32,
    Internal = ffi::WGPUErrorType_WGPUErrorType_Internal as u32,
    Unknown = ffi::WGPUErrorType_WGPUErrorType_Unknown as u32,
}
impl From<ffi::WGPUErrorType> for ErrorType {
    fn from(value: ffi::WGPUErrorType) -> Self {
        match value {
            ffi::WGPUErrorType_WGPUErrorType_NoError => ErrorType::NoError,
            ffi::WGPUErrorType_WGPUErrorType_Validation => ErrorType::Validation,
            ffi::WGPUErrorType_WGPUErrorType_OutOfMemory => ErrorType::OutOfMemory,
            ffi::WGPUErrorType_WGPUErrorType_Internal => ErrorType::Internal,
            ffi::WGPUErrorType_WGPUErrorType_Unknown => ErrorType::Unknown,
            _ => ErrorType::NoError,
        }
    }
}
impl From<ErrorType> for ffi::WGPUErrorType {
    fn from(value: ErrorType) -> Self {
        value as ffi::WGPUErrorType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ExternalTextureRotation {
    Rotate0Degrees =
        ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate0Degrees as u32,
    Rotate90Degrees =
        ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate90Degrees as u32,
    Rotate180Degrees =
        ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate180Degrees as u32,
    Rotate270Degrees =
        ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate270Degrees as u32,
}
impl From<ffi::WGPUExternalTextureRotation> for ExternalTextureRotation {
    fn from(value: ffi::WGPUExternalTextureRotation) -> Self {
        match value {
            ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate0Degrees => {
                ExternalTextureRotation::Rotate0Degrees
            }
            ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate90Degrees => {
                ExternalTextureRotation::Rotate90Degrees
            }
            ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate180Degrees => {
                ExternalTextureRotation::Rotate180Degrees
            }
            ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate270Degrees => {
                ExternalTextureRotation::Rotate270Degrees
            }
            _ => ExternalTextureRotation::Rotate0Degrees,
        }
    }
}
impl From<ExternalTextureRotation> for ffi::WGPUExternalTextureRotation {
    fn from(value: ExternalTextureRotation) -> Self {
        value as ffi::WGPUExternalTextureRotation
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FeatureLevel {
    Undefined = ffi::WGPUFeatureLevel_WGPUFeatureLevel_Undefined as u32,
    Compatibility = ffi::WGPUFeatureLevel_WGPUFeatureLevel_Compatibility as u32,
    Core = ffi::WGPUFeatureLevel_WGPUFeatureLevel_Core as u32,
}
impl From<ffi::WGPUFeatureLevel> for FeatureLevel {
    fn from(value: ffi::WGPUFeatureLevel) -> Self {
        match value {
            ffi::WGPUFeatureLevel_WGPUFeatureLevel_Undefined => FeatureLevel::Undefined,
            ffi::WGPUFeatureLevel_WGPUFeatureLevel_Compatibility => FeatureLevel::Compatibility,
            ffi::WGPUFeatureLevel_WGPUFeatureLevel_Core => FeatureLevel::Core,
            _ => FeatureLevel::Undefined,
        }
    }
}
impl From<FeatureLevel> for ffi::WGPUFeatureLevel {
    fn from(value: FeatureLevel) -> Self {
        value as ffi::WGPUFeatureLevel
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FeatureName {
    CoreFeaturesAndLimits = ffi::WGPUFeatureName_WGPUFeatureName_CoreFeaturesAndLimits as u32,
    DepthClipControl = ffi::WGPUFeatureName_WGPUFeatureName_DepthClipControl as u32,
    Depth32FloatStencil8 = ffi::WGPUFeatureName_WGPUFeatureName_Depth32FloatStencil8 as u32,
    TextureCompressionBc = ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionBC as u32,
    TextureCompressionBcSliced3D =
        ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionBCSliced3D as u32,
    TextureCompressionEtc2 = ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionETC2 as u32,
    TextureCompressionAstc = ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTC as u32,
    TextureCompressionAstcSliced3D =
        ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTCSliced3D as u32,
    TimestampQuery = ffi::WGPUFeatureName_WGPUFeatureName_TimestampQuery as u32,
    IndirectFirstInstance = ffi::WGPUFeatureName_WGPUFeatureName_IndirectFirstInstance as u32,
    ShaderF16 = ffi::WGPUFeatureName_WGPUFeatureName_ShaderF16 as u32,
    Rg11B10UfloatRenderable = ffi::WGPUFeatureName_WGPUFeatureName_RG11B10UfloatRenderable as u32,
    Bgra8UnormStorage = ffi::WGPUFeatureName_WGPUFeatureName_BGRA8UnormStorage as u32,
    Float32Filterable = ffi::WGPUFeatureName_WGPUFeatureName_Float32Filterable as u32,
    Float32Blendable = ffi::WGPUFeatureName_WGPUFeatureName_Float32Blendable as u32,
    ClipDistances = ffi::WGPUFeatureName_WGPUFeatureName_ClipDistances as u32,
    DualSourceBlending = ffi::WGPUFeatureName_WGPUFeatureName_DualSourceBlending as u32,
    Subgroups = ffi::WGPUFeatureName_WGPUFeatureName_Subgroups as u32,
    TextureFormatsTier1 = ffi::WGPUFeatureName_WGPUFeatureName_TextureFormatsTier1 as u32,
    TextureFormatsTier2 = ffi::WGPUFeatureName_WGPUFeatureName_TextureFormatsTier2 as u32,
    PrimitiveIndex = ffi::WGPUFeatureName_WGPUFeatureName_PrimitiveIndex as u32,
    TextureComponentSwizzle = ffi::WGPUFeatureName_WGPUFeatureName_TextureComponentSwizzle as u32,
    DawnInternalUsages = ffi::WGPUFeatureName_WGPUFeatureName_DawnInternalUsages as u32,
    DawnMultiPlanarFormats = ffi::WGPUFeatureName_WGPUFeatureName_DawnMultiPlanarFormats as u32,
    DawnNative = ffi::WGPUFeatureName_WGPUFeatureName_DawnNative as u32,
    ChromiumExperimentalTimestampQueryInsidePasses =
        ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalTimestampQueryInsidePasses as u32,
    ImplicitDeviceSynchronization =
        ffi::WGPUFeatureName_WGPUFeatureName_ImplicitDeviceSynchronization as u32,
    TransientAttachments = ffi::WGPUFeatureName_WGPUFeatureName_TransientAttachments as u32,
    MsaaRenderToSingleSampled =
        ffi::WGPUFeatureName_WGPUFeatureName_MSAARenderToSingleSampled as u32,
    D3D11MultithreadProtected =
        ffi::WGPUFeatureName_WGPUFeatureName_D3D11MultithreadProtected as u32,
    AngleTextureSharing = ffi::WGPUFeatureName_WGPUFeatureName_ANGLETextureSharing as u32,
    PixelLocalStorageCoherent =
        ffi::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageCoherent as u32,
    PixelLocalStorageNonCoherent =
        ffi::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageNonCoherent as u32,
    Unorm16TextureFormats = ffi::WGPUFeatureName_WGPUFeatureName_Unorm16TextureFormats as u32,
    MultiPlanarFormatExtendedUsages =
        ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatExtendedUsages as u32,
    MultiPlanarFormatP010 = ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP010 as u32,
    HostMappedPointer = ffi::WGPUFeatureName_WGPUFeatureName_HostMappedPointer as u32,
    MultiPlanarRenderTargets = ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarRenderTargets as u32,
    MultiPlanarFormatNv12A = ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv12a as u32,
    FramebufferFetch = ffi::WGPUFeatureName_WGPUFeatureName_FramebufferFetch as u32,
    BufferMapExtendedUsages = ffi::WGPUFeatureName_WGPUFeatureName_BufferMapExtendedUsages as u32,
    AdapterPropertiesMemoryHeaps =
        ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesMemoryHeaps as u32,
    AdapterPropertiesD3D = ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesD3D as u32,
    AdapterPropertiesVk = ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesVk as u32,
    DawnFormatCapabilities = ffi::WGPUFeatureName_WGPUFeatureName_DawnFormatCapabilities as u32,
    DawnDrmFormatCapabilities =
        ffi::WGPUFeatureName_WGPUFeatureName_DawnDrmFormatCapabilities as u32,
    MultiPlanarFormatNv16 = ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv16 as u32,
    MultiPlanarFormatNv24 = ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv24 as u32,
    MultiPlanarFormatP210 = ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP210 as u32,
    MultiPlanarFormatP410 = ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP410 as u32,
    SharedTextureMemoryVkDedicatedAllocation =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryVkDedicatedAllocation as u32,
    SharedTextureMemoryAHardwareBuffer =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryAHardwareBuffer as u32,
    SharedTextureMemoryDmaBuf =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDmaBuf as u32,
    SharedTextureMemoryOpaqueFD =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryOpaqueFD as u32,
    SharedTextureMemoryZirconHandle =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryZirconHandle as u32,
    SharedTextureMemoryDXGISharedHandle =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDXGISharedHandle as u32,
    SharedTextureMemoryD3D11Texture2D =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryD3D11Texture2D as u32,
    SharedTextureMemoryIOSurface =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryIOSurface as u32,
    SharedTextureMemoryEGLImage =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryEGLImage as u32,
    SharedFenceVkSemaphoreOpaqueFD =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreOpaqueFD as u32,
    SharedFenceSyncFD = ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceSyncFD as u32,
    SharedFenceVkSemaphoreZirconHandle =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreZirconHandle as u32,
    SharedFenceDXGISharedHandle =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceDXGISharedHandle as u32,
    SharedFenceMTLSharedEvent =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceMTLSharedEvent as u32,
    SharedBufferMemoryD3D12Resource =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedBufferMemoryD3D12Resource as u32,
    StaticSamplers = ffi::WGPUFeatureName_WGPUFeatureName_StaticSamplers as u32,
    YCbCrVulkanSamplers = ffi::WGPUFeatureName_WGPUFeatureName_YCbCrVulkanSamplers as u32,
    ShaderModuleCompilationOptions =
        ffi::WGPUFeatureName_WGPUFeatureName_ShaderModuleCompilationOptions as u32,
    DawnLoadResolveTexture = ffi::WGPUFeatureName_WGPUFeatureName_DawnLoadResolveTexture as u32,
    DawnPartialLoadResolveTexture =
        ffi::WGPUFeatureName_WGPUFeatureName_DawnPartialLoadResolveTexture as u32,
    MultiDrawIndirect = ffi::WGPUFeatureName_WGPUFeatureName_MultiDrawIndirect as u32,
    DawnTexelCopyBufferRowAlignment =
        ffi::WGPUFeatureName_WGPUFeatureName_DawnTexelCopyBufferRowAlignment as u32,
    FlexibleTextureViews = ffi::WGPUFeatureName_WGPUFeatureName_FlexibleTextureViews as u32,
    ChromiumExperimentalSubgroupMatrix =
        ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSubgroupMatrix as u32,
    SharedFenceEGLSync = ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceEGLSync as u32,
    DawnDeviceAllocatorControl =
        ffi::WGPUFeatureName_WGPUFeatureName_DawnDeviceAllocatorControl as u32,
    AdapterPropertiesWGPU = ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesWGPU as u32,
    SharedBufferMemoryD3D12SharedMemoryFileMappingHandle =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedBufferMemoryD3D12SharedMemoryFileMappingHandle
            as u32,
    SharedTextureMemoryD3D12Resource =
        ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryD3D12Resource as u32,
    ChromiumExperimentalSamplingResourceTable =
        ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSamplingResourceTable as u32,
    ChromiumExperimentalSubgroupSizeControl =
        ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSubgroupSizeControl as u32,
    AtomicVec2UMinMax = ffi::WGPUFeatureName_WGPUFeatureName_AtomicVec2uMinMax as u32,
}
impl From<ffi::WGPUFeatureName> for FeatureName {
    fn from(value: ffi::WGPUFeatureName) -> Self {
        match value {
            ffi::WGPUFeatureName_WGPUFeatureName_CoreFeaturesAndLimits => {
                FeatureName::CoreFeaturesAndLimits
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DepthClipControl => {
                FeatureName::DepthClipControl
            }
            ffi::WGPUFeatureName_WGPUFeatureName_Depth32FloatStencil8 => {
                FeatureName::Depth32FloatStencil8
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionBC => {
                FeatureName::TextureCompressionBc
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionBCSliced3D => {
                FeatureName::TextureCompressionBcSliced3D
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionETC2 => {
                FeatureName::TextureCompressionEtc2
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTC => {
                FeatureName::TextureCompressionAstc
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTCSliced3D => {
                FeatureName::TextureCompressionAstcSliced3D
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TimestampQuery => {
                FeatureName::TimestampQuery
            }
            ffi::WGPUFeatureName_WGPUFeatureName_IndirectFirstInstance => {
                FeatureName::IndirectFirstInstance
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ShaderF16 => FeatureName::ShaderF16,
            ffi::WGPUFeatureName_WGPUFeatureName_RG11B10UfloatRenderable => {
                FeatureName::Rg11B10UfloatRenderable
            }
            ffi::WGPUFeatureName_WGPUFeatureName_BGRA8UnormStorage => {
                FeatureName::Bgra8UnormStorage
            }
            ffi::WGPUFeatureName_WGPUFeatureName_Float32Filterable => {
                FeatureName::Float32Filterable
            }
            ffi::WGPUFeatureName_WGPUFeatureName_Float32Blendable => {
                FeatureName::Float32Blendable
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ClipDistances => {
                FeatureName::ClipDistances
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DualSourceBlending => {
                FeatureName::DualSourceBlending
            }
            ffi::WGPUFeatureName_WGPUFeatureName_Subgroups => FeatureName::Subgroups,
            ffi::WGPUFeatureName_WGPUFeatureName_TextureFormatsTier1 => {
                FeatureName::TextureFormatsTier1
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TextureFormatsTier2 => {
                FeatureName::TextureFormatsTier2
            }
            ffi::WGPUFeatureName_WGPUFeatureName_PrimitiveIndex => {
                FeatureName::PrimitiveIndex
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TextureComponentSwizzle => {
                FeatureName::TextureComponentSwizzle
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnInternalUsages => {
                FeatureName::DawnInternalUsages
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnMultiPlanarFormats => {
                FeatureName::DawnMultiPlanarFormats
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnNative => FeatureName::DawnNative,
            ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalTimestampQueryInsidePasses => {
                FeatureName::ChromiumExperimentalTimestampQueryInsidePasses
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ImplicitDeviceSynchronization => {
                FeatureName::ImplicitDeviceSynchronization
            }
            ffi::WGPUFeatureName_WGPUFeatureName_TransientAttachments => {
                FeatureName::TransientAttachments
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MSAARenderToSingleSampled => {
                FeatureName::MsaaRenderToSingleSampled
            }
            ffi::WGPUFeatureName_WGPUFeatureName_D3D11MultithreadProtected => {
                FeatureName::D3D11MultithreadProtected
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ANGLETextureSharing => {
                FeatureName::AngleTextureSharing
            }
            ffi::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageCoherent => {
                FeatureName::PixelLocalStorageCoherent
            }
            ffi::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageNonCoherent => {
                FeatureName::PixelLocalStorageNonCoherent
            }
            ffi::WGPUFeatureName_WGPUFeatureName_Unorm16TextureFormats => {
                FeatureName::Unorm16TextureFormats
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatExtendedUsages => {
                FeatureName::MultiPlanarFormatExtendedUsages
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP010 => {
                FeatureName::MultiPlanarFormatP010
            }
            ffi::WGPUFeatureName_WGPUFeatureName_HostMappedPointer => {
                FeatureName::HostMappedPointer
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarRenderTargets => {
                FeatureName::MultiPlanarRenderTargets
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv12a => {
                FeatureName::MultiPlanarFormatNv12A
            }
            ffi::WGPUFeatureName_WGPUFeatureName_FramebufferFetch => {
                FeatureName::FramebufferFetch
            }
            ffi::WGPUFeatureName_WGPUFeatureName_BufferMapExtendedUsages => {
                FeatureName::BufferMapExtendedUsages
            }
            ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesMemoryHeaps => {
                FeatureName::AdapterPropertiesMemoryHeaps
            }
            ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesD3D => {
                FeatureName::AdapterPropertiesD3D
            }
            ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesVk => {
                FeatureName::AdapterPropertiesVk
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnFormatCapabilities => {
                FeatureName::DawnFormatCapabilities
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnDrmFormatCapabilities => {
                FeatureName::DawnDrmFormatCapabilities
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv16 => {
                FeatureName::MultiPlanarFormatNv16
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv24 => {
                FeatureName::MultiPlanarFormatNv24
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP210 => {
                FeatureName::MultiPlanarFormatP210
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP410 => {
                FeatureName::MultiPlanarFormatP410
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryVkDedicatedAllocation => {
                FeatureName::SharedTextureMemoryVkDedicatedAllocation
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryAHardwareBuffer => {
                FeatureName::SharedTextureMemoryAHardwareBuffer
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDmaBuf => {
                FeatureName::SharedTextureMemoryDmaBuf
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryOpaqueFD => {
                FeatureName::SharedTextureMemoryOpaqueFD
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryZirconHandle => {
                FeatureName::SharedTextureMemoryZirconHandle
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDXGISharedHandle => {
                FeatureName::SharedTextureMemoryDXGISharedHandle
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryD3D11Texture2D => {
                FeatureName::SharedTextureMemoryD3D11Texture2D
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryIOSurface => {
                FeatureName::SharedTextureMemoryIOSurface
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryEGLImage => {
                FeatureName::SharedTextureMemoryEGLImage
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreOpaqueFD => {
                FeatureName::SharedFenceVkSemaphoreOpaqueFD
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceSyncFD => {
                FeatureName::SharedFenceSyncFD
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreZirconHandle => {
                FeatureName::SharedFenceVkSemaphoreZirconHandle
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceDXGISharedHandle => {
                FeatureName::SharedFenceDXGISharedHandle
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceMTLSharedEvent => {
                FeatureName::SharedFenceMTLSharedEvent
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedBufferMemoryD3D12Resource => {
                FeatureName::SharedBufferMemoryD3D12Resource
            }
            ffi::WGPUFeatureName_WGPUFeatureName_StaticSamplers => {
                FeatureName::StaticSamplers
            }
            ffi::WGPUFeatureName_WGPUFeatureName_YCbCrVulkanSamplers => {
                FeatureName::YCbCrVulkanSamplers
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ShaderModuleCompilationOptions => {
                FeatureName::ShaderModuleCompilationOptions
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnLoadResolveTexture => {
                FeatureName::DawnLoadResolveTexture
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnPartialLoadResolveTexture => {
                FeatureName::DawnPartialLoadResolveTexture
            }
            ffi::WGPUFeatureName_WGPUFeatureName_MultiDrawIndirect => {
                FeatureName::MultiDrawIndirect
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnTexelCopyBufferRowAlignment => {
                FeatureName::DawnTexelCopyBufferRowAlignment
            }
            ffi::WGPUFeatureName_WGPUFeatureName_FlexibleTextureViews => {
                FeatureName::FlexibleTextureViews
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSubgroupMatrix => {
                FeatureName::ChromiumExperimentalSubgroupMatrix
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceEGLSync => {
                FeatureName::SharedFenceEGLSync
            }
            ffi::WGPUFeatureName_WGPUFeatureName_DawnDeviceAllocatorControl => {
                FeatureName::DawnDeviceAllocatorControl
            }
            ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesWGPU => {
                FeatureName::AdapterPropertiesWGPU
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedBufferMemoryD3D12SharedMemoryFileMappingHandle => {
                FeatureName::SharedBufferMemoryD3D12SharedMemoryFileMappingHandle
            }
            ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryD3D12Resource => {
                FeatureName::SharedTextureMemoryD3D12Resource
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSamplingResourceTable => {
                FeatureName::ChromiumExperimentalSamplingResourceTable
            }
            ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSubgroupSizeControl => {
                FeatureName::ChromiumExperimentalSubgroupSizeControl
            }
            ffi::WGPUFeatureName_WGPUFeatureName_AtomicVec2uMinMax => {
                FeatureName::AtomicVec2UMinMax
            }
            _ => FeatureName::CoreFeaturesAndLimits,
        }
    }
}
impl From<FeatureName> for ffi::WGPUFeatureName {
    fn from(value: FeatureName) -> Self {
        value as ffi::WGPUFeatureName
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FilterMode {
    Undefined = ffi::WGPUFilterMode_WGPUFilterMode_Undefined as u32,
    Nearest = ffi::WGPUFilterMode_WGPUFilterMode_Nearest as u32,
    Linear = ffi::WGPUFilterMode_WGPUFilterMode_Linear as u32,
}
impl From<ffi::WGPUFilterMode> for FilterMode {
    fn from(value: ffi::WGPUFilterMode) -> Self {
        match value {
            ffi::WGPUFilterMode_WGPUFilterMode_Undefined => FilterMode::Undefined,
            ffi::WGPUFilterMode_WGPUFilterMode_Nearest => FilterMode::Nearest,
            ffi::WGPUFilterMode_WGPUFilterMode_Linear => FilterMode::Linear,
            _ => FilterMode::Undefined,
        }
    }
}
impl From<FilterMode> for ffi::WGPUFilterMode {
    fn from(value: FilterMode) -> Self {
        value as ffi::WGPUFilterMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FrontFace {
    Undefined = ffi::WGPUFrontFace_WGPUFrontFace_Undefined as u32,
    Ccw = ffi::WGPUFrontFace_WGPUFrontFace_CCW as u32,
    Cw = ffi::WGPUFrontFace_WGPUFrontFace_CW as u32,
}
impl From<ffi::WGPUFrontFace> for FrontFace {
    fn from(value: ffi::WGPUFrontFace) -> Self {
        match value {
            ffi::WGPUFrontFace_WGPUFrontFace_Undefined => FrontFace::Undefined,
            ffi::WGPUFrontFace_WGPUFrontFace_CCW => FrontFace::Ccw,
            ffi::WGPUFrontFace_WGPUFrontFace_CW => FrontFace::Cw,
            _ => FrontFace::Undefined,
        }
    }
}
impl From<FrontFace> for ffi::WGPUFrontFace {
    fn from(value: FrontFace) -> Self {
        value as ffi::WGPUFrontFace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IndexFormat {
    Undefined = ffi::WGPUIndexFormat_WGPUIndexFormat_Undefined as u32,
    Uint16 = ffi::WGPUIndexFormat_WGPUIndexFormat_Uint16 as u32,
    Uint32 = ffi::WGPUIndexFormat_WGPUIndexFormat_Uint32 as u32,
}
impl From<ffi::WGPUIndexFormat> for IndexFormat {
    fn from(value: ffi::WGPUIndexFormat) -> Self {
        match value {
            ffi::WGPUIndexFormat_WGPUIndexFormat_Undefined => IndexFormat::Undefined,
            ffi::WGPUIndexFormat_WGPUIndexFormat_Uint16 => IndexFormat::Uint16,
            ffi::WGPUIndexFormat_WGPUIndexFormat_Uint32 => IndexFormat::Uint32,
            _ => IndexFormat::Undefined,
        }
    }
}
impl From<IndexFormat> for ffi::WGPUIndexFormat {
    fn from(value: IndexFormat) -> Self {
        value as ffi::WGPUIndexFormat
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum InstanceFeatureName {
    TimedWaitAny = ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_TimedWaitAny as u32,
    ShaderSourceSPIRV =
        ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_ShaderSourceSPIRV as u32,
    MultipleDevicesPerAdapter =
        ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_MultipleDevicesPerAdapter as u32,
}
impl From<ffi::WGPUInstanceFeatureName> for InstanceFeatureName {
    fn from(value: ffi::WGPUInstanceFeatureName) -> Self {
        match value {
            ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_TimedWaitAny => {
                InstanceFeatureName::TimedWaitAny
            }
            ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_ShaderSourceSPIRV => {
                InstanceFeatureName::ShaderSourceSPIRV
            }
            ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_MultipleDevicesPerAdapter => {
                InstanceFeatureName::MultipleDevicesPerAdapter
            }
            _ => InstanceFeatureName::TimedWaitAny,
        }
    }
}
impl From<InstanceFeatureName> for ffi::WGPUInstanceFeatureName {
    fn from(value: InstanceFeatureName) -> Self {
        value as ffi::WGPUInstanceFeatureName
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LoadOp {
    Undefined = ffi::WGPULoadOp_WGPULoadOp_Undefined as u32,
    Load = ffi::WGPULoadOp_WGPULoadOp_Load as u32,
    Clear = ffi::WGPULoadOp_WGPULoadOp_Clear as u32,
    ExpandResolveTexture = ffi::WGPULoadOp_WGPULoadOp_ExpandResolveTexture as u32,
}
impl From<ffi::WGPULoadOp> for LoadOp {
    fn from(value: ffi::WGPULoadOp) -> Self {
        match value {
            ffi::WGPULoadOp_WGPULoadOp_Undefined => LoadOp::Undefined,
            ffi::WGPULoadOp_WGPULoadOp_Load => LoadOp::Load,
            ffi::WGPULoadOp_WGPULoadOp_Clear => LoadOp::Clear,
            ffi::WGPULoadOp_WGPULoadOp_ExpandResolveTexture => LoadOp::ExpandResolveTexture,
            _ => LoadOp::Undefined,
        }
    }
}
impl From<LoadOp> for ffi::WGPULoadOp {
    fn from(value: LoadOp) -> Self {
        value as ffi::WGPULoadOp
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LoggingType {
    Verbose = ffi::WGPULoggingType_WGPULoggingType_Verbose as u32,
    Info = ffi::WGPULoggingType_WGPULoggingType_Info as u32,
    Warning = ffi::WGPULoggingType_WGPULoggingType_Warning as u32,
    Error = ffi::WGPULoggingType_WGPULoggingType_Error as u32,
}
impl From<ffi::WGPULoggingType> for LoggingType {
    fn from(value: ffi::WGPULoggingType) -> Self {
        match value {
            ffi::WGPULoggingType_WGPULoggingType_Verbose => LoggingType::Verbose,
            ffi::WGPULoggingType_WGPULoggingType_Info => LoggingType::Info,
            ffi::WGPULoggingType_WGPULoggingType_Warning => LoggingType::Warning,
            ffi::WGPULoggingType_WGPULoggingType_Error => LoggingType::Error,
            _ => LoggingType::Verbose,
        }
    }
}
impl From<LoggingType> for ffi::WGPULoggingType {
    fn from(value: LoggingType) -> Self {
        value as ffi::WGPULoggingType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MapAsyncStatus {
    Success = ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Success as u32,
    CallbackCancelled = ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_CallbackCancelled as u32,
    Error = ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Error as u32,
    Aborted = ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Aborted as u32,
}
impl From<ffi::WGPUMapAsyncStatus> for MapAsyncStatus {
    fn from(value: ffi::WGPUMapAsyncStatus) -> Self {
        match value {
            ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Success => MapAsyncStatus::Success,
            ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_CallbackCancelled => {
                MapAsyncStatus::CallbackCancelled
            }
            ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Error => MapAsyncStatus::Error,
            ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Aborted => MapAsyncStatus::Aborted,
            _ => MapAsyncStatus::Success,
        }
    }
}
impl From<MapAsyncStatus> for ffi::WGPUMapAsyncStatus {
    fn from(value: MapAsyncStatus) -> Self {
        value as ffi::WGPUMapAsyncStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MipmapFilterMode {
    Undefined = ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Undefined as u32,
    Nearest = ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Nearest as u32,
    Linear = ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Linear as u32,
}
impl From<ffi::WGPUMipmapFilterMode> for MipmapFilterMode {
    fn from(value: ffi::WGPUMipmapFilterMode) -> Self {
        match value {
            ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Undefined => MipmapFilterMode::Undefined,
            ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Nearest => MipmapFilterMode::Nearest,
            ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Linear => MipmapFilterMode::Linear,
            _ => MipmapFilterMode::Undefined,
        }
    }
}
impl From<MipmapFilterMode> for ffi::WGPUMipmapFilterMode {
    fn from(value: MipmapFilterMode) -> Self {
        value as ffi::WGPUMipmapFilterMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum OptionalBool {
    False = ffi::WGPUOptionalBool_WGPUOptionalBool_False as u32,
    True = ffi::WGPUOptionalBool_WGPUOptionalBool_True as u32,
    Undefined = ffi::WGPUOptionalBool_WGPUOptionalBool_Undefined as u32,
}
impl From<ffi::WGPUOptionalBool> for OptionalBool {
    fn from(value: ffi::WGPUOptionalBool) -> Self {
        match value {
            ffi::WGPUOptionalBool_WGPUOptionalBool_False => OptionalBool::False,
            ffi::WGPUOptionalBool_WGPUOptionalBool_True => OptionalBool::True,
            ffi::WGPUOptionalBool_WGPUOptionalBool_Undefined => OptionalBool::Undefined,
            _ => OptionalBool::False,
        }
    }
}
impl From<OptionalBool> for ffi::WGPUOptionalBool {
    fn from(value: OptionalBool) -> Self {
        value as ffi::WGPUOptionalBool
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PopErrorScopeStatus {
    Success = ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Success as u32,
    CallbackCancelled =
        ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_CallbackCancelled as u32,
    Error = ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Error as u32,
}
impl From<ffi::WGPUPopErrorScopeStatus> for PopErrorScopeStatus {
    fn from(value: ffi::WGPUPopErrorScopeStatus) -> Self {
        match value {
            ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Success => {
                PopErrorScopeStatus::Success
            }
            ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_CallbackCancelled => {
                PopErrorScopeStatus::CallbackCancelled
            }
            ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Error => {
                PopErrorScopeStatus::Error
            }
            _ => PopErrorScopeStatus::Success,
        }
    }
}
impl From<PopErrorScopeStatus> for ffi::WGPUPopErrorScopeStatus {
    fn from(value: PopErrorScopeStatus) -> Self {
        value as ffi::WGPUPopErrorScopeStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PowerPreference {
    Undefined = ffi::WGPUPowerPreference_WGPUPowerPreference_Undefined as u32,
    LowPower = ffi::WGPUPowerPreference_WGPUPowerPreference_LowPower as u32,
    HighPerformance = ffi::WGPUPowerPreference_WGPUPowerPreference_HighPerformance as u32,
}
impl From<ffi::WGPUPowerPreference> for PowerPreference {
    fn from(value: ffi::WGPUPowerPreference) -> Self {
        match value {
            ffi::WGPUPowerPreference_WGPUPowerPreference_Undefined => PowerPreference::Undefined,
            ffi::WGPUPowerPreference_WGPUPowerPreference_LowPower => PowerPreference::LowPower,
            ffi::WGPUPowerPreference_WGPUPowerPreference_HighPerformance => {
                PowerPreference::HighPerformance
            }
            _ => PowerPreference::Undefined,
        }
    }
}
impl From<PowerPreference> for ffi::WGPUPowerPreference {
    fn from(value: PowerPreference) -> Self {
        value as ffi::WGPUPowerPreference
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PredefinedColorSpace {
    SRgb = ffi::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_SRGB as u32,
    DisplayP3 = ffi::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_DisplayP3 as u32,
}
impl From<ffi::WGPUPredefinedColorSpace> for PredefinedColorSpace {
    fn from(value: ffi::WGPUPredefinedColorSpace) -> Self {
        match value {
            ffi::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_SRGB => {
                PredefinedColorSpace::SRgb
            }
            ffi::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_DisplayP3 => {
                PredefinedColorSpace::DisplayP3
            }
            _ => PredefinedColorSpace::SRgb,
        }
    }
}
impl From<PredefinedColorSpace> for ffi::WGPUPredefinedColorSpace {
    fn from(value: PredefinedColorSpace) -> Self {
        value as ffi::WGPUPredefinedColorSpace
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PresentMode {
    Undefined = ffi::WGPUPresentMode_WGPUPresentMode_Undefined as u32,
    Fifo = ffi::WGPUPresentMode_WGPUPresentMode_Fifo as u32,
    FifoRelaxed = ffi::WGPUPresentMode_WGPUPresentMode_FifoRelaxed as u32,
    Immediate = ffi::WGPUPresentMode_WGPUPresentMode_Immediate as u32,
    Mailbox = ffi::WGPUPresentMode_WGPUPresentMode_Mailbox as u32,
}
impl From<ffi::WGPUPresentMode> for PresentMode {
    fn from(value: ffi::WGPUPresentMode) -> Self {
        match value {
            ffi::WGPUPresentMode_WGPUPresentMode_Undefined => PresentMode::Undefined,
            ffi::WGPUPresentMode_WGPUPresentMode_Fifo => PresentMode::Fifo,
            ffi::WGPUPresentMode_WGPUPresentMode_FifoRelaxed => PresentMode::FifoRelaxed,
            ffi::WGPUPresentMode_WGPUPresentMode_Immediate => PresentMode::Immediate,
            ffi::WGPUPresentMode_WGPUPresentMode_Mailbox => PresentMode::Mailbox,
            _ => PresentMode::Undefined,
        }
    }
}
impl From<PresentMode> for ffi::WGPUPresentMode {
    fn from(value: PresentMode) -> Self {
        value as ffi::WGPUPresentMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PrimitiveTopology {
    Undefined = ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_Undefined as u32,
    PointList = ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_PointList as u32,
    LineList = ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineList as u32,
    LineStrip = ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineStrip as u32,
    TriangleList = ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleList as u32,
    TriangleStrip = ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleStrip as u32,
}
impl From<ffi::WGPUPrimitiveTopology> for PrimitiveTopology {
    fn from(value: ffi::WGPUPrimitiveTopology) -> Self {
        match value {
            ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_Undefined => {
                PrimitiveTopology::Undefined
            }
            ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_PointList => {
                PrimitiveTopology::PointList
            }
            ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineList => {
                PrimitiveTopology::LineList
            }
            ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineStrip => {
                PrimitiveTopology::LineStrip
            }
            ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleList => {
                PrimitiveTopology::TriangleList
            }
            ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleStrip => {
                PrimitiveTopology::TriangleStrip
            }
            _ => PrimitiveTopology::Undefined,
        }
    }
}
impl From<PrimitiveTopology> for ffi::WGPUPrimitiveTopology {
    fn from(value: PrimitiveTopology) -> Self {
        value as ffi::WGPUPrimitiveTopology
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum QueryType {
    Occlusion = ffi::WGPUQueryType_WGPUQueryType_Occlusion as u32,
    Timestamp = ffi::WGPUQueryType_WGPUQueryType_Timestamp as u32,
}
impl From<ffi::WGPUQueryType> for QueryType {
    fn from(value: ffi::WGPUQueryType) -> Self {
        match value {
            ffi::WGPUQueryType_WGPUQueryType_Occlusion => QueryType::Occlusion,
            ffi::WGPUQueryType_WGPUQueryType_Timestamp => QueryType::Timestamp,
            _ => QueryType::Occlusion,
        }
    }
}
impl From<QueryType> for ffi::WGPUQueryType {
    fn from(value: QueryType) -> Self {
        value as ffi::WGPUQueryType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum QueueWorkDoneStatus {
    Success = ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Success as u32,
    CallbackCancelled =
        ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_CallbackCancelled as u32,
    Error = ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Error as u32,
}
impl From<ffi::WGPUQueueWorkDoneStatus> for QueueWorkDoneStatus {
    fn from(value: ffi::WGPUQueueWorkDoneStatus) -> Self {
        match value {
            ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Success => {
                QueueWorkDoneStatus::Success
            }
            ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_CallbackCancelled => {
                QueueWorkDoneStatus::CallbackCancelled
            }
            ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Error => {
                QueueWorkDoneStatus::Error
            }
            _ => QueueWorkDoneStatus::Success,
        }
    }
}
impl From<QueueWorkDoneStatus> for ffi::WGPUQueueWorkDoneStatus {
    fn from(value: QueueWorkDoneStatus) -> Self {
        value as ffi::WGPUQueueWorkDoneStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum RequestAdapterStatus {
    Success = ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Success as u32,
    CallbackCancelled =
        ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_CallbackCancelled as u32,
    Unavailable = ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Unavailable as u32,
    Error = ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Error as u32,
}
impl From<ffi::WGPURequestAdapterStatus> for RequestAdapterStatus {
    fn from(value: ffi::WGPURequestAdapterStatus) -> Self {
        match value {
            ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Success => {
                RequestAdapterStatus::Success
            }
            ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_CallbackCancelled => {
                RequestAdapterStatus::CallbackCancelled
            }
            ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Unavailable => {
                RequestAdapterStatus::Unavailable
            }
            ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Error => {
                RequestAdapterStatus::Error
            }
            _ => RequestAdapterStatus::Success,
        }
    }
}
impl From<RequestAdapterStatus> for ffi::WGPURequestAdapterStatus {
    fn from(value: RequestAdapterStatus) -> Self {
        value as ffi::WGPURequestAdapterStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum RequestDeviceStatus {
    Success = ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Success as u32,
    CallbackCancelled =
        ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_CallbackCancelled as u32,
    Error = ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Error as u32,
}
impl From<ffi::WGPURequestDeviceStatus> for RequestDeviceStatus {
    fn from(value: ffi::WGPURequestDeviceStatus) -> Self {
        match value {
            ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Success => {
                RequestDeviceStatus::Success
            }
            ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_CallbackCancelled => {
                RequestDeviceStatus::CallbackCancelled
            }
            ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Error => {
                RequestDeviceStatus::Error
            }
            _ => RequestDeviceStatus::Success,
        }
    }
}
impl From<RequestDeviceStatus> for ffi::WGPURequestDeviceStatus {
    fn from(value: RequestDeviceStatus) -> Self {
        value as ffi::WGPURequestDeviceStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SType {
    ShaderSourceSPIRV = ffi::WGPUSType_WGPUSType_ShaderSourceSPIRV as u32,
    ShaderSourceWGSL = ffi::WGPUSType_WGPUSType_ShaderSourceWGSL as u32,
    RenderPassMaxDrawCount = ffi::WGPUSType_WGPUSType_RenderPassMaxDrawCount as u32,
    SurfaceSourceMetalLayer = ffi::WGPUSType_WGPUSType_SurfaceSourceMetalLayer as u32,
    SurfaceSourceWindowsHWND = ffi::WGPUSType_WGPUSType_SurfaceSourceWindowsHWND as u32,
    SurfaceSourceXlibWindow = ffi::WGPUSType_WGPUSType_SurfaceSourceXlibWindow as u32,
    SurfaceSourceWaylandSurface = ffi::WGPUSType_WGPUSType_SurfaceSourceWaylandSurface as u32,
    SurfaceSourceAndroidNativeWindow =
        ffi::WGPUSType_WGPUSType_SurfaceSourceAndroidNativeWindow as u32,
    SurfaceSourceXCBWindow = ffi::WGPUSType_WGPUSType_SurfaceSourceXCBWindow as u32,
    SurfaceColorManagement = ffi::WGPUSType_WGPUSType_SurfaceColorManagement as u32,
    RequestAdapterWebXROptions = ffi::WGPUSType_WGPUSType_RequestAdapterWebXROptions as u32,
    TextureComponentSwizzleDescriptor =
        ffi::WGPUSType_WGPUSType_TextureComponentSwizzleDescriptor as u32,
    CompatibilityModeLimits = ffi::WGPUSType_WGPUSType_CompatibilityModeLimits as u32,
    TextureBindingViewDimensionDescriptor =
        ffi::WGPUSType_WGPUSType_TextureBindingViewDimensionDescriptor as u32,
    EmscriptenSurfaceSourceCanvasHTMLSelector =
        ffi::WGPUSType_WGPUSType_EmscriptenSurfaceSourceCanvasHTMLSelector as u32,
    SurfaceDescriptorFromWindowsCoreWindow =
        ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsCoreWindow as u32,
    ExternalTextureBindingEntry = ffi::WGPUSType_WGPUSType_ExternalTextureBindingEntry as u32,
    ExternalTextureBindingLayout = ffi::WGPUSType_WGPUSType_ExternalTextureBindingLayout as u32,
    SurfaceDescriptorFromWindowsUWPSwapChainPanel =
        ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsUWPSwapChainPanel as u32,
    DawnTextureInternalUsageDescriptor =
        ffi::WGPUSType_WGPUSType_DawnTextureInternalUsageDescriptor as u32,
    DawnEncoderInternalUsageDescriptor =
        ffi::WGPUSType_WGPUSType_DawnEncoderInternalUsageDescriptor as u32,
    DawnInstanceDescriptor = ffi::WGPUSType_WGPUSType_DawnInstanceDescriptor as u32,
    DawnCacheDeviceDescriptor = ffi::WGPUSType_WGPUSType_DawnCacheDeviceDescriptor as u32,
    DawnAdapterPropertiesPowerPreference =
        ffi::WGPUSType_WGPUSType_DawnAdapterPropertiesPowerPreference as u32,
    DawnBufferDescriptorErrorInfoFromWireClient =
        ffi::WGPUSType_WGPUSType_DawnBufferDescriptorErrorInfoFromWireClient as u32,
    DawnTogglesDescriptor = ffi::WGPUSType_WGPUSType_DawnTogglesDescriptor as u32,
    DawnShaderModuleSPIRVOptionsDescriptor =
        ffi::WGPUSType_WGPUSType_DawnShaderModuleSPIRVOptionsDescriptor as u32,
    RequestAdapterOptionsLuid = ffi::WGPUSType_WGPUSType_RequestAdapterOptionsLUID as u32,
    RequestAdapterOptionsGetGlProc = ffi::WGPUSType_WGPUSType_RequestAdapterOptionsGetGLProc as u32,
    RequestAdapterOptionsD3D11Device =
        ffi::WGPUSType_WGPUSType_RequestAdapterOptionsD3D11Device as u32,
    DawnRenderPassSampleCount = ffi::WGPUSType_WGPUSType_DawnRenderPassSampleCount as u32,
    RenderPassPixelLocalStorage = ffi::WGPUSType_WGPUSType_RenderPassPixelLocalStorage as u32,
    PipelineLayoutPixelLocalStorage =
        ffi::WGPUSType_WGPUSType_PipelineLayoutPixelLocalStorage as u32,
    BufferHostMappedPointer = ffi::WGPUSType_WGPUSType_BufferHostMappedPointer as u32,
    AdapterPropertiesMemoryHeaps = ffi::WGPUSType_WGPUSType_AdapterPropertiesMemoryHeaps as u32,
    AdapterPropertiesD3D = ffi::WGPUSType_WGPUSType_AdapterPropertiesD3D as u32,
    AdapterPropertiesVk = ffi::WGPUSType_WGPUSType_AdapterPropertiesVk as u32,
    DawnWireWGSLControl = ffi::WGPUSType_WGPUSType_DawnWireWGSLControl as u32,
    DawnWGSLBlocklist = ffi::WGPUSType_WGPUSType_DawnWGSLBlocklist as u32,
    DawnDrmFormatCapabilities = ffi::WGPUSType_WGPUSType_DawnDrmFormatCapabilities as u32,
    ShaderModuleCompilationOptions = ffi::WGPUSType_WGPUSType_ShaderModuleCompilationOptions as u32,
    ColorTargetStateExpandResolveTextureDawn =
        ffi::WGPUSType_WGPUSType_ColorTargetStateExpandResolveTextureDawn as u32,
    RenderPassDescriptorExpandResolveRect =
        ffi::WGPUSType_WGPUSType_RenderPassDescriptorExpandResolveRect as u32,
    SharedTextureMemoryVkDedicatedAllocationDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkDedicatedAllocationDescriptor as u32,
    SharedTextureMemoryAHardwareBufferDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferDescriptor as u32,
    SharedTextureMemoryDmaBufDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryDmaBufDescriptor as u32,
    SharedTextureMemoryOpaqueFDDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryOpaqueFDDescriptor as u32,
    SharedTextureMemoryZirconHandleDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryZirconHandleDescriptor as u32,
    SharedTextureMemoryDXGISharedHandleDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryDXGISharedHandleDescriptor as u32,
    SharedTextureMemoryD3D11Texture2DDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D11Texture2DDescriptor as u32,
    SharedTextureMemoryIOSurfaceDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryIOSurfaceDescriptor as u32,
    SharedTextureMemoryEGLImageDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryEGLImageDescriptor as u32,
    SharedTextureMemoryInitializedBeginState =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryInitializedBeginState as u32,
    SharedTextureMemoryInitializedEndState =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryInitializedEndState as u32,
    SharedTextureMemoryVkImageLayoutBeginState =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutBeginState as u32,
    SharedTextureMemoryVkImageLayoutEndState =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutEndState as u32,
    SharedTextureMemoryD3DSwapchainBeginState =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3DSwapchainBeginState as u32,
    SharedFenceVkSemaphoreOpaqueFDDescriptor =
        ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDDescriptor as u32,
    SharedFenceVkSemaphoreOpaqueFDExportInfo =
        ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDExportInfo as u32,
    SharedFenceSyncFDDescriptor = ffi::WGPUSType_WGPUSType_SharedFenceSyncFDDescriptor as u32,
    SharedFenceSyncFDExportInfo = ffi::WGPUSType_WGPUSType_SharedFenceSyncFDExportInfo as u32,
    SharedFenceVkSemaphoreZirconHandleDescriptor =
        ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleDescriptor as u32,
    SharedFenceVkSemaphoreZirconHandleExportInfo =
        ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleExportInfo as u32,
    SharedFenceDXGISharedHandleDescriptor =
        ffi::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleDescriptor as u32,
    SharedFenceDXGISharedHandleExportInfo =
        ffi::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleExportInfo as u32,
    SharedFenceMTLSharedEventDescriptor =
        ffi::WGPUSType_WGPUSType_SharedFenceMTLSharedEventDescriptor as u32,
    SharedFenceMTLSharedEventExportInfo =
        ffi::WGPUSType_WGPUSType_SharedFenceMTLSharedEventExportInfo as u32,
    SharedBufferMemoryD3D12ResourceDescriptor =
        ffi::WGPUSType_WGPUSType_SharedBufferMemoryD3D12ResourceDescriptor as u32,
    StaticSamplerBindingLayout = ffi::WGPUSType_WGPUSType_StaticSamplerBindingLayout as u32,
    YCbCrVkDescriptor = ffi::WGPUSType_WGPUSType_YCbCrVkDescriptor as u32,
    SharedTextureMemoryAHardwareBufferProperties =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferProperties as u32,
    AHardwareBufferProperties = ffi::WGPUSType_WGPUSType_AHardwareBufferProperties as u32,
    DawnTexelCopyBufferRowAlignmentLimits =
        ffi::WGPUSType_WGPUSType_DawnTexelCopyBufferRowAlignmentLimits as u32,
    AdapterPropertiesSubgroupMatrixConfigs =
        ffi::WGPUSType_WGPUSType_AdapterPropertiesSubgroupMatrixConfigs as u32,
    SharedFenceEGLSyncDescriptor = ffi::WGPUSType_WGPUSType_SharedFenceEGLSyncDescriptor as u32,
    SharedFenceEGLSyncExportInfo = ffi::WGPUSType_WGPUSType_SharedFenceEGLSyncExportInfo as u32,
    DawnInjectedInvalidSType = ffi::WGPUSType_WGPUSType_DawnInjectedInvalidSType as u32,
    DawnCompilationMessageUtf16 = ffi::WGPUSType_WGPUSType_DawnCompilationMessageUtf16 as u32,
    DawnFakeBufferOOMForTesting = ffi::WGPUSType_WGPUSType_DawnFakeBufferOOMForTesting as u32,
    SurfaceDescriptorFromWindowsWinUISwapChainPanel =
        ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsWinUISwapChainPanel as u32,
    DawnDeviceAllocatorControl = ffi::WGPUSType_WGPUSType_DawnDeviceAllocatorControl as u32,
    DawnHostMappedPointerLimits = ffi::WGPUSType_WGPUSType_DawnHostMappedPointerLimits as u32,
    RenderPassDescriptorResolveRect =
        ffi::WGPUSType_WGPUSType_RenderPassDescriptorResolveRect as u32,
    RequestAdapterWebGPUBackendOptions =
        ffi::WGPUSType_WGPUSType_RequestAdapterWebGPUBackendOptions as u32,
    DawnFakeDeviceInitializeErrorForTesting =
        ffi::WGPUSType_WGPUSType_DawnFakeDeviceInitializeErrorForTesting as u32,
    SharedTextureMemoryD3D11BeginState =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D11BeginState as u32,
    DawnConsumeAdapterDescriptor = ffi::WGPUSType_WGPUSType_DawnConsumeAdapterDescriptor as u32,
    TexelBufferBindingEntry = ffi::WGPUSType_WGPUSType_TexelBufferBindingEntry as u32,
    TexelBufferBindingLayout = ffi::WGPUSType_WGPUSType_TexelBufferBindingLayout as u32,
    SharedTextureMemoryMetalEndAccessState =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryMetalEndAccessState as u32,
    AdapterPropertiesWGPU = ffi::WGPUSType_WGPUSType_AdapterPropertiesWGPU as u32,
    SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor =
        ffi::WGPUSType_WGPUSType_SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor
            as u32,
    SharedTextureMemoryD3D12ResourceDescriptor =
        ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D12ResourceDescriptor as u32,
    RequestAdapterOptionsAngleVirtualizationGroup =
        ffi::WGPUSType_WGPUSType_RequestAdapterOptionsAngleVirtualizationGroup as u32,
    PipelineLayoutResourceTable = ffi::WGPUSType_WGPUSType_PipelineLayoutResourceTable as u32,
    AdapterPropertiesExplicitComputeSubgroupSizeConfigs =
        ffi::WGPUSType_WGPUSType_AdapterPropertiesExplicitComputeSubgroupSizeConfigs as u32,
}
impl From<ffi::WGPUSType> for SType {
    fn from(value: ffi::WGPUSType) -> Self {
        match value {
            ffi::WGPUSType_WGPUSType_ShaderSourceSPIRV => SType::ShaderSourceSPIRV,
            ffi::WGPUSType_WGPUSType_ShaderSourceWGSL => SType::ShaderSourceWGSL,
            ffi::WGPUSType_WGPUSType_RenderPassMaxDrawCount => {
                SType::RenderPassMaxDrawCount
            }
            ffi::WGPUSType_WGPUSType_SurfaceSourceMetalLayer => {
                SType::SurfaceSourceMetalLayer
            }
            ffi::WGPUSType_WGPUSType_SurfaceSourceWindowsHWND => {
                SType::SurfaceSourceWindowsHWND
            }
            ffi::WGPUSType_WGPUSType_SurfaceSourceXlibWindow => {
                SType::SurfaceSourceXlibWindow
            }
            ffi::WGPUSType_WGPUSType_SurfaceSourceWaylandSurface => {
                SType::SurfaceSourceWaylandSurface
            }
            ffi::WGPUSType_WGPUSType_SurfaceSourceAndroidNativeWindow => {
                SType::SurfaceSourceAndroidNativeWindow
            }
            ffi::WGPUSType_WGPUSType_SurfaceSourceXCBWindow => {
                SType::SurfaceSourceXCBWindow
            }
            ffi::WGPUSType_WGPUSType_SurfaceColorManagement => {
                SType::SurfaceColorManagement
            }
            ffi::WGPUSType_WGPUSType_RequestAdapterWebXROptions => {
                SType::RequestAdapterWebXROptions
            }
            ffi::WGPUSType_WGPUSType_TextureComponentSwizzleDescriptor => {
                SType::TextureComponentSwizzleDescriptor
            }
            ffi::WGPUSType_WGPUSType_CompatibilityModeLimits => {
                SType::CompatibilityModeLimits
            }
            ffi::WGPUSType_WGPUSType_TextureBindingViewDimensionDescriptor => {
                SType::TextureBindingViewDimensionDescriptor
            }
            ffi::WGPUSType_WGPUSType_EmscriptenSurfaceSourceCanvasHTMLSelector => {
                SType::EmscriptenSurfaceSourceCanvasHTMLSelector
            }
            ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsCoreWindow => {
                SType::SurfaceDescriptorFromWindowsCoreWindow
            }
            ffi::WGPUSType_WGPUSType_ExternalTextureBindingEntry => {
                SType::ExternalTextureBindingEntry
            }
            ffi::WGPUSType_WGPUSType_ExternalTextureBindingLayout => {
                SType::ExternalTextureBindingLayout
            }
            ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsUWPSwapChainPanel => {
                SType::SurfaceDescriptorFromWindowsUWPSwapChainPanel
            }
            ffi::WGPUSType_WGPUSType_DawnTextureInternalUsageDescriptor => {
                SType::DawnTextureInternalUsageDescriptor
            }
            ffi::WGPUSType_WGPUSType_DawnEncoderInternalUsageDescriptor => {
                SType::DawnEncoderInternalUsageDescriptor
            }
            ffi::WGPUSType_WGPUSType_DawnInstanceDescriptor => {
                SType::DawnInstanceDescriptor
            }
            ffi::WGPUSType_WGPUSType_DawnCacheDeviceDescriptor => {
                SType::DawnCacheDeviceDescriptor
            }
            ffi::WGPUSType_WGPUSType_DawnAdapterPropertiesPowerPreference => {
                SType::DawnAdapterPropertiesPowerPreference
            }
            ffi::WGPUSType_WGPUSType_DawnBufferDescriptorErrorInfoFromWireClient => {
                SType::DawnBufferDescriptorErrorInfoFromWireClient
            }
            ffi::WGPUSType_WGPUSType_DawnTogglesDescriptor => {
                SType::DawnTogglesDescriptor
            }
            ffi::WGPUSType_WGPUSType_DawnShaderModuleSPIRVOptionsDescriptor => {
                SType::DawnShaderModuleSPIRVOptionsDescriptor
            }
            ffi::WGPUSType_WGPUSType_RequestAdapterOptionsLUID => {
                SType::RequestAdapterOptionsLuid
            }
            ffi::WGPUSType_WGPUSType_RequestAdapterOptionsGetGLProc => {
                SType::RequestAdapterOptionsGetGlProc
            }
            ffi::WGPUSType_WGPUSType_RequestAdapterOptionsD3D11Device => {
                SType::RequestAdapterOptionsD3D11Device
            }
            ffi::WGPUSType_WGPUSType_DawnRenderPassSampleCount => {
                SType::DawnRenderPassSampleCount
            }
            ffi::WGPUSType_WGPUSType_RenderPassPixelLocalStorage => {
                SType::RenderPassPixelLocalStorage
            }
            ffi::WGPUSType_WGPUSType_PipelineLayoutPixelLocalStorage => {
                SType::PipelineLayoutPixelLocalStorage
            }
            ffi::WGPUSType_WGPUSType_BufferHostMappedPointer => {
                SType::BufferHostMappedPointer
            }
            ffi::WGPUSType_WGPUSType_AdapterPropertiesMemoryHeaps => {
                SType::AdapterPropertiesMemoryHeaps
            }
            ffi::WGPUSType_WGPUSType_AdapterPropertiesD3D => SType::AdapterPropertiesD3D,
            ffi::WGPUSType_WGPUSType_AdapterPropertiesVk => SType::AdapterPropertiesVk,
            ffi::WGPUSType_WGPUSType_DawnWireWGSLControl => SType::DawnWireWGSLControl,
            ffi::WGPUSType_WGPUSType_DawnWGSLBlocklist => SType::DawnWGSLBlocklist,
            ffi::WGPUSType_WGPUSType_DawnDrmFormatCapabilities => {
                SType::DawnDrmFormatCapabilities
            }
            ffi::WGPUSType_WGPUSType_ShaderModuleCompilationOptions => {
                SType::ShaderModuleCompilationOptions
            }
            ffi::WGPUSType_WGPUSType_ColorTargetStateExpandResolveTextureDawn => {
                SType::ColorTargetStateExpandResolveTextureDawn
            }
            ffi::WGPUSType_WGPUSType_RenderPassDescriptorExpandResolveRect => {
                SType::RenderPassDescriptorExpandResolveRect
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkDedicatedAllocationDescriptor => {
                SType::SharedTextureMemoryVkDedicatedAllocationDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferDescriptor => {
                SType::SharedTextureMemoryAHardwareBufferDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryDmaBufDescriptor => {
                SType::SharedTextureMemoryDmaBufDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryOpaqueFDDescriptor => {
                SType::SharedTextureMemoryOpaqueFDDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryZirconHandleDescriptor => {
                SType::SharedTextureMemoryZirconHandleDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryDXGISharedHandleDescriptor => {
                SType::SharedTextureMemoryDXGISharedHandleDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D11Texture2DDescriptor => {
                SType::SharedTextureMemoryD3D11Texture2DDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryIOSurfaceDescriptor => {
                SType::SharedTextureMemoryIOSurfaceDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryEGLImageDescriptor => {
                SType::SharedTextureMemoryEGLImageDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryInitializedBeginState => {
                SType::SharedTextureMemoryInitializedBeginState
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryInitializedEndState => {
                SType::SharedTextureMemoryInitializedEndState
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutBeginState => {
                SType::SharedTextureMemoryVkImageLayoutBeginState
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutEndState => {
                SType::SharedTextureMemoryVkImageLayoutEndState
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3DSwapchainBeginState => {
                SType::SharedTextureMemoryD3DSwapchainBeginState
            }
            ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDDescriptor => {
                SType::SharedFenceVkSemaphoreOpaqueFDDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDExportInfo => {
                SType::SharedFenceVkSemaphoreOpaqueFDExportInfo
            }
            ffi::WGPUSType_WGPUSType_SharedFenceSyncFDDescriptor => {
                SType::SharedFenceSyncFDDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedFenceSyncFDExportInfo => {
                SType::SharedFenceSyncFDExportInfo
            }
            ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleDescriptor => {
                SType::SharedFenceVkSemaphoreZirconHandleDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleExportInfo => {
                SType::SharedFenceVkSemaphoreZirconHandleExportInfo
            }
            ffi::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleDescriptor => {
                SType::SharedFenceDXGISharedHandleDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleExportInfo => {
                SType::SharedFenceDXGISharedHandleExportInfo
            }
            ffi::WGPUSType_WGPUSType_SharedFenceMTLSharedEventDescriptor => {
                SType::SharedFenceMTLSharedEventDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedFenceMTLSharedEventExportInfo => {
                SType::SharedFenceMTLSharedEventExportInfo
            }
            ffi::WGPUSType_WGPUSType_SharedBufferMemoryD3D12ResourceDescriptor => {
                SType::SharedBufferMemoryD3D12ResourceDescriptor
            }
            ffi::WGPUSType_WGPUSType_StaticSamplerBindingLayout => {
                SType::StaticSamplerBindingLayout
            }
            ffi::WGPUSType_WGPUSType_YCbCrVkDescriptor => SType::YCbCrVkDescriptor,
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferProperties => {
                SType::SharedTextureMemoryAHardwareBufferProperties
            }
            ffi::WGPUSType_WGPUSType_AHardwareBufferProperties => {
                SType::AHardwareBufferProperties
            }
            ffi::WGPUSType_WGPUSType_DawnTexelCopyBufferRowAlignmentLimits => {
                SType::DawnTexelCopyBufferRowAlignmentLimits
            }
            ffi::WGPUSType_WGPUSType_AdapterPropertiesSubgroupMatrixConfigs => {
                SType::AdapterPropertiesSubgroupMatrixConfigs
            }
            ffi::WGPUSType_WGPUSType_SharedFenceEGLSyncDescriptor => {
                SType::SharedFenceEGLSyncDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedFenceEGLSyncExportInfo => {
                SType::SharedFenceEGLSyncExportInfo
            }
            ffi::WGPUSType_WGPUSType_DawnInjectedInvalidSType => {
                SType::DawnInjectedInvalidSType
            }
            ffi::WGPUSType_WGPUSType_DawnCompilationMessageUtf16 => {
                SType::DawnCompilationMessageUtf16
            }
            ffi::WGPUSType_WGPUSType_DawnFakeBufferOOMForTesting => {
                SType::DawnFakeBufferOOMForTesting
            }
            ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsWinUISwapChainPanel => {
                SType::SurfaceDescriptorFromWindowsWinUISwapChainPanel
            }
            ffi::WGPUSType_WGPUSType_DawnDeviceAllocatorControl => {
                SType::DawnDeviceAllocatorControl
            }
            ffi::WGPUSType_WGPUSType_DawnHostMappedPointerLimits => {
                SType::DawnHostMappedPointerLimits
            }
            ffi::WGPUSType_WGPUSType_RenderPassDescriptorResolveRect => {
                SType::RenderPassDescriptorResolveRect
            }
            ffi::WGPUSType_WGPUSType_RequestAdapterWebGPUBackendOptions => {
                SType::RequestAdapterWebGPUBackendOptions
            }
            ffi::WGPUSType_WGPUSType_DawnFakeDeviceInitializeErrorForTesting => {
                SType::DawnFakeDeviceInitializeErrorForTesting
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D11BeginState => {
                SType::SharedTextureMemoryD3D11BeginState
            }
            ffi::WGPUSType_WGPUSType_DawnConsumeAdapterDescriptor => {
                SType::DawnConsumeAdapterDescriptor
            }
            ffi::WGPUSType_WGPUSType_TexelBufferBindingEntry => {
                SType::TexelBufferBindingEntry
            }
            ffi::WGPUSType_WGPUSType_TexelBufferBindingLayout => {
                SType::TexelBufferBindingLayout
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryMetalEndAccessState => {
                SType::SharedTextureMemoryMetalEndAccessState
            }
            ffi::WGPUSType_WGPUSType_AdapterPropertiesWGPU => {
                SType::AdapterPropertiesWGPU
            }
            ffi::WGPUSType_WGPUSType_SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor => {
                SType::SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor
            }
            ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D12ResourceDescriptor => {
                SType::SharedTextureMemoryD3D12ResourceDescriptor
            }
            ffi::WGPUSType_WGPUSType_RequestAdapterOptionsAngleVirtualizationGroup => {
                SType::RequestAdapterOptionsAngleVirtualizationGroup
            }
            ffi::WGPUSType_WGPUSType_PipelineLayoutResourceTable => {
                SType::PipelineLayoutResourceTable
            }
            ffi::WGPUSType_WGPUSType_AdapterPropertiesExplicitComputeSubgroupSizeConfigs => {
                SType::AdapterPropertiesExplicitComputeSubgroupSizeConfigs
            }
            _ => SType::ShaderSourceSPIRV,
        }
    }
}
impl From<SType> for ffi::WGPUSType {
    fn from(value: SType) -> Self {
        value as ffi::WGPUSType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SamplerBindingType {
    BindingNotUsed = ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_BindingNotUsed as u32,
    Undefined = ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Undefined as u32,
    Filtering = ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Filtering as u32,
    NonFiltering = ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_NonFiltering as u32,
    Comparison = ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Comparison as u32,
}
impl From<ffi::WGPUSamplerBindingType> for SamplerBindingType {
    fn from(value: ffi::WGPUSamplerBindingType) -> Self {
        match value {
            ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_BindingNotUsed => {
                SamplerBindingType::BindingNotUsed
            }
            ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Undefined => {
                SamplerBindingType::Undefined
            }
            ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Filtering => {
                SamplerBindingType::Filtering
            }
            ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_NonFiltering => {
                SamplerBindingType::NonFiltering
            }
            ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Comparison => {
                SamplerBindingType::Comparison
            }
            _ => SamplerBindingType::BindingNotUsed,
        }
    }
}
impl From<SamplerBindingType> for ffi::WGPUSamplerBindingType {
    fn from(value: SamplerBindingType) -> Self {
        value as ffi::WGPUSamplerBindingType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SharedFenceType {
    VkSemaphoreOpaqueFD = ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreOpaqueFD as u32,
    SyncFD = ffi::WGPUSharedFenceType_WGPUSharedFenceType_SyncFD as u32,
    VkSemaphoreZirconHandle =
        ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreZirconHandle as u32,
    DXGISharedHandle = ffi::WGPUSharedFenceType_WGPUSharedFenceType_DXGISharedHandle as u32,
    MTLSharedEvent = ffi::WGPUSharedFenceType_WGPUSharedFenceType_MTLSharedEvent as u32,
    EGLSync = ffi::WGPUSharedFenceType_WGPUSharedFenceType_EGLSync as u32,
}
impl From<ffi::WGPUSharedFenceType> for SharedFenceType {
    fn from(value: ffi::WGPUSharedFenceType) -> Self {
        match value {
            ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreOpaqueFD => {
                SharedFenceType::VkSemaphoreOpaqueFD
            }
            ffi::WGPUSharedFenceType_WGPUSharedFenceType_SyncFD => SharedFenceType::SyncFD,
            ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreZirconHandle => {
                SharedFenceType::VkSemaphoreZirconHandle
            }
            ffi::WGPUSharedFenceType_WGPUSharedFenceType_DXGISharedHandle => {
                SharedFenceType::DXGISharedHandle
            }
            ffi::WGPUSharedFenceType_WGPUSharedFenceType_MTLSharedEvent => {
                SharedFenceType::MTLSharedEvent
            }
            ffi::WGPUSharedFenceType_WGPUSharedFenceType_EGLSync => SharedFenceType::EGLSync,
            _ => SharedFenceType::VkSemaphoreOpaqueFD,
        }
    }
}
impl From<SharedFenceType> for ffi::WGPUSharedFenceType {
    fn from(value: SharedFenceType) -> Self {
        value as ffi::WGPUSharedFenceType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Status {
    Success = ffi::WGPUStatus_WGPUStatus_Success as u32,
    Error = ffi::WGPUStatus_WGPUStatus_Error as u32,
}
impl From<ffi::WGPUStatus> for Status {
    fn from(value: ffi::WGPUStatus) -> Self {
        match value {
            ffi::WGPUStatus_WGPUStatus_Success => Status::Success,
            ffi::WGPUStatus_WGPUStatus_Error => Status::Error,
            _ => Status::Success,
        }
    }
}
impl From<Status> for ffi::WGPUStatus {
    fn from(value: Status) -> Self {
        value as ffi::WGPUStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum StencilOperation {
    Undefined = ffi::WGPUStencilOperation_WGPUStencilOperation_Undefined as u32,
    Keep = ffi::WGPUStencilOperation_WGPUStencilOperation_Keep as u32,
    Zero = ffi::WGPUStencilOperation_WGPUStencilOperation_Zero as u32,
    Replace = ffi::WGPUStencilOperation_WGPUStencilOperation_Replace as u32,
    Invert = ffi::WGPUStencilOperation_WGPUStencilOperation_Invert as u32,
    IncrementClamp = ffi::WGPUStencilOperation_WGPUStencilOperation_IncrementClamp as u32,
    DecrementClamp = ffi::WGPUStencilOperation_WGPUStencilOperation_DecrementClamp as u32,
    IncrementWrap = ffi::WGPUStencilOperation_WGPUStencilOperation_IncrementWrap as u32,
    DecrementWrap = ffi::WGPUStencilOperation_WGPUStencilOperation_DecrementWrap as u32,
}
impl From<ffi::WGPUStencilOperation> for StencilOperation {
    fn from(value: ffi::WGPUStencilOperation) -> Self {
        match value {
            ffi::WGPUStencilOperation_WGPUStencilOperation_Undefined => StencilOperation::Undefined,
            ffi::WGPUStencilOperation_WGPUStencilOperation_Keep => StencilOperation::Keep,
            ffi::WGPUStencilOperation_WGPUStencilOperation_Zero => StencilOperation::Zero,
            ffi::WGPUStencilOperation_WGPUStencilOperation_Replace => StencilOperation::Replace,
            ffi::WGPUStencilOperation_WGPUStencilOperation_Invert => StencilOperation::Invert,
            ffi::WGPUStencilOperation_WGPUStencilOperation_IncrementClamp => {
                StencilOperation::IncrementClamp
            }
            ffi::WGPUStencilOperation_WGPUStencilOperation_DecrementClamp => {
                StencilOperation::DecrementClamp
            }
            ffi::WGPUStencilOperation_WGPUStencilOperation_IncrementWrap => {
                StencilOperation::IncrementWrap
            }
            ffi::WGPUStencilOperation_WGPUStencilOperation_DecrementWrap => {
                StencilOperation::DecrementWrap
            }
            _ => StencilOperation::Undefined,
        }
    }
}
impl From<StencilOperation> for ffi::WGPUStencilOperation {
    fn from(value: StencilOperation) -> Self {
        value as ffi::WGPUStencilOperation
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum StorageTextureAccess {
    BindingNotUsed = ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_BindingNotUsed as u32,
    Undefined = ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_Undefined as u32,
    WriteOnly = ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_WriteOnly as u32,
    ReadOnly = ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadOnly as u32,
    ReadWrite = ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadWrite as u32,
}
impl From<ffi::WGPUStorageTextureAccess> for StorageTextureAccess {
    fn from(value: ffi::WGPUStorageTextureAccess) -> Self {
        match value {
            ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_BindingNotUsed => {
                StorageTextureAccess::BindingNotUsed
            }
            ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_Undefined => {
                StorageTextureAccess::Undefined
            }
            ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_WriteOnly => {
                StorageTextureAccess::WriteOnly
            }
            ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadOnly => {
                StorageTextureAccess::ReadOnly
            }
            ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadWrite => {
                StorageTextureAccess::ReadWrite
            }
            _ => StorageTextureAccess::BindingNotUsed,
        }
    }
}
impl From<StorageTextureAccess> for ffi::WGPUStorageTextureAccess {
    fn from(value: StorageTextureAccess) -> Self {
        value as ffi::WGPUStorageTextureAccess
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum StoreOp {
    Undefined = ffi::WGPUStoreOp_WGPUStoreOp_Undefined as u32,
    Store = ffi::WGPUStoreOp_WGPUStoreOp_Store as u32,
    Discard = ffi::WGPUStoreOp_WGPUStoreOp_Discard as u32,
}
impl From<ffi::WGPUStoreOp> for StoreOp {
    fn from(value: ffi::WGPUStoreOp) -> Self {
        match value {
            ffi::WGPUStoreOp_WGPUStoreOp_Undefined => StoreOp::Undefined,
            ffi::WGPUStoreOp_WGPUStoreOp_Store => StoreOp::Store,
            ffi::WGPUStoreOp_WGPUStoreOp_Discard => StoreOp::Discard,
            _ => StoreOp::Undefined,
        }
    }
}
impl From<StoreOp> for ffi::WGPUStoreOp {
    fn from(value: StoreOp) -> Self {
        value as ffi::WGPUStoreOp
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SubgroupMatrixComponentType {
    F32 = ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F32 as u32,
    F16 = ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F16 as u32,
    U32 = ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_U32 as u32,
    I32 = ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_I32 as u32,
    U8 = ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_U8 as u32,
    I8 = ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_I8 as u32,
}
impl From<ffi::WGPUSubgroupMatrixComponentType> for SubgroupMatrixComponentType {
    fn from(value: ffi::WGPUSubgroupMatrixComponentType) -> Self {
        match value {
            ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F32 => {
                SubgroupMatrixComponentType::F32
            }
            ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F16 => {
                SubgroupMatrixComponentType::F16
            }
            ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_U32 => {
                SubgroupMatrixComponentType::U32
            }
            ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_I32 => {
                SubgroupMatrixComponentType::I32
            }
            ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_U8 => {
                SubgroupMatrixComponentType::U8
            }
            ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_I8 => {
                SubgroupMatrixComponentType::I8
            }
            _ => SubgroupMatrixComponentType::F32,
        }
    }
}
impl From<SubgroupMatrixComponentType> for ffi::WGPUSubgroupMatrixComponentType {
    fn from(value: SubgroupMatrixComponentType) -> Self {
        value as ffi::WGPUSubgroupMatrixComponentType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SurfaceGetCurrentTextureStatus {
    SuccessOptimal =
        ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal
            as u32,
    SuccessSuboptimal =
        ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal
            as u32,
    Timeout =
        ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Timeout as u32,
    Outdated =
        ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Outdated as u32,
    Lost = ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Lost as u32,
    Error = ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Error as u32,
}
impl From<ffi::WGPUSurfaceGetCurrentTextureStatus> for SurfaceGetCurrentTextureStatus {
    fn from(value: ffi::WGPUSurfaceGetCurrentTextureStatus) -> Self {
        match value {
            ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal => {
                SurfaceGetCurrentTextureStatus::SuccessOptimal
            }
            ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal => {
                SurfaceGetCurrentTextureStatus::SuccessSuboptimal
            }
            ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Timeout => {
                SurfaceGetCurrentTextureStatus::Timeout
            }
            ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Outdated => {
                SurfaceGetCurrentTextureStatus::Outdated
            }
            ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Lost => {
                SurfaceGetCurrentTextureStatus::Lost
            }
            ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Error => {
                SurfaceGetCurrentTextureStatus::Error
            }
            _ => SurfaceGetCurrentTextureStatus::SuccessOptimal,
        }
    }
}
impl From<SurfaceGetCurrentTextureStatus> for ffi::WGPUSurfaceGetCurrentTextureStatus {
    fn from(value: SurfaceGetCurrentTextureStatus) -> Self {
        value as ffi::WGPUSurfaceGetCurrentTextureStatus
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TexelBufferAccess {
    Undefined = ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_Undefined as u32,
    ReadOnly = ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_ReadOnly as u32,
    ReadWrite = ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_ReadWrite as u32,
}
impl From<ffi::WGPUTexelBufferAccess> for TexelBufferAccess {
    fn from(value: ffi::WGPUTexelBufferAccess) -> Self {
        match value {
            ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_Undefined => {
                TexelBufferAccess::Undefined
            }
            ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_ReadOnly => {
                TexelBufferAccess::ReadOnly
            }
            ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_ReadWrite => {
                TexelBufferAccess::ReadWrite
            }
            _ => TexelBufferAccess::Undefined,
        }
    }
}
impl From<TexelBufferAccess> for ffi::WGPUTexelBufferAccess {
    fn from(value: TexelBufferAccess) -> Self {
        value as ffi::WGPUTexelBufferAccess
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureAspect {
    Undefined = ffi::WGPUTextureAspect_WGPUTextureAspect_Undefined as u32,
    All = ffi::WGPUTextureAspect_WGPUTextureAspect_All as u32,
    StencilOnly = ffi::WGPUTextureAspect_WGPUTextureAspect_StencilOnly as u32,
    DepthOnly = ffi::WGPUTextureAspect_WGPUTextureAspect_DepthOnly as u32,
    Plane0Only = ffi::WGPUTextureAspect_WGPUTextureAspect_Plane0Only as u32,
    Plane1Only = ffi::WGPUTextureAspect_WGPUTextureAspect_Plane1Only as u32,
    Plane2Only = ffi::WGPUTextureAspect_WGPUTextureAspect_Plane2Only as u32,
}
impl From<ffi::WGPUTextureAspect> for TextureAspect {
    fn from(value: ffi::WGPUTextureAspect) -> Self {
        match value {
            ffi::WGPUTextureAspect_WGPUTextureAspect_Undefined => TextureAspect::Undefined,
            ffi::WGPUTextureAspect_WGPUTextureAspect_All => TextureAspect::All,
            ffi::WGPUTextureAspect_WGPUTextureAspect_StencilOnly => TextureAspect::StencilOnly,
            ffi::WGPUTextureAspect_WGPUTextureAspect_DepthOnly => TextureAspect::DepthOnly,
            ffi::WGPUTextureAspect_WGPUTextureAspect_Plane0Only => TextureAspect::Plane0Only,
            ffi::WGPUTextureAspect_WGPUTextureAspect_Plane1Only => TextureAspect::Plane1Only,
            ffi::WGPUTextureAspect_WGPUTextureAspect_Plane2Only => TextureAspect::Plane2Only,
            _ => TextureAspect::Undefined,
        }
    }
}
impl From<TextureAspect> for ffi::WGPUTextureAspect {
    fn from(value: TextureAspect) -> Self {
        value as ffi::WGPUTextureAspect
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureDimension {
    Undefined = ffi::WGPUTextureDimension_WGPUTextureDimension_Undefined as u32,
    D1 = ffi::WGPUTextureDimension_WGPUTextureDimension_1D as u32,
    D2 = ffi::WGPUTextureDimension_WGPUTextureDimension_2D as u32,
    D3 = ffi::WGPUTextureDimension_WGPUTextureDimension_3D as u32,
}
impl From<ffi::WGPUTextureDimension> for TextureDimension {
    fn from(value: ffi::WGPUTextureDimension) -> Self {
        match value {
            ffi::WGPUTextureDimension_WGPUTextureDimension_Undefined => TextureDimension::Undefined,
            ffi::WGPUTextureDimension_WGPUTextureDimension_1D => TextureDimension::D1,
            ffi::WGPUTextureDimension_WGPUTextureDimension_2D => TextureDimension::D2,
            ffi::WGPUTextureDimension_WGPUTextureDimension_3D => TextureDimension::D3,
            _ => TextureDimension::Undefined,
        }
    }
}
impl From<TextureDimension> for ffi::WGPUTextureDimension {
    fn from(value: TextureDimension) -> Self {
        value as ffi::WGPUTextureDimension
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureFormat {
    Undefined = ffi::WGPUTextureFormat_WGPUTextureFormat_Undefined as u32,
    R8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_R8Unorm as u32,
    R8Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_R8Snorm as u32,
    R8Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_R8Uint as u32,
    R8Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_R8Sint as u32,
    R16Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_R16Unorm as u32,
    R16Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_R16Snorm as u32,
    R16Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_R16Uint as u32,
    R16Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_R16Sint as u32,
    R16Float = ffi::WGPUTextureFormat_WGPUTextureFormat_R16Float as u32,
    Rg8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Unorm as u32,
    Rg8Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Snorm as u32,
    Rg8Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Uint as u32,
    Rg8Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Sint as u32,
    R32Float = ffi::WGPUTextureFormat_WGPUTextureFormat_R32Float as u32,
    R32Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_R32Uint as u32,
    R32Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_R32Sint as u32,
    Rg16Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Unorm as u32,
    Rg16Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Snorm as u32,
    Rg16Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Uint as u32,
    Rg16Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Sint as u32,
    Rg16Float = ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Float as u32,
    Rgba8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Unorm as u32,
    Rgba8UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8UnormSrgb as u32,
    Rgba8Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Snorm as u32,
    Rgba8Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Uint as u32,
    Rgba8Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Sint as u32,
    Bgra8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8Unorm as u32,
    Bgra8UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8UnormSrgb as u32,
    Rgb10A2Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Uint as u32,
    Rgb10A2Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Unorm as u32,
    Rg11B10Ufloat = ffi::WGPUTextureFormat_WGPUTextureFormat_RG11B10Ufloat as u32,
    Rgb9E5Ufloat = ffi::WGPUTextureFormat_WGPUTextureFormat_RGB9E5Ufloat as u32,
    Rg32Float = ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Float as u32,
    Rg32Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Uint as u32,
    Rg32Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Sint as u32,
    Rgba16Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Unorm as u32,
    Rgba16Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Snorm as u32,
    Rgba16Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Uint as u32,
    Rgba16Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Sint as u32,
    Rgba16Float = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Float as u32,
    Rgba32Float = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Float as u32,
    Rgba32Uint = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Uint as u32,
    Rgba32Sint = ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Sint as u32,
    Stencil8 = ffi::WGPUTextureFormat_WGPUTextureFormat_Stencil8 as u32,
    Depth16Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_Depth16Unorm as u32,
    Depth24Plus = ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24Plus as u32,
    Depth24PlusStencil8 = ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24PlusStencil8 as u32,
    Depth32Float = ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32Float as u32,
    Depth32FloatStencil8 = ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32FloatStencil8 as u32,
    Bc1RgbaUnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnorm as u32,
    Bc1RgbaUnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnormSrgb as u32,
    Bc2RgbaUnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnorm as u32,
    Bc2RgbaUnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnormSrgb as u32,
    Bc3RgbaUnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnorm as u32,
    Bc3RgbaUnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnormSrgb as u32,
    Bc4RUnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RUnorm as u32,
    Bc4RSnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RSnorm as u32,
    Bc5RgUnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGUnorm as u32,
    Bc5RgSnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGSnorm as u32,
    Bc6HRgbUfloat = ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBUfloat as u32,
    Bc6HRgbFloat = ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBFloat as u32,
    Bc7RgbaUnorm = ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnorm as u32,
    Bc7RgbaUnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnormSrgb as u32,
    Etc2Rgb8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8Unorm as u32,
    Etc2Rgb8UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8UnormSrgb as u32,
    Etc2Rgb8A1Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1Unorm as u32,
    Etc2Rgb8A1UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1UnormSrgb as u32,
    Etc2Rgba8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8Unorm as u32,
    Etc2Rgba8UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8UnormSrgb as u32,
    EacR11Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Unorm as u32,
    EacR11Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Snorm as u32,
    EacRg11Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Unorm as u32,
    EacRg11Snorm = ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Snorm as u32,
    Astc4X4Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4Unorm as u32,
    Astc4X4UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4UnormSrgb as u32,
    Astc5X4Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4Unorm as u32,
    Astc5X4UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4UnormSrgb as u32,
    Astc5X5Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5Unorm as u32,
    Astc5X5UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5UnormSrgb as u32,
    Astc6X5Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5Unorm as u32,
    Astc6X5UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5UnormSrgb as u32,
    Astc6X6Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6Unorm as u32,
    Astc6X6UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6UnormSrgb as u32,
    Astc8X5Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5Unorm as u32,
    Astc8X5UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5UnormSrgb as u32,
    Astc8X6Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6Unorm as u32,
    Astc8X6UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6UnormSrgb as u32,
    Astc8X8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8Unorm as u32,
    Astc8X8UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8UnormSrgb as u32,
    Astc10X5Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5Unorm as u32,
    Astc10X5UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5UnormSrgb as u32,
    Astc10X6Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6Unorm as u32,
    Astc10X6UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6UnormSrgb as u32,
    Astc10X8Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8Unorm as u32,
    Astc10X8UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8UnormSrgb as u32,
    Astc10X10Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10Unorm as u32,
    Astc10X10UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10UnormSrgb as u32,
    Astc12X10Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10Unorm as u32,
    Astc12X10UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10UnormSrgb as u32,
    Astc12X12Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12Unorm as u32,
    Astc12X12UnormSrgb = ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12UnormSrgb as u32,
    R8Bg8Biplanar420Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar420Unorm as u32,
    R10X6Bg10X6Biplanar420Unorm =
        ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar420Unorm as u32,
    R8Bg8A8Triplanar420Unorm =
        ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8A8Triplanar420Unorm as u32,
    R8Bg8Biplanar422Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar422Unorm as u32,
    R8Bg8Biplanar444Unorm = ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar444Unorm as u32,
    R10X6Bg10X6Biplanar422Unorm =
        ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar422Unorm as u32,
    R10X6Bg10X6Biplanar444Unorm =
        ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar444Unorm as u32,
    External = ffi::WGPUTextureFormat_WGPUTextureFormat_External as u32,
    #[cfg(target_os = "android")]
    OpaqueYCbCrAndroid = ffi::WGPUTextureFormat_WGPUTextureFormat_OpaqueYCbCrAndroid as u32,
}
impl From<ffi::WGPUTextureFormat> for TextureFormat {
    fn from(value: ffi::WGPUTextureFormat) -> Self {
        match value {
            ffi::WGPUTextureFormat_WGPUTextureFormat_Undefined => TextureFormat::Undefined,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8Unorm => TextureFormat::R8Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8Snorm => TextureFormat::R8Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8Uint => TextureFormat::R8Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8Sint => TextureFormat::R8Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R16Unorm => TextureFormat::R16Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R16Snorm => TextureFormat::R16Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R16Uint => TextureFormat::R16Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R16Sint => TextureFormat::R16Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R16Float => TextureFormat::R16Float,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Unorm => TextureFormat::Rg8Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Snorm => TextureFormat::Rg8Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Uint => TextureFormat::Rg8Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Sint => TextureFormat::Rg8Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R32Float => TextureFormat::R32Float,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R32Uint => TextureFormat::R32Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_R32Sint => TextureFormat::R32Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Unorm => TextureFormat::Rg16Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Snorm => TextureFormat::Rg16Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Uint => TextureFormat::Rg16Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Sint => TextureFormat::Rg16Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Float => TextureFormat::Rg16Float,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Unorm => TextureFormat::Rgba8Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8UnormSrgb => {
                TextureFormat::Rgba8UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Snorm => TextureFormat::Rgba8Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Uint => TextureFormat::Rgba8Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Sint => TextureFormat::Rgba8Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8Unorm => TextureFormat::Bgra8Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8UnormSrgb => {
                TextureFormat::Bgra8UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Uint => TextureFormat::Rgb10A2Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Unorm => TextureFormat::Rgb10A2Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG11B10Ufloat => TextureFormat::Rg11B10Ufloat,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGB9E5Ufloat => TextureFormat::Rgb9E5Ufloat,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Float => TextureFormat::Rg32Float,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Uint => TextureFormat::Rg32Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Sint => TextureFormat::Rg32Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Unorm => TextureFormat::Rgba16Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Snorm => TextureFormat::Rgba16Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Uint => TextureFormat::Rgba16Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Sint => TextureFormat::Rgba16Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Float => TextureFormat::Rgba16Float,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Float => TextureFormat::Rgba32Float,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Uint => TextureFormat::Rgba32Uint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Sint => TextureFormat::Rgba32Sint,
            ffi::WGPUTextureFormat_WGPUTextureFormat_Stencil8 => TextureFormat::Stencil8,
            ffi::WGPUTextureFormat_WGPUTextureFormat_Depth16Unorm => TextureFormat::Depth16Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24Plus => TextureFormat::Depth24Plus,
            ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24PlusStencil8 => {
                TextureFormat::Depth24PlusStencil8
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32Float => TextureFormat::Depth32Float,
            ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32FloatStencil8 => {
                TextureFormat::Depth32FloatStencil8
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnorm => TextureFormat::Bc1RgbaUnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnormSrgb => {
                TextureFormat::Bc1RgbaUnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnorm => TextureFormat::Bc2RgbaUnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnormSrgb => {
                TextureFormat::Bc2RgbaUnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnorm => TextureFormat::Bc3RgbaUnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnormSrgb => {
                TextureFormat::Bc3RgbaUnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RUnorm => TextureFormat::Bc4RUnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RSnorm => TextureFormat::Bc4RSnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGUnorm => TextureFormat::Bc5RgUnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGSnorm => TextureFormat::Bc5RgSnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBUfloat => TextureFormat::Bc6HRgbUfloat,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBFloat => TextureFormat::Bc6HRgbFloat,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnorm => TextureFormat::Bc7RgbaUnorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnormSrgb => {
                TextureFormat::Bc7RgbaUnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8Unorm => TextureFormat::Etc2Rgb8Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8UnormSrgb => {
                TextureFormat::Etc2Rgb8UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1Unorm => {
                TextureFormat::Etc2Rgb8A1Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1UnormSrgb => {
                TextureFormat::Etc2Rgb8A1UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8Unorm => {
                TextureFormat::Etc2Rgba8Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8UnormSrgb => {
                TextureFormat::Etc2Rgba8UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Unorm => TextureFormat::EacR11Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Snorm => TextureFormat::EacR11Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Unorm => TextureFormat::EacRg11Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Snorm => TextureFormat::EacRg11Snorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4Unorm => TextureFormat::Astc4X4Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4UnormSrgb => {
                TextureFormat::Astc4X4UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4Unorm => TextureFormat::Astc5X4Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4UnormSrgb => {
                TextureFormat::Astc5X4UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5Unorm => TextureFormat::Astc5X5Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5UnormSrgb => {
                TextureFormat::Astc5X5UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5Unorm => TextureFormat::Astc6X5Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5UnormSrgb => {
                TextureFormat::Astc6X5UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6Unorm => TextureFormat::Astc6X6Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6UnormSrgb => {
                TextureFormat::Astc6X6UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5Unorm => TextureFormat::Astc8X5Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5UnormSrgb => {
                TextureFormat::Astc8X5UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6Unorm => TextureFormat::Astc8X6Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6UnormSrgb => {
                TextureFormat::Astc8X6UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8Unorm => TextureFormat::Astc8X8Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8UnormSrgb => {
                TextureFormat::Astc8X8UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5Unorm => TextureFormat::Astc10X5Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5UnormSrgb => {
                TextureFormat::Astc10X5UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6Unorm => TextureFormat::Astc10X6Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6UnormSrgb => {
                TextureFormat::Astc10X6UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8Unorm => TextureFormat::Astc10X8Unorm,
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8UnormSrgb => {
                TextureFormat::Astc10X8UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10Unorm => {
                TextureFormat::Astc10X10Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10UnormSrgb => {
                TextureFormat::Astc10X10UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10Unorm => {
                TextureFormat::Astc12X10Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10UnormSrgb => {
                TextureFormat::Astc12X10UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12Unorm => {
                TextureFormat::Astc12X12Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12UnormSrgb => {
                TextureFormat::Astc12X12UnormSrgb
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar420Unorm => {
                TextureFormat::R8Bg8Biplanar420Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar420Unorm => {
                TextureFormat::R10X6Bg10X6Biplanar420Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8A8Triplanar420Unorm => {
                TextureFormat::R8Bg8A8Triplanar420Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar422Unorm => {
                TextureFormat::R8Bg8Biplanar422Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar444Unorm => {
                TextureFormat::R8Bg8Biplanar444Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar422Unorm => {
                TextureFormat::R10X6Bg10X6Biplanar422Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar444Unorm => {
                TextureFormat::R10X6Bg10X6Biplanar444Unorm
            }
            ffi::WGPUTextureFormat_WGPUTextureFormat_External => TextureFormat::External,
            #[cfg(target_os = "android")]
            ffi::WGPUTextureFormat_WGPUTextureFormat_OpaqueYCbCrAndroid => {
                TextureFormat::OpaqueYCbCrAndroid
            }
            _ => TextureFormat::Undefined,
        }
    }
}
impl From<TextureFormat> for ffi::WGPUTextureFormat {
    fn from(value: TextureFormat) -> Self {
        value as ffi::WGPUTextureFormat
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureSampleType {
    BindingNotUsed = ffi::WGPUTextureSampleType_WGPUTextureSampleType_BindingNotUsed as u32,
    Undefined = ffi::WGPUTextureSampleType_WGPUTextureSampleType_Undefined as u32,
    Float = ffi::WGPUTextureSampleType_WGPUTextureSampleType_Float as u32,
    UnfilterableFloat = ffi::WGPUTextureSampleType_WGPUTextureSampleType_UnfilterableFloat as u32,
    Depth = ffi::WGPUTextureSampleType_WGPUTextureSampleType_Depth as u32,
    Sint = ffi::WGPUTextureSampleType_WGPUTextureSampleType_Sint as u32,
    Uint = ffi::WGPUTextureSampleType_WGPUTextureSampleType_Uint as u32,
}
impl From<ffi::WGPUTextureSampleType> for TextureSampleType {
    fn from(value: ffi::WGPUTextureSampleType) -> Self {
        match value {
            ffi::WGPUTextureSampleType_WGPUTextureSampleType_BindingNotUsed => {
                TextureSampleType::BindingNotUsed
            }
            ffi::WGPUTextureSampleType_WGPUTextureSampleType_Undefined => {
                TextureSampleType::Undefined
            }
            ffi::WGPUTextureSampleType_WGPUTextureSampleType_Float => TextureSampleType::Float,
            ffi::WGPUTextureSampleType_WGPUTextureSampleType_UnfilterableFloat => {
                TextureSampleType::UnfilterableFloat
            }
            ffi::WGPUTextureSampleType_WGPUTextureSampleType_Depth => TextureSampleType::Depth,
            ffi::WGPUTextureSampleType_WGPUTextureSampleType_Sint => TextureSampleType::Sint,
            ffi::WGPUTextureSampleType_WGPUTextureSampleType_Uint => TextureSampleType::Uint,
            _ => TextureSampleType::BindingNotUsed,
        }
    }
}
impl From<TextureSampleType> for ffi::WGPUTextureSampleType {
    fn from(value: TextureSampleType) -> Self {
        value as ffi::WGPUTextureSampleType
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureViewDimension {
    Undefined = ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Undefined as u32,
    D1 = ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_1D as u32,
    D2 = ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2D as u32,
    D2Array = ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2DArray as u32,
    Cube = ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Cube as u32,
    CubeArray = ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_CubeArray as u32,
    D3 = ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_3D as u32,
}
impl From<ffi::WGPUTextureViewDimension> for TextureViewDimension {
    fn from(value: ffi::WGPUTextureViewDimension) -> Self {
        match value {
            ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Undefined => {
                TextureViewDimension::Undefined
            }
            ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_1D => TextureViewDimension::D1,
            ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2D => TextureViewDimension::D2,
            ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2DArray => {
                TextureViewDimension::D2Array
            }
            ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Cube => {
                TextureViewDimension::Cube
            }
            ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_CubeArray => {
                TextureViewDimension::CubeArray
            }
            ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_3D => TextureViewDimension::D3,
            _ => TextureViewDimension::Undefined,
        }
    }
}
impl From<TextureViewDimension> for ffi::WGPUTextureViewDimension {
    fn from(value: TextureViewDimension) -> Self {
        value as ffi::WGPUTextureViewDimension
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ToneMappingMode {
    Standard = ffi::WGPUToneMappingMode_WGPUToneMappingMode_Standard as u32,
    Extended = ffi::WGPUToneMappingMode_WGPUToneMappingMode_Extended as u32,
}
impl From<ffi::WGPUToneMappingMode> for ToneMappingMode {
    fn from(value: ffi::WGPUToneMappingMode) -> Self {
        match value {
            ffi::WGPUToneMappingMode_WGPUToneMappingMode_Standard => ToneMappingMode::Standard,
            ffi::WGPUToneMappingMode_WGPUToneMappingMode_Extended => ToneMappingMode::Extended,
            _ => ToneMappingMode::Standard,
        }
    }
}
impl From<ToneMappingMode> for ffi::WGPUToneMappingMode {
    fn from(value: ToneMappingMode) -> Self {
        value as ffi::WGPUToneMappingMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VertexFormat {
    Uint8 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8 as u32,
    Uint8X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8x2 as u32,
    Uint8X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8x4 as u32,
    Sint8 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8 as u32,
    Sint8X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8x2 as u32,
    Sint8X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8x4 as u32,
    Unorm8 = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8 as u32,
    Unorm8X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x2 as u32,
    Unorm8X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4 as u32,
    Snorm8 = ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8 as u32,
    Snorm8X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8x2 as u32,
    Snorm8X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8x4 as u32,
    Uint16 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16 as u32,
    Uint16X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16x2 as u32,
    Uint16X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16x4 as u32,
    Sint16 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16 as u32,
    Sint16X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16x2 as u32,
    Sint16X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16x4 as u32,
    Unorm16 = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16 as u32,
    Unorm16X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x2 as u32,
    Unorm16X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x4 as u32,
    Snorm16 = ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16 as u32,
    Snorm16X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x2 as u32,
    Snorm16X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x4 as u32,
    Float16 = ffi::WGPUVertexFormat_WGPUVertexFormat_Float16 as u32,
    Float16X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x2 as u32,
    Float16X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x4 as u32,
    Float32 = ffi::WGPUVertexFormat_WGPUVertexFormat_Float32 as u32,
    Float32X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x2 as u32,
    Float32X3 = ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x3 as u32,
    Float32X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x4 as u32,
    Uint32 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32 as u32,
    Uint32X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x2 as u32,
    Uint32X3 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x3 as u32,
    Uint32X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x4 as u32,
    Sint32 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32 as u32,
    Sint32X2 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x2 as u32,
    Sint32X3 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x3 as u32,
    Sint32X4 = ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x4 as u32,
    Unorm1010102 = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm10_10_10_2 as u32,
    Unorm8X4Bgra = ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4BGRA as u32,
}
impl From<ffi::WGPUVertexFormat> for VertexFormat {
    fn from(value: ffi::WGPUVertexFormat) -> Self {
        match value {
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8 => VertexFormat::Uint8,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8x2 => VertexFormat::Uint8X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8x4 => VertexFormat::Uint8X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8 => VertexFormat::Sint8,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8x2 => VertexFormat::Sint8X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8x4 => VertexFormat::Sint8X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8 => VertexFormat::Unorm8,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x2 => VertexFormat::Unorm8X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4 => VertexFormat::Unorm8X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8 => VertexFormat::Snorm8,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8x2 => VertexFormat::Snorm8X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8x4 => VertexFormat::Snorm8X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16 => VertexFormat::Uint16,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16x2 => VertexFormat::Uint16X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16x4 => VertexFormat::Uint16X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16 => VertexFormat::Sint16,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16x2 => VertexFormat::Sint16X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16x4 => VertexFormat::Sint16X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16 => VertexFormat::Unorm16,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x2 => VertexFormat::Unorm16X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x4 => VertexFormat::Unorm16X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16 => VertexFormat::Snorm16,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x2 => VertexFormat::Snorm16X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x4 => VertexFormat::Snorm16X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Float16 => VertexFormat::Float16,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x2 => VertexFormat::Float16X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x4 => VertexFormat::Float16X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Float32 => VertexFormat::Float32,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x2 => VertexFormat::Float32X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x3 => VertexFormat::Float32X3,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x4 => VertexFormat::Float32X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32 => VertexFormat::Uint32,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x2 => VertexFormat::Uint32X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x3 => VertexFormat::Uint32X3,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x4 => VertexFormat::Uint32X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32 => VertexFormat::Sint32,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x2 => VertexFormat::Sint32X2,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x3 => VertexFormat::Sint32X3,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x4 => VertexFormat::Sint32X4,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm10_10_10_2 => VertexFormat::Unorm1010102,
            ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4BGRA => VertexFormat::Unorm8X4Bgra,
            _ => VertexFormat::Uint8,
        }
    }
}
impl From<VertexFormat> for ffi::WGPUVertexFormat {
    fn from(value: VertexFormat) -> Self {
        value as ffi::WGPUVertexFormat
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VertexStepMode {
    Undefined = ffi::WGPUVertexStepMode_WGPUVertexStepMode_Undefined as u32,
    Vertex = ffi::WGPUVertexStepMode_WGPUVertexStepMode_Vertex as u32,
    Instance = ffi::WGPUVertexStepMode_WGPUVertexStepMode_Instance as u32,
}
impl From<ffi::WGPUVertexStepMode> for VertexStepMode {
    fn from(value: ffi::WGPUVertexStepMode) -> Self {
        match value {
            ffi::WGPUVertexStepMode_WGPUVertexStepMode_Undefined => VertexStepMode::Undefined,
            ffi::WGPUVertexStepMode_WGPUVertexStepMode_Vertex => VertexStepMode::Vertex,
            ffi::WGPUVertexStepMode_WGPUVertexStepMode_Instance => VertexStepMode::Instance,
            _ => VertexStepMode::Undefined,
        }
    }
}
impl From<VertexStepMode> for ffi::WGPUVertexStepMode {
    fn from(value: VertexStepMode) -> Self {
        value as ffi::WGPUVertexStepMode
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum WaitStatus {
    Success = ffi::WGPUWaitStatus_WGPUWaitStatus_Success as u32,
    TimedOut = ffi::WGPUWaitStatus_WGPUWaitStatus_TimedOut as u32,
    Error = ffi::WGPUWaitStatus_WGPUWaitStatus_Error as u32,
}
impl From<ffi::WGPUWaitStatus> for WaitStatus {
    fn from(value: ffi::WGPUWaitStatus) -> Self {
        match value {
            ffi::WGPUWaitStatus_WGPUWaitStatus_Success => WaitStatus::Success,
            ffi::WGPUWaitStatus_WGPUWaitStatus_TimedOut => WaitStatus::TimedOut,
            ffi::WGPUWaitStatus_WGPUWaitStatus_Error => WaitStatus::Error,
            _ => WaitStatus::Success,
        }
    }
}
impl From<WaitStatus> for ffi::WGPUWaitStatus {
    fn from(value: WaitStatus) -> Self {
        value as ffi::WGPUWaitStatus
    }
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct BufferUsage : u64 {
    const NONE = ffi::WGPUBufferUsage_None as u64; const MAP_READ =
    ffi::WGPUBufferUsage_MapRead as u64; const MAP_WRITE = ffi::WGPUBufferUsage_MapWrite
    as u64; const COPY_SRC = ffi::WGPUBufferUsage_CopySrc as u64; const COPY_DST =
    ffi::WGPUBufferUsage_CopyDst as u64; const INDEX = ffi::WGPUBufferUsage_Index as u64;
    const VERTEX = ffi::WGPUBufferUsage_Vertex as u64; const UNIFORM =
    ffi::WGPUBufferUsage_Uniform as u64; const STORAGE = ffi::WGPUBufferUsage_Storage as
    u64; const INDIRECT = ffi::WGPUBufferUsage_Indirect as u64; const QUERY_RESOLVE =
    ffi::WGPUBufferUsage_QueryResolve as u64; const TEXEL_BUFFER =
    ffi::WGPUBufferUsage_TexelBuffer as u64; }
}
impl From<ffi::WGPUBufferUsage> for BufferUsage {
    fn from(value: ffi::WGPUBufferUsage) -> Self {
        BufferUsage::from_bits_truncate(value as u64)
    }
}
impl From<BufferUsage> for ffi::WGPUBufferUsage {
    fn from(value: BufferUsage) -> Self {
        value.bits() as ffi::WGPUBufferUsage
    }
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct ColorWriteMask : u64 {
    const NONE = ffi::WGPUColorWriteMask_None as u64; const RED =
    ffi::WGPUColorWriteMask_Red as u64; const GREEN = ffi::WGPUColorWriteMask_Green as
    u64; const BLUE = ffi::WGPUColorWriteMask_Blue as u64; const ALPHA =
    ffi::WGPUColorWriteMask_Alpha as u64; const ALL = ffi::WGPUColorWriteMask_All as u64;
    }
}
impl From<ffi::WGPUColorWriteMask> for ColorWriteMask {
    fn from(value: ffi::WGPUColorWriteMask) -> Self {
        ColorWriteMask::from_bits_truncate(value as u64)
    }
}
impl From<ColorWriteMask> for ffi::WGPUColorWriteMask {
    fn from(value: ColorWriteMask) -> Self {
        value.bits() as ffi::WGPUColorWriteMask
    }
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct HeapProperty : u64 {
    const NONE = ffi::WGPUHeapProperty_None as u64; const DEVICE_LOCAL =
    ffi::WGPUHeapProperty_DeviceLocal as u64; const HOST_VISIBLE =
    ffi::WGPUHeapProperty_HostVisible as u64; const HOST_COHERENT =
    ffi::WGPUHeapProperty_HostCoherent as u64; const HOST_UNCACHED =
    ffi::WGPUHeapProperty_HostUncached as u64; const HOST_CACHED =
    ffi::WGPUHeapProperty_HostCached as u64; }
}
impl From<ffi::WGPUHeapProperty> for HeapProperty {
    fn from(value: ffi::WGPUHeapProperty) -> Self {
        HeapProperty::from_bits_truncate(value as u64)
    }
}
impl From<HeapProperty> for ffi::WGPUHeapProperty {
    fn from(value: HeapProperty) -> Self {
        value.bits() as ffi::WGPUHeapProperty
    }
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct MapMode : u64 { const
    NONE = ffi::WGPUMapMode_None as u64; const READ = ffi::WGPUMapMode_Read as u64; const
    WRITE = ffi::WGPUMapMode_Write as u64; }
}
impl From<ffi::WGPUMapMode> for MapMode {
    fn from(value: ffi::WGPUMapMode) -> Self {
        MapMode::from_bits_truncate(value as u64)
    }
}
impl From<MapMode> for ffi::WGPUMapMode {
    fn from(value: MapMode) -> Self {
        value.bits() as ffi::WGPUMapMode
    }
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct ShaderStage : u64 {
    const NONE = ffi::WGPUShaderStage_None as u64; const VERTEX =
    ffi::WGPUShaderStage_Vertex as u64; const FRAGMENT = ffi::WGPUShaderStage_Fragment as
    u64; const COMPUTE = ffi::WGPUShaderStage_Compute as u64; }
}
impl From<ffi::WGPUShaderStage> for ShaderStage {
    fn from(value: ffi::WGPUShaderStage) -> Self {
        ShaderStage::from_bits_truncate(value as u64)
    }
}
impl From<ShaderStage> for ffi::WGPUShaderStage {
    fn from(value: ShaderStage) -> Self {
        value.bits() as ffi::WGPUShaderStage
    }
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct TextureUsage : u64 {
    const NONE = ffi::WGPUTextureUsage_None as u64; const COPY_SRC =
    ffi::WGPUTextureUsage_CopySrc as u64; const COPY_DST = ffi::WGPUTextureUsage_CopyDst
    as u64; const TEXTURE_BINDING = ffi::WGPUTextureUsage_TextureBinding as u64; const
    STORAGE_BINDING = ffi::WGPUTextureUsage_StorageBinding as u64; const
    RENDER_ATTACHMENT = ffi::WGPUTextureUsage_RenderAttachment as u64; const
    TRANSIENT_ATTACHMENT = ffi::WGPUTextureUsage_TransientAttachment as u64; const
    STORAGE_ATTACHMENT = ffi::WGPUTextureUsage_StorageAttachment as u64; }
}
impl From<ffi::WGPUTextureUsage> for TextureUsage {
    fn from(value: ffi::WGPUTextureUsage) -> Self {
        TextureUsage::from_bits_truncate(value as u64)
    }
}
impl From<TextureUsage> for ffi::WGPUTextureUsage {
    fn from(value: TextureUsage) -> Self {
        value.bits() as ffi::WGPUTextureUsage
    }
}
