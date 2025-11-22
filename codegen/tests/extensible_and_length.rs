//! Tests for ExtensibleType and LengthValue functionality

use dawn_codegen::{DawnJsonParser, Definition, ExtensibleType, LengthValue};

#[test]
fn test_extensible_types() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "not extensible": {
            "category": "structure",
            "members": [{"name": "field", "type": "uint32_t"}]
        },
        "extensible true": {
            "category": "structure",
            "extensible": true,
            "members": [{"name": "field", "type": "uint32_t"}]
        },
        "extensible false": {
            "category": "structure",
            "extensible": false,
            "members": [{"name": "field", "type": "uint32_t"}]
        },
        "extensible in": {
            "category": "structure",
            "extensible": "in",
            "members": [{"name": "field", "type": "uint32_t"}]
        },
        "extensible out": {
            "category": "structure",
            "extensible": "out",
            "members": [{"name": "field", "type": "uint32_t"}]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    // Test default extensible (false)
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("not extensible") {
        assert!(!struct_def.extensible.is_extensible());
        assert!(matches!(struct_def.extensible, ExtensibleType::Bool(false)));
        assert!(struct_def.extensible.direction().is_none());
    } else {
        panic!("Expected structure definition");
    }

    // Test explicitly true
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("extensible true") {
        assert!(struct_def.extensible.is_extensible());
        assert!(matches!(struct_def.extensible, ExtensibleType::Bool(true)));
        assert!(struct_def.extensible.direction().is_none());
    } else {
        panic!("Expected structure definition");
    }

    // Test explicitly false
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("extensible false") {
        assert!(!struct_def.extensible.is_extensible());
        assert!(matches!(struct_def.extensible, ExtensibleType::Bool(false)));
        assert!(struct_def.extensible.direction().is_none());
    } else {
        panic!("Expected structure definition");
    }

    // Test "in" direction
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("extensible in") {
        assert!(struct_def.extensible.is_extensible());
        assert!(matches!(
            struct_def.extensible,
            ExtensibleType::Direction(_)
        ));
        assert_eq!(struct_def.extensible.direction(), Some("in"));
    } else {
        panic!("Expected structure definition");
    }

    // Test "out" direction
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("extensible out") {
        assert!(struct_def.extensible.is_extensible());
        assert!(matches!(
            struct_def.extensible,
            ExtensibleType::Direction(_)
        ));
        assert_eq!(struct_def.extensible.direction(), Some("out"));
    } else {
        panic!("Expected structure definition");
    }
}

#[test]
fn test_extensible_type_helper_methods() {
    // Test boolean extensible
    let ext_true = ExtensibleType::Bool(true);
    assert!(ext_true.is_extensible());
    assert!(ext_true.is_boolean());
    assert!(!ext_true.is_input());
    assert!(!ext_true.is_output());
    assert!(ext_true.direction().is_none());

    let ext_false = ExtensibleType::Bool(false);
    assert!(!ext_false.is_extensible());
    assert!(ext_false.is_boolean());
    assert!(!ext_false.is_input());
    assert!(!ext_false.is_output());
    assert!(ext_false.direction().is_none());

    // Test directional extensible
    let ext_in = ExtensibleType::Direction("in".to_string());
    assert!(ext_in.is_extensible());
    assert!(!ext_in.is_boolean());
    assert!(ext_in.is_input());
    assert!(!ext_in.is_output());
    assert_eq!(ext_in.direction(), Some("in"));

    let ext_out = ExtensibleType::Direction("out".to_string());
    assert!(ext_out.is_extensible());
    assert!(!ext_out.is_boolean());
    assert!(!ext_out.is_input());
    assert!(ext_out.is_output());
    assert_eq!(ext_out.direction(), Some("out"));

    // Test default
    let ext_default = ExtensibleType::default();
    assert!(!ext_default.is_extensible());
    assert!(ext_default.is_boolean());
}

#[test]
fn test_real_world_extensible_values() {
    // Test with real extensible values from dawn.json - both string directions and booleans
    let json = r#"{
        "_metadata": {
            "api": "webgpu",
            "namespace": "wgpu",
            "c_prefix": "WGPU",
            "proc_table_prefix": "WGPU",
            "native_namespace": "dawn native"
        },
        "request adapter options": {
            "category": "structure",
            "extensible": "in",
            "members": [
                {"name": "feature level", "type": "feature level", "default": "core"},
                {"name": "power preference", "type": "power preference"}
            ]
        },
        "surface configuration": {
            "category": "structure",
            "extensible": "out",
            "members": [
                {"name": "width", "type": "uint32_t"},
                {"name": "height", "type": "uint32_t"}
            ]
        },
        "simple structure": {
            "category": "structure",
            "extensible": true,
            "members": [
                {"name": "value", "type": "uint32_t"}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    // Test "in" direction
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("request adapter options")
    {
        assert!(struct_def.extensible.is_extensible());
        assert!(struct_def.extensible.is_input());
        assert!(!struct_def.extensible.is_output());
        assert!(!struct_def.extensible.is_boolean());
        assert_eq!(struct_def.extensible.direction(), Some("in"));
    } else {
        panic!("Expected request adapter options structure");
    }

    // Test "out" direction
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("surface configuration") {
        assert!(struct_def.extensible.is_extensible());
        assert!(!struct_def.extensible.is_input());
        assert!(struct_def.extensible.is_output());
        assert!(!struct_def.extensible.is_boolean());
        assert_eq!(struct_def.extensible.direction(), Some("out"));
    } else {
        panic!("Expected surface configuration structure");
    }

    // Test boolean true
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("simple structure") {
        assert!(struct_def.extensible.is_extensible());
        assert!(!struct_def.extensible.is_input());
        assert!(!struct_def.extensible.is_output());
        assert!(struct_def.extensible.is_boolean());
        assert_eq!(struct_def.extensible.direction(), None);
    } else {
        panic!("Expected simple structure");
    }
}

#[test]
fn test_length_value_helper_methods() {
    // Test string length
    let length_string = LengthValue::String("count".to_string());
    assert!(length_string.is_string());
    assert!(!length_string.is_number());
    assert_eq!(length_string.as_string(), Some("count"));
    assert_eq!(length_string.as_number(), None);

    // Test numeric length
    let length_number = LengthValue::Number(42);
    assert!(!length_number.is_string());
    assert!(length_number.is_number());
    assert_eq!(length_number.as_string(), None);
    assert_eq!(length_number.as_number(), Some(42));
}

#[test]
fn test_length_value_parsing() {
    // Test parsing structure with string length
    let json_string_length = r#"{
        "_metadata": {
            "api": "webgpu",
            "namespace": "wgpu",
            "c_prefix": "WGPU",
            "proc_table_prefix": "WGPU",
            "native_namespace": "dawn native"
        },
        "test structure string length": {
            "category": "structure",
            "extensible": true,
            "members": [
                {"name": "count", "type": "uint32_t"},
                {"name": "data", "type": "void const *", "annotation": "*", "length": "count"}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json_string_length).unwrap();
    if let Some(Definition::Structure(struct_def)) =
        api.definitions.get("test structure string length")
    {
        assert_eq!(struct_def.members.len(), 2);
        assert_eq!(
            struct_def.members[1].length,
            Some(LengthValue::String("count".to_string()))
        );
    } else {
        panic!("Expected structure definition");
    }

    // Test parsing structure with numeric length
    let json_numeric_length = r#"{
        "_metadata": {
            "api": "webgpu",
            "namespace": "wgpu",
            "c_prefix": "WGPU",
            "proc_table_prefix": "WGPU",
            "native_namespace": "dawn native"
        },
        "test structure numeric length": {
            "category": "structure",
            "extensible": true,
            "members": [
                {"name": "data", "type": "uint32_t", "annotation": "*", "length": 16}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json_numeric_length).unwrap();
    if let Some(Definition::Structure(struct_def)) =
        api.definitions.get("test structure numeric length")
    {
        assert_eq!(struct_def.members.len(), 1);
        assert_eq!(struct_def.members[0].length, Some(LengthValue::Number(16)));
    } else {
        panic!("Expected structure definition");
    }
}

#[test]
fn test_complex_length_scenarios() {
    let json = r#"{
        "_metadata": {
            "api": "webgpu",
            "namespace": "wgpu",
            "c_prefix": "WGPU",
            "proc_table_prefix": "WGPU",
            "native_namespace": "dawn native"
        },
        "mixed length structure": {
            "category": "structure",
            "members": [
                {"name": "count", "type": "uint32_t"},
                {"name": "dynamic_array", "type": "item", "annotation": "*", "length": "count"},
                {"name": "fixed_array", "type": "uint32_t", "annotation": "*", "length": 4},
                {"name": "single_value", "type": "float"}
            ]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();
    if let Some(Definition::Structure(struct_def)) = api.definitions.get("mixed length structure") {
        assert_eq!(struct_def.members.len(), 4);

        // No length field
        assert!(struct_def.members[0].length.is_none());

        // String length reference
        assert_eq!(
            struct_def.members[1].length,
            Some(LengthValue::String("count".to_string()))
        );
        assert!(struct_def.members[1].length.as_ref().unwrap().is_string());

        // Numeric length
        assert_eq!(struct_def.members[2].length, Some(LengthValue::Number(4)));
        assert!(struct_def.members[2].length.as_ref().unwrap().is_number());

        // No length field
        assert!(struct_def.members[3].length.is_none());
    } else {
        panic!("Expected mixed length structure");
    }
}
