mod enums {
    #![allow(dead_code, unused_imports)]
    use crate::ffi;
    use bitflags::bitflags;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum WGSLLanguageFeatureName {
        ReadonlyAndReadwriteStorageTextures,
        Packed4X8IntegerDotProduct,
        UnrestrictedPointerParameters,
        PointerCompositeAccess,
        UniformBufferStandardLayout,
        SubgroupId,
        TextureAndSamplerLet,
        ChromiumTestingUnimplemented,
        ChromiumTestingUnsafeExperimental,
        ChromiumTestingExperimental,
        ChromiumTestingShippedWithKillswitch,
        ChromiumTestingShipped,
        SizedBindingArray,
        TexelBuffers,
        ChromiumPrint,
        FragmentDepth,
        ImmediateAddressSpace,
        SubgroupUniformity,
        BufferView,
        FilteringParameters,
        SwizzleAssignment,
        LinearIndexing,
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
            match value {
                WGSLLanguageFeatureName::ReadonlyAndReadwriteStorageTextures => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ReadonlyAndReadwriteStorageTextures
                }
                WGSLLanguageFeatureName::Packed4X8IntegerDotProduct => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_Packed4x8IntegerDotProduct
                }
                WGSLLanguageFeatureName::UnrestrictedPointerParameters => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UnrestrictedPointerParameters
                }
                WGSLLanguageFeatureName::PointerCompositeAccess => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_PointerCompositeAccess
                }
                WGSLLanguageFeatureName::UniformBufferStandardLayout => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UniformBufferStandardLayout
                }
                WGSLLanguageFeatureName::SubgroupId => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SubgroupId
                }
                WGSLLanguageFeatureName::TextureAndSamplerLet => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_TextureAndSamplerLet
                }
                WGSLLanguageFeatureName::ChromiumTestingUnimplemented => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnimplemented
                }
                WGSLLanguageFeatureName::ChromiumTestingUnsafeExperimental => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnsafeExperimental
                }
                WGSLLanguageFeatureName::ChromiumTestingExperimental => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingExperimental
                }
                WGSLLanguageFeatureName::ChromiumTestingShippedWithKillswitch => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShippedWithKillswitch
                }
                WGSLLanguageFeatureName::ChromiumTestingShipped => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShipped
                }
                WGSLLanguageFeatureName::SizedBindingArray => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SizedBindingArray
                }
                WGSLLanguageFeatureName::TexelBuffers => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_TexelBuffers
                }
                WGSLLanguageFeatureName::ChromiumPrint => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumPrint
                }
                WGSLLanguageFeatureName::FragmentDepth => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_FragmentDepth
                }
                WGSLLanguageFeatureName::ImmediateAddressSpace => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ImmediateAddressSpace
                }
                WGSLLanguageFeatureName::SubgroupUniformity => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SubgroupUniformity
                }
                WGSLLanguageFeatureName::BufferView => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_BufferView
                }
                WGSLLanguageFeatureName::FilteringParameters => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_FilteringParameters
                }
                WGSLLanguageFeatureName::SwizzleAssignment => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SwizzleAssignment
                }
                WGSLLanguageFeatureName::LinearIndexing => {
                    ffi::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_LinearIndexing
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AdapterType {
        DiscreteGpu,
        IntegratedGpu,
        Cpu,
        Unknown,
    }
    impl From<ffi::WGPUAdapterType> for AdapterType {
        fn from(value: ffi::WGPUAdapterType) -> Self {
            match value {
                ffi::WGPUAdapterType_WGPUAdapterType_DiscreteGPU => {
                    AdapterType::DiscreteGpu
                }
                ffi::WGPUAdapterType_WGPUAdapterType_IntegratedGPU => {
                    AdapterType::IntegratedGpu
                }
                ffi::WGPUAdapterType_WGPUAdapterType_CPU => AdapterType::Cpu,
                ffi::WGPUAdapterType_WGPUAdapterType_Unknown => AdapterType::Unknown,
                _ => AdapterType::DiscreteGpu,
            }
        }
    }
    impl From<AdapterType> for ffi::WGPUAdapterType {
        fn from(value: AdapterType) -> Self {
            match value {
                AdapterType::DiscreteGpu => {
                    ffi::WGPUAdapterType_WGPUAdapterType_DiscreteGPU
                }
                AdapterType::IntegratedGpu => {
                    ffi::WGPUAdapterType_WGPUAdapterType_IntegratedGPU
                }
                AdapterType::Cpu => ffi::WGPUAdapterType_WGPUAdapterType_CPU,
                AdapterType::Unknown => ffi::WGPUAdapterType_WGPUAdapterType_Unknown,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AddressMode {
        Undefined,
        ClampToEdge,
        Repeat,
        MirrorRepeat,
    }
    impl From<ffi::WGPUAddressMode> for AddressMode {
        fn from(value: ffi::WGPUAddressMode) -> Self {
            match value {
                ffi::WGPUAddressMode_WGPUAddressMode_Undefined => AddressMode::Undefined,
                ffi::WGPUAddressMode_WGPUAddressMode_ClampToEdge => {
                    AddressMode::ClampToEdge
                }
                ffi::WGPUAddressMode_WGPUAddressMode_Repeat => AddressMode::Repeat,
                ffi::WGPUAddressMode_WGPUAddressMode_MirrorRepeat => {
                    AddressMode::MirrorRepeat
                }
                _ => AddressMode::Undefined,
            }
        }
    }
    impl From<AddressMode> for ffi::WGPUAddressMode {
        fn from(value: AddressMode) -> Self {
            match value {
                AddressMode::Undefined => ffi::WGPUAddressMode_WGPUAddressMode_Undefined,
                AddressMode::ClampToEdge => {
                    ffi::WGPUAddressMode_WGPUAddressMode_ClampToEdge
                }
                AddressMode::Repeat => ffi::WGPUAddressMode_WGPUAddressMode_Repeat,
                AddressMode::MirrorRepeat => {
                    ffi::WGPUAddressMode_WGPUAddressMode_MirrorRepeat
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AlphaMode {
        Opaque,
        Premultiplied,
        Unpremultiplied,
    }
    impl From<ffi::WGPUAlphaMode> for AlphaMode {
        fn from(value: ffi::WGPUAlphaMode) -> Self {
            match value {
                ffi::WGPUAlphaMode_WGPUAlphaMode_Opaque => AlphaMode::Opaque,
                ffi::WGPUAlphaMode_WGPUAlphaMode_Premultiplied => {
                    AlphaMode::Premultiplied
                }
                ffi::WGPUAlphaMode_WGPUAlphaMode_Unpremultiplied => {
                    AlphaMode::Unpremultiplied
                }
                _ => AlphaMode::Opaque,
            }
        }
    }
    impl From<AlphaMode> for ffi::WGPUAlphaMode {
        fn from(value: AlphaMode) -> Self {
            match value {
                AlphaMode::Opaque => ffi::WGPUAlphaMode_WGPUAlphaMode_Opaque,
                AlphaMode::Premultiplied => {
                    ffi::WGPUAlphaMode_WGPUAlphaMode_Premultiplied
                }
                AlphaMode::Unpremultiplied => {
                    ffi::WGPUAlphaMode_WGPUAlphaMode_Unpremultiplied
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BackendType {
        Undefined,
        Null,
        WebGPU,
        D3D11,
        D3D12,
        Metal,
        Vulkan,
        OpenGL,
        OpenGLes,
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
            match value {
                BackendType::Undefined => ffi::WGPUBackendType_WGPUBackendType_Undefined,
                BackendType::Null => ffi::WGPUBackendType_WGPUBackendType_Null,
                BackendType::WebGPU => ffi::WGPUBackendType_WGPUBackendType_WebGPU,
                BackendType::D3D11 => ffi::WGPUBackendType_WGPUBackendType_D3D11,
                BackendType::D3D12 => ffi::WGPUBackendType_WGPUBackendType_D3D12,
                BackendType::Metal => ffi::WGPUBackendType_WGPUBackendType_Metal,
                BackendType::Vulkan => ffi::WGPUBackendType_WGPUBackendType_Vulkan,
                BackendType::OpenGL => ffi::WGPUBackendType_WGPUBackendType_OpenGL,
                BackendType::OpenGLes => ffi::WGPUBackendType_WGPUBackendType_OpenGLES,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BlendFactor {
        Undefined,
        Zero,
        One,
        Src,
        OneMinusSrc,
        SrcAlpha,
        OneMinusSrcAlpha,
        Dst,
        OneMinusDst,
        DstAlpha,
        OneMinusDstAlpha,
        SrcAlphaSaturated,
        Constant,
        OneMinusConstant,
        Src1,
        OneMinusSrc1,
        Src1Alpha,
        OneMinusSrc1Alpha,
    }
    impl From<ffi::WGPUBlendFactor> for BlendFactor {
        fn from(value: ffi::WGPUBlendFactor) -> Self {
            match value {
                ffi::WGPUBlendFactor_WGPUBlendFactor_Undefined => BlendFactor::Undefined,
                ffi::WGPUBlendFactor_WGPUBlendFactor_Zero => BlendFactor::Zero,
                ffi::WGPUBlendFactor_WGPUBlendFactor_One => BlendFactor::One,
                ffi::WGPUBlendFactor_WGPUBlendFactor_Src => BlendFactor::Src,
                ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc => {
                    BlendFactor::OneMinusSrc
                }
                ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlpha => BlendFactor::SrcAlpha,
                ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrcAlpha => {
                    BlendFactor::OneMinusSrcAlpha
                }
                ffi::WGPUBlendFactor_WGPUBlendFactor_Dst => BlendFactor::Dst,
                ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDst => {
                    BlendFactor::OneMinusDst
                }
                ffi::WGPUBlendFactor_WGPUBlendFactor_DstAlpha => BlendFactor::DstAlpha,
                ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDstAlpha => {
                    BlendFactor::OneMinusDstAlpha
                }
                ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlphaSaturated => {
                    BlendFactor::SrcAlphaSaturated
                }
                ffi::WGPUBlendFactor_WGPUBlendFactor_Constant => BlendFactor::Constant,
                ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusConstant => {
                    BlendFactor::OneMinusConstant
                }
                ffi::WGPUBlendFactor_WGPUBlendFactor_Src1 => BlendFactor::Src1,
                ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1 => {
                    BlendFactor::OneMinusSrc1
                }
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
            match value {
                BlendFactor::Undefined => ffi::WGPUBlendFactor_WGPUBlendFactor_Undefined,
                BlendFactor::Zero => ffi::WGPUBlendFactor_WGPUBlendFactor_Zero,
                BlendFactor::One => ffi::WGPUBlendFactor_WGPUBlendFactor_One,
                BlendFactor::Src => ffi::WGPUBlendFactor_WGPUBlendFactor_Src,
                BlendFactor::OneMinusSrc => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc
                }
                BlendFactor::SrcAlpha => ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlpha,
                BlendFactor::OneMinusSrcAlpha => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrcAlpha
                }
                BlendFactor::Dst => ffi::WGPUBlendFactor_WGPUBlendFactor_Dst,
                BlendFactor::OneMinusDst => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDst
                }
                BlendFactor::DstAlpha => ffi::WGPUBlendFactor_WGPUBlendFactor_DstAlpha,
                BlendFactor::OneMinusDstAlpha => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusDstAlpha
                }
                BlendFactor::SrcAlphaSaturated => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_SrcAlphaSaturated
                }
                BlendFactor::Constant => ffi::WGPUBlendFactor_WGPUBlendFactor_Constant,
                BlendFactor::OneMinusConstant => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusConstant
                }
                BlendFactor::Src1 => ffi::WGPUBlendFactor_WGPUBlendFactor_Src1,
                BlendFactor::OneMinusSrc1 => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1
                }
                BlendFactor::Src1Alpha => ffi::WGPUBlendFactor_WGPUBlendFactor_Src1Alpha,
                BlendFactor::OneMinusSrc1Alpha => {
                    ffi::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1Alpha
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BlendOperation {
        Undefined,
        Add,
        Subtract,
        ReverseSubtract,
        Min,
        Max,
    }
    impl From<ffi::WGPUBlendOperation> for BlendOperation {
        fn from(value: ffi::WGPUBlendOperation) -> Self {
            match value {
                ffi::WGPUBlendOperation_WGPUBlendOperation_Undefined => {
                    BlendOperation::Undefined
                }
                ffi::WGPUBlendOperation_WGPUBlendOperation_Add => BlendOperation::Add,
                ffi::WGPUBlendOperation_WGPUBlendOperation_Subtract => {
                    BlendOperation::Subtract
                }
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
            match value {
                BlendOperation::Undefined => {
                    ffi::WGPUBlendOperation_WGPUBlendOperation_Undefined
                }
                BlendOperation::Add => ffi::WGPUBlendOperation_WGPUBlendOperation_Add,
                BlendOperation::Subtract => {
                    ffi::WGPUBlendOperation_WGPUBlendOperation_Subtract
                }
                BlendOperation::ReverseSubtract => {
                    ffi::WGPUBlendOperation_WGPUBlendOperation_ReverseSubtract
                }
                BlendOperation::Min => ffi::WGPUBlendOperation_WGPUBlendOperation_Min,
                BlendOperation::Max => ffi::WGPUBlendOperation_WGPUBlendOperation_Max,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BufferBindingType {
        BindingNotUsed,
        Undefined,
        Uniform,
        Storage,
        ReadOnlyStorage,
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
                ffi::WGPUBufferBindingType_WGPUBufferBindingType_Uniform => {
                    BufferBindingType::Uniform
                }
                ffi::WGPUBufferBindingType_WGPUBufferBindingType_Storage => {
                    BufferBindingType::Storage
                }
                ffi::WGPUBufferBindingType_WGPUBufferBindingType_ReadOnlyStorage => {
                    BufferBindingType::ReadOnlyStorage
                }
                _ => BufferBindingType::BindingNotUsed,
            }
        }
    }
    impl From<BufferBindingType> for ffi::WGPUBufferBindingType {
        fn from(value: BufferBindingType) -> Self {
            match value {
                BufferBindingType::BindingNotUsed => {
                    ffi::WGPUBufferBindingType_WGPUBufferBindingType_BindingNotUsed
                }
                BufferBindingType::Undefined => {
                    ffi::WGPUBufferBindingType_WGPUBufferBindingType_Undefined
                }
                BufferBindingType::Uniform => {
                    ffi::WGPUBufferBindingType_WGPUBufferBindingType_Uniform
                }
                BufferBindingType::Storage => {
                    ffi::WGPUBufferBindingType_WGPUBufferBindingType_Storage
                }
                BufferBindingType::ReadOnlyStorage => {
                    ffi::WGPUBufferBindingType_WGPUBufferBindingType_ReadOnlyStorage
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BufferMapState {
        Unmapped,
        Pending,
        Mapped,
    }
    impl From<ffi::WGPUBufferMapState> for BufferMapState {
        fn from(value: ffi::WGPUBufferMapState) -> Self {
            match value {
                ffi::WGPUBufferMapState_WGPUBufferMapState_Unmapped => {
                    BufferMapState::Unmapped
                }
                ffi::WGPUBufferMapState_WGPUBufferMapState_Pending => {
                    BufferMapState::Pending
                }
                ffi::WGPUBufferMapState_WGPUBufferMapState_Mapped => {
                    BufferMapState::Mapped
                }
                _ => BufferMapState::Unmapped,
            }
        }
    }
    impl From<BufferMapState> for ffi::WGPUBufferMapState {
        fn from(value: BufferMapState) -> Self {
            match value {
                BufferMapState::Unmapped => {
                    ffi::WGPUBufferMapState_WGPUBufferMapState_Unmapped
                }
                BufferMapState::Pending => {
                    ffi::WGPUBufferMapState_WGPUBufferMapState_Pending
                }
                BufferMapState::Mapped => {
                    ffi::WGPUBufferMapState_WGPUBufferMapState_Mapped
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CallbackMode {
        WaitAnyOnly,
        AllowProcessEvents,
        AllowSpontaneous,
    }
    impl From<ffi::WGPUCallbackMode> for CallbackMode {
        fn from(value: ffi::WGPUCallbackMode) -> Self {
            match value {
                ffi::WGPUCallbackMode_WGPUCallbackMode_WaitAnyOnly => {
                    CallbackMode::WaitAnyOnly
                }
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
            match value {
                CallbackMode::WaitAnyOnly => {
                    ffi::WGPUCallbackMode_WGPUCallbackMode_WaitAnyOnly
                }
                CallbackMode::AllowProcessEvents => {
                    ffi::WGPUCallbackMode_WGPUCallbackMode_AllowProcessEvents
                }
                CallbackMode::AllowSpontaneous => {
                    ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CompareFunction {
        Undefined,
        Never,
        Less,
        Equal,
        LessEqual,
        Greater,
        NotEqual,
        GreaterEqual,
        Always,
    }
    impl From<ffi::WGPUCompareFunction> for CompareFunction {
        fn from(value: ffi::WGPUCompareFunction) -> Self {
            match value {
                ffi::WGPUCompareFunction_WGPUCompareFunction_Undefined => {
                    CompareFunction::Undefined
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_Never => {
                    CompareFunction::Never
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_Less => {
                    CompareFunction::Less
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_Equal => {
                    CompareFunction::Equal
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_LessEqual => {
                    CompareFunction::LessEqual
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_Greater => {
                    CompareFunction::Greater
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_NotEqual => {
                    CompareFunction::NotEqual
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_GreaterEqual => {
                    CompareFunction::GreaterEqual
                }
                ffi::WGPUCompareFunction_WGPUCompareFunction_Always => {
                    CompareFunction::Always
                }
                _ => CompareFunction::Undefined,
            }
        }
    }
    impl From<CompareFunction> for ffi::WGPUCompareFunction {
        fn from(value: CompareFunction) -> Self {
            match value {
                CompareFunction::Undefined => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_Undefined
                }
                CompareFunction::Never => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_Never
                }
                CompareFunction::Less => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_Less
                }
                CompareFunction::Equal => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_Equal
                }
                CompareFunction::LessEqual => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_LessEqual
                }
                CompareFunction::Greater => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_Greater
                }
                CompareFunction::NotEqual => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_NotEqual
                }
                CompareFunction::GreaterEqual => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_GreaterEqual
                }
                CompareFunction::Always => {
                    ffi::WGPUCompareFunction_WGPUCompareFunction_Always
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CompilationInfoRequestStatus {
        Success,
        CallbackCancelled,
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
            match value {
                CompilationInfoRequestStatus::Success => {
                    ffi::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_Success
                }
                CompilationInfoRequestStatus::CallbackCancelled => {
                    ffi::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_CallbackCancelled
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CompilationMessageType {
        Error,
        Warning,
        Info,
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
            match value {
                CompilationMessageType::Error => {
                    ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Error
                }
                CompilationMessageType::Warning => {
                    ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Warning
                }
                CompilationMessageType::Info => {
                    ffi::WGPUCompilationMessageType_WGPUCompilationMessageType_Info
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ComponentSwizzle {
        Undefined,
        Zero,
        One,
        R,
        G,
        B,
        A,
    }
    impl From<ffi::WGPUComponentSwizzle> for ComponentSwizzle {
        fn from(value: ffi::WGPUComponentSwizzle) -> Self {
            match value {
                ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Undefined => {
                    ComponentSwizzle::Undefined
                }
                ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Zero => {
                    ComponentSwizzle::Zero
                }
                ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_One => {
                    ComponentSwizzle::One
                }
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
            match value {
                ComponentSwizzle::Undefined => {
                    ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Undefined
                }
                ComponentSwizzle::Zero => {
                    ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_Zero
                }
                ComponentSwizzle::One => {
                    ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_One
                }
                ComponentSwizzle::R => ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_R,
                ComponentSwizzle::G => ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_G,
                ComponentSwizzle::B => ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_B,
                ComponentSwizzle::A => ffi::WGPUComponentSwizzle_WGPUComponentSwizzle_A,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CompositeAlphaMode {
        Auto,
        Opaque,
        Premultiplied,
        Unpremultiplied,
        Inherit,
    }
    impl From<ffi::WGPUCompositeAlphaMode> for CompositeAlphaMode {
        fn from(value: ffi::WGPUCompositeAlphaMode) -> Self {
            match value {
                ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Auto => {
                    CompositeAlphaMode::Auto
                }
                ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Opaque => {
                    CompositeAlphaMode::Opaque
                }
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
            match value {
                CompositeAlphaMode::Auto => {
                    ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Auto
                }
                CompositeAlphaMode::Opaque => {
                    ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Opaque
                }
                CompositeAlphaMode::Premultiplied => {
                    ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Premultiplied
                }
                CompositeAlphaMode::Unpremultiplied => {
                    ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Unpremultiplied
                }
                CompositeAlphaMode::Inherit => {
                    ffi::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Inherit
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CreatePipelineAsyncStatus {
        Success,
        CallbackCancelled,
        ValidationError,
        InternalError,
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
            match value {
                CreatePipelineAsyncStatus::Success => {
                    ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_Success
                }
                CreatePipelineAsyncStatus::CallbackCancelled => {
                    ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_CallbackCancelled
                }
                CreatePipelineAsyncStatus::ValidationError => {
                    ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_ValidationError
                }
                CreatePipelineAsyncStatus::InternalError => {
                    ffi::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_InternalError
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum CullMode {
        Undefined,
        None,
        Front,
        Back,
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
            match value {
                CullMode::Undefined => ffi::WGPUCullMode_WGPUCullMode_Undefined,
                CullMode::None => ffi::WGPUCullMode_WGPUCullMode_None,
                CullMode::Front => ffi::WGPUCullMode_WGPUCullMode_Front,
                CullMode::Back => ffi::WGPUCullMode_WGPUCullMode_Back,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DeviceLostReason {
        Unknown,
        Destroyed,
        CallbackCancelled,
        FailedCreation,
    }
    impl From<ffi::WGPUDeviceLostReason> for DeviceLostReason {
        fn from(value: ffi::WGPUDeviceLostReason) -> Self {
            match value {
                ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Unknown => {
                    DeviceLostReason::Unknown
                }
                ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Destroyed => {
                    DeviceLostReason::Destroyed
                }
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
            match value {
                DeviceLostReason::Unknown => {
                    ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Unknown
                }
                DeviceLostReason::Destroyed => {
                    ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_Destroyed
                }
                DeviceLostReason::CallbackCancelled => {
                    ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_CallbackCancelled
                }
                DeviceLostReason::FailedCreation => {
                    ffi::WGPUDeviceLostReason_WGPUDeviceLostReason_FailedCreation
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ErrorFilter {
        Validation,
        OutOfMemory,
        Internal,
    }
    impl From<ffi::WGPUErrorFilter> for ErrorFilter {
        fn from(value: ffi::WGPUErrorFilter) -> Self {
            match value {
                ffi::WGPUErrorFilter_WGPUErrorFilter_Validation => {
                    ErrorFilter::Validation
                }
                ffi::WGPUErrorFilter_WGPUErrorFilter_OutOfMemory => {
                    ErrorFilter::OutOfMemory
                }
                ffi::WGPUErrorFilter_WGPUErrorFilter_Internal => ErrorFilter::Internal,
                _ => ErrorFilter::Validation,
            }
        }
    }
    impl From<ErrorFilter> for ffi::WGPUErrorFilter {
        fn from(value: ErrorFilter) -> Self {
            match value {
                ErrorFilter::Validation => {
                    ffi::WGPUErrorFilter_WGPUErrorFilter_Validation
                }
                ErrorFilter::OutOfMemory => {
                    ffi::WGPUErrorFilter_WGPUErrorFilter_OutOfMemory
                }
                ErrorFilter::Internal => ffi::WGPUErrorFilter_WGPUErrorFilter_Internal,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ErrorType {
        NoError,
        Validation,
        OutOfMemory,
        Internal,
        Unknown,
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
            match value {
                ErrorType::NoError => ffi::WGPUErrorType_WGPUErrorType_NoError,
                ErrorType::Validation => ffi::WGPUErrorType_WGPUErrorType_Validation,
                ErrorType::OutOfMemory => ffi::WGPUErrorType_WGPUErrorType_OutOfMemory,
                ErrorType::Internal => ffi::WGPUErrorType_WGPUErrorType_Internal,
                ErrorType::Unknown => ffi::WGPUErrorType_WGPUErrorType_Unknown,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ExternalTextureRotation {
        Rotate0Degrees,
        Rotate90Degrees,
        Rotate180Degrees,
        Rotate270Degrees,
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
            match value {
                ExternalTextureRotation::Rotate0Degrees => {
                    ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate0Degrees
                }
                ExternalTextureRotation::Rotate90Degrees => {
                    ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate90Degrees
                }
                ExternalTextureRotation::Rotate180Degrees => {
                    ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate180Degrees
                }
                ExternalTextureRotation::Rotate270Degrees => {
                    ffi::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate270Degrees
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FeatureLevel {
        Undefined,
        Compatibility,
        Core,
    }
    impl From<ffi::WGPUFeatureLevel> for FeatureLevel {
        fn from(value: ffi::WGPUFeatureLevel) -> Self {
            match value {
                ffi::WGPUFeatureLevel_WGPUFeatureLevel_Undefined => {
                    FeatureLevel::Undefined
                }
                ffi::WGPUFeatureLevel_WGPUFeatureLevel_Compatibility => {
                    FeatureLevel::Compatibility
                }
                ffi::WGPUFeatureLevel_WGPUFeatureLevel_Core => FeatureLevel::Core,
                _ => FeatureLevel::Undefined,
            }
        }
    }
    impl From<FeatureLevel> for ffi::WGPUFeatureLevel {
        fn from(value: FeatureLevel) -> Self {
            match value {
                FeatureLevel::Undefined => {
                    ffi::WGPUFeatureLevel_WGPUFeatureLevel_Undefined
                }
                FeatureLevel::Compatibility => {
                    ffi::WGPUFeatureLevel_WGPUFeatureLevel_Compatibility
                }
                FeatureLevel::Core => ffi::WGPUFeatureLevel_WGPUFeatureLevel_Core,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FeatureName {
        CoreFeaturesAndLimits,
        DepthClipControl,
        Depth32FloatStencil8,
        TextureCompressionBc,
        TextureCompressionBcSliced3D,
        TextureCompressionEtc2,
        TextureCompressionAstc,
        TextureCompressionAstcSliced3D,
        TimestampQuery,
        IndirectFirstInstance,
        ShaderF16,
        Rg11B10UfloatRenderable,
        Bgra8UnormStorage,
        Float32Filterable,
        Float32Blendable,
        ClipDistances,
        DualSourceBlending,
        Subgroups,
        TextureFormatsTier1,
        TextureFormatsTier2,
        PrimitiveIndex,
        TextureComponentSwizzle,
        DawnInternalUsages,
        DawnMultiPlanarFormats,
        DawnNative,
        ChromiumExperimentalTimestampQueryInsidePasses,
        ImplicitDeviceSynchronization,
        TransientAttachments,
        MsaaRenderToSingleSampled,
        D3D11MultithreadProtected,
        AngleTextureSharing,
        PixelLocalStorageCoherent,
        PixelLocalStorageNonCoherent,
        Unorm16TextureFormats,
        MultiPlanarFormatExtendedUsages,
        MultiPlanarFormatP010,
        HostMappedPointer,
        MultiPlanarRenderTargets,
        MultiPlanarFormatNv12A,
        FramebufferFetch,
        BufferMapExtendedUsages,
        AdapterPropertiesMemoryHeaps,
        AdapterPropertiesD3D,
        AdapterPropertiesVk,
        DawnFormatCapabilities,
        DawnDrmFormatCapabilities,
        MultiPlanarFormatNv16,
        MultiPlanarFormatNv24,
        MultiPlanarFormatP210,
        MultiPlanarFormatP410,
        SharedTextureMemoryVkDedicatedAllocation,
        SharedTextureMemoryAHardwareBuffer,
        SharedTextureMemoryDmaBuf,
        SharedTextureMemoryOpaqueFD,
        SharedTextureMemoryZirconHandle,
        SharedTextureMemoryDXGISharedHandle,
        SharedTextureMemoryD3D11Texture2D,
        SharedTextureMemoryIOSurface,
        SharedTextureMemoryEGLImage,
        SharedFenceVkSemaphoreOpaqueFD,
        SharedFenceSyncFD,
        SharedFenceVkSemaphoreZirconHandle,
        SharedFenceDXGISharedHandle,
        SharedFenceMTLSharedEvent,
        SharedBufferMemoryD3D12Resource,
        StaticSamplers,
        YCbCrVulkanSamplers,
        ShaderModuleCompilationOptions,
        DawnLoadResolveTexture,
        DawnPartialLoadResolveTexture,
        MultiDrawIndirect,
        DawnTexelCopyBufferRowAlignment,
        FlexibleTextureViews,
        ChromiumExperimentalSubgroupMatrix,
        SharedFenceEGLSync,
        DawnDeviceAllocatorControl,
        AdapterPropertiesWGPU,
        SharedBufferMemoryD3D12SharedMemoryFileMappingHandle,
        SharedTextureMemoryD3D12Resource,
        ChromiumExperimentalSamplingResourceTable,
        ChromiumExperimentalSubgroupSizeControl,
        AtomicVec2UMinMax,
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
                ffi::WGPUFeatureName_WGPUFeatureName_DawnNative => {
                    FeatureName::DawnNative
                }
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
            match value {
                FeatureName::CoreFeaturesAndLimits => {
                    ffi::WGPUFeatureName_WGPUFeatureName_CoreFeaturesAndLimits
                }
                FeatureName::DepthClipControl => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DepthClipControl
                }
                FeatureName::Depth32FloatStencil8 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_Depth32FloatStencil8
                }
                FeatureName::TextureCompressionBc => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionBC
                }
                FeatureName::TextureCompressionBcSliced3D => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionBCSliced3D
                }
                FeatureName::TextureCompressionEtc2 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionETC2
                }
                FeatureName::TextureCompressionAstc => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTC
                }
                FeatureName::TextureCompressionAstcSliced3D => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTCSliced3D
                }
                FeatureName::TimestampQuery => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TimestampQuery
                }
                FeatureName::IndirectFirstInstance => {
                    ffi::WGPUFeatureName_WGPUFeatureName_IndirectFirstInstance
                }
                FeatureName::ShaderF16 => ffi::WGPUFeatureName_WGPUFeatureName_ShaderF16,
                FeatureName::Rg11B10UfloatRenderable => {
                    ffi::WGPUFeatureName_WGPUFeatureName_RG11B10UfloatRenderable
                }
                FeatureName::Bgra8UnormStorage => {
                    ffi::WGPUFeatureName_WGPUFeatureName_BGRA8UnormStorage
                }
                FeatureName::Float32Filterable => {
                    ffi::WGPUFeatureName_WGPUFeatureName_Float32Filterable
                }
                FeatureName::Float32Blendable => {
                    ffi::WGPUFeatureName_WGPUFeatureName_Float32Blendable
                }
                FeatureName::ClipDistances => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ClipDistances
                }
                FeatureName::DualSourceBlending => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DualSourceBlending
                }
                FeatureName::Subgroups => ffi::WGPUFeatureName_WGPUFeatureName_Subgroups,
                FeatureName::TextureFormatsTier1 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureFormatsTier1
                }
                FeatureName::TextureFormatsTier2 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureFormatsTier2
                }
                FeatureName::PrimitiveIndex => {
                    ffi::WGPUFeatureName_WGPUFeatureName_PrimitiveIndex
                }
                FeatureName::TextureComponentSwizzle => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TextureComponentSwizzle
                }
                FeatureName::DawnInternalUsages => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnInternalUsages
                }
                FeatureName::DawnMultiPlanarFormats => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnMultiPlanarFormats
                }
                FeatureName::DawnNative => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnNative
                }
                FeatureName::ChromiumExperimentalTimestampQueryInsidePasses => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalTimestampQueryInsidePasses
                }
                FeatureName::ImplicitDeviceSynchronization => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ImplicitDeviceSynchronization
                }
                FeatureName::TransientAttachments => {
                    ffi::WGPUFeatureName_WGPUFeatureName_TransientAttachments
                }
                FeatureName::MsaaRenderToSingleSampled => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MSAARenderToSingleSampled
                }
                FeatureName::D3D11MultithreadProtected => {
                    ffi::WGPUFeatureName_WGPUFeatureName_D3D11MultithreadProtected
                }
                FeatureName::AngleTextureSharing => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ANGLETextureSharing
                }
                FeatureName::PixelLocalStorageCoherent => {
                    ffi::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageCoherent
                }
                FeatureName::PixelLocalStorageNonCoherent => {
                    ffi::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageNonCoherent
                }
                FeatureName::Unorm16TextureFormats => {
                    ffi::WGPUFeatureName_WGPUFeatureName_Unorm16TextureFormats
                }
                FeatureName::MultiPlanarFormatExtendedUsages => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatExtendedUsages
                }
                FeatureName::MultiPlanarFormatP010 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP010
                }
                FeatureName::HostMappedPointer => {
                    ffi::WGPUFeatureName_WGPUFeatureName_HostMappedPointer
                }
                FeatureName::MultiPlanarRenderTargets => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarRenderTargets
                }
                FeatureName::MultiPlanarFormatNv12A => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv12a
                }
                FeatureName::FramebufferFetch => {
                    ffi::WGPUFeatureName_WGPUFeatureName_FramebufferFetch
                }
                FeatureName::BufferMapExtendedUsages => {
                    ffi::WGPUFeatureName_WGPUFeatureName_BufferMapExtendedUsages
                }
                FeatureName::AdapterPropertiesMemoryHeaps => {
                    ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesMemoryHeaps
                }
                FeatureName::AdapterPropertiesD3D => {
                    ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesD3D
                }
                FeatureName::AdapterPropertiesVk => {
                    ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesVk
                }
                FeatureName::DawnFormatCapabilities => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnFormatCapabilities
                }
                FeatureName::DawnDrmFormatCapabilities => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnDrmFormatCapabilities
                }
                FeatureName::MultiPlanarFormatNv16 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv16
                }
                FeatureName::MultiPlanarFormatNv24 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv24
                }
                FeatureName::MultiPlanarFormatP210 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP210
                }
                FeatureName::MultiPlanarFormatP410 => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP410
                }
                FeatureName::SharedTextureMemoryVkDedicatedAllocation => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryVkDedicatedAllocation
                }
                FeatureName::SharedTextureMemoryAHardwareBuffer => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryAHardwareBuffer
                }
                FeatureName::SharedTextureMemoryDmaBuf => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDmaBuf
                }
                FeatureName::SharedTextureMemoryOpaqueFD => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryOpaqueFD
                }
                FeatureName::SharedTextureMemoryZirconHandle => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryZirconHandle
                }
                FeatureName::SharedTextureMemoryDXGISharedHandle => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDXGISharedHandle
                }
                FeatureName::SharedTextureMemoryD3D11Texture2D => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryD3D11Texture2D
                }
                FeatureName::SharedTextureMemoryIOSurface => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryIOSurface
                }
                FeatureName::SharedTextureMemoryEGLImage => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryEGLImage
                }
                FeatureName::SharedFenceVkSemaphoreOpaqueFD => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreOpaqueFD
                }
                FeatureName::SharedFenceSyncFD => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceSyncFD
                }
                FeatureName::SharedFenceVkSemaphoreZirconHandle => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreZirconHandle
                }
                FeatureName::SharedFenceDXGISharedHandle => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceDXGISharedHandle
                }
                FeatureName::SharedFenceMTLSharedEvent => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceMTLSharedEvent
                }
                FeatureName::SharedBufferMemoryD3D12Resource => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedBufferMemoryD3D12Resource
                }
                FeatureName::StaticSamplers => {
                    ffi::WGPUFeatureName_WGPUFeatureName_StaticSamplers
                }
                FeatureName::YCbCrVulkanSamplers => {
                    ffi::WGPUFeatureName_WGPUFeatureName_YCbCrVulkanSamplers
                }
                FeatureName::ShaderModuleCompilationOptions => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ShaderModuleCompilationOptions
                }
                FeatureName::DawnLoadResolveTexture => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnLoadResolveTexture
                }
                FeatureName::DawnPartialLoadResolveTexture => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnPartialLoadResolveTexture
                }
                FeatureName::MultiDrawIndirect => {
                    ffi::WGPUFeatureName_WGPUFeatureName_MultiDrawIndirect
                }
                FeatureName::DawnTexelCopyBufferRowAlignment => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnTexelCopyBufferRowAlignment
                }
                FeatureName::FlexibleTextureViews => {
                    ffi::WGPUFeatureName_WGPUFeatureName_FlexibleTextureViews
                }
                FeatureName::ChromiumExperimentalSubgroupMatrix => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSubgroupMatrix
                }
                FeatureName::SharedFenceEGLSync => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedFenceEGLSync
                }
                FeatureName::DawnDeviceAllocatorControl => {
                    ffi::WGPUFeatureName_WGPUFeatureName_DawnDeviceAllocatorControl
                }
                FeatureName::AdapterPropertiesWGPU => {
                    ffi::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesWGPU
                }
                FeatureName::SharedBufferMemoryD3D12SharedMemoryFileMappingHandle => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedBufferMemoryD3D12SharedMemoryFileMappingHandle
                }
                FeatureName::SharedTextureMemoryD3D12Resource => {
                    ffi::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryD3D12Resource
                }
                FeatureName::ChromiumExperimentalSamplingResourceTable => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSamplingResourceTable
                }
                FeatureName::ChromiumExperimentalSubgroupSizeControl => {
                    ffi::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSubgroupSizeControl
                }
                FeatureName::AtomicVec2UMinMax => {
                    ffi::WGPUFeatureName_WGPUFeatureName_AtomicVec2uMinMax
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FilterMode {
        Undefined,
        Nearest,
        Linear,
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
            match value {
                FilterMode::Undefined => ffi::WGPUFilterMode_WGPUFilterMode_Undefined,
                FilterMode::Nearest => ffi::WGPUFilterMode_WGPUFilterMode_Nearest,
                FilterMode::Linear => ffi::WGPUFilterMode_WGPUFilterMode_Linear,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FrontFace {
        Undefined,
        Ccw,
        Cw,
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
            match value {
                FrontFace::Undefined => ffi::WGPUFrontFace_WGPUFrontFace_Undefined,
                FrontFace::Ccw => ffi::WGPUFrontFace_WGPUFrontFace_CCW,
                FrontFace::Cw => ffi::WGPUFrontFace_WGPUFrontFace_CW,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum IndexFormat {
        Undefined,
        Uint16,
        Uint32,
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
            match value {
                IndexFormat::Undefined => ffi::WGPUIndexFormat_WGPUIndexFormat_Undefined,
                IndexFormat::Uint16 => ffi::WGPUIndexFormat_WGPUIndexFormat_Uint16,
                IndexFormat::Uint32 => ffi::WGPUIndexFormat_WGPUIndexFormat_Uint32,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum InstanceFeatureName {
        TimedWaitAny,
        ShaderSourceSPIRV,
        MultipleDevicesPerAdapter,
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
            match value {
                InstanceFeatureName::TimedWaitAny => {
                    ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_TimedWaitAny
                }
                InstanceFeatureName::ShaderSourceSPIRV => {
                    ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_ShaderSourceSPIRV
                }
                InstanceFeatureName::MultipleDevicesPerAdapter => {
                    ffi::WGPUInstanceFeatureName_WGPUInstanceFeatureName_MultipleDevicesPerAdapter
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum LoadOp {
        Undefined,
        Load,
        Clear,
        ExpandResolveTexture,
    }
    impl From<ffi::WGPULoadOp> for LoadOp {
        fn from(value: ffi::WGPULoadOp) -> Self {
            match value {
                ffi::WGPULoadOp_WGPULoadOp_Undefined => LoadOp::Undefined,
                ffi::WGPULoadOp_WGPULoadOp_Load => LoadOp::Load,
                ffi::WGPULoadOp_WGPULoadOp_Clear => LoadOp::Clear,
                ffi::WGPULoadOp_WGPULoadOp_ExpandResolveTexture => {
                    LoadOp::ExpandResolveTexture
                }
                _ => LoadOp::Undefined,
            }
        }
    }
    impl From<LoadOp> for ffi::WGPULoadOp {
        fn from(value: LoadOp) -> Self {
            match value {
                LoadOp::Undefined => ffi::WGPULoadOp_WGPULoadOp_Undefined,
                LoadOp::Load => ffi::WGPULoadOp_WGPULoadOp_Load,
                LoadOp::Clear => ffi::WGPULoadOp_WGPULoadOp_Clear,
                LoadOp::ExpandResolveTexture => {
                    ffi::WGPULoadOp_WGPULoadOp_ExpandResolveTexture
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum LoggingType {
        Verbose,
        Info,
        Warning,
        Error,
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
            match value {
                LoggingType::Verbose => ffi::WGPULoggingType_WGPULoggingType_Verbose,
                LoggingType::Info => ffi::WGPULoggingType_WGPULoggingType_Info,
                LoggingType::Warning => ffi::WGPULoggingType_WGPULoggingType_Warning,
                LoggingType::Error => ffi::WGPULoggingType_WGPULoggingType_Error,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum MapAsyncStatus {
        Success,
        CallbackCancelled,
        Error,
        Aborted,
    }
    impl From<ffi::WGPUMapAsyncStatus> for MapAsyncStatus {
        fn from(value: ffi::WGPUMapAsyncStatus) -> Self {
            match value {
                ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Success => {
                    MapAsyncStatus::Success
                }
                ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_CallbackCancelled => {
                    MapAsyncStatus::CallbackCancelled
                }
                ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Error => MapAsyncStatus::Error,
                ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Aborted => {
                    MapAsyncStatus::Aborted
                }
                _ => MapAsyncStatus::Success,
            }
        }
    }
    impl From<MapAsyncStatus> for ffi::WGPUMapAsyncStatus {
        fn from(value: MapAsyncStatus) -> Self {
            match value {
                MapAsyncStatus::Success => {
                    ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Success
                }
                MapAsyncStatus::CallbackCancelled => {
                    ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_CallbackCancelled
                }
                MapAsyncStatus::Error => ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Error,
                MapAsyncStatus::Aborted => {
                    ffi::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Aborted
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum MipmapFilterMode {
        Undefined,
        Nearest,
        Linear,
    }
    impl From<ffi::WGPUMipmapFilterMode> for MipmapFilterMode {
        fn from(value: ffi::WGPUMipmapFilterMode) -> Self {
            match value {
                ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Undefined => {
                    MipmapFilterMode::Undefined
                }
                ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Nearest => {
                    MipmapFilterMode::Nearest
                }
                ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Linear => {
                    MipmapFilterMode::Linear
                }
                _ => MipmapFilterMode::Undefined,
            }
        }
    }
    impl From<MipmapFilterMode> for ffi::WGPUMipmapFilterMode {
        fn from(value: MipmapFilterMode) -> Self {
            match value {
                MipmapFilterMode::Undefined => {
                    ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Undefined
                }
                MipmapFilterMode::Nearest => {
                    ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Nearest
                }
                MipmapFilterMode::Linear => {
                    ffi::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Linear
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum OptionalBool {
        False,
        True,
        Undefined,
    }
    impl From<ffi::WGPUOptionalBool> for OptionalBool {
        fn from(value: ffi::WGPUOptionalBool) -> Self {
            match value {
                ffi::WGPUOptionalBool_WGPUOptionalBool_False => OptionalBool::False,
                ffi::WGPUOptionalBool_WGPUOptionalBool_True => OptionalBool::True,
                ffi::WGPUOptionalBool_WGPUOptionalBool_Undefined => {
                    OptionalBool::Undefined
                }
                _ => OptionalBool::False,
            }
        }
    }
    impl From<OptionalBool> for ffi::WGPUOptionalBool {
        fn from(value: OptionalBool) -> Self {
            match value {
                OptionalBool::False => ffi::WGPUOptionalBool_WGPUOptionalBool_False,
                OptionalBool::True => ffi::WGPUOptionalBool_WGPUOptionalBool_True,
                OptionalBool::Undefined => {
                    ffi::WGPUOptionalBool_WGPUOptionalBool_Undefined
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PopErrorScopeStatus {
        Success,
        CallbackCancelled,
        Error,
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
            match value {
                PopErrorScopeStatus::Success => {
                    ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Success
                }
                PopErrorScopeStatus::CallbackCancelled => {
                    ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_CallbackCancelled
                }
                PopErrorScopeStatus::Error => {
                    ffi::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Error
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PowerPreference {
        Undefined,
        LowPower,
        HighPerformance,
    }
    impl From<ffi::WGPUPowerPreference> for PowerPreference {
        fn from(value: ffi::WGPUPowerPreference) -> Self {
            match value {
                ffi::WGPUPowerPreference_WGPUPowerPreference_Undefined => {
                    PowerPreference::Undefined
                }
                ffi::WGPUPowerPreference_WGPUPowerPreference_LowPower => {
                    PowerPreference::LowPower
                }
                ffi::WGPUPowerPreference_WGPUPowerPreference_HighPerformance => {
                    PowerPreference::HighPerformance
                }
                _ => PowerPreference::Undefined,
            }
        }
    }
    impl From<PowerPreference> for ffi::WGPUPowerPreference {
        fn from(value: PowerPreference) -> Self {
            match value {
                PowerPreference::Undefined => {
                    ffi::WGPUPowerPreference_WGPUPowerPreference_Undefined
                }
                PowerPreference::LowPower => {
                    ffi::WGPUPowerPreference_WGPUPowerPreference_LowPower
                }
                PowerPreference::HighPerformance => {
                    ffi::WGPUPowerPreference_WGPUPowerPreference_HighPerformance
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PredefinedColorSpace {
        SRgb,
        DisplayP3,
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
            match value {
                PredefinedColorSpace::SRgb => {
                    ffi::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_SRGB
                }
                PredefinedColorSpace::DisplayP3 => {
                    ffi::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_DisplayP3
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PresentMode {
        Undefined,
        Fifo,
        FifoRelaxed,
        Immediate,
        Mailbox,
    }
    impl From<ffi::WGPUPresentMode> for PresentMode {
        fn from(value: ffi::WGPUPresentMode) -> Self {
            match value {
                ffi::WGPUPresentMode_WGPUPresentMode_Undefined => PresentMode::Undefined,
                ffi::WGPUPresentMode_WGPUPresentMode_Fifo => PresentMode::Fifo,
                ffi::WGPUPresentMode_WGPUPresentMode_FifoRelaxed => {
                    PresentMode::FifoRelaxed
                }
                ffi::WGPUPresentMode_WGPUPresentMode_Immediate => PresentMode::Immediate,
                ffi::WGPUPresentMode_WGPUPresentMode_Mailbox => PresentMode::Mailbox,
                _ => PresentMode::Undefined,
            }
        }
    }
    impl From<PresentMode> for ffi::WGPUPresentMode {
        fn from(value: PresentMode) -> Self {
            match value {
                PresentMode::Undefined => ffi::WGPUPresentMode_WGPUPresentMode_Undefined,
                PresentMode::Fifo => ffi::WGPUPresentMode_WGPUPresentMode_Fifo,
                PresentMode::FifoRelaxed => {
                    ffi::WGPUPresentMode_WGPUPresentMode_FifoRelaxed
                }
                PresentMode::Immediate => ffi::WGPUPresentMode_WGPUPresentMode_Immediate,
                PresentMode::Mailbox => ffi::WGPUPresentMode_WGPUPresentMode_Mailbox,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PrimitiveTopology {
        Undefined,
        PointList,
        LineList,
        LineStrip,
        TriangleList,
        TriangleStrip,
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
            match value {
                PrimitiveTopology::Undefined => {
                    ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_Undefined
                }
                PrimitiveTopology::PointList => {
                    ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_PointList
                }
                PrimitiveTopology::LineList => {
                    ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineList
                }
                PrimitiveTopology::LineStrip => {
                    ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineStrip
                }
                PrimitiveTopology::TriangleList => {
                    ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleList
                }
                PrimitiveTopology::TriangleStrip => {
                    ffi::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleStrip
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum QueryType {
        Occlusion,
        Timestamp,
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
            match value {
                QueryType::Occlusion => ffi::WGPUQueryType_WGPUQueryType_Occlusion,
                QueryType::Timestamp => ffi::WGPUQueryType_WGPUQueryType_Timestamp,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum QueueWorkDoneStatus {
        Success,
        CallbackCancelled,
        Error,
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
            match value {
                QueueWorkDoneStatus::Success => {
                    ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Success
                }
                QueueWorkDoneStatus::CallbackCancelled => {
                    ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_CallbackCancelled
                }
                QueueWorkDoneStatus::Error => {
                    ffi::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Error
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RequestAdapterStatus {
        Success,
        CallbackCancelled,
        Unavailable,
        Error,
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
            match value {
                RequestAdapterStatus::Success => {
                    ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Success
                }
                RequestAdapterStatus::CallbackCancelled => {
                    ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_CallbackCancelled
                }
                RequestAdapterStatus::Unavailable => {
                    ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Unavailable
                }
                RequestAdapterStatus::Error => {
                    ffi::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Error
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RequestDeviceStatus {
        Success,
        CallbackCancelled,
        Error,
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
            match value {
                RequestDeviceStatus::Success => {
                    ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Success
                }
                RequestDeviceStatus::CallbackCancelled => {
                    ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_CallbackCancelled
                }
                RequestDeviceStatus::Error => {
                    ffi::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Error
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SType {
        ShaderSourceSPIRV,
        ShaderSourceWGSL,
        RenderPassMaxDrawCount,
        SurfaceSourceMetalLayer,
        SurfaceSourceWindowsHWND,
        SurfaceSourceXlibWindow,
        SurfaceSourceWaylandSurface,
        SurfaceSourceAndroidNativeWindow,
        SurfaceSourceXCBWindow,
        SurfaceColorManagement,
        RequestAdapterWebXROptions,
        TextureComponentSwizzleDescriptor,
        CompatibilityModeLimits,
        TextureBindingViewDimensionDescriptor,
        EmscriptenSurfaceSourceCanvasHTMLSelector,
        SurfaceDescriptorFromWindowsCoreWindow,
        ExternalTextureBindingEntry,
        ExternalTextureBindingLayout,
        SurfaceDescriptorFromWindowsUWPSwapChainPanel,
        DawnTextureInternalUsageDescriptor,
        DawnEncoderInternalUsageDescriptor,
        DawnInstanceDescriptor,
        DawnCacheDeviceDescriptor,
        DawnAdapterPropertiesPowerPreference,
        DawnBufferDescriptorErrorInfoFromWireClient,
        DawnTogglesDescriptor,
        DawnShaderModuleSPIRVOptionsDescriptor,
        RequestAdapterOptionsLuid,
        RequestAdapterOptionsGetGlProc,
        RequestAdapterOptionsD3D11Device,
        DawnRenderPassSampleCount,
        RenderPassPixelLocalStorage,
        PipelineLayoutPixelLocalStorage,
        BufferHostMappedPointer,
        AdapterPropertiesMemoryHeaps,
        AdapterPropertiesD3D,
        AdapterPropertiesVk,
        DawnWireWGSLControl,
        DawnWGSLBlocklist,
        DawnDrmFormatCapabilities,
        ShaderModuleCompilationOptions,
        ColorTargetStateExpandResolveTextureDawn,
        RenderPassDescriptorExpandResolveRect,
        SharedTextureMemoryVkDedicatedAllocationDescriptor,
        SharedTextureMemoryAHardwareBufferDescriptor,
        SharedTextureMemoryDmaBufDescriptor,
        SharedTextureMemoryOpaqueFDDescriptor,
        SharedTextureMemoryZirconHandleDescriptor,
        SharedTextureMemoryDXGISharedHandleDescriptor,
        SharedTextureMemoryD3D11Texture2DDescriptor,
        SharedTextureMemoryIOSurfaceDescriptor,
        SharedTextureMemoryEGLImageDescriptor,
        SharedTextureMemoryInitializedBeginState,
        SharedTextureMemoryInitializedEndState,
        SharedTextureMemoryVkImageLayoutBeginState,
        SharedTextureMemoryVkImageLayoutEndState,
        SharedTextureMemoryD3DSwapchainBeginState,
        SharedFenceVkSemaphoreOpaqueFDDescriptor,
        SharedFenceVkSemaphoreOpaqueFDExportInfo,
        SharedFenceSyncFDDescriptor,
        SharedFenceSyncFDExportInfo,
        SharedFenceVkSemaphoreZirconHandleDescriptor,
        SharedFenceVkSemaphoreZirconHandleExportInfo,
        SharedFenceDXGISharedHandleDescriptor,
        SharedFenceDXGISharedHandleExportInfo,
        SharedFenceMTLSharedEventDescriptor,
        SharedFenceMTLSharedEventExportInfo,
        SharedBufferMemoryD3D12ResourceDescriptor,
        StaticSamplerBindingLayout,
        YCbCrVkDescriptor,
        SharedTextureMemoryAHardwareBufferProperties,
        AHardwareBufferProperties,
        DawnTexelCopyBufferRowAlignmentLimits,
        AdapterPropertiesSubgroupMatrixConfigs,
        SharedFenceEGLSyncDescriptor,
        SharedFenceEGLSyncExportInfo,
        DawnInjectedInvalidSType,
        DawnCompilationMessageUtf16,
        DawnFakeBufferOOMForTesting,
        SurfaceDescriptorFromWindowsWinUISwapChainPanel,
        DawnDeviceAllocatorControl,
        DawnHostMappedPointerLimits,
        RenderPassDescriptorResolveRect,
        RequestAdapterWebGPUBackendOptions,
        DawnFakeDeviceInitializeErrorForTesting,
        SharedTextureMemoryD3D11BeginState,
        DawnConsumeAdapterDescriptor,
        TexelBufferBindingEntry,
        TexelBufferBindingLayout,
        SharedTextureMemoryMetalEndAccessState,
        AdapterPropertiesWGPU,
        SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor,
        SharedTextureMemoryD3D12ResourceDescriptor,
        RequestAdapterOptionsAngleVirtualizationGroup,
        PipelineLayoutResourceTable,
        AdapterPropertiesExplicitComputeSubgroupSizeConfigs,
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
                ffi::WGPUSType_WGPUSType_AdapterPropertiesD3D => {
                    SType::AdapterPropertiesD3D
                }
                ffi::WGPUSType_WGPUSType_AdapterPropertiesVk => {
                    SType::AdapterPropertiesVk
                }
                ffi::WGPUSType_WGPUSType_DawnWireWGSLControl => {
                    SType::DawnWireWGSLControl
                }
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
            match value {
                SType::ShaderSourceSPIRV => ffi::WGPUSType_WGPUSType_ShaderSourceSPIRV,
                SType::ShaderSourceWGSL => ffi::WGPUSType_WGPUSType_ShaderSourceWGSL,
                SType::RenderPassMaxDrawCount => {
                    ffi::WGPUSType_WGPUSType_RenderPassMaxDrawCount
                }
                SType::SurfaceSourceMetalLayer => {
                    ffi::WGPUSType_WGPUSType_SurfaceSourceMetalLayer
                }
                SType::SurfaceSourceWindowsHWND => {
                    ffi::WGPUSType_WGPUSType_SurfaceSourceWindowsHWND
                }
                SType::SurfaceSourceXlibWindow => {
                    ffi::WGPUSType_WGPUSType_SurfaceSourceXlibWindow
                }
                SType::SurfaceSourceWaylandSurface => {
                    ffi::WGPUSType_WGPUSType_SurfaceSourceWaylandSurface
                }
                SType::SurfaceSourceAndroidNativeWindow => {
                    ffi::WGPUSType_WGPUSType_SurfaceSourceAndroidNativeWindow
                }
                SType::SurfaceSourceXCBWindow => {
                    ffi::WGPUSType_WGPUSType_SurfaceSourceXCBWindow
                }
                SType::SurfaceColorManagement => {
                    ffi::WGPUSType_WGPUSType_SurfaceColorManagement
                }
                SType::RequestAdapterWebXROptions => {
                    ffi::WGPUSType_WGPUSType_RequestAdapterWebXROptions
                }
                SType::TextureComponentSwizzleDescriptor => {
                    ffi::WGPUSType_WGPUSType_TextureComponentSwizzleDescriptor
                }
                SType::CompatibilityModeLimits => {
                    ffi::WGPUSType_WGPUSType_CompatibilityModeLimits
                }
                SType::TextureBindingViewDimensionDescriptor => {
                    ffi::WGPUSType_WGPUSType_TextureBindingViewDimensionDescriptor
                }
                SType::EmscriptenSurfaceSourceCanvasHTMLSelector => {
                    ffi::WGPUSType_WGPUSType_EmscriptenSurfaceSourceCanvasHTMLSelector
                }
                SType::SurfaceDescriptorFromWindowsCoreWindow => {
                    ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsCoreWindow
                }
                SType::ExternalTextureBindingEntry => {
                    ffi::WGPUSType_WGPUSType_ExternalTextureBindingEntry
                }
                SType::ExternalTextureBindingLayout => {
                    ffi::WGPUSType_WGPUSType_ExternalTextureBindingLayout
                }
                SType::SurfaceDescriptorFromWindowsUWPSwapChainPanel => {
                    ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsUWPSwapChainPanel
                }
                SType::DawnTextureInternalUsageDescriptor => {
                    ffi::WGPUSType_WGPUSType_DawnTextureInternalUsageDescriptor
                }
                SType::DawnEncoderInternalUsageDescriptor => {
                    ffi::WGPUSType_WGPUSType_DawnEncoderInternalUsageDescriptor
                }
                SType::DawnInstanceDescriptor => {
                    ffi::WGPUSType_WGPUSType_DawnInstanceDescriptor
                }
                SType::DawnCacheDeviceDescriptor => {
                    ffi::WGPUSType_WGPUSType_DawnCacheDeviceDescriptor
                }
                SType::DawnAdapterPropertiesPowerPreference => {
                    ffi::WGPUSType_WGPUSType_DawnAdapterPropertiesPowerPreference
                }
                SType::DawnBufferDescriptorErrorInfoFromWireClient => {
                    ffi::WGPUSType_WGPUSType_DawnBufferDescriptorErrorInfoFromWireClient
                }
                SType::DawnTogglesDescriptor => {
                    ffi::WGPUSType_WGPUSType_DawnTogglesDescriptor
                }
                SType::DawnShaderModuleSPIRVOptionsDescriptor => {
                    ffi::WGPUSType_WGPUSType_DawnShaderModuleSPIRVOptionsDescriptor
                }
                SType::RequestAdapterOptionsLuid => {
                    ffi::WGPUSType_WGPUSType_RequestAdapterOptionsLUID
                }
                SType::RequestAdapterOptionsGetGlProc => {
                    ffi::WGPUSType_WGPUSType_RequestAdapterOptionsGetGLProc
                }
                SType::RequestAdapterOptionsD3D11Device => {
                    ffi::WGPUSType_WGPUSType_RequestAdapterOptionsD3D11Device
                }
                SType::DawnRenderPassSampleCount => {
                    ffi::WGPUSType_WGPUSType_DawnRenderPassSampleCount
                }
                SType::RenderPassPixelLocalStorage => {
                    ffi::WGPUSType_WGPUSType_RenderPassPixelLocalStorage
                }
                SType::PipelineLayoutPixelLocalStorage => {
                    ffi::WGPUSType_WGPUSType_PipelineLayoutPixelLocalStorage
                }
                SType::BufferHostMappedPointer => {
                    ffi::WGPUSType_WGPUSType_BufferHostMappedPointer
                }
                SType::AdapterPropertiesMemoryHeaps => {
                    ffi::WGPUSType_WGPUSType_AdapterPropertiesMemoryHeaps
                }
                SType::AdapterPropertiesD3D => {
                    ffi::WGPUSType_WGPUSType_AdapterPropertiesD3D
                }
                SType::AdapterPropertiesVk => {
                    ffi::WGPUSType_WGPUSType_AdapterPropertiesVk
                }
                SType::DawnWireWGSLControl => {
                    ffi::WGPUSType_WGPUSType_DawnWireWGSLControl
                }
                SType::DawnWGSLBlocklist => ffi::WGPUSType_WGPUSType_DawnWGSLBlocklist,
                SType::DawnDrmFormatCapabilities => {
                    ffi::WGPUSType_WGPUSType_DawnDrmFormatCapabilities
                }
                SType::ShaderModuleCompilationOptions => {
                    ffi::WGPUSType_WGPUSType_ShaderModuleCompilationOptions
                }
                SType::ColorTargetStateExpandResolveTextureDawn => {
                    ffi::WGPUSType_WGPUSType_ColorTargetStateExpandResolveTextureDawn
                }
                SType::RenderPassDescriptorExpandResolveRect => {
                    ffi::WGPUSType_WGPUSType_RenderPassDescriptorExpandResolveRect
                }
                SType::SharedTextureMemoryVkDedicatedAllocationDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkDedicatedAllocationDescriptor
                }
                SType::SharedTextureMemoryAHardwareBufferDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferDescriptor
                }
                SType::SharedTextureMemoryDmaBufDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryDmaBufDescriptor
                }
                SType::SharedTextureMemoryOpaqueFDDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryOpaqueFDDescriptor
                }
                SType::SharedTextureMemoryZirconHandleDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryZirconHandleDescriptor
                }
                SType::SharedTextureMemoryDXGISharedHandleDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryDXGISharedHandleDescriptor
                }
                SType::SharedTextureMemoryD3D11Texture2DDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D11Texture2DDescriptor
                }
                SType::SharedTextureMemoryIOSurfaceDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryIOSurfaceDescriptor
                }
                SType::SharedTextureMemoryEGLImageDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryEGLImageDescriptor
                }
                SType::SharedTextureMemoryInitializedBeginState => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryInitializedBeginState
                }
                SType::SharedTextureMemoryInitializedEndState => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryInitializedEndState
                }
                SType::SharedTextureMemoryVkImageLayoutBeginState => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutBeginState
                }
                SType::SharedTextureMemoryVkImageLayoutEndState => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutEndState
                }
                SType::SharedTextureMemoryD3DSwapchainBeginState => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3DSwapchainBeginState
                }
                SType::SharedFenceVkSemaphoreOpaqueFDDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDDescriptor
                }
                SType::SharedFenceVkSemaphoreOpaqueFDExportInfo => {
                    ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDExportInfo
                }
                SType::SharedFenceSyncFDDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedFenceSyncFDDescriptor
                }
                SType::SharedFenceSyncFDExportInfo => {
                    ffi::WGPUSType_WGPUSType_SharedFenceSyncFDExportInfo
                }
                SType::SharedFenceVkSemaphoreZirconHandleDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleDescriptor
                }
                SType::SharedFenceVkSemaphoreZirconHandleExportInfo => {
                    ffi::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleExportInfo
                }
                SType::SharedFenceDXGISharedHandleDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleDescriptor
                }
                SType::SharedFenceDXGISharedHandleExportInfo => {
                    ffi::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleExportInfo
                }
                SType::SharedFenceMTLSharedEventDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedFenceMTLSharedEventDescriptor
                }
                SType::SharedFenceMTLSharedEventExportInfo => {
                    ffi::WGPUSType_WGPUSType_SharedFenceMTLSharedEventExportInfo
                }
                SType::SharedBufferMemoryD3D12ResourceDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedBufferMemoryD3D12ResourceDescriptor
                }
                SType::StaticSamplerBindingLayout => {
                    ffi::WGPUSType_WGPUSType_StaticSamplerBindingLayout
                }
                SType::YCbCrVkDescriptor => ffi::WGPUSType_WGPUSType_YCbCrVkDescriptor,
                SType::SharedTextureMemoryAHardwareBufferProperties => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferProperties
                }
                SType::AHardwareBufferProperties => {
                    ffi::WGPUSType_WGPUSType_AHardwareBufferProperties
                }
                SType::DawnTexelCopyBufferRowAlignmentLimits => {
                    ffi::WGPUSType_WGPUSType_DawnTexelCopyBufferRowAlignmentLimits
                }
                SType::AdapterPropertiesSubgroupMatrixConfigs => {
                    ffi::WGPUSType_WGPUSType_AdapterPropertiesSubgroupMatrixConfigs
                }
                SType::SharedFenceEGLSyncDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedFenceEGLSyncDescriptor
                }
                SType::SharedFenceEGLSyncExportInfo => {
                    ffi::WGPUSType_WGPUSType_SharedFenceEGLSyncExportInfo
                }
                SType::DawnInjectedInvalidSType => {
                    ffi::WGPUSType_WGPUSType_DawnInjectedInvalidSType
                }
                SType::DawnCompilationMessageUtf16 => {
                    ffi::WGPUSType_WGPUSType_DawnCompilationMessageUtf16
                }
                SType::DawnFakeBufferOOMForTesting => {
                    ffi::WGPUSType_WGPUSType_DawnFakeBufferOOMForTesting
                }
                SType::SurfaceDescriptorFromWindowsWinUISwapChainPanel => {
                    ffi::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsWinUISwapChainPanel
                }
                SType::DawnDeviceAllocatorControl => {
                    ffi::WGPUSType_WGPUSType_DawnDeviceAllocatorControl
                }
                SType::DawnHostMappedPointerLimits => {
                    ffi::WGPUSType_WGPUSType_DawnHostMappedPointerLimits
                }
                SType::RenderPassDescriptorResolveRect => {
                    ffi::WGPUSType_WGPUSType_RenderPassDescriptorResolveRect
                }
                SType::RequestAdapterWebGPUBackendOptions => {
                    ffi::WGPUSType_WGPUSType_RequestAdapterWebGPUBackendOptions
                }
                SType::DawnFakeDeviceInitializeErrorForTesting => {
                    ffi::WGPUSType_WGPUSType_DawnFakeDeviceInitializeErrorForTesting
                }
                SType::SharedTextureMemoryD3D11BeginState => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D11BeginState
                }
                SType::DawnConsumeAdapterDescriptor => {
                    ffi::WGPUSType_WGPUSType_DawnConsumeAdapterDescriptor
                }
                SType::TexelBufferBindingEntry => {
                    ffi::WGPUSType_WGPUSType_TexelBufferBindingEntry
                }
                SType::TexelBufferBindingLayout => {
                    ffi::WGPUSType_WGPUSType_TexelBufferBindingLayout
                }
                SType::SharedTextureMemoryMetalEndAccessState => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryMetalEndAccessState
                }
                SType::AdapterPropertiesWGPU => {
                    ffi::WGPUSType_WGPUSType_AdapterPropertiesWGPU
                }
                SType::SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedBufferMemoryD3D12SharedMemoryFileMappingHandleDescriptor
                }
                SType::SharedTextureMemoryD3D12ResourceDescriptor => {
                    ffi::WGPUSType_WGPUSType_SharedTextureMemoryD3D12ResourceDescriptor
                }
                SType::RequestAdapterOptionsAngleVirtualizationGroup => {
                    ffi::WGPUSType_WGPUSType_RequestAdapterOptionsAngleVirtualizationGroup
                }
                SType::PipelineLayoutResourceTable => {
                    ffi::WGPUSType_WGPUSType_PipelineLayoutResourceTable
                }
                SType::AdapterPropertiesExplicitComputeSubgroupSizeConfigs => {
                    ffi::WGPUSType_WGPUSType_AdapterPropertiesExplicitComputeSubgroupSizeConfigs
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SamplerBindingType {
        BindingNotUsed,
        Undefined,
        Filtering,
        NonFiltering,
        Comparison,
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
            match value {
                SamplerBindingType::BindingNotUsed => {
                    ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_BindingNotUsed
                }
                SamplerBindingType::Undefined => {
                    ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Undefined
                }
                SamplerBindingType::Filtering => {
                    ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Filtering
                }
                SamplerBindingType::NonFiltering => {
                    ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_NonFiltering
                }
                SamplerBindingType::Comparison => {
                    ffi::WGPUSamplerBindingType_WGPUSamplerBindingType_Comparison
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SharedFenceType {
        VkSemaphoreOpaqueFD,
        SyncFD,
        VkSemaphoreZirconHandle,
        DXGISharedHandle,
        MTLSharedEvent,
        EGLSync,
    }
    impl From<ffi::WGPUSharedFenceType> for SharedFenceType {
        fn from(value: ffi::WGPUSharedFenceType) -> Self {
            match value {
                ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreOpaqueFD => {
                    SharedFenceType::VkSemaphoreOpaqueFD
                }
                ffi::WGPUSharedFenceType_WGPUSharedFenceType_SyncFD => {
                    SharedFenceType::SyncFD
                }
                ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreZirconHandle => {
                    SharedFenceType::VkSemaphoreZirconHandle
                }
                ffi::WGPUSharedFenceType_WGPUSharedFenceType_DXGISharedHandle => {
                    SharedFenceType::DXGISharedHandle
                }
                ffi::WGPUSharedFenceType_WGPUSharedFenceType_MTLSharedEvent => {
                    SharedFenceType::MTLSharedEvent
                }
                ffi::WGPUSharedFenceType_WGPUSharedFenceType_EGLSync => {
                    SharedFenceType::EGLSync
                }
                _ => SharedFenceType::VkSemaphoreOpaqueFD,
            }
        }
    }
    impl From<SharedFenceType> for ffi::WGPUSharedFenceType {
        fn from(value: SharedFenceType) -> Self {
            match value {
                SharedFenceType::VkSemaphoreOpaqueFD => {
                    ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreOpaqueFD
                }
                SharedFenceType::SyncFD => {
                    ffi::WGPUSharedFenceType_WGPUSharedFenceType_SyncFD
                }
                SharedFenceType::VkSemaphoreZirconHandle => {
                    ffi::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreZirconHandle
                }
                SharedFenceType::DXGISharedHandle => {
                    ffi::WGPUSharedFenceType_WGPUSharedFenceType_DXGISharedHandle
                }
                SharedFenceType::MTLSharedEvent => {
                    ffi::WGPUSharedFenceType_WGPUSharedFenceType_MTLSharedEvent
                }
                SharedFenceType::EGLSync => {
                    ffi::WGPUSharedFenceType_WGPUSharedFenceType_EGLSync
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Status {
        Success,
        Error,
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
            match value {
                Status::Success => ffi::WGPUStatus_WGPUStatus_Success,
                Status::Error => ffi::WGPUStatus_WGPUStatus_Error,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum StencilOperation {
        Undefined,
        Keep,
        Zero,
        Replace,
        Invert,
        IncrementClamp,
        DecrementClamp,
        IncrementWrap,
        DecrementWrap,
    }
    impl From<ffi::WGPUStencilOperation> for StencilOperation {
        fn from(value: ffi::WGPUStencilOperation) -> Self {
            match value {
                ffi::WGPUStencilOperation_WGPUStencilOperation_Undefined => {
                    StencilOperation::Undefined
                }
                ffi::WGPUStencilOperation_WGPUStencilOperation_Keep => {
                    StencilOperation::Keep
                }
                ffi::WGPUStencilOperation_WGPUStencilOperation_Zero => {
                    StencilOperation::Zero
                }
                ffi::WGPUStencilOperation_WGPUStencilOperation_Replace => {
                    StencilOperation::Replace
                }
                ffi::WGPUStencilOperation_WGPUStencilOperation_Invert => {
                    StencilOperation::Invert
                }
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
            match value {
                StencilOperation::Undefined => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_Undefined
                }
                StencilOperation::Keep => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_Keep
                }
                StencilOperation::Zero => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_Zero
                }
                StencilOperation::Replace => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_Replace
                }
                StencilOperation::Invert => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_Invert
                }
                StencilOperation::IncrementClamp => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_IncrementClamp
                }
                StencilOperation::DecrementClamp => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_DecrementClamp
                }
                StencilOperation::IncrementWrap => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_IncrementWrap
                }
                StencilOperation::DecrementWrap => {
                    ffi::WGPUStencilOperation_WGPUStencilOperation_DecrementWrap
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum StorageTextureAccess {
        BindingNotUsed,
        Undefined,
        WriteOnly,
        ReadOnly,
        ReadWrite,
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
            match value {
                StorageTextureAccess::BindingNotUsed => {
                    ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_BindingNotUsed
                }
                StorageTextureAccess::Undefined => {
                    ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_Undefined
                }
                StorageTextureAccess::WriteOnly => {
                    ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_WriteOnly
                }
                StorageTextureAccess::ReadOnly => {
                    ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadOnly
                }
                StorageTextureAccess::ReadWrite => {
                    ffi::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadWrite
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum StoreOp {
        Undefined,
        Store,
        Discard,
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
            match value {
                StoreOp::Undefined => ffi::WGPUStoreOp_WGPUStoreOp_Undefined,
                StoreOp::Store => ffi::WGPUStoreOp_WGPUStoreOp_Store,
                StoreOp::Discard => ffi::WGPUStoreOp_WGPUStoreOp_Discard,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SubgroupMatrixComponentType {
        F32,
        F16,
        U32,
        I32,
        U8,
        I8,
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
            match value {
                SubgroupMatrixComponentType::F32 => {
                    ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F32
                }
                SubgroupMatrixComponentType::F16 => {
                    ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F16
                }
                SubgroupMatrixComponentType::U32 => {
                    ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_U32
                }
                SubgroupMatrixComponentType::I32 => {
                    ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_I32
                }
                SubgroupMatrixComponentType::U8 => {
                    ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_U8
                }
                SubgroupMatrixComponentType::I8 => {
                    ffi::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_I8
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SurfaceGetCurrentTextureStatus {
        SuccessOptimal,
        SuccessSuboptimal,
        Timeout,
        Outdated,
        Lost,
        Error,
    }
    impl From<ffi::WGPUSurfaceGetCurrentTextureStatus>
    for SurfaceGetCurrentTextureStatus {
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
    impl From<SurfaceGetCurrentTextureStatus>
    for ffi::WGPUSurfaceGetCurrentTextureStatus {
        fn from(value: SurfaceGetCurrentTextureStatus) -> Self {
            match value {
                SurfaceGetCurrentTextureStatus::SuccessOptimal => {
                    ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal
                }
                SurfaceGetCurrentTextureStatus::SuccessSuboptimal => {
                    ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal
                }
                SurfaceGetCurrentTextureStatus::Timeout => {
                    ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Timeout
                }
                SurfaceGetCurrentTextureStatus::Outdated => {
                    ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Outdated
                }
                SurfaceGetCurrentTextureStatus::Lost => {
                    ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Lost
                }
                SurfaceGetCurrentTextureStatus::Error => {
                    ffi::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Error
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TexelBufferAccess {
        Undefined,
        ReadOnly,
        ReadWrite,
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
            match value {
                TexelBufferAccess::Undefined => {
                    ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_Undefined
                }
                TexelBufferAccess::ReadOnly => {
                    ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_ReadOnly
                }
                TexelBufferAccess::ReadWrite => {
                    ffi::WGPUTexelBufferAccess_WGPUTexelBufferAccess_ReadWrite
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TextureAspect {
        Undefined,
        All,
        StencilOnly,
        DepthOnly,
        Plane0Only,
        Plane1Only,
        Plane2Only,
    }
    impl From<ffi::WGPUTextureAspect> for TextureAspect {
        fn from(value: ffi::WGPUTextureAspect) -> Self {
            match value {
                ffi::WGPUTextureAspect_WGPUTextureAspect_Undefined => {
                    TextureAspect::Undefined
                }
                ffi::WGPUTextureAspect_WGPUTextureAspect_All => TextureAspect::All,
                ffi::WGPUTextureAspect_WGPUTextureAspect_StencilOnly => {
                    TextureAspect::StencilOnly
                }
                ffi::WGPUTextureAspect_WGPUTextureAspect_DepthOnly => {
                    TextureAspect::DepthOnly
                }
                ffi::WGPUTextureAspect_WGPUTextureAspect_Plane0Only => {
                    TextureAspect::Plane0Only
                }
                ffi::WGPUTextureAspect_WGPUTextureAspect_Plane1Only => {
                    TextureAspect::Plane1Only
                }
                ffi::WGPUTextureAspect_WGPUTextureAspect_Plane2Only => {
                    TextureAspect::Plane2Only
                }
                _ => TextureAspect::Undefined,
            }
        }
    }
    impl From<TextureAspect> for ffi::WGPUTextureAspect {
        fn from(value: TextureAspect) -> Self {
            match value {
                TextureAspect::Undefined => {
                    ffi::WGPUTextureAspect_WGPUTextureAspect_Undefined
                }
                TextureAspect::All => ffi::WGPUTextureAspect_WGPUTextureAspect_All,
                TextureAspect::StencilOnly => {
                    ffi::WGPUTextureAspect_WGPUTextureAspect_StencilOnly
                }
                TextureAspect::DepthOnly => {
                    ffi::WGPUTextureAspect_WGPUTextureAspect_DepthOnly
                }
                TextureAspect::Plane0Only => {
                    ffi::WGPUTextureAspect_WGPUTextureAspect_Plane0Only
                }
                TextureAspect::Plane1Only => {
                    ffi::WGPUTextureAspect_WGPUTextureAspect_Plane1Only
                }
                TextureAspect::Plane2Only => {
                    ffi::WGPUTextureAspect_WGPUTextureAspect_Plane2Only
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TextureDimension {
        Undefined,
        D1,
        D2,
        D3,
    }
    impl From<ffi::WGPUTextureDimension> for TextureDimension {
        fn from(value: ffi::WGPUTextureDimension) -> Self {
            match value {
                ffi::WGPUTextureDimension_WGPUTextureDimension_Undefined => {
                    TextureDimension::Undefined
                }
                ffi::WGPUTextureDimension_WGPUTextureDimension_1D => TextureDimension::D1,
                ffi::WGPUTextureDimension_WGPUTextureDimension_2D => TextureDimension::D2,
                ffi::WGPUTextureDimension_WGPUTextureDimension_3D => TextureDimension::D3,
                _ => TextureDimension::Undefined,
            }
        }
    }
    impl From<TextureDimension> for ffi::WGPUTextureDimension {
        fn from(value: TextureDimension) -> Self {
            match value {
                TextureDimension::Undefined => {
                    ffi::WGPUTextureDimension_WGPUTextureDimension_Undefined
                }
                TextureDimension::D1 => ffi::WGPUTextureDimension_WGPUTextureDimension_1D,
                TextureDimension::D2 => ffi::WGPUTextureDimension_WGPUTextureDimension_2D,
                TextureDimension::D3 => ffi::WGPUTextureDimension_WGPUTextureDimension_3D,
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TextureFormat {
        Undefined,
        R8Unorm,
        R8Snorm,
        R8Uint,
        R8Sint,
        R16Unorm,
        R16Snorm,
        R16Uint,
        R16Sint,
        R16Float,
        Rg8Unorm,
        Rg8Snorm,
        Rg8Uint,
        Rg8Sint,
        R32Float,
        R32Uint,
        R32Sint,
        Rg16Unorm,
        Rg16Snorm,
        Rg16Uint,
        Rg16Sint,
        Rg16Float,
        Rgba8Unorm,
        Rgba8UnormSrgb,
        Rgba8Snorm,
        Rgba8Uint,
        Rgba8Sint,
        Bgra8Unorm,
        Bgra8UnormSrgb,
        Rgb10A2Uint,
        Rgb10A2Unorm,
        Rg11B10Ufloat,
        Rgb9E5Ufloat,
        Rg32Float,
        Rg32Uint,
        Rg32Sint,
        Rgba16Unorm,
        Rgba16Snorm,
        Rgba16Uint,
        Rgba16Sint,
        Rgba16Float,
        Rgba32Float,
        Rgba32Uint,
        Rgba32Sint,
        Stencil8,
        Depth16Unorm,
        Depth24Plus,
        Depth24PlusStencil8,
        Depth32Float,
        Depth32FloatStencil8,
        Bc1RgbaUnorm,
        Bc1RgbaUnormSrgb,
        Bc2RgbaUnorm,
        Bc2RgbaUnormSrgb,
        Bc3RgbaUnorm,
        Bc3RgbaUnormSrgb,
        Bc4RUnorm,
        Bc4RSnorm,
        Bc5RgUnorm,
        Bc5RgSnorm,
        Bc6HRgbUfloat,
        Bc6HRgbFloat,
        Bc7RgbaUnorm,
        Bc7RgbaUnormSrgb,
        Etc2Rgb8Unorm,
        Etc2Rgb8UnormSrgb,
        Etc2Rgb8A1Unorm,
        Etc2Rgb8A1UnormSrgb,
        Etc2Rgba8Unorm,
        Etc2Rgba8UnormSrgb,
        EacR11Unorm,
        EacR11Snorm,
        EacRg11Unorm,
        EacRg11Snorm,
        Astc4X4Unorm,
        Astc4X4UnormSrgb,
        Astc5X4Unorm,
        Astc5X4UnormSrgb,
        Astc5X5Unorm,
        Astc5X5UnormSrgb,
        Astc6X5Unorm,
        Astc6X5UnormSrgb,
        Astc6X6Unorm,
        Astc6X6UnormSrgb,
        Astc8X5Unorm,
        Astc8X5UnormSrgb,
        Astc8X6Unorm,
        Astc8X6UnormSrgb,
        Astc8X8Unorm,
        Astc8X8UnormSrgb,
        Astc10X5Unorm,
        Astc10X5UnormSrgb,
        Astc10X6Unorm,
        Astc10X6UnormSrgb,
        Astc10X8Unorm,
        Astc10X8UnormSrgb,
        Astc10X10Unorm,
        Astc10X10UnormSrgb,
        Astc12X10Unorm,
        Astc12X10UnormSrgb,
        Astc12X12Unorm,
        Astc12X12UnormSrgb,
        R8Bg8Biplanar420Unorm,
        R10X6Bg10X6Biplanar420Unorm,
        R8Bg8A8Triplanar420Unorm,
        R8Bg8Biplanar422Unorm,
        R8Bg8Biplanar444Unorm,
        R10X6Bg10X6Biplanar422Unorm,
        R10X6Bg10X6Biplanar444Unorm,
        External,
        OpaqueYCbCrAndroid,
    }
    impl From<ffi::WGPUTextureFormat> for TextureFormat {
        fn from(value: ffi::WGPUTextureFormat) -> Self {
            match value {
                ffi::WGPUTextureFormat_WGPUTextureFormat_Undefined => {
                    TextureFormat::Undefined
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R8Unorm => {
                    TextureFormat::R8Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R8Snorm => {
                    TextureFormat::R8Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R8Uint => TextureFormat::R8Uint,
                ffi::WGPUTextureFormat_WGPUTextureFormat_R8Sint => TextureFormat::R8Sint,
                ffi::WGPUTextureFormat_WGPUTextureFormat_R16Unorm => {
                    TextureFormat::R16Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R16Snorm => {
                    TextureFormat::R16Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R16Uint => {
                    TextureFormat::R16Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R16Sint => {
                    TextureFormat::R16Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R16Float => {
                    TextureFormat::R16Float
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Unorm => {
                    TextureFormat::Rg8Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Snorm => {
                    TextureFormat::Rg8Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Uint => {
                    TextureFormat::Rg8Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Sint => {
                    TextureFormat::Rg8Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R32Float => {
                    TextureFormat::R32Float
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R32Uint => {
                    TextureFormat::R32Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_R32Sint => {
                    TextureFormat::R32Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Unorm => {
                    TextureFormat::Rg16Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Snorm => {
                    TextureFormat::Rg16Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Uint => {
                    TextureFormat::Rg16Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Sint => {
                    TextureFormat::Rg16Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Float => {
                    TextureFormat::Rg16Float
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Unorm => {
                    TextureFormat::Rgba8Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8UnormSrgb => {
                    TextureFormat::Rgba8UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Snorm => {
                    TextureFormat::Rgba8Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Uint => {
                    TextureFormat::Rgba8Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Sint => {
                    TextureFormat::Rgba8Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8Unorm => {
                    TextureFormat::Bgra8Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8UnormSrgb => {
                    TextureFormat::Bgra8UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Uint => {
                    TextureFormat::Rgb10A2Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Unorm => {
                    TextureFormat::Rgb10A2Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG11B10Ufloat => {
                    TextureFormat::Rg11B10Ufloat
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGB9E5Ufloat => {
                    TextureFormat::Rgb9E5Ufloat
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Float => {
                    TextureFormat::Rg32Float
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Uint => {
                    TextureFormat::Rg32Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Sint => {
                    TextureFormat::Rg32Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Unorm => {
                    TextureFormat::Rgba16Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Snorm => {
                    TextureFormat::Rgba16Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Uint => {
                    TextureFormat::Rgba16Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Sint => {
                    TextureFormat::Rgba16Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Float => {
                    TextureFormat::Rgba16Float
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Float => {
                    TextureFormat::Rgba32Float
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Uint => {
                    TextureFormat::Rgba32Uint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Sint => {
                    TextureFormat::Rgba32Sint
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_Stencil8 => {
                    TextureFormat::Stencil8
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_Depth16Unorm => {
                    TextureFormat::Depth16Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24Plus => {
                    TextureFormat::Depth24Plus
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24PlusStencil8 => {
                    TextureFormat::Depth24PlusStencil8
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32Float => {
                    TextureFormat::Depth32Float
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32FloatStencil8 => {
                    TextureFormat::Depth32FloatStencil8
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnorm => {
                    TextureFormat::Bc1RgbaUnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnormSrgb => {
                    TextureFormat::Bc1RgbaUnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnorm => {
                    TextureFormat::Bc2RgbaUnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnormSrgb => {
                    TextureFormat::Bc2RgbaUnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnorm => {
                    TextureFormat::Bc3RgbaUnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnormSrgb => {
                    TextureFormat::Bc3RgbaUnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RUnorm => {
                    TextureFormat::Bc4RUnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RSnorm => {
                    TextureFormat::Bc4RSnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGUnorm => {
                    TextureFormat::Bc5RgUnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGSnorm => {
                    TextureFormat::Bc5RgSnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBUfloat => {
                    TextureFormat::Bc6HRgbUfloat
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBFloat => {
                    TextureFormat::Bc6HRgbFloat
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnorm => {
                    TextureFormat::Bc7RgbaUnorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnormSrgb => {
                    TextureFormat::Bc7RgbaUnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8Unorm => {
                    TextureFormat::Etc2Rgb8Unorm
                }
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
                ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Unorm => {
                    TextureFormat::EacR11Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Snorm => {
                    TextureFormat::EacR11Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Unorm => {
                    TextureFormat::EacRg11Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Snorm => {
                    TextureFormat::EacRg11Snorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4Unorm => {
                    TextureFormat::Astc4X4Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4UnormSrgb => {
                    TextureFormat::Astc4X4UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4Unorm => {
                    TextureFormat::Astc5X4Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4UnormSrgb => {
                    TextureFormat::Astc5X4UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5Unorm => {
                    TextureFormat::Astc5X5Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5UnormSrgb => {
                    TextureFormat::Astc5X5UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5Unorm => {
                    TextureFormat::Astc6X5Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5UnormSrgb => {
                    TextureFormat::Astc6X5UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6Unorm => {
                    TextureFormat::Astc6X6Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6UnormSrgb => {
                    TextureFormat::Astc6X6UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5Unorm => {
                    TextureFormat::Astc8X5Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5UnormSrgb => {
                    TextureFormat::Astc8X5UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6Unorm => {
                    TextureFormat::Astc8X6Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6UnormSrgb => {
                    TextureFormat::Astc8X6UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8Unorm => {
                    TextureFormat::Astc8X8Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8UnormSrgb => {
                    TextureFormat::Astc8X8UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5Unorm => {
                    TextureFormat::Astc10X5Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5UnormSrgb => {
                    TextureFormat::Astc10X5UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6Unorm => {
                    TextureFormat::Astc10X6Unorm
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6UnormSrgb => {
                    TextureFormat::Astc10X6UnormSrgb
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8Unorm => {
                    TextureFormat::Astc10X8Unorm
                }
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
                ffi::WGPUTextureFormat_WGPUTextureFormat_External => {
                    TextureFormat::External
                }
                ffi::WGPUTextureFormat_WGPUTextureFormat_OpaqueYCbCrAndroid => {
                    TextureFormat::OpaqueYCbCrAndroid
                }
                _ => TextureFormat::Undefined,
            }
        }
    }
    impl From<TextureFormat> for ffi::WGPUTextureFormat {
        fn from(value: TextureFormat) -> Self {
            match value {
                TextureFormat::Undefined => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_Undefined
                }
                TextureFormat::R8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R8Unorm
                }
                TextureFormat::R8Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R8Snorm
                }
                TextureFormat::R8Uint => ffi::WGPUTextureFormat_WGPUTextureFormat_R8Uint,
                TextureFormat::R8Sint => ffi::WGPUTextureFormat_WGPUTextureFormat_R8Sint,
                TextureFormat::R16Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R16Unorm
                }
                TextureFormat::R16Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R16Snorm
                }
                TextureFormat::R16Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R16Uint
                }
                TextureFormat::R16Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R16Sint
                }
                TextureFormat::R16Float => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R16Float
                }
                TextureFormat::Rg8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Unorm
                }
                TextureFormat::Rg8Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Snorm
                }
                TextureFormat::Rg8Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Uint
                }
                TextureFormat::Rg8Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG8Sint
                }
                TextureFormat::R32Float => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R32Float
                }
                TextureFormat::R32Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R32Uint
                }
                TextureFormat::R32Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R32Sint
                }
                TextureFormat::Rg16Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Unorm
                }
                TextureFormat::Rg16Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Snorm
                }
                TextureFormat::Rg16Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Uint
                }
                TextureFormat::Rg16Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Sint
                }
                TextureFormat::Rg16Float => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG16Float
                }
                TextureFormat::Rgba8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Unorm
                }
                TextureFormat::Rgba8UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8UnormSrgb
                }
                TextureFormat::Rgba8Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Snorm
                }
                TextureFormat::Rgba8Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Uint
                }
                TextureFormat::Rgba8Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA8Sint
                }
                TextureFormat::Bgra8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8Unorm
                }
                TextureFormat::Bgra8UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BGRA8UnormSrgb
                }
                TextureFormat::Rgb10A2Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Uint
                }
                TextureFormat::Rgb10A2Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Unorm
                }
                TextureFormat::Rg11B10Ufloat => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG11B10Ufloat
                }
                TextureFormat::Rgb9E5Ufloat => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGB9E5Ufloat
                }
                TextureFormat::Rg32Float => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Float
                }
                TextureFormat::Rg32Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Uint
                }
                TextureFormat::Rg32Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RG32Sint
                }
                TextureFormat::Rgba16Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Unorm
                }
                TextureFormat::Rgba16Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Snorm
                }
                TextureFormat::Rgba16Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Uint
                }
                TextureFormat::Rgba16Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Sint
                }
                TextureFormat::Rgba16Float => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA16Float
                }
                TextureFormat::Rgba32Float => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Float
                }
                TextureFormat::Rgba32Uint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Uint
                }
                TextureFormat::Rgba32Sint => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_RGBA32Sint
                }
                TextureFormat::Stencil8 => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_Stencil8
                }
                TextureFormat::Depth16Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_Depth16Unorm
                }
                TextureFormat::Depth24Plus => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24Plus
                }
                TextureFormat::Depth24PlusStencil8 => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_Depth24PlusStencil8
                }
                TextureFormat::Depth32Float => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32Float
                }
                TextureFormat::Depth32FloatStencil8 => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_Depth32FloatStencil8
                }
                TextureFormat::Bc1RgbaUnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnorm
                }
                TextureFormat::Bc1RgbaUnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnormSrgb
                }
                TextureFormat::Bc2RgbaUnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnorm
                }
                TextureFormat::Bc2RgbaUnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnormSrgb
                }
                TextureFormat::Bc3RgbaUnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnorm
                }
                TextureFormat::Bc3RgbaUnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnormSrgb
                }
                TextureFormat::Bc4RUnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RUnorm
                }
                TextureFormat::Bc4RSnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC4RSnorm
                }
                TextureFormat::Bc5RgUnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGUnorm
                }
                TextureFormat::Bc5RgSnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC5RGSnorm
                }
                TextureFormat::Bc6HRgbUfloat => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBUfloat
                }
                TextureFormat::Bc6HRgbFloat => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBFloat
                }
                TextureFormat::Bc7RgbaUnorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnorm
                }
                TextureFormat::Bc7RgbaUnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnormSrgb
                }
                TextureFormat::Etc2Rgb8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8Unorm
                }
                TextureFormat::Etc2Rgb8UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8UnormSrgb
                }
                TextureFormat::Etc2Rgb8A1Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1Unorm
                }
                TextureFormat::Etc2Rgb8A1UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1UnormSrgb
                }
                TextureFormat::Etc2Rgba8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8Unorm
                }
                TextureFormat::Etc2Rgba8UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8UnormSrgb
                }
                TextureFormat::EacR11Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Unorm
                }
                TextureFormat::EacR11Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_EACR11Snorm
                }
                TextureFormat::EacRg11Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Unorm
                }
                TextureFormat::EacRg11Snorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_EACRG11Snorm
                }
                TextureFormat::Astc4X4Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4Unorm
                }
                TextureFormat::Astc4X4UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4UnormSrgb
                }
                TextureFormat::Astc5X4Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4Unorm
                }
                TextureFormat::Astc5X4UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4UnormSrgb
                }
                TextureFormat::Astc5X5Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5Unorm
                }
                TextureFormat::Astc5X5UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5UnormSrgb
                }
                TextureFormat::Astc6X5Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5Unorm
                }
                TextureFormat::Astc6X5UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5UnormSrgb
                }
                TextureFormat::Astc6X6Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6Unorm
                }
                TextureFormat::Astc6X6UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6UnormSrgb
                }
                TextureFormat::Astc8X5Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5Unorm
                }
                TextureFormat::Astc8X5UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5UnormSrgb
                }
                TextureFormat::Astc8X6Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6Unorm
                }
                TextureFormat::Astc8X6UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6UnormSrgb
                }
                TextureFormat::Astc8X8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8Unorm
                }
                TextureFormat::Astc8X8UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8UnormSrgb
                }
                TextureFormat::Astc10X5Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5Unorm
                }
                TextureFormat::Astc10X5UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5UnormSrgb
                }
                TextureFormat::Astc10X6Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6Unorm
                }
                TextureFormat::Astc10X6UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6UnormSrgb
                }
                TextureFormat::Astc10X8Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8Unorm
                }
                TextureFormat::Astc10X8UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8UnormSrgb
                }
                TextureFormat::Astc10X10Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10Unorm
                }
                TextureFormat::Astc10X10UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10UnormSrgb
                }
                TextureFormat::Astc12X10Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10Unorm
                }
                TextureFormat::Astc12X10UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10UnormSrgb
                }
                TextureFormat::Astc12X12Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12Unorm
                }
                TextureFormat::Astc12X12UnormSrgb => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12UnormSrgb
                }
                TextureFormat::R8Bg8Biplanar420Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar420Unorm
                }
                TextureFormat::R10X6Bg10X6Biplanar420Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar420Unorm
                }
                TextureFormat::R8Bg8A8Triplanar420Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8A8Triplanar420Unorm
                }
                TextureFormat::R8Bg8Biplanar422Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar422Unorm
                }
                TextureFormat::R8Bg8Biplanar444Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar444Unorm
                }
                TextureFormat::R10X6Bg10X6Biplanar422Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar422Unorm
                }
                TextureFormat::R10X6Bg10X6Biplanar444Unorm => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar444Unorm
                }
                TextureFormat::External => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_External
                }
                TextureFormat::OpaqueYCbCrAndroid => {
                    ffi::WGPUTextureFormat_WGPUTextureFormat_OpaqueYCbCrAndroid
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TextureSampleType {
        BindingNotUsed,
        Undefined,
        Float,
        UnfilterableFloat,
        Depth,
        Sint,
        Uint,
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
                ffi::WGPUTextureSampleType_WGPUTextureSampleType_Float => {
                    TextureSampleType::Float
                }
                ffi::WGPUTextureSampleType_WGPUTextureSampleType_UnfilterableFloat => {
                    TextureSampleType::UnfilterableFloat
                }
                ffi::WGPUTextureSampleType_WGPUTextureSampleType_Depth => {
                    TextureSampleType::Depth
                }
                ffi::WGPUTextureSampleType_WGPUTextureSampleType_Sint => {
                    TextureSampleType::Sint
                }
                ffi::WGPUTextureSampleType_WGPUTextureSampleType_Uint => {
                    TextureSampleType::Uint
                }
                _ => TextureSampleType::BindingNotUsed,
            }
        }
    }
    impl From<TextureSampleType> for ffi::WGPUTextureSampleType {
        fn from(value: TextureSampleType) -> Self {
            match value {
                TextureSampleType::BindingNotUsed => {
                    ffi::WGPUTextureSampleType_WGPUTextureSampleType_BindingNotUsed
                }
                TextureSampleType::Undefined => {
                    ffi::WGPUTextureSampleType_WGPUTextureSampleType_Undefined
                }
                TextureSampleType::Float => {
                    ffi::WGPUTextureSampleType_WGPUTextureSampleType_Float
                }
                TextureSampleType::UnfilterableFloat => {
                    ffi::WGPUTextureSampleType_WGPUTextureSampleType_UnfilterableFloat
                }
                TextureSampleType::Depth => {
                    ffi::WGPUTextureSampleType_WGPUTextureSampleType_Depth
                }
                TextureSampleType::Sint => {
                    ffi::WGPUTextureSampleType_WGPUTextureSampleType_Sint
                }
                TextureSampleType::Uint => {
                    ffi::WGPUTextureSampleType_WGPUTextureSampleType_Uint
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TextureViewDimension {
        Undefined,
        D1,
        D2,
        D2Array,
        Cube,
        CubeArray,
        D3,
    }
    impl From<ffi::WGPUTextureViewDimension> for TextureViewDimension {
        fn from(value: ffi::WGPUTextureViewDimension) -> Self {
            match value {
                ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Undefined => {
                    TextureViewDimension::Undefined
                }
                ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_1D => {
                    TextureViewDimension::D1
                }
                ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2D => {
                    TextureViewDimension::D2
                }
                ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2DArray => {
                    TextureViewDimension::D2Array
                }
                ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Cube => {
                    TextureViewDimension::Cube
                }
                ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_CubeArray => {
                    TextureViewDimension::CubeArray
                }
                ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_3D => {
                    TextureViewDimension::D3
                }
                _ => TextureViewDimension::Undefined,
            }
        }
    }
    impl From<TextureViewDimension> for ffi::WGPUTextureViewDimension {
        fn from(value: TextureViewDimension) -> Self {
            match value {
                TextureViewDimension::Undefined => {
                    ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Undefined
                }
                TextureViewDimension::D1 => {
                    ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_1D
                }
                TextureViewDimension::D2 => {
                    ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2D
                }
                TextureViewDimension::D2Array => {
                    ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_2DArray
                }
                TextureViewDimension::Cube => {
                    ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_Cube
                }
                TextureViewDimension::CubeArray => {
                    ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_CubeArray
                }
                TextureViewDimension::D3 => {
                    ffi::WGPUTextureViewDimension_WGPUTextureViewDimension_3D
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ToneMappingMode {
        Standard,
        Extended,
    }
    impl From<ffi::WGPUToneMappingMode> for ToneMappingMode {
        fn from(value: ffi::WGPUToneMappingMode) -> Self {
            match value {
                ffi::WGPUToneMappingMode_WGPUToneMappingMode_Standard => {
                    ToneMappingMode::Standard
                }
                ffi::WGPUToneMappingMode_WGPUToneMappingMode_Extended => {
                    ToneMappingMode::Extended
                }
                _ => ToneMappingMode::Standard,
            }
        }
    }
    impl From<ToneMappingMode> for ffi::WGPUToneMappingMode {
        fn from(value: ToneMappingMode) -> Self {
            match value {
                ToneMappingMode::Standard => {
                    ffi::WGPUToneMappingMode_WGPUToneMappingMode_Standard
                }
                ToneMappingMode::Extended => {
                    ffi::WGPUToneMappingMode_WGPUToneMappingMode_Extended
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum VertexFormat {
        Uint8,
        Uint8X2,
        Uint8X4,
        Sint8,
        Sint8X2,
        Sint8X4,
        Unorm8,
        Unorm8X2,
        Unorm8X4,
        Snorm8,
        Snorm8X2,
        Snorm8X4,
        Uint16,
        Uint16X2,
        Uint16X4,
        Sint16,
        Sint16X2,
        Sint16X4,
        Unorm16,
        Unorm16X2,
        Unorm16X4,
        Snorm16,
        Snorm16X2,
        Snorm16X4,
        Float16,
        Float16X2,
        Float16X4,
        Float32,
        Float32X2,
        Float32X3,
        Float32X4,
        Uint32,
        Uint32X2,
        Uint32X3,
        Uint32X4,
        Sint32,
        Sint32X2,
        Sint32X3,
        Sint32X4,
        Unorm1010102,
        Unorm8X4Bgra,
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
                ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x2 => {
                    VertexFormat::Unorm16X2
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x4 => {
                    VertexFormat::Unorm16X4
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16 => VertexFormat::Snorm16,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x2 => {
                    VertexFormat::Snorm16X2
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x4 => {
                    VertexFormat::Snorm16X4
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Float16 => VertexFormat::Float16,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x2 => {
                    VertexFormat::Float16X2
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x4 => {
                    VertexFormat::Float16X4
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Float32 => VertexFormat::Float32,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x2 => {
                    VertexFormat::Float32X2
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x3 => {
                    VertexFormat::Float32X3
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x4 => {
                    VertexFormat::Float32X4
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32 => VertexFormat::Uint32,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x2 => VertexFormat::Uint32X2,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x3 => VertexFormat::Uint32X3,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x4 => VertexFormat::Uint32X4,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32 => VertexFormat::Sint32,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x2 => VertexFormat::Sint32X2,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x3 => VertexFormat::Sint32X3,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x4 => VertexFormat::Sint32X4,
                ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm10_10_10_2 => {
                    VertexFormat::Unorm1010102
                }
                ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4BGRA => {
                    VertexFormat::Unorm8X4Bgra
                }
                _ => VertexFormat::Uint8,
            }
        }
    }
    impl From<VertexFormat> for ffi::WGPUVertexFormat {
        fn from(value: VertexFormat) -> Self {
            match value {
                VertexFormat::Uint8 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8,
                VertexFormat::Uint8X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8x2,
                VertexFormat::Uint8X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint8x4,
                VertexFormat::Sint8 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8,
                VertexFormat::Sint8X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8x2,
                VertexFormat::Sint8X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint8x4,
                VertexFormat::Unorm8 => ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8,
                VertexFormat::Unorm8X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x2,
                VertexFormat::Unorm8X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4,
                VertexFormat::Snorm8 => ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8,
                VertexFormat::Snorm8X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8x2,
                VertexFormat::Snorm8X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm8x4,
                VertexFormat::Uint16 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16,
                VertexFormat::Uint16X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16x2,
                VertexFormat::Uint16X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint16x4,
                VertexFormat::Sint16 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16,
                VertexFormat::Sint16X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16x2,
                VertexFormat::Sint16X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint16x4,
                VertexFormat::Unorm16 => ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16,
                VertexFormat::Unorm16X2 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x2
                }
                VertexFormat::Unorm16X4 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm16x4
                }
                VertexFormat::Snorm16 => ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16,
                VertexFormat::Snorm16X2 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x2
                }
                VertexFormat::Snorm16X4 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Snorm16x4
                }
                VertexFormat::Float16 => ffi::WGPUVertexFormat_WGPUVertexFormat_Float16,
                VertexFormat::Float16X2 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x2
                }
                VertexFormat::Float16X4 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Float16x4
                }
                VertexFormat::Float32 => ffi::WGPUVertexFormat_WGPUVertexFormat_Float32,
                VertexFormat::Float32X2 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x2
                }
                VertexFormat::Float32X3 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x3
                }
                VertexFormat::Float32X4 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Float32x4
                }
                VertexFormat::Uint32 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32,
                VertexFormat::Uint32X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x2,
                VertexFormat::Uint32X3 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x3,
                VertexFormat::Uint32X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Uint32x4,
                VertexFormat::Sint32 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32,
                VertexFormat::Sint32X2 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x2,
                VertexFormat::Sint32X3 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x3,
                VertexFormat::Sint32X4 => ffi::WGPUVertexFormat_WGPUVertexFormat_Sint32x4,
                VertexFormat::Unorm1010102 => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm10_10_10_2
                }
                VertexFormat::Unorm8X4Bgra => {
                    ffi::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4BGRA
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum VertexStepMode {
        Undefined,
        Vertex,
        Instance,
    }
    impl From<ffi::WGPUVertexStepMode> for VertexStepMode {
        fn from(value: ffi::WGPUVertexStepMode) -> Self {
            match value {
                ffi::WGPUVertexStepMode_WGPUVertexStepMode_Undefined => {
                    VertexStepMode::Undefined
                }
                ffi::WGPUVertexStepMode_WGPUVertexStepMode_Vertex => {
                    VertexStepMode::Vertex
                }
                ffi::WGPUVertexStepMode_WGPUVertexStepMode_Instance => {
                    VertexStepMode::Instance
                }
                _ => VertexStepMode::Undefined,
            }
        }
    }
    impl From<VertexStepMode> for ffi::WGPUVertexStepMode {
        fn from(value: VertexStepMode) -> Self {
            match value {
                VertexStepMode::Undefined => {
                    ffi::WGPUVertexStepMode_WGPUVertexStepMode_Undefined
                }
                VertexStepMode::Vertex => {
                    ffi::WGPUVertexStepMode_WGPUVertexStepMode_Vertex
                }
                VertexStepMode::Instance => {
                    ffi::WGPUVertexStepMode_WGPUVertexStepMode_Instance
                }
            }
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum WaitStatus {
        Success,
        TimedOut,
        Error,
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
            match value {
                WaitStatus::Success => ffi::WGPUWaitStatus_WGPUWaitStatus_Success,
                WaitStatus::TimedOut => ffi::WGPUWaitStatus_WGPUWaitStatus_TimedOut,
                WaitStatus::Error => ffi::WGPUWaitStatus_WGPUWaitStatus_Error,
            }
        }
    }
    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct BufferUsage : u64 {
        const NONE = ffi::WGPUBufferUsage_None as u64; const MAP_READ =
        ffi::WGPUBufferUsage_MapRead as u64; const MAP_WRITE =
        ffi::WGPUBufferUsage_MapWrite as u64; const COPY_SRC =
        ffi::WGPUBufferUsage_CopySrc as u64; const COPY_DST =
        ffi::WGPUBufferUsage_CopyDst as u64; const INDEX = ffi::WGPUBufferUsage_Index as
        u64; const VERTEX = ffi::WGPUBufferUsage_Vertex as u64; const UNIFORM =
        ffi::WGPUBufferUsage_Uniform as u64; const STORAGE = ffi::WGPUBufferUsage_Storage
        as u64; const INDIRECT = ffi::WGPUBufferUsage_Indirect as u64; const
        QUERY_RESOLVE = ffi::WGPUBufferUsage_QueryResolve as u64; const TEXEL_BUFFER =
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct ColorWriteMask :
        u64 { const NONE = ffi::WGPUColorWriteMask_None as u64; const RED =
        ffi::WGPUColorWriteMask_Red as u64; const GREEN = ffi::WGPUColorWriteMask_Green
        as u64; const BLUE = ffi::WGPUColorWriteMask_Blue as u64; const ALPHA =
        ffi::WGPUColorWriteMask_Alpha as u64; const ALL = ffi::WGPUColorWriteMask_All as
        u64; }
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct HeapProperty : u64
        { const NONE = ffi::WGPUHeapProperty_None as u64; const DEVICE_LOCAL =
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct MapMode : u64 {
        const NONE = ffi::WGPUMapMode_None as u64; const READ = ffi::WGPUMapMode_Read as
        u64; const WRITE = ffi::WGPUMapMode_Write as u64; }
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
        ffi::WGPUShaderStage_Vertex as u64; const FRAGMENT =
        ffi::WGPUShaderStage_Fragment as u64; const COMPUTE =
        ffi::WGPUShaderStage_Compute as u64; }
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct TextureUsage : u64
        { const NONE = ffi::WGPUTextureUsage_None as u64; const COPY_SRC =
        ffi::WGPUTextureUsage_CopySrc as u64; const COPY_DST =
        ffi::WGPUTextureUsage_CopyDst as u64; const TEXTURE_BINDING =
        ffi::WGPUTextureUsage_TextureBinding as u64; const STORAGE_BINDING =
        ffi::WGPUTextureUsage_StorageBinding as u64; const RENDER_ATTACHMENT =
        ffi::WGPUTextureUsage_RenderAttachment as u64; const TRANSIENT_ATTACHMENT =
        ffi::WGPUTextureUsage_TransientAttachment as u64; const STORAGE_ATTACHMENT =
        ffi::WGPUTextureUsage_StorageAttachment as u64; }
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
}
mod structs {
    #![allow(dead_code, unused_imports)]
    use crate::ffi;
    use crate::generated::*;
    use std::any::Any;
    use std::ffi::CStr;
    pub(crate) fn string_view_to_string(view: ffi::WGPUStringView) -> String {
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
            let mut raw: ffi::WGPUAHardwareBufferProperties = unsafe {
                std::mem::zeroed()
            };
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
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUAdapterInfo>,
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
                _free_members: None,
            }
        }
    }
    impl AdapterInfo {
        pub fn new() -> Self {
            Self::default()
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUAdapterInfo) {
            unsafe { ffi::wgpuAdapterInfoFreeMembers(value) };
        }
    }
    impl Drop for AdapterInfo {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuAdapterInfoFreeMembers(value) };
            }
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
        pub max_compute_workgroup_subgroups: Option<u32>,
    }
    impl Default for AdapterPropertiesExplicitComputeSubgroupSizeConfigs {
        fn default() -> Self {
            Self {
                min_explicit_compute_subgroup_size: None,
                max_explicit_compute_subgroup_size: None,
                max_compute_workgroup_subgroups: None,
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
            if let Some(value) = self.max_compute_workgroup_subgroups {
                raw.maxComputeWorkgroupSubgroups = value;
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
                max_compute_workgroup_subgroups: Some(value.maxComputeWorkgroupSubgroups),
            }
        }
    }
    pub struct AdapterPropertiesMemoryHeaps {
        pub heap_info: Option<Vec<MemoryHeapInfo>>,
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUAdapterPropertiesMemoryHeaps>,
    }
    impl Default for AdapterPropertiesMemoryHeaps {
        fn default() -> Self {
            Self {
                heap_info: None,
                _free_members: None,
            }
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUAdapterPropertiesMemoryHeaps) {
            unsafe { ffi::wgpuAdapterPropertiesMemoryHeapsFreeMembers(value) };
        }
    }
    impl Drop for AdapterPropertiesMemoryHeaps {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuAdapterPropertiesMemoryHeapsFreeMembers(value) };
            }
        }
    }
    pub struct AdapterPropertiesSubgroupMatrixConfigs {
        pub configs: Option<Vec<SubgroupMatrixConfig>>,
        #[doc(hidden)]
        pub(crate) _free_members: Option<
            ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs,
        >,
    }
    impl Default for AdapterPropertiesSubgroupMatrixConfigs {
        fn default() -> Self {
            Self {
                configs: None,
                _free_members: None,
            }
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(
            value: ffi::WGPUAdapterPropertiesSubgroupMatrixConfigs,
        ) {
            unsafe { ffi::wgpuAdapterPropertiesSubgroupMatrixConfigsFreeMembers(value) };
        }
    }
    impl Drop for AdapterPropertiesSubgroupMatrixConfigs {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe {
                    ffi::wgpuAdapterPropertiesSubgroupMatrixConfigsFreeMembers(value)
                };
            }
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUAdapterPropertiesVk, ChainedStructStorage) {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUBindGroupDescriptor, ChainedStructStorage) {
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
        pub fn with_extension(
            mut self,
            extension: BindGroupDescriptorExtension,
        ) -> Self {
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
                size: Some(WHOLE_SIZE),
                sampler: None,
                texture_view: None,
            }
        }
    }
    impl BindGroupEntry {
        pub fn new() -> Self {
            Self::default()
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUBindGroupLayoutDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUBindGroupLayoutDescriptor = unsafe {
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
        pub fn with_extension(
            mut self,
            extension: BindGroupLayoutEntryExtension,
        ) -> Self {
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
                size: Some(WHOLE_SIZE),
                sampler: None,
                texture_view: None,
            }
        }
    }
    impl BindingResource {
        pub fn new() -> Self {
            Self::default()
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUBufferBindingLayout, ChainedStructStorage) {
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
        pub fn with_extension(
            mut self,
            extension: BufferBindingLayoutExtension,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUBufferDescriptor, ChainedStructStorage) {
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
            let mut raw: ffi::WGPUBufferHostMappedPointer = unsafe {
                std::mem::zeroed()
            };
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUColorTargetState, ChainedStructStorage) {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUCommandBufferDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUCommandBufferDescriptor = unsafe {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUCommandEncoderDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUCommandEncoderDescriptor = unsafe {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUCompilationMessage, ChainedStructStorage) {
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
        pub fn with_extension(
            mut self,
            extension: ComputePassDescriptorExtension,
        ) -> Self {
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
                    Some(
                        PassTimestampWrites::from_ffi(unsafe { *value.timestampWrites }),
                    )
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUComputePipelineDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUComputePipelineDescriptor = unsafe {
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
                        unsafe {
                            std::slice::from_raw_parts(value.conversionMatrix, 9usize)
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUDawnWGSLBlocklist, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut raw: ffi::WGPUDawnWGSLBlocklist = unsafe { std::mem::zeroed() };
            raw.blocklistedFeatureCount = self
                .blocklisted_features
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);
            if let Some(values) = &self.blocklisted_features {
                let len_value = values.len();
                let mut c_strings: Vec<std::ffi::CString> = Vec::with_capacity(
                    values.len(),
                );
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
        ) -> (
            ffi::WGPUDawnBufferDescriptorErrorInfoFromWireClient,
            ChainedStructStorage,
        ) {
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
            let mut raw: ffi::WGPUDawnCacheDeviceDescriptor = unsafe {
                std::mem::zeroed()
            };
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
            raw.consumeAdapter = if self.consume_adapter.unwrap_or(false) {
                1
            } else {
                0
            };
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
            let mut raw: ffi::WGPUDawnDeviceAllocatorControl = unsafe {
                std::mem::zeroed()
            };
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
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUDawnDrmFormatCapabilities>,
    }
    impl Default for DawnDrmFormatCapabilities {
        fn default() -> Self {
            Self {
                properties: None,
                _free_members: None,
            }
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
            let mut raw: ffi::WGPUDawnDrmFormatCapabilities = unsafe {
                std::mem::zeroed()
            };
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUDawnDrmFormatCapabilities) {
            unsafe { ffi::wgpuDawnDrmFormatCapabilitiesFreeMembers(value) };
        }
    }
    impl Drop for DawnDrmFormatCapabilities {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuDawnDrmFormatCapabilitiesFreeMembers(value) };
            }
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
            let mut raw: ffi::WGPUDawnDrmFormatProperties = unsafe {
                std::mem::zeroed()
            };
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUDawnEncoderInternalUsageDescriptor,
        ) -> Self {
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
            raw.fakeOOMAtWireClientMap = if self
                .fake_oom_at_wire_client_map
                .unwrap_or(false)
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
        pub fn with_extension(
            mut self,
            extension: DawnFormatCapabilitiesExtension,
        ) -> Self {
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
                host_mapped_pointer_alignment: Some(LIMIT_U32_UNDEFINED),
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
            let mut raw: ffi::WGPUDawnInjectedInvalidSType = unsafe {
                std::mem::zeroed()
            };
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
    pub struct DawnRenderPassSampleCount {
        pub sample_count: Option<u32>,
    }
    impl Default for DawnRenderPassSampleCount {
        fn default() -> Self {
            Self { sample_count: Some(1) }
        }
    }
    impl DawnRenderPassSampleCount {
        pub fn new() -> Self {
            Self::default()
        }
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUDawnRenderPassSampleCount, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut raw: ffi::WGPUDawnRenderPassSampleCount = unsafe {
                std::mem::zeroed()
            };
            if let Some(value) = self.sample_count {
                raw.sampleCount = value;
            }
            (raw, storage)
        }
        pub(crate) fn from_ffi(value: ffi::WGPUDawnRenderPassSampleCount) -> Self {
            Self {
                sample_count: Some(value.sampleCount),
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
                allow_non_uniform_derivatives: Some(
                    value.allowNonUniformDerivatives != 0,
                ),
            }
        }
    }
    pub struct DawnTexelCopyBufferRowAlignmentLimits {
        pub min_texel_copy_buffer_row_alignment: Option<u32>,
    }
    impl Default for DawnTexelCopyBufferRowAlignmentLimits {
        fn default() -> Self {
            Self {
                min_texel_copy_buffer_row_alignment: Some(LIMIT_U32_UNDEFINED),
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUDawnTextureInternalUsageDescriptor,
        ) -> Self {
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
                let mut c_strings: Vec<std::ffi::CString> = Vec::with_capacity(
                    values.len(),
                );
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
                let mut c_strings: Vec<std::ffi::CString> = Vec::with_capacity(
                    values.len(),
                );
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUDawnWireWGSLControl, ChainedStructStorage) {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUDepthStencilState, ChainedStructStorage) {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUDeviceDescriptor, ChainedStructStorage) {
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
            if let Some(info) = &self.device_lost_callback_info {
                let mut callback_slot = info.callback.borrow_mut();
                let callback = callback_slot.take();
                let (
                    callback_ptr,
                    userdata1,
                ): (ffi::WGPUDeviceLostCallback, *mut std::ffi::c_void) = if let Some(
                    callback,
                ) = callback {
                    let callback_box: DeviceLostCallback = callback;
                    let callback_box = Box::new(Some(callback_box));
                    let userdata = Box::into_raw(callback_box)
                        .cast::<std::ffi::c_void>();
                    (Some(device_lost_callback_trampoline), userdata)
                } else {
                    (None, std::ptr::null_mut())
                };
                let mode = info.mode.unwrap_or(CallbackMode::AllowSpontaneous);
                raw.deviceLostCallbackInfo = ffi::WGPUDeviceLostCallbackInfo {
                    nextInChain: std::ptr::null_mut(),
                    mode: mode.into(),
                    callback: callback_ptr,
                    userdata1,
                    userdata2: std::ptr::null_mut(),
                };
            } else {
                raw.deviceLostCallbackInfo = ffi::WGPUDeviceLostCallbackInfo {
                    nextInChain: std::ptr::null_mut(),
                    mode: CallbackMode::AllowSpontaneous.into(),
                    callback: None,
                    userdata1: std::ptr::null_mut(),
                    userdata2: std::ptr::null_mut(),
                };
            }
            if let Some(info) = &self.uncaptured_error_callback_info {
                let mut callback_slot = info.callback.borrow_mut();
                let callback = callback_slot.take();
                let (
                    callback_ptr,
                    userdata1,
                ): (ffi::WGPUUncapturedErrorCallback, *mut std::ffi::c_void) = if let Some(
                    callback,
                ) = callback {
                    let callback_box: UncapturedErrorCallback = callback;
                    let callback_box = Box::new(Some(callback_box));
                    let userdata = Box::into_raw(callback_box)
                        .cast::<std::ffi::c_void>();
                    (Some(uncaptured_error_callback_trampoline), userdata)
                } else {
                    (None, std::ptr::null_mut())
                };
                raw.uncapturedErrorCallbackInfo = ffi::WGPUUncapturedErrorCallbackInfo {
                    nextInChain: std::ptr::null_mut(),
                    callback: callback_ptr,
                    userdata1,
                    userdata2: std::ptr::null_mut(),
                };
            } else {
                raw.uncapturedErrorCallbackInfo = ffi::WGPUUncapturedErrorCallbackInfo {
                    nextInChain: std::ptr::null_mut(),
                    callback: None,
                    userdata1: std::ptr::null_mut(),
                    userdata2: std::ptr::null_mut(),
                };
            }
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUExternalTextureDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUExternalTextureDescriptor = unsafe {
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
                yuv_to_rgb_conversion_matrix: if value.yuvToRgbConversionMatrix.is_null()
                {
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
                            std::slice::from_raw_parts(
                                value.gamutConversionMatrix,
                                9usize,
                            )
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUImageCopyExternalTexture, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUImageCopyExternalTexture = unsafe {
                std::mem::zeroed()
            };
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUInstanceDescriptor, ChainedStructStorage) {
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
                max_texture_dimension_1d: Some(LIMIT_U32_UNDEFINED),
                max_texture_dimension_2d: Some(LIMIT_U32_UNDEFINED),
                max_texture_dimension_3d: Some(LIMIT_U32_UNDEFINED),
                max_texture_array_layers: Some(LIMIT_U32_UNDEFINED),
                max_bind_groups: Some(LIMIT_U32_UNDEFINED),
                max_bind_groups_plus_vertex_buffers: Some(LIMIT_U32_UNDEFINED),
                max_bindings_per_bind_group: Some(LIMIT_U32_UNDEFINED),
                max_dynamic_uniform_buffers_per_pipeline_layout: Some(
                    LIMIT_U32_UNDEFINED,
                ),
                max_dynamic_storage_buffers_per_pipeline_layout: Some(
                    LIMIT_U32_UNDEFINED,
                ),
                max_sampled_textures_per_shader_stage: Some(LIMIT_U32_UNDEFINED),
                max_samplers_per_shader_stage: Some(LIMIT_U32_UNDEFINED),
                max_storage_buffers_per_shader_stage: Some(LIMIT_U32_UNDEFINED),
                max_storage_textures_per_shader_stage: Some(LIMIT_U32_UNDEFINED),
                max_uniform_buffers_per_shader_stage: Some(LIMIT_U32_UNDEFINED),
                max_uniform_buffer_binding_size: Some(LIMIT_U64_UNDEFINED),
                max_storage_buffer_binding_size: Some(LIMIT_U64_UNDEFINED),
                min_uniform_buffer_offset_alignment: Some(LIMIT_U32_UNDEFINED),
                min_storage_buffer_offset_alignment: Some(LIMIT_U32_UNDEFINED),
                max_vertex_buffers: Some(LIMIT_U32_UNDEFINED),
                max_buffer_size: Some(LIMIT_U64_UNDEFINED),
                max_vertex_attributes: Some(LIMIT_U32_UNDEFINED),
                max_vertex_buffer_array_stride: Some(LIMIT_U32_UNDEFINED),
                max_inter_stage_shader_variables: Some(LIMIT_U32_UNDEFINED),
                max_color_attachments: Some(LIMIT_U32_UNDEFINED),
                max_color_attachment_bytes_per_sample: Some(LIMIT_U32_UNDEFINED),
                max_compute_workgroup_storage_size: Some(LIMIT_U32_UNDEFINED),
                max_compute_invocations_per_workgroup: Some(LIMIT_U32_UNDEFINED),
                max_compute_workgroup_size_x: Some(LIMIT_U32_UNDEFINED),
                max_compute_workgroup_size_y: Some(LIMIT_U32_UNDEFINED),
                max_compute_workgroup_size_z: Some(LIMIT_U32_UNDEFINED),
                max_compute_workgroups_per_dimension: Some(LIMIT_U32_UNDEFINED),
                max_immediate_size: Some(LIMIT_U32_UNDEFINED),
            }
        }
    }
    impl Limits {
        pub fn new() -> Self {
            Self::default()
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
                max_inter_stage_shader_variables: Some(
                    value.maxInterStageShaderVariables,
                ),
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUMultisampleState, ChainedStructStorage) {
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
            raw.alphaToCoverageEnabled = if self
                .alpha_to_coverage_enabled
                .unwrap_or(false)
            {
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
                beginning_of_pass_write_index: Some(QUERY_SET_INDEX_UNDEFINED),
                end_of_pass_write_index: Some(QUERY_SET_INDEX_UNDEFINED),
            }
        }
    }
    impl PassTimestampWrites {
        pub fn new() -> Self {
            Self::default()
        }
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUPassTimestampWrites, ChainedStructStorage) {
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
        pub fn with_extension(
            mut self,
            extension: PassTimestampWritesExtension,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUPipelineLayoutDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUPipelineLayoutDescriptor = unsafe {
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
            raw.unclippedDepth = if self.unclipped_depth.unwrap_or(false) {
                1
            } else {
                0
            };
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUQuerySetDescriptor, ChainedStructStorage) {
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
        pub fn with_extension(
            mut self,
            extension: RenderBundleDescriptorExtension,
        ) -> Self {
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
            raw.colorFormatCount = self
                .color_formats
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);
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
            raw.depthReadOnly = if self.depth_read_only.unwrap_or(false) {
                1
            } else {
                0
            };
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
                depth_slice: Some(DEPTH_SLICE_UNDEFINED),
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPURenderPassColorAttachment, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPURenderPassColorAttachment = unsafe {
                std::mem::zeroed()
            };
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
                depth_clear_value: Some(DEPTH_CLEAR_VALUE_UNDEFINED),
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
            raw.depthReadOnly = if self.depth_read_only.unwrap_or(false) {
                1
            } else {
                0
            };
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
        pub(crate) fn from_ffi(
            value: ffi::WGPURenderPassDepthStencilAttachment,
        ) -> Self {
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
        pub fn with_extension(
            mut self,
            extension: RenderPassDescriptorExtension,
        ) -> Self {
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
                    Some(
                        PassTimestampWrites::from_ffi(unsafe { *value.timestampWrites }),
                    )
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPURenderPipelineDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPURenderPipelineDescriptor = unsafe {
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
        pub(crate) fn from_ffi(
            value: ffi::WGPURequestAdapterWebGPUBackendOptions,
        ) -> Self {
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
            let mut raw: ffi::WGPURequestAdapterWebXROptions = unsafe {
                std::mem::zeroed()
            };
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
        pub fn with_extension(
            mut self,
            extension: RequestAdapterOptionsExtension,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUResourceTableDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUResourceTableDescriptor = unsafe {
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
        pub fn with_extension(
            mut self,
            extension: SamplerBindingLayoutExtension,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUSamplerDescriptor, ChainedStructStorage) {
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
        pub fn with_extension(
            mut self,
            extension: ShaderModuleDescriptorExtension,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUShaderSourceSPIRV, ChainedStructStorage) {
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
                            std::slice::from_raw_parts(
                                value.code,
                                value.codeSize as usize,
                            )
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUShaderSourceWGSL, ChainedStructStorage) {
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
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUSharedBufferMemoryEndAccessState>,
    }
    impl Default for SharedBufferMemoryEndAccessState {
        fn default() -> Self {
            Self {
                extensions: Vec::new(),
                initialized: None,
                fences: None,
                signaled_values: None,
                _free_members: None,
            }
        }
    }
    impl SharedBufferMemoryEndAccessState {
        pub fn new() -> Self {
            Self::default()
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUSharedBufferMemoryEndAccessState,
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUSharedBufferMemoryEndAccessState) {
            unsafe { ffi::wgpuSharedBufferMemoryEndAccessStateFreeMembers(value) };
        }
    }
    impl Drop for SharedBufferMemoryEndAccessState {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuSharedBufferMemoryEndAccessStateFreeMembers(value) };
            }
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUSharedFenceMTLSharedEventDescriptor,
        ) -> Self {
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUSharedFenceMTLSharedEventExportInfo,
        ) -> Self {
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
        pub fn with_extension(
            mut self,
            extension: SharedFenceDescriptorExtension,
        ) -> Self {
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
        pub fn with_extension(
            mut self,
            extension: SharedFenceExportInfoExtension,
        ) -> Self {
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
        ) -> (
            ffi::WGPUSharedFenceVkSemaphoreZirconHandleDescriptor,
            ChainedStructStorage,
        ) {
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
        ) -> (
            ffi::WGPUSharedFenceVkSemaphoreZirconHandleExportInfo,
            ChainedStructStorage,
        ) {
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
            raw.requiresEndAccessFence = if self
                .requires_end_access_fence
                .unwrap_or(false)
            {
                1
            } else {
                0
            };
            (raw, storage)
        }
        pub(crate) fn from_ffi(
            value: ffi::WGPUSharedTextureMemoryD3D11BeginState,
        ) -> Self {
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
        ) -> (
            ffi::WGPUSharedTextureMemoryDXGISharedHandleDescriptor,
            ChainedStructStorage,
        ) {
            let mut storage = ChainedStructStorage::new();
            let mut raw: ffi::WGPUSharedTextureMemoryDXGISharedHandleDescriptor = unsafe {
                std::mem::zeroed()
            };
            if let Some(value) = self.handle {
                raw.handle = value;
            }
            raw.useKeyedMutex = if self.use_keyed_mutex.unwrap_or(false) {
                1
            } else {
                0
            };
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
        ) -> (
            ffi::WGPUSharedTextureMemoryAHardwareBufferDescriptor,
            ChainedStructStorage,
        ) {
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
        ) -> (
            ffi::WGPUSharedTextureMemoryAHardwareBufferProperties,
            ChainedStructStorage,
        ) {
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
            raw.concurrentRead = if self.concurrent_read.unwrap_or(false) {
                1
            } else {
                0
            };
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUSharedTextureMemoryDmaBufDescriptor,
        ) -> Self {
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
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUSharedTextureMemoryEndAccessState>,
    }
    impl Default for SharedTextureMemoryEndAccessState {
        fn default() -> Self {
            Self {
                extensions: Vec::new(),
                initialized: None,
                fences: None,
                signaled_values: None,
                _free_members: None,
            }
        }
    }
    impl SharedTextureMemoryEndAccessState {
        pub fn new() -> Self {
            Self::default()
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUSharedTextureMemoryEndAccessState,
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUSharedTextureMemoryEndAccessState) {
            unsafe { ffi::wgpuSharedTextureMemoryEndAccessStateFreeMembers(value) };
        }
    }
    impl Drop for SharedTextureMemoryEndAccessState {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuSharedTextureMemoryEndAccessStateFreeMembers(value) };
            }
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
        ) -> (
            ffi::WGPUSharedTextureMemoryVkImageLayoutBeginState,
            ChainedStructStorage,
        ) {
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
                sampled_texture_binding: Some(LIMIT_U32_UNDEFINED),
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
            let mut raw: ffi::WGPUStaticSamplerBindingLayout = unsafe {
                std::mem::zeroed()
            };
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUStencilFaceState, ChainedStructStorage) {
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
            Self {
                data: None,
                length: Some(STRLEN),
            }
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
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUSupportedWGSLLanguageFeatures>,
    }
    impl Default for SupportedWGSLLanguageFeatures {
        fn default() -> Self {
            Self {
                features: None,
                _free_members: None,
            }
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUSupportedWGSLLanguageFeatures) {
            unsafe { ffi::wgpuSupportedWGSLLanguageFeaturesFreeMembers(value) };
        }
    }
    impl Drop for SupportedWGSLLanguageFeatures {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuSupportedWGSLLanguageFeaturesFreeMembers(value) };
            }
        }
    }
    pub struct SupportedFeatures {
        pub features: Option<Vec<FeatureName>>,
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUSupportedFeatures>,
    }
    impl Default for SupportedFeatures {
        fn default() -> Self {
            Self {
                features: None,
                _free_members: None,
            }
        }
    }
    impl SupportedFeatures {
        pub fn new() -> Self {
            Self::default()
        }
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUSupportedFeatures, ChainedStructStorage) {
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUSupportedFeatures) {
            unsafe { ffi::wgpuSupportedFeaturesFreeMembers(value) };
        }
    }
    impl Drop for SupportedFeatures {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuSupportedFeaturesFreeMembers(value) };
            }
        }
    }
    pub struct SupportedInstanceFeatures {
        pub features: Option<Vec<InstanceFeatureName>>,
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUSupportedInstanceFeatures>,
    }
    impl Default for SupportedInstanceFeatures {
        fn default() -> Self {
            Self {
                features: None,
                _free_members: None,
            }
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
            let mut raw: ffi::WGPUSupportedInstanceFeatures = unsafe {
                std::mem::zeroed()
            };
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUSupportedInstanceFeatures) {
            unsafe { ffi::wgpuSupportedInstanceFeaturesFreeMembers(value) };
        }
    }
    impl Drop for SupportedInstanceFeatures {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuSupportedInstanceFeaturesFreeMembers(value) };
            }
        }
    }
    pub struct SurfaceCapabilities {
        pub(crate) extensions: Vec<SurfaceCapabilitiesExtension>,
        pub usages: Option<TextureUsage>,
        pub formats: Option<Vec<TextureFormat>>,
        pub present_modes: Option<Vec<PresentMode>>,
        pub alpha_modes: Option<Vec<CompositeAlphaMode>>,
        #[doc(hidden)]
        pub(crate) _free_members: Option<ffi::WGPUSurfaceCapabilities>,
    }
    impl Default for SurfaceCapabilities {
        fn default() -> Self {
            Self {
                extensions: Vec::new(),
                usages: None,
                formats: None,
                present_modes: None,
                alpha_modes: None,
                _free_members: None,
            }
        }
    }
    impl SurfaceCapabilities {
        pub fn new() -> Self {
            Self::default()
        }
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUSurfaceCapabilities, ChainedStructStorage) {
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
            raw.presentModeCount = self
                .present_modes
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);
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
        pub fn with_extension(
            mut self,
            extension: SurfaceCapabilitiesExtension,
        ) -> Self {
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
                _free_members: Some(value),
            }
        }
        pub(crate) fn free_members(value: ffi::WGPUSurfaceCapabilities) {
            unsafe { ffi::wgpuSurfaceCapabilitiesFreeMembers(value) };
        }
    }
    impl Drop for SurfaceCapabilities {
        fn drop(&mut self) {
            if let Some(value) = self._free_members.take() {
                unsafe { ffi::wgpuSurfaceCapabilitiesFreeMembers(value) };
            }
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
            raw.viewFormatCount = self
                .view_formats
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);
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
        pub fn with_extension(
            mut self,
            extension: SurfaceConfigurationExtension,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUSurfaceDescriptor, ChainedStructStorage) {
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
        ) -> (
            ffi::WGPUSurfaceDescriptorFromWindowsUWPSwapChainPanel,
            ChainedStructStorage,
        ) {
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUSurfaceSourceAndroidNativeWindow,
        ) -> Self {
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
            let mut raw: ffi::WGPUSurfaceSourceMetalLayer = unsafe {
                std::mem::zeroed()
            };
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
            let mut raw: ffi::WGPUSurfaceSourceWindowsHWND = unsafe {
                std::mem::zeroed()
            };
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
            let mut raw: ffi::WGPUSurfaceSourceXlibWindow = unsafe {
                std::mem::zeroed()
            };
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
            let mut raw: ffi::WGPUTexelBufferBindingEntry = unsafe {
                std::mem::zeroed()
            };
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
            let mut raw: ffi::WGPUTexelBufferBindingLayout = unsafe {
                std::mem::zeroed()
            };
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
                size: Some(WHOLE_SIZE),
            }
        }
    }
    impl TexelBufferViewDescriptor {
        pub fn new() -> Self {
            Self::default()
        }
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUTexelBufferViewDescriptor, ChainedStructStorage) {
            let mut storage = ChainedStructStorage::new();
            let mut next: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
            for ext in self.extensions.iter().rev() {
                next = ext.push_chain(&mut storage, next);
            }
            let mut raw: ffi::WGPUTexelBufferViewDescriptor = unsafe {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUTexelCopyBufferInfo, ChainedStructStorage) {
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
                bytes_per_row: Some(COPY_STRIDE_UNDEFINED),
                rows_per_image: Some(COPY_STRIDE_UNDEFINED),
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
        pub fn with_extension(
            mut self,
            extension: TextureBindingLayoutExtension,
        ) -> Self {
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
            let mut raw: ffi::WGPUTextureComponentSwizzle = unsafe {
                std::mem::zeroed()
            };
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
        pub(crate) fn from_ffi(
            value: ffi::WGPUTextureComponentSwizzleDescriptor,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUTextureDescriptor, ChainedStructStorage) {
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
            raw.viewFormatCount = self
                .view_formats
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);
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
                mip_level_count: Some(MIP_LEVEL_COUNT_UNDEFINED),
                base_array_layer: Some(0),
                array_layer_count: Some(ARRAY_LAYER_COUNT_UNDEFINED),
                aspect: Some(TextureAspect::All),
                usage: Some(TextureUsage::NONE),
            }
        }
    }
    impl TextureViewDescriptor {
        pub fn new() -> Self {
            Self::default()
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
        pub fn with_extension(
            mut self,
            extension: TextureViewDescriptorExtension,
        ) -> Self {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUVertexBufferLayout, ChainedStructStorage) {
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
        pub(crate) fn to_ffi(
            &self,
        ) -> (ffi::WGPUYCbCrVkDescriptor, ChainedStructStorage) {
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
                force_explicit_reconstruction: Some(
                    value.forceExplicitReconstruction != 0,
                ),
                external_format: Some(value.externalFormat),
            }
        }
    }
}
mod extensions {
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
            AdapterInfoExtension::AdapterPropertiesExplicitComputeSubgroupSizeConfigs(
                ext,
            )
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
    impl std::convert::From<DawnAdapterPropertiesPowerPreference>
    for AdapterInfoExtension {
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
                    raw.chain.sType = SType::AdapterPropertiesD3D.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                AdapterInfoExtension::AdapterPropertiesWGPU(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::AdapterPropertiesWGPU.into();
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                AdapterInfoExtension::AdapterPropertiesMemoryHeaps(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::AdapterPropertiesMemoryHeaps.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                AdapterInfoExtension::AdapterPropertiesSubgroupMatrixConfigs(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::AdapterPropertiesSubgroupMatrixConfigs
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                AdapterInfoExtension::AdapterPropertiesVk(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::AdapterPropertiesVk.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                AdapterInfoExtension::DawnAdapterPropertiesPowerPreference(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnAdapterPropertiesPowerPreference.into();
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
                    raw.chain.sType = SType::ExternalTextureBindingEntry.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                BindGroupEntryExtension::TexelBufferBindingEntry(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::TexelBufferBindingEntry.into();
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
    impl std::convert::From<ExternalTextureBindingLayout>
    for BindGroupLayoutEntryExtension {
        fn from(ext: ExternalTextureBindingLayout) -> Self {
            BindGroupLayoutEntryExtension::ExternalTextureBindingLayout(ext)
        }
    }
    impl std::convert::From<StaticSamplerBindingLayout>
    for BindGroupLayoutEntryExtension {
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
                    raw.chain.sType = SType::ExternalTextureBindingLayout.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                BindGroupLayoutEntryExtension::StaticSamplerBindingLayout(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::StaticSamplerBindingLayout.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                BindGroupLayoutEntryExtension::TexelBufferBindingLayout(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::TexelBufferBindingLayout.into();
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
                    raw.chain.sType = SType::BufferHostMappedPointer.into();
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                BufferDescriptorExtension::DawnFakeBufferOOMForTesting(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnFakeBufferOOMForTesting.into();
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
        ColorTargetStateExpandResolveTextureDawn(
            ColorTargetStateExpandResolveTextureDawn,
        ),
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
                        .into();
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
                    raw.chain.sType = SType::DawnEncoderInternalUsageDescriptor.into();
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
    impl std::convert::From<DawnCompilationMessageUtf16>
    for CompilationMessageExtension {
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
                    raw.chain.sType = SType::DawnCompilationMessageUtf16.into();
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
    impl std::convert::From<DawnDrmFormatCapabilities>
    for DawnFormatCapabilitiesExtension {
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
                    raw.chain.sType = SType::DawnDrmFormatCapabilities.into();
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
                    raw.chain.sType = SType::DawnCacheDeviceDescriptor.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                DeviceDescriptorExtension::DawnConsumeAdapterDescriptor(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnConsumeAdapterDescriptor.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                DeviceDescriptorExtension::DawnDeviceAllocatorControl(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnDeviceAllocatorControl.into();
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                DeviceDescriptorExtension::DawnTogglesDescriptor(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnTogglesDescriptor.into();
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
                    raw.chain.sType = SType::DawnWGSLBlocklist.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                InstanceDescriptorExtension::DawnTogglesDescriptor(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnTogglesDescriptor.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                InstanceDescriptorExtension::DawnWireWGSLControl(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnWireWGSLControl.into();
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
    impl LimitsExtension {
        pub(crate) fn push_chain(
            &self,
            storage: &mut ChainedStructStorage,
            next: *mut ffi::WGPUChainedStruct,
        ) -> *mut ffi::WGPUChainedStruct {
            match self {
                LimitsExtension::DawnHostMappedPointerLimits(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnHostMappedPointerLimits.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                LimitsExtension::DawnTexelCopyBufferRowAlignmentLimits(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnTexelCopyBufferRowAlignmentLimits
                        .into();
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
                    raw.chain.sType = SType::PipelineLayoutPixelLocalStorage.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                PipelineLayoutDescriptorExtension::PipelineLayoutResourceTable(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::PipelineLayoutResourceTable.into();
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
    pub enum RenderPassColorAttachmentExtension {}
    impl RenderPassColorAttachmentExtension {
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
    impl std::convert::From<DawnRenderPassSampleCount>
    for RenderPassDescriptorExtension {
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
    impl std::convert::From<RenderPassPixelLocalStorage>
    for RenderPassDescriptorExtension {
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
                    raw.chain.sType = SType::DawnRenderPassSampleCount.into();
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                RenderPassDescriptorExtension::RenderPassDescriptorResolveRect(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::RenderPassDescriptorResolveRect.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                RenderPassDescriptorExtension::RenderPassMaxDrawCount(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::RenderPassMaxDrawCount.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                RenderPassDescriptorExtension::RenderPassPixelLocalStorage(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::RenderPassPixelLocalStorage.into();
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
    impl std::convert::From<RequestAdapterWebXROptions>
    for RequestAdapterOptionsExtension {
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
                    raw.chain.sType = SType::DawnTogglesDescriptor.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                RequestAdapterOptionsExtension::RequestAdapterWebGPUBackendOptions(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::RequestAdapterWebGPUBackendOptions.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                RequestAdapterOptionsExtension::RequestAdapterWebXROptions(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::RequestAdapterWebXROptions.into();
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
                    raw.chain.sType = SType::YCbCrVkDescriptor.into();
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                ShaderModuleDescriptorExtension::ShaderModuleCompilationOptions(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::ShaderModuleCompilationOptions.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                ShaderModuleDescriptorExtension::ShaderSourceSPIRV(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::ShaderSourceSPIRV.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                ShaderModuleDescriptorExtension::ShaderSourceWGSL(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::ShaderSourceWGSL.into();
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
        SharedFenceVkSemaphoreOpaqueFDDescriptor(
            SharedFenceVkSemaphoreOpaqueFDDescriptor,
        ),
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
    impl std::convert::From<SharedFenceSyncFDDescriptor>
    for SharedFenceDescriptorExtension {
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
            SharedFenceDescriptorExtension::SharedFenceVkSemaphoreZirconHandleDescriptor(
                ext,
            )
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedFenceDescriptorExtension::SharedFenceEGLSyncDescriptor(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedFenceEGLSyncDescriptor.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedFenceDescriptorExtension::SharedFenceMTLSharedEventDescriptor(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedFenceMTLSharedEventDescriptor.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedFenceDescriptorExtension::SharedFenceSyncFDDescriptor(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedFenceSyncFDDescriptor.into();
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
                        .into();
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
                        .into();
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
        SharedFenceVkSemaphoreOpaqueFDExportInfo(
            SharedFenceVkSemaphoreOpaqueFDExportInfo,
        ),
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
    impl std::convert::From<SharedFenceSyncFDExportInfo>
    for SharedFenceExportInfoExtension {
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
            SharedFenceExportInfoExtension::SharedFenceVkSemaphoreZirconHandleExportInfo(
                ext,
            )
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedFenceExportInfoExtension::SharedFenceEGLSyncExportInfo(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedFenceEGLSyncExportInfo.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedFenceExportInfoExtension::SharedFenceMTLSharedEventExportInfo(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedFenceMTLSharedEventExportInfo.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedFenceExportInfoExtension::SharedFenceSyncFDExportInfo(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedFenceSyncFDExportInfo.into();
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
                        .into();
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
                        .into();
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
        SharedTextureMemoryD3DSwapchainBeginState(
            SharedTextureMemoryD3DSwapchainBeginState,
        ),
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedTextureMemoryBeginAccessDescriptorExtension::SharedTextureMemoryD3D11BeginState(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedTextureMemoryD3D11BeginState.into();
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
                        .into();
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
        SharedTextureMemoryZirconHandleDescriptor(
            SharedTextureMemoryZirconHandleDescriptor,
        ),
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
            SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDmaBufDescriptor(
                ext,
            )
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
                        .into();
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
                        .into();
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
                        .into();
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SharedTextureMemoryDescriptorExtension::SharedTextureMemoryDmaBufDescriptor(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SharedTextureMemoryDmaBufDescriptor.into();
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
                        .into();
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
                        .into();
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
                        .into();
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
        SharedTextureMemoryVkImageLayoutEndState(
            SharedTextureMemoryVkImageLayoutEndState,
        ),
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
                        .into();
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
                        .into();
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
                        .into();
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
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsUWPSwapChainPanel(
                ext,
            )
        }
    }
    impl std::convert::From<SurfaceDescriptorFromWindowsWinUISwapChainPanel>
    for SurfaceDescriptorExtension {
        fn from(ext: SurfaceDescriptorFromWindowsWinUISwapChainPanel) -> Self {
            SurfaceDescriptorExtension::SurfaceDescriptorFromWindowsWinUISwapChainPanel(
                ext,
            )
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
                    raw.chain.sType = SType::SurfaceColorManagement.into();
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
                        .into();
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
                        .into();
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
                        .into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SurfaceDescriptorExtension::SurfaceSourceXCBWindow(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SurfaceSourceXCBWindow.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SurfaceDescriptorExtension::SurfaceSourceAndroidNativeWindow(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SurfaceSourceAndroidNativeWindow.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SurfaceDescriptorExtension::SurfaceSourceMetalLayer(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SurfaceSourceMetalLayer.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SurfaceDescriptorExtension::SurfaceSourceWaylandSurface(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SurfaceSourceWaylandSurface.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SurfaceDescriptorExtension::SurfaceSourceWindowsHWND(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SurfaceSourceWindowsHWND.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                SurfaceDescriptorExtension::SurfaceSourceXlibWindow(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::SurfaceSourceXlibWindow.into();
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
                TextureDescriptorExtension::DawnTextureInternalUsageDescriptor(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::DawnTextureInternalUsageDescriptor.into();
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
                TextureViewDescriptorExtension::TextureComponentSwizzleDescriptor(
                    value,
                ) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::TextureComponentSwizzleDescriptor.into();
                    raw.chain.next = next;
                    storage.push_storage(storage_value);
                    let raw_ptr = storage.push_value_mut(raw);
                    raw_ptr.cast::<ffi::WGPUChainedStruct>()
                }
                TextureViewDescriptorExtension::YCbCrVkDescriptor(value) => {
                    let (mut raw, storage_value) = value.to_ffi();
                    raw.chain.sType = SType::YCbCrVkDescriptor.into();
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
}
mod objects {
    #![allow(dead_code, unused_imports)]
    use crate::generated::*;
    use crate::ffi;
    #[derive(Debug)]
    pub struct Adapter {
        raw: ffi::WGPUAdapter,
        _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    }
    impl Adapter {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUAdapter) -> Self {
            Self {
                raw,
                _not_sync: std::marker::PhantomData,
            }
        }
        pub fn as_raw(&self) -> ffi::WGPUAdapter {
            self.raw
        }
        pub fn get_instance(&self) -> Instance {
            let result = unsafe { ffi::wgpuAdapterGetInstance(self.raw) };
            unsafe { Instance::from_raw(result) }
        }
        pub fn get_limits(&self, limits: &mut Limits) -> Status {
            let (mut limits_ffi, _limits_storage) = limits.to_ffi();
            let limits_ptr = std::ptr::addr_of_mut!(limits_ffi);
            let result = unsafe { ffi::wgpuAdapterGetLimits(self.raw, limits_ptr) };
            *limits = Limits::from_ffi(limits_ffi);
            result.into()
        }
        pub fn get_info(&self, info: &mut AdapterInfo) -> Status {
            let (mut info_ffi, _info_storage) = info.to_ffi();
            let info_ptr = std::ptr::addr_of_mut!(info_ffi);
            let result = unsafe { ffi::wgpuAdapterGetInfo(self.raw, info_ptr) };
            *info = AdapterInfo::from_ffi(info_ffi);
            result.into()
        }
        pub fn has_feature(&self, feature: FeatureName) -> bool {
            let feature_ffi: ffi::WGPUFeatureName = feature.into();
            let result = unsafe { ffi::wgpuAdapterHasFeature(self.raw, feature_ffi) };
            result != 0
        }
        pub fn get_features(&self, features: &mut SupportedFeatures) -> () {
            let (mut features_ffi, _features_storage) = features.to_ffi();
            let features_ptr = std::ptr::addr_of_mut!(features_ffi);
            unsafe { ffi::wgpuAdapterGetFeatures(self.raw, features_ptr) };
            *features = SupportedFeatures::from_ffi(features_ffi);
            ()
        }
        pub fn request_device(
            &self,
            descriptor: Option<&DeviceDescriptor>,
            callback: impl FnMut(
                RequestDeviceStatus,
                Option<Device>,
                String,
            ) + Send + 'static,
        ) -> Future {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let callback_box: RequestDeviceCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPURequestDeviceCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(request_device_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuAdapterRequestDevice(
                    self.raw,
                    descriptor_ptr,
                    callback_info_ffi,
                )
            };
            Future::from_ffi(result)
        }
        pub fn create_device(&self, descriptor: Option<&DeviceDescriptor>) -> Device {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuAdapterCreateDevice(self.raw, descriptor_ptr)
            };
            unsafe { Device::from_raw(result) }
        }
        pub fn get_format_capabilities(
            &self,
            format: TextureFormat,
            capabilities: &mut DawnFormatCapabilities,
        ) -> Status {
            let format_ffi: ffi::WGPUTextureFormat = format.into();
            let (mut capabilities_ffi, _capabilities_storage) = capabilities.to_ffi();
            let capabilities_ptr = std::ptr::addr_of_mut!(capabilities_ffi);
            let result = unsafe {
                ffi::wgpuAdapterGetFormatCapabilities(
                    self.raw,
                    format_ffi,
                    capabilities_ptr,
                )
            };
            *capabilities = DawnFormatCapabilities::from_ffi(capabilities_ffi);
            result.into()
        }
    }
    impl Drop for Adapter {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuAdapterRelease(self.raw) };
        }
    }
    impl Clone for Adapter {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuAdapterAddRef(self.raw) };
            Self {
                raw: self.raw,
                _not_sync: std::marker::PhantomData,
            }
        }
    }
    unsafe impl Send for Adapter {}
    #[derive(Debug)]
    pub struct BindGroup {
        raw: ffi::WGPUBindGroup,
    }
    impl BindGroup {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUBindGroup) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUBindGroup {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuBindGroupSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for BindGroup {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuBindGroupRelease(self.raw) };
        }
    }
    impl Clone for BindGroup {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuBindGroupAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for BindGroup {}
    unsafe impl Sync for BindGroup {}
    #[derive(Debug)]
    pub struct BindGroupLayout {
        raw: ffi::WGPUBindGroupLayout,
    }
    impl BindGroupLayout {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUBindGroupLayout) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUBindGroupLayout {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuBindGroupLayoutSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for BindGroupLayout {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuBindGroupLayoutRelease(self.raw) };
        }
    }
    impl Clone for BindGroupLayout {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuBindGroupLayoutAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for BindGroupLayout {}
    unsafe impl Sync for BindGroupLayout {}
    #[derive(Debug)]
    pub struct Buffer {
        raw: ffi::WGPUBuffer,
    }
    impl Buffer {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUBuffer) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUBuffer {
            self.raw
        }
        pub fn map_async(
            &self,
            mode: MapMode,
            offset: usize,
            size: usize,
            callback: impl FnMut(MapAsyncStatus, String) + Send + 'static,
        ) -> Future {
            let mode_ffi: ffi::WGPUMapMode = mode.into();
            let callback_box: BufferMapCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPUBufferMapCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(buffer_map_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuBufferMapAsync(
                    self.raw,
                    mode_ffi,
                    offset,
                    size,
                    callback_info_ffi,
                )
            };
            Future::from_ffi(result)
        }
        pub fn get_mapped_range(
            &self,
            offset: usize,
            size: usize,
        ) -> *mut std::ffi::c_void {
            let result = unsafe {
                ffi::wgpuBufferGetMappedRange(self.raw, offset, size)
            };
            result
        }
        pub fn get_const_mapped_range(
            &self,
            offset: usize,
            size: usize,
        ) -> *const std::ffi::c_void {
            let result = unsafe {
                ffi::wgpuBufferGetConstMappedRange(self.raw, offset, size)
            };
            result
        }
        pub fn write_mapped_range(
            &self,
            offset: usize,
            data: &[std::ffi::c_void],
        ) -> Status {
            let data_ptr = data.as_ptr();
            let result = unsafe {
                ffi::wgpuBufferWriteMappedRange(self.raw, offset, data_ptr, data.len())
            };
            result.into()
        }
        pub fn read_mapped_range(
            &self,
            offset: usize,
            mut data: &mut [std::ffi::c_void],
        ) -> Status {
            let data_ptr = data.as_mut_ptr();
            let result = unsafe {
                ffi::wgpuBufferReadMappedRange(self.raw, offset, data_ptr, data.len())
            };
            result.into()
        }
        pub fn create_texel_view(
            &self,
            descriptor: &TexelBufferViewDescriptor,
        ) -> TexelBufferView {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuBufferCreateTexelView(self.raw, descriptor_ptr)
            };
            unsafe { TexelBufferView::from_raw(result) }
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuBufferSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn get_usage(&self) -> BufferUsage {
            let result = unsafe { ffi::wgpuBufferGetUsage(self.raw) };
            result.into()
        }
        pub fn get_size(&self) -> u64 {
            let result = unsafe { ffi::wgpuBufferGetSize(self.raw) };
            result
        }
        pub fn get_map_state(&self) -> BufferMapState {
            let result = unsafe { ffi::wgpuBufferGetMapState(self.raw) };
            result.into()
        }
        pub fn unmap(&self) -> () {
            unsafe { ffi::wgpuBufferUnmap(self.raw) };
            ()
        }
        pub fn destroy(&self) -> () {
            unsafe { ffi::wgpuBufferDestroy(self.raw) };
            ()
        }
    }
    impl Drop for Buffer {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuBufferRelease(self.raw) };
        }
    }
    impl Clone for Buffer {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuBufferAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for Buffer {}
    unsafe impl Sync for Buffer {}
    #[derive(Debug)]
    pub struct CommandBuffer {
        raw: ffi::WGPUCommandBuffer,
    }
    impl CommandBuffer {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUCommandBuffer) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUCommandBuffer {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuCommandBufferSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for CommandBuffer {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuCommandBufferRelease(self.raw) };
        }
    }
    impl Clone for CommandBuffer {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuCommandBufferAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for CommandBuffer {}
    unsafe impl Sync for CommandBuffer {}
    #[derive(Debug)]
    pub struct CommandEncoder {
        raw: ffi::WGPUCommandEncoder,
        _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    }
    impl CommandEncoder {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUCommandEncoder) -> Self {
            Self {
                raw,
                _not_sync: std::marker::PhantomData,
            }
        }
        pub fn as_raw(&self) -> ffi::WGPUCommandEncoder {
            self.raw
        }
        pub fn finish(
            &self,
            descriptor: Option<&CommandBufferDescriptor>,
        ) -> CommandBuffer {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuCommandEncoderFinish(self.raw, descriptor_ptr)
            };
            unsafe { CommandBuffer::from_raw(result) }
        }
        pub fn begin_compute_pass(
            &self,
            descriptor: Option<&ComputePassDescriptor>,
        ) -> ComputePassEncoder {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuCommandEncoderBeginComputePass(self.raw, descriptor_ptr)
            };
            unsafe { ComputePassEncoder::from_raw(result) }
        }
        pub fn begin_render_pass(
            &self,
            descriptor: &RenderPassDescriptor,
        ) -> RenderPassEncoder {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuCommandEncoderBeginRenderPass(self.raw, descriptor_ptr)
            };
            unsafe { RenderPassEncoder::from_raw(result) }
        }
        pub fn copy_buffer_to_buffer(
            &self,
            source: Buffer,
            source_offset: u64,
            destination: Buffer,
            destination_offset: u64,
            size: u64,
        ) -> () {
            unsafe {
                ffi::wgpuCommandEncoderCopyBufferToBuffer(
                    self.raw,
                    source.as_raw(),
                    source_offset,
                    destination.as_raw(),
                    destination_offset,
                    size,
                )
            };
            ()
        }
        pub fn copy_buffer_to_texture(
            &self,
            source: &TexelCopyBufferInfo,
            destination: &TexelCopyTextureInfo,
            copy_size: &Extent3D,
        ) -> () {
            let (source_ffi, _source_storage) = source.to_ffi();
            let source_ptr = std::ptr::addr_of!(source_ffi);
            let (destination_ffi, _destination_storage) = destination.to_ffi();
            let destination_ptr = std::ptr::addr_of!(destination_ffi);
            let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
            let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
            unsafe {
                ffi::wgpuCommandEncoderCopyBufferToTexture(
                    self.raw,
                    source_ptr,
                    destination_ptr,
                    copy_size_ptr,
                )
            };
            ()
        }
        pub fn copy_texture_to_buffer(
            &self,
            source: &TexelCopyTextureInfo,
            destination: &TexelCopyBufferInfo,
            copy_size: &Extent3D,
        ) -> () {
            let (source_ffi, _source_storage) = source.to_ffi();
            let source_ptr = std::ptr::addr_of!(source_ffi);
            let (destination_ffi, _destination_storage) = destination.to_ffi();
            let destination_ptr = std::ptr::addr_of!(destination_ffi);
            let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
            let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
            unsafe {
                ffi::wgpuCommandEncoderCopyTextureToBuffer(
                    self.raw,
                    source_ptr,
                    destination_ptr,
                    copy_size_ptr,
                )
            };
            ()
        }
        pub fn copy_texture_to_texture(
            &self,
            source: &TexelCopyTextureInfo,
            destination: &TexelCopyTextureInfo,
            copy_size: &Extent3D,
        ) -> () {
            let (source_ffi, _source_storage) = source.to_ffi();
            let source_ptr = std::ptr::addr_of!(source_ffi);
            let (destination_ffi, _destination_storage) = destination.to_ffi();
            let destination_ptr = std::ptr::addr_of!(destination_ffi);
            let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
            let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
            unsafe {
                ffi::wgpuCommandEncoderCopyTextureToTexture(
                    self.raw,
                    source_ptr,
                    destination_ptr,
                    copy_size_ptr,
                )
            };
            ()
        }
        pub fn clear_buffer(&self, buffer: Buffer, offset: u64, size: u64) -> () {
            unsafe {
                ffi::wgpuCommandEncoderClearBuffer(
                    self.raw,
                    buffer.as_raw(),
                    offset,
                    size,
                )
            };
            ()
        }
        pub fn inject_validation_error(&self, message: String) -> () {
            let message_ffi = ffi::WGPUStringView {
                data: message.as_ptr().cast(),
                length: message.len(),
            };
            unsafe {
                ffi::wgpuCommandEncoderInjectValidationError(self.raw, message_ffi)
            };
            ()
        }
        pub fn insert_debug_marker(&self, marker_label: String) -> () {
            let marker_label_ffi = ffi::WGPUStringView {
                data: marker_label.as_ptr().cast(),
                length: marker_label.len(),
            };
            unsafe {
                ffi::wgpuCommandEncoderInsertDebugMarker(self.raw, marker_label_ffi)
            };
            ()
        }
        pub fn pop_debug_group(&self) -> () {
            unsafe { ffi::wgpuCommandEncoderPopDebugGroup(self.raw) };
            ()
        }
        pub fn push_debug_group(&self, group_label: String) -> () {
            let group_label_ffi = ffi::WGPUStringView {
                data: group_label.as_ptr().cast(),
                length: group_label.len(),
            };
            unsafe { ffi::wgpuCommandEncoderPushDebugGroup(self.raw, group_label_ffi) };
            ()
        }
        pub fn resolve_query_set(
            &self,
            query_set: QuerySet,
            first_query: u32,
            query_count: u32,
            destination: Buffer,
            destination_offset: u64,
        ) -> () {
            unsafe {
                ffi::wgpuCommandEncoderResolveQuerySet(
                    self.raw,
                    query_set.as_raw(),
                    first_query,
                    query_count,
                    destination.as_raw(),
                    destination_offset,
                )
            };
            ()
        }
        pub fn write_buffer(
            &self,
            buffer: Buffer,
            buffer_offset: u64,
            data: &[u8],
        ) -> () {
            let data_ptr = data.as_ptr();
            unsafe {
                ffi::wgpuCommandEncoderWriteBuffer(
                    self.raw,
                    buffer.as_raw(),
                    buffer_offset,
                    data_ptr,
                    (data.len()) as u64,
                )
            };
            ()
        }
        pub fn write_timestamp(&self, query_set: QuerySet, query_index: u32) -> () {
            unsafe {
                ffi::wgpuCommandEncoderWriteTimestamp(
                    self.raw,
                    query_set.as_raw(),
                    query_index,
                )
            };
            ()
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuCommandEncoderSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for CommandEncoder {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuCommandEncoderRelease(self.raw) };
        }
    }
    impl Clone for CommandEncoder {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuCommandEncoderAddRef(self.raw) };
            Self {
                raw: self.raw,
                _not_sync: std::marker::PhantomData,
            }
        }
    }
    unsafe impl Send for CommandEncoder {}
    #[derive(Debug)]
    pub struct ComputePassEncoder {
        raw: ffi::WGPUComputePassEncoder,
        _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    }
    impl ComputePassEncoder {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUComputePassEncoder) -> Self {
            Self {
                raw,
                _not_sync: std::marker::PhantomData,
            }
        }
        pub fn as_raw(&self) -> ffi::WGPUComputePassEncoder {
            self.raw
        }
        pub fn insert_debug_marker(&self, marker_label: String) -> () {
            let marker_label_ffi = ffi::WGPUStringView {
                data: marker_label.as_ptr().cast(),
                length: marker_label.len(),
            };
            unsafe {
                ffi::wgpuComputePassEncoderInsertDebugMarker(self.raw, marker_label_ffi)
            };
            ()
        }
        pub fn pop_debug_group(&self) -> () {
            unsafe { ffi::wgpuComputePassEncoderPopDebugGroup(self.raw) };
            ()
        }
        pub fn push_debug_group(&self, group_label: String) -> () {
            let group_label_ffi = ffi::WGPUStringView {
                data: group_label.as_ptr().cast(),
                length: group_label.len(),
            };
            unsafe {
                ffi::wgpuComputePassEncoderPushDebugGroup(self.raw, group_label_ffi)
            };
            ()
        }
        pub fn set_pipeline(&self, pipeline: ComputePipeline) -> () {
            unsafe {
                ffi::wgpuComputePassEncoderSetPipeline(self.raw, pipeline.as_raw())
            };
            ()
        }
        pub fn set_bind_group(
            &self,
            group_index: u32,
            group: Option<BindGroup>,
            dynamic_offsets: &[u32],
        ) -> () {
            let group_raw = group
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            let dynamic_offsets_ptr = dynamic_offsets.as_ptr();
            unsafe {
                ffi::wgpuComputePassEncoderSetBindGroup(
                    self.raw,
                    group_index,
                    group_raw,
                    dynamic_offsets.len(),
                    dynamic_offsets_ptr,
                )
            };
            ()
        }
        pub fn write_timestamp(&self, query_set: QuerySet, query_index: u32) -> () {
            unsafe {
                ffi::wgpuComputePassEncoderWriteTimestamp(
                    self.raw,
                    query_set.as_raw(),
                    query_index,
                )
            };
            ()
        }
        pub fn dispatch_workgroups(
            &self,
            workgroup_count_x: u32,
            workgroup_count_y: u32,
            workgroup_count_z: u32,
        ) -> () {
            unsafe {
                ffi::wgpuComputePassEncoderDispatchWorkgroups(
                    self.raw,
                    workgroup_count_x,
                    workgroup_count_y,
                    workgroup_count_z,
                )
            };
            ()
        }
        pub fn dispatch_workgroups_indirect(
            &self,
            indirect_buffer: Buffer,
            indirect_offset: u64,
        ) -> () {
            unsafe {
                ffi::wgpuComputePassEncoderDispatchWorkgroupsIndirect(
                    self.raw,
                    indirect_buffer.as_raw(),
                    indirect_offset,
                )
            };
            ()
        }
        pub fn end(&self) -> () {
            unsafe { ffi::wgpuComputePassEncoderEnd(self.raw) };
            ()
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuComputePassEncoderSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn set_immediates(&self, offset: u32, data: &[std::ffi::c_void]) -> () {
            let data_ptr = data.as_ptr();
            unsafe {
                ffi::wgpuComputePassEncoderSetImmediates(
                    self.raw,
                    offset,
                    data_ptr,
                    data.len(),
                )
            };
            ()
        }
        pub fn set_resource_table(&self, table: Option<ResourceTable>) -> () {
            let table_raw = table
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            unsafe { ffi::wgpuComputePassEncoderSetResourceTable(self.raw, table_raw) };
            ()
        }
    }
    impl Drop for ComputePassEncoder {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuComputePassEncoderRelease(self.raw) };
        }
    }
    impl Clone for ComputePassEncoder {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuComputePassEncoderAddRef(self.raw) };
            Self {
                raw: self.raw,
                _not_sync: std::marker::PhantomData,
            }
        }
    }
    unsafe impl Send for ComputePassEncoder {}
    #[derive(Debug)]
    pub struct ComputePipeline {
        raw: ffi::WGPUComputePipeline,
    }
    impl ComputePipeline {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUComputePipeline) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUComputePipeline {
            self.raw
        }
        pub fn get_bind_group_layout(&self, group_index: u32) -> BindGroupLayout {
            let result = unsafe {
                ffi::wgpuComputePipelineGetBindGroupLayout(self.raw, group_index)
            };
            unsafe { BindGroupLayout::from_raw(result) }
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuComputePipelineSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for ComputePipeline {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuComputePipelineRelease(self.raw) };
        }
    }
    impl Clone for ComputePipeline {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuComputePipelineAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for ComputePipeline {}
    unsafe impl Sync for ComputePipeline {}
    #[derive(Debug)]
    pub struct Device {
        raw: ffi::WGPUDevice,
    }
    impl Device {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUDevice) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUDevice {
            self.raw
        }
        pub fn create_bind_group(&self, descriptor: &BindGroupDescriptor) -> BindGroup {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateBindGroup(self.raw, descriptor_ptr)
            };
            unsafe { BindGroup::from_raw(result) }
        }
        pub fn create_bind_group_layout(
            &self,
            descriptor: &BindGroupLayoutDescriptor,
        ) -> BindGroupLayout {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateBindGroupLayout(self.raw, descriptor_ptr)
            };
            unsafe { BindGroupLayout::from_raw(result) }
        }
        pub fn create_buffer(&self, descriptor: &BufferDescriptor) -> Option<Buffer> {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateBuffer(self.raw, descriptor_ptr)
            };
            if result.is_null() {
                None
            } else {
                Some(unsafe { Buffer::from_raw(result) })
            }
        }
        pub fn create_error_buffer(&self, descriptor: &BufferDescriptor) -> Buffer {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateErrorBuffer(self.raw, descriptor_ptr)
            };
            unsafe { Buffer::from_raw(result) }
        }
        pub fn create_command_encoder(
            &self,
            descriptor: Option<&CommandEncoderDescriptor>,
        ) -> CommandEncoder {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuDeviceCreateCommandEncoder(self.raw, descriptor_ptr)
            };
            unsafe { CommandEncoder::from_raw(result) }
        }
        pub fn create_compute_pipeline(
            &self,
            descriptor: &ComputePipelineDescriptor,
        ) -> ComputePipeline {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateComputePipeline(self.raw, descriptor_ptr)
            };
            unsafe { ComputePipeline::from_raw(result) }
        }
        pub fn create_compute_pipeline_async(
            &self,
            descriptor: &ComputePipelineDescriptor,
            callback: impl FnMut(
                CreatePipelineAsyncStatus,
                Option<ComputePipeline>,
                String,
            ) + Send + 'static,
        ) -> Future {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let callback_box: CreateComputePipelineAsyncCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPUCreateComputePipelineAsyncCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(create_compute_pipeline_async_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuDeviceCreateComputePipelineAsync(
                    self.raw,
                    descriptor_ptr,
                    callback_info_ffi,
                )
            };
            Future::from_ffi(result)
        }
        pub fn create_external_texture(
            &self,
            external_texture_descriptor: &ExternalTextureDescriptor,
        ) -> ExternalTexture {
            let (
                external_texture_descriptor_ffi,
                _external_texture_descriptor_storage,
            ) = external_texture_descriptor.to_ffi();
            let external_texture_descriptor_ptr = std::ptr::addr_of!(
                external_texture_descriptor_ffi
            );
            let result = unsafe {
                ffi::wgpuDeviceCreateExternalTexture(
                    self.raw,
                    external_texture_descriptor_ptr,
                )
            };
            unsafe { ExternalTexture::from_raw(result) }
        }
        pub fn create_error_external_texture(&self) -> ExternalTexture {
            let result = unsafe { ffi::wgpuDeviceCreateErrorExternalTexture(self.raw) };
            unsafe { ExternalTexture::from_raw(result) }
        }
        pub fn create_pipeline_layout(
            &self,
            descriptor: &PipelineLayoutDescriptor,
        ) -> PipelineLayout {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreatePipelineLayout(self.raw, descriptor_ptr)
            };
            unsafe { PipelineLayout::from_raw(result) }
        }
        pub fn create_query_set(&self, descriptor: &QuerySetDescriptor) -> QuerySet {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateQuerySet(self.raw, descriptor_ptr)
            };
            unsafe { QuerySet::from_raw(result) }
        }
        pub fn create_render_pipeline_async(
            &self,
            descriptor: &RenderPipelineDescriptor,
            callback: impl FnMut(
                CreatePipelineAsyncStatus,
                Option<RenderPipeline>,
                String,
            ) + Send + 'static,
        ) -> Future {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let callback_box: CreateRenderPipelineAsyncCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPUCreateRenderPipelineAsyncCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(create_render_pipeline_async_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuDeviceCreateRenderPipelineAsync(
                    self.raw,
                    descriptor_ptr,
                    callback_info_ffi,
                )
            };
            Future::from_ffi(result)
        }
        pub fn create_render_bundle_encoder(
            &self,
            descriptor: &RenderBundleEncoderDescriptor,
        ) -> RenderBundleEncoder {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateRenderBundleEncoder(self.raw, descriptor_ptr)
            };
            unsafe { RenderBundleEncoder::from_raw(result) }
        }
        pub fn create_render_pipeline(
            &self,
            descriptor: &RenderPipelineDescriptor,
        ) -> RenderPipeline {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateRenderPipeline(self.raw, descriptor_ptr)
            };
            unsafe { RenderPipeline::from_raw(result) }
        }
        pub fn create_sampler(&self, descriptor: Option<&SamplerDescriptor>) -> Sampler {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuDeviceCreateSampler(self.raw, descriptor_ptr)
            };
            unsafe { Sampler::from_raw(result) }
        }
        pub fn create_shader_module(
            &self,
            descriptor: &ShaderModuleDescriptor,
        ) -> ShaderModule {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateShaderModule(self.raw, descriptor_ptr)
            };
            unsafe { ShaderModule::from_raw(result) }
        }
        pub fn create_error_shader_module(
            &self,
            descriptor: &ShaderModuleDescriptor,
            error_message: String,
        ) -> ShaderModule {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let error_message_ffi = ffi::WGPUStringView {
                data: error_message.as_ptr().cast(),
                length: error_message.len(),
            };
            let result = unsafe {
                ffi::wgpuDeviceCreateErrorShaderModule(
                    self.raw,
                    descriptor_ptr,
                    error_message_ffi,
                )
            };
            unsafe { ShaderModule::from_raw(result) }
        }
        pub fn create_texture(&self, descriptor: &TextureDescriptor) -> Texture {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateTexture(self.raw, descriptor_ptr)
            };
            unsafe { Texture::from_raw(result) }
        }
        pub fn create_resource_table(
            &self,
            descriptor: &ResourceTableDescriptor,
        ) -> ResourceTable {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateResourceTable(self.raw, descriptor_ptr)
            };
            unsafe { ResourceTable::from_raw(result) }
        }
        pub fn import_shared_buffer_memory(
            &self,
            descriptor: &SharedBufferMemoryDescriptor,
        ) -> SharedBufferMemory {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceImportSharedBufferMemory(self.raw, descriptor_ptr)
            };
            unsafe { SharedBufferMemory::from_raw(result) }
        }
        pub fn import_shared_texture_memory(
            &self,
            descriptor: &SharedTextureMemoryDescriptor,
        ) -> SharedTextureMemory {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceImportSharedTextureMemory(self.raw, descriptor_ptr)
            };
            unsafe { SharedTextureMemory::from_raw(result) }
        }
        pub fn import_shared_fence(
            &self,
            descriptor: &SharedFenceDescriptor,
        ) -> SharedFence {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceImportSharedFence(self.raw, descriptor_ptr)
            };
            unsafe { SharedFence::from_raw(result) }
        }
        pub fn create_error_texture(&self, descriptor: &TextureDescriptor) -> Texture {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuDeviceCreateErrorTexture(self.raw, descriptor_ptr)
            };
            unsafe { Texture::from_raw(result) }
        }
        pub fn destroy(&self) -> () {
            unsafe { ffi::wgpuDeviceDestroy(self.raw) };
            ()
        }
        pub fn get_a_hardware_buffer_properties(
            &self,
            handle: *mut std::ffi::c_void,
            properties: &mut AHardwareBufferProperties,
        ) -> Status {
            let (mut properties_ffi, _properties_storage) = properties.to_ffi();
            let properties_ptr = std::ptr::addr_of_mut!(properties_ffi);
            let result = unsafe {
                ffi::wgpuDeviceGetAHardwareBufferProperties(
                    self.raw,
                    handle,
                    properties_ptr,
                )
            };
            *properties = AHardwareBufferProperties::from_ffi(properties_ffi);
            result.into()
        }
        pub fn get_limits(&self, limits: &mut Limits) -> Status {
            let (mut limits_ffi, _limits_storage) = limits.to_ffi();
            let limits_ptr = std::ptr::addr_of_mut!(limits_ffi);
            let result = unsafe { ffi::wgpuDeviceGetLimits(self.raw, limits_ptr) };
            *limits = Limits::from_ffi(limits_ffi);
            result.into()
        }
        pub fn get_lost_future(&self) -> Future {
            let result = unsafe { ffi::wgpuDeviceGetLostFuture(self.raw) };
            Future::from_ffi(result)
        }
        pub fn has_feature(&self, feature: FeatureName) -> bool {
            let feature_ffi: ffi::WGPUFeatureName = feature.into();
            let result = unsafe { ffi::wgpuDeviceHasFeature(self.raw, feature_ffi) };
            result != 0
        }
        pub fn get_features(&self, features: &mut SupportedFeatures) -> () {
            let (mut features_ffi, _features_storage) = features.to_ffi();
            let features_ptr = std::ptr::addr_of_mut!(features_ffi);
            unsafe { ffi::wgpuDeviceGetFeatures(self.raw, features_ptr) };
            *features = SupportedFeatures::from_ffi(features_ffi);
            ()
        }
        pub fn get_adapter_info(&self, adapter_info: &mut AdapterInfo) -> Status {
            let (mut adapter_info_ffi, _adapter_info_storage) = adapter_info.to_ffi();
            let adapter_info_ptr = std::ptr::addr_of_mut!(adapter_info_ffi);
            let result = unsafe {
                ffi::wgpuDeviceGetAdapterInfo(self.raw, adapter_info_ptr)
            };
            *adapter_info = AdapterInfo::from_ffi(adapter_info_ffi);
            result.into()
        }
        pub fn get_adapter(&self) -> Adapter {
            let result = unsafe { ffi::wgpuDeviceGetAdapter(self.raw) };
            unsafe { Adapter::from_raw(result) }
        }
        pub fn get_queue(&self) -> Queue {
            let result = unsafe { ffi::wgpuDeviceGetQueue(self.raw) };
            unsafe { Queue::from_raw(result) }
        }
        pub fn inject_error(&self, r#type: ErrorType, message: String) -> () {
            let r#type_ffi: ffi::WGPUErrorType = r#type.into();
            let message_ffi = ffi::WGPUStringView {
                data: message.as_ptr().cast(),
                length: message.len(),
            };
            unsafe { ffi::wgpuDeviceInjectError(self.raw, r#type_ffi, message_ffi) };
            ()
        }
        pub fn force_loss(&self, r#type: DeviceLostReason, message: String) -> () {
            let r#type_ffi: ffi::WGPUDeviceLostReason = r#type.into();
            let message_ffi = ffi::WGPUStringView {
                data: message.as_ptr().cast(),
                length: message.len(),
            };
            unsafe { ffi::wgpuDeviceForceLoss(self.raw, r#type_ffi, message_ffi) };
            ()
        }
        pub fn tick(&self) -> () {
            unsafe { ffi::wgpuDeviceTick(self.raw) };
            ()
        }
        pub fn set_logging_callback(
            &self,
            callback: impl FnMut(LoggingType, String) + Send + 'static,
        ) -> () {
            let callback_box: LoggingCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPULoggingCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                callback: Some(logging_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            unsafe { ffi::wgpuDeviceSetLoggingCallback(self.raw, callback_info_ffi) };
            ()
        }
        pub fn push_error_scope(&self, filter: ErrorFilter) -> () {
            let filter_ffi: ffi::WGPUErrorFilter = filter.into();
            unsafe { ffi::wgpuDevicePushErrorScope(self.raw, filter_ffi) };
            ()
        }
        pub fn pop_error_scope(
            &self,
            callback: impl FnMut(PopErrorScopeStatus, ErrorType, String) + Send + 'static,
        ) -> Future {
            let callback_box: PopErrorScopeCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPUPopErrorScopeCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(pop_error_scope_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuDevicePopErrorScope(self.raw, callback_info_ffi)
            };
            Future::from_ffi(result)
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuDeviceSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn validate_texture_descriptor(&self, descriptor: &TextureDescriptor) -> () {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            unsafe {
                ffi::wgpuDeviceValidateTextureDescriptor(self.raw, descriptor_ptr)
            };
            ()
        }
    }
    impl Drop for Device {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuDeviceRelease(self.raw) };
        }
    }
    impl Clone for Device {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuDeviceAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for Device {}
    unsafe impl Sync for Device {}
    #[derive(Debug)]
    pub struct ExternalTexture {
        raw: ffi::WGPUExternalTexture,
    }
    impl ExternalTexture {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUExternalTexture) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUExternalTexture {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuExternalTextureSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn destroy(&self) -> () {
            unsafe { ffi::wgpuExternalTextureDestroy(self.raw) };
            ()
        }
        pub fn expire(&self) -> () {
            unsafe { ffi::wgpuExternalTextureExpire(self.raw) };
            ()
        }
        pub fn refresh(&self) -> () {
            unsafe { ffi::wgpuExternalTextureRefresh(self.raw) };
            ()
        }
    }
    impl Drop for ExternalTexture {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuExternalTextureRelease(self.raw) };
        }
    }
    impl Clone for ExternalTexture {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuExternalTextureAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for ExternalTexture {}
    unsafe impl Sync for ExternalTexture {}
    #[derive(Debug)]
    pub struct Instance {
        raw: ffi::WGPUInstance,
        _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    }
    impl Instance {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUInstance) -> Self {
            Self {
                raw,
                _not_sync: std::marker::PhantomData,
            }
        }
        pub fn as_raw(&self) -> ffi::WGPUInstance {
            self.raw
        }
        pub fn new(descriptor: Option<&InstanceDescriptor>) -> Self {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe { ffi::wgpuCreateInstance(descriptor_ptr) };
            unsafe { Instance::from_raw(result) }
        }
        pub fn create_surface(&self, descriptor: &SurfaceDescriptor) -> Surface {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuInstanceCreateSurface(self.raw, descriptor_ptr)
            };
            unsafe { Surface::from_raw(result) }
        }
        pub fn process_events(&self) -> () {
            unsafe { ffi::wgpuInstanceProcessEvents(self.raw) };
            ()
        }
        pub fn wait_any(
            &self,
            mut futures: Option<&mut [FutureWaitInfo]>,
            timeout_ns: u64,
        ) -> WaitStatus {
            let mut futures_raw: Vec<ffi::WGPUFutureWaitInfo> = Vec::new();
            let mut futures_storage: Vec<ChainedStructStorage> = Vec::new();
            let futures_ptr = if let Some(value) = futures.as_deref() {
                for item in value {
                    let (raw, storage) = item.to_ffi();
                    futures_raw.push(raw);
                    futures_storage.push(storage);
                }
                futures_raw.as_mut_ptr()
            } else {
                std::ptr::null_mut()
            };
            let result = unsafe {
                ffi::wgpuInstanceWaitAny(
                    self.raw,
                    futures.as_deref().map(|v| v.len()).unwrap_or(0),
                    futures_ptr,
                    timeout_ns,
                )
            };
            result.into()
        }
        pub fn request_adapter(
            &self,
            options: Option<&RequestAdapterOptions>,
            callback: impl FnMut(
                RequestAdapterStatus,
                Option<Adapter>,
                String,
            ) + Send + 'static,
        ) -> Future {
            let mut options_storage = ChainedStructStorage::new();
            let options_ptr = if let Some(value) = &options {
                let (options_ffi, storage) = value.to_ffi();
                options_storage = storage;
                std::ptr::addr_of!(options_ffi)
            } else {
                std::ptr::null()
            };
            let callback_box: RequestAdapterCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPURequestAdapterCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(request_adapter_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuInstanceRequestAdapter(self.raw, options_ptr, callback_info_ffi)
            };
            Future::from_ffi(result)
        }
        pub fn has_wgsl_language_feature(
            &self,
            feature: WGSLLanguageFeatureName,
        ) -> bool {
            let feature_ffi: ffi::WGPUWGSLLanguageFeatureName = feature.into();
            let result = unsafe {
                ffi::wgpuInstanceHasWGSLLanguageFeature(self.raw, feature_ffi)
            };
            result != 0
        }
        pub fn get_wgsl_language_features(
            &self,
            features: &mut SupportedWGSLLanguageFeatures,
        ) -> () {
            let (mut features_ffi, _features_storage) = features.to_ffi();
            let features_ptr = std::ptr::addr_of_mut!(features_ffi);
            unsafe { ffi::wgpuInstanceGetWGSLLanguageFeatures(self.raw, features_ptr) };
            *features = SupportedWGSLLanguageFeatures::from_ffi(features_ffi);
            ()
        }
    }
    impl Drop for Instance {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuInstanceRelease(self.raw) };
        }
    }
    impl Clone for Instance {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuInstanceAddRef(self.raw) };
            Self {
                raw: self.raw,
                _not_sync: std::marker::PhantomData,
            }
        }
    }
    unsafe impl Send for Instance {}
    #[derive(Debug)]
    pub struct PipelineLayout {
        raw: ffi::WGPUPipelineLayout,
    }
    impl PipelineLayout {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUPipelineLayout) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUPipelineLayout {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuPipelineLayoutSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for PipelineLayout {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuPipelineLayoutRelease(self.raw) };
        }
    }
    impl Clone for PipelineLayout {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuPipelineLayoutAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for PipelineLayout {}
    unsafe impl Sync for PipelineLayout {}
    #[derive(Debug)]
    pub struct QuerySet {
        raw: ffi::WGPUQuerySet,
    }
    impl QuerySet {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUQuerySet) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUQuerySet {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuQuerySetSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn get_type(&self) -> QueryType {
            let result = unsafe { ffi::wgpuQuerySetGetType(self.raw) };
            result.into()
        }
        pub fn get_count(&self) -> u32 {
            let result = unsafe { ffi::wgpuQuerySetGetCount(self.raw) };
            result
        }
        pub fn destroy(&self) -> () {
            unsafe { ffi::wgpuQuerySetDestroy(self.raw) };
            ()
        }
    }
    impl Drop for QuerySet {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuQuerySetRelease(self.raw) };
        }
    }
    impl Clone for QuerySet {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuQuerySetAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for QuerySet {}
    unsafe impl Sync for QuerySet {}
    #[derive(Debug)]
    pub struct Queue {
        raw: ffi::WGPUQueue,
    }
    impl Queue {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUQueue) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUQueue {
            self.raw
        }
        pub fn submit(&self, commands: &[CommandBuffer]) -> () {
            let mut commands_raw: Vec<ffi::WGPUCommandBuffer> = commands
                .iter()
                .map(|v| v.as_raw())
                .collect();
            let commands_ptr = commands_raw.as_ptr();
            unsafe { ffi::wgpuQueueSubmit(self.raw, commands.len(), commands_ptr) };
            ()
        }
        pub fn on_submitted_work_done(
            &self,
            callback: impl FnMut(QueueWorkDoneStatus, String) + Send + 'static,
        ) -> Future {
            let callback_box: QueueWorkDoneCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPUQueueWorkDoneCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(queue_work_done_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuQueueOnSubmittedWorkDone(self.raw, callback_info_ffi)
            };
            Future::from_ffi(result)
        }
        pub fn write_buffer(
            &self,
            buffer: Buffer,
            buffer_offset: u64,
            data: &[std::ffi::c_void],
        ) -> () {
            let data_ptr = data.as_ptr();
            unsafe {
                ffi::wgpuQueueWriteBuffer(
                    self.raw,
                    buffer.as_raw(),
                    buffer_offset,
                    data_ptr,
                    data.len(),
                )
            };
            ()
        }
        pub fn write_texture(
            &self,
            destination: &TexelCopyTextureInfo,
            data: &[std::ffi::c_void],
            data_layout: &TexelCopyBufferLayout,
            write_size: &Extent3D,
        ) -> () {
            let (destination_ffi, _destination_storage) = destination.to_ffi();
            let destination_ptr = std::ptr::addr_of!(destination_ffi);
            let data_ptr = data.as_ptr();
            let (data_layout_ffi, _data_layout_storage) = data_layout.to_ffi();
            let data_layout_ptr = std::ptr::addr_of!(data_layout_ffi);
            let (write_size_ffi, _write_size_storage) = write_size.to_ffi();
            let write_size_ptr = std::ptr::addr_of!(write_size_ffi);
            unsafe {
                ffi::wgpuQueueWriteTexture(
                    self.raw,
                    destination_ptr,
                    data_ptr,
                    data.len(),
                    data_layout_ptr,
                    write_size_ptr,
                )
            };
            ()
        }
        pub fn copy_texture_for_browser(
            &self,
            source: &TexelCopyTextureInfo,
            destination: &TexelCopyTextureInfo,
            copy_size: &Extent3D,
            options: &CopyTextureForBrowserOptions,
        ) -> () {
            let (source_ffi, _source_storage) = source.to_ffi();
            let source_ptr = std::ptr::addr_of!(source_ffi);
            let (destination_ffi, _destination_storage) = destination.to_ffi();
            let destination_ptr = std::ptr::addr_of!(destination_ffi);
            let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
            let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
            let (options_ffi, _options_storage) = options.to_ffi();
            let options_ptr = std::ptr::addr_of!(options_ffi);
            unsafe {
                ffi::wgpuQueueCopyTextureForBrowser(
                    self.raw,
                    source_ptr,
                    destination_ptr,
                    copy_size_ptr,
                    options_ptr,
                )
            };
            ()
        }
        pub fn copy_external_texture_for_browser(
            &self,
            source: &ImageCopyExternalTexture,
            destination: &TexelCopyTextureInfo,
            copy_size: &Extent3D,
            options: &CopyTextureForBrowserOptions,
        ) -> () {
            let (source_ffi, _source_storage) = source.to_ffi();
            let source_ptr = std::ptr::addr_of!(source_ffi);
            let (destination_ffi, _destination_storage) = destination.to_ffi();
            let destination_ptr = std::ptr::addr_of!(destination_ffi);
            let (copy_size_ffi, _copy_size_storage) = copy_size.to_ffi();
            let copy_size_ptr = std::ptr::addr_of!(copy_size_ffi);
            let (options_ffi, _options_storage) = options.to_ffi();
            let options_ptr = std::ptr::addr_of!(options_ffi);
            unsafe {
                ffi::wgpuQueueCopyExternalTextureForBrowser(
                    self.raw,
                    source_ptr,
                    destination_ptr,
                    copy_size_ptr,
                    options_ptr,
                )
            };
            ()
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuQueueSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for Queue {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuQueueRelease(self.raw) };
        }
    }
    impl Clone for Queue {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuQueueAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for Queue {}
    unsafe impl Sync for Queue {}
    #[derive(Debug)]
    pub struct RenderBundle {
        raw: ffi::WGPURenderBundle,
    }
    impl RenderBundle {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderBundle) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPURenderBundle {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuRenderBundleSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for RenderBundle {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuRenderBundleRelease(self.raw) };
        }
    }
    impl Clone for RenderBundle {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuRenderBundleAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for RenderBundle {}
    unsafe impl Sync for RenderBundle {}
    #[derive(Debug)]
    pub struct RenderBundleEncoder {
        raw: ffi::WGPURenderBundleEncoder,
        _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    }
    impl RenderBundleEncoder {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderBundleEncoder) -> Self {
            Self {
                raw,
                _not_sync: std::marker::PhantomData,
            }
        }
        pub fn as_raw(&self) -> ffi::WGPURenderBundleEncoder {
            self.raw
        }
        pub fn set_pipeline(&self, pipeline: RenderPipeline) -> () {
            unsafe {
                ffi::wgpuRenderBundleEncoderSetPipeline(self.raw, pipeline.as_raw())
            };
            ()
        }
        pub fn set_bind_group(
            &self,
            group_index: u32,
            group: Option<BindGroup>,
            dynamic_offsets: &[u32],
        ) -> () {
            let group_raw = group
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            let dynamic_offsets_ptr = dynamic_offsets.as_ptr();
            unsafe {
                ffi::wgpuRenderBundleEncoderSetBindGroup(
                    self.raw,
                    group_index,
                    group_raw,
                    dynamic_offsets.len(),
                    dynamic_offsets_ptr,
                )
            };
            ()
        }
        pub fn draw(
            &self,
            vertex_count: u32,
            instance_count: u32,
            first_vertex: u32,
            first_instance: u32,
        ) -> () {
            unsafe {
                ffi::wgpuRenderBundleEncoderDraw(
                    self.raw,
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                )
            };
            ()
        }
        pub fn draw_indexed(
            &self,
            index_count: u32,
            instance_count: u32,
            first_index: u32,
            base_vertex: i32,
            first_instance: u32,
        ) -> () {
            unsafe {
                ffi::wgpuRenderBundleEncoderDrawIndexed(
                    self.raw,
                    index_count,
                    instance_count,
                    first_index,
                    base_vertex,
                    first_instance,
                )
            };
            ()
        }
        pub fn draw_indirect(
            &self,
            indirect_buffer: Buffer,
            indirect_offset: u64,
        ) -> () {
            unsafe {
                ffi::wgpuRenderBundleEncoderDrawIndirect(
                    self.raw,
                    indirect_buffer.as_raw(),
                    indirect_offset,
                )
            };
            ()
        }
        pub fn draw_indexed_indirect(
            &self,
            indirect_buffer: Buffer,
            indirect_offset: u64,
        ) -> () {
            unsafe {
                ffi::wgpuRenderBundleEncoderDrawIndexedIndirect(
                    self.raw,
                    indirect_buffer.as_raw(),
                    indirect_offset,
                )
            };
            ()
        }
        pub fn insert_debug_marker(&self, marker_label: String) -> () {
            let marker_label_ffi = ffi::WGPUStringView {
                data: marker_label.as_ptr().cast(),
                length: marker_label.len(),
            };
            unsafe {
                ffi::wgpuRenderBundleEncoderInsertDebugMarker(self.raw, marker_label_ffi)
            };
            ()
        }
        pub fn pop_debug_group(&self) -> () {
            unsafe { ffi::wgpuRenderBundleEncoderPopDebugGroup(self.raw) };
            ()
        }
        pub fn push_debug_group(&self, group_label: String) -> () {
            let group_label_ffi = ffi::WGPUStringView {
                data: group_label.as_ptr().cast(),
                length: group_label.len(),
            };
            unsafe {
                ffi::wgpuRenderBundleEncoderPushDebugGroup(self.raw, group_label_ffi)
            };
            ()
        }
        pub fn set_vertex_buffer(
            &self,
            slot: u32,
            buffer: Option<Buffer>,
            offset: u64,
            size: u64,
        ) -> () {
            let buffer_raw = buffer
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            unsafe {
                ffi::wgpuRenderBundleEncoderSetVertexBuffer(
                    self.raw,
                    slot,
                    buffer_raw,
                    offset,
                    size,
                )
            };
            ()
        }
        pub fn set_index_buffer(
            &self,
            buffer: Buffer,
            format: IndexFormat,
            offset: u64,
            size: u64,
        ) -> () {
            let format_ffi: ffi::WGPUIndexFormat = format.into();
            unsafe {
                ffi::wgpuRenderBundleEncoderSetIndexBuffer(
                    self.raw,
                    buffer.as_raw(),
                    format_ffi,
                    offset,
                    size,
                )
            };
            ()
        }
        pub fn finish(
            &self,
            descriptor: Option<&RenderBundleDescriptor>,
        ) -> RenderBundle {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuRenderBundleEncoderFinish(self.raw, descriptor_ptr)
            };
            unsafe { RenderBundle::from_raw(result) }
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuRenderBundleEncoderSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn set_immediates(&self, offset: u32, data: &[std::ffi::c_void]) -> () {
            let data_ptr = data.as_ptr();
            unsafe {
                ffi::wgpuRenderBundleEncoderSetImmediates(
                    self.raw,
                    offset,
                    data_ptr,
                    data.len(),
                )
            };
            ()
        }
        pub fn set_resource_table(&self, table: Option<ResourceTable>) -> () {
            let table_raw = table
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            unsafe { ffi::wgpuRenderBundleEncoderSetResourceTable(self.raw, table_raw) };
            ()
        }
    }
    impl Drop for RenderBundleEncoder {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuRenderBundleEncoderRelease(self.raw) };
        }
    }
    impl Clone for RenderBundleEncoder {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuRenderBundleEncoderAddRef(self.raw) };
            Self {
                raw: self.raw,
                _not_sync: std::marker::PhantomData,
            }
        }
    }
    unsafe impl Send for RenderBundleEncoder {}
    #[derive(Debug)]
    pub struct RenderPassEncoder {
        raw: ffi::WGPURenderPassEncoder,
        _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    }
    impl RenderPassEncoder {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderPassEncoder) -> Self {
            Self {
                raw,
                _not_sync: std::marker::PhantomData,
            }
        }
        pub fn as_raw(&self) -> ffi::WGPURenderPassEncoder {
            self.raw
        }
        pub fn set_pipeline(&self, pipeline: RenderPipeline) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderSetPipeline(self.raw, pipeline.as_raw())
            };
            ()
        }
        pub fn set_bind_group(
            &self,
            group_index: u32,
            group: Option<BindGroup>,
            dynamic_offsets: &[u32],
        ) -> () {
            let group_raw = group
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            let dynamic_offsets_ptr = dynamic_offsets.as_ptr();
            unsafe {
                ffi::wgpuRenderPassEncoderSetBindGroup(
                    self.raw,
                    group_index,
                    group_raw,
                    dynamic_offsets.len(),
                    dynamic_offsets_ptr,
                )
            };
            ()
        }
        pub fn draw(
            &self,
            vertex_count: u32,
            instance_count: u32,
            first_vertex: u32,
            first_instance: u32,
        ) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderDraw(
                    self.raw,
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                )
            };
            ()
        }
        pub fn draw_indexed(
            &self,
            index_count: u32,
            instance_count: u32,
            first_index: u32,
            base_vertex: i32,
            first_instance: u32,
        ) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderDrawIndexed(
                    self.raw,
                    index_count,
                    instance_count,
                    first_index,
                    base_vertex,
                    first_instance,
                )
            };
            ()
        }
        pub fn draw_indirect(
            &self,
            indirect_buffer: Buffer,
            indirect_offset: u64,
        ) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderDrawIndirect(
                    self.raw,
                    indirect_buffer.as_raw(),
                    indirect_offset,
                )
            };
            ()
        }
        pub fn draw_indexed_indirect(
            &self,
            indirect_buffer: Buffer,
            indirect_offset: u64,
        ) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderDrawIndexedIndirect(
                    self.raw,
                    indirect_buffer.as_raw(),
                    indirect_offset,
                )
            };
            ()
        }
        pub fn multi_draw_indirect(
            &self,
            indirect_buffer: Buffer,
            indirect_offset: u64,
            max_draw_count: u32,
            draw_count_buffer: Option<Buffer>,
            draw_count_buffer_offset: u64,
        ) -> () {
            let draw_count_buffer_raw = draw_count_buffer
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            unsafe {
                ffi::wgpuRenderPassEncoderMultiDrawIndirect(
                    self.raw,
                    indirect_buffer.as_raw(),
                    indirect_offset,
                    max_draw_count,
                    draw_count_buffer_raw,
                    draw_count_buffer_offset,
                )
            };
            ()
        }
        pub fn multi_draw_indexed_indirect(
            &self,
            indirect_buffer: Buffer,
            indirect_offset: u64,
            max_draw_count: u32,
            draw_count_buffer: Option<Buffer>,
            draw_count_buffer_offset: u64,
        ) -> () {
            let draw_count_buffer_raw = draw_count_buffer
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            unsafe {
                ffi::wgpuRenderPassEncoderMultiDrawIndexedIndirect(
                    self.raw,
                    indirect_buffer.as_raw(),
                    indirect_offset,
                    max_draw_count,
                    draw_count_buffer_raw,
                    draw_count_buffer_offset,
                )
            };
            ()
        }
        pub fn execute_bundles(&self, bundles: &[RenderBundle]) -> () {
            let mut bundles_raw: Vec<ffi::WGPURenderBundle> = bundles
                .iter()
                .map(|v| v.as_raw())
                .collect();
            let bundles_ptr = bundles_raw.as_ptr();
            unsafe {
                ffi::wgpuRenderPassEncoderExecuteBundles(
                    self.raw,
                    bundles.len(),
                    bundles_ptr,
                )
            };
            ()
        }
        pub fn insert_debug_marker(&self, marker_label: String) -> () {
            let marker_label_ffi = ffi::WGPUStringView {
                data: marker_label.as_ptr().cast(),
                length: marker_label.len(),
            };
            unsafe {
                ffi::wgpuRenderPassEncoderInsertDebugMarker(self.raw, marker_label_ffi)
            };
            ()
        }
        pub fn pop_debug_group(&self) -> () {
            unsafe { ffi::wgpuRenderPassEncoderPopDebugGroup(self.raw) };
            ()
        }
        pub fn push_debug_group(&self, group_label: String) -> () {
            let group_label_ffi = ffi::WGPUStringView {
                data: group_label.as_ptr().cast(),
                length: group_label.len(),
            };
            unsafe {
                ffi::wgpuRenderPassEncoderPushDebugGroup(self.raw, group_label_ffi)
            };
            ()
        }
        pub fn set_stencil_reference(&self, reference: u32) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderSetStencilReference(self.raw, reference)
            };
            ()
        }
        pub fn set_blend_constant(&self, color: &Color) -> () {
            let (color_ffi, _color_storage) = color.to_ffi();
            let color_ptr = std::ptr::addr_of!(color_ffi);
            unsafe { ffi::wgpuRenderPassEncoderSetBlendConstant(self.raw, color_ptr) };
            ()
        }
        pub fn set_viewport(
            &self,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            min_depth: f32,
            max_depth: f32,
        ) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderSetViewport(
                    self.raw,
                    x,
                    y,
                    width,
                    height,
                    min_depth,
                    max_depth,
                )
            };
            ()
        }
        pub fn set_scissor_rect(&self, x: u32, y: u32, width: u32, height: u32) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderSetScissorRect(self.raw, x, y, width, height)
            };
            ()
        }
        pub fn set_vertex_buffer(
            &self,
            slot: u32,
            buffer: Option<Buffer>,
            offset: u64,
            size: u64,
        ) -> () {
            let buffer_raw = buffer
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            unsafe {
                ffi::wgpuRenderPassEncoderSetVertexBuffer(
                    self.raw,
                    slot,
                    buffer_raw,
                    offset,
                    size,
                )
            };
            ()
        }
        pub fn set_index_buffer(
            &self,
            buffer: Buffer,
            format: IndexFormat,
            offset: u64,
            size: u64,
        ) -> () {
            let format_ffi: ffi::WGPUIndexFormat = format.into();
            unsafe {
                ffi::wgpuRenderPassEncoderSetIndexBuffer(
                    self.raw,
                    buffer.as_raw(),
                    format_ffi,
                    offset,
                    size,
                )
            };
            ()
        }
        pub fn begin_occlusion_query(&self, query_index: u32) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderBeginOcclusionQuery(self.raw, query_index)
            };
            ()
        }
        pub fn end_occlusion_query(&self) -> () {
            unsafe { ffi::wgpuRenderPassEncoderEndOcclusionQuery(self.raw) };
            ()
        }
        pub fn write_timestamp(&self, query_set: QuerySet, query_index: u32) -> () {
            unsafe {
                ffi::wgpuRenderPassEncoderWriteTimestamp(
                    self.raw,
                    query_set.as_raw(),
                    query_index,
                )
            };
            ()
        }
        pub fn pixel_local_storage_barrier(&self) -> () {
            unsafe { ffi::wgpuRenderPassEncoderPixelLocalStorageBarrier(self.raw) };
            ()
        }
        pub fn end(&self) -> () {
            unsafe { ffi::wgpuRenderPassEncoderEnd(self.raw) };
            ()
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuRenderPassEncoderSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn set_immediates(&self, offset: u32, data: &[std::ffi::c_void]) -> () {
            let data_ptr = data.as_ptr();
            unsafe {
                ffi::wgpuRenderPassEncoderSetImmediates(
                    self.raw,
                    offset,
                    data_ptr,
                    data.len(),
                )
            };
            ()
        }
        pub fn set_resource_table(&self, table: Option<ResourceTable>) -> () {
            let table_raw = table
                .as_ref()
                .map(|v| v.as_raw())
                .unwrap_or(std::ptr::null_mut());
            unsafe { ffi::wgpuRenderPassEncoderSetResourceTable(self.raw, table_raw) };
            ()
        }
    }
    impl Drop for RenderPassEncoder {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuRenderPassEncoderRelease(self.raw) };
        }
    }
    impl Clone for RenderPassEncoder {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuRenderPassEncoderAddRef(self.raw) };
            Self {
                raw: self.raw,
                _not_sync: std::marker::PhantomData,
            }
        }
    }
    unsafe impl Send for RenderPassEncoder {}
    #[derive(Debug)]
    pub struct RenderPipeline {
        raw: ffi::WGPURenderPipeline,
    }
    impl RenderPipeline {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPURenderPipeline) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPURenderPipeline {
            self.raw
        }
        pub fn get_bind_group_layout(&self, group_index: u32) -> BindGroupLayout {
            let result = unsafe {
                ffi::wgpuRenderPipelineGetBindGroupLayout(self.raw, group_index)
            };
            unsafe { BindGroupLayout::from_raw(result) }
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuRenderPipelineSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for RenderPipeline {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuRenderPipelineRelease(self.raw) };
        }
    }
    impl Clone for RenderPipeline {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuRenderPipelineAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for RenderPipeline {}
    unsafe impl Sync for RenderPipeline {}
    #[derive(Debug)]
    pub struct ResourceTable {
        raw: ffi::WGPUResourceTable,
    }
    impl ResourceTable {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUResourceTable) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUResourceTable {
            self.raw
        }
        pub fn get_size(&self) -> u32 {
            let result = unsafe { ffi::wgpuResourceTableGetSize(self.raw) };
            result
        }
        pub fn destroy(&self) -> () {
            unsafe { ffi::wgpuResourceTableDestroy(self.raw) };
            ()
        }
        pub fn update(&self, slot: u32, resource: &BindingResource) -> Status {
            let (resource_ffi, _resource_storage) = resource.to_ffi();
            let resource_ptr = std::ptr::addr_of!(resource_ffi);
            let result = unsafe {
                ffi::wgpuResourceTableUpdate(self.raw, slot, resource_ptr)
            };
            result.into()
        }
        pub fn insert_binding(&self, resource: &BindingResource) -> u32 {
            let (resource_ffi, _resource_storage) = resource.to_ffi();
            let resource_ptr = std::ptr::addr_of!(resource_ffi);
            let result = unsafe {
                ffi::wgpuResourceTableInsertBinding(self.raw, resource_ptr)
            };
            result
        }
        pub fn remove_binding(&self, slot: u32) -> Status {
            let result = unsafe { ffi::wgpuResourceTableRemoveBinding(self.raw, slot) };
            result.into()
        }
    }
    impl Drop for ResourceTable {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuResourceTableRelease(self.raw) };
        }
    }
    impl Clone for ResourceTable {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuResourceTableAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for ResourceTable {}
    unsafe impl Sync for ResourceTable {}
    #[derive(Debug)]
    pub struct Sampler {
        raw: ffi::WGPUSampler,
    }
    impl Sampler {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUSampler) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUSampler {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuSamplerSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for Sampler {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuSamplerRelease(self.raw) };
        }
    }
    impl Clone for Sampler {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuSamplerAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for Sampler {}
    unsafe impl Sync for Sampler {}
    #[derive(Debug)]
    pub struct ShaderModule {
        raw: ffi::WGPUShaderModule,
    }
    impl ShaderModule {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUShaderModule) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUShaderModule {
            self.raw
        }
        pub fn get_compilation_info(
            &self,
            callback: impl FnMut(
                CompilationInfoRequestStatus,
                &CompilationInfo,
            ) + Send + 'static,
        ) -> Future {
            let callback_box: CompilationInfoCallback = Box::new(callback);
            let callback_box = Box::new(Some(callback_box));
            let callback_userdata = Box::into_raw(callback_box)
                .cast::<std::ffi::c_void>();
            let callback_info_ffi = ffi::WGPUCompilationInfoCallbackInfo {
                nextInChain: std::ptr::null_mut(),
                mode: ffi::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
                callback: Some(compilation_info_callback_trampoline),
                userdata1: callback_userdata,
                userdata2: std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::wgpuShaderModuleGetCompilationInfo(self.raw, callback_info_ffi)
            };
            Future::from_ffi(result)
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuShaderModuleSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for ShaderModule {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuShaderModuleRelease(self.raw) };
        }
    }
    impl Clone for ShaderModule {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuShaderModuleAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for ShaderModule {}
    unsafe impl Sync for ShaderModule {}
    #[derive(Debug)]
    pub struct SharedBufferMemory {
        raw: ffi::WGPUSharedBufferMemory,
    }
    impl SharedBufferMemory {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUSharedBufferMemory) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUSharedBufferMemory {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuSharedBufferMemorySetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn get_properties(
            &self,
            properties: &mut SharedBufferMemoryProperties,
        ) -> Status {
            let (mut properties_ffi, _properties_storage) = properties.to_ffi();
            let properties_ptr = std::ptr::addr_of_mut!(properties_ffi);
            let result = unsafe {
                ffi::wgpuSharedBufferMemoryGetProperties(self.raw, properties_ptr)
            };
            *properties = SharedBufferMemoryProperties::from_ffi(properties_ffi);
            result.into()
        }
        pub fn create_buffer(&self, descriptor: Option<&BufferDescriptor>) -> Buffer {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuSharedBufferMemoryCreateBuffer(self.raw, descriptor_ptr)
            };
            unsafe { Buffer::from_raw(result) }
        }
        pub fn begin_access(
            &self,
            buffer: Buffer,
            descriptor: &SharedBufferMemoryBeginAccessDescriptor,
        ) -> Status {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuSharedBufferMemoryBeginAccess(
                    self.raw,
                    buffer.as_raw(),
                    descriptor_ptr,
                )
            };
            result.into()
        }
        pub fn end_access(
            &self,
            buffer: Buffer,
            descriptor: &mut SharedBufferMemoryEndAccessState,
        ) -> Status {
            let (mut descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of_mut!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuSharedBufferMemoryEndAccess(
                    self.raw,
                    buffer.as_raw(),
                    descriptor_ptr,
                )
            };
            *descriptor = SharedBufferMemoryEndAccessState::from_ffi(descriptor_ffi);
            result.into()
        }
        pub fn is_device_lost(&self) -> bool {
            let result = unsafe { ffi::wgpuSharedBufferMemoryIsDeviceLost(self.raw) };
            result != 0
        }
    }
    impl Drop for SharedBufferMemory {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuSharedBufferMemoryRelease(self.raw) };
        }
    }
    impl Clone for SharedBufferMemory {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuSharedBufferMemoryAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for SharedBufferMemory {}
    unsafe impl Sync for SharedBufferMemory {}
    #[derive(Debug)]
    pub struct SharedFence {
        raw: ffi::WGPUSharedFence,
    }
    impl SharedFence {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUSharedFence) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUSharedFence {
            self.raw
        }
        pub fn export_info(&self, info: &mut SharedFenceExportInfo) -> () {
            let (mut info_ffi, _info_storage) = info.to_ffi();
            let info_ptr = std::ptr::addr_of_mut!(info_ffi);
            unsafe { ffi::wgpuSharedFenceExportInfo(self.raw, info_ptr) };
            *info = SharedFenceExportInfo::from_ffi(info_ffi);
            ()
        }
    }
    impl Drop for SharedFence {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuSharedFenceRelease(self.raw) };
        }
    }
    impl Clone for SharedFence {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuSharedFenceAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for SharedFence {}
    unsafe impl Sync for SharedFence {}
    #[derive(Debug)]
    pub struct SharedTextureMemory {
        raw: ffi::WGPUSharedTextureMemory,
    }
    impl SharedTextureMemory {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUSharedTextureMemory) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUSharedTextureMemory {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuSharedTextureMemorySetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn get_properties(
            &self,
            properties: &mut SharedTextureMemoryProperties,
        ) -> Status {
            let (mut properties_ffi, _properties_storage) = properties.to_ffi();
            let properties_ptr = std::ptr::addr_of_mut!(properties_ffi);
            let result = unsafe {
                ffi::wgpuSharedTextureMemoryGetProperties(self.raw, properties_ptr)
            };
            *properties = SharedTextureMemoryProperties::from_ffi(properties_ffi);
            result.into()
        }
        pub fn create_texture(&self, descriptor: Option<&TextureDescriptor>) -> Texture {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuSharedTextureMemoryCreateTexture(self.raw, descriptor_ptr)
            };
            unsafe { Texture::from_raw(result) }
        }
        pub fn begin_access(
            &self,
            texture: Texture,
            descriptor: &SharedTextureMemoryBeginAccessDescriptor,
        ) -> Status {
            let (descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuSharedTextureMemoryBeginAccess(
                    self.raw,
                    texture.as_raw(),
                    descriptor_ptr,
                )
            };
            result.into()
        }
        pub fn end_access(
            &self,
            texture: Texture,
            descriptor: &mut SharedTextureMemoryEndAccessState,
        ) -> Status {
            let (mut descriptor_ffi, _descriptor_storage) = descriptor.to_ffi();
            let descriptor_ptr = std::ptr::addr_of_mut!(descriptor_ffi);
            let result = unsafe {
                ffi::wgpuSharedTextureMemoryEndAccess(
                    self.raw,
                    texture.as_raw(),
                    descriptor_ptr,
                )
            };
            *descriptor = SharedTextureMemoryEndAccessState::from_ffi(descriptor_ffi);
            result.into()
        }
        pub fn is_device_lost(&self) -> bool {
            let result = unsafe { ffi::wgpuSharedTextureMemoryIsDeviceLost(self.raw) };
            result != 0
        }
    }
    impl Drop for SharedTextureMemory {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuSharedTextureMemoryRelease(self.raw) };
        }
    }
    impl Clone for SharedTextureMemory {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuSharedTextureMemoryAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for SharedTextureMemory {}
    unsafe impl Sync for SharedTextureMemory {}
    #[derive(Debug)]
    pub struct Surface {
        raw: ffi::WGPUSurface,
        _not_sync: std::marker::PhantomData<std::cell::Cell<()>>,
    }
    impl Surface {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUSurface) -> Self {
            Self {
                raw,
                _not_sync: std::marker::PhantomData,
            }
        }
        pub fn as_raw(&self) -> ffi::WGPUSurface {
            self.raw
        }
        pub fn configure(&self, config: &SurfaceConfiguration) -> () {
            let (config_ffi, _config_storage) = config.to_ffi();
            let config_ptr = std::ptr::addr_of!(config_ffi);
            unsafe { ffi::wgpuSurfaceConfigure(self.raw, config_ptr) };
            ()
        }
        pub fn get_capabilities(
            &self,
            adapter: Adapter,
            capabilities: &mut SurfaceCapabilities,
        ) -> Status {
            let (mut capabilities_ffi, _capabilities_storage) = capabilities.to_ffi();
            let capabilities_ptr = std::ptr::addr_of_mut!(capabilities_ffi);
            let result = unsafe {
                ffi::wgpuSurfaceGetCapabilities(
                    self.raw,
                    adapter.as_raw(),
                    capabilities_ptr,
                )
            };
            *capabilities = SurfaceCapabilities::from_ffi(capabilities_ffi);
            result.into()
        }
        pub fn get_current_texture(&self, surface_texture: &mut SurfaceTexture) -> () {
            let (mut surface_texture_ffi, _surface_texture_storage) = surface_texture
                .to_ffi();
            let surface_texture_ptr = std::ptr::addr_of_mut!(surface_texture_ffi);
            unsafe { ffi::wgpuSurfaceGetCurrentTexture(self.raw, surface_texture_ptr) };
            *surface_texture = SurfaceTexture::from_ffi(surface_texture_ffi);
            ()
        }
        pub fn present(&self) -> Status {
            let result = unsafe { ffi::wgpuSurfacePresent(self.raw) };
            result.into()
        }
        pub fn unconfigure(&self) -> () {
            unsafe { ffi::wgpuSurfaceUnconfigure(self.raw) };
            ()
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuSurfaceSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for Surface {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuSurfaceRelease(self.raw) };
        }
    }
    impl Clone for Surface {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuSurfaceAddRef(self.raw) };
            Self {
                raw: self.raw,
                _not_sync: std::marker::PhantomData,
            }
        }
    }
    unsafe impl Send for Surface {}
    #[derive(Debug)]
    pub struct TexelBufferView {
        raw: ffi::WGPUTexelBufferView,
    }
    impl TexelBufferView {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUTexelBufferView) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUTexelBufferView {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuTexelBufferViewSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for TexelBufferView {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuTexelBufferViewRelease(self.raw) };
        }
    }
    impl Clone for TexelBufferView {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuTexelBufferViewAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for TexelBufferView {}
    unsafe impl Sync for TexelBufferView {}
    #[derive(Debug)]
    pub struct Texture {
        raw: ffi::WGPUTexture,
    }
    impl Texture {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUTexture) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUTexture {
            self.raw
        }
        pub fn create_view(
            &self,
            descriptor: Option<&TextureViewDescriptor>,
        ) -> TextureView {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe { ffi::wgpuTextureCreateView(self.raw, descriptor_ptr) };
            unsafe { TextureView::from_raw(result) }
        }
        pub fn create_error_view(
            &self,
            descriptor: Option<&TextureViewDescriptor>,
        ) -> TextureView {
            let mut descriptor_storage = ChainedStructStorage::new();
            let descriptor_ptr = if let Some(value) = &descriptor {
                let (descriptor_ffi, storage) = value.to_ffi();
                descriptor_storage = storage;
                std::ptr::addr_of!(descriptor_ffi)
            } else {
                std::ptr::null()
            };
            let result = unsafe {
                ffi::wgpuTextureCreateErrorView(self.raw, descriptor_ptr)
            };
            unsafe { TextureView::from_raw(result) }
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuTextureSetLabel(self.raw, label_ffi) };
            ()
        }
        pub fn get_width(&self) -> u32 {
            let result = unsafe { ffi::wgpuTextureGetWidth(self.raw) };
            result
        }
        pub fn get_height(&self) -> u32 {
            let result = unsafe { ffi::wgpuTextureGetHeight(self.raw) };
            result
        }
        pub fn get_depth_or_array_layers(&self) -> u32 {
            let result = unsafe { ffi::wgpuTextureGetDepthOrArrayLayers(self.raw) };
            result
        }
        pub fn get_mip_level_count(&self) -> u32 {
            let result = unsafe { ffi::wgpuTextureGetMipLevelCount(self.raw) };
            result
        }
        pub fn get_sample_count(&self) -> u32 {
            let result = unsafe { ffi::wgpuTextureGetSampleCount(self.raw) };
            result
        }
        pub fn get_dimension(&self) -> TextureDimension {
            let result = unsafe { ffi::wgpuTextureGetDimension(self.raw) };
            result.into()
        }
        pub fn get_format(&self) -> TextureFormat {
            let result = unsafe { ffi::wgpuTextureGetFormat(self.raw) };
            result.into()
        }
        pub fn get_usage(&self) -> TextureUsage {
            let result = unsafe { ffi::wgpuTextureGetUsage(self.raw) };
            result.into()
        }
        pub fn get_texture_binding_view_dimension(&self) -> TextureViewDimension {
            let result = unsafe {
                ffi::wgpuTextureGetTextureBindingViewDimension(self.raw)
            };
            result.into()
        }
        pub fn destroy(&self) -> () {
            unsafe { ffi::wgpuTextureDestroy(self.raw) };
            ()
        }
        pub fn pin(&self, usage: TextureUsage) -> () {
            let usage_ffi: ffi::WGPUTextureUsage = usage.into();
            unsafe { ffi::wgpuTexturePin(self.raw, usage_ffi) };
            ()
        }
        pub fn unpin(&self) -> () {
            unsafe { ffi::wgpuTextureUnpin(self.raw) };
            ()
        }
        pub fn set_ownership_for_memory_dump(&self, owner_guid: u64) -> () {
            unsafe { ffi::wgpuTextureSetOwnershipForMemoryDump(self.raw, owner_guid) };
            ()
        }
    }
    impl Drop for Texture {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuTextureRelease(self.raw) };
        }
    }
    impl Clone for Texture {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuTextureAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for Texture {}
    unsafe impl Sync for Texture {}
    #[derive(Debug)]
    pub struct TextureView {
        raw: ffi::WGPUTextureView,
    }
    impl TextureView {
        pub(crate) unsafe fn from_raw(raw: ffi::WGPUTextureView) -> Self {
            Self { raw }
        }
        pub fn as_raw(&self) -> ffi::WGPUTextureView {
            self.raw
        }
        pub fn set_label(&self, label: String) -> () {
            let label_ffi = ffi::WGPUStringView {
                data: label.as_ptr().cast(),
                length: label.len(),
            };
            unsafe { ffi::wgpuTextureViewSetLabel(self.raw, label_ffi) };
            ()
        }
    }
    impl Drop for TextureView {
        fn drop(&mut self) {
            if self.as_raw().is_null() {
                return;
            }
            unsafe { ffi::wgpuTextureViewRelease(self.raw) };
        }
    }
    impl Clone for TextureView {
        fn clone(&self) -> Self {
            unsafe { ffi::wgpuTextureViewAddRef(self.raw) };
            Self { raw: self.raw }
        }
    }
    unsafe impl Send for TextureView {}
    unsafe impl Sync for TextureView {}
}
mod callbacks {
    #![allow(dead_code, unused_imports)]
    use crate::ffi;
    use crate::generated::*;
    pub type Callback = Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>;
    pub type DawnLoadCacheDataFunction = Option<
        unsafe extern "C" fn(
            *const std::ffi::c_void,
            usize,
            *mut std::ffi::c_void,
            usize,
            *mut std::ffi::c_void,
        ) -> usize,
    >;
    pub type DawnStoreCacheDataFunction = Option<
        unsafe extern "C" fn(
            *const std::ffi::c_void,
            usize,
            *const std::ffi::c_void,
            usize,
            *mut std::ffi::c_void,
        ) -> (),
    >;
    pub type Proc = Option<unsafe extern "C" fn() -> ()>;
    pub type BufferMapCallback = Box<dyn FnMut(MapAsyncStatus, String) + Send + 'static>;
    pub(crate) unsafe extern "C" fn buffer_map_callback_trampoline(
        status: ffi::WGPUMapAsyncStatus,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<BufferMapCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, message);
        }
    }
    pub type CompilationInfoCallback = Box<
        dyn FnMut(CompilationInfoRequestStatus, &CompilationInfo) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn compilation_info_callback_trampoline(
        status: ffi::WGPUCompilationInfoRequestStatus,
        compilation_info: *const ffi::WGPUCompilationInfo,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let compilation_info = if compilation_info.is_null() {
            CompilationInfo::new()
        } else {
            CompilationInfo::new()
        };
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<CompilationInfoCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, &compilation_info);
        }
    }
    pub type CreateComputePipelineAsyncCallback = Box<
        dyn FnMut(
            CreatePipelineAsyncStatus,
            Option<ComputePipeline>,
            String,
        ) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn create_compute_pipeline_async_callback_trampoline(
        status: ffi::WGPUCreatePipelineAsyncStatus,
        pipeline: ffi::WGPUComputePipeline,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let pipeline = if pipeline.is_null() {
            None
        } else {
            Some(unsafe { ComputePipeline::from_raw(pipeline) })
        };
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<CreateComputePipelineAsyncCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, pipeline, message);
        }
    }
    pub type CreateRenderPipelineAsyncCallback = Box<
        dyn FnMut(
            CreatePipelineAsyncStatus,
            Option<RenderPipeline>,
            String,
        ) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn create_render_pipeline_async_callback_trampoline(
        status: ffi::WGPUCreatePipelineAsyncStatus,
        pipeline: ffi::WGPURenderPipeline,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let pipeline = if pipeline.is_null() {
            None
        } else {
            Some(unsafe { RenderPipeline::from_raw(pipeline) })
        };
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<CreateRenderPipelineAsyncCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, pipeline, message);
        }
    }
    pub type DeviceLostCallback = Box<
        dyn FnMut(Vec<Device>, DeviceLostReason, String) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn device_lost_callback_trampoline(
        device: *const ffi::WGPUDevice,
        reason: ffi::WGPUDeviceLostReason,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let device = if device.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(device, 1) }
                .iter()
                .map(|raw| unsafe { Device::from_raw(*raw) })
                .collect()
        };
        let reason = reason.into();
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<DeviceLostCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(device, reason, message);
        }
    }
    pub type LoggingCallback = Box<dyn FnMut(LoggingType, String) + Send + 'static>;
    pub(crate) unsafe extern "C" fn logging_callback_trampoline(
        r#type: ffi::WGPULoggingType,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let r#type = r#type.into();
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<LoggingCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(r#type, message);
        }
    }
    pub type PopErrorScopeCallback = Box<
        dyn FnMut(PopErrorScopeStatus, ErrorType, String) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn pop_error_scope_callback_trampoline(
        status: ffi::WGPUPopErrorScopeStatus,
        r#type: ffi::WGPUErrorType,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let r#type = r#type.into();
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<PopErrorScopeCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, r#type, message);
        }
    }
    pub type QueueWorkDoneCallback = Box<
        dyn FnMut(QueueWorkDoneStatus, String) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn queue_work_done_callback_trampoline(
        status: ffi::WGPUQueueWorkDoneStatus,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<QueueWorkDoneCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, message);
        }
    }
    pub type RequestAdapterCallback = Box<
        dyn FnMut(RequestAdapterStatus, Option<Adapter>, String) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn request_adapter_callback_trampoline(
        status: ffi::WGPURequestAdapterStatus,
        adapter: ffi::WGPUAdapter,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let adapter = if adapter.is_null() {
            None
        } else {
            Some(unsafe { Adapter::from_raw(adapter) })
        };
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<RequestAdapterCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, adapter, message);
        }
    }
    pub type RequestDeviceCallback = Box<
        dyn FnMut(RequestDeviceStatus, Option<Device>, String) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn request_device_callback_trampoline(
        status: ffi::WGPURequestDeviceStatus,
        device: ffi::WGPUDevice,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let status = status.into();
        let device = if device.is_null() {
            None
        } else {
            Some(unsafe { Device::from_raw(device) })
        };
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<RequestDeviceCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(status, device, message);
        }
    }
    pub type UncapturedErrorCallback = Box<
        dyn FnMut(Vec<Device>, ErrorType, String) + Send + 'static,
    >;
    pub(crate) unsafe extern "C" fn uncaptured_error_callback_trampoline(
        device: *const ffi::WGPUDevice,
        r#type: ffi::WGPUErrorType,
        message: ffi::WGPUStringView,
        userdata1: *mut std::ffi::c_void,
        userdata2: *mut std::ffi::c_void,
    ) {
        let _ = userdata2;
        let device = if device.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(device, 1) }
                .iter()
                .map(|raw| unsafe { Device::from_raw(*raw) })
                .collect()
        };
        let r#type = r#type.into();
        let message = string_view_to_string(message);
        let mut callback = unsafe {
            Box::from_raw(userdata1.cast::<Option<UncapturedErrorCallback>>())
        };
        if let Some(mut callback) = callback.take() {
            callback(device, r#type, message);
        }
    }
    pub struct BufferMapCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<BufferMapCallback>>,
    }
    impl Default for BufferMapCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl BufferMapCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct CompilationInfoCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<CompilationInfoCallback>>,
    }
    impl Default for CompilationInfoCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl CompilationInfoCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct CreateComputePipelineAsyncCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<CreateComputePipelineAsyncCallback>>,
    }
    impl Default for CreateComputePipelineAsyncCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl CreateComputePipelineAsyncCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct CreateRenderPipelineAsyncCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<CreateRenderPipelineAsyncCallback>>,
    }
    impl Default for CreateRenderPipelineAsyncCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl CreateRenderPipelineAsyncCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct DeviceLostCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<DeviceLostCallback>>,
    }
    impl Default for DeviceLostCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl DeviceLostCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct LoggingCallbackInfo {
        pub callback: std::cell::RefCell<Option<LoggingCallback>>,
    }
    impl Default for LoggingCallbackInfo {
        fn default() -> Self {
            Self {
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl LoggingCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct PopErrorScopeCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<PopErrorScopeCallback>>,
    }
    impl Default for PopErrorScopeCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl PopErrorScopeCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct QueueWorkDoneCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<QueueWorkDoneCallback>>,
    }
    impl Default for QueueWorkDoneCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl QueueWorkDoneCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct RequestAdapterCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<RequestAdapterCallback>>,
    }
    impl Default for RequestAdapterCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl RequestAdapterCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct RequestDeviceCallbackInfo {
        pub mode: Option<CallbackMode>,
        pub callback: std::cell::RefCell<Option<RequestDeviceCallback>>,
    }
    impl Default for RequestDeviceCallbackInfo {
        fn default() -> Self {
            Self {
                mode: None,
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl RequestDeviceCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub struct UncapturedErrorCallbackInfo {
        pub callback: std::cell::RefCell<Option<UncapturedErrorCallback>>,
    }
    impl Default for UncapturedErrorCallbackInfo {
        fn default() -> Self {
            Self {
                callback: std::cell::RefCell::new(None),
            }
        }
    }
    impl UncapturedErrorCallbackInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
mod functions {
    #![allow(dead_code, unused_imports)]
    use crate::generated::*;
    use crate::ffi;
    pub fn create_instance(descriptor: Option<&InstanceDescriptor>) -> Instance {
        let mut descriptor_storage = ChainedStructStorage::new();
        let descriptor_ptr = if let Some(value) = &descriptor {
            let (descriptor_ffi, storage) = value.to_ffi();
            descriptor_storage = storage;
            std::ptr::addr_of!(descriptor_ffi)
        } else {
            std::ptr::null()
        };
        let result = unsafe { ffi::wgpuCreateInstance(descriptor_ptr) };
        unsafe { Instance::from_raw(result) }
    }
    pub fn get_instance_features(features: &mut SupportedInstanceFeatures) -> () {
        let (mut features_ffi, _features_storage) = features.to_ffi();
        let features_ptr = std::ptr::addr_of_mut!(features_ffi);
        unsafe { ffi::wgpuGetInstanceFeatures(features_ptr) };
        *features = SupportedInstanceFeatures::from_ffi(features_ffi);
        ()
    }
    pub fn get_instance_limits(limits: &mut InstanceLimits) -> Status {
        let (mut limits_ffi, _limits_storage) = limits.to_ffi();
        let limits_ptr = std::ptr::addr_of_mut!(limits_ffi);
        let result = unsafe { ffi::wgpuGetInstanceLimits(limits_ptr) };
        *limits = InstanceLimits::from_ffi(limits_ffi);
        result.into()
    }
    pub fn get_proc_address(proc_name: String) -> Proc {
        let proc_name_ffi = ffi::WGPUStringView {
            data: proc_name.as_ptr().cast(),
            length: proc_name.len(),
        };
        let result = unsafe { ffi::wgpuGetProcAddress(proc_name_ffi) };
        result
    }
    pub fn has_instance_feature(feature: InstanceFeatureName) -> bool {
        let feature_ffi: ffi::WGPUInstanceFeatureName = feature.into();
        let result = unsafe { ffi::wgpuHasInstanceFeature(feature_ffi) };
        result != 0
    }
}
mod constants {
    #![allow(dead_code, unused_imports)]
    use crate::generated::*;
    use crate::ffi;
    pub const ARRAY_LAYER_COUNT_UNDEFINED: u32 = u32::MAX;
    pub const COPY_STRIDE_UNDEFINED: u32 = u32::MAX;
    pub const DEPTH_CLEAR_VALUE_UNDEFINED: f32 = f32::NAN;
    pub const DEPTH_SLICE_UNDEFINED: u32 = u32::MAX;
    pub const INVALID_BINDING: u32 = u32::MAX;
    pub const LIMIT_U32_UNDEFINED: u32 = u32::MAX;
    pub const LIMIT_U64_UNDEFINED: u64 = u64::MAX;
    pub const MIP_LEVEL_COUNT_UNDEFINED: u32 = u32::MAX;
    pub const QUERY_SET_INDEX_UNDEFINED: u32 = u32::MAX;
    pub const STRLEN: usize = usize::MAX;
    pub const WHOLE_MAP_SIZE: usize = usize::MAX;
    pub const WHOLE_SIZE: u64 = u64::MAX;
}
pub use enums::*;
pub use structs::*;
pub use extensions::*;
pub use objects::*;
pub use callbacks::*;
pub use functions::*;
pub use constants::*;
