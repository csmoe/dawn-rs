#![allow(dead_code, unused_imports)]

use crate::ffi;
use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum WGSLLanguageFeatureName {
    ReadonlyAndReadwriteStorageTextures = 1,
    Packed4X8IntegerDotProduct = 2,
    UnrestrictedPointerParameters = 3,
    PointerCompositeAccess = 4,
    UniformBufferStandardLayout = 5,
    SubgroupId = 6,
    TextureAndSamplerLet = 7,
    SizedBindingArray = 327688,
    TexelBuffers = 327689,
    ChromiumPrint = 327690,
    FragmentDepth = 327691,
    ImmediateAddressSpace = 327692,
    SubgroupUniformity = 327693,
    BufferView = 327694,
    ChromiumTestingUnimplemented = 327680,
    ChromiumTestingUnsafeExperimental = 327681,
    ChromiumTestingExperimental = 327682,
    ChromiumTestingShippedWithKillswitch = 327683,
    ChromiumTestingShipped = 327684,
}

impl From<ffi::WGPUWGSLLanguageFeatureName> for WGSLLanguageFeatureName {
    fn from(value: ffi::WGPUWGSLLanguageFeatureName) -> Self {
        match value as u32 {
            1 => WGSLLanguageFeatureName::ReadonlyAndReadwriteStorageTextures,
            2 => WGSLLanguageFeatureName::Packed4X8IntegerDotProduct,
            3 => WGSLLanguageFeatureName::UnrestrictedPointerParameters,
            4 => WGSLLanguageFeatureName::PointerCompositeAccess,
            5 => WGSLLanguageFeatureName::UniformBufferStandardLayout,
            6 => WGSLLanguageFeatureName::SubgroupId,
            7 => WGSLLanguageFeatureName::TextureAndSamplerLet,
            327688 => WGSLLanguageFeatureName::SizedBindingArray,
            327689 => WGSLLanguageFeatureName::TexelBuffers,
            327690 => WGSLLanguageFeatureName::ChromiumPrint,
            327691 => WGSLLanguageFeatureName::FragmentDepth,
            327692 => WGSLLanguageFeatureName::ImmediateAddressSpace,
            327693 => WGSLLanguageFeatureName::SubgroupUniformity,
            327694 => WGSLLanguageFeatureName::BufferView,
            327680 => WGSLLanguageFeatureName::ChromiumTestingUnimplemented,
            327681 => WGSLLanguageFeatureName::ChromiumTestingUnsafeExperimental,
            327682 => WGSLLanguageFeatureName::ChromiumTestingExperimental,
            327683 => WGSLLanguageFeatureName::ChromiumTestingShippedWithKillswitch,
            327684 => WGSLLanguageFeatureName::ChromiumTestingShipped,
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
    DiscreteGpu = 1,
    IntegratedGpu = 2,
    Cpu = 3,
    Unknown = 4,
}

impl From<ffi::WGPUAdapterType> for AdapterType {
    fn from(value: ffi::WGPUAdapterType) -> Self {
        match value as u32 {
            1 => AdapterType::DiscreteGpu,
            2 => AdapterType::IntegratedGpu,
            3 => AdapterType::Cpu,
            4 => AdapterType::Unknown,
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
    Undefined = 0,
    ClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
}

impl From<ffi::WGPUAddressMode> for AddressMode {
    fn from(value: ffi::WGPUAddressMode) -> Self {
        match value as u32 {
            0 => AddressMode::Undefined,
            1 => AddressMode::ClampToEdge,
            2 => AddressMode::Repeat,
            3 => AddressMode::MirrorRepeat,
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
    Opaque = 1,
    Premultiplied = 2,
    Unpremultiplied = 3,
}

impl From<ffi::WGPUAlphaMode> for AlphaMode {
    fn from(value: ffi::WGPUAlphaMode) -> Self {
        match value as u32 {
            1 => AlphaMode::Opaque,
            2 => AlphaMode::Premultiplied,
            3 => AlphaMode::Unpremultiplied,
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
    Undefined = 0,
    Null = 1,
    WebGPU = 2,
    D3D11 = 3,
    D3D12 = 4,
    Metal = 5,
    Vulkan = 6,
    OpenGL = 7,
    OpenGLes = 8,
}

impl From<ffi::WGPUBackendType> for BackendType {
    fn from(value: ffi::WGPUBackendType) -> Self {
        match value as u32 {
            0 => BackendType::Undefined,
            1 => BackendType::Null,
            2 => BackendType::WebGPU,
            3 => BackendType::D3D11,
            4 => BackendType::D3D12,
            5 => BackendType::Metal,
            6 => BackendType::Vulkan,
            7 => BackendType::OpenGL,
            8 => BackendType::OpenGLes,
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
    Undefined = 0,
    Zero = 1,
    One = 2,
    Src = 3,
    OneMinusSrc = 4,
    SrcAlpha = 5,
    OneMinusSrcAlpha = 6,
    Dst = 7,
    OneMinusDst = 8,
    DstAlpha = 9,
    OneMinusDstAlpha = 10,
    SrcAlphaSaturated = 11,
    Constant = 12,
    OneMinusConstant = 13,
    Src1 = 14,
    OneMinusSrc1 = 15,
    Src1Alpha = 16,
    OneMinusSrc1Alpha = 17,
}

impl From<ffi::WGPUBlendFactor> for BlendFactor {
    fn from(value: ffi::WGPUBlendFactor) -> Self {
        match value as u32 {
            0 => BlendFactor::Undefined,
            1 => BlendFactor::Zero,
            2 => BlendFactor::One,
            3 => BlendFactor::Src,
            4 => BlendFactor::OneMinusSrc,
            5 => BlendFactor::SrcAlpha,
            6 => BlendFactor::OneMinusSrcAlpha,
            7 => BlendFactor::Dst,
            8 => BlendFactor::OneMinusDst,
            9 => BlendFactor::DstAlpha,
            10 => BlendFactor::OneMinusDstAlpha,
            11 => BlendFactor::SrcAlphaSaturated,
            12 => BlendFactor::Constant,
            13 => BlendFactor::OneMinusConstant,
            14 => BlendFactor::Src1,
            15 => BlendFactor::OneMinusSrc1,
            16 => BlendFactor::Src1Alpha,
            17 => BlendFactor::OneMinusSrc1Alpha,
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
    Undefined = 0,
    Add = 1,
    Subtract = 2,
    ReverseSubtract = 3,
    Min = 4,
    Max = 5,
}

impl From<ffi::WGPUBlendOperation> for BlendOperation {
    fn from(value: ffi::WGPUBlendOperation) -> Self {
        match value as u32 {
            0 => BlendOperation::Undefined,
            1 => BlendOperation::Add,
            2 => BlendOperation::Subtract,
            3 => BlendOperation::ReverseSubtract,
            4 => BlendOperation::Min,
            5 => BlendOperation::Max,
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
    BindingNotUsed = 0,
    Undefined = 1,
    Uniform = 2,
    Storage = 3,
    ReadOnlyStorage = 4,
}

impl From<ffi::WGPUBufferBindingType> for BufferBindingType {
    fn from(value: ffi::WGPUBufferBindingType) -> Self {
        match value as u32 {
            0 => BufferBindingType::BindingNotUsed,
            1 => BufferBindingType::Undefined,
            2 => BufferBindingType::Uniform,
            3 => BufferBindingType::Storage,
            4 => BufferBindingType::ReadOnlyStorage,
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
    Unmapped = 1,
    Pending = 2,
    Mapped = 3,
}

impl From<ffi::WGPUBufferMapState> for BufferMapState {
    fn from(value: ffi::WGPUBufferMapState) -> Self {
        match value as u32 {
            1 => BufferMapState::Unmapped,
            2 => BufferMapState::Pending,
            3 => BufferMapState::Mapped,
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
    WaitAnyOnly = 1,
    AllowProcessEvents = 2,
    AllowSpontaneous = 3,
}

impl From<ffi::WGPUCallbackMode> for CallbackMode {
    fn from(value: ffi::WGPUCallbackMode) -> Self {
        match value as u32 {
            1 => CallbackMode::WaitAnyOnly,
            2 => CallbackMode::AllowProcessEvents,
            3 => CallbackMode::AllowSpontaneous,
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
    Undefined = 0,
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8,
}

impl From<ffi::WGPUCompareFunction> for CompareFunction {
    fn from(value: ffi::WGPUCompareFunction) -> Self {
        match value as u32 {
            0 => CompareFunction::Undefined,
            1 => CompareFunction::Never,
            2 => CompareFunction::Less,
            3 => CompareFunction::Equal,
            4 => CompareFunction::LessEqual,
            5 => CompareFunction::Greater,
            6 => CompareFunction::NotEqual,
            7 => CompareFunction::GreaterEqual,
            8 => CompareFunction::Always,
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
    Success = 1,
    CallbackCancelled = 2,
}

impl From<ffi::WGPUCompilationInfoRequestStatus> for CompilationInfoRequestStatus {
    fn from(value: ffi::WGPUCompilationInfoRequestStatus) -> Self {
        match value as u32 {
            1 => CompilationInfoRequestStatus::Success,
            2 => CompilationInfoRequestStatus::CallbackCancelled,
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
    Error = 1,
    Warning = 2,
    Info = 3,
}

impl From<ffi::WGPUCompilationMessageType> for CompilationMessageType {
    fn from(value: ffi::WGPUCompilationMessageType) -> Self {
        match value as u32 {
            1 => CompilationMessageType::Error,
            2 => CompilationMessageType::Warning,
            3 => CompilationMessageType::Info,
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
    Undefined = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}

impl From<ffi::WGPUComponentSwizzle> for ComponentSwizzle {
    fn from(value: ffi::WGPUComponentSwizzle) -> Self {
        match value as u32 {
            0 => ComponentSwizzle::Undefined,
            1 => ComponentSwizzle::Zero,
            2 => ComponentSwizzle::One,
            3 => ComponentSwizzle::R,
            4 => ComponentSwizzle::G,
            5 => ComponentSwizzle::B,
            6 => ComponentSwizzle::A,
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
    Auto = 0,
    Opaque = 1,
    Premultiplied = 2,
    Unpremultiplied = 3,
    Inherit = 4,
}

impl From<ffi::WGPUCompositeAlphaMode> for CompositeAlphaMode {
    fn from(value: ffi::WGPUCompositeAlphaMode) -> Self {
        match value as u32 {
            0 => CompositeAlphaMode::Auto,
            1 => CompositeAlphaMode::Opaque,
            2 => CompositeAlphaMode::Premultiplied,
            3 => CompositeAlphaMode::Unpremultiplied,
            4 => CompositeAlphaMode::Inherit,
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
    Success = 1,
    CallbackCancelled = 2,
    ValidationError = 3,
    InternalError = 4,
}

impl From<ffi::WGPUCreatePipelineAsyncStatus> for CreatePipelineAsyncStatus {
    fn from(value: ffi::WGPUCreatePipelineAsyncStatus) -> Self {
        match value as u32 {
            1 => CreatePipelineAsyncStatus::Success,
            2 => CreatePipelineAsyncStatus::CallbackCancelled,
            3 => CreatePipelineAsyncStatus::ValidationError,
            4 => CreatePipelineAsyncStatus::InternalError,
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
    Undefined = 0,
    None = 1,
    Front = 2,
    Back = 3,
}

impl From<ffi::WGPUCullMode> for CullMode {
    fn from(value: ffi::WGPUCullMode) -> Self {
        match value as u32 {
            0 => CullMode::Undefined,
            1 => CullMode::None,
            2 => CullMode::Front,
            3 => CullMode::Back,
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
    Unknown = 1,
    Destroyed = 2,
    CallbackCancelled = 3,
    FailedCreation = 4,
}

impl From<ffi::WGPUDeviceLostReason> for DeviceLostReason {
    fn from(value: ffi::WGPUDeviceLostReason) -> Self {
        match value as u32 {
            1 => DeviceLostReason::Unknown,
            2 => DeviceLostReason::Destroyed,
            3 => DeviceLostReason::CallbackCancelled,
            4 => DeviceLostReason::FailedCreation,
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
    Validation = 1,
    OutOfMemory = 2,
    Internal = 3,
}

impl From<ffi::WGPUErrorFilter> for ErrorFilter {
    fn from(value: ffi::WGPUErrorFilter) -> Self {
        match value as u32 {
            1 => ErrorFilter::Validation,
            2 => ErrorFilter::OutOfMemory,
            3 => ErrorFilter::Internal,
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
    NoError = 1,
    Validation = 2,
    OutOfMemory = 3,
    Internal = 4,
    Unknown = 5,
}

impl From<ffi::WGPUErrorType> for ErrorType {
    fn from(value: ffi::WGPUErrorType) -> Self {
        match value as u32 {
            1 => ErrorType::NoError,
            2 => ErrorType::Validation,
            3 => ErrorType::OutOfMemory,
            4 => ErrorType::Internal,
            5 => ErrorType::Unknown,
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
    Rotate0Degrees = 1,
    Rotate90Degrees = 2,
    Rotate180Degrees = 3,
    Rotate270Degrees = 4,
}

impl From<ffi::WGPUExternalTextureRotation> for ExternalTextureRotation {
    fn from(value: ffi::WGPUExternalTextureRotation) -> Self {
        match value as u32 {
            1 => ExternalTextureRotation::Rotate0Degrees,
            2 => ExternalTextureRotation::Rotate90Degrees,
            3 => ExternalTextureRotation::Rotate180Degrees,
            4 => ExternalTextureRotation::Rotate270Degrees,
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
    Undefined = 0,
    Compatibility = 1,
    Core = 2,
}

impl From<ffi::WGPUFeatureLevel> for FeatureLevel {
    fn from(value: ffi::WGPUFeatureLevel) -> Self {
        match value as u32 {
            0 => FeatureLevel::Undefined,
            1 => FeatureLevel::Compatibility,
            2 => FeatureLevel::Core,
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
    CoreFeaturesAndLimits = 1,
    DepthClipControl = 2,
    Depth32FloatStencil8 = 3,
    TextureCompressionBc = 4,
    TextureCompressionBcSliced3D = 5,
    TextureCompressionEtc2 = 6,
    TextureCompressionAstc = 7,
    TextureCompressionAstcSliced3D = 8,
    TimestampQuery = 9,
    IndirectFirstInstance = 10,
    ShaderF16 = 11,
    Rg11B10UfloatRenderable = 12,
    Bgra8UnormStorage = 13,
    Float32Filterable = 14,
    Float32Blendable = 15,
    ClipDistances = 16,
    DualSourceBlending = 17,
    Subgroups = 18,
    TextureFormatsTier1 = 19,
    TextureFormatsTier2 = 20,
    PrimitiveIndex = 21,
    TextureComponentSwizzle = 22,
    DawnInternalUsages = 327680,
    DawnMultiPlanarFormats = 327681,
    DawnNative = 327682,
    ChromiumExperimentalTimestampQueryInsidePasses = 327683,
    ImplicitDeviceSynchronization = 327684,
    TransientAttachments = 327686,
    MsaaRenderToSingleSampled = 327687,
    D3D11MultithreadProtected = 327688,
    AngleTextureSharing = 327689,
    PixelLocalStorageCoherent = 327690,
    PixelLocalStorageNonCoherent = 327691,
    Unorm16TextureFormats = 262156,
    MultiPlanarFormatExtendedUsages = 327693,
    MultiPlanarFormatP010 = 327694,
    HostMappedPointer = 327695,
    MultiPlanarRenderTargets = 327696,
    MultiPlanarFormatNv12A = 327697,
    FramebufferFetch = 327698,
    BufferMapExtendedUsages = 327699,
    AdapterPropertiesMemoryHeaps = 327700,
    AdapterPropertiesD3D = 327701,
    AdapterPropertiesVk = 327702,
    DawnFormatCapabilities = 327703,
    DawnDrmFormatCapabilities = 327704,
    MultiPlanarFormatNv16 = 327705,
    MultiPlanarFormatNv24 = 327706,
    MultiPlanarFormatP210 = 327707,
    MultiPlanarFormatP410 = 327708,
    SharedTextureMemoryVkDedicatedAllocation = 327709,
    SharedTextureMemoryAHardwareBuffer = 327710,
    SharedTextureMemoryDmaBuf = 327711,
    SharedTextureMemoryOpaqueFD = 327712,
    SharedTextureMemoryZirconHandle = 327713,
    SharedTextureMemoryDXGISharedHandle = 327714,
    SharedTextureMemoryD3D11Texture2D = 327715,
    SharedTextureMemoryIOSurface = 327716,
    SharedTextureMemoryEGLImage = 327717,
    SharedFenceVkSemaphoreOpaqueFD = 327718,
    SharedFenceSyncFD = 327719,
    SharedFenceVkSemaphoreZirconHandle = 327720,
    SharedFenceDXGISharedHandle = 327721,
    SharedFenceMTLSharedEvent = 327722,
    SharedBufferMemoryD3D12Resource = 327723,
    StaticSamplers = 327724,
    YCbCrVulkanSamplers = 327725,
    ShaderModuleCompilationOptions = 327726,
    DawnLoadResolveTexture = 327727,
    DawnPartialLoadResolveTexture = 327728,
    MultiDrawIndirect = 262193,
    DawnTexelCopyBufferRowAlignment = 327730,
    FlexibleTextureViews = 327731,
    ChromiumExperimentalSubgroupMatrix = 327732,
    SharedFenceEGLSync = 327733,
    DawnDeviceAllocatorControl = 327734,
    AdapterPropertiesWGPU = 327735,
    SharedBufferMemoryD3D12SharedMemoryFileMappingHandle = 327736,
    SharedTextureMemoryD3D12Resource = 327737,
    ChromiumExperimentalSamplingResourceTable = 327738,
    ChromiumExperimentalSubgroupSizeControl = 327739,
}

impl From<ffi::WGPUFeatureName> for FeatureName {
    fn from(value: ffi::WGPUFeatureName) -> Self {
        match value as u32 {
            1 => FeatureName::CoreFeaturesAndLimits,
            2 => FeatureName::DepthClipControl,
            3 => FeatureName::Depth32FloatStencil8,
            4 => FeatureName::TextureCompressionBc,
            5 => FeatureName::TextureCompressionBcSliced3D,
            6 => FeatureName::TextureCompressionEtc2,
            7 => FeatureName::TextureCompressionAstc,
            8 => FeatureName::TextureCompressionAstcSliced3D,
            9 => FeatureName::TimestampQuery,
            10 => FeatureName::IndirectFirstInstance,
            11 => FeatureName::ShaderF16,
            12 => FeatureName::Rg11B10UfloatRenderable,
            13 => FeatureName::Bgra8UnormStorage,
            14 => FeatureName::Float32Filterable,
            15 => FeatureName::Float32Blendable,
            16 => FeatureName::ClipDistances,
            17 => FeatureName::DualSourceBlending,
            18 => FeatureName::Subgroups,
            19 => FeatureName::TextureFormatsTier1,
            20 => FeatureName::TextureFormatsTier2,
            21 => FeatureName::PrimitiveIndex,
            22 => FeatureName::TextureComponentSwizzle,
            327680 => FeatureName::DawnInternalUsages,
            327681 => FeatureName::DawnMultiPlanarFormats,
            327682 => FeatureName::DawnNative,
            327683 => FeatureName::ChromiumExperimentalTimestampQueryInsidePasses,
            327684 => FeatureName::ImplicitDeviceSynchronization,
            327686 => FeatureName::TransientAttachments,
            327687 => FeatureName::MsaaRenderToSingleSampled,
            327688 => FeatureName::D3D11MultithreadProtected,
            327689 => FeatureName::AngleTextureSharing,
            327690 => FeatureName::PixelLocalStorageCoherent,
            327691 => FeatureName::PixelLocalStorageNonCoherent,
            262156 => FeatureName::Unorm16TextureFormats,
            327693 => FeatureName::MultiPlanarFormatExtendedUsages,
            327694 => FeatureName::MultiPlanarFormatP010,
            327695 => FeatureName::HostMappedPointer,
            327696 => FeatureName::MultiPlanarRenderTargets,
            327697 => FeatureName::MultiPlanarFormatNv12A,
            327698 => FeatureName::FramebufferFetch,
            327699 => FeatureName::BufferMapExtendedUsages,
            327700 => FeatureName::AdapterPropertiesMemoryHeaps,
            327701 => FeatureName::AdapterPropertiesD3D,
            327702 => FeatureName::AdapterPropertiesVk,
            327703 => FeatureName::DawnFormatCapabilities,
            327704 => FeatureName::DawnDrmFormatCapabilities,
            327705 => FeatureName::MultiPlanarFormatNv16,
            327706 => FeatureName::MultiPlanarFormatNv24,
            327707 => FeatureName::MultiPlanarFormatP210,
            327708 => FeatureName::MultiPlanarFormatP410,
            327709 => FeatureName::SharedTextureMemoryVkDedicatedAllocation,
            327710 => FeatureName::SharedTextureMemoryAHardwareBuffer,
            327711 => FeatureName::SharedTextureMemoryDmaBuf,
            327712 => FeatureName::SharedTextureMemoryOpaqueFD,
            327713 => FeatureName::SharedTextureMemoryZirconHandle,
            327714 => FeatureName::SharedTextureMemoryDXGISharedHandle,
            327715 => FeatureName::SharedTextureMemoryD3D11Texture2D,
            327716 => FeatureName::SharedTextureMemoryIOSurface,
            327717 => FeatureName::SharedTextureMemoryEGLImage,
            327718 => FeatureName::SharedFenceVkSemaphoreOpaqueFD,
            327719 => FeatureName::SharedFenceSyncFD,
            327720 => FeatureName::SharedFenceVkSemaphoreZirconHandle,
            327721 => FeatureName::SharedFenceDXGISharedHandle,
            327722 => FeatureName::SharedFenceMTLSharedEvent,
            327723 => FeatureName::SharedBufferMemoryD3D12Resource,
            327724 => FeatureName::StaticSamplers,
            327725 => FeatureName::YCbCrVulkanSamplers,
            327726 => FeatureName::ShaderModuleCompilationOptions,
            327727 => FeatureName::DawnLoadResolveTexture,
            327728 => FeatureName::DawnPartialLoadResolveTexture,
            262193 => FeatureName::MultiDrawIndirect,
            327730 => FeatureName::DawnTexelCopyBufferRowAlignment,
            327731 => FeatureName::FlexibleTextureViews,
            327732 => FeatureName::ChromiumExperimentalSubgroupMatrix,
            327733 => FeatureName::SharedFenceEGLSync,
            327734 => FeatureName::DawnDeviceAllocatorControl,
            327735 => FeatureName::AdapterPropertiesWGPU,
            327736 => FeatureName::SharedBufferMemoryD3D12SharedMemoryFileMappingHandle,
            327737 => FeatureName::SharedTextureMemoryD3D12Resource,
            327738 => FeatureName::ChromiumExperimentalSamplingResourceTable,
            327739 => FeatureName::ChromiumExperimentalSubgroupSizeControl,
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
    Undefined = 0,
    Nearest = 1,
    Linear = 2,
}

impl From<ffi::WGPUFilterMode> for FilterMode {
    fn from(value: ffi::WGPUFilterMode) -> Self {
        match value as u32 {
            0 => FilterMode::Undefined,
            1 => FilterMode::Nearest,
            2 => FilterMode::Linear,
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
    Undefined = 0,
    Ccw = 1,
    Cw = 2,
}

impl From<ffi::WGPUFrontFace> for FrontFace {
    fn from(value: ffi::WGPUFrontFace) -> Self {
        match value as u32 {
            0 => FrontFace::Undefined,
            1 => FrontFace::Ccw,
            2 => FrontFace::Cw,
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
    Undefined = 0,
    Uint16 = 1,
    Uint32 = 2,
}

impl From<ffi::WGPUIndexFormat> for IndexFormat {
    fn from(value: ffi::WGPUIndexFormat) -> Self {
        match value as u32 {
            0 => IndexFormat::Undefined,
            1 => IndexFormat::Uint16,
            2 => IndexFormat::Uint32,
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
    TimedWaitAny = 1,
    ShaderSourceSPIRV = 2,
    MultipleDevicesPerAdapter = 3,
}

impl From<ffi::WGPUInstanceFeatureName> for InstanceFeatureName {
    fn from(value: ffi::WGPUInstanceFeatureName) -> Self {
        match value as u32 {
            1 => InstanceFeatureName::TimedWaitAny,
            2 => InstanceFeatureName::ShaderSourceSPIRV,
            3 => InstanceFeatureName::MultipleDevicesPerAdapter,
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
    Undefined = 0,
    Load = 1,
    Clear = 2,
    ExpandResolveTexture = 327683,
}

impl From<ffi::WGPULoadOp> for LoadOp {
    fn from(value: ffi::WGPULoadOp) -> Self {
        match value as u32 {
            0 => LoadOp::Undefined,
            1 => LoadOp::Load,
            2 => LoadOp::Clear,
            327683 => LoadOp::ExpandResolveTexture,
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
    Verbose = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
}

impl From<ffi::WGPULoggingType> for LoggingType {
    fn from(value: ffi::WGPULoggingType) -> Self {
        match value as u32 {
            1 => LoggingType::Verbose,
            2 => LoggingType::Info,
            3 => LoggingType::Warning,
            4 => LoggingType::Error,
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
    Success = 1,
    CallbackCancelled = 2,
    Error = 3,
    Aborted = 4,
}

impl From<ffi::WGPUMapAsyncStatus> for MapAsyncStatus {
    fn from(value: ffi::WGPUMapAsyncStatus) -> Self {
        match value as u32 {
            1 => MapAsyncStatus::Success,
            2 => MapAsyncStatus::CallbackCancelled,
            3 => MapAsyncStatus::Error,
            4 => MapAsyncStatus::Aborted,
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
    Undefined = 0,
    Nearest = 1,
    Linear = 2,
}

impl From<ffi::WGPUMipmapFilterMode> for MipmapFilterMode {
    fn from(value: ffi::WGPUMipmapFilterMode) -> Self {
        match value as u32 {
            0 => MipmapFilterMode::Undefined,
            1 => MipmapFilterMode::Nearest,
            2 => MipmapFilterMode::Linear,
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
    False = 0,
    True = 1,
    Undefined = 2,
}

impl From<ffi::WGPUOptionalBool> for OptionalBool {
    fn from(value: ffi::WGPUOptionalBool) -> Self {
        match value as u32 {
            0 => OptionalBool::False,
            1 => OptionalBool::True,
            2 => OptionalBool::Undefined,
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
    Success = 1,
    CallbackCancelled = 2,
    Error = 3,
}

impl From<ffi::WGPUPopErrorScopeStatus> for PopErrorScopeStatus {
    fn from(value: ffi::WGPUPopErrorScopeStatus) -> Self {
        match value as u32 {
            1 => PopErrorScopeStatus::Success,
            2 => PopErrorScopeStatus::CallbackCancelled,
            3 => PopErrorScopeStatus::Error,
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
    Undefined = 0,
    LowPower = 1,
    HighPerformance = 2,
}

impl From<ffi::WGPUPowerPreference> for PowerPreference {
    fn from(value: ffi::WGPUPowerPreference) -> Self {
        match value as u32 {
            0 => PowerPreference::Undefined,
            1 => PowerPreference::LowPower,
            2 => PowerPreference::HighPerformance,
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
    SRgb = 1,
    DisplayP3 = 2,
}

impl From<ffi::WGPUPredefinedColorSpace> for PredefinedColorSpace {
    fn from(value: ffi::WGPUPredefinedColorSpace) -> Self {
        match value as u32 {
            1 => PredefinedColorSpace::SRgb,
            2 => PredefinedColorSpace::DisplayP3,
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
    Undefined = 0,
    Fifo = 1,
    FifoRelaxed = 2,
    Immediate = 3,
    Mailbox = 4,
}

impl From<ffi::WGPUPresentMode> for PresentMode {
    fn from(value: ffi::WGPUPresentMode) -> Self {
        match value as u32 {
            0 => PresentMode::Undefined,
            1 => PresentMode::Fifo,
            2 => PresentMode::FifoRelaxed,
            3 => PresentMode::Immediate,
            4 => PresentMode::Mailbox,
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
    Undefined = 0,
    PointList = 1,
    LineList = 2,
    LineStrip = 3,
    TriangleList = 4,
    TriangleStrip = 5,
}

impl From<ffi::WGPUPrimitiveTopology> for PrimitiveTopology {
    fn from(value: ffi::WGPUPrimitiveTopology) -> Self {
        match value as u32 {
            0 => PrimitiveTopology::Undefined,
            1 => PrimitiveTopology::PointList,
            2 => PrimitiveTopology::LineList,
            3 => PrimitiveTopology::LineStrip,
            4 => PrimitiveTopology::TriangleList,
            5 => PrimitiveTopology::TriangleStrip,
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
    Occlusion = 1,
    Timestamp = 2,
}

impl From<ffi::WGPUQueryType> for QueryType {
    fn from(value: ffi::WGPUQueryType) -> Self {
        match value as u32 {
            1 => QueryType::Occlusion,
            2 => QueryType::Timestamp,
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
    Success = 1,
    CallbackCancelled = 2,
    Error = 3,
}

impl From<ffi::WGPUQueueWorkDoneStatus> for QueueWorkDoneStatus {
    fn from(value: ffi::WGPUQueueWorkDoneStatus) -> Self {
        match value as u32 {
            1 => QueueWorkDoneStatus::Success,
            2 => QueueWorkDoneStatus::CallbackCancelled,
            3 => QueueWorkDoneStatus::Error,
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
    Success = 1,
    CallbackCancelled = 2,
    Unavailable = 3,
    Error = 4,
}

impl From<ffi::WGPURequestAdapterStatus> for RequestAdapterStatus {
    fn from(value: ffi::WGPURequestAdapterStatus) -> Self {
        match value as u32 {
            1 => RequestAdapterStatus::Success,
            2 => RequestAdapterStatus::CallbackCancelled,
            3 => RequestAdapterStatus::Unavailable,
            4 => RequestAdapterStatus::Error,
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
    Success = 1,
    CallbackCancelled = 2,
    Error = 3,
}

impl From<ffi::WGPURequestDeviceStatus> for RequestDeviceStatus {
    fn from(value: ffi::WGPURequestDeviceStatus) -> Self {
        match value as u32 {
            1 => RequestDeviceStatus::Success,
            2 => RequestDeviceStatus::CallbackCancelled,
            3 => RequestDeviceStatus::Error,
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
    ShaderSourceSPIRV = 1,
    ShaderSourceWGSL = 2,
    RenderPassMaxDrawCount = 3,
    SurfaceSourceMetalLayer = 4,
    SurfaceSourceWindowsHWND = 5,
    SurfaceSourceXlibWindow = 6,
    SurfaceSourceWaylandSurface = 7,
    SurfaceSourceAndroidNativeWindow = 8,
    SurfaceSourceXCBWindow = 9,
    SurfaceColorManagement = 10,
    RequestAdapterWebXROptions = 11,
    TextureComponentSwizzleDescriptor = 12,
    CompatibilityModeLimits = 131072,
    TextureBindingViewDimensionDescriptor = 131073,
    EmscriptenSurfaceSourceCanvasHtmlSelector = 262144,
    SurfaceDescriptorFromWindowsCoreWindow = 327680,
    ExternalTextureBindingEntry = 262145,
    ExternalTextureBindingLayout = 262146,
    SurfaceDescriptorFromWindowsUWPSwapChainPanel = 327683,
    DawnTextureInternalUsageDescriptor = 327684,
    DawnEncoderInternalUsageDescriptor = 327685,
    DawnInstanceDescriptor = 327686,
    DawnCacheDeviceDescriptor = 327687,
    DawnAdapterPropertiesPowerPreference = 327688,
    DawnBufferDescriptorErrorInfoFromWireClient = 327689,
    DawnTogglesDescriptor = 327690,
    DawnShaderModuleSPIRVOptionsDescriptor = 327691,
    RequestAdapterOptionsLuid = 327692,
    RequestAdapterOptionsGetGlProc = 327693,
    RequestAdapterOptionsD3D11Device = 327694,
    DawnRenderPassColorAttachmentRenderToSingleSampled = 327695,
    RenderPassPixelLocalStorage = 327696,
    PipelineLayoutPixelLocalStorage = 327697,
    BufferHostMappedPointer = 327698,
    AdapterPropertiesMemoryHeaps = 327699,
    AdapterPropertiesD3D = 327700,
    AdapterPropertiesVk = 327701,
    DawnWireWGSLControl = 327702,
    DawnWGSLBlocklist = 327703,
    DawnDrmFormatCapabilities = 327704,
    ShaderModuleCompilationOptions = 327705,
    ColorTargetStateExpandResolveTextureDawn = 327706,
    RenderPassDescriptorExpandResolveRect = 327707,
    SharedTextureMemoryVkDedicatedAllocationDescriptor = 327708,
    SharedTextureMemoryAHardwareBufferDescriptor = 327709,
    SharedTextureMemoryDmaBufDescriptor = 327710,
    SharedTextureMemoryOpaqueFDDescriptor = 327711,
    SharedTextureMemoryZirconHandleDescriptor = 327712,
    SharedTextureMemoryDXGISharedHandleDescriptor = 327713,
    SharedTextureMemoryD3D11Texture2DDescriptor = 327714,
    SharedTextureMemoryIOSurfaceDescriptor = 327715,
    SharedTextureMemoryEGLImageDescriptor = 327716,
    SharedTextureMemoryInitializedBeginState = 327717,
    SharedTextureMemoryInitializedEndState = 327718,
    SharedTextureMemoryVkImageLayoutBeginState = 327719,
    SharedTextureMemoryVkImageLayoutEndState = 327720,
    SharedTextureMemoryD3DSwapchainBeginState = 327721,
    SharedFenceVkSemaphoreOpaqueFDDescriptor = 327722,
    SharedFenceVkSemaphoreOpaqueFDExportInfo = 327723,
    SharedFenceSyncFDDescriptor = 327724,
    SharedFenceSyncFDExportInfo = 327725,
    SharedFenceVkSemaphoreZirconHandleDescriptor = 327726,
    SharedFenceVkSemaphoreZirconHandleExportInfo = 327727,
    SharedFenceDXGISharedHandleDescriptor = 327728,
    SharedFenceDXGISharedHandleExportInfo = 327729,
    SharedFenceMTLSharedEventDescriptor = 327730,
    SharedFenceMTLSharedEventExportInfo = 327731,
    SharedBufferMemoryD3D12ResourceDescriptor = 327732,
    StaticSamplerBindingLayout = 327733,
    YCbCrVkDescriptor = 327734,
    SharedTextureMemoryAHardwareBufferProperties = 327735,
    AHardwareBufferProperties = 327736,
    DawnTexelCopyBufferRowAlignmentLimits = 327738,
    AdapterPropertiesSubgroupMatrixConfigs = 327739,
    SharedFenceEGLSyncDescriptor = 327740,
    SharedFenceEGLSyncExportInfo = 327741,
    DawnInjectedInvalidSType = 327742,
    DawnCompilationMessageUtf16 = 262207,
    DawnFakeBufferOOMForTesting = 327744,
    SurfaceDescriptorFromWindowsWinUISwapChainPanel = 327745,
    DawnDeviceAllocatorControl = 327746,
    DawnHostMappedPointerLimits = 327747,
    RenderPassDescriptorResolveRect = 327748,
    RequestAdapterWebGPUBackendOptions = 327749,
    DawnFakeDeviceInitializeErrorForTesting = 327750,
    SharedTextureMemoryD3D11BeginState = 327751,
    DawnConsumeAdapterDescriptor = 327752,
    TexelBufferBindingEntry = 327753,
    TexelBufferBindingLayout = 327754,
    SharedTextureMemoryMetalEndAccessState = 327755,
    AdapterPropertiesWGPU = 327756,
    SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor = 327757,
    SharedTextureMemoryD3D12ResourceDescriptor = 327758,
    RequestAdapterOptionsAngleVirtualizationGroup = 327759,
    ResourceTableLimits = 327760,
    PipelineLayoutResourceTable = 327761,
    AdapterPropertiesExplicitComputeSubgroupSizeConfigs = 327762,
}

impl From<ffi::WGPUSType> for SType {
    fn from(value: ffi::WGPUSType) -> Self {
        match value as u32 {
            1 => SType::ShaderSourceSPIRV,
            2 => SType::ShaderSourceWGSL,
            3 => SType::RenderPassMaxDrawCount,
            4 => SType::SurfaceSourceMetalLayer,
            5 => SType::SurfaceSourceWindowsHWND,
            6 => SType::SurfaceSourceXlibWindow,
            7 => SType::SurfaceSourceWaylandSurface,
            8 => SType::SurfaceSourceAndroidNativeWindow,
            9 => SType::SurfaceSourceXCBWindow,
            10 => SType::SurfaceColorManagement,
            11 => SType::RequestAdapterWebXROptions,
            12 => SType::TextureComponentSwizzleDescriptor,
            131072 => SType::CompatibilityModeLimits,
            131073 => SType::TextureBindingViewDimensionDescriptor,
            262144 => SType::EmscriptenSurfaceSourceCanvasHtmlSelector,
            327680 => SType::SurfaceDescriptorFromWindowsCoreWindow,
            262145 => SType::ExternalTextureBindingEntry,
            262146 => SType::ExternalTextureBindingLayout,
            327683 => SType::SurfaceDescriptorFromWindowsUWPSwapChainPanel,
            327684 => SType::DawnTextureInternalUsageDescriptor,
            327685 => SType::DawnEncoderInternalUsageDescriptor,
            327686 => SType::DawnInstanceDescriptor,
            327687 => SType::DawnCacheDeviceDescriptor,
            327688 => SType::DawnAdapterPropertiesPowerPreference,
            327689 => SType::DawnBufferDescriptorErrorInfoFromWireClient,
            327690 => SType::DawnTogglesDescriptor,
            327691 => SType::DawnShaderModuleSPIRVOptionsDescriptor,
            327692 => SType::RequestAdapterOptionsLuid,
            327693 => SType::RequestAdapterOptionsGetGlProc,
            327694 => SType::RequestAdapterOptionsD3D11Device,
            327695 => SType::DawnRenderPassColorAttachmentRenderToSingleSampled,
            327696 => SType::RenderPassPixelLocalStorage,
            327697 => SType::PipelineLayoutPixelLocalStorage,
            327698 => SType::BufferHostMappedPointer,
            327699 => SType::AdapterPropertiesMemoryHeaps,
            327700 => SType::AdapterPropertiesD3D,
            327701 => SType::AdapterPropertiesVk,
            327702 => SType::DawnWireWGSLControl,
            327703 => SType::DawnWGSLBlocklist,
            327704 => SType::DawnDrmFormatCapabilities,
            327705 => SType::ShaderModuleCompilationOptions,
            327706 => SType::ColorTargetStateExpandResolveTextureDawn,
            327707 => SType::RenderPassDescriptorExpandResolveRect,
            327708 => SType::SharedTextureMemoryVkDedicatedAllocationDescriptor,
            327709 => SType::SharedTextureMemoryAHardwareBufferDescriptor,
            327710 => SType::SharedTextureMemoryDmaBufDescriptor,
            327711 => SType::SharedTextureMemoryOpaqueFDDescriptor,
            327712 => SType::SharedTextureMemoryZirconHandleDescriptor,
            327713 => SType::SharedTextureMemoryDXGISharedHandleDescriptor,
            327714 => SType::SharedTextureMemoryD3D11Texture2DDescriptor,
            327715 => SType::SharedTextureMemoryIOSurfaceDescriptor,
            327716 => SType::SharedTextureMemoryEGLImageDescriptor,
            327717 => SType::SharedTextureMemoryInitializedBeginState,
            327718 => SType::SharedTextureMemoryInitializedEndState,
            327719 => SType::SharedTextureMemoryVkImageLayoutBeginState,
            327720 => SType::SharedTextureMemoryVkImageLayoutEndState,
            327721 => SType::SharedTextureMemoryD3DSwapchainBeginState,
            327722 => SType::SharedFenceVkSemaphoreOpaqueFDDescriptor,
            327723 => SType::SharedFenceVkSemaphoreOpaqueFDExportInfo,
            327724 => SType::SharedFenceSyncFDDescriptor,
            327725 => SType::SharedFenceSyncFDExportInfo,
            327726 => SType::SharedFenceVkSemaphoreZirconHandleDescriptor,
            327727 => SType::SharedFenceVkSemaphoreZirconHandleExportInfo,
            327728 => SType::SharedFenceDXGISharedHandleDescriptor,
            327729 => SType::SharedFenceDXGISharedHandleExportInfo,
            327730 => SType::SharedFenceMTLSharedEventDescriptor,
            327731 => SType::SharedFenceMTLSharedEventExportInfo,
            327732 => SType::SharedBufferMemoryD3D12ResourceDescriptor,
            327733 => SType::StaticSamplerBindingLayout,
            327734 => SType::YCbCrVkDescriptor,
            327735 => SType::SharedTextureMemoryAHardwareBufferProperties,
            327736 => SType::AHardwareBufferProperties,
            327738 => SType::DawnTexelCopyBufferRowAlignmentLimits,
            327739 => SType::AdapterPropertiesSubgroupMatrixConfigs,
            327740 => SType::SharedFenceEGLSyncDescriptor,
            327741 => SType::SharedFenceEGLSyncExportInfo,
            327742 => SType::DawnInjectedInvalidSType,
            262207 => SType::DawnCompilationMessageUtf16,
            327744 => SType::DawnFakeBufferOOMForTesting,
            327745 => SType::SurfaceDescriptorFromWindowsWinUISwapChainPanel,
            327746 => SType::DawnDeviceAllocatorControl,
            327747 => SType::DawnHostMappedPointerLimits,
            327748 => SType::RenderPassDescriptorResolveRect,
            327749 => SType::RequestAdapterWebGPUBackendOptions,
            327750 => SType::DawnFakeDeviceInitializeErrorForTesting,
            327751 => SType::SharedTextureMemoryD3D11BeginState,
            327752 => SType::DawnConsumeAdapterDescriptor,
            327753 => SType::TexelBufferBindingEntry,
            327754 => SType::TexelBufferBindingLayout,
            327755 => SType::SharedTextureMemoryMetalEndAccessState,
            327756 => SType::AdapterPropertiesWGPU,
            327757 => SType::SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor,
            327758 => SType::SharedTextureMemoryD3D12ResourceDescriptor,
            327759 => SType::RequestAdapterOptionsAngleVirtualizationGroup,
            327760 => SType::ResourceTableLimits,
            327761 => SType::PipelineLayoutResourceTable,
            327762 => SType::AdapterPropertiesExplicitComputeSubgroupSizeConfigs,
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
    BindingNotUsed = 0,
    Undefined = 1,
    Filtering = 2,
    NonFiltering = 3,
    Comparison = 4,
}

impl From<ffi::WGPUSamplerBindingType> for SamplerBindingType {
    fn from(value: ffi::WGPUSamplerBindingType) -> Self {
        match value as u32 {
            0 => SamplerBindingType::BindingNotUsed,
            1 => SamplerBindingType::Undefined,
            2 => SamplerBindingType::Filtering,
            3 => SamplerBindingType::NonFiltering,
            4 => SamplerBindingType::Comparison,
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
    VkSemaphoreOpaqueFD = 1,
    SyncFD = 2,
    VkSemaphoreZirconHandle = 3,
    DXGISharedHandle = 4,
    MTLSharedEvent = 5,
    EGLSync = 6,
}

impl From<ffi::WGPUSharedFenceType> for SharedFenceType {
    fn from(value: ffi::WGPUSharedFenceType) -> Self {
        match value as u32 {
            1 => SharedFenceType::VkSemaphoreOpaqueFD,
            2 => SharedFenceType::SyncFD,
            3 => SharedFenceType::VkSemaphoreZirconHandle,
            4 => SharedFenceType::DXGISharedHandle,
            5 => SharedFenceType::MTLSharedEvent,
            6 => SharedFenceType::EGLSync,
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
    Success = 1,
    Error = 2,
}

impl From<ffi::WGPUStatus> for Status {
    fn from(value: ffi::WGPUStatus) -> Self {
        match value as u32 {
            1 => Status::Success,
            2 => Status::Error,
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
    Undefined = 0,
    Keep = 1,
    Zero = 2,
    Replace = 3,
    Invert = 4,
    IncrementClamp = 5,
    DecrementClamp = 6,
    IncrementWrap = 7,
    DecrementWrap = 8,
}

impl From<ffi::WGPUStencilOperation> for StencilOperation {
    fn from(value: ffi::WGPUStencilOperation) -> Self {
        match value as u32 {
            0 => StencilOperation::Undefined,
            1 => StencilOperation::Keep,
            2 => StencilOperation::Zero,
            3 => StencilOperation::Replace,
            4 => StencilOperation::Invert,
            5 => StencilOperation::IncrementClamp,
            6 => StencilOperation::DecrementClamp,
            7 => StencilOperation::IncrementWrap,
            8 => StencilOperation::DecrementWrap,
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
    BindingNotUsed = 0,
    Undefined = 1,
    WriteOnly = 2,
    ReadOnly = 3,
    ReadWrite = 4,
}

impl From<ffi::WGPUStorageTextureAccess> for StorageTextureAccess {
    fn from(value: ffi::WGPUStorageTextureAccess) -> Self {
        match value as u32 {
            0 => StorageTextureAccess::BindingNotUsed,
            1 => StorageTextureAccess::Undefined,
            2 => StorageTextureAccess::WriteOnly,
            3 => StorageTextureAccess::ReadOnly,
            4 => StorageTextureAccess::ReadWrite,
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
    Undefined = 0,
    Store = 1,
    Discard = 2,
}

impl From<ffi::WGPUStoreOp> for StoreOp {
    fn from(value: ffi::WGPUStoreOp) -> Self {
        match value as u32 {
            0 => StoreOp::Undefined,
            1 => StoreOp::Store,
            2 => StoreOp::Discard,
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
    F32 = 1,
    F16 = 2,
    U32 = 3,
    I32 = 4,
    U8 = 5,
    I8 = 6,
}

impl From<ffi::WGPUSubgroupMatrixComponentType> for SubgroupMatrixComponentType {
    fn from(value: ffi::WGPUSubgroupMatrixComponentType) -> Self {
        match value as u32 {
            1 => SubgroupMatrixComponentType::F32,
            2 => SubgroupMatrixComponentType::F16,
            3 => SubgroupMatrixComponentType::U32,
            4 => SubgroupMatrixComponentType::I32,
            5 => SubgroupMatrixComponentType::U8,
            6 => SubgroupMatrixComponentType::I8,
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
    SuccessOptimal = 1,
    SuccessSuboptimal = 2,
    Timeout = 3,
    Outdated = 4,
    Lost = 5,
    Error = 6,
}

impl From<ffi::WGPUSurfaceGetCurrentTextureStatus> for SurfaceGetCurrentTextureStatus {
    fn from(value: ffi::WGPUSurfaceGetCurrentTextureStatus) -> Self {
        match value as u32 {
            1 => SurfaceGetCurrentTextureStatus::SuccessOptimal,
            2 => SurfaceGetCurrentTextureStatus::SuccessSuboptimal,
            3 => SurfaceGetCurrentTextureStatus::Timeout,
            4 => SurfaceGetCurrentTextureStatus::Outdated,
            5 => SurfaceGetCurrentTextureStatus::Lost,
            6 => SurfaceGetCurrentTextureStatus::Error,
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
    Undefined = 0,
    ReadOnly = 1,
    ReadWrite = 2,
}

impl From<ffi::WGPUTexelBufferAccess> for TexelBufferAccess {
    fn from(value: ffi::WGPUTexelBufferAccess) -> Self {
        match value as u32 {
            0 => TexelBufferAccess::Undefined,
            1 => TexelBufferAccess::ReadOnly,
            2 => TexelBufferAccess::ReadWrite,
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
    Undefined = 0,
    All = 1,
    StencilOnly = 2,
    DepthOnly = 3,
    Plane0Only = 327680,
    Plane1Only = 327681,
    Plane2Only = 327682,
}

impl From<ffi::WGPUTextureAspect> for TextureAspect {
    fn from(value: ffi::WGPUTextureAspect) -> Self {
        match value as u32 {
            0 => TextureAspect::Undefined,
            1 => TextureAspect::All,
            2 => TextureAspect::StencilOnly,
            3 => TextureAspect::DepthOnly,
            327680 => TextureAspect::Plane0Only,
            327681 => TextureAspect::Plane1Only,
            327682 => TextureAspect::Plane2Only,
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
    Undefined = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
}

impl From<ffi::WGPUTextureDimension> for TextureDimension {
    fn from(value: ffi::WGPUTextureDimension) -> Self {
        match value as u32 {
            0 => TextureDimension::Undefined,
            1 => TextureDimension::D1,
            2 => TextureDimension::D2,
            3 => TextureDimension::D3,
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
    Undefined = 0,
    R8Unorm = 1,
    R8Snorm = 2,
    R8Uint = 3,
    R8Sint = 4,
    R16Unorm = 5,
    R16Snorm = 6,
    R16Uint = 7,
    R16Sint = 8,
    R16Float = 9,
    Rg8Unorm = 10,
    Rg8Snorm = 11,
    Rg8Uint = 12,
    Rg8Sint = 13,
    R32Float = 14,
    R32Uint = 15,
    R32Sint = 16,
    Rg16Unorm = 17,
    Rg16Snorm = 18,
    Rg16Uint = 19,
    Rg16Sint = 20,
    Rg16Float = 21,
    Rgba8Unorm = 22,
    Rgba8UnormSrgb = 23,
    Rgba8Snorm = 24,
    Rgba8Uint = 25,
    Rgba8Sint = 26,
    Bgra8Unorm = 27,
    Bgra8UnormSrgb = 28,
    Rgb10A2Uint = 29,
    Rgb10A2Unorm = 30,
    Rg11B10Ufloat = 31,
    Rgb9E5Ufloat = 32,
    Rg32Float = 33,
    Rg32Uint = 34,
    Rg32Sint = 35,
    Rgba16Unorm = 36,
    Rgba16Snorm = 37,
    Rgba16Uint = 38,
    Rgba16Sint = 39,
    Rgba16Float = 40,
    Rgba32Float = 41,
    Rgba32Uint = 42,
    Rgba32Sint = 43,
    Stencil8 = 44,
    Depth16Unorm = 45,
    Depth24Plus = 46,
    Depth24PlusStencil8 = 47,
    Depth32Float = 48,
    Depth32FloatStencil8 = 49,
    Bc1RgbaUnorm = 50,
    Bc1RgbaUnormSrgb = 51,
    Bc2RgbaUnorm = 52,
    Bc2RgbaUnormSrgb = 53,
    Bc3RgbaUnorm = 54,
    Bc3RgbaUnormSrgb = 55,
    Bc4RUnorm = 56,
    Bc4RSnorm = 57,
    Bc5RgUnorm = 58,
    Bc5RgSnorm = 59,
    Bc6HRgbUfloat = 60,
    Bc6HRgbFloat = 61,
    Bc7RgbaUnorm = 62,
    Bc7RgbaUnormSrgb = 63,
    Etc2Rgb8Unorm = 64,
    Etc2Rgb8UnormSrgb = 65,
    Etc2Rgb8A1Unorm = 66,
    Etc2Rgb8A1UnormSrgb = 67,
    Etc2Rgba8Unorm = 68,
    Etc2Rgba8UnormSrgb = 69,
    EacR11Unorm = 70,
    EacR11Snorm = 71,
    EacRg11Unorm = 72,
    EacRg11Snorm = 73,
    Astc4X4Unorm = 74,
    Astc4X4UnormSrgb = 75,
    Astc5X4Unorm = 76,
    Astc5X4UnormSrgb = 77,
    Astc5X5Unorm = 78,
    Astc5X5UnormSrgb = 79,
    Astc6X5Unorm = 80,
    Astc6X5UnormSrgb = 81,
    Astc6X6Unorm = 82,
    Astc6X6UnormSrgb = 83,
    Astc8X5Unorm = 84,
    Astc8X5UnormSrgb = 85,
    Astc8X6Unorm = 86,
    Astc8X6UnormSrgb = 87,
    Astc8X8Unorm = 88,
    Astc8X8UnormSrgb = 89,
    Astc10X5Unorm = 90,
    Astc10X5UnormSrgb = 91,
    Astc10X6Unorm = 92,
    Astc10X6UnormSrgb = 93,
    Astc10X8Unorm = 94,
    Astc10X8UnormSrgb = 95,
    Astc10X10Unorm = 96,
    Astc10X10UnormSrgb = 97,
    Astc12X10Unorm = 98,
    Astc12X10UnormSrgb = 99,
    Astc12X12Unorm = 100,
    Astc12X12UnormSrgb = 101,
    R8Bg8Biplanar420Unorm = 327686,
    R10X6Bg10X6Biplanar420Unorm = 327687,
    R8Bg8A8Triplanar420Unorm = 327688,
    R8Bg8Biplanar422Unorm = 327689,
    R8Bg8Biplanar444Unorm = 327690,
    R10X6Bg10X6Biplanar422Unorm = 327691,
    R10X6Bg10X6Biplanar444Unorm = 327692,
    External = 327693,
}

impl From<ffi::WGPUTextureFormat> for TextureFormat {
    fn from(value: ffi::WGPUTextureFormat) -> Self {
        match value as u32 {
            0 => TextureFormat::Undefined,
            1 => TextureFormat::R8Unorm,
            2 => TextureFormat::R8Snorm,
            3 => TextureFormat::R8Uint,
            4 => TextureFormat::R8Sint,
            5 => TextureFormat::R16Unorm,
            6 => TextureFormat::R16Snorm,
            7 => TextureFormat::R16Uint,
            8 => TextureFormat::R16Sint,
            9 => TextureFormat::R16Float,
            10 => TextureFormat::Rg8Unorm,
            11 => TextureFormat::Rg8Snorm,
            12 => TextureFormat::Rg8Uint,
            13 => TextureFormat::Rg8Sint,
            14 => TextureFormat::R32Float,
            15 => TextureFormat::R32Uint,
            16 => TextureFormat::R32Sint,
            17 => TextureFormat::Rg16Unorm,
            18 => TextureFormat::Rg16Snorm,
            19 => TextureFormat::Rg16Uint,
            20 => TextureFormat::Rg16Sint,
            21 => TextureFormat::Rg16Float,
            22 => TextureFormat::Rgba8Unorm,
            23 => TextureFormat::Rgba8UnormSrgb,
            24 => TextureFormat::Rgba8Snorm,
            25 => TextureFormat::Rgba8Uint,
            26 => TextureFormat::Rgba8Sint,
            27 => TextureFormat::Bgra8Unorm,
            28 => TextureFormat::Bgra8UnormSrgb,
            29 => TextureFormat::Rgb10A2Uint,
            30 => TextureFormat::Rgb10A2Unorm,
            31 => TextureFormat::Rg11B10Ufloat,
            32 => TextureFormat::Rgb9E5Ufloat,
            33 => TextureFormat::Rg32Float,
            34 => TextureFormat::Rg32Uint,
            35 => TextureFormat::Rg32Sint,
            36 => TextureFormat::Rgba16Unorm,
            37 => TextureFormat::Rgba16Snorm,
            38 => TextureFormat::Rgba16Uint,
            39 => TextureFormat::Rgba16Sint,
            40 => TextureFormat::Rgba16Float,
            41 => TextureFormat::Rgba32Float,
            42 => TextureFormat::Rgba32Uint,
            43 => TextureFormat::Rgba32Sint,
            44 => TextureFormat::Stencil8,
            45 => TextureFormat::Depth16Unorm,
            46 => TextureFormat::Depth24Plus,
            47 => TextureFormat::Depth24PlusStencil8,
            48 => TextureFormat::Depth32Float,
            49 => TextureFormat::Depth32FloatStencil8,
            50 => TextureFormat::Bc1RgbaUnorm,
            51 => TextureFormat::Bc1RgbaUnormSrgb,
            52 => TextureFormat::Bc2RgbaUnorm,
            53 => TextureFormat::Bc2RgbaUnormSrgb,
            54 => TextureFormat::Bc3RgbaUnorm,
            55 => TextureFormat::Bc3RgbaUnormSrgb,
            56 => TextureFormat::Bc4RUnorm,
            57 => TextureFormat::Bc4RSnorm,
            58 => TextureFormat::Bc5RgUnorm,
            59 => TextureFormat::Bc5RgSnorm,
            60 => TextureFormat::Bc6HRgbUfloat,
            61 => TextureFormat::Bc6HRgbFloat,
            62 => TextureFormat::Bc7RgbaUnorm,
            63 => TextureFormat::Bc7RgbaUnormSrgb,
            64 => TextureFormat::Etc2Rgb8Unorm,
            65 => TextureFormat::Etc2Rgb8UnormSrgb,
            66 => TextureFormat::Etc2Rgb8A1Unorm,
            67 => TextureFormat::Etc2Rgb8A1UnormSrgb,
            68 => TextureFormat::Etc2Rgba8Unorm,
            69 => TextureFormat::Etc2Rgba8UnormSrgb,
            70 => TextureFormat::EacR11Unorm,
            71 => TextureFormat::EacR11Snorm,
            72 => TextureFormat::EacRg11Unorm,
            73 => TextureFormat::EacRg11Snorm,
            74 => TextureFormat::Astc4X4Unorm,
            75 => TextureFormat::Astc4X4UnormSrgb,
            76 => TextureFormat::Astc5X4Unorm,
            77 => TextureFormat::Astc5X4UnormSrgb,
            78 => TextureFormat::Astc5X5Unorm,
            79 => TextureFormat::Astc5X5UnormSrgb,
            80 => TextureFormat::Astc6X5Unorm,
            81 => TextureFormat::Astc6X5UnormSrgb,
            82 => TextureFormat::Astc6X6Unorm,
            83 => TextureFormat::Astc6X6UnormSrgb,
            84 => TextureFormat::Astc8X5Unorm,
            85 => TextureFormat::Astc8X5UnormSrgb,
            86 => TextureFormat::Astc8X6Unorm,
            87 => TextureFormat::Astc8X6UnormSrgb,
            88 => TextureFormat::Astc8X8Unorm,
            89 => TextureFormat::Astc8X8UnormSrgb,
            90 => TextureFormat::Astc10X5Unorm,
            91 => TextureFormat::Astc10X5UnormSrgb,
            92 => TextureFormat::Astc10X6Unorm,
            93 => TextureFormat::Astc10X6UnormSrgb,
            94 => TextureFormat::Astc10X8Unorm,
            95 => TextureFormat::Astc10X8UnormSrgb,
            96 => TextureFormat::Astc10X10Unorm,
            97 => TextureFormat::Astc10X10UnormSrgb,
            98 => TextureFormat::Astc12X10Unorm,
            99 => TextureFormat::Astc12X10UnormSrgb,
            100 => TextureFormat::Astc12X12Unorm,
            101 => TextureFormat::Astc12X12UnormSrgb,
            327686 => TextureFormat::R8Bg8Biplanar420Unorm,
            327687 => TextureFormat::R10X6Bg10X6Biplanar420Unorm,
            327688 => TextureFormat::R8Bg8A8Triplanar420Unorm,
            327689 => TextureFormat::R8Bg8Biplanar422Unorm,
            327690 => TextureFormat::R8Bg8Biplanar444Unorm,
            327691 => TextureFormat::R10X6Bg10X6Biplanar422Unorm,
            327692 => TextureFormat::R10X6Bg10X6Biplanar444Unorm,
            327693 => TextureFormat::External,
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
    BindingNotUsed = 0,
    Undefined = 1,
    Float = 2,
    UnfilterableFloat = 3,
    Depth = 4,
    Sint = 5,
    Uint = 6,
}

impl From<ffi::WGPUTextureSampleType> for TextureSampleType {
    fn from(value: ffi::WGPUTextureSampleType) -> Self {
        match value as u32 {
            0 => TextureSampleType::BindingNotUsed,
            1 => TextureSampleType::Undefined,
            2 => TextureSampleType::Float,
            3 => TextureSampleType::UnfilterableFloat,
            4 => TextureSampleType::Depth,
            5 => TextureSampleType::Sint,
            6 => TextureSampleType::Uint,
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
    Undefined = 0,
    D1 = 1,
    D2 = 2,
    D2Array = 3,
    Cube = 4,
    CubeArray = 5,
    D3 = 6,
}

impl From<ffi::WGPUTextureViewDimension> for TextureViewDimension {
    fn from(value: ffi::WGPUTextureViewDimension) -> Self {
        match value as u32 {
            0 => TextureViewDimension::Undefined,
            1 => TextureViewDimension::D1,
            2 => TextureViewDimension::D2,
            3 => TextureViewDimension::D2Array,
            4 => TextureViewDimension::Cube,
            5 => TextureViewDimension::CubeArray,
            6 => TextureViewDimension::D3,
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
    Standard = 1,
    Extended = 2,
}

impl From<ffi::WGPUToneMappingMode> for ToneMappingMode {
    fn from(value: ffi::WGPUToneMappingMode) -> Self {
        match value as u32 {
            1 => ToneMappingMode::Standard,
            2 => ToneMappingMode::Extended,
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
    Uint8 = 1,
    Uint8X2 = 2,
    Uint8X4 = 3,
    Sint8 = 4,
    Sint8X2 = 5,
    Sint8X4 = 6,
    Unorm8 = 7,
    Unorm8X2 = 8,
    Unorm8X4 = 9,
    Snorm8 = 10,
    Snorm8X2 = 11,
    Snorm8X4 = 12,
    Uint16 = 13,
    Uint16X2 = 14,
    Uint16X4 = 15,
    Sint16 = 16,
    Sint16X2 = 17,
    Sint16X4 = 18,
    Unorm16 = 19,
    Unorm16X2 = 20,
    Unorm16X4 = 21,
    Snorm16 = 22,
    Snorm16X2 = 23,
    Snorm16X4 = 24,
    Float16 = 25,
    Float16X2 = 26,
    Float16X4 = 27,
    Float32 = 28,
    Float32X2 = 29,
    Float32X3 = 30,
    Float32X4 = 31,
    Uint32 = 32,
    Uint32X2 = 33,
    Uint32X3 = 34,
    Uint32X4 = 35,
    Sint32 = 36,
    Sint32X2 = 37,
    Sint32X3 = 38,
    Sint32X4 = 39,
    Unorm1010102 = 40,
    Unorm8X4Bgra = 41,
}

impl From<ffi::WGPUVertexFormat> for VertexFormat {
    fn from(value: ffi::WGPUVertexFormat) -> Self {
        match value as u32 {
            1 => VertexFormat::Uint8,
            2 => VertexFormat::Uint8X2,
            3 => VertexFormat::Uint8X4,
            4 => VertexFormat::Sint8,
            5 => VertexFormat::Sint8X2,
            6 => VertexFormat::Sint8X4,
            7 => VertexFormat::Unorm8,
            8 => VertexFormat::Unorm8X2,
            9 => VertexFormat::Unorm8X4,
            10 => VertexFormat::Snorm8,
            11 => VertexFormat::Snorm8X2,
            12 => VertexFormat::Snorm8X4,
            13 => VertexFormat::Uint16,
            14 => VertexFormat::Uint16X2,
            15 => VertexFormat::Uint16X4,
            16 => VertexFormat::Sint16,
            17 => VertexFormat::Sint16X2,
            18 => VertexFormat::Sint16X4,
            19 => VertexFormat::Unorm16,
            20 => VertexFormat::Unorm16X2,
            21 => VertexFormat::Unorm16X4,
            22 => VertexFormat::Snorm16,
            23 => VertexFormat::Snorm16X2,
            24 => VertexFormat::Snorm16X4,
            25 => VertexFormat::Float16,
            26 => VertexFormat::Float16X2,
            27 => VertexFormat::Float16X4,
            28 => VertexFormat::Float32,
            29 => VertexFormat::Float32X2,
            30 => VertexFormat::Float32X3,
            31 => VertexFormat::Float32X4,
            32 => VertexFormat::Uint32,
            33 => VertexFormat::Uint32X2,
            34 => VertexFormat::Uint32X3,
            35 => VertexFormat::Uint32X4,
            36 => VertexFormat::Sint32,
            37 => VertexFormat::Sint32X2,
            38 => VertexFormat::Sint32X3,
            39 => VertexFormat::Sint32X4,
            40 => VertexFormat::Unorm1010102,
            41 => VertexFormat::Unorm8X4Bgra,
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
    Undefined = 0,
    Vertex = 1,
    Instance = 2,
}

impl From<ffi::WGPUVertexStepMode> for VertexStepMode {
    fn from(value: ffi::WGPUVertexStepMode) -> Self {
        match value as u32 {
            0 => VertexStepMode::Undefined,
            1 => VertexStepMode::Vertex,
            2 => VertexStepMode::Instance,
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
    Success = 1,
    TimedOut = 2,
    Error = 3,
}

impl From<ffi::WGPUWaitStatus> for WaitStatus {
    fn from(value: ffi::WGPUWaitStatus) -> Self {
        match value as u32 {
            1 => WaitStatus::Success,
            2 => WaitStatus::TimedOut,
            3 => WaitStatus::Error,
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BufferUsage: u64 {
        const NONE = 0;
        const MAP_READ = 1;
        const MAP_WRITE = 2;
        const COPY_SRC = 4;
        const COPY_DST = 8;
        const INDEX = 16;
        const VERTEX = 32;
        const UNIFORM = 64;
        const STORAGE = 128;
        const INDIRECT = 256;
        const QUERY_RESOLVE = 512;
        const TEXEL_BUFFER = 328704;
    }
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ColorWriteMask: u64 {
        const NONE = 0;
        const RED = 1;
        const GREEN = 2;
        const BLUE = 4;
        const ALPHA = 8;
        const ALL = 15;
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HeapProperty: u64 {
        const NONE = 0;
        const DEVICE_LOCAL = 1;
        const HOST_VISIBLE = 2;
        const HOST_COHERENT = 4;
        const HOST_UNCACHED = 8;
        const HOST_CACHED = 16;
    }
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MapMode: u64 {
        const NONE = 0;
        const READ = 1;
        const WRITE = 2;
    }
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ShaderStage: u64 {
        const NONE = 0;
        const VERTEX = 1;
        const FRAGMENT = 2;
        const COMPUTE = 4;
    }
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TextureUsage: u64 {
        const NONE = 0;
        const COPY_SRC = 1;
        const COPY_DST = 2;
        const TEXTURE_BINDING = 4;
        const STORAGE_BINDING = 8;
        const RENDER_ATTACHMENT = 16;
        const TRANSIENT_ATTACHMENT = 327712;
        const STORAGE_ATTACHMENT = 327744;
    }
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

