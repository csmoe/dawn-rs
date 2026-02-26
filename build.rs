use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=DAWN_ROOT");
    let Some(dawn_root) = resolve_dawn_root() else {
        println!("cargo:warning=DAWN_ROOT not set; skipping Dawn link directives");
        return;
    };
    let Some(lib_dir) = resolve_dawn_lib_dir(&dawn_root) else {
        println!(
            "cargo:warning=Missing Dawn build output under {} (expected lib/ or out/Release or out/Debug)",
            dawn_root.display()
        );
        return;
    };
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=webgpu_dawn");
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=c++abi");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=IOSurface");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=Cocoa");
    }
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=onecore_apiset");
        println!("cargo:rustc-link-lib=dxguid");
    }
}

fn resolve_dawn_root() -> Option<PathBuf> {
    env::var("DAWN_ROOT").ok().map(PathBuf::from)
}

fn resolve_dawn_lib_dir(dawn_root: &Path) -> Option<PathBuf> {
    let lib = dawn_root.join("lib");
    if lib.exists() {
        return Some(lib);
    }
    let lib = dawn_root.join("lib64");
    if lib.exists() {
        return Some(lib);
    }
    let release = dawn_root.join("out/Release");
    if release.exists() {
        return Some(release);
    }
    let debug = dawn_root.join("out/Debug");
    if debug.exists() {
        return Some(debug);
    }
    None
}
