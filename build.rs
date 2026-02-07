use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let (dawn_root, dawn_bin, api_header) = resolve_dawn_paths();
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
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let gen_out = env::var("DAWN_OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| out_dir.join("generated"));
    println!("cargo:rerun-if-env-changed=DAWN_OUT_DIR");

    let dawn_json = env::var("DAWN_JSON").map(PathBuf::from).unwrap();

    println!("cargo:rerun-if-changed={}", dawn_json.display());
    println!("cargo:rerun-if-env-changed=DAWN_JSON");

    let tags = tags_from_features();

    let api =
        dawn_codegen::DawnJsonParser::parse_file(&dawn_json).expect("failed to parse dawn.json");
    let filtered = if tags.is_empty() {
        api
    } else {
        api.filter_by_tags(&tags)
    };
    let model = dawn_codegen::ApiModel::from_api(&filtered);
    let files = dawn_codegen::generate_strings(&model);
    files
        .write_to_dir(&gen_out)
        .expect("failed to write generated files");

    std::fs::create_dir_all(&gen_out).expect("create generated dir");
    let ffi_out = gen_out.join("ffi.rs");
    println!("cargo:rerun-if-changed={}", api_header.display());
    let _ = dawn_bin;
    let clang_args = build_clang_args(&dawn_root, &api_header);
    let ffi_rs = dawn_codegen::generate_ffi_string(&api_header, &clang_args)
        .expect("failed to generate ffi.rs");
    std::fs::write(&ffi_out, ffi_rs).expect("failed to write ffi.rs");
}

fn resolve_dawn_paths() -> (PathBuf, PathBuf, PathBuf) {
    let dawn_root = match std::env::var("DAWN_ROOT") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            panic!("DAWN_ROOT not set");
        }
    };
    let dawn_bin = dawn_root.join("bin");
    if !dawn_bin.exists() {
        panic!("Missing Dawn bin dir at {}", dawn_bin.display());
    }
    let api_header = dawn_root.join("include/webgpu/webgpu.h");
    if !api_header.exists() {
        panic!("Missing webgpu.h at {}", api_header.display());
    }
    (dawn_root, dawn_bin, api_header)
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

fn build_clang_args(dawn_root: &Path, api_header: &Path) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(include_dir) = api_header.parent().and_then(|p| p.parent()) {
        args.push(format!("-I{}", include_dir.display()));
    }
    let dawn_include = dawn_root.join("include");
    args.push(format!("-I{}", dawn_include.display()));

    #[cfg(target_os = "macos")]
    {
        let sdk_path = macos_sdk_path().expect("xcrun failed to locate macOS SDK");
        args.push(format!("-isysroot{}", sdk_path));
    }
    args
}

#[cfg(target_os = "macos")]
fn macos_sdk_path() -> Option<String> {
    let output = std::process::Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        None
    } else {
        Some(path)
    }
}

fn tags_from_features() -> Vec<String> {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_DAWN");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_NATIVE");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_EMSCRIPTEN");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_COMPAT");

    let mut tags = Vec::new();
    if env::var("CARGO_FEATURE_DAWN").is_ok() {
        tags.push("dawn".to_string());
    }
    if env::var("CARGO_FEATURE_NATIVE").is_ok() {
        tags.push("native".to_string());
    }
    if env::var("CARGO_FEATURE_EMSCRIPTEN").is_ok() {
        tags.push("emscripten".to_string());
    }
    if env::var("CARGO_FEATURE_COMPAT").is_ok() {
        tags.push("compat".to_string());
    }
    tags
}
