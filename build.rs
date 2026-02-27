use std::env;
use std::fs;
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
    let dawn_source_root = resolve_dawn_source_root(&dawn_root);
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
            let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
            let out_dir = PathBuf::from(out_dir);
            if let Err(e) = generate_wire_client_proc_table_inc(&gen_include, &out_dir) {
                println!(
                    "cargo:warning=Failed to generate wire client proc table from C API: {}",
                    e
                );
            }
            if let Some(source_root) = dawn_source_root.as_ref() {
                build_wire_cpp_shim(source_root, &gen_include, &out_dir);
            } else {
                println!(
                    "cargo:warning=Failed to resolve Dawn source root from DAWN_ROOT={} (wire shim disabled)",
                    dawn_root.display()
                );
            }
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
        dawn_root.to_path_buf(),
        dawn_root.join("lib"),
        dawn_root.join("lib64"),
        dawn_root.join("src/dawn"),
        dawn_root.join("src/dawn/native"),
        dawn_root.join("src/dawn/wire"),
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

fn resolve_dawn_source_root(dawn_root: &Path) -> Option<PathBuf> {
    if has_dawn_public_headers(dawn_root) {
        return Some(dawn_root.to_path_buf());
    }

    let cache = dawn_root.join("CMakeCache.txt");
    if let Ok(content) = fs::read_to_string(&cache) {
        for line in content.lines() {
            if let Some(value) = line.strip_prefix("Dawn_SOURCE_DIR:STATIC=") {
                let path = PathBuf::from(value.trim());
                if has_dawn_public_headers(&path) {
                    return Some(path);
                }
            }
            if let Some(value) = line.strip_prefix("CMAKE_HOME_DIRECTORY:INTERNAL=") {
                let path = PathBuf::from(value.trim());
                if has_dawn_public_headers(&path) {
                    return Some(path);
                }
            }
        }
    }

    if dawn_root.ends_with("out/Release") || dawn_root.ends_with("out/Debug") {
        if let Some(parent) = dawn_root.parent().and_then(|p| p.parent()) {
            if has_dawn_public_headers(parent) {
                return Some(parent.to_path_buf());
            }
        }
    }

    None
}

fn has_dawn_public_headers(path: &Path) -> bool {
    path.join("include/webgpu/webgpu.h").exists()
        && path.join("include/dawn/native/DawnNative.h").exists()
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

fn generate_wire_client_proc_table_inc(
    gen_include_dir: &Path,
    out_dir: &Path,
) -> Result<(), String> {
    let proc_table_h = gen_include_dir.join("dawn/dawn_proc_table.h");
    let content = fs::read_to_string(&proc_table_h)
        .map_err(|e| format!("read {}: {}", proc_table_h.display(), e))?;
    let mut fields = Vec::<String>::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("WGPUProc") || !trimmed.ends_with(';') {
            continue;
        }
        let no_semicolon = &trimmed[..trimmed.len() - 1];
        let mut parts = no_semicolon.split_whitespace();
        let _ty = parts.next();
        let Some(name) = parts.next() else {
            continue;
        };
        if parts.next().is_some() {
            continue;
        }
        fields.push(name.to_string());
    }
    if fields.is_empty() {
        return Err("no DawnProcTable fields parsed".to_string());
    }

    let mut out = String::new();
    out.push_str("// @generated by build.rs; do not edit.\n");
    out.push_str("static WGPUProc DawnRsResolveWireClientProc(const char* name) {\n");
    out.push_str("    WGPUStringView sv = {name, std::strlen(name)};\n");
    out.push_str("    return wgpuDawnWireClientGetProcAddress(sv);\n");
    out.push_str("}\n\n");
    out.push_str("static DawnProcTable DawnRsBuildWireClientProcTableFromCAPI() {\n");
    out.push_str("    DawnProcTable procs = {};\n");
    out.push_str("    procs.getProcAddress = wgpuDawnWireClientGetProcAddress;\n");
    for field in fields {
        if field == "getProcAddress" {
            continue;
        }
        let mut chars = field.chars();
        let Some(first) = chars.next() else {
            continue;
        };
        let proc_name = format!("wgpu{}{}", first.to_ascii_uppercase(), chars.as_str());
        out.push_str(&format!(
            "    procs.{field} = reinterpret_cast<decltype(procs.{field})>(DawnRsResolveWireClientProc(\"{proc_name}\"));\n"
        ));
    }
    out.push_str("    return procs;\n");
    out.push_str("}\n");

    let out_file = out_dir.join("wire_client_proc_table_autogen.inc");
    fs::write(&out_file, out).map_err(|e| format!("write {}: {}", out_file.display(), e))?;
    Ok(())
}

fn resolve_dawn_gen_include_dir(dawn_root: &Path) -> Option<PathBuf> {
    let direct = dawn_root.join("gen/include");
    if direct.exists() {
        return Some(direct);
    }
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

fn build_wire_cpp_shim(dawn_root: &Path, gen_include_dir: &Path, out_dir: &Path) {
    let mut build = cc::Build::new();
    build.cpp(true);
    build.file("src/wire_cpp_shim.cc");
    build.include(dawn_root.join("include"));
    build.include(gen_include_dir);
    build.include(out_dir);
    build.flag_if_supported("-std=c++20");
    // Match Dawn's own toolchain settings to avoid ABI/RTTI mismatches.
    build.flag_if_supported("-fno-rtti");
    build.flag_if_supported("-fno-exceptions");
    build.compile("dawn_rs_wire_cpp_shim");
}
