use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let dawn_root = resolve_dawn_root();
    let lib_dir = resolve_dawn_lib_dir(&dawn_root);
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=webgpu_dawn");

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
}

fn resolve_dawn_root() -> PathBuf {
    match env::var("DAWN_ROOT") {
        Ok(path) => PathBuf::from(path),
        Err(_) => panic!("DAWN_ROOT not set"),
    }
}

fn resolve_dawn_lib_dir(dawn_root: &Path) -> PathBuf {
    let lib = dawn_root.join("lib");
    if lib.exists() {
        return lib;
    }
    let release = dawn_root.join("out/Release");
    if release.exists() {
        return release;
    }
    let debug = dawn_root.join("out/Debug");
    if debug.exists() {
        return debug;
    }
    panic!(
        "Missing Dawn build output under {} (expected lib/ or out/Release or out/Debug)",
        dawn_root.display()
    );
}
