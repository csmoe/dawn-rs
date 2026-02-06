# Dawn Rust Code Generator - Multi-Agent System

## Overview

This document defines a multi-agent system for designing and implementing a Rust-based code generator for the Dawn graphics API. The generator produces idiomatic Rust wrappers that mirror wgpu's API style while calling through `src/ffi.rs` bindings.

## Project-Specific Instructions (From User)

- Always iterate with `cargo check` when changing generator logic or outputs.
- Use the `heck` crate for all name conversions, including enum variants and field names. Do not emit `allow(non_camel_case_types)`.
- The generator must not accept an `ffi_path` parameter or read `ffi.rs`. FFI is only for type conversions and function calls.
- Follow `codegen.md` default handling: `null`/`nullptr` defaults map to `Option::None`, enum/bitmask defaults map by name.
- Callback info fields: determine `nextInChain` and callback `userdata` based on `dawn.json` extensible/chain metadata.
- Free-members: follow `api_cpp.h` and only generate free calls for out-structures that contain pointers/arrays/string views needing freeing.
- Codegen should accept external output path and Dawn path (Dawn bin + api header) for FFI generation; use as `build-dependency`.
- The generator should produce strings and run `prettyplease`. If `syn`/`prettyplease` fails, return the raw string for debugging.
- Generated code should not be committed to the repo; write under `OUT_DIR` (for example `OUT_DIR/generated`).
- The example directory is `examples/` and must use generated APIs only (no direct `ffi` usage).
- `map_async` and other future-returning methods must return `Future::from_ffi(...)` (no `Future::new()` placeholder).
- Do not generate getters/setters; struct fields are `pub` (except `extensions`).
- FFI generation is handled by `build.rs`; `ffi.rs` is generated under the output directory.
- Dawn artifacts live under `~/Downloads/dawn-debug` (include + bin). Reduce environment variable usage; on macOS, find SDK via `xcrun`.
- Dawn tags live in `dawn.json` and must map to Rust features with `cfg` gating.
- Use `c_prefix` from `dawn.json` to drive the emitter.

---

## Agent Definitions

### Agent 1: Architecture Analyst

**Role:** Analyze existing artifacts and establish the overall generator architecture.

**Responsibilities:**
- Parse and understand `dawn.json` schema and `src/parser.rs` implementation
- Study `api_cpp.h` patterns for wrapper generation insights
- Review `codegen.md` conventions and requirements
- Define the data flow: `dawn.json` â†’ Parser â†’ API Model â†’ Visitor â†’ Rust Source

**Inputs:**
- https://github.com/google/dawn - google's dawn repo
  - important files
    - `codegen.md` - Codegen design document
    - `dawn.json` - API specification
    - `api_cpp.h` - C++ wrapper reference
- `src/parser.rs` - Existing parser implementation

**Outputs:**
- Architecture diagram/description
- API model structure definition
- Module organization plan
- Pattern mapping: C++ concepts â†’ Rust equivalents

**Key Decisions:**
```
1. API Model Structure:
   - Define ApiModel { enums, structs, objects, functions, callbacks }
   - Each element carries metadata: name, members, attributes, documentation

2. Module Layout:
   generated/
    mod.rs          # Re-exports
    enums.rs        # All enum definitions
    structs.rs      # Non-handle structs with builders
    objects.rs      # Object handles (Device, Queue, Buffer, etc.)
    callbacks.rs    # Callback type definitions
    extensions.rs   # Chained structure extensions

3. Data Flow:
   dawn.json
        (parse)
   ApiModel (in-memory AST)
        (visit)
   CodeEmitter (visitor implementation)
        (format!())
   String buffers per module
        (write)
   generated/*.rs files
```

---

### Agent 2: Visitor Pattern Designer

**Role:** Design the visitor trait hierarchy and traversal mechanism.

**Responsibilities:**
- Define visitor traits for each API element type
- Implement traversal order and dependency resolution
- Handle cross-references between types
- Design emit context for code accumulation

**Outputs:**
- Visitor trait definitions
- Visit method signatures
- Traversal algorithm
- EmitContext structure

**Trait Design:**
```rust
/// Core visitor trait for API model traversal
pub trait ApiVisitor {
    /// Called before visiting any elements
    fn begin(&mut self, model: &ApiModel);

    /// Visit an enumeration definition
    fn visit_enum(&mut self, e: &EnumDef);

    /// Visit a bitflag definition
    fn visit_flags(&mut self, f: &FlagsDef);

    /// Visit a plain struct (non-chained)
    fn visit_struct(&mut self, s: &StructDef);

    /// Visit a chained/extensible struct
    fn visit_chained_struct(&mut self, s: &ChainedStructDef);

    /// Visit an object handle type (Device, Buffer, etc.)
    fn visit_object(&mut self, o: &ObjectDef);

    /// Visit a method on an object
    fn visit_method(&mut self, obj: &ObjectDef, m: &MethodDef);

    /// Visit a standalone function
    fn visit_function(&mut self, f: &FunctionDef);

    /// Visit a callback type definition
    fn visit_callback(&mut self, c: &CallbackDef);

    /// Called after visiting all elements
    fn end(&mut self, model: &ApiModel);
}

/// Context passed during emission
pub struct EmitContext {
    pub enums_buf: String,
    pub structs_buf: String,
    pub handles_buf: String,
    pub callbacks_buf: String,
    pub extensions_buf: String,
    pub imports: HashSet<String>,
}
```

**Traversal Order:**
1. Enums and flags (no dependencies)
2. Callback type definitions
3. Plain structs (may reference enums)
4. Chained structs / extensions
5. Object handles
6. Methods and functions

---

### Agent 3: FFI Bridge Designer

**Role:** Design the Rust wrapper layer that calls into `src/ffi.rs`.

**Responsibilities:**
- Map FFI types to safe Rust types
- Design handle wrappers with Drop implementation
- Implement callback conversion (Rust closures FFI function pointers)
- Use Rust strict provenance apis to handle FFI pointers(like std::ptr::addr_of/addr_of_mut/cast/cast_mut), NO ptr `as` 
- Design chained structure builder pattern(no need to create XxxBuilder, the rust structure is the actual builder)
- Use heck crate to handle name conversions
- Read the codegen.md every time you trying to write the codegen logics in details.
- Follow the default handling in codegen.md to use rust's Option and lifetime annotations to handle FFI pointers and ensure safe memory management.
- Always use strict provenance pointer APIs (`std::ptr::addr_of`, `addr_of_mut`, `cast`, `cast_mut`) and avoid `as *mut` / `as *const` casts.
- Do not depend on `src/ffi.rs` to drive code generation; it is only for FFI type conversions and FFI function calls.
- Remove `allow(non_camel_case_types)` in generated enums; use `heck` to convert all names including enum variants, fields, and methods.
- When determining callback info fields, rely on `dawn.json` extensible/chain metadata; include `nextInChain` and optional `mode` and populate userdata fields as required by the callback info schema.
- Do not use `ffi` directly in examples; examples must use generated APIs only.
- Use `cargo check` to iterate during development and fix compile errors before moving on.

**Outputs:**
- Type mapping table
- Handle wrapper template
- Callback conversion strategy
- Builder pattern for chained structs

**Type Mapping:**
- All the name and code style should follow the rust's conventions.
- convert number starting names to second char to first like 3D to D3
```
| FFI Type              | Rust Wrapper Type        | Notes                    |
|-----------------------|--------------------------|--------------------------|
| WGPUDevice            | Device (newtype)         | Drop calls ffi::release  |
| WGPUBuffer            | Buffer (newtype)         | Drop calls ffi::release  |
| WGPUEnum              | enum Foo { ... }         | repr(u32), From<ffi>     |
| WGPUFlags             | bitflags! Foo            | From/Into ffi type       |
| WGPUStruct            | struct Foo + FooBuilder  | Builder for optional     |
| WGPUChainedStruct     | Vec<Extension>           | Flatten at FFI boundary  |
| Callback fn ptr       | Box<dyn FnMut(...)>      | Trampoline + closure     |
| char*                 | &str / String            | CString conversion       |
| WGPUStringView        | &str                     | &str to ptr+len conversion       |
| T*                    | &T / &mut T / Option<&T> | Context-dependent        |
```

**Handle Wrapper Template:**
```rust
// Generated pattern for object handles
pub struct Device {
    raw: ffi::WGPUDevice,
}

impl Device {
    /// Wraps a raw FFI handle. Takes ownership.
    pub(crate) unsafe fn from_raw(raw: ffi::WGPUDevice) -> Self {
        Self { raw }
    }

    /// Returns the raw handle without transferring ownership.
    pub fn as_raw(&self) -> ffi::WGPUDevice {
        self.raw
    }

    // Generated methods call ffi functions
    pub fn create_buffer(&self, descriptor: &BufferDescriptor) -> Buffer {
        let ffi_desc = descriptor.to_ffi();
        let raw = unsafe { ffi::wgpuDeviceCreateBuffer(self.raw, &ffi_desc) };
        unsafe { Buffer::from_raw(raw) }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { ffi::wgpuDeviceRelease(self.raw) };
    }
}
```

**Callback Conversion Strategy:**
```rust
// Callback wrapper using Box<dyn FnMut>
pub type ErrorCallback = Box<dyn FnMut(ErrorType, &str) + Send>;

// Trampoline for FFI
unsafe extern "C" fn error_callback_trampoline(
    ty: ffi::WGPUErrorType,
    message: *const c_char,
    userdata: *mut c_void,
) {
    let callback = &mut *(userdata as *mut ErrorCallback);
    let msg = CStr::from_ptr(message).to_str().unwrap_or("");
    callback(ErrorType::from(ty), msg);
}

// Usage in generated code
impl Device {
    pub fn set_uncaptured_error_callback(&self, callback: ErrorCallback) {
        let boxed = Box::new(callback);
        let userdata = Box::into_raw(boxed).as_mut_ptr();
        unsafe {
            ffi::wgpuDeviceSetUncapturedErrorCallback(
                self.raw,
                Some(error_callback_trampoline),
                userdata,
            );
        }
    }
}
```

**Chained Structure Builder:**
```rust
// Extension enum for descriptor chains
pub enum DeviceDescriptorExtension {
    DawnToggles(DawnTogglesDescriptor),
    DawnCacheDevice(DawnCacheDeviceDescriptor),
}

// Builder pattern
pub struct DeviceDescriptorBuilder {
    label: Option<String>,
    required_features: Vec<FeatureName>,
    required_limits: Option<Limits>,
    extensions: Vec<DeviceDescriptorExtension>,
}

impl DeviceDescriptorBuilder {
    pub fn new() -> Self { /* defaults */ }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn extension(mut self, ext: DeviceDescriptorExtension) -> Self {
        self.extensions.push(ext);
        self
    }

    pub fn build(self) -> DeviceDescriptor {
        DeviceDescriptor { /* ... */ }
    }
}

impl DeviceDescriptor {
    // Convert to FFI, building the chain internally
    pub(crate) fn to_ffi(&self) -> ffi::WGPUDeviceDescriptor {
        // Build chained struct linked list here
        // Keep allocations alive for FFI call duration
    }
}
```

---

### Agent 4: Code Emitter Implementer

**Role:** Implement the code generation using `format!(r#"..."#)` patterns.

**Responsibilities:**
- Write format templates for each code construct
- Ensure no escape sequences (`\n`, `\t`, `\r`) and padding in output, generated code will be formatted with prettyplease finally
- Handle indentation via literal whitespace in raw strings
- Generate module files with proper imports
- Ensure not using ffi.rs as parsing/codegen input, they are ONLY used for ffi type conversions and ffi functions call
- Do not accept an `ffi_path` parameter in codegen; generation must not read `ffi.rs`.
- Use `heck` to convert all names, including enum variants and fields; remove any `allow(non_camel_case_types)` in generated code.
- Respect `codegen.md` default handling rules for members with `"default"` (including numeric, enum/bitmask value names, and `nullptr`/`null` meaning `None`).
- Follow `api_cpp.h` behavior for `FreeMembers` by only generating free-member calls for out-structures that contain pointers/arrays/string views requiring freeing.
- Prefer `cargo check` after generator changes and after regenerating outputs.

**Outputs:**
- Format templates for all constructs
- File writing strategy
- Import management

**Format Template Examples:**

**Enum Generation:**
 - variant's value should be sign with the one from ffi like `Foo = ffi::WGPU_xxx_FOO,`
```rust
fn emit_enum(e: &EnumDef) -> String {
    let variants = e.values.iter()
        .map(|v| format!(r#"    {} = ffi::{name},"#, v.rust_name))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum {name} {{
{variants},
}}

impl From<ffi::WGPU{name}> for {name} {{
    fn from(value: ffi::WGPU{name}) -> Self {{
        match value {{
{from_variants}
        }}
    }}
}}

impl From<{name}> for ffi::WGPU{name} {{
    fn from(value: {name}) -> Self {{
        value as u32
    }}
}}
"#,
        name = e.rust_name,
        variants = variants,
        from_variants = emit_from_variants(e),
    )
}
```

**Struct with extension:**
```rust
fn emit_struct(s: &StructDef) -> String {
    let fields = s.fields.iter()
        .map(|f| format!(r#"    pub {}: {},"#, f.rust_name, f.rust_type))
        .collect::<Vec<_>>()
        .join("\n");

    let builder_methods = s.fields.iter()
        .filter(|f| f.has_default)
        .map(|f| format!(r#"
    pub fn {name}(mut self, value: {ty}) -> Self {{
        self.{name} = value;
        self
    }}"#,
            name = f.rust_name,
            ty = f.rust_type,
        ))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"
#[derive(Debug, Clone)]
pub struct {name} {{
// if extensible
extensions:Vec<{name}Ext>
{fields}
}}

impl Default for {name} {{
    fn default() -> Self {{
        Self {{
{defaults}
        }}
    }}
}}

impl {name} {{
    pub fn with_foo(mut self, ext: {name}Ext) -> {name} {{
        self.extensions.push(ext);
    }}

    pub fn to_ffi(self) -> ffi::{c_type} {{
        let mut next_in_chain: *mut ffi::WGPUChainedStruct = std::ptr::null_mut();
        for ext in self.extensions.iter_mut().rev() {
            match ext {
                {name}Ext::xxx(inner) => {

                }
                ... => {}
            }
        }
        ffi::{c_type} { nextInChain: next_in_chain, ...}
    }}
}}

        "#);


```

**Object Handle with Methods:**
```rust
fn emit_object(o: &ObjectDef) -> String {
    let methods = o.methods.iter()
        .map(|m| emit_method(o, m))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"
/// {doc}
pub struct {name} {{
    raw: ffi::WGPU{name},
}}

impl {name} {{
    pub(crate) unsafe fn from_raw(raw: ffi::WGPU{name}) -> Self {{
        debug_assert!(!raw.is_null());
        Self {{ raw }}
    }}

    pub fn as_raw(&self) -> ffi::WGPU{name} {{
        self.raw
    }}

{methods}
}}

impl Drop for {name} {{
    fn drop(&mut self) {{
        if self.raw.is_null() {return;}
        unsafe {{ ffi::wgpu{name}Release(self.raw) }};
    }}
}}

// Clone via AddRef if supported
impl Clone for {name} {{
    fn clone(&self) -> Self {{
        unsafe {{ ffi::wgpu{name}AddRef(self.raw) }};
        Self {{ raw: self.raw }}
    }}
}}
"#,
        name = o.rust_name,
        doc = o.doc,
        methods = methods,
    )
}

fn emit_method(obj: &ObjectDef, m: &MethodDef) -> String {
    let params = m.params.iter()
        .map(|p| format!("{}: {}", p.rust_name, p.rust_type))
        .collect::<Vec<_>>()
        .join(", ");

    let ffi_args = m.params.iter()
        .map(|p| emit_ffi_conversion(p))
        .collect::<Vec<_>>()
        .join(", ");

    format!(r#"
    /// {doc}
    pub fn {name}(&self, {params}) -> {return_type} {{
        let result = unsafe {{
            ffi::wgpu{obj}{method}(self.raw, {ffi_args})
        }};
        {return_conversion}
    }}"#,
        name = m.rust_name,
        doc = m.doc,
        params = params,
        return_type = m.rust_return_type,
        obj = obj.rust_name,
        method = m.ffi_name,
        ffi_args = ffi_args,
        return_conversion = emit_return_conversion(m),
    )
}
```

---

### Agent 5: Integration & Validation Engineer

**Role:** Design the example project and validation strategy.

**Responsibilities:**
- Create example Cargo project structure
- Define validation test cases
- Set up golden file testing
- Ensure `cargo check` passes

**Outputs:**
- Example crate structure
- Test strategy document
- CI/CD integration plan

**Example Crate Structure:**
```
dawn-example/
â”œâ”€â”€ Cargo.toml
â”œâ”€â”€ src/
â”‚   â””â”€â”€ main.rs
â””â”€â”€ build.rs (optional, if generation happens at build time)
```

**Cargo.toml:**
```toml
[package]
name = "dawn-example"
version = "0.1.0"
edition = "2021"

[dependencies]
dawn = { path = "../dawn-rs" }

# For callback support
```

**src/main.rs:**
```rust
use dawn::{
    Instance, InstanceDescriptor, BackendType,
    RequestAdapterOptions, PowerPreference,
    DeviceDescriptor, BufferDescriptor, BufferUsages,
};

fn main() {
    // Create instance
    let instance_desc = InstanceDescriptor::builder()
        .backends(BackendType::Vulkan | BackendType::Metal)
        .build();
    let instance = Instance::new(&instance_desc);

    // Request adapter
    let adapter_options = RequestAdapterOptions::builder()
        .power_preference(PowerPreference::HighPerformance)
        .build();

    let adapter = instance.request_adapter(&adapter_options, |status, adapter, msg| {
        if status == RequestAdapterStatus::Success {
            println!("Adapter acquired: {:?}", adapter);
        } else {
            eprintln!("Failed: {}", msg);
        }
    });

    // This example validates:
    // 1. Instance creation with builder pattern
    // 2. Enum usage (BackendType, PowerPreference)
    // 3. Callback invocation
    // 4. Method chaining
}
```

**Test Strategy:**

1. **Unit Tests for Codegen Helpers:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_name_conversion() {
        assert_eq!(to_rust_name("WGPUBufferUsage"), "BufferUsage");
        assert_eq!(to_rust_name("WGPU_BUFFER_USAGE_COPY_SRC"), "COPY_SRC");
    }

    #[test]
    fn test_enum_emission() {
        let e = EnumDef { /* ... */ };
        let output = emit_enum(&e);
        assert!(output.contains("pub enum"));
        assert!(output.contains("impl From<ffi::"));
    }
}
```

2. **Golden File Tests:**
```rust
#[test]
fn test_generated_matches_golden() {
    let model = parse_dawn_json("dawn.json");
    let generated = generate_all(&model);

    let golden = std::fs::read_to_string("golden/api.rs").unwrap();
    assert_eq!(generated.trim(), golden.trim());
}
```

3. **Cargo Check Validation:**
```bash
#!/bin/bash
# In CI pipeline
cd dawn-example
cargo check --all-targets
cargo clippy -- -D warnings
```

---

### Agent 6: Documentation & Standards Keeper

**Role:** Ensure alignment with `codegen.md` and `api_cpp.h` patterns.

**Responsibilities:**
- Extract and document conventions from `codegen.md`
- Map C++ patterns from `api_cpp.h` to Rust equivalents
- Maintain naming consistency
- Document edge cases and special handling

**Outputs:**
- Convention checklist
- Pattern mapping document
- Edge case handling guide

**Convention Checklist from codegen.md:**

| Rule | Description | Rust Implementation |
|------|-------------|---------------------|
| Naming | Types strip `WGPU` prefix | `WGPUDevice` â†’ `Device` |
| Ownership | Objects have reference counting | `Clone` via `AddRef`, `Drop` via `Release` |
| Errors | Return error enums, not exceptions | `Result<T, Error>` or status callback |
| Nullability | Explicit optional parameters | `Option<&T>` for nullable |
| Strings | UTF-8, null-terminated in FFI | `&str` â†’ `CString` at boundary |
| Arrays | Pointer + length pairs | `&[T]` converted at FFI |

**C++ to Rust Pattern Mapping:**

| C++ Pattern (`api_cpp.h`) | Rust Equivalent |
|---------------------------|-----------------|
| Default arguments in methods | Builder pattern with defaults |
| Method chaining (`device.Create...`) | Methods on handle structs |
| RAII via destructor | `impl Drop` calling `Release` |
| Callback with userdata `void*` | `Box<dyn FnMut>` + trampoline |
| `ChainedStruct* nextInChain` | `Vec<Extension>` flattened at FFI |
| Implicit conversions | `From`/`Into` traits |
| Const references | `&T` borrows |

**Edge Cases:**

1. **Async Callbacks:** Some operations (adapter request, buffer mapping) use callbacks that fire later. Store callback boxes in a registry to prevent premature drop.

2. **Chained Struct Lifetime:** The linked list for `nextInChain` must outlive the FFI call. Build it in a temporary buffer within `to_ffi()` and ensure the descriptor holds references.

3. **String Lifetime:** `CString` must outlive FFI calls. Use a string arena or stack allocation for the call scope.

4. **Bitflags Combination:** Use `bitflags!` macro for flag types to enable `|` operator.

---

## Implementation Plan

### Phase 1: Foundation (Days 1-2)
1. Review and annotate `dawn.json` schema
2. Extend `src/parser.rs` to produce typed `ApiModel`
3. Define all visitor traits in `src/visitor.rs`
4. Set up output directory structure

### Phase 2: Core Types (Days 3-4)
5. Implement enum and flags visitor/emitter
6. Implement plain struct visitor/emitter with builders
7. Add `From`/`Into` conversions for FFI types
8. Write unit tests for type emission

### Phase 3: Handles & Methods (Days 5-6)
9. Implement object handle visitor/emitter
10. Generate methods with FFI calls
11. Handle return type conversions
12. Implement `Drop` and `Clone` for handles

### Phase 4: Advanced Features (Days 7-8)
13. Implement callback type emission
14. Create trampoline generator for callbacks
15. Implement chained struct extension system
16. Add builder pattern for complex descriptors

### Phase 5: Integration (Days 9-10)
17. Generate complete module structure
18. Create example crate
19. Run `cargo check` and fix issues
20. Run `cargo clippy` and address warnings

### Phase 6: Validation (Days 11-12)
21. Create golden file tests
22. Write integration tests
23. Document generated API
24. Performance review of generated code

---

## Success Criteria

- [ ] All enum types match wgpu style
- [ ] All handles implement `Drop` correctly
- [ ] Callbacks work with Rust closures
- [ ] Builder pattern available for all descriptors
- [ ] Chained structs hide `nextInChain` complexity
- [ ] Generated code formatted by `rustfmt`
- [ ] Golden file tests pass
- [ ] Example compiles with cargo run
  - [ ] create the example with winit and objc2 or any crates you need
  - [ ] link with libwebgpu_dawn.a
  - [ ] execute `cargo run` to print the adapter info to verify
  - [ ] Example must use generated API only (no direct `ffi` usage)

---

## File Output Structure

```
src/
 generator/
  mod.rs
  visitor.rs      # Visitor traits
  emitter.rs      # Code emission helpers
    constants.rs # Constants generation
    enums.rs     # Enum generation
    structs.rs   # Struct generation
    objects.rs   # Handle generation
    callbacks.rs # Callback generation
    extensions.rs    # Chained struct generation
    functions.rs     # Function generation
 parser.rs           # Existing, extended
 ffi.rs              # Existing, unchanged
 generated/          # Output directory
     mod.rs
     enums.rs
     structs.rs
     handles.rs
     callbacks.rs
     extensions.rs
