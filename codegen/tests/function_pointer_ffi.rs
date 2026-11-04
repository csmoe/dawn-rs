use dawn_codegen::{ApiModel, DawnJsonParser, generate_strings};

#[test]
fn function_pointer_uses_ffi_enum_type() {
    let api = DawnJsonParser::parse_string(
        r#"
{
    "_metadata": {
        "api": "webgpu",
        "namespace": "wgpu",
        "c_prefix": "WGPU",
        "proc_table_prefix": "wgpu",
        "native_namespace": "dawn::native"
    },
    "callback status": {
        "category": "enum",
        "values": [
            {
                "value": 1,
                "name": "success"
            }
        ]
    },
    "dispose callback": {
        "category": "function pointer",
        "args": [
            {
                "name": "status",
                "type": "callback status"
            }
        ]
    }
}
"#,
    )
    .unwrap();

    api.validate().unwrap();

    let model = ApiModel::from_api(&api);
    let generated = generate_strings(&model);

    assert!(
        generated.callbacks.contains(
            "pub type DisposeCallback = Option<unsafe extern \"C\" fn(ffi::WGPUCallbackStatus) -> ()>;"
        )
    );
}
