#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod ffi_linux_x86_64 {
    include!("ffi/linux_x86_64.rs");
}
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use ffi_linux_x86_64::*;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod ffi_macos_aarch64 {
    include!("ffi/macos_aarch64.rs");
}
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use ffi_macos_aarch64::*;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod ffi_windows_x86_64 {
    include!("ffi/windows_x86_64.rs");
}
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub use ffi_windows_x86_64::*;
