//! Basic parsing tests for Dawn JSON Parser

use dawn_codegen::{
    ReturnType, {DawnJsonParser, Definition, LengthValue},
};

#[test]
fn test_parse_simple_json() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "simple enum": {
            "category": "enum",
            "values": [
                {"name": "value1", "value": 1},
                {"name": "value2", "value": 2}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();
    assert_eq!(api.metadata.api, "WebGPU");
    assert_eq!(api.definitions.len(), 1);

    if let Some(Definition::Enum(enum_def)) = api.definitions.get("simple enum") {
        assert_eq!(enum_def.values.len(), 2);
        assert_eq!(enum_def.values[0].name, "value1");
    } else {
        panic!("Expected enum definition");
    }
}

#[test]
fn test_parse_structure() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "test structure": {
            "category": "structure",
            "extensible": true,
            "members": [
                {"name": "count", "type": "uint32_t"},
                {"name": "data", "type": "void const *", "annotation": "*", "length": "count"},
                {"name": "optional_field", "type": "string", "optional": true}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    if let Some(Definition::Structure(struct_def)) = api.definitions.get("test structure") {
        assert!(struct_def.extensible.is_extensible());
        assert_eq!(struct_def.members.len(), 3);
        assert_eq!(struct_def.members[0].name, "count");
        assert_eq!(struct_def.members[0].member_type, "uint32_t");
        assert_eq!(struct_def.members[1].annotation, "*");
        assert_eq!(
            struct_def.members[1].length,
            Some(LengthValue::String("count".to_string()))
        );
        assert!(struct_def.members[2].optional);
    } else {
        panic!("Expected structure definition");
    }
}

#[test]
fn test_parse_object() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "test object": {
            "category": "object",
            "methods": [
                {
                    "name": "create buffer",
                    "returns": "buffer",
                    "args": [
                        {"name": "descriptor", "type": "buffer descriptor", "annotation": "*"}
                    ]
                },
                {
                    "name": "destroy",
                    "args": []
                }
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    if let Some(Definition::Object(object_def)) = api.definitions.get("test object") {
        assert_eq!(object_def.methods.len(), 2);
        assert_eq!(object_def.methods[0].name, "create buffer");
        assert_eq!(
            object_def.methods[0].returns,
            Some(ReturnType::Simple("buffer".to_string()))
        );
        assert_eq!(object_def.methods[0].args.len(), 1);
        assert_eq!(object_def.methods[1].name, "destroy");
        assert_eq!(object_def.methods[1].args.len(), 0);
    } else {
        panic!("Expected object definition");
    }
}

#[test]
fn test_parse_bitmask() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "test bitmask": {
            "category": "bitmask",
            "values": [
                {"name": "none", "value": "0x00000000"},
                {"name": "flag1", "value": "0x00000001"},
                {"name": "flag2", "value": "0x00000002"}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    if let Some(Definition::Bitmask(bitmask_def)) = api.definitions.get("test bitmask") {
        assert_eq!(bitmask_def.values.len(), 3);
        assert_eq!(bitmask_def.values[0].name, "none");
        assert_eq!(bitmask_def.values[1].name, "flag1");
    } else {
        panic!("Expected bitmask definition");
    }
}

#[test]
fn test_parse_function_pointer() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "test callback": {
            "category": "function pointer",
            "returns": "void",
            "args": [
                {"name": "status", "type": "status"},
                {"name": "user_data", "type": "void *", "annotation": "*"}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    if let Some(Definition::FunctionPointer(func_ptr)) = api.definitions.get("test callback") {
        assert_eq!(
            func_ptr.returns(),
            Some(&ReturnType::Simple("void".to_string()))
        );
        assert_eq!(func_ptr.args().len(), 2);
        assert_eq!(func_ptr.args()[0].name, "status");
        assert_eq!(func_ptr.args()[1].annotation, "*");
    } else {
        panic!("Expected function pointer definition");
    }
}

#[test]
fn test_parse_constant() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "test constant": {
            "category": "constant",
            "type": "uint64_t",
            "value": "0xFFFFFFFFFFFFFFFF"
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    if let Some(Definition::Constant(constant_def)) = api.definitions.get("test constant") {
        assert_eq!(constant_def.const_type, "uint64_t");
    } else {
        panic!("Expected constant definition");
    }
}

#[test]
fn test_parse_native_type() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "void": {
            "category": "native"
        },
        "void *": {
            "category": "native",
            "is nullable pointer": true,
            "wasm type": "i32"
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    if let Some(Definition::Native(native_def)) = api.definitions.get("void") {
        assert!(native_def.wire_transparent); // default true
        assert!(native_def.wasm_type.is_none());
    } else {
        panic!("Expected native definition for void");
    }

    if let Some(Definition::Native(native_def)) = api.definitions.get("void *") {
        assert_eq!(native_def.is_nullable_pointer, Some(true));
        assert_eq!(native_def.wasm_type, Some("i32".to_string()));
    } else {
        panic!("Expected native definition for void *");
    }
}

#[test]
fn test_callback_function_parsing() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "request adapter callback": {
            "category": "callback function",
            "args": [
                {"name": "status", "type": "request adapter status"},
                {"name": "adapter", "type": "adapter", "optional": true},
                {"name": "message", "type": "string view"}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    // Verify callback function was parsed correctly
    if let Some(Definition::CallbackFunction(callback_def)) =
        api.definitions.get("request adapter callback")
    {
        assert_eq!(callback_def.args.len(), 3);
        assert_eq!(callback_def.args[0].name, "status");
        assert_eq!(callback_def.args[0].member_type, "request adapter status");
        assert_eq!(callback_def.args[1].name, "adapter");
        assert!(callback_def.args[1].optional);
        assert_eq!(callback_def.args[2].name, "message");
        assert_eq!(callback_def.args[2].member_type, "string view");
    } else {
        panic!("Expected callback function definition");
    }

    // Test getter function
    let callback_functions = DawnJsonParser::get_callback_functions(&api);
    assert_eq!(callback_functions.len(), 1);
    assert_eq!(callback_functions[0].0, "request adapter callback");

    println!(
        "✓ Callback function parsing test passed! Successfully parsed callback function with {} args",
        callback_functions[0].1.args.len()
    );
}
