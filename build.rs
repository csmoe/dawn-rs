use std::path::PathBuf;

use dawn_codegen::{
    Annotation, DawnJsonParser, Definition, ExtensibleType, LengthValue, ReturnType, codegen,
};

fn main() {
    let outdir = std::env::var("OUT_DIR").unwrap();
    let manifest_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = manifest_path.join("target");
    let target_os = build_rs::input::cargo_cfg_target_os();
    let ret = DawnJsonParser::parse_file(target_dir.join("dawn.json"))
        .unwrap()
        .codegen();
    std::fs::write(format!("{}/dawn.rs", outdir), ret).unwrap();
    let output = format!("{}/dawn-capi.rs", outdir);
    codegen(&target_dir.join("dawn"), &PathBuf::from(output), &target_os);

    build_rs::output::rustc_link_search(target_dir.join("dawn").join("lib"));
    build_rs::output::rustc_link_lib_kind("static", "webgpu_dawn");

    build_rs::output::rerun_if_changed(manifest_path.join("codgen"));
}
