//! Example demonstrating import of a DXGI shared handle on D3D11 backend.
//!
//! This example shows how to:
//! 1. Request a D3D11 adapter using `RequestAdapterOptionsD3D11Device` to pin
//!    Dawn to an existing `ID3D11Device`.
//! 2. Import a `HANDLE` (DXGI shared handle) as a `SharedTextureMemory` using
//!    `SharedTextureMemoryDXGISharedHandleDescriptor`.
//! 3. Alternatively import an `ID3D11Texture2D*` directly using
//!    `SharedTextureMemoryD3D11Texture2DDescriptor`.
//!
//! On non-Windows targets the example exits immediately with a note.

#[cfg(target_os = "windows")]
mod imp {
    use dawn_rs::{
        BackendType, Device, DeviceDescriptor, FeatureName, FutureWaitInfo, Instance,
        InstanceDescriptor, InstanceFeatureName, RequestAdapterOptions,
        RequestAdapterOptionsD3D11Device, RequestAdapterStatus,
        SharedFenceDXGISharedHandleDescriptor, SharedFenceDescriptor,
        SharedTextureMemoryD3D11Texture2DDescriptor, SharedTextureMemoryDescriptor,
        SharedTextureMemoryDXGISharedHandleDescriptor, Status, WaitStatus,
    };
    use std::ffi::c_void;
    use std::sync::mpsc;
    use std::time::Duration;

    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
    use windows::Win32::Graphics::Direct3D11::{
        D3D11CreateDevice, ID3D11Device, ID3D11Texture2D, D3D11_BIND_RENDER_TARGET,
        D3D11_BIND_SHADER_RESOURCE, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
        D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
    };
    use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC};
    use windows::Win32::Graphics::Dxgi::{
        IDXGIResource1, DXGI_SHARED_RESOURCE_READ, DXGI_SHARED_RESOURCE_WRITE,
    };

    pub fn run() {
        // ------------------------------------------------------------------ //
        // 1. Create a D3D11 device                                            //
        // ------------------------------------------------------------------ //
        let d3d11_device: ID3D11Device = unsafe {
            let mut device = None;
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                None,
                D3D11_CREATE_DEVICE_FLAG(0),
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                None,
            )
            .expect("D3D11CreateDevice failed");
            device.unwrap()
        };

        // ------------------------------------------------------------------ //
        // 2. Create a shared texture on the D3D11 device                      //
        // ------------------------------------------------------------------ //
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: 256,
            Height: 256,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0x20, // D3D11_RESOURCE_MISC_SHARED_NTHANDLE | D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX
        };

        let d3d11_texture: ID3D11Texture2D = unsafe {
            let mut tex = None;
            d3d11_device
                .CreateTexture2D(&texture_desc, None, Some(&mut tex))
                .expect("CreateTexture2D failed");
            tex.unwrap()
        };

        // Obtain an NT shared handle for the texture.
        let shared_handle: HANDLE = unsafe {
            let dxgi_resource: IDXGIResource1 = d3d11_texture
                .cast()
                .expect("cast to IDXGIResource1 failed");
            dxgi_resource
                .CreateSharedHandle(
                    None,
                    (DXGI_SHARED_RESOURCE_READ | DXGI_SHARED_RESOURCE_WRITE).0,
                    None,
                )
                .expect("CreateSharedHandle failed")
        };

        // ------------------------------------------------------------------ //
        // 3. Create a Dawn instance and request an adapter backed by our      //
        //    existing D3D11 device via RequestAdapterOptionsD3D11Device.       //
        // ------------------------------------------------------------------ //
        let mut instance_desc = InstanceDescriptor::new();
        instance_desc.required_features =
            Some(vec![InstanceFeatureName::TimedWaitAny]);
        let instance = Instance::new(Some(&instance_desc));

        let (tx, rx) = mpsc::channel::<Result<Device, String>>();

        // Build adapter options that pin Dawn to our ID3D11Device.
        let mut adapter_opts = RequestAdapterOptions::new();
        adapter_opts.backend_type = Some(BackendType::D3D11);
        let mut d3d11_opt = RequestAdapterOptionsD3D11Device::new();
        d3d11_opt.device = Some(d3d11_device.as_raw() as *mut c_void);
        adapter_opts = adapter_opts.with_extension(d3d11_opt.into());

        let tx_clone = tx.clone();
        let future = instance.request_adapter(Some(&adapter_opts), move |status, adapter, msg| {
            if status != RequestAdapterStatus::Success {
                let _ = tx_clone.send(Err(format!("request_adapter: {status:?}: {msg}")));
                return;
            }
            let adapter = match adapter {
                Some(a) => a,
                None => {
                    let _ = tx_clone.send(Err("no adapter returned".into()));
                    return;
                }
            };

            // ------------------------------------------------------------------ //
            // 4. Create a Dawn device with the DXGI shared-handle features        //
            // ------------------------------------------------------------------ //
            let mut device_desc = DeviceDescriptor::new();
            device_desc.required_features = Some(vec![
                FeatureName::SharedTextureMemoryDXGISharedHandle,
                FeatureName::SharedFenceDXGISharedHandle,
                // Also request D3D11Texture2D support while we are here.
                FeatureName::SharedTextureMemoryD3D11Texture2D,
            ]);
            let tx2 = tx_clone.clone();
            adapter.request_device(&device_desc, move |status, device, msg| {
                if status != dawn_rs::RequestDeviceStatus::Success {
                    let _ = tx2.send(Err(format!("request_device: {status:?}: {msg}")));
                    return;
                }
                match device {
                    Some(d) => {
                        let _ = tx2.send(Ok(d));
                    }
                    None => {
                        let _ = tx2.send(Err("no device returned".into()));
                    }
                }
            });
        });

        let wait_status = instance.wait_any(
            Some(&mut [FutureWaitInfo {
                future: Some(future),
                completed: None,
            }]),
            Duration::from_secs(5).as_nanos() as u64,
        );
        assert_eq!(wait_status, WaitStatus::Success, "wait_any timed out");

        let device = rx
            .recv_timeout(Duration::from_millis(100))
            .expect("recv timeout")
            .expect("device creation failed");

        // ------------------------------------------------------------------ //
        // 5a. Import the DXGI shared handle as SharedTextureMemory             //
        // ------------------------------------------------------------------ //
        let mut stm_desc = SharedTextureMemoryDescriptor::new();
        let mut dxgi_handle_desc = SharedTextureMemoryDXGISharedHandleDescriptor::new();
        dxgi_handle_desc.handle = Some(shared_handle.0 as *mut c_void);
        dxgi_handle_desc.use_keyed_mutex = Some(true);
        stm_desc = stm_desc.with_extension(dxgi_handle_desc.into());

        let shared_texture_memory = device.import_shared_texture_memory(&stm_desc);
        let props = shared_texture_memory.get_properties();
        println!(
            "SharedTextureMemory (DXGI handle): size={:?}",
            props.size
        );

        // ------------------------------------------------------------------ //
        // 5b. Import the ID3D11Texture2D* directly as SharedTextureMemory     //
        // ------------------------------------------------------------------ //
        let mut stm_desc2 = SharedTextureMemoryDescriptor::new();
        let mut d3d11_tex_desc = SharedTextureMemoryD3D11Texture2DDescriptor::new();
        d3d11_tex_desc.texture = Some(d3d11_texture.as_raw() as *mut c_void);
        stm_desc2 = stm_desc2.with_extension(d3d11_tex_desc.into());

        let shared_texture_memory2 = device.import_shared_texture_memory(&stm_desc2);
        let props2 = shared_texture_memory2.get_properties();
        println!(
            "SharedTextureMemory (D3D11Texture2D): size={:?}",
            props2.size
        );

        // ------------------------------------------------------------------ //
        // 6. Import a shared fence                                             //
        // ------------------------------------------------------------------ //
        let mut fence_desc = SharedFenceDescriptor::new();
        let mut dxgi_fence_desc = SharedFenceDXGISharedHandleDescriptor::new();
        dxgi_fence_desc.handle = Some(shared_handle.0 as *mut c_void);
        fence_desc = fence_desc.with_extension(dxgi_fence_desc.into());
        let _shared_fence = device.import_shared_fence(&fence_desc);
        println!("SharedFence (DXGI handle): imported successfully");

        // Clean up the NT handle.
        unsafe {
            windows::Win32::Foundation::CloseHandle(shared_handle)
                .expect("CloseHandle failed");
        }
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    imp::run();

    #[cfg(not(target_os = "windows"))]
    println!("This example is Windows-only (requires D3D11 + DXGI).");
}
