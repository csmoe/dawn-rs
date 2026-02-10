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
    files.write_to_dir(&out_dir).expect("write generated files");

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
