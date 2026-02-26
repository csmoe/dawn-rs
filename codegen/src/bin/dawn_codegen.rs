use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let mut dawn_json: Option<PathBuf> = None;
    let mut dawn_wire_json: Option<PathBuf> = None;
    let mut api_header: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut tags: Vec<String> = Vec::new();
    let mut clang_args: Vec<String> = Vec::new();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dawn-json" => {
                dawn_json = args.next().map(PathBuf::from);
            }
            "--api-header" => {
                api_header = args.next().map(PathBuf::from);
            }
            "--dawn-wire-json" => {
                dawn_wire_json = args.next().map(PathBuf::from);
            }
            "--out-dir" => {
                out_dir = args.next().map(PathBuf::from);
            }
            "--tags" => {
                if let Some(value) = args.next() {
                    tags = value
                        .split(',')
                        .map(|v| v.trim().to_string())
                        .filter(|v| !v.is_empty())
                        .collect();
                }
            }
            "--clang-arg" => {
                if let Some(value) = args.next() {
                    clang_args.push(value);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                std::process::exit(2);
            }
        }
    }

    let dawn_json = dawn_json.expect("--dawn-json is required");
    let api_header = api_header.expect("--api-header is required");
    let out_dir = out_dir.expect("--out-dir is required");
    let src_dir = out_dir
        .parent()
        .expect("--out-dir should point inside the src directory");
    let ffi_dir = src_dir.join("ffi");

    let api = dawn_codegen::DawnJsonParser::parse_file(&dawn_json).expect("parse dawn.json");
    let filtered = if tags.is_empty() {
        api
    } else {
        api.filter_by_tags(&tags)
    };
    let model = dawn_codegen::ApiModel::from_api(&filtered);
    let files = dawn_codegen::generate_strings(&model);

    std::fs::create_dir_all(&out_dir).expect("create output dir");
    cleanup_old_layout(&out_dir).expect("cleanup old generated layout");

    let target_os = env::consts::OS;
    let target_arch = env::consts::ARCH;
    fs::create_dir_all(&ffi_dir).expect("create ffi module dir");
    let ffi_out = ffi_dir.join(format!("{target_os}_{target_arch}.rs"));

    let mut combined_clang_args = clang_args;
    combined_clang_args.extend(default_clang_args(&api_header));

    let ffi_rs = dawn_codegen::generate_ffi_string(&api_header, &combined_clang_args)
        .expect("generate ffi.rs");
    std::fs::write(&ffi_out, ffi_rs).expect("write ffi bindings");

    write_generated_single_file(&out_dir, target_os, target_arch, &files)
        .expect("write generated single file");
    if let Some(path) = dawn_wire_json.as_deref() {
        write_wire_files(&out_dir, &filtered, path).expect("write wire files");
    }
    write_generated_wrapper(&out_dir).expect("write generated wrapper");
    write_ffi_wrapper(&ffi_dir).expect("write ffi wrapper");
}

fn default_clang_args(api_header: &Path) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(include_dir) = api_header.parent().and_then(|p| p.parent()) {
        args.push(format!("-I{}", include_dir.display()));
    }
    args
}

fn cleanup_old_layout(out_dir: &Path) -> std::io::Result<()> {
    let targets_dir = out_dir.join("targets");
    if targets_dir.exists() {
        fs::remove_dir_all(targets_dir)?;
    }

    let ffi_dir = out_dir.join("ffi");
    if ffi_dir.exists() {
        fs::remove_dir_all(ffi_dir)?;
    }

    let old_generated_ffi = out_dir.join("ffi.rs");
    if old_generated_ffi.exists() {
        fs::remove_file(old_generated_ffi)?;
    }

    let src_dir = out_dir
        .parent()
        .expect("--out-dir should point inside the src directory");
    let old_root_ffi = src_dir.join("ffi.rs");
    if old_root_ffi.exists() {
        fs::remove_file(old_root_ffi)?;
    }
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if name.starts_with("ffi_") {
                fs::remove_file(path)?;
            }
        }
    }
    for entry in fs::read_dir(out_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if name.starts_with("generated_") || name.starts_with("ffi_") {
                fs::remove_file(path)?;
            }
        }
    }
    let ffi_dir = src_dir.join("ffi");
    if ffi_dir.exists() {
        for entry in fs::read_dir(&ffi_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if name.starts_with("ffi_") {
                    fs::remove_file(path)?;
                }
            }
        }
    }

    Ok(())
}

fn write_ffi_wrapper(ffi_dir: &Path) -> std::io::Result<()> {
    let mut targets: Vec<(String, String)> = Vec::new();
    for entry in fs::read_dir(ffi_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let file_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => continue,
        };
        let stem = match file_name.strip_suffix(".rs") {
            Some(stem) => stem,
            None => continue,
        };
        if stem == "mod" || stem.starts_with("ffi_") {
            continue;
        }
        let (os, arch) = match stem.split_once('_') {
            Some((os, arch)) if !os.is_empty() && !arch.is_empty() => {
                (os.to_string(), arch.to_string())
            }
            _ => continue,
        };
        targets.push((os, arch));
    }
    targets.sort();
    targets.dedup();

    let mut out = String::new();
    for (os, arch) in targets {
        let module_name = format!("{}_{}", os.replace('-', "_"), arch.replace('-', "_"));
        out.push_str(&format!(
            r#"#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
mod {module_name};
#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
pub use {module_name}::*;

"#,
            os = os,
            arch = arch,
            module_name = module_name,
        ));
    }

    let out = dawn_codegen::emitter::format_rust_source(&out);
    fs::write(ffi_dir.join("mod.rs"), out)
}

fn write_generated_single_file(
    out_dir: &Path,
    target_os: &str,
    target_arch: &str,
    files: &dawn_codegen::GeneratedFiles,
) -> std::io::Result<()> {
    let combined = build_combined_generated_source(files);
    let target_file = out_dir.join(format!("{target_os}_{target_arch}.rs"));
    fs::write(target_file, &combined)?;

    Ok(())
}

fn write_generated_wrapper(out_dir: &Path) -> std::io::Result<()> {
    let mut targets: Vec<(String, String)> = Vec::new();
    for entry in fs::read_dir(out_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let file_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => continue,
        };
        let stem = match file_name.strip_suffix(".rs") {
            Some(stem) => stem,
            None => continue,
        };
        if stem == "mod"
            || stem.starts_with("ffi_")
            || stem.starts_with("generated_")
            || stem.starts_with("wire_")
        {
            continue;
        }
        let (os, arch) = match stem.split_once('_') {
            Some((os, arch)) if !os.is_empty() && !arch.is_empty() => {
                (os.to_string(), arch.to_string())
            }
            _ => continue,
        };
        targets.push((os, arch));
    }
    targets.sort();
    targets.dedup();

    let mut out = String::new();
    for (os, arch) in targets {
        let module_name = format!("{}_{}", os.replace('-', "_"), arch.replace('-', "_"));
        out.push_str(&format!(
            r#"#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
mod {module_name};

#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
pub use {module_name}::*;

"#,
            os = os,
            arch = arch,
            module_name = module_name,
        ));
    }

    let wire_types = out_dir.join("wire_types.rs");
    let wire_client = out_dir.join("wire_client.rs");
    let wire_server = out_dir.join("wire_server.rs");
    if wire_types.exists() && wire_client.exists() && wire_server.exists() {
        out.push_str(
            r#"#[cfg(feature = "wire")]
pub mod wire_types;
#[cfg(feature = "wire")]
pub mod wire_client;
#[cfg(feature = "wire")]
pub mod wire_server;

"#,
        );
    }

    let out = dawn_codegen::emitter::format_rust_source(&out);
    fs::write(out_dir.join("mod.rs"), out)
}

fn write_wire_files(
    out_dir: &Path,
    api: &dawn_codegen::parser::DawnApi,
    wire_json_path: &Path,
) -> std::io::Result<()> {
    let wire = dawn_codegen::DawnWireJsonParser::parse_file(wire_json_path)
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    let generated = dawn_codegen::generate_wire_files(api, &wire);
    fs::write(out_dir.join("wire_types.rs"), generated.wire_types)?;
    fs::write(out_dir.join("wire_client.rs"), generated.wire_client)?;
    fs::write(out_dir.join("wire_server.rs"), generated.wire_server)?;
    Ok(())
}

fn build_combined_generated_source(files: &dawn_codegen::GeneratedFiles) -> String {
    let mut out = String::new();

    out.push_str(&emit_inline_module("enums", &files.enums));
    out.push_str(&emit_inline_module("structs", &files.structs));
    out.push_str(&emit_inline_module("extensions", &files.extensions));
    out.push_str(&emit_inline_module("objects", &files.objects));
    out.push_str(&emit_inline_module("callbacks", &files.callbacks));
    out.push_str(&emit_inline_module("functions", &files.functions));
    out.push_str(&emit_inline_module("constants", &files.constants));

    out.push_str(
        r#"pub use enums::*;
pub use structs::*;
pub use extensions::*;
pub use objects::*;
pub use callbacks::*;
pub use functions::*;
pub use constants::*;
"#,
    );

    dawn_codegen::emitter::format_rust_source(&out)
}

fn emit_inline_module(name: &str, source: &str) -> String {
    format!(
        r#"mod {name} {{
        {source}
    }}"#,
    )
}
