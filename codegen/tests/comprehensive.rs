//! Comprehensive integration tests for the Dawn JSON Parser

use dawn_codegen::{
    ReturnType,
    parser::{DawnJsonParser, Definition, ExtensibleType, LengthValue},
};

#[test]
fn test_parse_dawn_json_sample() {
    // Test with a realistic sample from dawn.json
    let json = r#"{
        "_comment": ["Sample Dawn JSON"],
        "_doc": "See docs/dawn/codegen.md",
        "_metadata": {
            "api": "WebGPU",
            "c_prefix": "WGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native",
            "copyright_year": "2019"
        },
        "buffer usage": {
            "category": "bitmask",
            "values": [
                {"value": "0x00000000", "name": "none"},
                {"value": "0x00000001", "name": "map read"},
                {"value": "0x00000002", "name": "map write"},
                {"value": "0x00000004", "name": "copy src"},
                {"value": "0x00000008", "name": "copy dst"}
            ]
        },
        "texture format": {
            "category": "enum",
            "values": [
                {"value": 1, "name": "r8 unorm", "jsrepr": "r8unorm", "valid": true},
                {"value": 2, "name": "r8 snorm"},
                {"value": 3, "name": "r8 uint"}
            ]
        },
        "buffer descriptor": {
            "category": "structure",
            "extensible": true,
            "members": [
                {"name": "label", "type": "string view", "optional": true},
                {"name": "usage", "type": "buffer usage"},
                {"name": "size", "type": "uint64_t"},
                {"name": "mapped at creation", "type": "bool", "default": "false"}
            ]
        },
        "surface capabilities": {
            "category": "structure",
            "extensible": "out",
            "members": [
                {"name": "usage", "type": "texture usage"}
            ]
        },
        "device": {
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
        },
        "create instance": {
            "category": "function",
            "returns": "instance",
            "_comment": "Creates a new WebGPU instance",
            "args": [
                {"name": "descriptor", "type": "instance descriptor", "annotation": "*", "optional": true}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    // Verify metadata
    assert_eq!(api.metadata.api, "WebGPU");
    assert_eq!(api.metadata.namespace, "wgpu");
    assert_eq!(api.metadata.proc_table_prefix, "Dawn");
    assert_eq!(api.metadata.c_prefix, Some("WGPU".to_string()));

    // Verify we have all expected definitions
    assert_eq!(api.definitions.len(), 6);
    assert!(api.definitions.contains_key("buffer usage"));
    assert!(api.definitions.contains_key("texture format"));
    assert!(api.definitions.contains_key("buffer descriptor"));
    assert!(api.definitions.contains_key("surface capabilities"));
    assert!(api.definitions.contains_key("device"));
    assert!(api.definitions.contains_key("create instance"));

    // Test bitmask
    if let Some(Definition::Bitmask(bitmask)) = api.definitions.get("buffer usage") {
        assert_eq!(bitmask.values.len(), 5);
        assert_eq!(bitmask.values[0].name, "none");
        assert_eq!(bitmask.values[1].name, "map read");
    } else {
        panic!("Expected bitmask definition");
    }

    // Test enum
    if let Some(Definition::Enum(enum_def)) = api.definitions.get("texture format") {
        assert_eq!(enum_def.values.len(), 3);
        assert_eq!(enum_def.values[0].jsrepr, Some("r8unorm".to_string()));
        assert_eq!(enum_def.values[0].valid, true);
    } else {
        panic!("Expected enum definition");
    }

    // Test structure with boolean extensible
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("buffer descriptor") {
        assert!(struct_def.extensible.is_extensible());
        assert!(matches!(struct_def.extensible, ExtensibleType::Bool(true)));
        assert_eq!(struct_def.members.len(), 4);
        assert!(struct_def.members[0].optional);
        assert_eq!(
            struct_def.members[3].default,
            Some(serde_json::Value::String("false".to_string()))
        );
    } else {
        panic!("Expected structure definition");
    }

    // Test structure with directional extensible
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("surface capabilities") {
        assert!(struct_def.extensible.is_extensible());
        assert!(matches!(
            struct_def.extensible,
            ExtensibleType::Direction(_)
        ));
        assert_eq!(struct_def.extensible.direction(), Some("out"));
    } else {
        panic!("Expected structure definition");
    }

    // Test object
    if let Some(Definition::Object(object_def)) = api.definitions.get("device") {
        assert_eq!(object_def.methods.len(), 2);
        assert_eq!(object_def.methods[0].name, "create buffer");
        assert_eq!(
            object_def.methods[0].returns,
            Some(ReturnType::Simple("buffer".to_string()))
        );
        assert_eq!(object_def.methods[0].args.len(), 1);
    } else {
        panic!("Expected object definition");
    }

    // Test function
    if let Some(Definition::Function(func_def)) = api.definitions.get("create instance") {
        assert_eq!(
            func_def.returns(),
            Some(&ReturnType::Simple("instance".to_string()))
        );
        assert_eq!(func_def.args().len(), 1);
        assert!(func_def.args()[0].optional);
    } else {
        panic!("Expected function definition");
    }

    // Test helper functions
    let enums = DawnJsonParser::get_enums(&api);
    assert_eq!(enums.len(), 1);

    let bitmasks = DawnJsonParser::get_bitmasks(&api);
    assert_eq!(bitmasks.len(), 1);

    let structures = DawnJsonParser::get_structures(&api);
    assert_eq!(structures.len(), 2);

    let objects = DawnJsonParser::get_objects(&api);
    assert_eq!(objects.len(), 1);

    let functions = DawnJsonParser::get_functions(&api);
    assert_eq!(functions.len(), 1);

    let callback_functions = DawnJsonParser::get_callback_functions(&api);
    assert_eq!(callback_functions.len(), 0); // No callback functions in this test

    println!(
        "✓ Dawn JSON parser test passed! Parsed {} definitions successfully.",
        api.definitions.len()
    );
}

#[test]
fn test_record_member_defaults() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "test structure": {
            "category": "structure",
            "members": [
                {"name": "basic_field", "type": "uint32_t"},
                {"name": "pointer_field", "type": "data", "annotation": "*"},
                {"name": "array_field", "type": "item", "annotation": "*", "length": "count"},
                {"name": "optional_field", "type": "value", "optional": true}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    if let Some(Definition::Structure(struct_def)) = api.definitions.get("test structure") {
        // Test default annotation
        assert_eq!(struct_def.members[0].annotation, "value");
        assert_eq!(struct_def.members[1].annotation, "*");

        // Test optional field
        assert!(!struct_def.members[0].optional);
        assert!(struct_def.members[3].optional);

        // Test length field
        assert_eq!(
            struct_def.members[2].length,
            Some(LengthValue::String("count".to_string()))
        );
        assert!(struct_def.members[0].length.is_none());
    } else {
        panic!("Expected structure definition");
    }
}

#[test]
fn test_parse_real_dawn_json() {
    // Test parsing the actual dawn.json file if it exists
    let dawn_json_path = "../target/tmp/dawn.json";

    if std::path::Path::new(dawn_json_path).exists() {
        let result = DawnJsonParser::parse_file(dawn_json_path);

        match result {
            Ok(api) => {
                println!("✓ Successfully parsed real dawn.json!");
                println!("  API: {}", api.metadata.api);
                println!("  Namespace: {}", api.metadata.namespace);
                println!("  Total definitions: {}", api.definitions.len());

                // Count definition types
                let mut type_counts = std::collections::HashMap::new();
                for (_, def) in &api.definitions {
                    let def_type = match def {
                        Definition::Native(_) => "native",
                        Definition::Typedef(_) => "typedef",
                        Definition::Enum(_) => "enum",
                        Definition::Bitmask(_) => "bitmask",
                        Definition::FunctionPointer(_) => "function pointer",
                        Definition::Structure(_) => "structure",
                        Definition::Object(_) => "object",
                        Definition::Constant(_) => "constant",
                        Definition::Function(_) => "function",
                        Definition::Callback(_) => "callback",
                        Definition::CallbackFunction(_) => "callback function",
                        Definition::CallbackInfo(_) => "callback info",
                    };
                    *type_counts.entry(def_type).or_insert(0) += 1;
                }

                println!("  Definition counts:");
                for (def_type, count) in &type_counts {
                    println!("    {}: {}", def_type, count);
                }

                // Test specific getters
                let enums = DawnJsonParser::get_enums(&api);
                let structures = DawnJsonParser::get_structures(&api);
                let objects = DawnJsonParser::get_objects(&api);
                let callback_functions = DawnJsonParser::get_callback_functions(&api);

                println!("  Getter results:");
                println!("    Enums: {}", enums.len());
                println!("    Structures: {}", structures.len());
                println!("    Objects: {}", objects.len());
                println!("    Callback functions: {}", callback_functions.len());

                // Verify we have some expected definitions
                assert!(api.definitions.len() > 100);
                assert!(enums.len() > 10);
                assert!(structures.len() > 10);
                assert!(objects.len() > 5);

                // Note: callback functions might be 0 in some versions of dawn.json
                // so we don't assert their count
            }
            Err(e) => {
                panic!("Failed to parse real dawn.json: {}", e);
            }
        }
    } else {
        println!(
            "⚠ Skipping real dawn.json test - file not found at {}",
            dawn_json_path
        );
    }
}

#[test]
fn test_all_definition_categories() {
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
        "size_t": {
            "category": "typedef",
            "type": "uint64_t"
        },
        "test enum": {
            "category": "enum",
            "values": [{"name": "value1", "value": 1}]
        },
        "test bitmask": {
            "category": "bitmask",
            "values": [{"name": "flag1", "value": "0x1"}]
        },
        "callback ptr": {
            "category": "function pointer",
            "returns": "void",
            "args": []
        },
        "test struct": {
            "category": "structure",
            "members": [{"name": "field", "type": "uint32_t"}]
        },
        "test object": {
            "category": "object",
            "methods": [{"name": "method", "args": []}]
        },
        "test constant": {
            "category": "constant",
            "type": "uint32_t",
            "value": "42"
        },
        "test function": {
            "category": "function",
            "returns": "void",
            "args": []
        },
        "test callback": {
            "category": "callback",
            "returns": "void",
            "args": []
        },
        "test callback function": {
            "category": "callback function",
            "args": []
        },
        "test callback info": {
            "category": "callback info",
            "returns": "void",
            "args": [],
            "members": []
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();
    assert_eq!(api.definitions.len(), 12);

    // Verify each category exists and can be retrieved
    assert!(matches!(
        api.definitions.get("void"),
        Some(Definition::Native(_))
    ));
    assert!(matches!(
        api.definitions.get("size_t"),
        Some(Definition::Typedef(_))
    ));
    assert!(matches!(
        api.definitions.get("test enum"),
        Some(Definition::Enum(_))
    ));
    assert!(matches!(
        api.definitions.get("test bitmask"),
        Some(Definition::Bitmask(_))
    ));
    assert!(matches!(
        api.definitions.get("callback ptr"),
        Some(Definition::FunctionPointer(_))
    ));
    assert!(matches!(
        api.definitions.get("test struct"),
        Some(Definition::Structure(_))
    ));
    assert!(matches!(
        api.definitions.get("test object"),
        Some(Definition::Object(_))
    ));
    assert!(matches!(
        api.definitions.get("test constant"),
        Some(Definition::Constant(_))
    ));
    assert!(matches!(
        api.definitions.get("test function"),
        Some(Definition::Function(_))
    ));
    assert!(matches!(
        api.definitions.get("test callback"),
        Some(Definition::Callback(_))
    ));
    assert!(matches!(
        api.definitions.get("test callback function"),
        Some(Definition::CallbackFunction(_))
    ));
    assert!(matches!(
        api.definitions.get("test callback info"),
        Some(Definition::CallbackInfo(_))
    ));

    println!("✓ All 12 definition categories parsed successfully!");
}
