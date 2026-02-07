use dawn_rs::{
    AdapterInfo, FutureWaitInfo, Instance, InstanceDescriptor, RequestAdapterStatus, Status,
};
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let mut instance_desc = InstanceDescriptor::new();
    instance_desc.required_features = Some(vec![dawn_rs::InstanceFeatureName::TimedWaitAny]);
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
    let status = instance.wait_any(
        Some(&mut [FutureWaitInfo {
            future: Some(future),
            completed: None,
        }]),
        0,
    );
    assert_eq!(status, dawn_rs::WaitStatus::Success);

    let result = rx.recv_timeout(Duration::from_millis(10)).unwrap();

    if let Err(message) = result {
        eprintln!("Request adapter failed: {message}");
    }
}

fn format_adapter_info(info: &AdapterInfo) -> String {
    let vendor = info.vendor.as_deref().unwrap_or("Unknown");
    let device = info.device.as_deref().unwrap_or("Unknown");
    let description = info.description.as_deref().unwrap_or("Unknown");
    format!("{vendor} {device} ({description})")
}
