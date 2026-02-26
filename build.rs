use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=DAWN_ROOT");
    println!("cargo:rerun-if-changed=src/wire_cpp_shim.cc");
    let Some(dawn_root) = resolve_dawn_root() else {
        println!("cargo:warning=DAWN_ROOT not set; skipping Dawn link directives");
        return;
    };
    let wire_enabled = env::var_os("CARGO_FEATURE_WIRE").is_some();
    let lib_dirs = resolve_dawn_lib_dirs(&dawn_root);
    if lib_dirs.is_empty() {
        println!(
            "cargo:warning=Missing Dawn build output under {} (expected lib/ or out/Release or out/Debug)",
            dawn_root.display()
        );
        return;
    }
    for lib_dir in &lib_dirs {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
    }
    if wire_enabled {
        // Link dawn_proc first so WebGPU C symbols resolve to proc-table dispatch stubs.
        println!("cargo:rustc-link-lib=static=dawn_proc");
        println!("cargo:rustc-link-lib=static=webgpu_dawn");
        if has_static_lib(&lib_dirs, "wire_client") {
            println!("cargo:rustc-link-lib=static=wire_client");
        }
        if has_static_lib(&lib_dirs, "wire_server") {
            println!("cargo:rustc-link-lib=static=wire_server");
        }
        // Dawn's default GN/CMake static build usually emits libdawn_wire.a containing both
        // client/server symbols.
        if has_static_lib(&lib_dirs, "dawn_wire") {
            println!("cargo:rustc-link-lib=static=dawn_wire");
        } else if !has_static_lib(&lib_dirs, "wire_server") {
            println!(
                "cargo:warning=Missing Dawn wire runtime library (expected dawn_wire or wire_server)"
            );
        }
        if let Some(gen_include) = resolve_dawn_gen_include_dir(&dawn_root) {
            build_wire_cpp_shim(&dawn_root, &gen_include);
        } else {
            println!(
                "cargo:warning=Missing Dawn generated headers for wire shim under {}",
                dawn_root.display()
            );
        }
    } else {
        println!("cargo:rustc-link-lib=static=webgpu_dawn");
    }
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

fn resolve_dawn_lib_dirs(dawn_root: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    for dir in [
        dawn_root.join("lib"),
        dawn_root.join("lib64"),
        dawn_root.join("out/cmake-install-static/lib"),
        dawn_root.join("out/cmake-install-static/src/dawn"),
        dawn_root.join("out/cmake-install-static/src/dawn/native"),
        dawn_root.join("out/cmake-install-static/src/dawn/wire"),
        dawn_root.join("out/Release"),
        dawn_root.join("out/Release/lib"),
        dawn_root.join("out/Release/src/dawn"),
        dawn_root.join("out/Release/src/dawn/native"),
        dawn_root.join("out/Release/src/dawn/wire"),
        dawn_root.join("out/Debug"),
        dawn_root.join("out/Debug/lib"),
        dawn_root.join("out/Debug/src/dawn"),
        dawn_root.join("out/Debug/src/dawn/native"),
        dawn_root.join("out/Debug/src/dawn/wire"),
    ] {
        if dir.exists() {
            dirs.push(dir);
        }
    }
    dirs
}

fn has_static_lib(lib_dirs: &[PathBuf], name: &str) -> bool {
    for dir in lib_dirs {
        let unix = dir.join(format!("lib{name}.a"));
        let windows = dir.join(format!("{name}.lib"));
        if unix.exists() || windows.exists() {
            return true;
        }
    }
    false
}

fn resolve_dawn_gen_include_dir(dawn_root: &Path) -> Option<PathBuf> {
    let release = dawn_root.join("out/Release/gen/include");
    if release.exists() {
        return Some(release);
    }
    let debug = dawn_root.join("out/Debug/gen/include");
    if debug.exists() {
        return Some(debug);
    }
    None
}

fn build_wire_cpp_shim(dawn_root: &Path, gen_include_dir: &Path) {
    let mut build = cc::Build::new();
    build.cpp(true);
    build.file("src/wire_cpp_shim.cc");
    build.include(dawn_root.join("include"));
    build.include(gen_include_dir);
    build.flag_if_supported("-std=c++20");
    // Match Dawn's own toolchain settings to avoid ABI/RTTI mismatches.
    build.flag_if_supported("-fno-rtti");
    build.flag_if_supported("-fno-exceptions");
    build.compile("dawn_rs_wire_cpp_shim");
}
