use crate::emitter::core::type_name;

pub(crate) fn indent_block(text: &str, spaces: usize) -> String {
    if text.is_empty() {
        return String::new();
    }
    let pad = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{pad}{line}", pad = pad, line = line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn rust_type_for(name: &str) -> String {
    match name {
        "void" => "std::ffi::c_void".to_string(),
        "void *" => "*mut std::ffi::c_void".to_string(),
        "void const *" => "*const std::ffi::c_void".to_string(),
        "bool" => "bool".to_string(),
        "int" => "i32".to_string(),
        "uint" => "u32".to_string(),
        "uint8_t" => "u8".to_string(),
        "uint16_t" => "u16".to_string(),
        "uint32_t" => "u32".to_string(),
        "uint64_t" => "u64".to_string(),
        "int8_t" => "i8".to_string(),
        "int16_t" => "i16".to_string(),
        "int32_t" => "i32".to_string(),
        "int64_t" => "i64".to_string(),
        "size_t" => "usize".to_string(),
        "float" => "f32".to_string(),
        "double" => "f64".to_string(),
        "char" => "std::os::raw::c_char".to_string(),
        "string view" => "String".to_string(),
        other => type_name(other),
    }
}
