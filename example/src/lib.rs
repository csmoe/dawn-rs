use dawn_rs::{AdapterInfo, Instance, InstanceDescriptor, RequestAdapterStatus, Status};
use objc2::rc::autoreleasepool;
use std::sync::mpsc;
use std::time::Duration;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub fn run_example() {
    autoreleasepool(|_| {
        let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Dawn Example")
            .build(&event_loop)
            .expect("create window");

        let instance_desc = InstanceDescriptor::new();
        let instance = Instance::new(Some(&instance_desc));

        let (tx, rx) = mpsc::channel::<Result<(), String>>();
        let future = instance.request_adapter(None, move |status, adapter, message| {
            if status != RequestAdapterStatus::Success {
                let _ = tx.send(Err(format!("{status:?}: {message}")));
                return;
            }

            let adapter = match adapter {
                Some(adapter) => adapter,
                None => {
                    let _ = tx.send(Err("Request adapter returned no adapter".to_string()));
                    return;
                }
            };

            let mut info = AdapterInfo::new();
            let status = adapter.get_info(&mut info);
            if status != Status::Success {
                let _ = tx.send(Err(format!("Adapter info failed: {status:?}")));
                return;
            }

            println!("Adapter: {}", format_adapter_info(&info));
            let _ = tx.send(Ok(()));
        });

        let result = loop {
            match rx.recv_timeout(Duration::from_millis(10)) {
                Ok(result) => break result,
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    instance.process_events();
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    eprintln!("Request adapter callback disconnected");
                    return;
                }
            }
        };

        if let Err(message) = result {
            eprintln!("Request adapter failed: {message}");
        }
    });
}

fn format_adapter_info(info: &AdapterInfo) -> String {
    let vendor = info.vendor.as_deref().unwrap_or("Unknown");
    let device = info.device.as_deref().unwrap_or("Unknown");
    let description = info.description.as_deref().unwrap_or("Unknown");
    format!("{vendor} {device} ({description})")
}
