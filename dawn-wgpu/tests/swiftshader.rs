use pollster::block_on;
use std::path::PathBuf;

#[test]
fn request_adapter_with_fallback_uses_swiftshader() {
    let backends = wgpu::Backends::VULKAN;
    let instance = dawn_wgpu::create_instance(&wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });

    let adapter = block_on(dawn_wgpu::request_adapter_with_fallback(
        &instance,
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None,
            force_fallback_adapter: true,
        },
        backends,
    ))
    .expect("request SwiftShader fallback adapter");

    let info = adapter.get_info();
    let description =
        format!("{} {} {}", info.name, info.driver, info.driver_info).to_ascii_lowercase();

    assert_eq!(info.backend, wgpu::Backend::Vulkan);
    assert_eq!(info.device_type, wgpu::DeviceType::Cpu);
    assert!(
        description.contains("swiftshader"),
        "expected SwiftShader adapter, got {info:?}"
    );
}
