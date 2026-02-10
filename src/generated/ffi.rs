#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod ffi_macos_aarch64 {
    include!("ffi/macos_aarch64.rs");
}
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use ffi_macos_aarch64::*;
