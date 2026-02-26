use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let mut dawn_json: Option<PathBuf> = None;
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

    let api = dawn_codegen::DawnJsonParser::parse_file(&dawn_json).expect("parse dawn.json");
    let filtered = if tags.is_empty() {
        api
    } else {
        api.filter_by_tags(&tags)
    };
    let model = dawn_codegen::ApiModel::from_api(&filtered);
    let files = dawn_codegen::generate_strings(&model);

    std::fs::create_dir_all(&out_dir).expect("create output dir");
    let ffi_dir = out_dir.join("ffi");
    fs::create_dir_all(&ffi_dir).expect("create ffi output dir");
    let target_os = env::consts::OS;
    let target_arch = env::consts::ARCH;
    let ffi_out = ffi_dir.join(format!("{target_os}_{target_arch}.rs"));

    let mut combined_clang_args = clang_args;
    combined_clang_args.extend(default_clang_args(&api_header));

    let ffi_rs = dawn_codegen::generate_ffi_string(&api_header, &combined_clang_args)
        .expect("generate ffi.rs");
    std::fs::write(&ffi_out, ffi_rs).expect("write ffi bindings");

    write_generated_single_files(&out_dir, &ffi_dir, &files).expect("write generated single files");
    write_generated_wrapper(&out_dir).expect("write generated wrapper");
    write_ffi_wrapper(&out_dir, &ffi_dir).expect("write ffi wrapper");
}

fn default_clang_args(api_header: &Path) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(include_dir) = api_header.parent().and_then(|p| p.parent()) {
        args.push(format!("-I{}", include_dir.display()));
    }
    args
}

fn write_ffi_wrapper(out_dir: &Path, ffi_dir: &Path) -> std::io::Result<()> {
    let mut targets: Vec<(String, String, String)> = Vec::new();
    for entry in fs::read_dir(ffi_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let file_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };
        let stem = match file_name.strip_suffix(".rs") {
            Some(stem) => stem,
            None => continue,
        };
        let (os, arch) = match stem.split_once('_') {
            Some((os, arch)) if !os.is_empty() && !arch.is_empty() => {
                (os.to_string(), arch.to_string())
            }
            _ => continue,
        };
        targets.push((os, arch, file_name));
    }
    targets.sort_by(|a, b| a.2.cmp(&b.2));

    let mut out = String::new();
    for (os, arch, file_name) in targets {
        let module_name = format!("ffi_{}_{}", os.replace('-', "_"), arch.replace('-', "_"));
        out.push_str(&format!(
            r#"#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
            mod {module_name} {{
                include!("ffi/{file_name}");
            }}
            "#,
            os = os,
            arch = arch,
            module_name = module_name,
            file_name = file_name
        ));
        out.push_str(&format!(
            r#"#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
            pub use {module_name}::*;
            "#,
            os = os,
            arch = arch,
            module_name = module_name
        ));
    }

    let out = dawn_codegen::emitter::format_rust_source(&out);
    fs::write(out_dir.join("ffi.rs"), out)
}

fn write_generated_single_files(
    out_dir: &Path,
    ffi_dir: &Path,
    files: &dawn_codegen::GeneratedFiles,
) -> std::io::Result<()> {
    let combined = build_combined_generated_source(files);
    let generated_dir = out_dir.join("targets");
    fs::create_dir_all(&generated_dir)?;

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
        let (os, arch) = match stem.split_once('_') {
            Some((os, arch)) if !os.is_empty() && !arch.is_empty() => {
                (os.to_string(), arch.to_string())
            }
            _ => continue,
        };
        targets.push((os, arch));
    }
    if targets.is_empty() {
        targets.push((env::consts::OS.to_string(), env::consts::ARCH.to_string()));
    }
    targets.sort();
    targets.dedup();

    for (os, arch) in targets {
        let target_file = generated_dir.join(format!("{os}_{arch}.rs"));
        fs::write(target_file, &combined)?;
    }

    Ok(())
}

fn write_generated_wrapper(out_dir: &Path) -> std::io::Result<()> {
    let generated_dir = out_dir.join("targets");
    let mut targets: Vec<(String, String, String)> = Vec::new();
    for entry in fs::read_dir(&generated_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let file_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };
        let stem = match file_name.strip_suffix(".rs") {
            Some(stem) => stem,
            None => continue,
        };
        let (os, arch) = match stem.split_once('_') {
            Some((os, arch)) if !os.is_empty() && !arch.is_empty() => {
                (os.to_string(), arch.to_string())
            }
            _ => continue,
        };
        targets.push((os, arch, file_name));
    }
    targets.sort_by(|a, b| a.2.cmp(&b.2));

    let mut out = String::new();
    for (os, arch, file_name) in &targets {
        let module_name = format!(
            "generated_{}_{}",
            os.replace('-', "_"),
            arch.replace('-', "_")
        );
        out.push_str(&format!(
            r#"#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
mod {module_name} {{
    include!("targets/{file_name}");
}}

#[cfg(all(target_os = "{os}", target_arch = "{arch}"))]
pub use {module_name}::*;

"#,
            os = os,
            arch = arch,
            module_name = module_name,
            file_name = file_name
        ));
    }

    let out = dawn_codegen::emitter::format_rust_source(&out);
    fs::write(out_dir.join("mod.rs"), out)
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
        "mod {name} {{\n{body}\n}}\n\n",
        name = name,
        body = indent_block(source, 4)
    )
}

fn indent_block(source: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    source
        .lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{prefix}{line}", prefix = prefix, line = line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
