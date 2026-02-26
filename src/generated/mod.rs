#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod generated_linux_x86_64 {
    include!("targets/linux_x86_64.rs");
}
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use generated_linux_x86_64::*;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod generated_macos_aarch64 {
    include!("targets/macos_aarch64.rs");
}
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use generated_macos_aarch64::*;
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod generated_windows_x86_64 {
    include!("targets/windows_x86_64.rs");
}
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub use generated_windows_x86_64::*;
