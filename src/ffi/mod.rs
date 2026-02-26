#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod macos_aarch64;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use macos_aarch64::*;
