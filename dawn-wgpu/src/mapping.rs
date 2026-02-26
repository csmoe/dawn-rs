use crate::dispatch::*;
use crate::error::DawnError;
use dawn_rs::*;

pub(crate) fn label_to_string(label: wgpu::Label<'_>) -> Option<String> {
    label.map(|s| s.to_string())
}

pub(crate) fn map_backend_type_to_wgpu(value: BackendType) -> wgpu::Backend {
    match value {
        BackendType::Vulkan => wgpu::Backend::Vulkan,
        BackendType::Metal => wgpu::Backend::Metal,
        BackendType::D3D12 => wgpu::Backend::Dx12,
        BackendType::D3D11 => wgpu::Backend::Noop,
        BackendType::OpenGL | BackendType::OpenGLes => wgpu::Backend::Gl,
        BackendType::WebGPU => wgpu::Backend::BrowserWebGpu,
        _ => wgpu::Backend::Noop,
    }
}

pub(crate) fn map_power_preference(value: wgpu::PowerPreference) -> PowerPreference {
    match value {
        wgpu::PowerPreference::None => PowerPreference::Undefined,
        wgpu::PowerPreference::LowPower => PowerPreference::LowPower,
        wgpu::PowerPreference::HighPerformance => PowerPreference::HighPerformance,
        _ => PowerPreference::Undefined,
    }
}

pub(crate) fn map_texture_format(value: wgpu::TextureFormat) -> TextureFormat {
    use wgpu::TextureFormat as W;
    match value {
        W::R8Unorm => TextureFormat::R8Unorm,
        W::R8Snorm => TextureFormat::R8Snorm,
        W::R8Uint => TextureFormat::R8Uint,
        W::R8Sint => TextureFormat::R8Sint,
        W::R16Unorm => TextureFormat::R16Unorm,
        W::R16Snorm => TextureFormat::R16Snorm,
        W::R16Uint => TextureFormat::R16Uint,
        W::R16Sint => TextureFormat::R16Sint,
        W::R16Float => TextureFormat::R16Float,
        W::Rg8Unorm => TextureFormat::Rg8Unorm,
        W::Rg8Snorm => TextureFormat::Rg8Snorm,
        W::Rg8Uint => TextureFormat::Rg8Uint,
        W::Rg8Sint => TextureFormat::Rg8Sint,
        W::R32Float => TextureFormat::R32Float,
        W::R32Uint => TextureFormat::R32Uint,
        W::R32Sint => TextureFormat::R32Sint,
        W::Rg16Unorm => TextureFormat::Rg16Unorm,
        W::Rg16Snorm => TextureFormat::Rg16Snorm,
        W::Rg16Uint => TextureFormat::Rg16Uint,
        W::Rg16Sint => TextureFormat::Rg16Sint,
        W::Rg16Float => TextureFormat::Rg16Float,
        W::Rgba8Unorm => TextureFormat::Rgba8Unorm,
        W::Rgba8UnormSrgb => TextureFormat::Rgba8UnormSrgb,
        W::Rgba8Snorm => TextureFormat::Rgba8Snorm,
        W::Rgba8Uint => TextureFormat::Rgba8Uint,
        W::Rgba8Sint => TextureFormat::Rgba8Sint,
        W::Bgra8Unorm => TextureFormat::Bgra8Unorm,
        W::Bgra8UnormSrgb => TextureFormat::Bgra8UnormSrgb,
        W::Rgb10a2Unorm => TextureFormat::Rgb10A2Unorm,
        W::Rgb10a2Uint => TextureFormat::Rgb10A2Uint,
        W::Rg11b10Ufloat => TextureFormat::Rg11B10Ufloat,
        W::Rgb9e5Ufloat => TextureFormat::Rgb9E5Ufloat,
        W::Rg32Float => TextureFormat::Rg32Float,
        W::Rg32Uint => TextureFormat::Rg32Uint,
        W::Rg32Sint => TextureFormat::Rg32Sint,
        W::Rgba16Unorm => TextureFormat::Rgba16Unorm,
        W::Rgba16Snorm => TextureFormat::Rgba16Snorm,
        W::Rgba16Uint => TextureFormat::Rgba16Uint,
        W::Rgba16Sint => TextureFormat::Rgba16Sint,
        W::Rgba16Float => TextureFormat::Rgba16Float,
        W::Rgba32Float => TextureFormat::Rgba32Float,
        W::Rgba32Uint => TextureFormat::Rgba32Uint,
        W::Rgba32Sint => TextureFormat::Rgba32Sint,
        W::Stencil8 => TextureFormat::Stencil8,
        W::Depth16Unorm => TextureFormat::Depth16Unorm,
        W::Depth24Plus => TextureFormat::Depth24Plus,
        W::Depth24PlusStencil8 => TextureFormat::Depth24PlusStencil8,
        W::Depth32Float => TextureFormat::Depth32Float,
        W::Depth32FloatStencil8 => TextureFormat::Depth32FloatStencil8,
        W::Bc1RgbaUnorm => TextureFormat::Bc1RgbaUnorm,
        W::Bc1RgbaUnormSrgb => TextureFormat::Bc1RgbaUnormSrgb,
        W::Bc2RgbaUnorm => TextureFormat::Bc2RgbaUnorm,
        W::Bc2RgbaUnormSrgb => TextureFormat::Bc2RgbaUnormSrgb,
        W::Bc3RgbaUnorm => TextureFormat::Bc3RgbaUnorm,
        W::Bc3RgbaUnormSrgb => TextureFormat::Bc3RgbaUnormSrgb,
        W::Bc4RUnorm => TextureFormat::Bc4RUnorm,
        W::Bc4RSnorm => TextureFormat::Bc4RSnorm,
        W::Bc5RgUnorm => TextureFormat::Bc5RgUnorm,
        W::Bc5RgSnorm => TextureFormat::Bc5RgSnorm,
        W::Bc6hRgbUfloat => TextureFormat::Bc6HRgbUfloat,
        W::Bc6hRgbFloat => TextureFormat::Bc6HRgbFloat,
        W::Bc7RgbaUnorm => TextureFormat::Bc7RgbaUnorm,
        W::Bc7RgbaUnormSrgb => TextureFormat::Bc7RgbaUnormSrgb,
        W::Etc2Rgb8Unorm => TextureFormat::Etc2Rgb8Unorm,
        W::Etc2Rgb8UnormSrgb => TextureFormat::Etc2Rgb8UnormSrgb,
        W::Etc2Rgb8A1Unorm => TextureFormat::Etc2Rgb8A1Unorm,
        W::Etc2Rgb8A1UnormSrgb => TextureFormat::Etc2Rgb8A1UnormSrgb,
        W::Etc2Rgba8Unorm => TextureFormat::Etc2Rgba8Unorm,
        W::Etc2Rgba8UnormSrgb => TextureFormat::Etc2Rgba8UnormSrgb,
        W::EacR11Unorm => TextureFormat::EacR11Unorm,
        W::EacR11Snorm => TextureFormat::EacR11Snorm,
        W::EacRg11Unorm => TextureFormat::EacRg11Unorm,
        W::EacRg11Snorm => TextureFormat::EacRg11Snorm,
        W::Astc { block, channel } => match (block, channel) {
            (wgpu::AstcBlock::B4x4, wgpu::AstcChannel::Unorm) => TextureFormat::Astc4X4Unorm,
            (wgpu::AstcBlock::B4x4, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc4X4UnormSrgb
            }
            (wgpu::AstcBlock::B5x4, wgpu::AstcChannel::Unorm) => TextureFormat::Astc5X4Unorm,
            (wgpu::AstcBlock::B5x4, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc5X4UnormSrgb
            }
            (wgpu::AstcBlock::B5x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc5X5Unorm,
            (wgpu::AstcBlock::B5x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc5X5UnormSrgb
            }
            (wgpu::AstcBlock::B6x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc6X5Unorm,
            (wgpu::AstcBlock::B6x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc6X5UnormSrgb
            }
            (wgpu::AstcBlock::B6x6, wgpu::AstcChannel::Unorm) => TextureFormat::Astc6X6Unorm,
            (wgpu::AstcBlock::B6x6, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc6X6UnormSrgb
            }
            (wgpu::AstcBlock::B8x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc8X5Unorm,
            (wgpu::AstcBlock::B8x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc8X5UnormSrgb
            }
            (wgpu::AstcBlock::B8x6, wgpu::AstcChannel::Unorm) => TextureFormat::Astc8X6Unorm,
            (wgpu::AstcBlock::B8x6, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc8X6UnormSrgb
            }
            (wgpu::AstcBlock::B8x8, wgpu::AstcChannel::Unorm) => TextureFormat::Astc8X8Unorm,
            (wgpu::AstcBlock::B8x8, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc8X8UnormSrgb
            }
            (wgpu::AstcBlock::B10x5, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X5Unorm,
            (wgpu::AstcBlock::B10x5, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X5UnormSrgb
            }
            (wgpu::AstcBlock::B10x6, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X6Unorm,
            (wgpu::AstcBlock::B10x6, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X6UnormSrgb
            }
            (wgpu::AstcBlock::B10x8, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X8Unorm,
            (wgpu::AstcBlock::B10x8, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X8UnormSrgb
            }
            (wgpu::AstcBlock::B10x10, wgpu::AstcChannel::Unorm) => TextureFormat::Astc10X10Unorm,
            (wgpu::AstcBlock::B10x10, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc10X10UnormSrgb
            }
            (wgpu::AstcBlock::B12x10, wgpu::AstcChannel::Unorm) => TextureFormat::Astc12X10Unorm,
            (wgpu::AstcBlock::B12x10, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc12X10UnormSrgb
            }
            (wgpu::AstcBlock::B12x12, wgpu::AstcChannel::Unorm) => TextureFormat::Astc12X12Unorm,
            (wgpu::AstcBlock::B12x12, wgpu::AstcChannel::UnormSrgb) => {
                TextureFormat::Astc12X12UnormSrgb
            }
            _ => panic!("wgpu-compat: unsupported ASTC format"),
        },
        _ => panic!("wgpu-compat: unsupported texture format"),
    }
}

pub(crate) fn map_texture_format_to_wgpu(value: TextureFormat) -> wgpu::TextureFormat {
    use wgpu::TextureFormat as W;
    match value {
        TextureFormat::R8Unorm => W::R8Unorm,
        TextureFormat::R8Snorm => W::R8Snorm,
        TextureFormat::R8Uint => W::R8Uint,
        TextureFormat::R8Sint => W::R8Sint,
        TextureFormat::R16Unorm => W::R16Unorm,
        TextureFormat::R16Snorm => W::R16Snorm,
        TextureFormat::R16Uint => W::R16Uint,
        TextureFormat::R16Sint => W::R16Sint,
        TextureFormat::R16Float => W::R16Float,
        TextureFormat::Rg8Unorm => W::Rg8Unorm,
        TextureFormat::Rg8Snorm => W::Rg8Snorm,
        TextureFormat::Rg8Uint => W::Rg8Uint,
        TextureFormat::Rg8Sint => W::Rg8Sint,
        TextureFormat::R32Float => W::R32Float,
        TextureFormat::R32Uint => W::R32Uint,
        TextureFormat::R32Sint => W::R32Sint,
        TextureFormat::Rg16Unorm => W::Rg16Unorm,
        TextureFormat::Rg16Snorm => W::Rg16Snorm,
        TextureFormat::Rg16Uint => W::Rg16Uint,
        TextureFormat::Rg16Sint => W::Rg16Sint,
        TextureFormat::Rg16Float => W::Rg16Float,
        TextureFormat::Rgba8Unorm => W::Rgba8Unorm,
        TextureFormat::Rgba8UnormSrgb => W::Rgba8UnormSrgb,
        TextureFormat::Rgba8Snorm => W::Rgba8Snorm,
        TextureFormat::Rgba8Uint => W::Rgba8Uint,
        TextureFormat::Rgba8Sint => W::Rgba8Sint,
        TextureFormat::Bgra8Unorm => W::Bgra8Unorm,
        TextureFormat::Bgra8UnormSrgb => W::Bgra8UnormSrgb,
        TextureFormat::Rgb10A2Uint => W::Rgb10a2Uint,
        TextureFormat::Rgb10A2Unorm => W::Rgb10a2Unorm,
        TextureFormat::Rg11B10Ufloat => W::Rg11b10Ufloat,
        TextureFormat::Rgb9E5Ufloat => W::Rgb9e5Ufloat,
        TextureFormat::Rg32Float => W::Rg32Float,
        TextureFormat::Rg32Uint => W::Rg32Uint,
        TextureFormat::Rg32Sint => W::Rg32Sint,
        TextureFormat::Rgba16Unorm => W::Rgba16Unorm,
        TextureFormat::Rgba16Snorm => W::Rgba16Snorm,
        TextureFormat::Rgba16Uint => W::Rgba16Uint,
        TextureFormat::Rgba16Sint => W::Rgba16Sint,
        TextureFormat::Rgba16Float => W::Rgba16Float,
        TextureFormat::Rgba32Float => W::Rgba32Float,
        TextureFormat::Rgba32Uint => W::Rgba32Uint,
        TextureFormat::Rgba32Sint => W::Rgba32Sint,
        TextureFormat::Stencil8 => W::Stencil8,
        TextureFormat::Depth16Unorm => W::Depth16Unorm,
        TextureFormat::Depth24Plus => W::Depth24Plus,
        TextureFormat::Depth24PlusStencil8 => W::Depth24PlusStencil8,
        TextureFormat::Depth32Float => W::Depth32Float,
        TextureFormat::Depth32FloatStencil8 => W::Depth32FloatStencil8,
        TextureFormat::Bc1RgbaUnorm => W::Bc1RgbaUnorm,
        TextureFormat::Bc1RgbaUnormSrgb => W::Bc1RgbaUnormSrgb,
        TextureFormat::Bc2RgbaUnorm => W::Bc2RgbaUnorm,
        TextureFormat::Bc2RgbaUnormSrgb => W::Bc2RgbaUnormSrgb,
        TextureFormat::Bc3RgbaUnorm => W::Bc3RgbaUnorm,
        TextureFormat::Bc3RgbaUnormSrgb => W::Bc3RgbaUnormSrgb,
        TextureFormat::Bc4RUnorm => W::Bc4RUnorm,
        TextureFormat::Bc4RSnorm => W::Bc4RSnorm,
        TextureFormat::Bc5RgUnorm => W::Bc5RgUnorm,
        TextureFormat::Bc5RgSnorm => W::Bc5RgSnorm,
        TextureFormat::Bc6HRgbUfloat => W::Bc6hRgbUfloat,
        TextureFormat::Bc6HRgbFloat => W::Bc6hRgbFloat,
        TextureFormat::Bc7RgbaUnorm => W::Bc7RgbaUnorm,
        TextureFormat::Bc7RgbaUnormSrgb => W::Bc7RgbaUnormSrgb,
        TextureFormat::Etc2Rgb8Unorm => W::Etc2Rgb8Unorm,
        TextureFormat::Etc2Rgb8UnormSrgb => W::Etc2Rgb8UnormSrgb,
        TextureFormat::Etc2Rgb8A1Unorm => W::Etc2Rgb8A1Unorm,
        TextureFormat::Etc2Rgb8A1UnormSrgb => W::Etc2Rgb8A1UnormSrgb,
        TextureFormat::Etc2Rgba8Unorm => W::Etc2Rgba8Unorm,
        TextureFormat::Etc2Rgba8UnormSrgb => W::Etc2Rgba8UnormSrgb,
        TextureFormat::EacR11Unorm => W::EacR11Unorm,
        TextureFormat::EacR11Snorm => W::EacR11Snorm,
        TextureFormat::EacRg11Unorm => W::EacRg11Unorm,
        TextureFormat::EacRg11Snorm => W::EacRg11Snorm,
        TextureFormat::Astc4X4Unorm => W::Astc {
            block: wgpu::AstcBlock::B4x4,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc4X4UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B4x4,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc5X4Unorm => W::Astc {
            block: wgpu::AstcBlock::B5x4,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc5X4UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B5x4,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc5X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B5x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc5X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B5x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc6X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B6x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc6X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B6x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc6X6Unorm => W::Astc {
            block: wgpu::AstcBlock::B6x6,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc6X6UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B6x6,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc8X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B8x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc8X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B8x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc8X6Unorm => W::Astc {
            block: wgpu::AstcBlock::B8x6,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc8X6UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B8x6,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc8X8Unorm => W::Astc {
            block: wgpu::AstcBlock::B8x8,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc8X8UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B8x8,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X5Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x5,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X5UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x5,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X6Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x6,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X6UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x6,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X8Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x8,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X8UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x8,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc10X10Unorm => W::Astc {
            block: wgpu::AstcBlock::B10x10,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc10X10UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B10x10,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc12X10Unorm => W::Astc {
            block: wgpu::AstcBlock::B12x10,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc12X10UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B12x10,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        TextureFormat::Astc12X12Unorm => W::Astc {
            block: wgpu::AstcBlock::B12x12,
            channel: wgpu::AstcChannel::Unorm,
        },
        TextureFormat::Astc12X12UnormSrgb => W::Astc {
            block: wgpu::AstcBlock::B12x12,
            channel: wgpu::AstcChannel::UnormSrgb,
        },
        _ => panic!("wgpu-compat: unsupported texture format conversion"),
    }
}

pub(crate) fn map_texture_dimension(value: wgpu::TextureDimension) -> TextureDimension {
    match value {
        wgpu::TextureDimension::D1 => TextureDimension::D1,
        wgpu::TextureDimension::D2 => TextureDimension::D2,
        wgpu::TextureDimension::D3 => TextureDimension::D3,
        _ => TextureDimension::D2,
    }
}

pub(crate) fn map_texture_view_dimension(
    value: wgpu::TextureViewDimension,
) -> TextureViewDimension {
    match value {
        wgpu::TextureViewDimension::D1 => TextureViewDimension::D1,
        wgpu::TextureViewDimension::D2 => TextureViewDimension::D2,
        wgpu::TextureViewDimension::D2Array => TextureViewDimension::D2Array,
        wgpu::TextureViewDimension::Cube => TextureViewDimension::Cube,
        wgpu::TextureViewDimension::CubeArray => TextureViewDimension::CubeArray,
        wgpu::TextureViewDimension::D3 => TextureViewDimension::D3,
        _ => TextureViewDimension::D2,
    }
}

pub(crate) fn map_texture_aspect(value: wgpu::TextureAspect) -> TextureAspect {
    match value {
        wgpu::TextureAspect::All => TextureAspect::All,
        wgpu::TextureAspect::StencilOnly => TextureAspect::StencilOnly,
        wgpu::TextureAspect::DepthOnly => TextureAspect::DepthOnly,
        wgpu::TextureAspect::Plane0 => TextureAspect::Plane0Only,
        wgpu::TextureAspect::Plane1 => TextureAspect::Plane1Only,
        wgpu::TextureAspect::Plane2 => TextureAspect::Plane2Only,
        _ => TextureAspect::All,
    }
}

pub(crate) fn map_filter_mode(value: wgpu::FilterMode) -> FilterMode {
    match value {
        wgpu::FilterMode::Nearest => FilterMode::Nearest,
        wgpu::FilterMode::Linear => FilterMode::Linear,
        _ => FilterMode::Nearest,
    }
}

pub(crate) fn map_mipmap_filter_mode(value: wgpu::MipmapFilterMode) -> MipmapFilterMode {
    match value {
        wgpu::MipmapFilterMode::Nearest => MipmapFilterMode::Nearest,
        wgpu::MipmapFilterMode::Linear => MipmapFilterMode::Linear,
        _ => MipmapFilterMode::Nearest,
    }
}

pub(crate) fn map_address_mode(value: wgpu::AddressMode) -> AddressMode {
    match value {
        wgpu::AddressMode::ClampToEdge => AddressMode::ClampToEdge,
        wgpu::AddressMode::Repeat => AddressMode::Repeat,
        wgpu::AddressMode::MirrorRepeat => AddressMode::MirrorRepeat,
        _ => AddressMode::ClampToEdge,
    }
}

pub(crate) fn map_compare_function(value: wgpu::CompareFunction) -> CompareFunction {
    match value {
        wgpu::CompareFunction::Never => CompareFunction::Never,
        wgpu::CompareFunction::Less => CompareFunction::Less,
        wgpu::CompareFunction::Equal => CompareFunction::Equal,
        wgpu::CompareFunction::LessEqual => CompareFunction::LessEqual,
        wgpu::CompareFunction::Greater => CompareFunction::Greater,
        wgpu::CompareFunction::NotEqual => CompareFunction::NotEqual,
        wgpu::CompareFunction::GreaterEqual => CompareFunction::GreaterEqual,
        wgpu::CompareFunction::Always => CompareFunction::Always,
        _ => CompareFunction::Always,
    }
}

pub(crate) fn map_index_format(value: wgpu::IndexFormat) -> IndexFormat {
    match value {
        wgpu::IndexFormat::Uint16 => IndexFormat::Uint16,
        wgpu::IndexFormat::Uint32 => IndexFormat::Uint32,
        _ => IndexFormat::Uint32,
    }
}

pub(crate) fn map_primitive_topology(value: wgpu::PrimitiveTopology) -> PrimitiveTopology {
    match value {
        wgpu::PrimitiveTopology::PointList => PrimitiveTopology::PointList,
        wgpu::PrimitiveTopology::LineList => PrimitiveTopology::LineList,
        wgpu::PrimitiveTopology::LineStrip => PrimitiveTopology::LineStrip,
        wgpu::PrimitiveTopology::TriangleList => PrimitiveTopology::TriangleList,
        wgpu::PrimitiveTopology::TriangleStrip => PrimitiveTopology::TriangleStrip,
        _ => PrimitiveTopology::TriangleList,
    }
}

pub(crate) fn map_front_face(value: wgpu::FrontFace) -> FrontFace {
    match value {
        wgpu::FrontFace::Ccw => FrontFace::Ccw,
        wgpu::FrontFace::Cw => FrontFace::Cw,
        _ => FrontFace::Ccw,
    }
}

pub(crate) fn map_cull_mode(value: Option<wgpu::Face>) -> CullMode {
    match value {
        Some(wgpu::Face::Front) => CullMode::Front,
        Some(wgpu::Face::Back) => CullMode::Back,
        None => CullMode::None,
    }
}

pub(crate) fn map_stencil_operation(value: wgpu::StencilOperation) -> StencilOperation {
    match value {
        wgpu::StencilOperation::Keep => StencilOperation::Keep,
        wgpu::StencilOperation::Zero => StencilOperation::Zero,
        wgpu::StencilOperation::Replace => StencilOperation::Replace,
        wgpu::StencilOperation::Invert => StencilOperation::Invert,
        wgpu::StencilOperation::IncrementClamp => StencilOperation::IncrementClamp,
        wgpu::StencilOperation::DecrementClamp => StencilOperation::DecrementClamp,
        wgpu::StencilOperation::IncrementWrap => StencilOperation::IncrementWrap,
        wgpu::StencilOperation::DecrementWrap => StencilOperation::DecrementWrap,
        _ => StencilOperation::Keep,
    }
}

pub(crate) fn map_blend_operation(value: wgpu::BlendOperation) -> BlendOperation {
    match value {
        wgpu::BlendOperation::Add => BlendOperation::Add,
        wgpu::BlendOperation::Subtract => BlendOperation::Subtract,
        wgpu::BlendOperation::ReverseSubtract => BlendOperation::ReverseSubtract,
        wgpu::BlendOperation::Min => BlendOperation::Min,
        wgpu::BlendOperation::Max => BlendOperation::Max,
        _ => BlendOperation::Add,
    }
}

pub(crate) fn map_blend_factor(value: wgpu::BlendFactor) -> BlendFactor {
    match value {
        wgpu::BlendFactor::Zero => BlendFactor::Zero,
        wgpu::BlendFactor::One => BlendFactor::One,
        wgpu::BlendFactor::Src => BlendFactor::Src,
        wgpu::BlendFactor::OneMinusSrc => BlendFactor::OneMinusSrc,
        wgpu::BlendFactor::SrcAlpha => BlendFactor::SrcAlpha,
        wgpu::BlendFactor::OneMinusSrcAlpha => BlendFactor::OneMinusSrcAlpha,
        wgpu::BlendFactor::Dst => BlendFactor::Dst,
        wgpu::BlendFactor::OneMinusDst => BlendFactor::OneMinusDst,
        wgpu::BlendFactor::DstAlpha => BlendFactor::DstAlpha,
        wgpu::BlendFactor::OneMinusDstAlpha => BlendFactor::OneMinusDstAlpha,
        wgpu::BlendFactor::SrcAlphaSaturated => BlendFactor::SrcAlphaSaturated,
        wgpu::BlendFactor::Constant => BlendFactor::Constant,
        wgpu::BlendFactor::OneMinusConstant => BlendFactor::OneMinusConstant,
        _ => BlendFactor::One,
    }
}

pub(crate) fn map_color_write_mask(value: wgpu::ColorWrites) -> ColorWriteMask {
    ColorWriteMask::from_bits_truncate(value.bits() as u64)
}

pub(crate) fn map_buffer_usages(value: wgpu::BufferUsages) -> BufferUsage {
    BufferUsage::from_bits_truncate(value.bits() as u64)
}

pub(crate) fn map_texture_usages(value: wgpu::TextureUsages) -> TextureUsage {
    TextureUsage::from_bits_truncate(value.bits() as u64)
}

pub(crate) fn map_texture_usage_to_wgpu(value: TextureUsage) -> wgpu::TextureUsages {
    let mut out = wgpu::TextureUsages::empty();
    if value.contains(TextureUsage::COPY_SRC) {
        out |= wgpu::TextureUsages::COPY_SRC;
    }
    if value.contains(TextureUsage::COPY_DST) {
        out |= wgpu::TextureUsages::COPY_DST;
    }
    if value.contains(TextureUsage::TEXTURE_BINDING) {
        out |= wgpu::TextureUsages::TEXTURE_BINDING;
    }
    if value.contains(TextureUsage::STORAGE_BINDING) {
        out |= wgpu::TextureUsages::STORAGE_BINDING;
    }
    if value.contains(TextureUsage::RENDER_ATTACHMENT) {
        out |= wgpu::TextureUsages::RENDER_ATTACHMENT;
    }
    if value.contains(TextureUsage::TRANSIENT_ATTACHMENT) {
        out |= wgpu::TextureUsages::TRANSIENT;
    }
    if value.contains(TextureUsage::STORAGE_ATTACHMENT) {
        out |= wgpu::TextureUsages::STORAGE_BINDING;
    }
    out
}

pub(crate) fn map_shader_stages(value: wgpu::ShaderStages) -> ShaderStage {
    ShaderStage::from_bits_truncate(value.bits() as u64)
}

pub(crate) fn map_sampler_binding_type(value: wgpu::SamplerBindingType) -> SamplerBindingType {
    match value {
        wgpu::SamplerBindingType::Filtering => SamplerBindingType::Filtering,
        wgpu::SamplerBindingType::NonFiltering => SamplerBindingType::NonFiltering,
        wgpu::SamplerBindingType::Comparison => SamplerBindingType::Comparison,
        _ => SamplerBindingType::Filtering,
    }
}

pub(crate) fn map_texture_sample_type(value: wgpu::TextureSampleType) -> TextureSampleType {
    match value {
        wgpu::TextureSampleType::Float { filterable } => {
            if filterable {
                TextureSampleType::Float
            } else {
                TextureSampleType::UnfilterableFloat
            }
        }
        wgpu::TextureSampleType::Depth => TextureSampleType::Depth,
        wgpu::TextureSampleType::Sint => TextureSampleType::Sint,
        wgpu::TextureSampleType::Uint => TextureSampleType::Uint,
        _ => TextureSampleType::Float,
    }
}

pub(crate) fn map_storage_texture_access(
    value: wgpu::StorageTextureAccess,
) -> StorageTextureAccess {
    match value {
        wgpu::StorageTextureAccess::ReadOnly => StorageTextureAccess::ReadOnly,
        wgpu::StorageTextureAccess::WriteOnly => StorageTextureAccess::WriteOnly,
        wgpu::StorageTextureAccess::ReadWrite => StorageTextureAccess::ReadWrite,
        _ => StorageTextureAccess::WriteOnly,
    }
}

pub(crate) fn map_vertex_format(value: wgpu::VertexFormat) -> VertexFormat {
    match value {
        wgpu::VertexFormat::Uint8x2 => VertexFormat::Uint8X2,
        wgpu::VertexFormat::Uint8x4 => VertexFormat::Uint8X4,
        wgpu::VertexFormat::Sint8x2 => VertexFormat::Sint8X2,
        wgpu::VertexFormat::Sint8x4 => VertexFormat::Sint8X4,
        wgpu::VertexFormat::Unorm8x2 => VertexFormat::Unorm8X2,
        wgpu::VertexFormat::Unorm8x4 => VertexFormat::Unorm8X4,
        wgpu::VertexFormat::Snorm8x2 => VertexFormat::Snorm8X2,
        wgpu::VertexFormat::Snorm8x4 => VertexFormat::Snorm8X4,
        wgpu::VertexFormat::Uint16x2 => VertexFormat::Uint16X2,
        wgpu::VertexFormat::Uint16x4 => VertexFormat::Uint16X4,
        wgpu::VertexFormat::Sint16x2 => VertexFormat::Sint16X2,
        wgpu::VertexFormat::Sint16x4 => VertexFormat::Sint16X4,
        wgpu::VertexFormat::Unorm16x2 => VertexFormat::Unorm16X2,
        wgpu::VertexFormat::Unorm16x4 => VertexFormat::Unorm16X4,
        wgpu::VertexFormat::Snorm16x2 => VertexFormat::Snorm16X2,
        wgpu::VertexFormat::Snorm16x4 => VertexFormat::Snorm16X4,
        wgpu::VertexFormat::Float16x2 => VertexFormat::Float16X2,
        wgpu::VertexFormat::Float16x4 => VertexFormat::Float16X4,
        wgpu::VertexFormat::Float32 => VertexFormat::Float32,
        wgpu::VertexFormat::Float32x2 => VertexFormat::Float32X2,
        wgpu::VertexFormat::Float32x3 => VertexFormat::Float32X3,
        wgpu::VertexFormat::Float32x4 => VertexFormat::Float32X4,
        wgpu::VertexFormat::Uint32 => VertexFormat::Uint32,
        wgpu::VertexFormat::Uint32x2 => VertexFormat::Uint32X2,
        wgpu::VertexFormat::Uint32x3 => VertexFormat::Uint32X3,
        wgpu::VertexFormat::Uint32x4 => VertexFormat::Uint32X4,
        wgpu::VertexFormat::Sint32 => VertexFormat::Sint32,
        wgpu::VertexFormat::Sint32x2 => VertexFormat::Sint32X2,
        wgpu::VertexFormat::Sint32x3 => VertexFormat::Sint32X3,
        wgpu::VertexFormat::Sint32x4 => VertexFormat::Sint32X4,
        wgpu::VertexFormat::Unorm10_10_10_2 => VertexFormat::Unorm1010102,
        wgpu::VertexFormat::Unorm8x4Bgra => VertexFormat::Unorm8X4Bgra,
        _ => VertexFormat::Float32X3,
    }
}

pub(crate) fn map_vertex_step_mode(value: wgpu::VertexStepMode) -> VertexStepMode {
    match value {
        wgpu::VertexStepMode::Vertex => VertexStepMode::Vertex,
        wgpu::VertexStepMode::Instance => VertexStepMode::Instance,
        _ => VertexStepMode::Vertex,
    }
}

pub(crate) fn map_load_op<T>(value: wgpu::LoadOp<T>) -> LoadOp {
    match value {
        wgpu::LoadOp::Load => LoadOp::Load,
        wgpu::LoadOp::Clear(_) => LoadOp::Clear,
        wgpu::LoadOp::DontCare(_) => LoadOp::Load,
    }
}

pub(crate) fn map_store_op(value: wgpu::StoreOp) -> StoreOp {
    match value {
        wgpu::StoreOp::Store => StoreOp::Store,
        wgpu::StoreOp::Discard => StoreOp::Discard,
        _ => StoreOp::Store,
    }
}

pub(crate) fn map_color(value: wgpu::Color) -> Color {
    Color {
        r: Some(value.r),
        g: Some(value.g),
        b: Some(value.b),
        a: Some(value.a),
    }
}

pub(crate) fn map_origin_3d(value: wgpu::Origin3d) -> Origin3D {
    Origin3D {
        x: Some(value.x),
        y: Some(value.y),
        z: Some(value.z),
    }
}

pub(crate) fn map_extent_3d(value: wgpu::Extent3d) -> Extent3D {
    Extent3D {
        width: Some(value.width),
        height: Some(value.height),
        depth_or_array_layers: Some(value.depth_or_array_layers),
    }
}

pub(crate) fn map_texel_copy_buffer_layout(
    value: wgpu::TexelCopyBufferLayout,
) -> TexelCopyBufferLayout {
    TexelCopyBufferLayout {
        offset: Some(value.offset),
        bytes_per_row: value.bytes_per_row,
        rows_per_image: value.rows_per_image,
    }
}

pub(crate) fn map_texel_copy_buffer_info(
    value: wgpu::TexelCopyBufferInfo<'_>,
) -> TexelCopyBufferInfo {
    TexelCopyBufferInfo {
        layout: Some(map_texel_copy_buffer_layout(value.layout)),
        buffer: Some(expect_buffer_from_api(value.buffer)),
    }
}

pub(crate) fn map_texel_copy_texture_info(
    value: wgpu::TexelCopyTextureInfo<'_>,
) -> TexelCopyTextureInfo {
    TexelCopyTextureInfo {
        texture: Some(expect_texture_from_api(value.texture)),
        mip_level: Some(value.mip_level),
        origin: Some(map_origin_3d(value.origin)),
        aspect: Some(map_texture_aspect(value.aspect)),
    }
}

pub(crate) fn map_shader_module_descriptor(
    desc: wgpu::ShaderModuleDescriptor<'_>,
) -> ShaderModuleDescriptor {
    let mut out = ShaderModuleDescriptor::new();
    out.label = label_to_string(desc.label);
    match desc.source {
        wgpu::ShaderSource::Wgsl(source) => {
            let ext = ShaderSourceWGSL {
                code: Some(source.to_string()),
            };
            out = out.with_extension(ShaderModuleDescriptorExtension::from(ext));
        }
        _ => panic!("wgpu-compat: unsupported shader source"),
    }
    out
}

pub(crate) fn map_buffer_descriptor(desc: &wgpu::BufferDescriptor<'_>) -> BufferDescriptor {
    let mut out = BufferDescriptor::new();
    out.label = label_to_string(desc.label);
    out.size = Some(desc.size);
    out.usage = Some(map_buffer_usages(desc.usage));
    out.mapped_at_creation = Some(desc.mapped_at_creation);
    out
}

pub(crate) fn map_texture_descriptor(desc: &wgpu::TextureDescriptor<'_>) -> TextureDescriptor {
    let mut out = TextureDescriptor::new();
    out.label = label_to_string(desc.label);
    out.size = Some(map_extent_3d(desc.size));
    out.mip_level_count = Some(desc.mip_level_count);
    out.sample_count = Some(desc.sample_count);
    out.dimension = Some(map_texture_dimension(desc.dimension));
    out.format = Some(map_texture_format(desc.format));
    out.usage = Some(map_texture_usages(desc.usage));
    out.view_formats = Some(
        desc.view_formats
            .iter()
            .copied()
            .map(map_texture_format)
            .collect(),
    );
    out
}

pub(crate) fn map_texture_view_descriptor(
    desc: &wgpu::TextureViewDescriptor<'_>,
) -> TextureViewDescriptor {
    let mut out = TextureViewDescriptor::new();
    out.label = label_to_string(desc.label);
    out.format = desc.format.map(map_texture_format);
    out.dimension = desc.dimension.map(map_texture_view_dimension);
    out.aspect = Some(map_texture_aspect(desc.aspect));
    out.base_mip_level = Some(desc.base_mip_level);
    if let Some(count) = desc.mip_level_count {
        out.mip_level_count = Some(count);
    }
    out.base_array_layer = Some(desc.base_array_layer);
    if let Some(count) = desc.array_layer_count {
        out.array_layer_count = Some(count);
    }
    out.usage = desc.usage.map(map_texture_usages);
    out
}

pub(crate) fn map_sampler_descriptor(desc: &wgpu::SamplerDescriptor<'_>) -> SamplerDescriptor {
    let mut out = SamplerDescriptor::new();
    out.label = label_to_string(desc.label);
    out.address_mode_u = Some(map_address_mode(desc.address_mode_u));
    out.address_mode_v = Some(map_address_mode(desc.address_mode_v));
    out.address_mode_w = Some(map_address_mode(desc.address_mode_w));
    out.mag_filter = Some(map_filter_mode(desc.mag_filter));
    out.min_filter = Some(map_filter_mode(desc.min_filter));
    out.mipmap_filter = Some(map_mipmap_filter_mode(desc.mipmap_filter));
    out.lod_min_clamp = Some(desc.lod_min_clamp);
    out.lod_max_clamp = Some(desc.lod_max_clamp);
    out.compare = desc.compare.map(map_compare_function);
    out.max_anisotropy = Some(desc.anisotropy_clamp);
    out
}

pub(crate) fn map_bind_group_layout_entry(
    entry: &wgpu::BindGroupLayoutEntry,
) -> BindGroupLayoutEntry {
    let mut out = BindGroupLayoutEntry::new();
    out.binding = Some(entry.binding);
    out.visibility = Some(map_shader_stages(entry.visibility));
    out.binding_array_size = entry.count.map(|v| v.get()).or(Some(0));

    match entry.ty {
        wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset,
            min_binding_size,
        } => {
            let mut layout = BufferBindingLayout::new();
            layout.r#type = Some(match ty {
                wgpu::BufferBindingType::Uniform => BufferBindingType::Uniform,
                wgpu::BufferBindingType::Storage { read_only } => {
                    if read_only {
                        BufferBindingType::ReadOnlyStorage
                    } else {
                        BufferBindingType::Storage
                    }
                }
                _ => BufferBindingType::Uniform,
            });
            layout.has_dynamic_offset = Some(has_dynamic_offset);
            layout.min_binding_size = min_binding_size.map(|v| v.get());
            out.buffer = Some(layout);
        }
        wgpu::BindingType::Sampler(ty) => {
            let mut layout = SamplerBindingLayout::new();
            layout.r#type = Some(map_sampler_binding_type(ty));
            out.sampler = Some(layout);
        }
        wgpu::BindingType::Texture {
            sample_type,
            view_dimension,
            multisampled,
        } => {
            let mut layout = TextureBindingLayout::new();
            layout.sample_type = Some(map_texture_sample_type(sample_type));
            layout.view_dimension = Some(map_texture_view_dimension(view_dimension));
            layout.multisampled = Some(multisampled);
            out.texture = Some(layout);
        }
        wgpu::BindingType::StorageTexture {
            access,
            format,
            view_dimension,
        } => {
            let mut layout = StorageTextureBindingLayout::new();
            layout.access = Some(map_storage_texture_access(access));
            layout.format = Some(map_texture_format(format));
            layout.view_dimension = Some(map_texture_view_dimension(view_dimension));
            out.storage_texture = Some(layout);
        }
        _ => panic!("wgpu-compat: unsupported binding type"),
    }

    out
}

pub(crate) fn map_bind_group_layout_descriptor(
    desc: &wgpu::BindGroupLayoutDescriptor<'_>,
) -> BindGroupLayoutDescriptor {
    let mut out = BindGroupLayoutDescriptor::new();
    out.label = label_to_string(desc.label);
    out.entries = Some(
        desc.entries
            .iter()
            .map(map_bind_group_layout_entry)
            .collect(),
    );
    out
}

pub(crate) fn map_bind_group_entry(entry: &wgpu::BindGroupEntry<'_>) -> BindGroupEntry {
    let mut out = BindGroupEntry::default();
    out.binding = Some(entry.binding);
    match &entry.resource {
        wgpu::BindingResource::Buffer(buffer) => {
            out.buffer = Some(expect_buffer_from_api(buffer.buffer));
            out.offset = Some(buffer.offset);
            out.size = Some(buffer.size.map(|v| v.get()).unwrap_or(WHOLE_SIZE));
        }
        wgpu::BindingResource::Sampler(sampler) => {
            out.sampler = Some(expect_sampler_from_api(sampler));
        }
        wgpu::BindingResource::TextureView(view) => {
            out.texture_view = Some(expect_texture_view_from_api(view));
        }
        _ => panic!("wgpu-compat: binding resource arrays not supported"),
    }
    out
}

pub(crate) fn map_bind_group_descriptor(
    desc: &wgpu::BindGroupDescriptor<'_>,
) -> BindGroupDescriptor {
    let mut out = BindGroupDescriptor::new();
    out.label = label_to_string(desc.label);
    out.layout = Some(expect_bind_group_layout_from_api(desc.layout));
    out.entries = Some(desc.entries.iter().map(map_bind_group_entry).collect());
    out
}

pub(crate) fn map_pipeline_layout_descriptor(
    desc: &wgpu::PipelineLayoutDescriptor<'_>,
) -> PipelineLayoutDescriptor {
    let mut out = PipelineLayoutDescriptor::new();
    out.label = label_to_string(desc.label);
    out.bind_group_layouts = Some(
        desc.bind_group_layouts
            .iter()
            .copied()
            .map(expect_bind_group_layout_from_api)
            .collect(),
    );
    out.immediate_size = Some(0);
    out
}

pub(crate) fn map_vertex_attribute(attr: &wgpu::VertexAttribute) -> VertexAttribute {
    let mut out = VertexAttribute::new();
    out.format = Some(map_vertex_format(attr.format));
    out.offset = Some(attr.offset);
    out.shader_location = Some(attr.shader_location);
    out
}

pub(crate) fn map_vertex_buffer_layout(
    layout: &wgpu::VertexBufferLayout<'_>,
) -> VertexBufferLayout {
    let mut out = VertexBufferLayout::new();
    out.array_stride = Some(layout.array_stride);
    out.step_mode = Some(map_vertex_step_mode(layout.step_mode));
    out.attributes = Some(layout.attributes.iter().map(map_vertex_attribute).collect());
    out
}

pub(crate) fn map_vertex_state(state: &wgpu::VertexState<'_>) -> VertexState {
    let mut out = VertexState::new();
    out.module = Some(expect_shader_module_from_api(state.module));
    out.entry_point = state.entry_point.map(str::to_string);
    out.constants = Some(
        state
            .compilation_options
            .constants
            .iter()
            .map(|(k, v)| {
                let mut entry = ConstantEntry::new();
                entry.key = Some(k.to_string());
                entry.value = Some(*v);
                entry
            })
            .collect(),
    );
    out.buffers = Some(state.buffers.iter().map(map_vertex_buffer_layout).collect());
    out
}

pub(crate) fn map_blend_component(comp: &wgpu::BlendComponent) -> BlendComponent {
    BlendComponent {
        operation: Some(map_blend_operation(comp.operation)),
        src_factor: Some(map_blend_factor(comp.src_factor)),
        dst_factor: Some(map_blend_factor(comp.dst_factor)),
    }
}

pub(crate) fn map_blend_state(state: &wgpu::BlendState) -> BlendState {
    BlendState {
        color: Some(map_blend_component(&state.color)),
        alpha: Some(map_blend_component(&state.alpha)),
    }
}

pub(crate) fn map_color_target_state(state: &wgpu::ColorTargetState) -> ColorTargetState {
    let mut out = ColorTargetState::new();
    out.format = Some(map_texture_format(state.format));
    out.blend = state.blend.as_ref().map(map_blend_state);
    out.write_mask = Some(map_color_write_mask(state.write_mask));
    out
}

pub(crate) fn map_fragment_state(state: &wgpu::FragmentState<'_>) -> FragmentState {
    let mut out = FragmentState::new();
    out.module = Some(expect_shader_module_from_api(state.module));
    out.entry_point = state.entry_point.map(str::to_string);
    out.constants = Some(
        state
            .compilation_options
            .constants
            .iter()
            .map(|(k, v)| {
                let mut entry = ConstantEntry::new();
                entry.key = Some(k.to_string());
                entry.value = Some(*v);
                entry
            })
            .collect(),
    );
    out.targets = Some(
        state
            .targets
            .iter()
            .map(|t| t.as_ref().map(map_color_target_state).unwrap_or_default())
            .collect(),
    );
    out
}

pub(crate) fn map_stencil_face_state(state: &wgpu::StencilFaceState) -> StencilFaceState {
    StencilFaceState {
        compare: Some(map_compare_function(state.compare)),
        fail_op: Some(map_stencil_operation(state.fail_op)),
        depth_fail_op: Some(map_stencil_operation(state.depth_fail_op)),
        pass_op: Some(map_stencil_operation(state.pass_op)),
    }
}

pub(crate) fn map_depth_stencil_state(state: &wgpu::DepthStencilState) -> DepthStencilState {
    let mut out = DepthStencilState::new();
    out.format = Some(map_texture_format(state.format));
    out.depth_write_enabled = Some(if state.depth_write_enabled {
        OptionalBool::True
    } else {
        OptionalBool::False
    });
    out.depth_compare = Some(map_compare_function(state.depth_compare));
    out.stencil_front = Some(map_stencil_face_state(&state.stencil.front));
    out.stencil_back = Some(map_stencil_face_state(&state.stencil.back));
    out.stencil_read_mask = Some(state.stencil.read_mask);
    out.stencil_write_mask = Some(state.stencil.write_mask);
    out.depth_bias = Some(state.bias.constant);
    out.depth_bias_slope_scale = Some(state.bias.slope_scale);
    out.depth_bias_clamp = Some(state.bias.clamp);
    out
}

pub(crate) fn map_multisample_state(state: wgpu::MultisampleState) -> MultisampleState {
    let mut out = MultisampleState::new();
    out.count = Some(state.count);
    out.mask = Some(state.mask.try_into().unwrap_or(u32::MAX));
    out.alpha_to_coverage_enabled = Some(state.alpha_to_coverage_enabled);
    out
}

pub(crate) fn map_primitive_state(state: wgpu::PrimitiveState) -> PrimitiveState {
    let mut out = PrimitiveState::new();
    out.topology = Some(map_primitive_topology(state.topology));
    out.strip_index_format = state.strip_index_format.map(map_index_format);
    out.front_face = Some(map_front_face(state.front_face));
    out.cull_mode = Some(map_cull_mode(state.cull_mode));
    out.unclipped_depth = Some(state.unclipped_depth);
    out
}

pub(crate) fn map_render_pipeline_descriptor(
    desc: &wgpu::RenderPipelineDescriptor<'_>,
) -> RenderPipelineDescriptor {
    let mut out = RenderPipelineDescriptor::new();
    out.label = label_to_string(desc.label);
    out.layout = desc.layout.map(expect_pipeline_layout_from_api);
    out.vertex = Some(map_vertex_state(&desc.vertex));
    out.primitive = Some(map_primitive_state(desc.primitive));
    out.depth_stencil = desc.depth_stencil.as_ref().map(map_depth_stencil_state);
    out.multisample = Some(map_multisample_state(desc.multisample));
    out.fragment = desc.fragment.as_ref().map(map_fragment_state);
    out
}

pub(crate) fn map_compute_pipeline_descriptor(
    desc: &wgpu::ComputePipelineDescriptor<'_>,
) -> ComputePipelineDescriptor {
    let mut compute = ComputeState::new();
    compute.module = Some(expect_shader_module_from_api(desc.module));
    compute.entry_point = desc.entry_point.map(str::to_string);
    compute.constants = Some(
        desc.compilation_options
            .constants
            .iter()
            .map(|(k, v)| {
                let mut entry = ConstantEntry::new();
                entry.key = Some(k.to_string());
                entry.value = Some(*v);
                entry
            })
            .collect(),
    );

    let mut out = ComputePipelineDescriptor::new();
    out.label = label_to_string(desc.label);
    out.layout = desc.layout.map(expect_pipeline_layout_from_api);
    out.compute = Some(compute);
    out
}

pub(crate) fn map_command_encoder_descriptor(
    desc: &wgpu::CommandEncoderDescriptor<'_>,
) -> CommandEncoderDescriptor {
    let mut out = CommandEncoderDescriptor::new();
    out.label = label_to_string(desc.label);
    out
}

pub(crate) fn map_render_bundle_encoder_descriptor(
    desc: &wgpu::RenderBundleEncoderDescriptor<'_>,
) -> RenderBundleEncoderDescriptor {
    let (depth_stencil_format, depth_read_only, stencil_read_only) = match desc.depth_stencil {
        Some(depth_stencil) => (
            Some(map_texture_format(depth_stencil.format)),
            Some(depth_stencil.depth_read_only),
            Some(depth_stencil.stencil_read_only),
        ),
        None => (None, None, None),
    };
    let mut out = RenderBundleEncoderDescriptor::new();
    out.label = label_to_string(desc.label);
    out.color_formats = Some(
        desc.color_formats
            .iter()
            .map(|v| {
                v.map(map_texture_format)
                    .unwrap_or(TextureFormat::Undefined)
            })
            .collect(),
    );
    out.depth_stencil_format = depth_stencil_format;
    out.sample_count = Some(desc.sample_count);
    out.depth_read_only = depth_read_only;
    out.stencil_read_only = stencil_read_only;
    out
}

pub(crate) fn map_render_pass_color_attachment(
    attachment: &wgpu::RenderPassColorAttachment<'_>,
) -> RenderPassColorAttachment {
    let (load_op, clear_value) = match attachment.ops.load {
        wgpu::LoadOp::Load => (LoadOp::Load, None),
        wgpu::LoadOp::Clear(value) => (LoadOp::Clear, Some(map_color(value))),
        wgpu::LoadOp::DontCare(_) => (LoadOp::Load, None),
    };
    let mut out = RenderPassColorAttachment::new();
    out.view = Some(expect_texture_view_from_api(attachment.view));
    if let Some(depth_slice) = attachment.depth_slice {
        out.depth_slice = Some(depth_slice);
    }
    out.resolve_target = attachment.resolve_target.map(expect_texture_view_from_api);
    out.load_op = Some(load_op);
    out.store_op = Some(map_store_op(attachment.ops.store));
    out.clear_value = clear_value;
    out
}

pub(crate) fn map_render_pass_depth_stencil_attachment(
    attachment: &wgpu::RenderPassDepthStencilAttachment<'_>,
) -> RenderPassDepthStencilAttachment {
    let depth_read_only = attachment.depth_ops.is_none();
    let stencil_read_only = attachment.stencil_ops.is_none();
    let (depth_load_op, depth_store_op, depth_clear_value) = match attachment.depth_ops.as_ref() {
        Some(ops) => match ops.load {
            wgpu::LoadOp::Load => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
            wgpu::LoadOp::Clear(value) => (
                Some(LoadOp::Clear),
                Some(map_store_op(ops.store)),
                Some(value),
            ),
            wgpu::LoadOp::DontCare(_) => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
        },
        None => (None, None, None),
    };
    let (stencil_load_op, stencil_store_op, stencil_clear_value) = match attachment
        .stencil_ops
        .as_ref()
    {
        Some(ops) => match ops.load {
            wgpu::LoadOp::Load => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
            wgpu::LoadOp::Clear(value) => (
                Some(LoadOp::Clear),
                Some(map_store_op(ops.store)),
                Some(value),
            ),
            wgpu::LoadOp::DontCare(_) => (Some(LoadOp::Load), Some(map_store_op(ops.store)), None),
        },
        None => (None, None, None),
    };
    let mut out = RenderPassDepthStencilAttachment::new();
    out.view = Some(expect_texture_view_from_api(attachment.view));
    out.depth_load_op = depth_load_op;
    out.depth_store_op = depth_store_op;
    out.depth_clear_value = depth_clear_value;
    out.depth_read_only = Some(depth_read_only);
    out.stencil_load_op = stencil_load_op;
    out.stencil_store_op = stencil_store_op;
    out.stencil_clear_value = stencil_clear_value;
    out.stencil_read_only = Some(stencil_read_only);
    out
}

pub(crate) fn map_render_pass_descriptor(
    desc: &wgpu::RenderPassDescriptor<'_>,
) -> RenderPassDescriptor {
    let mut out = RenderPassDescriptor::new();
    out.label = label_to_string(desc.label);
    out.color_attachments = Some(
        desc.color_attachments
            .iter()
            .map(|att| {
                att.as_ref()
                    .map(map_render_pass_color_attachment)
                    .unwrap_or_default()
            })
            .collect(),
    );
    out.depth_stencil_attachment = desc
        .depth_stencil_attachment
        .as_ref()
        .map(map_render_pass_depth_stencil_attachment);
    out.occlusion_query_set = desc.occlusion_query_set.map(expect_query_set_from_api);
    out.timestamp_writes = None;
    out
}

pub(crate) fn map_compute_pass_descriptor(
    desc: &wgpu::ComputePassDescriptor<'_>,
) -> ComputePassDescriptor {
    let mut out = ComputePassDescriptor::new();
    out.label = label_to_string(desc.label);
    out.timestamp_writes = None;
    out
}

pub(crate) fn map_surface_configuration(
    config: &wgpu::SurfaceConfiguration,
) -> SurfaceConfiguration {
    let mut out = SurfaceConfiguration::new();
    out.format = Some(map_texture_format(config.format));
    out.usage = Some(map_texture_usages(config.usage));
    out.width = Some(config.width);
    out.height = Some(config.height);
    out.present_mode = Some(match config.present_mode {
        wgpu::PresentMode::Fifo => PresentMode::Fifo,
        wgpu::PresentMode::Mailbox => PresentMode::Mailbox,
        wgpu::PresentMode::Immediate => PresentMode::Immediate,
        wgpu::PresentMode::AutoVsync => PresentMode::Fifo,
        wgpu::PresentMode::AutoNoVsync => PresentMode::Immediate,
        _ => PresentMode::Fifo,
    });
    out.alpha_mode = Some(match config.alpha_mode {
        wgpu::CompositeAlphaMode::Auto => CompositeAlphaMode::Auto,
        wgpu::CompositeAlphaMode::Opaque => CompositeAlphaMode::Opaque,
        wgpu::CompositeAlphaMode::PreMultiplied => CompositeAlphaMode::Premultiplied,
        wgpu::CompositeAlphaMode::PostMultiplied => CompositeAlphaMode::Premultiplied,
        wgpu::CompositeAlphaMode::Inherit => CompositeAlphaMode::Inherit,
        _ => CompositeAlphaMode::Auto,
    });
    out.view_formats = Some(
        config
            .view_formats
            .iter()
            .copied()
            .map(map_texture_format)
            .collect(),
    );
    out
}

pub(crate) fn map_surface_capabilities(caps: SurfaceCapabilities) -> wgpu::SurfaceCapabilities {
    wgpu::SurfaceCapabilities {
        formats: caps
            .formats
            .clone()
            .unwrap_or_default()
            .iter()
            .copied()
            .map(map_texture_format_to_wgpu)
            .collect(),
        present_modes: caps
            .present_modes
            .clone()
            .unwrap_or_default()
            .iter()
            .copied()
            .map(|m| match m {
                PresentMode::Fifo => wgpu::PresentMode::Fifo,
                PresentMode::Mailbox => wgpu::PresentMode::Mailbox,
                PresentMode::Immediate => wgpu::PresentMode::Immediate,
                PresentMode::FifoRelaxed => wgpu::PresentMode::AutoVsync,
                _ => wgpu::PresentMode::Fifo,
            })
            .collect(),
        alpha_modes: caps
            .alpha_modes
            .clone()
            .unwrap_or_default()
            .iter()
            .copied()
            .map(|m| match m {
                CompositeAlphaMode::Auto => wgpu::CompositeAlphaMode::Auto,
                CompositeAlphaMode::Opaque => wgpu::CompositeAlphaMode::Opaque,
                CompositeAlphaMode::Premultiplied => wgpu::CompositeAlphaMode::PreMultiplied,
                CompositeAlphaMode::Unpremultiplied => wgpu::CompositeAlphaMode::PostMultiplied,
                CompositeAlphaMode::Inherit => wgpu::CompositeAlphaMode::Inherit,
                _ => wgpu::CompositeAlphaMode::Auto,
            })
            .collect(),
        usages: map_texture_usage_to_wgpu(caps.usages.unwrap_or(TextureUsage::RENDER_ATTACHMENT)),
    }
}

pub(crate) fn map_compilation_info(info: &CompilationInfo) -> wgpu::CompilationInfo {
    wgpu::CompilationInfo {
        messages: info
            .messages
            .as_ref()
            .map(|messages| {
                messages
                    .iter()
                    .map(|message| wgpu::CompilationMessage {
                        message: message.message.clone().unwrap_or_default(),
                        message_type: match message.r#type.unwrap_or(CompilationMessageType::Info) {
                            CompilationMessageType::Error => wgpu::CompilationMessageType::Error,
                            CompilationMessageType::Warning => {
                                wgpu::CompilationMessageType::Warning
                            }
                            CompilationMessageType::Info => wgpu::CompilationMessageType::Info,
                        },
                        location: Some(wgpu::SourceLocation {
                            line_number: message.line_num.unwrap_or(0) as u32,
                            line_position: message.line_pos.unwrap_or(0) as u32,
                            offset: message.offset.unwrap_or(0) as u32,
                            length: message.length.unwrap_or(0) as u32,
                        }),
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

pub(crate) fn map_uncaptured_error(error_type: ErrorType, message: String) -> wgpu::Error {
    match error_type {
        ErrorType::OutOfMemory => wgpu::Error::OutOfMemory {
            source: Box::new(DawnError(message)),
        },
        ErrorType::Validation => wgpu::Error::Validation {
            source: Box::new(DawnError(message.clone())),
            description: message,
        },
        ErrorType::Internal | ErrorType::Unknown => wgpu::Error::Internal {
            source: Box::new(DawnError(message.clone())),
            description: message,
        },
        ErrorType::NoError => wgpu::Error::Internal {
            source: Box::new(DawnError("no error".to_string())),
            description: "no error".to_string(),
        },
    }
}

pub(crate) fn map_limits_to_dawn(limits: &wgpu::Limits) -> Limits {
    let mut out = Limits::new();
    out.max_texture_dimension_1d = Some(limits.max_texture_dimension_1d);
    out.max_texture_dimension_2d = Some(limits.max_texture_dimension_2d);
    out.max_texture_dimension_3d = Some(limits.max_texture_dimension_3d);
    out.max_texture_array_layers = Some(limits.max_texture_array_layers);
    out.max_bind_groups = Some(limits.max_bind_groups);
    out.max_bindings_per_bind_group = Some(limits.max_bindings_per_bind_group);
    out.max_dynamic_uniform_buffers_per_pipeline_layout =
        Some(limits.max_dynamic_uniform_buffers_per_pipeline_layout);
    out.max_dynamic_storage_buffers_per_pipeline_layout =
        Some(limits.max_dynamic_storage_buffers_per_pipeline_layout);
    out.max_sampled_textures_per_shader_stage = Some(limits.max_sampled_textures_per_shader_stage);
    out.max_samplers_per_shader_stage = Some(limits.max_samplers_per_shader_stage);
    out.max_storage_buffers_per_shader_stage = Some(limits.max_storage_buffers_per_shader_stage);
    out.max_storage_textures_per_shader_stage = Some(limits.max_storage_textures_per_shader_stage);
    out.max_uniform_buffers_per_shader_stage = Some(limits.max_uniform_buffers_per_shader_stage);
    out.max_uniform_buffer_binding_size = Some(limits.max_uniform_buffer_binding_size as u64);
    out.max_storage_buffer_binding_size = Some(limits.max_storage_buffer_binding_size as u64);
    out.min_uniform_buffer_offset_alignment = Some(limits.min_uniform_buffer_offset_alignment);
    out.min_storage_buffer_offset_alignment = Some(limits.min_storage_buffer_offset_alignment);
    out.max_vertex_buffers = Some(limits.max_vertex_buffers);
    out.max_buffer_size = Some(limits.max_buffer_size);
    out.max_vertex_attributes = Some(limits.max_vertex_attributes);
    out.max_vertex_buffer_array_stride = Some(limits.max_vertex_buffer_array_stride);
    out.max_inter_stage_shader_variables = Some(limits.max_inter_stage_shader_components);
    out.max_color_attachments = Some(limits.max_color_attachments);
    out.max_color_attachment_bytes_per_sample = Some(limits.max_color_attachment_bytes_per_sample);
    out.max_compute_workgroup_storage_size = Some(limits.max_compute_workgroup_storage_size);
    out.max_compute_invocations_per_workgroup = Some(limits.max_compute_invocations_per_workgroup);
    out.max_compute_workgroup_size_x = Some(limits.max_compute_workgroup_size_x);
    out.max_compute_workgroup_size_y = Some(limits.max_compute_workgroup_size_y);
    out.max_compute_workgroup_size_z = Some(limits.max_compute_workgroup_size_z);
    out.max_compute_workgroups_per_dimension = Some(limits.max_compute_workgroups_per_dimension);
    out.max_bind_groups_plus_vertex_buffers =
        Some(limits.max_bind_groups + limits.max_vertex_buffers);
    out
}

pub(crate) fn map_limits_to_wgpu(limits: &Limits) -> wgpu::Limits {
    let mut out = wgpu::Limits::default();
    if let Some(value) = limits.max_texture_dimension_1d {
        out.max_texture_dimension_1d = value;
    }
    if let Some(value) = limits.max_texture_dimension_2d {
        out.max_texture_dimension_2d = value;
    }
    if let Some(value) = limits.max_texture_dimension_3d {
        out.max_texture_dimension_3d = value;
    }
    if let Some(value) = limits.max_texture_array_layers {
        out.max_texture_array_layers = value;
    }
    if let Some(value) = limits.max_bind_groups {
        out.max_bind_groups = value;
    }
    if let Some(value) = limits.max_bindings_per_bind_group {
        out.max_bindings_per_bind_group = value;
    }
    if let Some(value) = limits.max_dynamic_uniform_buffers_per_pipeline_layout {
        out.max_dynamic_uniform_buffers_per_pipeline_layout = value;
    }
    if let Some(value) = limits.max_dynamic_storage_buffers_per_pipeline_layout {
        out.max_dynamic_storage_buffers_per_pipeline_layout = value;
    }
    if let Some(value) = limits.max_sampled_textures_per_shader_stage {
        out.max_sampled_textures_per_shader_stage = value;
    }
    if let Some(value) = limits.max_samplers_per_shader_stage {
        out.max_samplers_per_shader_stage = value;
    }
    if let Some(value) = limits.max_storage_buffers_per_shader_stage {
        out.max_storage_buffers_per_shader_stage = value;
    }
    if let Some(value) = limits.max_storage_textures_per_shader_stage {
        out.max_storage_textures_per_shader_stage = value;
    }
    if let Some(value) = limits.max_uniform_buffers_per_shader_stage {
        out.max_uniform_buffers_per_shader_stage = value;
    }
    if let Some(value) = limits.max_uniform_buffer_binding_size {
        out.max_uniform_buffer_binding_size = value as u32;
    }
    if let Some(value) = limits.max_storage_buffer_binding_size {
        out.max_storage_buffer_binding_size = value as u32;
    }
    if let Some(value) = limits.min_uniform_buffer_offset_alignment {
        out.min_uniform_buffer_offset_alignment = value;
    }
    if let Some(value) = limits.min_storage_buffer_offset_alignment {
        out.min_storage_buffer_offset_alignment = value;
    }
    if let Some(value) = limits.max_vertex_buffers {
        out.max_vertex_buffers = value;
    }
    if let Some(value) = limits.max_buffer_size {
        out.max_buffer_size = value;
    }
    if let Some(value) = limits.max_vertex_attributes {
        out.max_vertex_attributes = value;
    }
    if let Some(value) = limits.max_vertex_buffer_array_stride {
        out.max_vertex_buffer_array_stride = value;
    }
    if let Some(value) = limits.max_inter_stage_shader_variables {
        out.max_inter_stage_shader_components = value;
    }
    if let Some(value) = limits.max_color_attachments {
        out.max_color_attachments = value;
    }
    if let Some(value) = limits.max_color_attachment_bytes_per_sample {
        out.max_color_attachment_bytes_per_sample = value;
    }
    if let Some(value) = limits.max_compute_workgroup_storage_size {
        out.max_compute_workgroup_storage_size = value;
    }
    if let Some(value) = limits.max_compute_invocations_per_workgroup {
        out.max_compute_invocations_per_workgroup = value;
    }
    if let Some(value) = limits.max_compute_workgroup_size_x {
        out.max_compute_workgroup_size_x = value;
    }
    if let Some(value) = limits.max_compute_workgroup_size_y {
        out.max_compute_workgroup_size_y = value;
    }
    if let Some(value) = limits.max_compute_workgroup_size_z {
        out.max_compute_workgroup_size_z = value;
    }
    if let Some(value) = limits.max_compute_workgroups_per_dimension {
        out.max_compute_workgroups_per_dimension = value;
    }
    out
}

pub(crate) fn map_features_to_dawn(features: wgpu::Features) -> Vec<FeatureName> {
    let mut out = Vec::new();
    if features.contains(wgpu::Features::DEPTH_CLIP_CONTROL) {
        out.push(FeatureName::DepthClipControl);
    }
    if features.contains(wgpu::Features::DEPTH32FLOAT_STENCIL8) {
        out.push(FeatureName::Depth32FloatStencil8);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
        out.push(FeatureName::TextureCompressionBc);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC_SLICED_3D) {
        out.push(FeatureName::TextureCompressionBcSliced3D);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2) {
        out.push(FeatureName::TextureCompressionEtc2);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC) {
        out.push(FeatureName::TextureCompressionAstc);
    }
    if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_HDR) {
        out.push(FeatureName::TextureCompressionAstc);
    }
    if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        out.push(FeatureName::TimestampQuery);
    }
    if features.contains(wgpu::Features::INDIRECT_FIRST_INSTANCE) {
        out.push(FeatureName::IndirectFirstInstance);
    }
    if features.contains(wgpu::Features::SHADER_F16) {
        out.push(FeatureName::ShaderF16);
    }
    if features.contains(wgpu::Features::RG11B10UFLOAT_RENDERABLE) {
        out.push(FeatureName::Rg11B10UfloatRenderable);
    }
    if features.contains(wgpu::Features::BGRA8UNORM_STORAGE) {
        out.push(FeatureName::Bgra8UnormStorage);
    }
    if features.contains(wgpu::Features::FLOAT32_FILTERABLE) {
        out.push(FeatureName::Float32Filterable);
    }
    if features.contains(wgpu::Features::CLIP_DISTANCES) {
        out.push(FeatureName::ClipDistances);
    }
    if features.contains(wgpu::Features::DUAL_SOURCE_BLENDING) {
        out.push(FeatureName::DualSourceBlending);
    }
    if features.contains(wgpu::Features::SUBGROUP) {
        out.push(FeatureName::Subgroups);
    }
    if features.contains(wgpu::Features::TEXTURE_FORMAT_16BIT_NORM) {
        out.push(FeatureName::Unorm16TextureFormats);
    }
    if features.contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT) {
        out.push(FeatureName::MultiDrawIndirect);
    }
    out
}

pub(crate) fn map_features_to_wgpu(features: &SupportedFeatures) -> wgpu::Features {
    let mut out = wgpu::Features::empty();
    if let Some(list) = &features.features {
        for feature in list {
            match feature {
                FeatureName::DepthClipControl => out |= wgpu::Features::DEPTH_CLIP_CONTROL,
                FeatureName::Depth32FloatStencil8 => out |= wgpu::Features::DEPTH32FLOAT_STENCIL8,
                FeatureName::TextureCompressionBc => out |= wgpu::Features::TEXTURE_COMPRESSION_BC,
                FeatureName::TextureCompressionBcSliced3D => {
                    out |= wgpu::Features::TEXTURE_COMPRESSION_BC_SLICED_3D
                }
                FeatureName::TextureCompressionEtc2 => {
                    out |= wgpu::Features::TEXTURE_COMPRESSION_ETC2
                }
                FeatureName::TextureCompressionAstc => {
                    out |= wgpu::Features::TEXTURE_COMPRESSION_ASTC
                }
                FeatureName::TimestampQuery => out |= wgpu::Features::TIMESTAMP_QUERY,
                FeatureName::IndirectFirstInstance => {
                    out |= wgpu::Features::INDIRECT_FIRST_INSTANCE
                }
                FeatureName::ShaderF16 => out |= wgpu::Features::SHADER_F16,
                FeatureName::Rg11B10UfloatRenderable => {
                    out |= wgpu::Features::RG11B10UFLOAT_RENDERABLE
                }
                FeatureName::Bgra8UnormStorage => out |= wgpu::Features::BGRA8UNORM_STORAGE,
                FeatureName::Float32Filterable => out |= wgpu::Features::FLOAT32_FILTERABLE,
                FeatureName::Float32Blendable => {}
                FeatureName::ClipDistances => out |= wgpu::Features::CLIP_DISTANCES,
                FeatureName::DualSourceBlending => out |= wgpu::Features::DUAL_SOURCE_BLENDING,
                FeatureName::Subgroups => out |= wgpu::Features::SUBGROUP,
                FeatureName::Unorm16TextureFormats => {
                    out |= wgpu::Features::TEXTURE_FORMAT_16BIT_NORM
                }
                FeatureName::MultiDrawIndirect => out |= wgpu::Features::MULTI_DRAW_INDIRECT_COUNT,
                _ => {}
            }
        }
    }
    out
}

pub(crate) fn bytes_to_u32(data: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity((data.len() + 3) / 4);
    for chunk in data.chunks(4) {
        let mut buf = [0u8; 4];
        buf[..chunk.len()].copy_from_slice(chunk);
        out.push(u32::from_le_bytes(buf));
    }
    out
}
