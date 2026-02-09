use std::collections::HashSet;
use std::env;
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

    let api =
        dawn_codegen::DawnJsonParser::parse_file(&dawn_json).expect("parse dawn.json");
    let filtered = if tags.is_empty() {
        api
    } else {
        api.filter_by_tags(&tags)
    };
    let model = dawn_codegen::ApiModel::from_api(&filtered);
    let ffi_consts = collect_ffi_enum_consts(&api_header);
    let files = dawn_codegen::generate_strings_with_ffi_consts(&model, Some(&ffi_consts));
    files
        .write_to_dir(&out_dir)
        .expect("write generated files");

    std::fs::create_dir_all(&out_dir).expect("create output dir");
    let ffi_out = out_dir.join("ffi.rs");

    let mut combined_clang_args = clang_args;
    combined_clang_args.extend(default_clang_args(&api_header));

    let ffi_rs = dawn_codegen::generate_ffi_string(&api_header, &combined_clang_args)
        .expect("generate ffi.rs");
    std::fs::write(&ffi_out, ffi_rs).expect("write ffi.rs");
}

fn default_clang_args(api_header: &Path) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(include_dir) = api_header.parent().and_then(|p| p.parent()) {
        args.push(format!("-I{}", include_dir.display()));
    }
    args
}

fn collect_ffi_enum_consts(api_header: &Path) -> HashSet<String> {
    let mut contents = String::new();
    if let Ok(text) = std::fs::read_to_string(api_header) {
        contents.push_str(&text);
        contents.push('\n');
    }
    if let Some(include_root) = api_header.parent().and_then(|p| p.parent()) {
        let dawn_header = include_root.join("dawn").join("webgpu.h");
        if let Ok(text) = std::fs::read_to_string(&dawn_header) {
            contents.push_str(&text);
            contents.push('\n');
        }
    }
    let mut out = HashSet::new();
    for line in contents.lines() {
        if !line.contains("WGPU") || !line.contains('=') {
            continue;
        }
        let (left, _) = match line.split_once('=') {
            Some(parts) => parts,
            None => continue,
        };
        let token = match left.split_whitespace().last() {
            Some(value) => value.trim_end_matches(','),
            None => continue,
        };
        if !token.starts_with("WGPU") {
            continue;
        }
        out.insert(token.to_string());
        if let Some((enum_type, _)) = token.split_once('_') {
            out.insert(format!("{enum_type}_{token}"));
        }
    }
    out
}
