//! Tests for tag filtering and getter methods

use dawn_codegen::DawnJsonParser;

#[test]
fn test_filter_by_tags() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "dawn only": {
            "category": "enum",
            "tags": ["dawn"],
            "values": [{"name": "value1", "value": 1}]
        },
        "emscripten only": {
            "category": "enum",
            "tags": ["emscripten"],
            "values": [{"name": "value2", "value": 2}]
        },
        "no tags": {
            "category": "enum",
            "values": [{"name": "value3", "value": 3}]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();
    assert_eq!(api.definitions.len(), 3);

    let dawn_filtered = api.filter_by_tags(&["dawn".to_string()]);
    assert_eq!(dawn_filtered.definitions.len(), 2); // "dawn only" + "no tags"
    assert!(dawn_filtered.definitions.contains_key("dawn only"));
    assert!(dawn_filtered.definitions.contains_key("no tags"));
    assert!(!dawn_filtered.definitions.contains_key("emscripten only"));

    let emscripten_filtered = api.filter_by_tags(&["emscripten".to_string()]);
    assert_eq!(emscripten_filtered.definitions.len(), 2); // "emscripten only" + "no tags"
    assert!(
        emscripten_filtered
            .definitions
            .contains_key("emscripten only")
    );
    assert!(emscripten_filtered.definitions.contains_key("no tags"));
    assert!(!emscripten_filtered.definitions.contains_key("dawn only"));
}

#[test]
fn test_get_specific_definitions() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "test enum": {
            "category": "enum",
            "values": [{"name": "value1", "value": 1}]
        },
        "test structure": {
            "category": "structure",
            "members": [{"name": "field", "type": "uint32_t"}]
        },
        "test object": {
            "category": "object",
            "methods": [{"name": "method1", "args": []}]
        },
        "test function": {
            "category": "function",
            "returns": "void",
            "args": []
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    let enums = api.enums();
    assert_eq!(enums.len(), 1);
    assert_eq!(enums[0].0, "test enum");

    let structures = api.structures();
    assert_eq!(structures.len(), 1);
    assert_eq!(structures[0].0, "test structure");

    let objects = api.objects();
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].0, "test object");

    let functions = api.functions();
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].0, "test function");
}

#[test]
fn test_empty_filter() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "tagged item": {
            "category": "enum",
            "tags": ["dawn"],
            "values": [{"name": "value1", "value": 1}]
        },
        "untagged item": {
            "category": "enum",
            "values": [{"name": "value2", "value": 2}]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();
    assert_eq!(api.definitions.len(), 2);

    // Empty filter should return only untagged items
    let empty_filtered = api.filter_by_tags(&[]);
    assert_eq!(empty_filtered.definitions.len(), 1);
    assert!(empty_filtered.definitions.contains_key("untagged item"));
    assert!(!empty_filtered.definitions.contains_key("tagged item"));
}

#[test]
fn test_multiple_tags_filter() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "dawn only": {
            "category": "enum",
            "tags": ["dawn"],
            "values": [{"name": "value1", "value": 1}]
        },
        "emscripten only": {
            "category": "enum",
            "tags": ["emscripten"],
            "values": [{"name": "value2", "value": 2}]
        },
        "both tags": {
            "category": "enum",
            "tags": ["dawn", "emscripten"],
            "values": [{"name": "value3", "value": 3}]
        },
        "no tags": {
            "category": "enum",
            "values": [{"name": "value4", "value": 4}]
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();
    assert_eq!(api.definitions.len(), 4);

    // Filter for both dawn and emscripten
    let multi_filtered = api.filter_by_tags(&["dawn".to_string(), "emscripten".to_string()]);
    assert_eq!(multi_filtered.definitions.len(), 4); // All items should be included
    assert!(multi_filtered.definitions.contains_key("dawn only"));
    assert!(multi_filtered.definitions.contains_key("emscripten only"));
    assert!(multi_filtered.definitions.contains_key("both tags"));
    assert!(multi_filtered.definitions.contains_key("no tags"));
}

#[test]
fn test_get_all_callback_types() {
    let json = r#"{
        "_metadata": {
            "api": "WebGPU",
            "namespace": "wgpu",
            "proc_table_prefix": "Dawn",
            "native_namespace": "dawn native"
        },
        "regular callback": {
            "category": "callback",
            "returns": "void",
            "args": [{"name": "status", "type": "status"}]
        },
        "callback function": {
            "category": "callback function",
            "args": [{"name": "result", "type": "result"}]
        },
        "callback info": {
            "category": "callback info",
            "returns": "void",
            "args": [],
            "members": []
        }
    }"#;

    let api = DawnJsonParser::parse_string(json).unwrap();

    let callbacks = api.callbacks();
    assert_eq!(callbacks.len(), 1);
    assert_eq!(callbacks[0].0, "regular callback");

    let callback_functions = api.callback_functions();
    assert_eq!(callback_functions.len(), 1);
    assert_eq!(callback_functions[0].0, "callback function");

    let callback_infos = api.callback_infos();
    assert_eq!(callback_infos.len(), 1);
    assert_eq!(callback_infos[0].0, "callback info");
}
