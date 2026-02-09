#![allow(dead_code, unused_imports)]
use crate::ffi;
use super::*;
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
