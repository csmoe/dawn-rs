use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};

pub(crate) fn type_name(name: &str) -> String {
    let raw = camel_case_with_acronyms(name);
    normalize_digit_prefix_camel(&raw)
}
pub(crate) fn enum_variant_name_camel(name: &str) -> String {
    let raw = camel_case_with_acronyms(name);
    normalize_digit_prefix_camel(&raw)
}
pub(crate) fn ffi_enum_const_name(ffi_type: &str, variant: &str) -> String {
    format!("{ffi_type}_{ffi_type}_{variant}")
}
pub(crate) fn ffi_bitmask_const_name(ffi_type: &str, variant: &str) -> String {
    format!("{ffi_type}_{variant}")
}
pub(crate) fn ffi_enum_value_name(name: &str) -> String {
    if name.trim().eq_ignore_ascii_case("srgb") {
        return "SRGB".to_string();
    }
    let mut out = String::new();
    for word in name.split_whitespace() {
        let mut parts = Vec::new();
        for part in word.split('-') {
            if part.is_empty() {
                continue;
            }
            let value = if part.chars().all(|c| c.is_ascii_digit()) {
                part.to_string()
            } else if part
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            {
                part.to_string()
            } else {
                let mut chars = part.chars();
                let mut s = String::new();
                if let Some(first) = chars.next() {
                    s.push(first.to_ascii_uppercase());
                    s.push_str(&chars.as_str().to_ascii_lowercase());
                }
                s
            };
            parts.push(value);
        }
        out.push_str(&parts.join("_"));
    }
    for (from, to) in acronym_fixes() {
        out = out.replace(from, to);
    }
    let extra_fixes = [
        ("Opengles", "OpenGLES"),
        ("Opengl", "OpenGL"),
        ("Gpu", "GPU"),
        ("Cpu", "CPU"),
        ("Winui", "WinUI"),
    ];
    for (from, to) in extra_fixes {
        out = out.replace(from, to);
    }
    out
}
pub(crate) fn bitmask_variant_name(name: &str) -> String {
    let raw = shouty_snake_case_name(name);
    if let Some(first) = raw.chars().next() {
        if first.is_ascii_digit() {
            return normalize_digit_prefix(&raw);
        }
    }
    raw
}
pub(crate) fn safe_ident(name: &str) -> String {
    let keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn",
    ];

    if keywords.contains(&name) {
        format!(r#"r#{name}"#, name = name)
    } else {
        name.to_string()
    }
}
pub(crate) fn normalize_digit_prefix_camel(value: &str) -> String {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return value.to_string();
    };
    if !first.is_ascii_digit() {
        return value.to_string();
    }
    let Some(second) = chars.next() else {
        return value.to_string();
    };
    let mut rest: String = chars.collect();
    rest.insert(0, first);
    let mut out = String::new();
    out.push(second.to_ascii_uppercase());
    out.push_str(&rest);
    out
}
pub(crate) fn normalize_digit_prefix(raw: &str) -> String {
    let mut chars = raw.chars().peekable();
    let mut digits = String::new();
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            digits.push(*c);
            chars.next();
        } else {
            break;
        }
    }

    let rest: String = chars.collect();
    if let Some(stripped) = rest.strip_prefix('D') {
        format!("D{digits}{stripped}", digits = digits, stripped = stripped)
    } else {
        format!("D{digits}{rest}", digits = digits, rest = rest)
    }
}
pub(crate) fn normalize_digit_prefix_snake(raw: &str) -> String {
    let mut chars = raw.chars();
    let Some(first) = chars.next() else {
        return raw.to_string();
    };
    if !first.is_ascii_digit() {
        return raw.to_string();
    }
    let Some(second) = chars.next() else {
        return raw.to_string();
    };
    let mut rest: String = chars.collect();
    rest.insert(0, first);
    let mut out = String::new();
    out.push(second);
    out.push_str(&rest);
    out
}
pub(crate) fn uppercase_after_digits(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut prev_digit = false;
    for ch in value.chars() {
        if prev_digit && ch.is_ascii_lowercase() {
            out.push(ch.to_ascii_uppercase());
        } else {
            out.push(ch);
        }
        prev_digit = ch.is_ascii_digit();
    }
    out
}
pub(crate) fn ffi_type_name(canonical: &str, c_prefix: &str) -> String {
    if canonical == "INTERNAL_HAVE_EMDAWNWEBGPU_HEADER" {
        return format!("{c_prefix}{canonical}");
    }
    format!("{}{}", c_prefix, camel_case_with_acronyms(canonical))
}
pub(crate) fn acronym_fixes() -> &'static [(&'static str, &'static str)] {
    &[
        ("WebGpu", "WebGPU"),
        ("WebXr", "WebXR"),
        ("Webgpu", "WebGPU"),
        ("Webxr", "WebXR"),
        ("Wgpu", "WGPU"),
        ("Wgsl", "WGSL"),
        ("Hlsl", "HLSL"),
        ("Html", "HTML"),
        ("Spirv", "SPIRV"),
        ("Dxgi", "DXGI"),
        ("Mtl", "MTL"),
        ("IoSurface", "IOSurface"),
        ("Hwnd", "HWND"),
        ("Uwp", "UWP"),
        ("WinUi", "WinUI"),
        ("Oom", "OOM"),
        ("Fd", "FD"),
        ("D3d12", "D3D12"),
        ("D3d11", "D3D11"),
        ("D3d", "D3D"),
        ("Vk", "Vk"),
        ("Xcb", "XCB"),
        ("OpenGl", "OpenGL"),
        ("OpenGles", "OpenGLES"),
        ("Egl", "EGL"),
    ]
}

pub(crate) fn camel_case_name(name: &str) -> String {
    name.to_upper_camel_case()
}

pub(crate) fn snake_case_name(name: &str) -> String {
    let raw = name.to_snake_case();
    normalize_digit_prefix_snake(&raw)
}

pub(crate) fn shouty_snake_case_name(name: &str) -> String {
    let raw = name.to_shouty_snake_case();
    if let Some(first) = raw.chars().next() {
        if first.is_ascii_digit() {
            return normalize_digit_prefix(&raw);
        }
    }
    raw
}

pub(crate) fn camel_case_with_acronyms(name: &str) -> String {
    let mut out = camel_case_name(name);
    for (from, to) in acronym_fixes() {
        out = out.replace(from, to);
    }
    uppercase_after_digits(&out)
}
