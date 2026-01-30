use std::process::Command;

pub enum Target {
    Dawn,
    DawnWire,
}

impl Target {
    fn c_api(&self) -> &str {
        match self {
            Target::Dawn => "include/webgpu/webgpu.h",
            Target::DawnWire => "include/dawn/wire/client/webgpu.h",
        }
    }
}

pub fn codegen(dawn_path: &std::path::Path, output: &std::path::Path, os: &str, target: Target) {
    let mut builder = bindgen::builder()
        .header(dawn_path.join(target.c_api()).to_string_lossy())
        .clang_arg(format!("-I{}/include", dawn_path.display()))
        .detect_include_paths(true)
        .clang_arg(format!("--sysroot={}", sdk_path()))
        .use_core();
    match os {
        "macos" => {
            builder = builder.clang_arg(format!("--sysroot={}", sdk_path()));
        }
        _ => {}
    }

    let bindings = builder.generate().unwrap();
    bindings.write_to_file(output).unwrap();
}

fn sdk_path() -> String {
    String::from_utf8_lossy(
        &Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .to_string()
}
